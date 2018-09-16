use alloc::*;
use atomic::{Atomic, AtomicBox};
use incinerator;
use std::{
    borrow::Borrow,
    cmp::Ordering,
    ptr::{null_mut, NonNull},
    sync::atomic::Ordering::{self as AtomicOrd, *},
};

static _NON_NULL: u8 = 0;

pub struct Pair<K, V> {
    pub key: K,
    pub val: V,
}

pub struct Entry<K, V> {
    pair: *mut Pair<K, V>,
    next: *mut List<K, V>,
}

impl<K, V> Entry<K, V> {
    pub fn new(pair: *mut Pair<K, V>, next: *mut List<K, V>) -> Self {
        Self { pair, next }
    }

    pub fn root(next: *mut List<K, V>) -> Self {
        Self::new(&_NON_NULL as *const _ as *mut _, next)
    }

    pub fn is_empty(&self) -> bool {
        self.is_root() && self.next.is_null()
    }

    pub fn is_root(&self) -> bool {
        self.pair == &_NON_NULL as *const _ as *mut _
    }

    pub fn pair(&self) -> *mut Pair<K, V> {
        self.pair
    }

    pub fn next(&self) -> *mut List<K, V> {
        self.next
    }
}

impl<K, V> PartialEq for Entry<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.pair == other.pair && self.next == other.next
    }
}

impl<K, V> Eq for Entry<K, V> {}

impl<K, V> Clone for Entry<K, V> {
    fn clone(&self) -> Self {
        Self { pair: self.pair, next: self.next }
    }
}

impl<K, V> Copy for Entry<K, V> {}

pub struct List<K, V> {
    ptr: AtomicBox<Entry<K, V>>,
}

impl<K, V> List<K, V> {
    pub fn new(entry: Entry<K, V>) -> Self {
        Self { ptr: AtomicBox::new(entry) }
    }

    pub fn load(&self, ord: AtomicOrd) -> Entry<K, V> {
        self.ptr.load(ord)
    }
}

enum FindRes<'list, K, V>
where
    K: 'list,
    V: 'list,
{
    Delete,
    Eq {
        #[allow(dead_code)]
        prev_list: &'list List<K, V>,
        prev: Entry<K, V>,
        curr: Entry<K, V>,
    },
    Between {
        prev_list: &'list List<K, V>,
        prev: Entry<K, V>,
        #[allow(dead_code)]
        next: Option<Entry<K, V>>,
    },
}

impl<'list, K, V> PartialEq for FindRes<'list, K, V> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FindRes::Delete, FindRes::Delete) => true,

            (
                FindRes::Eq { prev: p0, curr: c0, .. },
                FindRes::Eq { prev: p1, curr: c1, .. },
            ) => p0 == p1 && c0 == c1,

            (
                FindRes::Between { prev: p0, next: n0, .. },
                FindRes::Between { prev: p1, next: n1, .. },
            ) => p0 == p1 && n0 == n1,

            _ => false,
        }
    }
}

impl<'list, K, V> Eq for FindRes<'list, K, V> {}

pub struct Bucket<K, V> {
    hash: u64,
    list: List<K, V>,
}

impl<K, V> Bucket<K, V> {
    pub fn new(hash: u64, list: List<K, V>) -> Self {
        Self { hash, list }
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub fn list(&self) -> &List<K, V> {
        &self.list
    }

    pub unsafe fn insert(
        &self,
        pair: NonNull<Pair<K, V>>,
    ) -> Option<*mut Pair<K, V>>
    where
        K: Ord,
    {
        loop {
            match self.find(&pair.as_ref().key) {
                FindRes::Delete => break None,

                FindRes::Eq { prev, curr, .. } => {
                    let new_entry =
                        Entry { pair: pair.as_ptr(), next: curr.next };
                    let res = (*prev.next)
                        .ptr
                        .compare_and_swap(curr, new_entry, Release);
                    if res == curr {
                        break Some(curr.pair);
                    }
                },

                FindRes::Between { prev_list, prev, .. } => {
                    let list = alloc(List {
                        ptr: AtomicBox::new(Entry {
                            pair: pair.as_ptr(),
                            next: prev.next,
                        }),
                    });

                    let new_entry =
                        Entry { pair: prev.pair, next: list.as_ptr() };

                    let res = prev_list
                        .ptr
                        .compare_and_swap(prev, new_entry, Release);
                    if res == prev {
                        break Some(null_mut());
                    }

                    dealloc(list);
                },
            }
        }
    }

    pub unsafe fn get<Q>(&self, key: &Q) -> Option<*mut Pair<K, V>>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        match self.find(key) {
            FindRes::Delete => None,
            FindRes::Eq { curr, .. } => Some(curr.pair),
            _ => Some(null_mut()),
        }
    }

    pub unsafe fn remove<Q>(&self, key: &Q) -> Option<(*mut Pair<K, V>, bool)>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        loop {
            match self.find(key) {
                FindRes::Delete => break None,

                FindRes::Eq { prev, curr, .. } => {
                    let new_entry = Entry { pair: null_mut(), next: curr.next };
                    let res = (*prev.next)
                        .ptr
                        .compare_and_swap(curr, new_entry, Release);

                    if res == curr {
                        break Some((
                            curr.pair,
                            Entry { pair: prev.pair, next: curr.next }
                                .is_empty()
                                && self.try_clean_first(),
                        ));
                    }
                },

                _ => break Some((null_mut(), false)),
            }
        }
    }

    unsafe fn try_clean_first(&self) -> bool {
        loop {
            let prev_list = &self.list;
            let prev = prev_list.ptr.load(Acquire);
            let next_list = match prev.next.as_ref() {
                Some(next) => next,
                None => break true,
            };

            let next = next_list.ptr.load(Acquire);
            if next.pair.is_null() {
                let new = Entry { pair: prev.pair, next: next.next };
                let res = prev_list.ptr.compare_and_swap(prev, new, Release);

                if res != prev {
                    break false;
                }

                incinerator::add(NonNull::new_unchecked(prev.next), dealloc);

                if new.is_empty() {
                    break true;
                }
            } else {
                break false;
            }
        }
    }

    unsafe fn find<Q>(&self, key: &Q) -> FindRes<K, V>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        'outer: loop {
            let mut prev_list = &self.list;
            let mut prev = prev_list.ptr.load(Acquire);
            if prev.is_empty() {
                break FindRes::Delete;
            }

            loop {
                let next_list = match prev.next.as_ref() {
                    Some(next) => next,
                    None => {
                        break 'outer FindRes::Between {
                            prev_list,
                            prev,
                            next: None,
                        }
                    },
                };

                let next = next_list.ptr.load(Acquire);
                if next.pair.is_null() {
                    let new = Entry { pair: prev.pair, next: next.next };
                    let res =
                        prev_list.ptr.compare_and_swap(prev, new, Release);

                    if res != prev {
                        break;
                    }

                    incinerator::add(
                        NonNull::new_unchecked(prev.next),
                        dealloc,
                    );

                    if new.is_empty() {
                        break 'outer FindRes::Delete;
                    }

                    continue;
                }

                match (*next.pair).key.borrow().cmp(key) {
                    Ordering::Less => {
                        prev_list = next_list;
                        prev = next;
                    },

                    Ordering::Equal => {
                        break 'outer FindRes::Eq { prev_list, prev, curr: next }
                    },

                    Ordering::Greater => {
                        break 'outer FindRes::Between {
                            prev_list,
                            prev,
                            next: Some(next),
                        }
                    },
                }
            }
        }
    }
}
