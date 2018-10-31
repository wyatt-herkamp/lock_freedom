#![allow(dead_code, missing_docs, unused_imports)]

mod table;
mod bucket;
mod insertion;
mod removed;

pub use self::removed::Removed;
pub use std::collections::hash_map::RandomState;

use self::{
    bucket::{Elem, Garbage},
    insertion::{Insertion, Preview},
    table::Table,
};
use atomic::AtomicBoxIncin;
use incin::Incinerator;
use owned_alloc::OwnedAlloc;
use std::{
    hash::{BuildHasher, Hash, Hasher},
    sync::Arc,
};

pub struct Map<K, V, H = RandomState> {
    top: OwnedAlloc<Table<K, V>>,
    incin: Arc<Incinerator<Garbage<K, V>>>,
    box_incin: AtomicBoxIncin<Elem<K, V>>,
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
