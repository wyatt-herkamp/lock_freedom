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
use std::sync::Arc;

pub struct Map<K, V, H = RandomState> {
    top: OwnedAlloc<Table<K, V>>,
    incin: Arc<Incinerator<Garbage<K, V>>>,
    box_incin: AtomicBoxIncin<Elem<K, V>>,
    builder: H,
}
