#[macro_use]
extern crate benchsuite;
extern crate lockfree;
extern crate thread_local;

use benchsuite::exec::Target;
use lockfree::tls::ThreadLocal;
use std::{cell::Cell, sync::Arc};
use thread_local::ThreadLocal as LockTls;

#[derive(Debug, Clone, Default)]
struct LockTarget {
    inner: Arc<LockTls<Cell<u128>>>,
}

#[derive(Debug, Clone, Default)]
struct LockfreeTarget {
    inner: Arc<ThreadLocal<Cell<u128>>>,
}

impl Target for LockTarget {
    #[inline(always)]
    fn round(&mut self) {
        let cell = self.inner.get_or(|| Box::new(Cell::new(0)));
        cell.set(cell.get().wrapping_add(1));
    }
}

impl Target for LockfreeTarget {
    #[inline(always)]
    fn round(&mut self) {
        self.inner.with_init(
            || Cell::new(0),
            |cell| cell.set(cell.get().wrapping_add(1)),
        );
    }
}

fn main() {
    bench! {
        levels 1, 4, 64, 256;
        "lock" => LockTarget::default(),
        "lockfree" => LockfreeTarget::default(),
    }
}
