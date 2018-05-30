/// Provides convenient re-exports.
pub mod prelude;

/// Hazard pointer API. Please, note that this API is not implemented in
/// the "traditional" way hazard pointers are implemented. This is because
/// this implemention aims to be lock-free.
pub mod hazard;

/// A lock-free queue.
pub mod queue;

/// A lock-free stack.
pub mod stack;

/// Provides a doubly atomic reference counter.
pub mod darc;

#[allow(dead_code)]
mod alloc;
