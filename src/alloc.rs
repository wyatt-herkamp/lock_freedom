use std::{
    mem::forget,
    ptr::{write, NonNull},
};

pub unsafe fn alloc<T>(val: T) -> NonNull<T> {
    let ptr = alloc_uninit();
    write(ptr.as_ptr(), val);
    ptr
}

pub unsafe fn alloc_uninit<T>() -> NonNull<T> {
    alloc_array(1)
}

pub unsafe fn alloc_array<T>(len: usize) -> NonNull<T> {
    let mut vec = Vec::with_capacity(len);
    let ptr = vec.as_mut_ptr();
    forget(vec);
    NonNull::new_unchecked(ptr)
}

pub unsafe fn dealloc<T>(ptr: NonNull<T>) {
    let _vec = Vec::from_raw_parts(ptr.as_ptr(), 1, 1);
}

pub unsafe fn dealloc_moved<T>(ptr: NonNull<T>) {
    dealloc_array(ptr, 1);
}

pub unsafe fn dealloc_array<T>(ptr: NonNull<T>, len: usize) {
    let _vec = Vec::from_raw_parts(ptr.as_ptr(), 0, len);
}
