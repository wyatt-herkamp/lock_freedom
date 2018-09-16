use super::bucket::{Bucket, Entry, List, Pair};
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

    pub unsafe fn insert(
        &self,
        pair: NonNull<Pair<K, V>>,
        hash: u64,
    ) -> *mut Pair<K, V>
    where
        K: Ord,
    {
        let entry = Entry::new(pair.as_ptr(), null_mut());
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
                                Release,
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

                None => break null_mut(),
            }
        }
    }

    pub unsafe fn get<Q>(&self, key: &Q, hash: u64) -> *mut Pair<K, V>
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
                        Some(x) => break x,

                        None => {
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
                                break null_mut();
                            }
                        },
                    }
                },

                Some(Node::Branch(new_table)) => {
                    table = &*new_table.as_ptr();
                    index >>= BITS as u64;
                },

                _ => break null_mut(),
            }
        }
    }

    pub unsafe fn remove<Q>(&self, key: &Q, hash: u64) -> *mut Pair<K, V>
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
                        Some((pair, delete)) => {
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
                            break pair;
                        },

                        None => {
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
                                break null_mut();
                            }
                        },
                    }
                },

                Some(Node::Branch(new_table)) => {
                    table = &*new_table.as_ptr();
                    index >>= BITS as u64;
                },

                _ => break null_mut(),
            }
        }
    }
}
