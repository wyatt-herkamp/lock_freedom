use owned_alloc::{OwnedAlloc, UninitAlloc};
use std::{marker::PhantomData, ptr::null_mut, sync::atomic::AtomicPtr};

const BITS: usize = 8;

#[repr(align(/* at least */ 2))]
pub struct Table<K, V> {
    nodes: [Node<K, V>; 1 << BITS],
}

impl<K, V> Table<K, V> {
    pub fn new_alloc() -> OwnedAlloc<Self> {
        unsafe {
            UninitAlloc::<Self>::new().init_in_place(|val| val.init_in_place())
        }
    }

    pub unsafe fn init_in_place(&mut self) {
        for node in &mut self.nodes as &mut [_] {
            (node as *mut Node<K, V>).write(Node::new())
        }
    }
}

struct Node<K, V> {
    // First lower bit is 0 for leaf and 1 for branch
    atomic: AtomicPtr<()>,
    _marker: PhantomData<(K, V)>,
}

impl<K, V> Node<K, V> {
    fn new() -> Self {
        Self { atomic: AtomicPtr::new(null_mut()), _marker: PhantomData }
    }
}
