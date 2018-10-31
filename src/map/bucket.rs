use atomic::AtomicBox;
use owned_alloc::{OwnedAlloc, UninitAlloc};

pub struct Elem<K, V> {
    pair: *mut (K, V),
    next: *mut List<K, V>,
}

pub struct List<K, V> {
    atomic: AtomicBox<Elem<K, V>>,
}

pub enum Garbage<K, V> {
    Pair(OwnedAlloc<(K, V)>),
    List(OwnedAlloc<Bucket<K, V>>),
    Bucket(OwnedAlloc<Bucket<K, V>>),
}

#[repr(align(/* at least */ 2))]
pub struct Bucket<K, V> {
    hash: u64,
    list: List<K, V>,
}
