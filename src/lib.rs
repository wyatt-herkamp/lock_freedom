#![warn(missing_docs)]
//! A crate providing lock-free data structures and a solution for the "ABA
//! problem" related to pointers.
//!
//! The incinerator is the API which tries to solve the "ABA problem" when
//! related to pointer dropping. With incinerator, every thread has a local
//! deletion queue. Dropping a shared consist of first removing the pointer
//! from the shared context, then adding the pointer to the local queue. Then,
//! a global counter is checked. If the counter is zero, then the whole queue
//! is deleted, otherwise, the queue will only be deleted later.
//!
//! This counter is counting how many times the incinerator was asked to
//! "pause". A thread may pause the incinerator to load and use the shared
//! pointer, and this is why it is important to remove the pointer from the
//! shared context before deleting.
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

extern crate owned_alloc;

#[allow(dead_code)]
mod alloc;

/// Provides convenient re-exports.
pub mod prelude;

/// Incinerator API. The purpouse of this module is to solve the "ABA problem"
/// related to pointers while still being lock-free. See documentation of the
/// inner type for more details.
pub mod incinerator;

/// Lock-free per-object Thread Local Storage (TLS).
pub mod tls;

/// Atomic abstractions, such an atomic trait and atomic boxes.
pub mod atomic;

/// A lock-free queue.
pub mod queue;

/// A lock-free stack.
pub mod stack;

/// A lock-free map.
pub mod map;

/// A lock-free set.
pub mod set;

/// Provides a doubly atomic reference counter.
pub mod darc;

#[allow(dead_code)]
mod compat;
