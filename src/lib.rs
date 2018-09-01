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
//! - [x] Stack
//! - [x] Queue
//! - [ ] Deque
//! - [ ] Map
//! - [ ] Set

/// Provides convenient re-exports.
pub mod prelude;

/// Incinerator API. The purpouse of this module is to solve the "ABA problem"
/// related to pointers while still being lock-free. Incinerator is a garbage
/// deleter which does not necessarilly deletes all garbage at the moment it
/// was added to the queue, i.e. it can be paused while still not-blocking any
/// thread.
///
/// Whenever a thread exits, its garbage queue is dropped.
/// # Example
/// ```rust
/// extern crate lockfree;
///
/// use lockfree::prelude::*;
/// use std::{
///     ptr::{null_mut, NonNull},
///     sync::{
///         atomic::{AtomicPtr, Ordering::*},
///         Arc,
///     },
///     thread,
/// };
///
/// unsafe fn create_ptr<T>(val: T) -> NonNull<T> {
///     NonNull::new_unchecked(Box::into_raw(Box::new(val)))
/// }
///
/// unsafe fn drop_ptr<T>(ptr: NonNull<T>) {
///     Box::from_raw(ptr.as_ptr());
/// }
///
/// let ptr = unsafe { create_ptr(55).as_ptr() };
/// let dummy_state = Arc::new(AtomicPtr::new(ptr));
///
/// let mut threads = Vec::with_capacity(16);
///
/// for i in 0 .. 16 {
///     let state = dummy_state.clone();
///     threads.push(thread::spawn(move || {
///         let ptr = incinerator::pause(|| {
///             let loaded = state.load(SeqCst);
///             let new = if let Some(num) = unsafe { loaded.as_ref() } {
///                 i + num
///             } else {
///                 i
///             };
///             state.swap(unsafe { create_ptr(new).as_ptr() }, SeqCst)
///         });
///
///         if let Some(nnptr) = NonNull::new(ptr) {
///             // dropping
///             unsafe { incinerator::add(nnptr, drop_ptr) }
///         }
///     }));
/// }
///
/// for thread in threads {
///     thread.join().unwrap();
/// }
///
/// if let Some(ptr) = NonNull::new(dummy_state.load(SeqCst)) {
///     assert!(unsafe { *ptr.as_ref() } <= 15 * 15);
///     unsafe { drop_ptr(ptr) }
/// }
/// ```
pub mod incinerator;

/// Atomic abstractions, such an atomic trait and atomic boxes.
pub mod atomic;

/// A lock-free queue.
pub mod queue;

/// A lock-free stack.
pub mod stack;

/// Provides a doubly atomic reference counter.
#[deprecated(
    since = "0.2.0",
    note = "this API has been found too slow, messy, and not that useful for \
            such slowness"
)]
pub mod darc;

#[allow(dead_code)]
mod alloc;
