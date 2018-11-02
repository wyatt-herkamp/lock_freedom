use super::Removed;
use owned_alloc::{OwnedAlloc, UninitAlloc};
use std::{mem::forget, ptr::NonNull};

#[derive(Debug, PartialEq, Eq)]
pub enum Insertion<K, V, E> {
    Created,
    Updated(Removed<K, V>),
    Failed(E),
}

impl<K, V, E> Insertion<K, V, E> {
    pub fn created(&self) -> bool {
        match self {
            Insertion::Created => true,
            _ => false,
        }
    }

    pub fn updated(&self) -> Option<&Removed<K, V>> {
        match self {
            Insertion::Updated(pair) => Some(pair),
            _ => None,
        }
    }

    pub fn take_updated(self) -> Result<Removed<K, V>, Self> {
        match self {
            Insertion::Updated(pair) => Ok(pair),
            this => Err(this),
        }
    }

    pub fn failed(&self) -> Option<&E> {
        match self {
            Insertion::Failed(err) => Some(err),
            _ => None,
        }
    }

    pub fn take_failed(self) -> Result<E, Self> {
        match self {
            Insertion::Failed(e) => Ok(e),
            this => Err(this),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preview<V> {
    Discard,
    Keep,
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
            let val = if self.is_val_init { Some(&mut *val) } else { None };
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
        Self { interactive, removed, is_valid: false }
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
