use std::ptr::NonNull;

static _NON_NULL: u8 = /* dummy value */ 1;

// don't use with bit flags
#[inline(always)]
pub fn non_zero_null<T>() -> NonNull<T> {
    NonNull::from(&_NON_NULL).cast()
}
