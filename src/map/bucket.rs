use super::{guard::Removed, insertion::Inserter};
use incin::Incinerator;
use owned_alloc::OwnedAlloc;
use ptr;
use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt,
    ptr::{null_mut, NonNull},
    sync::{
        atomic::{AtomicPtr, Ordering::*},
        Arc,
    },
};

#[repr(align(/* at least */ 2))]
pub struct Bucket<K, V> {
    hash: u64,
    list: List<K, V>,
}

impl<K, V> Bucket<K, V> {
    pub fn new(hash: u64, pair: NonNull<(K, V)>) -> Self {
        let entry = Entry {
            pair,
            next: null_mut(),
        };
        let list = List::new(entry);
        let list_ptr = OwnedAlloc::new(list).into_raw().as_ptr();
        Self {
            hash,
            list: List::new(Entry::root(list_ptr)),
        }
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub unsafe fn is_empty(&self) -> bool {
        (*self.list.atomic.load(Acquire)).is_empty()
    }

    pub fn take_first(&mut self) -> Option<OwnedAlloc<Entry<K, V>>> {
        let entry = unsafe { &mut *self.list.atomic.load(Relaxed) };
        let prev = entry.next;
        entry.next = null_mut();
        NonNull::new(prev).map(|nnptr| {
            let list = unsafe { OwnedAlloc::from_raw(nnptr) };
            let ptr = list.atomic.load(Relaxed);
            unsafe { OwnedAlloc::from_raw(NonNull::new_unchecked(ptr)) }
        })
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

            FindRes::Exact { curr, .. } => {
                GetRes::Found(&*curr.as_ref().pair.as_ptr())
            },

            FindRes::After { .. } => GetRes::NotFound,
        }
    }

    pub unsafe fn insert<I>(
        &self,
        mut inserter: I,
        incin: &Arc<Incinerator<Garbage<K, V>>>,
    ) -> InsertRes<I, K, V>
    where
        I: Inserter<K, V>,
        K: Ord,
    {
        loop {
            match self.find(inserter.key(), &**incin) {
                FindRes::Delete => break InsertRes::Delete(inserter),

                FindRes::Exact { curr_list, curr } => {
                    inserter.input(Some(curr.as_ref().pair.as_ref()));
                    let pair = match inserter.pointer() {
                        Some(nnptr) => nnptr,
                        None => break InsertRes::Failed(inserter),
                    };
                    let new_entry = Entry {
                        pair,
                        next: curr.as_ref().next,
                    };
                    let new_ptr = OwnedAlloc::new(new_entry).into_raw();

                    if curr_list.try_update(curr, new_ptr, incin) {
                        inserter.take_pointer();
                        let pair = OwnedAlloc::from_raw(curr.as_ref().pair);
                        let removed = Removed::new(pair, incin);
                        break InsertRes::Updated(removed);
                    }
                },

                FindRes::After { prev_list, prev } => {
                    inserter.input(None);
                    let pair = match inserter.pointer() {
                        Some(nnptr) => nnptr,
                        None => break InsertRes::Failed(inserter),
                    };
                    let curr_entry = Entry {
                        pair,
                        next: prev.as_ref().next,
                    };
                    let curr_list = List::new(curr_entry);
                    let curr_nnptr = OwnedAlloc::new(curr_list).into_raw();
                    let new_prev = Entry {
                        pair: prev.as_ref().pair,
                        next: curr_nnptr.as_ptr(),
                    };
                    let new_ptr = OwnedAlloc::new(new_prev).into_raw();

                    if prev_list.try_update(prev, new_ptr, incin) {
                        inserter.take_pointer();
                        break InsertRes::Created;
                    } else {
                        OwnedAlloc::from_raw(curr_nnptr.as_ref().load());
                        OwnedAlloc::from_raw(curr_nnptr);
                    }
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
                FindRes::Delete => {
                    break RemoveRes {
                        pair: None,
                        delete: true,
                    }
                },

                FindRes::Exact { curr_list, curr } => {
                    if !interactive(curr.as_ref().pair.as_ref()) {
                        break RemoveRes {
                            pair: None,
                            delete: false,
                        };
                    }

                    let new_entry = Entry {
                        pair: curr.as_ref().pair,
                        next: (curr.as_ref().next as usize | 1) as *mut _,
                    };
                    let new_ptr = OwnedAlloc::new(new_entry).into_raw();

                    if curr_list.try_update(curr, new_ptr, incin) {
                        let pair = OwnedAlloc::from_raw(curr.as_ref().pair);
                        break RemoveRes {
                            pair: Some(Removed::new(pair, incin)),
                            delete: self.try_clear_first(&*incin),
                        };
                    }
                },

                FindRes::After { .. } => {
                    break RemoveRes {
                        pair: None,
                        delete: false,
                    }
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
            let mut prev = prev_list.load();

            loop {
                match prev_list.load_next(prev, incin) {
                    LoadNextRes::Failed => continue 'retry,
                    LoadNextRes::End => break 'retry,
                    LoadNextRes::Cleared { new_prev } => prev = new_prev,
                    LoadNextRes::Ok { list, entry } => {
                        out.push(map(&*entry.as_ref().pair.as_ptr()));
                        prev_list = &*list.as_ptr();
                        prev = entry;
                    },
                }
            }
        }
    }

    unsafe fn try_clear_first(
        &self,
        incin: &Incinerator<Garbage<K, V>>,
    ) -> bool {
        let mut prev = self.list.load();
        loop {
            match self.list.load_next(prev, incin) {
                LoadNextRes::Failed => break false,
                LoadNextRes::End => break true,
                LoadNextRes::Cleared { new_prev } => prev = new_prev,
                LoadNextRes::Ok { .. } => break false,
            }
        }
    }

    unsafe fn find<'origin, Q>(
        &'origin self,
        key: &Q,
        incin: &Incinerator<Garbage<K, V>>,
    ) -> FindRes<'origin, K, V>
    where
        Q: ?Sized + Ord,
        K: Borrow<Q>,
    {
        'retry: loop {
            let mut prev_list = &self.list;
            let mut prev = prev_list.load();

            loop {
                match prev_list.load_next(prev, incin) {
                    LoadNextRes::Failed => continue 'retry,

                    LoadNextRes::End => {
                        break 'retry if prev.as_ref().is_root() {
                            FindRes::Delete
                        } else {
                            FindRes::After { prev_list, prev }
                        }
                    },

                    LoadNextRes::Cleared { new_prev } => prev = new_prev,

                    LoadNextRes::Ok { list, entry } => {
                        let comparison = {
                            let (stored_key, _) = entry.as_ref().pair.as_ref();
                            key.cmp(stored_key.borrow())
                        };

                        match comparison {
                            Ordering::Equal => {
                                break 'retry FindRes::Exact {
                                    curr_list: &*list.as_ptr(),
                                    curr: entry,
                                }
                            },

                            Ordering::Less => {
                                break 'retry FindRes::After { prev_list, prev }
                            },

                            Ordering::Greater => {
                                prev_list = &*list.as_ptr();
                                prev = entry;
                            },
                        }
                    },
                }
            }
        }
    }
}

impl<K, V> Drop for Bucket<K, V> {
    fn drop(&mut self) {
        unsafe {
            let ptr = self.list.atomic.load(Relaxed);
            let sentinel = NonNull::new_unchecked(ptr);
            let mut top = sentinel.as_ref().next;
            OwnedAlloc::from_raw(sentinel);

            while let Some(list) = NonNull::new(top) {
                let ptr = list.as_ref().atomic.load(Relaxed);
                let entry = NonNull::new_unchecked(ptr);
                OwnedAlloc::from_raw(list);
                let next = if entry.as_ref().next as usize & 1 == 0 {
                    OwnedAlloc::from_raw(entry.as_ref().pair);
                    entry.as_ref().next
                } else {
                    (entry.as_ref().next as usize & !1) as *mut _
                };
                OwnedAlloc::from_raw(entry);
                top = next;
            }
        }
    }
}

pub struct Entry<K, V> {
    pair: NonNull<(K, V)>,
    next: *mut List<K, V>,
}

impl<K, V> Entry<K, V> {
    #[inline]
    pub fn root(next: *mut List<K, V>) -> Self {
        Self {
            pair: ptr::non_zero_null(),
            next,
        }
    }

    #[inline]
    pub fn is_root(&self) -> bool {
        self.pair == ptr::non_zero_null()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.is_root() && self.next == null_mut()
    }
}

impl<K, V> Clone for Entry<K, V> {
    fn clone(&self) -> Self {
        Self {
            pair: self.pair,
            next: self.next,
        }
    }
}

impl<K, V> Copy for Entry<K, V> {}

impl<K, V> PartialEq for Entry<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.pair == other.pair && self.next == other.next
    }
}

impl<K, V> Eq for Entry<K, V> {}

#[repr(align(/* at least */ 2))]
pub struct List<K, V> {
    atomic: AtomicPtr<Entry<K, V>>,
}

impl<K, V> List<K, V> {
    #[inline]
    fn new(entry: Entry<K, V>) -> Self {
        let ptr = OwnedAlloc::new(entry).into_raw().as_ptr();
        Self {
            atomic: AtomicPtr::new(ptr),
        }
    }

    unsafe fn load(&self) -> NonNull<Entry<K, V>> {
        NonNull::new_unchecked(self.atomic.load(Acquire))
    }

    unsafe fn load_next(
        &self,
        prev: NonNull<Entry<K, V>>,
        incin: &Incinerator<Garbage<K, V>>,
    ) -> LoadNextRes<K, V> {
        let list = match NonNull::new(prev.as_ref().next) {
            Some(nnptr) => nnptr,
            None => return LoadNextRes::End,
        };

        let entry = list.as_ref().load();
        let next = entry.as_ref().next as usize;

        if next & 1 == 1 {
            let new_entry = Entry {
                pair: prev.as_ref().pair,
                next: (next & !1) as *mut _,
            };
            let new_ptr = OwnedAlloc::new(new_entry).into_raw();

            if self.try_update(prev, new_ptr, incin) {
                incin.add(Garbage::List(OwnedAlloc::from_raw(list)));
                incin.add(Garbage::Entry(OwnedAlloc::from_raw(entry)));
                LoadNextRes::Cleared { new_prev: new_ptr }
            } else {
                LoadNextRes::Failed
            }
        } else {
            LoadNextRes::Ok { list, entry }
        }
    }

    unsafe fn try_update(
        &self,
        loaded: NonNull<Entry<K, V>>,
        new: NonNull<Entry<K, V>>,
        incin: &Incinerator<Garbage<K, V>>,
    ) -> bool {
        let res = self.atomic.compare_and_swap(
            loaded.as_ptr(),
            new.as_ptr(),
            Release,
        );

        if res == loaded.as_ptr() {
            let alloc = OwnedAlloc::from_raw(loaded);
            incin.add(Garbage::Entry(alloc));
            true
        } else {
            OwnedAlloc::from_raw(new);
            false
        }
    }
}

pub enum Garbage<K, V> {
    Pair(OwnedAlloc<(K, V)>),
    Entry(OwnedAlloc<Entry<K, V>>),
    List(OwnedAlloc<List<K, V>>),
    Bucket(OwnedAlloc<Bucket<K, V>>),
}

impl<K, V> fmt::Debug for Garbage<K, V> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Garbage::Pair(ptr) => write!(fmtr, "Garbage::Pair({:?})", ptr),
            Garbage::List(ptr) => write!(fmtr, "Garbage::List({:?})", ptr),
            Garbage::Bucket(ptr) => write!(fmtr, "Garbage::Bucket({:?})", ptr),
            Garbage::Entry(ptr) => write!(fmtr, "Garbage::Entry({:?})", ptr),
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

enum FindRes<'origin, K, V>
where
    K: 'origin,
    V: 'origin,
{
    Delete,

    Exact {
        curr_list: &'origin List<K, V>,
        curr: NonNull<Entry<K, V>>,
    },

    After {
        prev_list: &'origin List<K, V>,
        prev: NonNull<Entry<K, V>>,
    },
}

enum LoadNextRes<K, V> {
    Failed,
    End,
    Cleared {
        new_prev: NonNull<Entry<K, V>>,
    },
    Ok {
        list: NonNull<List<K, V>>,
        entry: NonNull<Entry<K, V>>,
    },
}
