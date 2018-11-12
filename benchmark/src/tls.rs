#[macro_use]
extern crate benchsuite;
extern crate lockfree;
extern crate thread_local;

use benchsuite::exec::{Target, TargetData};
use lockfree::tls::{CachedId, ThreadLocal};
use std::{cell::Cell, sync::Arc};
use thread_local::{
    CachedThreadLocal as CachedLockTls,
    ThreadLocal as LockTls,
};

#[derive(Debug, Clone, Default)]
struct BlockingTarget {
    inner: Arc<LockTls<Cell<u128>>>,
}

#[derive(Debug, Clone, Default)]
struct BlkCachedTarget {
    inner: Arc<CachedLockTls<Cell<u128>>>,
}

#[derive(Debug, Clone, Default)]
struct LockfreeTarget {
    inner: Arc<ThreadLocal<Cell<u128>>>,
}

#[derive(Debug, Clone, Default)]
struct LfCachedData {
    inner: Arc<ThreadLocal<Cell<u128>>>,
}

#[derive(Debug)]
struct LfCachedTarget {
    shared: LfCachedData,
    id: CachedId,
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

impl Target for BlkCachedTarget {
    #[inline(always)]
    fn round(&mut self) {
        let cell = self.inner.get_or(|| Box::new(Cell::new(0)));
        cell.set(cell.get().wrapping_add(1));
    }
}

impl Target for LockfreeTarget {
    #[inline(always)]
    fn round(&mut self) {
        let cell = self.inner.with_init(|| Cell::new(0));
        cell.set(cell.get().wrapping_add(1));
    }
}

impl TargetData for LfCachedData {
    type Target = LfCachedTarget;

    fn init_thread(self) -> Self::Target {
        LfCachedTarget {
            shared: self,
            id: CachedId::load(),
        }
    }
}

impl Target for LfCachedTarget {
    #[inline(always)]
    fn round(&mut self) {
        let cell = self.shared.inner.with_id_and_init(self.id, || Cell::new(0));
        cell.set(cell.get().wrapping_add(1));
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
        levels 1, 4, 16, 32, 128;
        "std/global" => StdTarget::default(),
        "blocking" => BlockingTarget::default(),
        "blocking with cached access" => BlkCachedTarget::default(),
        "lockfree" => LockfreeTarget::default(),
        "lockfree with cached id" => LfCachedData::default(),
    }
}
