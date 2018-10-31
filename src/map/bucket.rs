use atomic::AtomicBox;
use owned_alloc::{OwnedAlloc, UninitAlloc};
use std::fmt;

pub struct Elem<K, V> {
    pair: *mut (K, V),
    next: *mut List<K, V>,
}

pub struct List<K, V> {
    atomic: AtomicBox<Elem<K, V>>,
}

#[repr(align(/* at least */ 2))]
pub struct Bucket<K, V> {
    hash: u64,
    list: List<K, V>,
}

pub enum Garbage<K, V> {
    Pair(OwnedAlloc<(K, V)>),
    List(OwnedAlloc<Bucket<K, V>>),
    Bucket(OwnedAlloc<Bucket<K, V>>),
}

impl<K, V> fmt::Debug for Garbage<K, V> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Garbage::Pair(ptr) => write!(fmtr, "Garbage::Pair({:?})", ptr),
            Garbage::List(ptr) => write!(fmtr, "Garbage::List({:?})", ptr),
            Garbage::Bucket(ptr) => write!(fmtr, "Garbage::Bucket({:?})", ptr),
        }
    }
}
