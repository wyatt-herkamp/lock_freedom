extern crate lockfree;

use lockfree::prelude::*;
use std::{
    hash::{Hash, Hasher},
    sync::Arc,
    thread,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct BadHash(u128);

impl Hash for BadHash {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        hasher.write_u8(self.0 as u8);
        hasher.write_u8((self.0 >> 16) as u8);
        hasher.write_u8((self.0 >> 32) as u8);
        hasher.write_u8((self.0 >> 48) as u8);
        hasher.write_u8((self.0 >> 64) as u8);
        hasher.write_u8((self.0 >> 80) as u8);
        hasher.write_u8((self.0 >> 96) as u8);
        hasher.write_u8((self.0 >> 112) as u8);
    }
}

fn thread_main(i: usize, map: Arc<Map<BadHash, usize>>) {
    for j in 0 .. 20 {
        let inner = (i as u128)
            .wrapping_mul(i as u128 ^ j as u128 >> 16)
            .wrapping_add(j as u128);
        let key = BadHash(inner);
        let prev = map.get(&key, |&x| x).unwrap_or(0);
        map.insert(key, prev + i + j);
    }
}

fn main() {
    let mut threads = Vec::new();
    let map = Arc::new(Map::new());
    for i in 0 .. 4 {
        let mut map = map.clone();
        threads.push(thread::spawn(move || thread_main(i, map)))
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
