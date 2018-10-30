mod owned;

pub use self::owned::{
    OptionOwnedAccessPtr,
    OptionOwnedCasError,
    OptionOwnedPreview,
    OwnedAccessPtr,
    OwnedCasError,
    OwnedPreview,
};
use owned_alloc::OwnedAlloc;
use std::ptr::{null_mut, NonNull};
pub use std::sync::atomic::Ordering;

#[derive(Debug)]
pub struct CasError<T, P> {
    pub desired: T,
    pub expected: P,
    pub found: P,
}

unsafe fn raw_to_owned_alloc<T>(raw: *mut T) -> Option<OwnedAlloc<T>> {
    NonNull::new(raw).map(|nnptr| OwnedAlloc::from_raw(nnptr))
}

fn owned_alloc_to_raw<T>(alloc: Option<OwnedAlloc<T>>) -> *mut T {
    alloc.map_or(null_mut(), |alloc| alloc.into_raw().as_ptr())
}
