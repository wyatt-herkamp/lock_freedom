#![feature(allocator_api, const_fn)]

use std::alloc::{oom as alloc_oom, AllocErr};

/// Provides convenient re-exports.
pub mod prelude;

/// Provides a queue without FIFO garantees on multithread environments,
/// but still concurrent and lock-free. Single thread environments still
/// have FIFO garanteees.
pub mod loose_queue;

fn oom<T>(e: AllocErr) -> T {
    eprintln!("{}", e);
    alloc_oom()
}
