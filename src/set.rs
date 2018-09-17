pub use map::RandomState;
use map::{Insertion, Iter as MapIter, Map, Preview, Removed as MapRemoved};
use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt,
    hash::{BuildHasher, Hash},
    ops::Deref,
};

/// A lockfree set. Implemented using multi-level hash-tables (in a tree
/// fashion) with ordered buckets. For more implementation details, see
/// `Map` documentation.
pub struct Set<T, H = RandomState> {
    inner: Map<T, (), H>,
}

impl<T> Set<T> {
    /// Creates this set with random state.
    pub fn new() -> Self {
        Self { inner: Map::new() }
    }
}

impl<T, H> Set<T, H> {
    /// Creates this set with a hasher builder.
    pub fn with_hasher(hasher_builder: H) -> Self
    where
        H: BuildHasher,
    {
        Self { inner: Map::with_hasher(hasher_builder) }
    }

    /// Inserts an element into the set. If the element was already present,
    /// the provided element value is returned back as an error.
    pub fn insert(&self, elem: T) -> Result<(), T>
    where
        H: BuildHasher,
        T: Hash + Ord,
    {
        let res = self.inner.insert_with(elem, |_, stored, _| {
            if stored.is_none() {
                Preview::New(())
            } else {
                Preview::Discard
            }
        });

        match res {
            Insertion::Created => Ok(()),
            Insertion::Failed((elem, _)) => Err(elem),
            Insertion::Updated(_) => unreachable!(),
        }
    }

    /// Reinserts a previously removed element into the set. The element may
    /// have been removed from another set. If the element is already present,
    /// the provided value is returned back as an error.
    pub fn reinsert(&self, elem: Removed<T>) -> Result<(), Removed<T>>
    where
        H: BuildHasher,
        T: Hash + Ord,
    {
        let res =
            self.inner.reinsert_with(elem.inner, |_, stored| stored.is_none());

        match res {
            Insertion::Created => Ok(()),
            Insertion::Failed(elem) => Err(Removed::new(elem)),
            Insertion::Updated(_) => unreachable!(),
        }
    }

    /// Tests whether an element is present or not. For this method to work,
    /// besides being borrowable from `T`, `U` must implement `Hash` in the
    /// same way as `T`.
    pub fn contains<U>(&self, elem: &U) -> bool
    where
        H: BuildHasher,
        U: Hash + Ord,
        T: Borrow<U>,
    {
        self.inner.get(elem, |_| ()).is_some()
    }

    /// Removes the element from the set. For this method to work,
    /// besides being borrowable from `T`, `U` must implement `Hash` in the
    /// same way as `T`.
    pub fn remove<U>(&self, elem: &U) -> Option<Removed<T>>
    where
        H: BuildHasher,
        U: Hash + Ord,
        T: Borrow<U>,
    {
        self.inner.remove(elem).map(Removed::new)
    }

    /// The hasher builder with which this set was created.
    pub fn hasher(&self) -> &H {
        self.inner.hasher()
    }
}

impl<T, H> Default for Set<T, H>
where
    H: BuildHasher + Default,
{
    fn default() -> Self {
        Self { inner: Map::default() }
    }
}

impl<T, H> fmt::Debug for Set<T, H>
where
    H: fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "Set {} hasher_builder = {:?}, elements = ... {}",
            '{',
            self.inner.hasher(),
            '}'
        )
    }
}

/// A removed entry from the set. It is reinsertable. Note that because it is
/// shared between threads, it is not safe to be moved out without proper
/// specific deallocation.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Removed<T> {
    inner: MapRemoved<T, ()>,
}

impl<T> Removed<T> {
    fn new(inner: MapRemoved<T, ()>) -> Self {
        Self { inner }
    }
}

impl<T> Deref for Removed<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.inner.key()
    }
}

impl<T> fmt::Debug for Removed<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        (**self).fmt(fmtr)
    }
}

impl<T> fmt::Display for Removed<T>
where
    T: fmt::Display,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        (**self).fmt(fmtr)
    }
}

impl<T> PartialEq<T> for Removed<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        **self == *other
    }
}

impl<T> PartialOrd<T> for Removed<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        (**self).partial_cmp(other)
    }
}

impl<T> Borrow<T> for Removed<T> {
    fn borrow(&self) -> &T {
        self.deref()
    }
}

impl<T> AsRef<T> for Removed<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

/// An iterator over `Set`. The `Item` of this iterator is a vector of set
/// elements with the same hash. It is like that because there are some
/// limitations on how the iterator can stop navigating the set.
pub struct Iter<'set, T>
where
    T: 'set,
{
    inner: MapIter<'set, T, ()>,
}

impl<'set, T> Iterator for Iter<'set, T> {
    type Item = Vec<&'set T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|vec| vec.into_iter().map(|(k, _)| k).collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn inserts_and_contains_checks() {
        let set = Set::new();
        assert!(!set.contains(&3));
        assert!(!set.contains(&5));
        set.insert(3).unwrap();
        assert!(set.contains(&3));
        assert!(!set.contains(&5));
        set.insert(3).unwrap_err();
        assert!(set.contains(&3));
        assert!(!set.contains(&5));
        set.insert(5).unwrap();
        assert!(set.contains(&3));
        assert!(set.contains(&5));
    }

    #[test]
    fn inserts_and_removes() {
        let set = Set::new();
        assert!(set.remove(&7).is_none());
        set.insert(7).unwrap();
        assert_eq!(set.remove(&7).unwrap(), 7);
        assert!(set.remove(&7).is_none());
        set.insert(3).unwrap();
        set.insert(5).unwrap();
        assert_eq!(set.remove(&5).unwrap(), 5);
        assert_eq!(set.remove(&3).unwrap(), 3);
        assert!(set.remove(&3).is_none());
        assert!(set.remove(&5).is_none());
    }
}
