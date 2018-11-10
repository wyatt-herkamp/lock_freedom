use super::Removed;
use owned_alloc::{OwnedAlloc, UninitAlloc};
use std::{mem::forget, ptr::NonNull};

/// A [`insert_with`](super::Map::insert_with) operation result.
#[derive(Debug, PartialEq, Eq)]
pub enum Insertion<K, V, E> {
    /// The entry was created.
    Created,
    /// The entry was updated and this was the old pair.
    Updated(Removed<K, V>),
    /// The insertion failed and no operation was performed. Failure of an
    /// insertion might happen because the closure rejected the conditions.
    /// Another reason is that method-specific contract was not respected (such
    /// as the one of [`reinsert_with`](super::Map::reinsert_with)).
    Failed(E),
}

impl<K, V, E> Insertion<K, V, E> {
    /// Returns whether the insertion created an entry.
    pub fn created(&self) -> bool {
        match self {
            Insertion::Created => true,
            _ => false,
        }
    }

    /// Returns whether the insertion updated an entry.
    pub fn updated(&self) -> Option<&Removed<K, V>> {
        match self {
            Insertion::Updated(pair) => Some(pair),
            _ => None,
        }
    }

    /// Tries to take the updated entry of this insertion and encodes it as a
    /// [`Result`]. [`Ok`] is returned only if this insertion updated a value.
    pub fn take_updated(self) -> Result<Removed<K, V>, Self> {
        match self {
            Insertion::Updated(pair) => Ok(pair),
            this => Err(this),
        }
    }

    /// Returns whether the insertion failed.
    pub fn failed(&self) -> Option<&E> {
        match self {
            Insertion::Failed(err) => Some(err),
            _ => None,
        }
    }

    /// Tries to take the failure of this insertion and encodes it as a
    /// [`Result`]. [`Ok`] is returned only if this insertion has a failure.
    pub fn take_failed(self) -> Result<E, Self> {
        match self {
            Insertion::Failed(e) => Ok(e),
            this => Err(this),
        }
    }
}

/// The preview of an _interactive_ insertion. It is used by the
/// [`insert_with`](super::Map::insert_with) method and it is the return value
/// of the closure passed to the method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preview<V> {
    /// Tells the [`Map`](super::Map) to discard the currently generated value.
    /// After this return value, the insertion is probably canceled and it
    /// fails. However, concurrent accesses to the [`Map`](super::Map) may
    /// cause the conditions to be tested again.
    Discard,
    /// Tells the [`Map`](super::Map) to keep the currently generated value. If
    /// there was no generated value, this has the same effect as
    /// [`Preview::Discard`].
    Keep,
    /// Tells the `Map to use this value instead of the previously generated
    /// (if any).
    New(V),
}

pub trait Inserter<K, V>: Sized {
    fn input(&mut self, found: Option<&(K, V)>);

    fn pointer(&self) -> Option<NonNull<(K, V)>>;

    fn key(&self) -> &K;

    fn take_pointer(self) {
        forget(self);
    }
}

pub struct InsertNew<F, K, V>
where
    F: FnMut(&K, Option<&mut V>, Option<&(K, V)>) -> Preview<V>,
{
    interactive: F,
    nnptr: NonNull<(K, V)>,
    is_val_init: bool,
}

impl<F, K, V> InsertNew<F, K, V>
where
    F: FnMut(&K, Option<&mut V>, Option<&(K, V)>) -> Preview<V>,
{
    pub fn with_key(interactive: F, key: K) -> Self {
        Self {
            interactive,
            nnptr: unsafe {
                let alloc = UninitAlloc::new().init_in_place(|(key_mem, _)| {
                    (key_mem as *mut K).write(key)
                });
                alloc.into_raw()
            },
            is_val_init: false,
        }
    }

    pub fn with_pair(interactive: F, pair: (K, V)) -> Self {
        Self {
            interactive,
            nnptr: OwnedAlloc::new(pair).forget_inner().into_raw(),
            is_val_init: true,
        }
    }

    pub fn into_pair(self) -> (K, Option<V>) {
        let ((key, val), _) =
            unsafe { OwnedAlloc::from_raw(self.nnptr) }.move_inner();
        let val = if self.is_val_init {
            Some(val)
        } else {
            forget(val);
            None
        };
        forget(self);
        (key, val)
    }
}

impl<F, K, V> Drop for InsertNew<F, K, V>
where
    F: FnMut(&K, Option<&mut V>, Option<&(K, V)>) -> Preview<V>,
{
    fn drop(&mut self) {
        if self.is_val_init {
            unsafe { OwnedAlloc::from_raw(self.nnptr) };
        } else {
            unsafe {
                {
                    let (key, _) = self.nnptr.as_mut();
                    (key as *mut K).drop_in_place();
                }
                UninitAlloc::from_raw(self.nnptr);
            }
        }
    }
}

impl<F, K, V> Inserter<K, V> for InsertNew<F, K, V>
where
    F: FnMut(&K, Option<&mut V>, Option<&(K, V)>) -> Preview<V>,
{
    fn input(&mut self, found: Option<&(K, V)>) {
        let (key, val) = unsafe { self.nnptr.as_mut() };

        let preview = {
            let val = if self.is_val_init {
                Some(&mut *val)
            } else {
                None
            };
            (self.interactive)(key, val, found)
        };

        match preview {
            Preview::Discard if self.is_val_init => {
                self.is_val_init = false;
                unsafe { (val as *mut V).drop_in_place() };
            },

            Preview::New(new_val) => {
                if self.is_val_init {
                    *val = new_val;
                } else {
                    self.is_val_init = true;
                    unsafe { (val as *mut V).write(new_val) };
                }
            },

            _ => (),
        }
    }

    fn pointer(&self) -> Option<NonNull<(K, V)>> {
        if self.is_val_init {
            Some(self.nnptr)
        } else {
            None
        }
    }

    fn key(&self) -> &K {
        let (key, _) = unsafe { self.nnptr.as_ref() };
        key
    }
}

pub struct Reinsert<F, K, V>
where
    F: FnMut(&(K, V), Option<&(K, V)>) -> bool,
{
    interactive: F,
    removed: Removed<K, V>,
    is_valid: bool,
}

impl<F, K, V> Reinsert<F, K, V>
where
    F: FnMut(&(K, V), Option<&(K, V)>) -> bool,
{
    pub fn new(interactive: F, removed: Removed<K, V>) -> Self {
        Self {
            interactive,
            removed,
            is_valid: false,
        }
    }

    pub fn into_removed(self) -> Removed<K, V> {
        self.removed
    }
}

impl<F, K, V> Inserter<K, V> for Reinsert<F, K, V>
where
    F: FnMut(&(K, V), Option<&(K, V)>) -> bool,
{
    fn input(&mut self, found: Option<&(K, V)>) {
        self.is_valid = (self.interactive)(&*self.removed, found);
    }

    fn pointer(&self) -> Option<NonNull<(K, V)>> {
        if self.is_valid {
            Some(Removed::raw(&self.removed))
        } else {
            None
        }
    }

    fn key(&self) -> &K {
        let (key, _) = &*self.removed;
        key
    }

    fn take_pointer(self) {
        forget(Removed::into_alloc(self.removed));
    }
}
