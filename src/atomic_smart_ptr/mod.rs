mod no_acc;

use owned_alloc::OwnedAlloc;
use std::{
    mem::forget,
    ptr::{null_mut, NonNull},
};

pub use self::no_acc::{NoAccCasErr, NoAccPreview, NoAccPtr};
pub use std::sync::atomic::Ordering;

#[derive(Debug)]
pub struct CasErr<D, E, F> {
    pub desired: D,
    pub expected: E,
    pub found: F,
}

pub trait Pointer: Sized {
    type Target;

    unsafe fn from_raw(ptr: *mut Self::Target) -> Self;

    fn as_raw(&self) -> *mut Self::Target;

    fn into_raw(self) -> *mut Self::Target {
        let ptr = self.as_raw();
        forget(self);
        ptr
    }
}

impl<T> Pointer for Box<T> {
    type Target = T;

    unsafe fn from_raw(ptr: *mut Self::Target) -> Self {
        Box::from_raw(ptr)
    }

    fn as_raw(&self) -> *mut Self::Target {
        &**self as *const _ as *mut _
    }
}

impl<T> Pointer for Option<Box<T>> {
    type Target = T;

    unsafe fn from_raw(ptr: *mut Self::Target) -> Self {
        if ptr.is_null() {
            None
        } else {
            Some(Box::from_raw(ptr))
        }
    }

    fn as_raw(&self) -> *mut Self::Target {
        self.as_ref().map_or(null_mut(), |boxed| &**boxed as *const _ as *mut _)
    }
}

impl<T> Pointer for OwnedAlloc<T> {
    type Target = T;

    unsafe fn from_raw(ptr: *mut Self::Target) -> Self {
        Self::from_raw(NonNull::new_unchecked(ptr))
    }

    fn as_raw(&self) -> *mut Self::Target {
        self.raw().as_ptr()
    }
}

impl<T> Pointer for Option<OwnedAlloc<T>> {
    type Target = T;

    unsafe fn from_raw(ptr: *mut Self::Target) -> Self {
        NonNull::new(ptr).map(|nnptr| OwnedAlloc::from_raw(nnptr))
    }

    fn as_raw(&self) -> *mut Self::Target {
        self.as_ref().map(OwnedAlloc::raw).map_or(null_mut(), NonNull::as_ptr)
    }
}
