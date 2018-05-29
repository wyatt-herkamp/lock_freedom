#![feature(allocator_api, const_fn)]

use std::alloc::{oom as alloc_oom, AllocErr};

/// Provides convenient re-exports.
pub mod prelude;

/// Hazard pointer API. Please, note that this API is not implemented in
/// the "traditional" way hazard pointers are implemented. This is because
/// this implemention aims to be lock-free.
pub mod hazard;

/// Provides a queue with strict FIFO semanthics in single and multithread
/// environments.
pub mod queue;

/// Provides a queue without FIFO garantees on multithread environments,
/// but still concurrent and lock-free. Single thread environments still
/// have FIFO garanteees. This queue does not use the hazard API.
pub mod loose_queue;

fn oom<T>(e: AllocErr) -> T {
    eprintln!("{}", e);
    alloc_oom()
}
