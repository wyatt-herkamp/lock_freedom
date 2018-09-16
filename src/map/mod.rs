mod table;
mod bucket;
mod insertion;
mod removed;

pub use self::{
    insertion::{Insertion, Preview},
    removed::Removed,
};

use alloc::*;
use atomic::{Atomic, AtomicBox};
use incinerator;
use std::{
    borrow::Borrow,
    collections::hash_map::RandomState,
    fmt,
    hash::{BuildHasher, Hash, Hasher},
    mem,
    ptr::{null_mut, NonNull},
    sync::atomic::Ordering::*,
};

/// A lock-free map. Implemented using multi-level hash-tables (in a tree
/// fashion) with ordered buckets.
///
/// # Design
/// In order to implement this map, we shall fix a constant named `BITS`, which
/// should be smaller than the number of bits in the hash. We chose `8` for it.
/// Now, we define a table structure: an array of nodes with length `1 << BITS`
/// (`256` in this case).
///
/// For inserting, we take the first `BITS` bits of the hash. Now, we verify
/// the node. If it is empty, insert a new bucket with our entry (a leaf of the
/// tree), and assign our hash to the bucket. If there is a branch (i.e. a
/// sub-table), we shift the hash `BITS` bits to the left, but we also keep the
/// original hash for consultation. Then we try again in the sub-table. If
/// there is another leaf, and if the hash of the leaf's bucket is equal to
/// ours, we insert our entry into the bucket. If the hashes are not equal, we
/// create a sub-table, insert the old leaf into the new sub-table, and insert
/// our pair after.
///
/// Entries in a bucket are a single linked list ordered by key. The ordering
/// of the list is because of possible race conditions if e.g. new nodes were
/// always inserted at end. And if a bucket is detected to be empty, the
/// table will be requested to delete the bucket.
///
/// For searching, in a similar way, the hash is shifted and sub-tables are
/// entered until either a node is empty or a leaf is found. If the hash of the
/// leaf's bucket is equal to our hash, we search for the entry into the bucket.
/// Because the bucket is ordered, we may know the entry is not present with
/// ease.
///
/// Because of limitation of sharing in concurrent contexts, we do return
/// references to the entries, neither allow the user to move out removed
/// values, as they must be deinitialized correctly. Returning references would
/// also imply pausing the deallocation of sensitive resources for indefinite
/// time.
pub struct Map<K, V, H = RandomState> {
    table: table::Table<K, V>,
    builder: H,
}

impl<K, V> Map<K, V, RandomState> {
    /// Creates a new empty map with a random state.
    pub fn new() -> Self {
        Self::with_hasher(RandomState::default())
    }
}

impl<K, V, H> Map<K, V, H> {
    /// Creates a new empty map with a hash builder.
    pub fn with_hasher(builder: H) -> Self
    where
        H: BuildHasher,
    {
        Self { table: table::Table::new(), builder }
    }

    /// Sets the mapped value of a key, disregarding it exists or not. If it
    /// does exists, the old pair is removed and returned.
    pub fn insert(&self, key: K, val: V) -> Option<Removed<K, V>>
    where
        K: Hash + Ord,
        H: BuildHasher,
    {
        let hash = self.hash_of(&key);
        incinerator::pause(|| unsafe {
            let ptr = alloc(bucket::Pair { key, val });
            let res = self.table.insert(ptr, hash);
            NonNull::new(res).map(|x| Removed::new(x))
        })
    }

    /// Reinserts a removed pair (which can have been removed from any map),
    /// disregarding the key entry exists or not. If it does exists, the
    /// old pair is removed and returned.
    pub fn reinsert(&self, removed: Removed<K, V>) -> Option<Removed<K, V>>
    where
        K: Hash + Ord,
        H: BuildHasher,
    {
        let hash = self.hash_of(removed.key());
        incinerator::pause(|| unsafe {
            let pair = removed.pair_ptr();
            mem::forget(removed);
            let res = self.table.insert(pair, hash);
            NonNull::new(res).map(|x| Removed::new(x))
        })
    }

    /// Gets a reference to the mapped value of a key, it exists. Then, it
    /// calls the `reader` function argument with the reference. Please note
    /// that returning a reference would imply in pausing any sensitive
    /// incinerator resource deallocation for indefinite time.
    pub fn get<Q, F, T>(&self, key: &Q, reader: F) -> Option<T>
    where
        Q: Hash + Ord + ?Sized,
        K: Borrow<Q>,
        H: BuildHasher,
        F: FnOnce(&V) -> T,
    {
        let hash = self.hash_of(key);
        incinerator::pause(|| unsafe {
            let res = self.table.get(key, hash);
            res.as_ref().map(|x| reader(&x.val))
        })
    }

    /// Same as `get`, but calls the `reader` function argument with key and
    /// value, respectively, instead.
    pub fn get_pair<Q, F, T>(&self, key: &Q, reader: F) -> Option<T>
    where
        Q: Hash + Ord + ?Sized,
        K: Borrow<Q>,
        H: BuildHasher,
        F: FnOnce(&K, &V) -> T,
    {
        let hash = self.hash_of(key);
        incinerator::pause(|| unsafe {
            let res = self.table.get(key, hash);
            res.as_ref().map(|x| reader(&x.key, &x.val))
        })
    }

    /// Removes the given entry identified by the given key.
    pub fn remove<Q>(&self, key: &Q) -> Option<Removed<K, V>>
    where
        Q: Hash + Ord + ?Sized,
        K: Borrow<Q>,
        H: BuildHasher,
    {
        let hash = self.hash_of(key);
        incinerator::pause(|| unsafe {
            let res = self.table.remove(key, hash);
            NonNull::new(res).map(|x| Removed::new(x))
        })
    }

    #[inline]
    fn hash_of<Q>(&self, key: &Q) -> u64
    where
        Q: Hash + ?Sized,
        H: BuildHasher,
    {
        let mut hasher = self.builder.build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
    }
}

impl<K, V, H> Drop for Map<K, V, H> {
    fn drop(&mut self) {
        let mut node_ptrs = Vec::new();
        for node in self.table.nodes() {
            let loaded = node.load(Acquire);
            if let Some(nnptr) = NonNull::new(loaded) {
                node_ptrs.push(nnptr);
            }
        }

        while let Some(node_ptr) = node_ptrs.pop() {
            match unsafe { node_ptr.as_ref() } {
                table::Node::Leaf(bucket) => {
                    let mut list = bucket.list().load(Relaxed).next();
                    while let Some(nnptr) = NonNull::new(list) {
                        let entry = unsafe { nnptr.as_ref().load(Relaxed) };
                        if let Some(nnptr) = NonNull::new(entry.pair()) {
                            unsafe { dealloc(nnptr) }
                        }
                        unsafe { dealloc(nnptr) }
                        list = entry.next();
                    }
                },

                table::Node::Branch(table) => {
                    for node in unsafe { table.as_ref().nodes() } {
                        let loaded = node.load(Acquire);
                        if let Some(nnptr) = NonNull::new(loaded) {
                            node_ptrs.push(nnptr);
                        }
                    }
                    unsafe { dealloc(*table) }
                },
            }

            unsafe { dealloc(node_ptr) }
        }
    }
}

impl<K, V, H> Default for Map<K, V, H>
where
    H: BuildHasher + Default,
{
    fn default() -> Self {
        Self::with_hasher(H::default())
    }
}

unsafe impl<K, V, H> Send for Map<K, V, H>
where
    K: Send + Sync,
    V: Send + Sync,
    H: Send,
{}

unsafe impl<K, V, H> Sync for Map<K, V, H>
where
    K: Send + Sync,
    V: Send + Sync,
    H: Sync,
{}

impl<K, V, H> fmt::Debug for Map<K, V, H>
where
    H: fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "Map {} hasher_builder = {:?}, entries = ... {}",
            '{', self.builder, '}'
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{sync::Arc, thread};

    #[test]
    fn inserts_and_gets() {
        let map = Map::new();
        assert_eq!(map.get("five", |x| *x), None);
        assert!(map.insert("five".to_owned(), 5).is_none());
        assert_eq!(map.get("five", |x| *x), Some(5));
        assert_eq!(map.get("four", |x| *x), None);
        assert!(map.insert("four".to_owned(), 4).is_none());
        assert_eq!(map.get("five", |x| *x), Some(5));
        assert_eq!(map.get("four", |x| *x), Some(4));
        map.get_pair("four", |k, v| {
            assert_eq!(k, "four");
            assert_eq!(*v, 4);
        });
    }

    #[test]
    fn inserts_reinserts() {
        let map = Map::new();
        assert!(map.insert("four".to_owned(), 4).is_none());
        let prev = map.insert("four".to_owned(), 40).unwrap();
        assert_eq!(prev, ("four", 4));
        assert_eq!(map.reinsert(prev).unwrap(), ("four", 40));
        assert!(map.get("four", |&x| x == 4).unwrap());
    }

    #[test]
    fn inserts_and_removes() {
        let map = Map::new();
        assert!(map.remove("five").is_none());
        assert!(map.remove("four").is_none());
        map.insert("five".to_owned(), 5);
        let removed = map.remove("five").unwrap();
        assert_eq!(removed, ("five", 5));
        assert!(map.insert("four".to_owned(), 4).is_none());
        map.insert("three".to_owned(), 3);
        assert!(map.remove("two").is_none());
        map.insert("two".to_owned(), 2);
        let removed = map.remove("three").unwrap();
        assert_eq!(removed, ("three", 3));
        let removed = map.remove("two").unwrap();
        assert_eq!(removed, ("two", 2));
        let removed = map.remove("four").unwrap();
        assert_eq!(removed, ("four", 4));
    }

    #[test]
    fn repeated_inserts() {
        let map = Map::new();
        assert!(map.insert("five".to_owned(), 5).is_none());
        assert!(*map.insert("five".to_owned(), 5).unwrap().val() == 5);
    }

    #[test]
    fn multithreaded() {
        let map = Arc::new(Map::new());
        let mut threads = Vec::new();
        for i in 1i64 ..= 20 {
            let map = map.clone();
            threads.push(thread::spawn(move || {
                let prev = map
                    .get(&format!("prefix{}suffix", i - 1), |x| *x)
                    .unwrap_or(0);
                map.insert(format!("prefix{}suffix", i), prev + i);
            }));
        }
        for thread in threads {
            thread.join().expect("thread failed");
        }
        for i in 1i64 ..= 20 {
            assert!(
                map.get(&format!("prefix{}suffix", i), |x| *x > 0).unwrap()
            );
        }
    }
}
