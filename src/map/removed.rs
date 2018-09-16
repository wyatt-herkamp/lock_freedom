use super::bucket::Pair;
use alloc::*;
use incinerator;
use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    ptr::NonNull,
};

/// A removed entry. Although the entry allows the user to immutable access key
/// and value, it does not allow moving them. This is because it cannot be
/// dropped by the user. Imagine that a thread would remove and drop (by user
/// defined code) the entry after another thread began would be reading, but,
/// in the moment of the drop, still reading. This would cause use-after-free.
#[derive(Eq)]
pub struct Removed<K, V> {
    pair: NonNull<Pair<K, V>>,
}

impl<K, V> Removed<K, V> {
    pub(crate) unsafe fn new(pair: NonNull<Pair<K, V>>) -> Self {
        Self { pair }
    }

    pub(crate) fn ptr(&self) -> NonNull<Pair<K, V>> {
        self.pair
    }

    /// The key of this removed entry.
    pub fn key(&self) -> &K {
        &unsafe { self.pair.as_ref() }.key
    }

    /// The value of this removed entry.
    pub fn val(&self) -> &V {
        &unsafe { self.pair.as_ref() }.val
    }
}

impl<K, V> Drop for Removed<K, V> {
    fn drop(&mut self) {
        unsafe { incinerator::add(self.pair, dealloc) }
    }
}

impl<K, V> fmt::Debug for Removed<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "Removed {} key = {:?}, val = {:?} {}",
            '{',
            self.key(),
            self.val(),
            '}'
        )
    }
}

impl<K, V, Q, U> PartialEq<Removed<Q, U>> for Removed<K, V>
where
    K: PartialEq<Q>,
    V: PartialEq<U>,
{
    fn eq(&self, other: &Removed<Q, U>) -> bool {
        self.key() == other.key() && self.val() == other.val()
    }
}

impl<K, V, Q, U> PartialEq<(Q, U)> for Removed<K, V>
where
    K: PartialEq<Q>,
    V: PartialEq<U>,
{
    fn eq(&self, (key, val): &(Q, U)) -> bool {
        self.key() == key && self.val() == val
    }
}

impl<K, V, Q, U> PartialOrd<Removed<Q, U>> for Removed<K, V>
where
    K: PartialOrd<Q>,
    V: PartialOrd<U>,
{
    fn partial_cmp(&self, other: &Removed<Q, U>) -> Option<Ordering> {
        self.key().partial_cmp(other.key()).and_then(|ord_a| {
            self.val().partial_cmp(other.val()).map(|ord_b| ord_a.then(ord_b))
        })
    }
}

impl<K, V, Q, U> PartialOrd<(Q, U)> for Removed<K, V>
where
    K: PartialOrd<Q>,
    V: PartialOrd<U>,
{
    fn partial_cmp(&self, (key, val): &(Q, U)) -> Option<Ordering> {
        self.key().partial_cmp(key).and_then(|ord_a| {
            self.val().partial_cmp(val).map(|ord_b| ord_a.then(ord_b))
        })
    }
}

impl<K, V> Ord for Removed<K, V>
where
    K: Ord,
    V: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.key().cmp(other.key()).then_with(|| self.val().cmp(other.val()))
    }
}

impl<K, V> Hash for Removed<K, V>
where
    K: Hash,
    V: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.key().hash(state);
        self.val().hash(state);
    }
}
