use super::{guard::Removed, insertion::Inserter};
use atomic::{Atomic, AtomicBox, AtomicBoxIncin};
use incin::Incinerator;
use owned_alloc::OwnedAlloc;
use ptr;
use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt,
    ptr::NonNull,
    sync::{atomic::Ordering::*, Arc},
};

pub struct Entry<K, V> {
    pair: Option<NonNull<(K, V)>>,
    next: Option<NonNull<List<K, V>>>,
}

impl<K, V> Entry<K, V> {
    #[inline]
    pub fn root(next: Option<NonNull<List<K, V>>>) -> Self {
        Self { pair: Some(ptr::non_zero_null()), next }
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

impl<K, V> List<K, V> {
    #[inline]
    pub fn new(
        entry: Entry<K, V>,
        box_incin: AtomicBoxIncin<Entry<K, V>>,
    ) -> Self {
        Self { atomic: AtomicBox::with_incinerator(entry, box_incin) }
    }
}

#[repr(align(/* at least */ 2))]
pub struct Bucket<K, V> {
    hash: u64,
    list: List<K, V>,
}

impl<K, V> Bucket<K, V> {
    pub fn new(
        hash: u64,
        nnptr: NonNull<(K, V)>,
        box_incin: AtomicBoxIncin<Entry<K, V>>,
    ) -> Self {
        let entry = Entry { pair: Some(nnptr), next: None };
        let list = List::new(entry, box_incin.clone());
        let list_ptr = Some(OwnedAlloc::new(list).into_raw());
        Self { hash, list: List::new(Entry::root(list_ptr), box_incin) }
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub fn is_empty(&self) -> bool {
        self.list.atomic.load(Acquire).is_empty()
    }

    pub fn take_first(&mut self) -> Option<Entry<K, V>> {
        self.list.atomic.swap(Entry::root(None), Relaxed).next.map(
            |nnptr| unsafe { OwnedAlloc::from_raw(nnptr).atomic.load(Relaxed) },
        )
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

    pub unsafe fn insert<I>(
        &self,
        mut inserter: I,
        incin: &Arc<Incinerator<Garbage<K, V>>>,
        box_incin: &AtomicBoxIncin<Entry<K, V>>,
    ) -> InsertRes<I, K, V>
    where
        I: Inserter<K, V>,
        K: Ord,
    {
        loop {
            match self.find(inserter.key(), &**incin) {
                FindRes::Delete => break InsertRes::Delete(inserter),

                FindRes::Exact { curr_list, curr_pair, curr_next, .. } => {
                    inserter.input(Some(curr_pair.as_ref()));
                    let pair = match inserter.pointer() {
                        Some(nnptr) => nnptr,
                        None => break InsertRes::Failed(inserter),
                    };
                    let old = Entry { pair: Some(curr_pair), next: curr_next };
                    let new = Entry { pair: Some(pair), next: curr_next };
                    let res =
                        curr_list.atomic.compare_and_swap(old, new, Release);
                    if old == res {
                        inserter.take_pointer();
                        let alloc = OwnedAlloc::from_raw(curr_pair);
                        let removed = Removed::new(alloc, incin);
                        break InsertRes::Updated(removed);
                    }
                },

                FindRes::After { prev_list, prev } => {
                    inserter.input(None);
                    let pair = match inserter.pointer() {
                        Some(nnptr) => nnptr,
                        None => break InsertRes::Failed(inserter),
                    };
                    let curr_entry =
                        Entry { pair: Some(pair), next: prev.next };
                    let curr_list = List::new(curr_entry, box_incin.clone());
                    let curr_nnptr = OwnedAlloc::new(curr_list).into_raw();
                    let new_prev =
                        Entry { pair: prev.pair, next: Some(curr_nnptr) };
                    let res = prev_list
                        .atomic
                        .compare_and_swap(prev, new_prev, Release);

                    if res == prev {
                        inserter.take_pointer();
                        break InsertRes::Created;
                    }

                    OwnedAlloc::from_raw(curr_nnptr);
                },
            }
        }
    }

    pub unsafe fn remove<Q, F>(
        &self,
        key: &Q,
        mut interactive: F,
        incin: &Arc<Incinerator<Garbage<K, V>>>,
    ) -> RemoveRes<K, V>
    where
        Q: ?Sized + Ord,
        K: Borrow<Q>,
        F: FnMut(&(K, V)) -> bool,
    {
        loop {
            match self.find(key, &*incin) {
                FindRes::Delete => break RemoveRes { pair: None, delete: true },

                FindRes::Exact { prev, curr_list, curr_pair, curr_next } => {
                    if !interactive(&*curr_pair.as_ptr()) {
                        break RemoveRes { pair: None, delete: false };
                    }

                    let old = Entry { pair: Some(curr_pair), next: curr_next };
                    let new = Entry { pair: None, next: curr_next };
                    let res =
                        curr_list.atomic.compare_and_swap(old, new, Release);

                    if res == old {
                        let prev = Entry { pair: prev.pair, next: curr_next };
                        let delete =
                            prev.is_empty() && self.try_clear_first(&*incin);
                        let alloc = OwnedAlloc::from_raw(curr_pair);
                        break RemoveRes {
                            pair: Some(Removed::new(alloc, incin)),
                            delete,
                        };
                    }
                },

                FindRes::After { .. } => {
                    break RemoveRes { pair: None, delete: false }
                },
            }
        }
    }

    pub unsafe fn collect<'origin, F, T>(
        &'origin self,
        incin: &Incinerator<Garbage<K, V>>,
        out: &mut Vec<T>,
        mut map: F,
    ) where
        F: FnMut(&'origin (K, V)) -> T,
    {
        let trunc = out.len();

        'retry: loop {
            out.truncate(trunc);
            let mut prev_list = &self.list;
            let mut prev = prev_list.atomic.load(Acquire);

            if prev.is_empty() {
                break;
            }

            loop {
                let (curr_list, curr_ptr) = match prev.next {
                    Some(curr) => (&*curr.as_ptr(), curr),
                    None => break 'retry,
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
                            break 'retry;
                        }

                        continue;
                    },
                };

                out.push(map(&*curr_pair.as_ptr()));
                prev_list = curr_list;
                prev = curr;
            }
        }
    }

    unsafe fn try_clear_first(
        &self,
        incin: &Incinerator<Garbage<K, V>>,
    ) -> bool {
        let mut prev = self.list.atomic.load(Acquire);
        loop {
            let (curr_list, curr_ptr) = match prev.next {
                Some(curr) => (&*curr.as_ptr(), curr),
                None => break true,
            };

            let curr = curr_list.atomic.load(Acquire);

            match curr.pair {
                Some(_) => break false,

                None => {
                    let new = Entry { pair: prev.pair, next: curr.next };
                    let res =
                        self.list.atomic.compare_and_swap(prev, new, Release);
                    if res != prev {
                        break false;
                    }

                    let alloc = OwnedAlloc::from_raw(curr_ptr);
                    incin.add(Garbage::List(alloc));

                    if new.is_empty() {
                        break true;
                    }

                    prev = new;
                },
            }
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
                            prev,
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

impl<K, V> Drop for Bucket<K, V> {
    fn drop(&mut self) {
        let mut top = self.list.atomic.load(Relaxed);

        while let Some(list) = top.next {
            top = unsafe { list.as_ref() }.atomic.load(Relaxed);

            if let Some(pair) = top.pair {
                unsafe { OwnedAlloc::from_raw(pair) };
            }

            unsafe { OwnedAlloc::from_raw(list) };
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

pub enum GetRes<'origin, K, V>
where
    K: 'origin,
    V: 'origin,
{
    Found(&'origin (K, V)),
    NotFound,
    Delete,
}

pub enum InsertRes<I, K, V> {
    Created,
    Updated(Removed<K, V>),
    Failed(I),
    Delete(I),
}

pub struct RemoveRes<K, V> {
    pub pair: Option<Removed<K, V>>,
    pub delete: bool,
}

enum FindRes<'list, K, V>
where
    K: 'list,
    V: 'list,
{
    Exact {
        prev: Entry<K, V>,
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
