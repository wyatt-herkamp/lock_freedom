#[macro_use]
extern crate benchsuite;
extern crate lockfree;

use benchsuite::exec::Target;
use lockfree::stack::Stack;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
struct MutexTarget {
    inner: Arc<Mutex<Vec<u8>>>,
}

#[derive(Debug, Clone, Default)]
struct LockfreeTarget {
    inner: Arc<Stack<u8>>,
}

impl Target for MutexTarget {
    fn round(&self) {
        let mut stack = self.inner.lock().unwrap();
        stack.pop();
        stack.push(234);
    }
}

impl Target for LockfreeTarget {
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
