use super::table::{Node, Table};
use incinerator;
use std::{collections::VecDeque, fmt, sync::atomic::Ordering::*};

/// An iterator over `Map`. As in the `Map`'s `get` method, we cannot return a
/// reference to the elements. Therefore, the iterator has a reader which reads
/// a temporary reference and returns the actual iterator `Item`.
pub struct Iter<'map, K, V, R>
where
    K: 'map,
    V: 'map,
    R: IterReader<K, V>,
{
    tables: VecDeque<&'map Table<K, V>>,
    index: usize,
    cache: Vec<R::Out>,
    reader: R,
}

impl<'map, K, V, R> Iter<'map, K, V, R>
where
    R: IterReader<K, V>,
{
    pub(crate) fn with_table(table: &'map Table<K, V>, reader: R) -> Self {
        Self {
            tables: {
                let mut deque = VecDeque::with_capacity(1);
                deque.push_back(table);
                deque
            },
            index: 0,
            cache: Vec::new(),
            reader,
        }
    }
}

impl<'map, K, V, R> Iterator for Iter<'map, K, V, R>
where
    R: IterReader<K, V>,
{
    type Item = R::Out;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.cache.pop() {
            return Some(val);
        }

        let mut table = *self.tables.front()?;
        let mut stop = false;
        loop {
            incinerator::pause(|| match table.nodes().get(self.index) {
                Some(node) => {
                    self.index += 1;
                    match unsafe { node.load(Relaxed).as_ref() } {
                        Some(Node::Leaf(bucket)) => {
                            for (k, v) in unsafe { bucket.collect() } {
                                let elem = self.reader.read(k, v);
                                self.cache.push(elem);
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
                    match self.tables.front() {
                        Some(x) => table = *x,
                        None => stop = true,
                    };
                },
            });

            if stop {
                break None;
            }

            if let Some(val) = self.cache.pop() {
                break Some(val);
            }
        }
    }
}

/// A reader on map's iterator's temporary references.
pub trait IterReader<K, V> {
    /// The actual item of the iterator which is coupled with this reader.
    type Out;

    /// Transforms the temporary references into the iterator item.
    fn read(&mut self, key: &K, val: &V) -> Self::Out;
}

impl<K, V, F, T> IterReader<K, V> for F
where
    F: FnMut(&K, &V) -> T,
{
    type Out = T;

    fn read(&mut self, key: &K, val: &V) -> Self::Out {
        self(key, val)
    }
}

impl<'map, K, V, R> fmt::Debug for Iter<'map, K, V, R>
where
    R: IterReader<K, V>,
    R::Out: fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "Iter {} cache: {:?}, tables.len(): {}, index: {} {}",
            '{',
            self.cache,
            self.tables.len(),
            self.index,
            '}'
        )
    }
}
