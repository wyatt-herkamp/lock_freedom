use super::{
    bucket::{Bucket, Entry, Garbage, GetRes, InsertRes},
    insertion::{Inserter, Insertion},
};
use atomic::AtomicBoxIncin;
use incin::Incinerator;
use owned_alloc::{Cache, OwnedAlloc, UninitAlloc};
use std::{
    borrow::Borrow,
    marker::PhantomData,
    ptr::{null_mut, NonNull},
    sync::{
        atomic::{AtomicPtr, Ordering::*},
        Arc,
    },
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

                        let alloc = OwnedAlloc::from_raw(
                            NonNull::new_unchecked(loaded as *mut _),
                        );
                        incin.add(Garbage::Bucket(alloc));

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
        mut inserter: I,
        hash: u64,
        incin: &Arc<Incinerator<Garbage<K, V>>>,
        box_incin: &AtomicBoxIncin<Entry<K, V>>,
    ) -> Insertion<K, V, I>
    where
        I: Inserter<K, V>,
        K: Ord,
    {
        let mut table = self;
        let mut shifted = hash;
        let mut depth = 1;
        let mut tbl_cache = Cache::<OwnedAlloc<Self>>::new();

        loop {
            let index = shifted as usize & (1 << BITS) - 1;

            let loaded = table.nodes[index].atomic.load(Acquire);

            if loaded.is_null() {
                inserter.input(None);
                let pair = match inserter.pointer() {
                    Some(nnptr) => nnptr,
                    None => break Insertion::Failed(inserter),
                };

                let bucket = Bucket::new(hash, pair, box_incin.clone());
                let bucket_nnptr = OwnedAlloc::new(bucket).into_raw();

                let res = table.nodes[index].atomic.compare_and_swap(
                    loaded,
                    bucket_nnptr.as_ptr() as *mut (),
                    Release,
                );

                if res.is_null() {
                    break Insertion::Created;
                }

                OwnedAlloc::from_raw(bucket_nnptr);
            } else if loaded as usize & 1 == 0 {
                let bucket = &*(loaded as *mut Bucket<K, V>);

                if bucket.hash() == hash {
                    match bucket.insert(inserter, incin, box_incin) {
                        InsertRes::Created => break Insertion::Created,

                        InsertRes::Updated(old) => {
                            break Insertion::Updated(old)
                        },

                        InsertRes::Failed(inserter) => {
                            break Insertion::Failed(inserter)
                        },

                        InsertRes::Delete(returned) => {
                            let res = table.nodes[index]
                                .atomic
                                .compare_and_swap(loaded, null_mut(), Release);

                            if res == loaded {
                                let alloc = OwnedAlloc::from_raw(
                                    NonNull::new_unchecked(loaded as *mut _),
                                );
                                incin.add(Garbage::Bucket(alloc));
                            }

                            inserter = returned;
                        },
                    }
                } else {
                    let new_table = tbl_cache.take_or(|| Self::new_alloc());
                    let other_shifted = bucket.hash() >> (depth * BITS);
                    let other_index = other_shifted as usize & (1 << BITS) - 1;

                    new_table.nodes[other_index].atomic.store(loaded, Relaxed);

                    let new_table_nnptr = new_table.into_raw();
                    let res = table.nodes[index].atomic.compare_and_swap(
                        loaded,
                        (new_table_nnptr.as_ptr() as usize | 1) as *mut (),
                        Release,
                    );

                    if res == loaded {
                        depth += 1;
                        table = &*new_table_nnptr.as_ptr();
                        shifted >>= BITS;
                    } else {
                        let new_table = OwnedAlloc::from_raw(new_table_nnptr);
                        new_table.nodes[other_index]
                            .atomic
                            .store(null_mut(), Relaxed);
                        tbl_cache.store(new_table);
                    }
                }
            } else {
                depth += 1;
                table = &*((loaded as usize & !1) as *mut Self);
                shifted >>= BITS;
            }
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
