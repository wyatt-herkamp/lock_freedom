#![allow(dead_code, missing_docs, unused_imports)]

mod table;
mod bucket;
mod insertion;
mod removed;
mod guard;

pub use self::{
    guard::ReadGuard,
    insertion::{Insertion, Preview},
    removed::Removed,
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

    pub fn insert<F>(&self, key: K, val: V) -> Option<Removed<K, V>>
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

    pub fn remove<Q, F>(&self, key: &Q) -> Option<Removed<K, V>>
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
