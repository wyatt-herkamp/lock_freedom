use super::{
    bucket::{Bucket, Entry, GetRes, InsertRes, List, Pair, RemoveRes},
    insertion::{Inserter, PreviewAlloc},
};
use alloc::*;
use incinerator;
use std::{
    borrow::Borrow,
    mem,
    ptr::{null_mut, NonNull},
    sync::atomic::{AtomicPtr, Ordering::*},
};

const BITS: usize = 8;

pub enum Node<K, V> {
    Leaf(Bucket<K, V>),
    Branch(NonNull<Table<K, V>>),
}

pub struct Table<K, V> {
    nodes: [AtomicPtr<Node<K, V>>; 1 << BITS],
}

impl<K, V> Table<K, V> {
    pub fn new() -> Self {
        let mut this = Self { nodes: unsafe { mem::uninitialized() } };
        unsafe { Self::write_new(NonNull::from(&mut this)) }
        this
    }

    pub unsafe fn write_new(mut ptr: NonNull<Self>) {
        for node in &mut ptr.as_mut().nodes as &mut [_] {
            (node as *mut AtomicPtr<_>).write(AtomicPtr::new(null_mut()))
        }
    }

    pub fn nodes(&self) -> &[AtomicPtr<Node<K, V>>] {
        &self.nodes
    }

    pub fn nodes_mut(&mut self) -> &mut [AtomicPtr<Node<K, V>>] {
        &mut self.nodes
    }

    pub unsafe fn insert<I>(
        &self,
        hash: u64,
        preview: &mut PreviewAlloc<K, V>,
        inserter: &mut I,
    ) -> Option<NonNull<Pair<K, V>>>
    where
        K: Ord,
        I: Inserter<K, V>,
    {
        if !inserter.update_and_test(preview, None) {
            return self.insert_non_new(hash, preview, inserter);
        }

        let entry = Entry::new(preview.ptr().as_ptr(), null_mut());
        let list = alloc(List::new(entry));
        let root = Entry::root(list.as_ptr());
        let bucket = Bucket::new(hash, List::new(root));
        let node = alloc(Node::Leaf(bucket));
        let mut table_ptr = CachedAlloc::empty();
        let mut branch_ptr = CachedAlloc::<Node<K, V>>::empty();

        let mut table = self;
        let mut index = hash;
        let mut depth = 1;

        loop {
            let node_index = index as usize & (1 << BITS) - 1;
            let old = table.nodes[node_index].compare_and_swap(
                null_mut(),
                node.as_ptr(),
                AcqRel,
            );
            match old.as_ref() {
                Some(Node::Leaf(in_place)) if in_place.hash() == hash => {
                    match in_place.insert(preview, inserter) {
                        InsertRes::Updated(ptr) => {
                            dealloc(node);
                            dealloc(list);
                            break Some(ptr);
                        },

                        InsertRes::Created => break None,

                        InsertRes::Failed => {
                            preview.discard();
                            break None;
                        },

                        InsertRes::Delete => {
                            let res = table.nodes[node_index].compare_and_swap(
                                old,
                                node.as_ptr(),
                                Release,
                            );

                            if res == old {
                                incinerator::add(
                                    NonNull::new_unchecked(res),
                                    dealloc,
                                );
                                break None;
                            }
                        },
                    }
                },

                Some(Node::Leaf(in_place)) => {
                    let nnptr = table_ptr.get_or(|x| Table::write_new(x));
                    let branch = branch_ptr
                        .get_or(|x| x.as_ptr().write(Node::Branch(nnptr)));
                    let new_table = &*nnptr.as_ptr();

                    let shifted = in_place.hash() >> (depth * BITS as u64);
                    let in_place_index = shifted as usize & (1 << BITS) - 1;

                    new_table.nodes[in_place_index].store(old, Relaxed);
                    let res = table.nodes[node_index].compare_and_swap(
                        old,
                        branch.as_ptr(),
                        Release,
                    );

                    if res == old {
                        table = new_table;
                        index >>= BITS as u64;
                        depth += 1;
                        table_ptr.take();
                        branch_ptr.take();
                    } else {
                        new_table.nodes[in_place_index]
                            .store(null_mut(), Relaxed);
                    }
                },

                Some(Node::Branch(new_table)) => {
                    table = &*new_table.as_ptr();
                    index >>= BITS as u64;
                    depth += 1;
                },

                None => break None,
            }
        }
    }

    unsafe fn insert_non_new<I>(
        &self,
        hash: u64,
        preview: &mut PreviewAlloc<K, V>,
        inserter: &mut I,
    ) -> Option<NonNull<Pair<K, V>>>
    where
        K: Ord,
        I: Inserter<K, V>,
    {
        let mut table = self;
        let mut index = hash;
        let mut depth = 1;

        loop {
            let node_index = index as usize & (1 << BITS) - 1;
            let in_place = table.nodes[node_index].load(Acquire);
            match in_place.as_ref() {
                Some(Node::Leaf(bucket)) if bucket.hash() == hash => {
                    match bucket.insert(preview, inserter) {
                        InsertRes::Updated(ptr) => break Some(ptr),

                        InsertRes::Created => break None,

                        InsertRes::Failed => {
                            preview.discard();
                            break None;
                        },

                        InsertRes::Delete => {
                            let res = table.nodes[node_index].compare_and_swap(
                                in_place,
                                null_mut(),
                                Release,
                            );

                            if res == in_place {
                                incinerator::add(
                                    NonNull::new_unchecked(res),
                                    dealloc,
                                );
                                preview.discard();
                                break None;
                            }
                        },
                    }
                },

                Some(Node::Branch(new_table)) => {
                    table = &*new_table.as_ptr();
                    index >>= BITS as u64;
                    depth += 1;
                },

                _ => {
                    preview.discard();
                    break None;
                },
            }
        }
    }

    pub unsafe fn get<Q>(
        &self,
        key: &Q,
        hash: u64,
    ) -> Option<NonNull<Pair<K, V>>>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        let mut table = self;
        let mut index = hash;

        loop {
            let node_index = index as usize & (1 << BITS) - 1;
            let in_place = table.nodes[node_index].load(Acquire);
            match in_place.as_ref() {
                Some(Node::Leaf(bucket)) if bucket.hash() == hash => {
                    match bucket.get(key) {
                        GetRes::Found(x) => break Some(x),

                        GetRes::NotFound => break None,

                        GetRes::Delete => {
                            let res = table.nodes[node_index].compare_and_swap(
                                in_place,
                                null_mut(),
                                Release,
                            );

                            if res == in_place {
                                incinerator::add(
                                    NonNull::new_unchecked(res),
                                    dealloc,
                                );
                                break None;
                            }
                        },
                    }
                },

                Some(Node::Branch(new_table)) => {
                    table = &*new_table.as_ptr();
                    index >>= BITS as u64;
                },

                _ => break None,
            }
        }
    }

    pub unsafe fn remove<Q>(
        &self,
        key: &Q,
        hash: u64,
    ) -> Option<NonNull<Pair<K, V>>>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        let mut table = self;
        let mut index = hash;

        loop {
            let node_index = index as usize & (1 << BITS) - 1;
            let in_place = table.nodes[node_index].load(Acquire);
            match in_place.as_ref() {
                Some(Node::Leaf(bucket)) if bucket.hash() == hash => {
                    match bucket.remove(key) {
                        RemoveRes::Removed { pair, delete } => {
                            if delete {
                                let res = table.nodes[node_index]
                                    .compare_and_swap(
                                        in_place,
                                        null_mut(),
                                        Release,
                                    );

                                if res == in_place {
                                    incinerator::add(
                                        NonNull::new_unchecked(res),
                                        dealloc,
                                    );
                                }
                            }
                            break Some(pair);
                        },

                        RemoveRes::NotFound => break None,

                        RemoveRes::Delete => {
                            let res = table.nodes[node_index].compare_and_swap(
                                in_place,
                                null_mut(),
                                Release,
                            );

                            if res == in_place {
                                incinerator::add(
                                    NonNull::new_unchecked(res),
                                    dealloc,
                                );
                                break None;
                            }
                        },
                    }
                },

                Some(Node::Branch(new_table)) => {
                    table = &*new_table.as_ptr();
                    index >>= BITS as u64;
                },

                _ => break None,
            }
        }
    }
}
