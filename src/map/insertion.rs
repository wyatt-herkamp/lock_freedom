use super::{bucket::Pair, removed::Removed};
use alloc::*;
use std::{mem, ptr::NonNull};

/// The result of an _interactive_ insertion.
#[derive(Debug, PartialEq, Eq)]
pub enum Insertion<K, V, E> {
    /// If this is returned, an entry for the given key was successfully
    /// created with the provided value i.e. there was no entry before.
    Created,
    /// If this is returned, an entry for the given key already existed and
    /// was successfully updated with the provided value. The field is the old
    /// entry pair.
    Updated(Removed<K, V>),
    /// If this is returned, the insertion failed and no action was done.
    /// Failure may have happened because the given closure rejected the
    /// conditions. The field will depend on the method you called.
    Failed(E),
}

impl<K, V, E> Insertion<K, V, E> {
    /// Is this insertion a creation?
    pub fn created(&self) -> bool {
        match self {
            Insertion::Created => true,
            _ => false,
        }
    }

    /// Is this insertion an update? If so, the return is a reference to the
    /// old value.
    pub fn updated(&self) -> Option<&Removed<K, V>> {
        match self {
            Insertion::Updated(pair) => Some(pair),
            _ => None,
        }
    }

    /// Is this insertion an update? If so, the old value is taken and
    /// returned. Otherwise, the insertion is returned.
    pub fn take_updated(self) -> Result<Removed<K, V>, Self> {
        match self {
            Insertion::Updated(pair) => Ok(pair),
            this => Err(this),
        }
    }

    /// Is this a failure? If so, return a reference to the custom field.
    pub fn failed(&self) -> Option<&E> {
        match self {
            Insertion::Failed(err) => Some(err),
            _ => None,
        }
    }

    /// Is this a failure? If so, the custom field is taken and
    /// returned. Otherwise, the insertion is returned.
    pub fn take_failed(self) -> Result<E, Self> {
        match self {
            Insertion::Failed(e) => Ok(e),
            this => Err(this),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ValStatus {
    Uninited,
    Discarded,
    Kept,
}

#[derive(Debug)]
pub struct PreviewAlloc<K, V> {
    ptr: NonNull<Pair<K, V>>,
    status: ValStatus,
}

impl<K, V> PreviewAlloc<K, V> {
    pub fn from_key(key: K) -> Self {
        Self {
            ptr: unsafe { alloc(Pair { key, val: { mem::uninitialized() } }) },
            status: ValStatus::Uninited,
        }
    }

    pub fn from_key_val(key: K, val: V) -> Self {
        Self {
            ptr: unsafe { alloc(Pair { key, val }) },
            status: ValStatus::Kept,
        }
    }

    pub unsafe fn from_alloc(ptr: NonNull<Pair<K, V>>, has_val: bool) -> Self {
        Self {
            ptr,
            status: if has_val {
                ValStatus::Kept
            } else {
                ValStatus::Discarded
            },
        }
    }

    pub fn is_val_kept(&self) -> bool {
        self.status == ValStatus::Kept
    }

    pub fn is_val_uninited(&self) -> bool {
        self.status == ValStatus::Uninited
    }

    pub fn key(&self) -> &K {
        unsafe { &self.ptr.as_ref().key }
    }

    pub fn val(&self) -> Option<&V> {
        if self.is_val_kept() {
            Some(unsafe { &self.ptr.as_ref().val })
        } else {
            None
        }
    }

    pub fn ptr(&self) -> NonNull<Pair<K, V>> {
        self.ptr
    }

    pub fn set_val(&mut self, val: V) {
        if self.is_val_uninited() {
            unsafe { (&mut self.ptr.as_mut().val as *mut V).write(val) }
        } else {
            unsafe { self.ptr.as_mut().val = val }
        }
        self.status = ValStatus::Kept;
    }

    pub fn discard(&mut self) {
        if self.is_val_kept() {
            self.status = ValStatus::Discarded
        }
    }

    pub fn keep(&mut self) {
        if self.is_val_uninited() {
            panic!("Cannot keep uninitialized value")
        }
        self.status = ValStatus::Kept;
    }
}

impl<K, V> Drop for PreviewAlloc<K, V> {
    fn drop(&mut self) {
        if self.is_val_uninited() {
            unsafe {
                (&mut self.ptr.as_mut().key as *mut K).drop_in_place();
                dealloc_moved(self.ptr)
            }
        } else {
            unsafe { dealloc(self.ptr) }
        }
    }
}

/// A preview of the value in an __interactive__ insertion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preview<V> {
    /// Discard the current generated value. If there is no currently
    /// generated value, the closure returning this is not generating anyone
    /// (i.e. conditions not met).
    Discard,
    /// Keep the current genereated value. If there is no currently
    /// generated value, the closure returning this is not generating anyone
    /// (i.e. conditions not met).
    Keep,
    /// Replace the current value with this new one.
    New(V),
}

pub trait Inserter<K, V> {
    unsafe fn update(
        &mut self,
        created: &mut PreviewAlloc<K, V>,
        stored: Option<NonNull<Pair<K, V>>>,
    );

    unsafe fn update_and_test(
        &mut self,
        created: &mut PreviewAlloc<K, V>,
        stored: Option<NonNull<Pair<K, V>>>,
    ) -> bool {
        self.update(created, stored);
        created.is_val_kept()
    }
}

pub struct NewInserter<F> {
    update: F,
}

impl<F> NewInserter<F> {
    pub fn new<K, V>(update: F) -> Self
    where
        F: FnMut(&K, Option<&V>, Option<&V>) -> Preview<V>,
    {
        Self { update }
    }
}

impl<K, V, F> Inserter<K, V> for NewInserter<F>
where
    F: FnMut(&K, Option<&V>, Option<&V>) -> Preview<V>,
{
    unsafe fn update(
        &mut self,
        created: &mut PreviewAlloc<K, V>,
        stored: Option<NonNull<Pair<K, V>>>,
    ) {
        match (self.update)(
            created.key(),
            stored.map(|p| &*p.as_ptr()).map(|p| &p.val),
            created.val(),
        ) {
            Preview::Keep => (),
            Preview::Discard => created.discard(),
            Preview::New(val) => created.set_val(val),
        }
    }
}

pub struct Reinserter<F> {
    pred: F,
}

impl<F> Reinserter<F> {
    pub fn new<K, V>(pred: F) -> Self
    where
        F: FnMut(&Removed<K, V>, Option<&V>) -> bool,
    {
        Self { pred }
    }
}

impl<K, V, F> Inserter<K, V> for Reinserter<F>
where
    F: FnMut(&Removed<K, V>, Option<&V>) -> bool,
{
    unsafe fn update(
        &mut self,
        created: &mut PreviewAlloc<K, V>,
        stored: Option<NonNull<Pair<K, V>>>,
    ) {
        debug_assert!(!created.is_val_uninited());
        let removed = Removed::new(created.ptr());
        let keep =
            (self.pred)(&removed, stored.map(|p| &*p.as_ptr()).map(|p| &p.val));
        mem::forget(removed);
        if keep {
            created.keep();
        } else {
            created.discard();
        }
    }
}
