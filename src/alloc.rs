use std::{mem::forget, ptr::NonNull};

pub unsafe fn alloc<T>(val: T) -> NonNull<T> {
    let ptr = alloc_uninit::<T>();
    ptr.as_ptr().write(val);
    ptr
}

pub unsafe fn alloc_with_init<T, F>(init: F) -> NonNull<T>
where
    F: FnOnce(NonNull<T>),
{
    let ptr = alloc_uninit::<T>();
    init(ptr);
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

pub struct CachedAlloc<T> {
    ptr: Option<NonNull<T>>,
}

impl<T> CachedAlloc<T> {
    pub fn empty() -> Self {
        Self { ptr: None }
    }

    pub unsafe fn with_ptr(ptr: NonNull<T>) -> Self {
        Self { ptr: Some(ptr) }
    }

    pub unsafe fn get_or<F>(&mut self, init: F) -> NonNull<T>
    where
        F: FnOnce(NonNull<T>),
    {
        match self.ptr {
            Some(nnptr) => nnptr,
            None => {
                let nnptr = alloc_with_init(init);
                self.ptr = Some(nnptr);
                nnptr
            },
        }
    }

    pub unsafe fn take(&mut self) -> Option<NonNull<T>> {
        self.ptr.take()
    }
}

impl<T> Drop for CachedAlloc<T> {
    fn drop(&mut self) {
        if let Some(nnptr) = self.ptr {
            unsafe { dealloc(nnptr) }
        }
    }
}
