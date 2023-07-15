#![warn(missing_docs)]
//! Owned Allocations. A crate to help reducing manual memory management errors.
//!
//! The idea is to use a type like `UninitAlloc` for uninitialized dynamic
//! allocations. After initializing it, you have a `OwnedAlloc` which is pretty
//! similar to a `Box`. However, unlike a `Box`, you may move the value out from
//! the `OwnedAlloc` and getting an `UninitAlloc` back.
//!
//! For vec-like structures, a type `RawVec` is available, pretty similar to the
//! one used by the standard library. Currently, no other help is provided for
//! arrays/vectors.
//!
//! There is also a type `Cache`, which is actually more general than
//! allocation, but may be useful for allocations. It can save unused
//! allocations requested on a tight loop.

mod uninit;
mod owned;
mod cache;
mod raw_vec;
mod maybe_uninit;
mod err;

pub use self::{
    cache::Cache,
    err::{AllocErr, LayoutErr, RawVecErr},
    maybe_uninit::MaybeUninitAlloc,
    owned::OwnedAlloc,
    raw_vec::RawVec,
    uninit::UninitAlloc,
};
