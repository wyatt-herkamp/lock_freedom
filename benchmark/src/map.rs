#[macro_use]
extern crate benchsuite;
extern crate lockfree;

use benchsuite::exec::Target;
use lockfree::map::Map;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

type MutexInner = Arc<Mutex<HashMap<String, usize>>>;
type LockfreeInner = Arc<Map<String, usize>>;

#[derive(Debug, Clone, Default)]
struct MutexInsert {
    inner: MutexInner,
    i: usize,
}

#[derive(Debug, Clone, Default)]
struct LockfreeInsert {
    inner: LockfreeInner,
    i: usize,
}

impl Target for MutexInsert {
    #[inline(always)]
    fn round(&mut self) {
        let i = self.i;
        self.i += 1;
        let mut map = self.inner.lock().unwrap();
        map.insert(format!("{i}{i:064b}", i = i), i);
        if i % 5 == 0 {
            map.remove(&format!("{i}{i:064b}", i = i / 5));
        }
    }
}

impl Target for LockfreeInsert {
    #[inline(always)]
    fn round(&mut self) {
        let i = self.i;
        self.i += 1;
        self.inner.insert(format!("{i}{i:064b}", i = i), i);
    }
}

fn main() {
    bench! {
        levels 1, 4, 16, 32;
        "mutex" => MutexInsert::default(),
        "lockfree" => LockfreeInsert::default(),
    }
}
