#![allow(dead_code, missing_docs, unused_imports)]

mod table;
mod bucket;
mod insertion;
mod guard;
mod iter;

pub use self::{
    guard::{ReadGuard, Removed},
    insertion::{Insertion, Preview},
    iter::Iter,
};
pub use std::collections::hash_map::RandomState;

use self::{
    bucket::{Entry, Garbage},
    insertion::{InsertNew, Reinsert},
    table::Table,
};
use atomic::AtomicBoxIncin;
use incin::Incinerator;
use owned_alloc::OwnedAlloc;
use std::{
    borrow::Borrow,
    fmt,
    hash::{BuildHasher, Hash, Hasher},
    sync::Arc,
};

pub struct Map<K, V, H = RandomState> {
    top: OwnedAlloc<Table<K, V>>,
    incin: Arc<Incinerator<Garbage<K, V>>>,
    box_incin: AtomicBoxIncin<Entry<K, V>>,
    builder: H,
}

impl<K, V> Map<K, V> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<K, V, H> Map<K, V, H>
where
    H: BuildHasher,
{
    pub fn with_hasher(builder: H) -> Self {
        Self {
            top: Table::new_alloc(),
            incin: Arc::new(Incinerator::new()),
            box_incin: AtomicBoxIncin::new(),
            builder,
        }
    }

    pub fn get<'origin, Q>(
        &'origin self,
        key: &Q,
    ) -> Option<ReadGuard<'origin, K, V>>
    where
        Q: ?Sized + Hash + Ord,
        K: Borrow<Q>,
    {
        let pause = self.incin.pause();
        let hash = self.hash_of(key);
        let result = unsafe { self.top.get(key, hash, &self.incin) };
        result.map(|pair| ReadGuard::new(pair, pause))
    }

    pub fn insert(&self, key: K, val: V) -> Option<Removed<K, V>>
    where
        K: Hash + Ord,
    {
        let insertion = self.incin.pause_with(|| {
            let hash = self.hash_of(&key);
            unsafe {
                self.top.insert(
                    InsertNew::with_pair(|_, _, _| Preview::Keep, (key, val)),
                    hash,
                    &self.incin,
                    &self.box_incin,
                )
            }
        });

        match insertion {
            Insertion::Created => None,
            Insertion::Updated(old) => Some(old),
            Insertion::Failed(_) => unreachable!(),
        }
    }

    pub fn insert_with<F>(
        &self,
        key: K,
        interactive: F,
    ) -> Insertion<K, V, (K, Option<V>)>
    where
        K: Hash + Ord,
        F: FnMut(&K, Option<&mut V>, Option<&(K, V)>) -> Preview<V>,
    {
        let insertion = self.incin.pause_with(|| {
            let hash = self.hash_of(&key);
            unsafe {
                self.top.insert(
                    InsertNew::with_key(interactive, key),
                    hash,
                    &self.incin,
                    &self.box_incin,
                )
            }
        });

        match insertion {
            Insertion::Created => Insertion::Created,
            Insertion::Updated(old) => Insertion::Updated(old),
            Insertion::Failed(inserter) => {
                Insertion::Failed(inserter.into_pair())
            },
        }
    }

    pub fn reinsert(&self, removed: Removed<K, V>) -> Option<Removed<K, V>>
    where
        K: Hash + Ord,
    {
        let insertion = self.incin.pause_with(|| {
            let hash = self.hash_of(removed.key());
            unsafe {
                self.top.insert(
                    Reinsert::new(|_, _| true, removed),
                    hash,
                    &self.incin,
                    &self.box_incin,
                )
            }
        });

        match insertion {
            Insertion::Created => None,
            Insertion::Updated(old) => Some(old),
            Insertion::Failed(_) => unreachable!(),
        }
    }

    pub fn reinsert_with<F>(
        &self,
        removed: Removed<K, V>,
        interactive: F,
    ) -> Insertion<K, V, Removed<K, V>>
    where
        K: Hash + Ord,
        F: FnMut(&(K, V), Option<&(K, V)>) -> bool,
    {
        let insertion = self.incin.pause_with(|| {
            let hash = self.hash_of(removed.key());
            unsafe {
                self.top.insert(
                    Reinsert::new(interactive, removed),
                    hash,
                    &self.incin,
                    &self.box_incin,
                )
            }
        });

        match insertion {
            Insertion::Created => Insertion::Created,
            Insertion::Updated(old) => Insertion::Updated(old),
            Insertion::Failed(inserter) => {
                Insertion::Failed(inserter.into_removed())
            },
        }
    }

    pub fn remove<Q>(&self, key: &Q) -> Option<Removed<K, V>>
    where
        Q: ?Sized + Hash + Ord,
        K: Borrow<Q>,
    {
        self.remove_with(key, |_| true)
    }

    pub fn remove_with<Q, F>(
        &self,
        key: &Q,
        interactive: F,
    ) -> Option<Removed<K, V>>
    where
        Q: ?Sized + Hash + Ord,
        K: Borrow<Q>,
        F: FnMut(&(K, V)) -> bool,
    {
        self.incin.pause_with(|| {
            let hash = self.hash_of(key);
            unsafe { self.top.remove(key, interactive, hash, &self.incin) }
        })
    }

    pub fn optimize_space(&mut self) {
        self.incin.try_clear();
        self.top.optimize_space();
    }

    pub fn clear(&mut self) {
        self.incin.try_clear();

        let mut tables = Vec::new();

        self.top.clear(&mut tables);

        while let Some(mut table) = tables.pop() {
            table.free_nodes(&mut tables);
        }
    }

    fn hash_of<Q>(&self, key: &Q) -> u64
    where
        Q: ?Sized + Hash,
    {
        let mut hasher = self.builder.build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
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

impl<K, V, H> fmt::Debug for Map<K, V, H>
where
    H: fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "Map {} top_table: {:?}, incin: {:?}, box_incin: {:?}, \
             build_hasher: {:?}  {}",
            '{', self.top, self.incin, self.box_incin, self.builder, '}'
        )
    }
}

impl<K, V, H> Drop for Map<K, V, H> {
    fn drop(&mut self) {
        let mut tables = Vec::new();

        self.top.free_nodes(&mut tables);

        while let Some(mut table) = tables.pop() {
            table.free_nodes(&mut tables);
        }
    }
}

impl<'origin, K, V, H> IntoIterator for &'origin Map<K, V, H> {
    type Item = ReadGuard<'origin, K, V>;

    type IntoIter = Iter<'origin, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self.incin.pause(), &self.top)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{collections::HashMap, sync::Arc, thread};

    #[test]
    fn inserts_and_gets() {
        let map = Map::new();
        assert!(map.get("five").is_none());
        assert!(map.insert("five".to_owned(), 5).is_none());
        assert_eq!(*map.get("five").unwrap().val(), 5);
        assert!(map.get("four").is_none());
        assert!(map.insert("four".to_owned(), 4).is_none());
        assert_eq!(*map.get("five").unwrap().val(), 5);
        assert_eq!(*map.get("four").unwrap().val(), 4);
        let guard = map.get("four").unwrap();
        assert_eq!(guard.key(), "four");
        assert_eq!(*guard.val(), 4);
    }

    #[test]
    fn create() {
        let map = Map::new();
        assert!(map
            .insert_with(
                "five".to_owned(),
                |_, _, stored| if stored.is_none() {
                    Preview::New(5)
                } else {
                    Preview::Discard
                }
            )
            .created());
        assert_eq!(*map.get("five").unwrap().val(), 5);
        assert!(map
            .insert_with(
                "five".to_owned(),
                |_, _, stored| if stored.is_none() {
                    Preview::New(500)
                } else {
                    Preview::Discard
                }
            )
            .failed()
            .is_some());
    }

    #[test]
    fn update() {
        let map = Map::new();
        assert!(map
            .insert_with("five".to_owned(), |_, _, stored| {
                if let Some((_, n)) = stored {
                    Preview::New(*n + 6)
                } else {
                    Preview::Discard
                }
            })
            .failed()
            .is_some());
        assert!(map.insert("five".to_owned(), 5).is_none());
        let guard = map
            .insert_with("five".to_owned(), |_, _, stored| {
                if let Some((_, n)) = stored {
                    Preview::New(*n + 7)
                } else {
                    Preview::Discard
                }
            })
            .take_updated()
            .unwrap();
        assert_eq!(guard.key(), "five");
        assert_eq!(*guard.val(), 5);
        assert_eq!(*map.get("five").unwrap().val(), 12);
    }

    #[test]
    fn never_inserts() {
        let map = Map::new();
        assert!(map
            .insert_with("five".to_owned(), |_, _, _| Preview::Discard)
            .failed()
            .is_some());
        assert!(map.insert("five".to_owned(), 5).is_none());
        assert!(map
            .insert_with("five".to_owned(), |_, _, _| Preview::Discard)
            .failed()
            .is_some());
    }

    #[test]
    fn inserts_reinserts() {
        let map = Map::new();
        assert!(map.insert("four".to_owned(), 4).is_none());
        let prev = map.insert("four".to_owned(), 40).unwrap();
        assert_eq!(prev.key(), "four");
        assert_eq!(*prev.val(), 4);
        let prev = map.reinsert(prev).unwrap();
        assert_eq!(prev.key(), "four");
        assert_eq!(*prev.val(), 40);
        assert!(*map.get("four").unwrap().val() == 4);
    }

    #[test]
    fn never_reinserts() {
        let map = Map::new();
        map.insert("five".to_owned(), 5);
        let prev = map.remove("five").unwrap();
        let prev = map.reinsert_with(prev, |_, _| false).take_failed().unwrap();
        assert!(map.insert("five".to_owned(), 5).is_none());
        map.reinsert_with(prev, |_, _| false).take_failed().unwrap();
    }

    /*
    #[test]
    fn reinserts_create() {
        let map = Map::new();
        map.insert("five".to_owned(), 5);
        let first = map.remove("five").unwrap();
        map.insert("five".to_owned(), 5);
        let second = map.remove("five").unwrap();
        assert!(map
            .reinsert_with(first, |_, stored| stored.is_none())
            .created());
        assert_eq!(map.get("five", |x| *x), Some(5));
        assert!(map
            .reinsert_with(second, |_, stored| stored.is_none())
            .failed()
            .is_some());
    }

    #[test]
    fn reinserts_update() {
        let map = Map::new();
        map.insert("five".to_owned(), 5);
        let prev = map.remove("five").unwrap();
        let prev = map
            .reinsert_with(prev, |_, stored| stored.is_some())
            .take_failed()
            .unwrap();
        map.insert("five".to_owned(), 5);
        assert!(map
            .reinsert_with(prev, |_, stored| stored.is_some())
            .updated()
            .is_some());
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
    fn iter_valid_items() {
        let map = Map::new();
        for i in 0 .. 10u128 {
            for j in 0 .. 32 {
                map.insert((i, j), i << j);
            }
        }

        let mut result = HashMap::new();
        for (k, v) in map.iter(|&k, &v| (k, v)) {
            let in_place = result.get(&(k, v)).map_or(0, |&x| x);
            result.insert((k, v), in_place + 1);
        }

        for i in 0 .. 10 {
            for j in 0 .. 32 {
                let pair = ((i, j), i << j);
                assert_eq!(*result.get(&pair).unwrap(), 1);
            }
        }
    }

    #[test]
    fn remove_unneeded_preserves_needed() {
        let mut map = Map::new();
        for i in 0 .. 200u128 {
            for j in 0 .. 128 {
                map.insert((i, j), i << j);
            }
        }

        for i in 0 .. 200 {
            for j in 0 .. 16 {
                map.remove(&(i, j));
            }
        }

        map.remove_unneeded_tables();

        let mut result = HashMap::new();
        for (k, v) in map.iter(|&k, &v| (k, v)) {
            let in_place = result.get(&(k, v)).map_or(0, |&x| x);
            result.insert((k, v), in_place + 1);
        }

        for i in 0 .. 200 {
            for j in 16 .. 128 {
                let pair = ((i, j), i << j);
                assert_eq!(*result.get(&pair).unwrap(), 1);
            }
        }
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
                map.insert_with(
                    format!("prefix{}suffix", i + 1),
                    |_, stored, _| Preview::New(stored.map_or(0, |&x| x + i)),
                );
            }));
        }
        for thread in threads {
            thread.join().expect("thread failed");
        }
        for i in 1i64 ..= 20 {
            assert!(map
                .get(&format!("prefix{}suffix", i), |x| *x > 0)
                .unwrap());
        }
    }*/
}
