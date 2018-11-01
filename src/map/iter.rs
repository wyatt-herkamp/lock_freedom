use super::{
    bucket::{Bucket, Garbage, List},
    guard::ReadGuard,
    table::Table,
};
use incin::Pause;
use std::mem::replace;

#[derive(Debug)]
pub struct Iter<'origin, K, V>
where
    K: 'origin,
    V: 'origin,
{
    pause: Pause<'origin, Garbage<K, V>>,
    tables: Vec<&'origin Table<K, V>>,
    curr_table: Option<(&'origin Table<K, V>, usize)>,
    cache: Vec<ReadGuard<'origin, K, V>>,
}

impl<'origin, K, V> Iter<'origin, K, V> {
    pub(super) fn new(
        pause: Pause<'origin, Garbage<K, V>>,
        top: &'origin Table<K, V>,
    ) -> Self {
        Self {
            pause,
            tables: Vec::new(),
            curr_table: Some((top, 0)),
            cache: Vec::new(),
        }
    }
}

impl<'origin, K, V> Iterator for Iter<'origin, K, V> {
    type Item = ReadGuard<'origin, K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(guard) = self.cache.pop() {
                break Some(guard);
            }

            let (table, index) = self.curr_table?;
            self.curr_table = match table.load_index(index) {
                Some(ptr) if ptr.is_null() => Some((table, index + 1)),

                Some(ptr) if ptr as usize & 1 == 0 => unsafe {
                    let ptr = ptr as *mut Bucket<K, V>;
                    let mut cache = replace(&mut self.cache, Vec::new());

                    (*ptr).collect(self.pause.incin(), &mut cache, |pair| {
                        ReadGuard::new(pair, self.pause.clone())
                    });

                    self.cache = cache;
                    Some((table, index + 1))
                },

                Some(ptr) => {
                    let ptr = (ptr as usize & !1) as *mut Table<K, V>;
                    self.tables.push(unsafe { &*ptr });
                    Some((table, index + 1))
                },

                None => self.tables.pop().map(|tbl| (tbl, 0)),
            };
        }
    }
}
