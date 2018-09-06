#[macro_use]
extern crate benchsuite;
extern crate lockfree;

use benchsuite::exec::Target;
use lockfree::map::Map;
use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, Mutex},
};

type LockfreeInner = Arc<Map<String, usize>>;
type MutexInner = Arc<Mutex<HashMap<String, usize>>>;

#[derive(Debug, Clone, Default)]
struct MutexInsert {
    inner: MutexInner,
    i: RefCell<usize>,
}

#[derive(Debug, Clone, Default)]
struct LockfreeInsert {
    inner: LockfreeInner,
    i: RefCell<usize>,
}

impl Target for MutexInsert {
    #[inline(always)]
    fn round(&self) {
        let i = {
            let mut ptr = self.i.borrow_mut();
            let res = *ptr;
            *ptr += 1;
            res
        };
        let mut map = self.inner.lock().unwrap();
        map.insert(format!("{i}-{i:064b}", i = i), i);
    }
}

impl Target for LockfreeInsert {
    #[inline(always)]
    fn round(&self) {
        let i = {
            let mut ptr = self.i.borrow_mut();
            let res = *ptr;
            *ptr += 1;
            res
        };
        self.inner.insert(format!("{i}-{i:064b}", i = i), i);
    }
}

fn main() {
    bench! {
        levels 1, 4, 16, 32;
        "mutex" => MutexInsert::default(),
        "lockfree" => LockfreeInsert::default(),
    }
}
