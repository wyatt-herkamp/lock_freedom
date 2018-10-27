#[macro_use]
extern crate benchsuite;
extern crate lockfree;
extern crate thread_local;

use benchsuite::exec::Target;
use lockfree::tls::ThreadLocal;
use std::{cell::Cell, sync::Arc};
use thread_local::ThreadLocal as LockTls;

#[derive(Debug, Clone, Default)]
struct BlockingTarget {
    inner: Arc<LockTls<Cell<u128>>>,
}

#[derive(Debug, Clone, Default)]
struct LockfreeTarget {
    inner: Arc<ThreadLocal<Cell<u128>>>,
}

#[derive(Debug, Clone, Default)]
struct StdTarget;

thread_local! {
    static STATIC: Cell<u128> = Cell::new(0);
}

impl Target for BlockingTarget {
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

impl Target for StdTarget {
    #[inline(always)]
    fn round(&mut self) {
        STATIC.with(|cell| cell.set(cell.get().wrapping_add(1)))
    }
}

fn main() {
    bench! {
        levels 1, 4, 16, 32, 128, 512;
        "std/global" => StdTarget::default(),
        "blocking" => BlockingTarget::default(),
        "lockfree" => LockfreeTarget::default(),
    }
}
