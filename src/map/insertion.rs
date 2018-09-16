use super::{bucket::Pair, removed::Removed};
use alloc::*;
use std::{
    mem,
    ptr::{null_mut, NonNull},
};

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
pub enum Preview<'curr, V> {
    Discard,
    Keep(&'curr V),
    New(V),
}

pub trait Inserter<K, V> {
    unsafe fn test_create(
        &mut self,
        key: K,
        stored: *mut Pair<K, V>,
    ) -> Result<NonNull<Pair<K, V>>, K>;

    unsafe fn test_update(
        &mut self,
        created: NonNull<Pair<K, V>>,
        stored: *mut Pair<K, V>,
    ) -> *mut Pair<K, V>;
}

pub struct NewInserter<F> {
    update: F,
}

impl<F> NewInserter<F> {
    pub fn new(update: F) -> Self {
        Self { update }
    }
}

impl<K, V, F> Inserter<K, V> for NewInserter<F>
where
    F: for<'new> FnMut(&K, Option<&V>, Option<&'new V>) -> Preview<'new, V>,
{
    unsafe fn test_create(
        &mut self,
        key: K,
        stored: *mut Pair<K, V>,
    ) -> Result<NonNull<Pair<K, V>>, K> {
        match (self.update)(&key, stored.as_ref().map(|p| &p.val), None) {
            Preview::Discard => Err(key),
            Preview::Keep(_) => unreachable!(),
            Preview::New(val) => Ok(alloc(Pair { key, val })),
        }
    }

    unsafe fn test_update(
        &mut self,
        mut created: NonNull<Pair<K, V>>,
        stored: *mut Pair<K, V>,
    ) -> *mut Pair<K, V> {
        match (self.update)(
            &created.as_ref().key,
            stored.as_ref().map(|p| &p.val),
            Some(&created.as_ref().val),
        ) {
            Preview::Discard => null_mut(),
            Preview::Keep(_) => created.as_ptr(),
            Preview::New(val) => {
                created.as_mut().val = val;
                created.as_ptr()
            },
        }
    }
}

pub struct Reinserter<F> {
    pred: F,
}

impl<F> Reinserter<F> {
    pub fn new(pred: F) -> Self {
        Self { pred }
    }
}

impl<K, V, F> Inserter<K, V> for Reinserter<F>
where
    F: FnMut(&Removed<K, V>, Option<&V>) -> bool,
{
    unsafe fn test_create(
        &mut self,
        _key: K,
        _stored: *mut Pair<K, V>,
    ) -> Result<NonNull<Pair<K, V>>, K> {
        unreachable!()
    }

    unsafe fn test_update(
        &mut self,
        created: NonNull<Pair<K, V>>,
        stored: *mut Pair<K, V>,
    ) -> *mut Pair<K, V> {
        let removed = Removed::new(created);
        let ret = if (self.pred)(&removed, stored.as_ref().map(|p| &p.val)) {
            created.as_ptr()
        } else {
            null_mut()
        };
        mem::forget(removed);
        ret
    }
}
