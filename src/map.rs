use alloc::*;
use atomic::{Atomic, AtomicBox};
use incinerator;
use std::{
    borrow::Borrow,
    cmp::Ordering,
    collections::hash_map::RandomState,
    fmt,
    hash::{BuildHasher, Hash, Hasher},
    mem,
    ptr::{null_mut, NonNull},
    sync::atomic::{AtomicPtr, Ordering::*},
};

static mut _NON_NULL: u8 = 255;

const BITS: usize = 8;

struct Pair<K, V> {
    key: K,
    val: V,
}

struct Entry<K, V> {
    pair: *mut Pair<K, V>,
    next: *mut List<K, V>,
}

struct List<K, V> {
    ptr: AtomicBox<Entry<K, V>>,
}

struct Bucket<K, V> {
    hash: u64,
    list: List<K, V>,
}

struct Table<K, V> {
    nodes: [AtomicPtr<Node<K, V>>; 1 << BITS],
}

enum Node<K, V> {
    Leaf(Bucket<K, V>),
    Branch(Table<K, V>),
}

enum FindRes<'list, K, V>
where
    K: 'list,
    V: 'list,
{
    Delete,
    Eq {
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

pub struct Map<K, V, H = RandomState> {
    table: Table<K, V>,
    builder: H,
}

#[derive(Eq)]
pub struct Removed<K, V> {
    pair: NonNull<Pair<K, V>>,
}

impl<K, V> Map<K, V, RandomState> {
    pub fn new() -> Self {
        Self::with_hasher_builder(RandomState::default())
    }
}

impl<K, V, H> Map<K, V, H> {
    pub fn with_hasher_builder(builder: H) -> Self
    where
        H: BuildHasher,
    {
        Self { table: Table::new(), builder }
    }

    pub fn insert(&self, key: K, val: V) -> Option<Removed<K, V>>
    where
        K: Hash + Ord,
        H: BuildHasher,
    {
        let mut hasher = self.builder.build_hasher();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        incinerator::pause(|| unsafe {
            NonNull::new(self.table.insert(key, hash, val)).map(Removed::new)
        })
    }

    pub fn get<Q, F, T>(&self, key: &Q, reader: F) -> Option<T>
    where
        Q: Hash + Ord + ?Sized,
        K: Borrow<Q>,
        H: BuildHasher,
        F: FnOnce(&V) -> T,
    {
        let mut hasher = self.builder.build_hasher();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        incinerator::pause(|| unsafe {
            self.table.get(key, hash).as_ref().map(|x| reader(&x.val))
        })
    }

    pub fn get_pair<Q, F, T>(&self, key: &Q, reader: F) -> Option<T>
    where
        Q: Hash + Ord + ?Sized,
        K: Borrow<Q>,
        H: BuildHasher,
        F: FnOnce(&K, &V) -> T,
    {
        let mut hasher = self.builder.build_hasher();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        incinerator::pause(|| unsafe {
            self.table.get(key, hash).as_ref().map(|x| reader(&x.key, &x.val))
        })
    }

    pub fn remove<Q>(&self, key: &Q) -> Option<Removed<K, V>>
    where
        Q: Hash + Ord + ?Sized,
        K: Borrow<Q>,
        H: BuildHasher,
    {
        let mut hasher = self.builder.build_hasher();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        incinerator::pause(|| unsafe {
            NonNull::new(self.table.remove(key, hash)).map(Removed::new)
        })
    }
}

impl<K, V> Removed<K, V> {
    fn new(pair: NonNull<Pair<K, V>>) -> Self {
        Self { pair }
    }

    pub fn key(&self) -> &K {
        &unsafe { self.pair.as_ref() }.key
    }

    pub fn val(&self) -> &V {
        &unsafe { self.pair.as_ref() }.val
    }
}

impl<K, V> Table<K, V> {
    fn new() -> Self {
        let mut this = Self { nodes: unsafe { mem::uninitialized() } };
        for node in &mut this.nodes as &mut [_] {
            unsafe {
                (node as *mut AtomicPtr<_>).write(AtomicPtr::new(null_mut()))
            }
        }
        this
    }

    unsafe fn insert(&self, key: K, hash: u64, val: V) -> *mut Pair<K, V>
    where
        K: Ord,
    {
        let pair = alloc(Pair { key, val });
        let entry = Entry { pair: pair.as_ptr(), next: null_mut() };
        let list = alloc(List { ptr: AtomicBox::new(entry) });
        let bucket = Bucket {
            hash,
            list: List {
                ptr: AtomicBox::new(Entry {
                    pair: &mut _NON_NULL as *mut _ as _,
                    next: list.as_ptr(),
                }),
            },
        };
        let node = alloc(Node::Leaf(bucket));

        let mut table = self;
        let mut index = hash;
        let mut depth = 1;

        loop {
            let node_index = index as usize & (1 << BITS) - 1;
            let old = table.nodes[node_index].compare_and_swap(
                null_mut(),
                node.as_ptr(),
                SeqCst,
            );
            match old.as_ref() {
                Some(Node::Leaf(in_place)) if in_place.hash == hash => {
                    match in_place.insert(pair) {
                        Some(ptr) => {
                            dealloc(node);
                            dealloc(list);
                            break ptr;
                        },

                        None => {
                            let res = table.nodes[node_index].compare_and_swap(
                                old,
                                node.as_ptr(),
                                SeqCst,
                            );

                            if res == old {
                                incinerator::add(
                                    NonNull::new_unchecked(res),
                                    dealloc,
                                );
                                break null_mut();
                            }
                        },
                    }
                },

                Some(Node::Leaf(in_place)) => {
                    let branch = alloc(Node::Branch(Table::new()));
                    let new_table = match &*branch.as_ptr() {
                        Node::Branch(t) => t,
                        _ => unreachable!(),
                    };
                    let shifted = in_place.hash >> (depth * BITS as u64);
                    let in_place_index = shifted as usize & (1 << BITS) - 1;

                    new_table.nodes[in_place_index].store(old, Relaxed);
                    let res = table.nodes[node_index].compare_and_swap(
                        old,
                        branch.as_ptr(),
                        SeqCst,
                    );

                    if res == old {
                        table = new_table;
                        index >>= BITS as u64;
                        depth += 1;
                    } else {
                        dealloc(branch);
                    }
                },

                Some(Node::Branch(new_table)) => {
                    table = new_table;
                    index >>= BITS as u64;
                    depth += 1;
                },

                None => break null_mut(),
            }
        }
    }

    unsafe fn get<Q>(&self, key: &Q, hash: u64) -> *mut Pair<K, V>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        let mut table = self;
        let mut index = hash;

        loop {
            let node_index = index as usize & (1 << BITS) - 1;
            let in_place = table.nodes[node_index].load(SeqCst);
            match in_place.as_ref() {
                Some(Node::Leaf(bucket)) if bucket.hash == hash => {
                    match bucket.get(key) {
                        Some(x) => break x,

                        None => {
                            let res = table.nodes[node_index].compare_and_swap(
                                in_place,
                                null_mut(),
                                SeqCst,
                            );

                            if res == in_place {
                                incinerator::add(
                                    NonNull::new_unchecked(res),
                                    dealloc,
                                );
                                break null_mut();
                            }
                        },
                    }
                },

                Some(Node::Branch(new_table)) => {
                    table = new_table;
                    index >>= BITS as u64;
                },

                _ => break null_mut(),
            }
        }
    }

    unsafe fn remove<Q>(&self, key: &Q, hash: u64) -> *mut Pair<K, V>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        let mut table = self;
        let mut index = hash;

        loop {
            let node_index = index as usize & (1 << BITS) - 1;
            let in_place = table.nodes[node_index].load(SeqCst);
            match in_place.as_ref() {
                Some(Node::Leaf(bucket)) if bucket.hash == hash => match bucket
                    .remove(key)
                {
                    Some((pair, delete)) => {
                        if delete {
                            let res = table.nodes[node_index].compare_and_swap(
                                in_place,
                                null_mut(),
                                SeqCst,
                            );

                            if res == in_place {
                                incinerator::add(
                                    NonNull::new_unchecked(res),
                                    dealloc,
                                );
                            }
                        }
                        break pair;
                    },

                    None => {
                        let res = table.nodes[node_index].compare_and_swap(
                            in_place,
                            null_mut(),
                            SeqCst,
                        );

                        if res == in_place {
                            dealloc(NonNull::new_unchecked(res));
                            break null_mut();
                        }
                    },
                },

                Some(Node::Branch(new_table)) => {
                    table = new_table;
                    index >>= BITS as u64;
                },

                _ => break null_mut(),
            }
        }
    }
}

impl<K, V> Bucket<K, V> {
    unsafe fn insert(
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
                        .compare_and_swap(curr, new_entry, SeqCst);
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

                    let res =
                        prev_list.ptr.compare_and_swap(prev, new_entry, SeqCst);
                    if res == prev {
                        break Some(null_mut());
                    }

                    dealloc(list);
                },
            }
        }
    }

    unsafe fn get<Q>(&self, key: &Q) -> Option<*mut Pair<K, V>>
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

    unsafe fn remove<Q>(&self, key: &Q) -> Option<(*mut Pair<K, V>, bool)>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        loop {
            match self.find(key) {
                FindRes::Delete => break None,

                FindRes::Eq { prev_list, prev, curr } => {
                    if prev.pair == &mut _NON_NULL as *mut _ as _
                        && curr.next.is_null()
                    {
                        let empty =
                            Entry { pair: null_mut(), next: null_mut() };
                        let res =
                            prev_list.ptr.compare_and_swap(prev, empty, SeqCst);

                        if res == prev {
                            incinerator::add(
                                NonNull::new_unchecked(prev.next),
                                dealloc,
                            );
                            break Some((curr.pair, true));
                        }
                    } else {
                        let new_entry =
                            Entry { pair: prev.pair, next: curr.next };
                        let res = prev_list
                            .ptr
                            .compare_and_swap(prev, new_entry, SeqCst);

                        if res == prev {
                            incinerator::add(
                                NonNull::new_unchecked(prev.next),
                                dealloc,
                            );
                            break Some((curr.pair, false));
                        }
                    }
                },

                _ => break Some((null_mut(), false)),
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
            let mut prev = prev_list.ptr.load(SeqCst);
            if prev.pair.is_null() {
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

                let next = next_list.ptr.load(SeqCst);
                if next.pair.is_null() {
                    let new = Entry { pair: prev.pair, next: next.next };
                    let res = prev_list.ptr.compare_and_swap(prev, new, SeqCst);

                    if res != prev {
                        break;
                    }

                    incinerator::add(
                        NonNull::new_unchecked(prev.next),
                        dealloc,
                    );
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

impl<K, V, H> Drop for Map<K, V, H> {
    fn drop(&mut self) {
        let mut node_ptrs = Vec::new();
        for node in &self.table.nodes as &[AtomicPtr<_>] {
            let loaded = node.load(Acquire);
            if let Some(nnptr) = NonNull::new(loaded) {
                node_ptrs.push(nnptr);
            }
        }

        while let Some(node_ptr) = node_ptrs.pop() {
            match unsafe { node_ptr.as_ref() } {
                Node::Leaf(bucket) => {
                    let mut list = bucket.list.ptr.load(Relaxed).next;
                    while let Some(nnptr) = NonNull::new(list) {
                        let entry = unsafe { nnptr.as_ref().ptr.load(Relaxed) };
                        if let Some(nnptr) = NonNull::new(entry.pair) {
                            unsafe { dealloc(nnptr) }
                        }
                        unsafe { dealloc(nnptr) }
                        list = entry.next;
                    }
                },

                Node::Branch(table) => {
                    for node in &table.nodes as &[AtomicPtr<_>] {
                        let loaded = node.load(Acquire);
                        if let Some(nnptr) = NonNull::new(loaded) {
                            node_ptrs.push(nnptr);
                        }
                    }
                },
            }

            unsafe { dealloc(node_ptr) }
        }
    }
}

impl<K, V> Drop for Removed<K, V> {
    fn drop(&mut self) {
        unsafe { incinerator::add(self.pair, dealloc) }
    }
}

impl<K, V, H> Default for Map<K, V, H>
where
    H: BuildHasher + Default,
{
    fn default() -> Self {
        Self::with_hasher_builder(H::default())
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

impl<K, V, H> fmt::Debug for Map<K, V, H>
where
    H: fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "Map {} hasher_builder = {:?}, entries = ... {}",
            '{', self.builder, '}'
        )
    }
}

impl<K, V> fmt::Debug for Removed<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "Removed {} key = {:?}, val = {:?} {}",
            '{',
            self.key(),
            self.val(),
            '}'
        )
    }
}

impl<K, V, Q, U> PartialEq<Removed<Q, U>> for Removed<K, V>
where
    K: PartialEq<Q>,
    V: PartialEq<U>,
{
    fn eq(&self, other: &Removed<Q, U>) -> bool {
        self.key() == other.key() && self.val() == other.val()
    }
}

impl<K, V, Q, U> PartialEq<(Q, U)> for Removed<K, V>
where
    K: PartialEq<Q>,
    V: PartialEq<U>,
{
    fn eq(&self, (key, val): &(Q, U)) -> bool {
        self.key() == key && self.val() == val
    }
}

impl<K, V, Q, U> PartialOrd<Removed<Q, U>> for Removed<K, V>
where
    K: PartialOrd<Q>,
    V: PartialOrd<U>,
{
    fn partial_cmp(&self, other: &Removed<Q, U>) -> Option<Ordering> {
        self.key().partial_cmp(other.key()).and_then(|ord_a| {
            self.val().partial_cmp(other.val()).map(|ord_b| ord_a.then(ord_b))
        })
    }
}

impl<K, V, Q, U> PartialOrd<(Q, U)> for Removed<K, V>
where
    K: PartialOrd<Q>,
    V: PartialOrd<U>,
{
    fn partial_cmp(&self, (key, val): &(Q, U)) -> Option<Ordering> {
        self.key().partial_cmp(key).and_then(|ord_a| {
            self.val().partial_cmp(val).map(|ord_b| ord_a.then(ord_b))
        })
    }
}

impl<K, V> Ord for Removed<K, V>
where
    K: Ord,
    V: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.key().cmp(other.key()).then_with(|| self.val().cmp(other.val()))
    }
}

impl<K, V> Hash for Removed<K, V>
where
    K: Hash,
    V: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.key().hash(state);
        self.val().hash(state);
    }
}

unsafe impl<K, V, H> Send for Map<K, V, H>
where
    K: Send + Sync,
    V: Send + Sync,
    H: Send + Sync,
{}

unsafe impl<K, V, H> Sync for Map<K, V, H>
where
    K: Send + Sync,
    V: Send + Sync,
    H: Send + Sync,
{}

#[cfg(test)]
mod test {
    use super::*;
    use std::{sync::Arc, thread};

    #[test]
    fn inserts_and_gets() {
        let map = Map::new();
        assert_eq!(map.get("five", |x| *x), None);
        assert!(map.insert("five".to_owned(), 5).is_none());
        assert_eq!(map.get("five", |x| *x), Some(5));
        assert_eq!(map.get("four", |x| *x), None);
        assert!(map.insert("four".to_owned(), 4).is_none());
        assert_eq!(map.get("five", |x| *x), Some(5));
        assert_eq!(map.get("four", |x| *x), Some(4));
        map.get_pair("four", |k, v| {
            assert_eq!(k, "four");
            assert_eq!(*v, 4);
        });
    }

    #[test]
    fn inserts_and_removes() {
        let map = Map::new();
        assert!(map.remove("five").is_none());
        assert!(map.remove("four").is_none());
        map.insert("five".to_owned(), 5);
        let removed = map.remove("five").unwrap();
        assert_eq!(removed, ("five", 5));
        assert!(map.insert("four".to_owned(), 4).is_none());
        map.insert("three".to_owned(), 3);
        assert!(map.remove("two").is_none());
        map.insert("two".to_owned(), 2);
        let removed = map.remove("three").unwrap();
        assert_eq!(removed, ("three", 3));
        let removed = map.remove("two").unwrap();
        assert_eq!(removed, ("two", 2));
        let removed = map.remove("four").unwrap();
        assert_eq!(removed, ("four", 4));
    }

    #[test]
    fn repeated_inserts() {
        let map = Map::new();
        assert!(map.insert("five".to_owned(), 5).is_none());
        assert!(*map.insert("five".to_owned(), 5).unwrap().val() == 5);
    }

    #[test]
    fn multithreaded() {
        let map = Arc::new(Map::new());
        let mut threads = Vec::new();
        for i in 1i64 ..= 20 {
            let map = map.clone();
            threads.push(thread::spawn(move || {
                let prev = map
                    .get(&format!("prefix{}suffix", i - 1), |x| *x)
                    .unwrap_or(0);
                map.insert(format!("prefix{}suffix", i), prev + i);
            }));
        }
        for thread in threads {
            thread.join().expect("thread failed");
        }
        for i in 1i64 ..= 20 {
            assert!(
                map.get(&format!("prefix{}suffix", i), |x| *x > 0).unwrap()
            );
        }
    }
}
