use std::{marker::PhantomData, sync::atomic::AtomicPtr};

const BITS: usize = 8;

#[repr(align(/* at least */ 2))]
pub struct Table<K, V> {
    nodes: [Node<K, V>; 1 << BITS],
}

struct Node<K, V> {
    atomic: AtomicPtr<()>,
    _marker: PhantomData<(K, V)>,
}
