use super::{
    bucket::{Bucket, Entry, Garbage, GetRes},
    insertion::{Inserter, Insertion},
};
use atomic::AtomicBoxIncin;
use incin::Incinerator;
use owned_alloc::{OwnedAlloc, UninitAlloc};
use std::{
    borrow::Borrow,
    marker::PhantomData,
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering::*},
};

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

    pub unsafe fn get<'origin, Q>(
        &'origin self,
        key: &Q,
        hash: u64,
        incin: &Incinerator<Garbage<K, V>>,
    ) -> Option<&'origin (K, V)>
    where
        Q: ?Sized + Ord,
        K: Borrow<Q>,
    {
        let mut table = self;
        let mut shifted = hash;

        loop {
            let index = shifted as usize & (1 << BITS) - 1;
            let loaded = table.nodes[index].atomic.load(Acquire);

            if loaded.is_null() {
                break None;
            }

            if loaded as usize & 1 == 0 {
                let bucket = &*(loaded as *mut Bucket<K, V>);

                if bucket.hash() != hash {
                    break None;
                }

                break match bucket.get(key, incin) {
                    GetRes::Found(pair) => Some(pair),

                    GetRes::NotFound => None,

                    GetRes::Delete => {
                        let res = self.nodes[index].atomic.compare_and_swap(
                            loaded,
                            null_mut(),
                            Release,
                        );

                        if res != loaded {
                            continue;
                        }

                        None
                    },
                };
            }

            table = &*((loaded as usize & !1) as *mut Self);
            shifted >>= BITS;
        }
    }

    pub unsafe fn insert<I>(
        &self,
        inserter: I,
        hash: u64,
        incin: &Incinerator<Garbage<K, V>>,
        box_incin: &AtomicBoxIncin<Entry<K, V>>,
    ) -> Insertion<K, V, I>
    where
        I: Inserter<K, V>,
    {
        unimplemented!()
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
