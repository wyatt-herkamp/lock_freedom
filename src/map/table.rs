use super::{
    bucket::{Bucket, Garbage, GetRes, InsertRes},
    guard::Removed,
    insertion::{Inserter, Insertion},
};
use incin::Incinerator;
use owned_alloc::{Cache, OwnedAlloc, UninitAlloc};
use std::{
    borrow::Borrow,
    fmt,
    marker::PhantomData,
    ptr::{null_mut, NonNull},
    sync::{
        atomic::{AtomicPtr, Ordering::*},
        Arc,
    },
};

const BITS: usize = 8;

// If you remove this alignment, don't remove it. Please, set it to 2.
#[repr(align(64))]
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

    pub unsafe fn get<'map, Q>(
        &'map self,
        key: &Q,
        hash: u64,
        incin: &Incinerator<Garbage<K, V>>,
    ) -> Option<&'map (K, V)>
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

                        if res == loaded {
                            let alloc = OwnedAlloc::from_raw(
                                NonNull::new_unchecked(loaded as *mut _),
                            );
                            incin.add(Garbage::Bucket(alloc));
                        }

                        None
                    },
                };
            }

            table = &*((loaded as usize & !1) as *mut Self);
            shifted >>= BITS;
        }
    }

    #[inline(never)]
    pub unsafe fn insert<I>(
        &self,
        mut inserter: I,
        hash: u64,
        incin: &Arc<Incinerator<Garbage<K, V>>>,
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

                let bucket = Bucket::new(hash, pair);
                let bucket_nnptr = OwnedAlloc::new(bucket).into_raw();

                let res = table.nodes[index].atomic.compare_and_swap(
                    loaded,
                    bucket_nnptr.as_ptr() as *mut (),
                    Release,
                );

                if res.is_null() {
                    inserter.take_pointer();
                    break Insertion::Created;
                }

                let mut bucket = OwnedAlloc::from_raw(bucket_nnptr);
                bucket.take_first();
            } else if loaded as usize & 1 == 0 {
                let bucket = &*(loaded as *mut Bucket<K, V>);

                if bucket.hash() == hash {
                    match bucket.insert(inserter, incin) {
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

    pub unsafe fn remove<Q, F>(
        &self,
        key: &Q,
        interactive: F,
        hash: u64,
        incin: &Arc<Incinerator<Garbage<K, V>>>,
    ) -> Option<Removed<K, V>>
    where
        Q: ?Sized + Ord,
        K: Borrow<Q>,
        F: FnMut(&(K, V)) -> bool,
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

                let res = bucket.remove(key, interactive, incin);

                if res.delete {
                    let res = self.nodes[index].atomic.compare_and_swap(
                        loaded,
                        null_mut(),
                        Release,
                    );

                    if res == loaded {
                        let alloc = OwnedAlloc::from_raw(
                            NonNull::new_unchecked(loaded as *mut _),
                        );
                        incin.add(Garbage::Bucket(alloc));
                    }
                }
                break res.pair;
            }

            table = &*((loaded as usize & !1) as *mut Self);
            shifted >>= BITS;
        }
    }

    #[inline]
    pub fn free_nodes(&mut self, tbl_stack: &mut Vec<OwnedAlloc<Table<K, V>>>) {
        for node in &self.nodes as &[Node<K, V>] {
            unsafe {
                Node::free_ptr(node.atomic.load(Relaxed), tbl_stack);
            }
        }
    }

    #[inline]
    pub fn clear(&mut self, tbl_stack: &mut Vec<OwnedAlloc<Table<K, V>>>) {
        for node in &self.nodes as &[Node<K, V>] {
            unsafe {
                Node::free_ptr(
                    node.atomic.swap(null_mut(), Relaxed),
                    tbl_stack,
                );
            }
        }
    }

    pub fn optimize_space(&mut self) -> OptSpaceRes<K, V> {
        let mut removed = 0usize;
        let mut last_bucket = None;

        for node in &self.nodes as &[Node<K, V>] {
            let loaded = node.atomic.load(Relaxed);

            if loaded.is_null() {
                removed += 1;
            } else if loaded as usize & 1 == 0 {
                let bucket_ptr = loaded as *mut Bucket<K, V>;
                if unsafe { (*bucket_ptr).is_empty() } {
                    node.atomic.store(null_mut(), Release);
                    removed += 1;

                    unsafe {
                        OwnedAlloc::from_raw(NonNull::new_unchecked(
                            bucket_ptr,
                        ));
                    }
                } else {
                    let nnptr = unsafe { NonNull::new_unchecked(bucket_ptr) };
                    last_bucket = Some(nnptr);
                }
            } else {
                let table_ptr = (loaded as usize & !1) as *mut Table<K, V>;

                match unsafe { &mut *table_ptr }.optimize_space() {
                    OptSpaceRes::NoOpt => (),

                    OptSpaceRes::Remove => {
                        node.atomic.store(null_mut(), Relaxed);
                        unsafe {
                            let nnptr = NonNull::new_unchecked(table_ptr);
                            OwnedAlloc::from_raw(nnptr);
                        }
                        removed += 1;
                    },

                    OptSpaceRes::TableToBucket(nnptr) => {
                        unsafe {
                            let nnptr = NonNull::new_unchecked(table_ptr);
                            OwnedAlloc::from_raw(nnptr);
                        }
                        node.atomic.store(nnptr.as_ptr() as *mut _, Relaxed)
                    },
                }
            }
        }

        match (last_bucket, self.nodes.len() - removed) {
            (Some(nnptr), 1) => OptSpaceRes::TableToBucket(nnptr),

            (_, 0) => OptSpaceRes::Remove,

            _ => OptSpaceRes::NoOpt,
        }
    }

    pub fn load_index(&self, index: usize) -> Option<*mut ()> {
        self.nodes.get(index).map(|node| node.atomic.load(Acquire))
    }
}

impl<K, V> fmt::Debug for Table<K, V> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "Table {} nodes: {:?} {}",
            '{', &self.nodes as &[Node<K, V>], '}'
        )
    }
}

struct Node<K, V> {
    // First lower bit is 0 for leaf and 1 for branch
    atomic: AtomicPtr<()>,
    _marker: PhantomData<(K, V)>,
}

impl<K, V> Node<K, V> {
    unsafe fn free_ptr(
        ptr: *mut (),
        tbl_stack: &mut Vec<OwnedAlloc<Table<K, V>>>,
    ) {
        if ptr.is_null() {
            return;
        }

        if ptr as usize & 1 == 0 {
            OwnedAlloc::from_raw(NonNull::new_unchecked(
                ptr as *mut Bucket<K, V>,
            ));
        } else {
            let table_ptr = (ptr as usize & !1) as *mut Table<K, V>;

            debug_assert!(!table_ptr.is_null());
            tbl_stack
                .push(OwnedAlloc::from_raw(NonNull::new_unchecked(table_ptr)));
        }
    }
}

impl<K, V> Node<K, V> {
    fn new() -> Self {
        Self {
            atomic: AtomicPtr::new(null_mut()),
            _marker: PhantomData,
        }
    }
}

impl<K, V> fmt::Debug for Node<K, V> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "Node {} pointer: {:?} {}", '{', self.atomic, '}')
    }
}

pub enum OptSpaceRes<K, V> {
    NoOpt,
    Remove,
    TableToBucket(NonNull<Bucket<K, V>>),
}
