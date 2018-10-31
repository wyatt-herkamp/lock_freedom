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
    bucket::{Elem, Garbage},
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

    pub fn get<'origin, Q>(
        &'origin self,
        key: &Q,
    ) -> Option<ReadGuard<'origin, K, V>>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>,
    {
        let pause = self.incin.pause();
        let result = unsafe { self.top.get(key, self.hash_of(key)) };
        result.map(|pair| ReadGuard { pair, pause })
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
