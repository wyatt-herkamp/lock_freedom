use std::{mem::size_of, ptr::NonNull, slice};

pub unsafe fn alloc<T>(val: T) -> NonNull<T> {
    NonNull::new_unchecked(Box::into_raw(Box::new(val)))
}

pub unsafe fn dealloc<T>(ptr: NonNull<T>) {
    Box::from_raw(ptr.as_ptr());
}

pub unsafe fn dealloc_moved<T>(ptr: NonNull<T>) {
    let slice =
        slice::from_raw_parts_mut(ptr.as_ptr() as *mut u8, size_of::<T>());
    Box::from_raw(slice as *mut _);
}
