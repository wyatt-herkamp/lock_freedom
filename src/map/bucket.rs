use atomic::{Atomic, AtomicBox};
use incin::Incinerator;
use owned_alloc::{OwnedAlloc, UninitAlloc};
use ptr;
use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt,
    ptr::NonNull,
    sync::atomic::Ordering::*,
};

pub struct Entry<K, V> {
    pair: Option<NonNull<(K, V)>>,
    next: Option<NonNull<List<K, V>>>,
}

impl<K, V> Entry<K, V> {
    #[inline]
    pub fn root() -> Self {
        Self { pair: Some(ptr::non_zero_null()), next: None }
    }

    #[inline]
    pub fn is_root(&self) -> bool {
        self.pair == Some(ptr::non_zero_null())
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.is_root() && self.next == None
    }
}

impl<K, V> Clone for Entry<K, V> {
    fn clone(&self) -> Self {
        Self { pair: self.pair, next: self.next }
    }
}

impl<K, V> Copy for Entry<K, V> {}

impl<K, V> PartialEq for Entry<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.pair == other.pair && self.next == other.next
    }
}

impl<K, V> Eq for Entry<K, V> {}

pub struct List<K, V> {
    atomic: AtomicBox<Entry<K, V>>,
}

#[repr(align(/* at least */ 2))]
pub struct Bucket<K, V> {
    hash: u64,
    list: List<K, V>,
}

impl<K, V> Bucket<K, V> {
    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub unsafe fn get<'origin, Q>(
        &'origin self,
        key: &Q,
        incin: &Incinerator<Garbage<K, V>>,
    ) -> GetRes<'origin, K, V>
    where
        Q: ?Sized + Ord,
        K: Borrow<Q>,
    {
        match self.find(key, incin) {
            FindRes::Delete => GetRes::Delete,

            FindRes::Exact { curr_pair, .. } => {
                GetRes::Found(&*curr_pair.as_ptr())
            },

            FindRes::After { .. } => GetRes::NotFound,
        }
    }

    unsafe fn find<'list, Q>(
        &'list self,
        key: &Q,
        incin: &Incinerator<Garbage<K, V>>,
    ) -> FindRes<'list, K, V>
    where
        Q: ?Sized + Ord,
        K: Borrow<Q>,
    {
        'retry: loop {
            let mut prev_list = &self.list;
            let mut prev = prev_list.atomic.load(Acquire);

            if prev.is_empty() {
                break FindRes::Delete;
            }

            loop {
                let (curr_list, curr_ptr) = match prev.next {
                    Some(curr) => (&*curr.as_ptr(), curr),
                    None => break 'retry FindRes::After { prev_list, prev },
                };

                let curr = curr_list.atomic.load(Acquire);

                let curr_pair = match curr.pair {
                    Some(nnptr) => nnptr,

                    None => {
                        let new = Entry { pair: prev.pair, next: curr.next };
                        let res = prev_list
                            .atomic
                            .compare_and_swap(prev, new, Release);
                        if res != prev {
                            continue 'retry;
                        }

                        let alloc = OwnedAlloc::from_raw(curr_ptr);
                        incin.add(Garbage::List(alloc));

                        if new.is_empty() {
                            break 'retry FindRes::Delete;
                        }

                        continue;
                    },
                };

                let comparison = {
                    let (stored_key, _) = curr_pair.as_ref();
                    key.cmp(stored_key.borrow())
                };

                match comparison {
                    Ordering::Equal => {
                        break 'retry FindRes::Exact {
                            curr_list,
                            curr_pair,
                            curr_next: curr.next,
                        }
                    },

                    Ordering::Less => {
                        break 'retry FindRes::After { prev_list, prev }
                    },

                    Ordering::Greater => {
                        prev_list = curr_list;
                        prev = curr;
                    },
                }
            }
        }
    }
}

pub enum Garbage<K, V> {
    Pair(OwnedAlloc<(K, V)>),
    List(OwnedAlloc<List<K, V>>),
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

pub enum GetRes<'origin, K, V> {
    Found(&'origin (K, V)),
    NotFound,
    Delete,
}

enum FindRes<'list, K, V> {
    Exact {
        curr_list: &'list List<K, V>,
        curr_pair: NonNull<(K, V)>,
        curr_next: Option<NonNull<List<K, V>>>,
    },

    After {
        prev_list: &'list List<K, V>,
        prev: Entry<K, V>,
    },

    Delete,
}
