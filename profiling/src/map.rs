extern crate lockfree;

use lockfree::prelude::*;
use std::{sync::Arc, thread};

fn thread_main(i: usize, map: Arc<Map<String, usize>>) {
    for j in 0 .. 20 {
        let key = format!("{}{}{:b}", j, i + j, i + j);
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
