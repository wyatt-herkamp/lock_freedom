use std::ptr::{null_mut, NonNull};

static _NON_NULL: u8 = /* dummy value */ 1;

// don't use with bit flags
#[inline(always)]
pub fn non_zero_null<T>() -> NonNull<T> {
    NonNull::from(&_NON_NULL).cast()
}

#[inline(always)]
pub fn from_nnptr<T>(nnptr: Option<NonNull<T>>) -> *mut T {
    nnptr.map_or(null_mut(), NonNull::as_ptr)
}
