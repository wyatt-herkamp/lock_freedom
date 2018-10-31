use super::bucket::Garbage;
use incin::Incinerator;
use owned_alloc::OwnedAlloc;
use std::{
    mem::forget,
    ops::Deref,
    ptr::NonNull,
    sync::{Arc, Weak},
};

pub struct Removed<K, V> {
    nnptr: NonNull<(K, V)>,
    origin: Weak<Incinerator<Garbage<K, V>>>,
}

impl<K, V> Removed<K, V> {
    pub(super) fn new(
        alloc: OwnedAlloc<(K, V)>,
        origin: &Arc<Incinerator<Garbage<K, V>>>,
    ) -> Self {
        Self { nnptr: alloc.into_raw(), origin: Arc::downgrade(origin) }
    }

    pub(super) fn into_alloc(this: Self) -> OwnedAlloc<(K, V)> {
        let alloc = unsafe { OwnedAlloc::from_raw(this.nnptr) };
        forget(this);
        alloc
    }

    pub(super) fn raw(this: &Self) -> NonNull<(K, V)> {
        this.nnptr
    }

    pub fn key(&self) -> &K {
        let (k, _) = &**self;
        k
    }

    pub fn val(&self) -> &V {
        let (_, v) = &**self;
        v
    }

    pub fn try_as_mut(this: &mut Self) -> Option<&mut (K, V)> {
        if this.origin.upgrade().is_none() {
            Some(unsafe { this.nnptr.as_mut() })
        } else {
            None
        }
    }

    pub fn try_into(this: Self) -> Result<(K, V), Self> {
        if this.origin.upgrade().is_none() {
            let (ret, _) =
                unsafe { OwnedAlloc::from_raw(this.nnptr) }.move_inner();
            forget(this);
            Ok(ret)
        } else {
            Err(this)
        }
    }
}

impl<K, V> Drop for Removed<K, V> {
    fn drop(&mut self) {
        let alloc = unsafe { OwnedAlloc::from_raw(self.nnptr) };
        self.origin.upgrade().map(|incin| incin.add(Garbage::Pair(alloc)));
    }
}

impl<K, V> Deref for Removed<K, V> {
    type Target = (K, V);

    fn deref(&self) -> &Self::Target {
        unsafe { self.nnptr.as_ref() }
    }
}
