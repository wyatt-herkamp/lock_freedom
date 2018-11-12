use std::{
    marker::PhantomData,
    sync::atomic::{AtomicUsize, Ordering::*},
};

/// A cached thread-id. Repeated calls to [`ThreadLocal`](super::ThreadLocal)'s
/// methods with cached IDs should be faster than reloading the ID everytime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreadId {
    bits: usize,
    _non_tsafe: PhantomData<*mut ()>,
}

impl ThreadId {
    /// Loads the ID for this thread.
    #[inline]
    pub fn current() -> Self {
        ID.with(|id| Self {
            bits: id.bits,
            _non_tsafe: PhantomData,
        })
    }

    pub(super) fn bits(self) -> usize {
        self.bits
    }
}

impl Default for ThreadId {
    fn default() -> Self {
        Self::current()
    }
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

thread_local! {
    static ID: IdGuard = IdGuard::new();
}

struct IdGuard {
    bits: usize,
}

impl IdGuard {
    fn new() -> Self {
        loop {
            let bits = COUNTER.load(Acquire);
            let new = bits.checked_add(1).expect("Too many threads");
            if COUNTER.compare_and_swap(bits, new, Release) == bits {
                break Self { bits };
            }
        }
    }
}

impl Drop for IdGuard {
    fn drop(&mut self) {
        COUNTER.compare_and_swap(self.bits + 1, self.bits, Release);
    }
}
