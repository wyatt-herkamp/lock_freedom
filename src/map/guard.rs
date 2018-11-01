use super::bucket::Garbage;
use incin::{Incinerator, Pause};
use owned_alloc::OwnedAlloc;
use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    mem::forget,
    ops::Deref,
    ptr::NonNull,
    sync::{Arc, Weak},
};

#[derive(Debug)]
pub struct ReadGuard<'origin, K, V> {
    pair: &'origin (K, V),
    pause: Pause<'origin, Garbage<K, V>>,
}

impl<'origin, K, V> ReadGuard<'origin, K, V> {
    pub(super) fn new(
        pair: &'origin (K, V),
        pause: Pause<'origin, Garbage<K, V>>,
    ) -> Self {
        Self { pair, pause }
    }
}

impl<'origin, K, V> ReadGuard<'origin, K, V> {
    pub fn key(&self) -> &K {
        let (k, _) = &**self;
        k
    }

    pub fn val(&self) -> &V {
        let (_, v) = &**self;
        v
    }
}

impl<'origin, K, V> Deref for ReadGuard<'origin, K, V> {
    type Target = (K, V);

    fn deref(&self) -> &Self::Target {
        self.pair
    }
}

impl<'origin, K, V> PartialEq for ReadGuard<'origin, K, V>
where
    (K, V): PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<'origin, K, V> PartialEq<(K, V)> for ReadGuard<'origin, K, V>
where
    (K, V): PartialEq,
{
    fn eq(&self, other: &(K, V)) -> bool {
        **self == *other
    }
}

impl<'origin, K, V> Eq for ReadGuard<'origin, K, V> where (K, V): Eq {}

impl<'origin, K, V> PartialOrd for ReadGuard<'origin, K, V>
where
    (K, V): PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<'origin, K, V> PartialOrd<(K, V)> for ReadGuard<'origin, K, V>
where
    (K, V): PartialOrd,
{
    fn partial_cmp(&self, other: &(K, V)) -> Option<Ordering> {
        (**self).partial_cmp(other)
    }
}

impl<'origin, K, V> Ord for ReadGuard<'origin, K, V>
where
    (K, V): Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl<'origin, K, V> Hash for ReadGuard<'origin, K, V>
where
    (K, V): Hash,
{
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        (**self).hash(hasher)
    }
}

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

    pub(super) fn into_alloc(mut this: Self) -> OwnedAlloc<(K, V)> {
        let alloc = unsafe { OwnedAlloc::from_raw(this.nnptr) };
        unsafe { (&mut this.origin as *mut Weak<_>).drop_in_place() }
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

impl<K, V> fmt::Debug for Removed<K, V>
where
    (K, V): fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{:?}", **self)
    }
}

impl<K, V> PartialEq for Removed<K, V>
where
    (K, V): PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<K, V> PartialEq<(K, V)> for Removed<K, V>
where
    (K, V): PartialEq,
{
    fn eq(&self, other: &(K, V)) -> bool {
        **self == *other
    }
}

impl<K, V> Eq for Removed<K, V> where (K, V): Eq {}

impl<K, V> PartialOrd for Removed<K, V>
where
    (K, V): PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<K, V> PartialOrd<(K, V)> for Removed<K, V>
where
    (K, V): PartialOrd,
{
    fn partial_cmp(&self, other: &(K, V)) -> Option<Ordering> {
        (**self).partial_cmp(other)
    }
}

impl<K, V> Ord for Removed<K, V>
where
    (K, V): Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl<K, V> Hash for Removed<K, V>
where
    (K, V): Hash,
{
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        (**self).hash(hasher)
    }
}
