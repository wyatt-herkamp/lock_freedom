pub use map::RandomState;
use map::{
    Insertion as MapInsertion,
    Iter as MapIter,
    IterReader as MapIterReader,
    Map,
    Preview,
    Removed as MapRemoved,
};
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

    /// Inserts an element into the set. If the element is already present,
    /// nothing is changed and an error is returned with the element passed by
    /// argument.
    pub fn insert(&self, elem: T) -> Result<(), T>
    where
        T: Hash + Ord,
        H: BuildHasher,
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

    /// An _interactive_ insertion. The element to be inserted is passed with a
    /// checker function. The first argument of the function is the provided
    /// element, while the second is the already present element, equal to
    /// the provided, if any. Then, the function returns whether to insert or
    /// not, given the conditions.
    pub fn insert_with<F>(&self, elem: T, mut checker: F) -> Insertion<T, T>
    where
        F: FnMut(&T, Option<&T>) -> bool,
        T: Hash + Ord,
        H: BuildHasher,
    {
        let result = self.inner.insert_with(elem, |elem, _, stored| {
            if checker(elem, stored.map(|(elem, _)| elem)) {
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

    /// Reinserts a previously removed element from the set. If the element is
    /// already present, nothing is changed and an error is returned with the
    /// element passed by argument.
    pub fn reinsert(&self, elem: Removed<T>) -> Result<(), Removed<T>>
    where
        T: Hash + Ord,
        H: BuildHasher,
    {
        let result =
            self.inner.reinsert_with(elem.inner, |_, stored| stored.is_none());
        match result {
            MapInsertion::Created => Ok(()),
            MapInsertion::Failed(removed) => Err(Removed::new(removed)),
            MapInsertion::Updated(_) => unreachable!(),
        }
    }

    /// An _interactive_ reinsertion. The element to be inserted (previously
    /// removed) is passed with a checker function. The first argument of
    /// the function is the provided element, while the second is the
    /// already present element, equal to the provided, if any. Then, the
    /// function returns whether to insert or not, given the conditions.
    pub fn reinsert_with<F>(
        &self,
        elem: Removed<T>,
        mut checker: F,
    ) -> Insertion<T, Removed<T>>
    where
        F: FnMut(&T, Option<&T>) -> bool,
        T: Hash + Ord,
        H: BuildHasher,
    {
        let result = self.inner.reinsert_with(elem.inner, |elem, stored| {
            checker(elem.key(), stored.map(|(elem, _)| elem))
        });

        match result {
            MapInsertion::Created => Insertion::Created,
            MapInsertion::Updated(old) => Insertion::Updated(Removed::new(old)),
            MapInsertion::Failed(e) => Insertion::Failed(Removed::new(e)),
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

    /// Gets the element into a temporary reference to be read by a provided
    /// closure. The return of the closure is wrapped into a `Some`, and if
    /// there was no element, `None` is returned.
    pub fn get<U, F, A>(&self, elem: &U, reader: F) -> Option<A>
    where
        H: BuildHasher,
        U: Hash + Ord,
        T: Borrow<U>,
        F: FnOnce(&T) -> A,
    {
        self.inner.get_pair(elem, |k, _| reader(k))
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

    /// Iterates over the set with a given reader. The reader must be a
    /// function/closure. This methods is just a specific version of
    /// `iter_with_reader` due to Rust limitations on inference of closures
    /// polymorphic on lifetimes.
    pub fn iter<'set, F, U>(&'set self, reader: F) -> Iter<'set, T, F>
    where
        F: FnMut(&T) -> U,
    {
        self.iter_with_reader(reader)
    }

    /// Iterates over the set with a given reader. The reader may be a closure,
    /// primitive function, or any other type that implements the reader trait.
    pub fn iter_with_reader<'set, R>(&'set self, reader: R) -> Iter<'set, T, R>
    where
        R: IterReader<T>,
    {
        Iter { inner: self.inner.iter_with_reader(BridgeReader { reader }) }
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

/// The result of an _interactive_ insertion.
#[derive(Debug, PartialEq, Eq)]
pub enum Insertion<T, E> {
    /// If this is returned, an entry for the given element was successfully
    /// created i.e. there was no entry before.
    Created,
    /// If this is returned, an entry for the given element already existed and
    /// was successfully updated with the provided value. The field is the old
    /// entry element.
    Updated(Removed<T>),
    /// If this is returned, the insertion failed and no action was done.
    /// Failure may have happened because the given closure rejected the
    /// conditions. The field will depend on the method you called.
    Failed(E),
}

impl<T, E> Insertion<T, E> {
    /// Is this insertion a creation?
    pub fn created(&self) -> bool {
        match self {
            Insertion::Created => true,
            _ => false,
        }
    }

    /// Is this insertion an update? If so, the return is a reference to the
    /// old value.
    pub fn updated(&self) -> Option<&Removed<T>> {
        match self {
            Insertion::Updated(pair) => Some(pair),
            _ => None,
        }
    }

    /// Is this insertion an update? If so, the old value is taken and
    /// returned. Otherwise, the insertion is returned.
    pub fn take_updated(self) -> Result<Removed<T>, Self> {
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

/// An iterator over `Set`. As in the `Map`'s `get` method, we cannot return a
/// reference to the elements. Therefore, the iterator has a reader which reads
/// a temporary reference and returns the actual iterator `Item`.
pub struct Iter<'set, T, R>
where
    T: 'set,
    R: IterReader<T>,
{
    inner: MapIter<'set, T, (), BridgeReader<R>>,
}

impl<'set, T, R> Iterator for Iter<'set, T, R>
where
    R: IterReader<T>,
{
    type Item = R::Out;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'set, T, R> fmt::Debug for Iter<'set, T, R>
where
    R: IterReader<T>,
    R::Out: fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(fmtr)
    }
}

/// A reader on set's iterator's temporary reference to elements.
pub trait IterReader<T> {
    /// The actual item of the iterator which is coupled with this reader.
    type Out;

    /// Transforms the temporary references into the iterator item.
    fn read(&mut self, elem: &T) -> Self::Out;
}

impl<T, F, U> IterReader<T> for F
where
    F: FnMut(&T) -> U,
{
    type Out = U;

    fn read(&mut self, elem: &T) -> Self::Out {
        self(elem)
    }
}

#[derive(Debug)]
struct BridgeReader<R> {
    reader: R,
}

impl<T, R> MapIterReader<T, ()> for BridgeReader<R>
where
    R: IterReader<T>,
{
    type Out = R::Out;

    fn read(&mut self, elem: &T, _: &()) -> Self::Out {
        self.reader.read(elem)
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
