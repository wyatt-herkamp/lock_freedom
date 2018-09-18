#![warn(missing_docs)]
//! A crate providing lock-free data structures and a solution for the "ABA
//! problem" related to pointers.
//!
//! The incinerator is the API which tries to solve the "ABA problem" when
//! related to pointer dropping and other kinds of resource reclamation. In
//! older versions, incinerator was a global state + thread local state.
//! Currently, it is a "per-object" storage, which has a much better
//! performance.
//!
//! This crate is under development, and there are plans for some structures.
//! We have:
//! - `[x]` Stack
//! - `[x]` Queue
//! - `[ ]` Deque
//! - `[x]` Map
//! - `[x]` Set
//!
//! # Performance Guide
//! In order to achieve a better time performance with lockfree, it is
//! recommended to avoid global locking things like heap allocation.

#[allow(dead_code)]
mod alloc;

/// Provides convenient re-exports.
pub mod prelude;

/// Incinerator API. The purpouse of this module is to solve the "ABA problem"
/// related to resource reclamation while still being lock-free. Incinerator is
/// a garbage deleter which does not necessarilly deletes all garbage at the
/// moment it was requested, i.e. it can be paused while still not blocking any
/// thread. See module items for more details.
pub mod incinerator;
/*

/// Atomic abstractions, such an atomic trait and atomic boxes.
pub mod atomic;

*/
/// A lock-free queue.
pub mod queue;
/*

/// A lock-free stack.
pub mod stack;

/// A lock-free map.
pub mod map;

/// A lock-free set.
pub mod set;

/// Provides a doubly atomic reference counter.
pub mod darc;
*/
