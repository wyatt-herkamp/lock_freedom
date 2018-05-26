#![feature(allocator_api)]

/// Provides convenient re-exports.
pub mod prelude;

/// Provides a queue without FIFO garantees on multithread environments,
/// but still concurrent and lock-free. Single thread environments still
/// have FIFO garanteees.
pub mod queue;
