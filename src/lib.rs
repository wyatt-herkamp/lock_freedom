#![feature(allocator_api, const_fn)]

use std::alloc::{oom as alloc_oom, AllocErr};

/// Provides convenient re-exports.
pub mod prelude;

/// Provides a queue without FIFO garantees on multithread environments,
/// but still concurrent and lock-free. Single thread environments still
/// have FIFO garanteees.
pub mod loose_queue;

/// Hazard pointer API. Please, note that this API is not implemented in
/// the "traditional" way hazard pointers are implemented. This is because
/// this implemention aims to be lock-free.
pub mod hazard;

fn oom<T>(e: AllocErr) -> T {
    eprintln!("{}", e);
    alloc_oom()
}
