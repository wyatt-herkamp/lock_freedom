#![allow(missing_docs)]

pub use map::RandomState;
use map::{
    Insertion as MapInsertion,
    Iter as MapIter,
    Map,
    Preview,
    ReadGuard as MapGuard,
    Removed as MapRemoved,
};
use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt,
    hash::{BuildHasher, Hash},
    ops::Deref,
};

pub struct Set<T, H = RandomState> {
    inner: Map<T, (), H>,
}

impl<T> Set<T> {
    pub fn new() -> Self {
        Self { inner: Map::new() }
    }
}

impl<T, H> Set<T, H>
where
    H: BuildHasher,
{
    pub fn with_hasher(hasher_builder: H) -> Self {
        Self { inner: Map::with_hasher(hasher_builder) }
    }

    pub fn hasher(&self) -> &H {
        self.inner.hasher()
    }

    pub fn contains<U>(&self, elem: &U) -> bool
    where
        U: Hash + Ord,
        T: Borrow<U>,
    {
        self.inner.get(elem).is_some()
    }

    pub fn get<'origin, U>(
        &'origin self,
        elem: &U,
    ) -> Option<ReadGuard<'origin, T>>
    where
        U: Hash + Ord,
        T: Borrow<U>,
    {
        self.inner.get(elem).map(ReadGuard::new)
    }

    pub fn insert(&self, elem: T) -> Result<(), T>
    where
        T: Hash + Ord,
    {
        let result = self.inner.insert_with(elem, |_, _, stored| {
            if stored.is_some() {
                Preview::Discard
            } else {
                Preview::New(())
            }
        });
        match result {
            MapInsertion::Created => Ok(()),
            MapInsertion::Failed((elem, _)) => Err(elem),
            MapInsertion::Updated(_) => unreachable!(),
        }
    }

    pub fn insert_with<F>(&self, elem: T, mut interactive: F) -> Insertion<T, T>
    where
        F: FnMut(&T, Option<&T>) -> bool,
        T: Hash + Ord,
    {
        let result = self.inner.insert_with(elem, |elem, _, stored| {
            if interactive(elem, stored.map(|(elem, _)| elem)) {
                Preview::New(())
            } else {
                Preview::Discard
            }
        });

        match result {
            MapInsertion::Created => Insertion::Created,
            MapInsertion::Updated(old) => Insertion::Updated(Removed::new(old)),
            MapInsertion::Failed((elem, _)) => Insertion::Failed(elem),
        }
    }

    pub fn reinsert(&self, elem: Removed<T>) -> Result<(), Removed<T>>
    where
        T: Hash + Ord,
    {
        let result =
            self.inner.reinsert_with(elem.inner, |_, stored| stored.is_none());
        match result {
            MapInsertion::Created => Ok(()),
            MapInsertion::Failed(removed) => Err(Removed::new(removed)),
            MapInsertion::Updated(_) => unreachable!(),
        }
    }

    pub fn reinsert_with<F>(
        &self,
        elem: Removed<T>,
        mut interactive: F,
    ) -> Insertion<T, Removed<T>>
    where
        F: FnMut(&T, Option<&T>) -> bool,
        T: Hash + Ord,
    {
        let result =
            self.inner.reinsert_with(elem.inner, |(elem, _), stored| {
                interactive(elem, stored.map(|(elem, _)| elem))
            });

        match result {
            MapInsertion::Created => Insertion::Created,
            MapInsertion::Updated(old) => Insertion::Updated(Removed::new(old)),
            MapInsertion::Failed(e) => Insertion::Failed(Removed::new(e)),
        }
    }

    pub fn remove<U>(&self, elem: &U) -> Option<Removed<T>>
    where
        U: Hash + Ord,
        T: Borrow<U>,
    {
        self.inner.remove(elem).map(Removed::new)
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
        write!(fmtr, "Set {} inner_map: {:?} {}", '{', self.inner, '}')
    }
}

impl<'origin, T, H> IntoIterator for &'origin Set<T, H> {
    type Item = ReadGuard<'origin, T>;

    type IntoIter = Iter<'origin, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { inner: self.inner.into_iter() }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Insertion<T, E> {
    Created,
    Updated(Removed<T>),
    Failed(E),
}

impl<T, E> Insertion<T, E> {
    pub fn created(&self) -> bool {
        match self {
            Insertion::Created => true,
            _ => false,
        }
    }

    pub fn updated(&self) -> Option<&Removed<T>> {
        match self {
            Insertion::Updated(pair) => Some(pair),
            _ => None,
        }
    }

    pub fn take_updated(self) -> Result<Removed<T>, Self> {
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadGuard<'origin, T>
where
    T: 'origin,
{
    inner: MapGuard<'origin, T, ()>,
}

impl<'origin, T> ReadGuard<'origin, T> {
    fn new(inner: MapGuard<'origin, T, ()>) -> Self {
        Self { inner }
    }
}

impl<'origin, T> Deref for ReadGuard<'origin, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.inner.key()
    }
}

impl<'origin, T> fmt::Debug for ReadGuard<'origin, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        (**self).fmt(fmtr)
    }
}

impl<'origin, T> fmt::Display for ReadGuard<'origin, T>
where
    T: fmt::Display,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        (**self).fmt(fmtr)
    }
}

impl<'origin, T> PartialEq<T> for ReadGuard<'origin, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        **self == *other
    }
}

impl<'origin, T> PartialOrd<T> for ReadGuard<'origin, T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        (**self).partial_cmp(other)
    }
}

impl<'origin, T> Borrow<T> for ReadGuard<'origin, T> {
    fn borrow(&self) -> &T {
        self.deref()
    }
}

impl<'origin, T> AsRef<T> for ReadGuard<'origin, T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

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

#[derive(Debug)]
pub struct Iter<'origin, T>
where
    T: 'origin,
{
    inner: MapIter<'origin, T, ()>,
}

impl<'origin, T> Iterator for Iter<'origin, T> {
    type Item = ReadGuard<'origin, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(ReadGuard::new)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{
        cmp::Ordering,
        hash::{Hash, Hasher},
    };

    #[derive(Debug, Clone, Copy)]
    struct EqI {
        i: usize,
        j: usize,
    }

    impl PartialEq for EqI {
        fn eq(&self, other: &Self) -> bool {
            self.i == other.i
        }
    }

    impl Eq for EqI {}

    impl PartialOrd for EqI {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.i.partial_cmp(&other.i)
        }
    }

    impl Ord for EqI {
        fn cmp(&self, other: &Self) -> Ordering {
            self.i.cmp(&other.i)
        }
    }

    impl Hash for EqI {
        fn hash<H>(&self, hasher: &mut H)
        where
            H: Hasher,
        {
            self.i.hash(hasher)
        }
    }

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

    #[test]
    fn inserts_and_reinserts() {
        let set = Set::new();
        set.insert(9).unwrap();
        set.insert(7).unwrap();
        set.insert(0).unwrap();
        let removed = set.remove(&9).unwrap();
        set.reinsert(removed).unwrap();
        set.insert(9).unwrap_err();
    }

    #[test]
    fn insert_with() {
        let set = Set::new();
        set.insert(EqI { i: 32, j: 0 }).unwrap();
        set.insert(EqI { i: 34, j: 10 }).unwrap();
        set.insert(EqI { i: 34, j: 6 }).unwrap_err();
        set.insert_with(EqI { i: 34, j: 6 }, |_, _| true).updated().unwrap();
        set.insert_with(EqI { i: 34, j: 2 }, |_, _| false).failed().unwrap();
        assert!(set.insert_with(EqI { i: 33, j: 2 }, |_, _| true).created());
        set.insert_with(EqI { i: 32, j: 3 }, |_, _| true).updated().unwrap();
    }

    #[test]
    fn reinsert_with() {
        let set = Set::new();
        set.insert(EqI { i: 32, j: 0 }).unwrap();
        set.insert(EqI { i: 34, j: 10 }).unwrap();
        set.insert(EqI { i: 34, j: 6 }).unwrap_err();
        let _34 = set.remove(&EqI { i: 34, j: 325 }).unwrap();
        let _32 = set.remove(&EqI { i: 32, j: 534 }).unwrap();

        set.insert(EqI { i: 34, j: 6 }).unwrap();
        set.reinsert_with(_34, |_, _| true).updated().unwrap();
        let _32 = set.reinsert_with(_32, |_, _| false).take_failed().unwrap();
        assert!(set.reinsert_with(_32, |_, _| true).created());
    }
}
