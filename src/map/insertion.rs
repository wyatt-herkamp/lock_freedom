use super::{bucket::Pair, removed::Removed};
use alloc::*;
use std::{mem, ptr::NonNull};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preview<V> {
    Discard,
    Keep,
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
        F: for<'a> FnMut(&'a K, Option<&'a V>, Option<&'a V>) -> Preview<V>,
    {
        Self { update }
    }
}

impl<K, V, F> Inserter<K, V> for NewInserter<F>
where
    F: for<'a> FnMut(&'a K, Option<&'a V>, Option<&'a V>) -> Preview<V>,
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
        F: for<'a> FnMut(&'a Removed<K, V>, Option<&'a V>) -> bool,
    {
        Self { pred }
    }
}

impl<K, V, F> Inserter<K, V> for Reinserter<F>
where
    F: for<'a> FnMut(&'a Removed<K, V>, Option<&'a V>) -> bool,
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
