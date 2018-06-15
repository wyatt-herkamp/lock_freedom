/// Provides convenient re-exports.
pub mod prelude;

/// Incinerator API. The purpouse of this module is to solve the "ABA problem"
/// related to pointers while still being lock-free. Incinerator is a garbage
/// deleter which does not necessarilly deletes all garbage at the moment it
/// was added to the queue, i.e. it can be paused while still not-blocking any
/// thread.
pub mod incinerator;

/// A lock-free queue.
pub mod queue;

/// A lock-free stack.
pub mod stack;

/// Provides a doubly atomic reference counter.
pub mod darc;

#[allow(dead_code)]
mod alloc;
