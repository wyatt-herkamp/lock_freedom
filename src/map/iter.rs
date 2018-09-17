use super::table::{Node, Table};
use incinerator;
use std::{collections::VecDeque, sync::atomic::Ordering::*};

/// An iterator over `Map`. The `Item` of this iterator is a vector of entries
/// with the same hash. It is like that because there are some limitations on
/// how the iterator can stop navigating the map.
pub struct Iter<'map, K, V>
where
    K: 'map,
    V: 'map,
{
    tables: VecDeque<&'map Table<K, V>>,
    index: usize,
}

impl<'map, K, V> Iter<'map, K, V> {
    pub(crate) fn with_table(table: &'map Table<K, V>) -> Self {
        Self {
            tables: {
                let mut deque = VecDeque::with_capacity(1);
                deque.push_back(table);
                deque
            },
            index: 0,
        }
    }
}

impl<'map, K, V> Iterator for Iter<'map, K, V> {
    type Item = Vec<(&'map K, &'map V)>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut table = *self.tables.front()?;
        let mut done = false;
        loop {
            let res = incinerator::pause(|| {
                done = true;

                match table.nodes().get(self.index) {
                    Some(node) => {
                        self.index += 1;
                        match unsafe { node.load(Relaxed).as_ref() } {
                            Some(Node::Leaf(bucket)) => {
                                let vec = unsafe { bucket.collect() };
                                if vec.len() > 0 {
                                    return Some(vec);
                                }
                            },

                            Some(Node::Branch(ptr)) => {
                                self.tables.push_back(unsafe { &*ptr.as_ptr() })
                            },

                            None => (),
                        }
                    },

                    None => {
                        self.index = 0;
                        self.tables.pop_front();
                        table = *self.tables.front()?;
                    },
                }

                done = false;
                None
            });

            if done {
                break res;
            }
        }
    }
}
