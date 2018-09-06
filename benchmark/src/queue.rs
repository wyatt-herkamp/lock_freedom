#[macro_use]
extern crate benchsuite;
extern crate lockfree;

use benchsuite::exec::Target;
use lockfree::queue::Queue;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Default)]
struct MutexTarget {
    inner: Arc<Mutex<VecDeque<u8>>>,
}

#[derive(Debug, Clone, Default)]
struct LockfreeTarget {
    inner: Arc<Queue<u8>>,
}

impl Target for MutexTarget {
    #[inline(always)]
    fn round(&self) {
        let mut stack = self.inner.lock().unwrap();
        stack.pop_front();
        stack.push_back(234);
    }
}

impl Target for LockfreeTarget {
    #[inline(always)]
    fn round(&self) {
        self.inner.pop();
        self.inner.push(234);
    }
}

fn main() {
    bench! {
        levels 1, 4, 16, 32;
        "mutex" => MutexTarget::default(),
        "lockfree" => LockfreeTarget::default(),
    }
}
