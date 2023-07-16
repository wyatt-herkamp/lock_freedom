use super::{AllocErr, UninitAlloc};
use std::{
    alloc::{dealloc, Layout},
    fmt,
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/// Dynamic allocation of a `T` whose memory is considered fully initialized.
/// The allocation and its content are freed on `drop`. Similar to a `Box`. If
/// the size of the allocation is zero, no allocation is performed and a
/// dangling pointer is used (just like in `std`). For the drop checker, the
/// type acts as if it contains a `T` due to usage of `PhantomData<T>`.
pub struct OwnedAlloc<T>
where
    T: ?Sized,
{
    nnptr: NonNull<T>,
    _marker: PhantomData<T>,
}

impl<T> OwnedAlloc<T> {
    /// Creates an allocation and initializes it to the passed argument. In case
    /// of allocation error, the handler registered via stdlib is called.
    pub fn new(val: T) -> Self {
        UninitAlloc::new().init(val)
    }

    /// Creates an allocation and initializes it to the passed argument. In case
    /// of allocation error, `Err` is returned.
    pub fn try_new(val: T) -> Result<Self, AllocErr> {
        UninitAlloc::try_new().map(|alloc| alloc.init(val))
    }

    /// Moves the stored value out from the allocation. The value and the
    /// allocation now considered uninitialized are returned.
    pub fn move_inner(self) -> (T, UninitAlloc<T>) {
        let val = unsafe { self.nnptr.as_ptr().read() };
        let alloc = unsafe { UninitAlloc::from_raw(self.nnptr) };
        mem::forget(self);
        (val, alloc)
    }
}

impl<T> OwnedAlloc<T>
where
    T: ?Sized,
{
    /// Recreate the `OwnedAlloc` from a raw non-null pointer.
    ///
    /// # Safety
    /// This functions is `unsafe` because passing the wrong pointer leads to
    /// undefined behaviour. Passing a pointer to uninitialized memory is also
    /// undefined behaviour.
    pub unsafe fn from_raw(nnptr: NonNull<T>) -> Self {
        Self { nnptr, _marker: PhantomData }
    }

    /// Converts the plain old standard library `Box` into an owned allocation.
    ///
    /// # Safety
    /// This function is `unsafe` because there are no guarantees that `Box` and
    /// `OwnedAlloc` allocate in the same way. They probably do in the Rust
    /// version you are using, but there are no future guarantees.
    pub unsafe fn from_box(boxed: Box<T>) -> Self {
        Self::from_raw(NonNull::new_unchecked(Box::into_raw(boxed)))
    }

    /// Returns the raw non-null pointer of the allocation.
    pub fn raw(&self) -> NonNull<T> {
        self.nnptr
    }

    /// "Forgets" dropping both the allocation and its content and returns its
    /// raw non-null pointer.
    pub fn into_raw(self) -> NonNull<T> {
        let nnptr = self.nnptr;
        mem::forget(self);
        nnptr
    }

    /// Converts the owned allocation into a plain old standard library `Box`.
    ///
    /// # Safety
    /// This function is `unsafe` because there are no guarantees that `Box` and
    /// `OwnedAlloc` allocate in the same way. They probably do in the Rust
    /// version you are using, but there are no future guarantees.
    pub unsafe fn into_box(self) -> Box<T> {
        Box::from_raw(self.into_raw().as_ptr())
    }

    /// Drops the memory and returns the allocation now considered
    /// uninitialized.
    pub fn drop_in_place(self) -> UninitAlloc<T> {
        unsafe {
            self.nnptr.as_ptr().drop_in_place();
            UninitAlloc::from_raw(self.into_raw())
        }
    }

    /// "Forgets" about dropping the inner value and returns an uninitialized
    /// allocation.
    pub fn forget_inner(self) -> UninitAlloc<T> {
        unsafe { UninitAlloc::from_raw(self.into_raw()) }
    }
}

impl<T> Drop for OwnedAlloc<T>
where
    T: ?Sized,
{
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::for_value(self.nnptr.as_ref());
            self.nnptr.as_ptr().drop_in_place();
            if layout.size() != 0 {
                dealloc(self.nnptr.cast().as_ptr(), layout);
            }
        }
    }
}

impl<T> Deref for OwnedAlloc<T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.nnptr.as_ref() }
    }
}

impl<T> DerefMut for OwnedAlloc<T>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.nnptr.as_mut() }
    }
}

impl<T> fmt::Debug for OwnedAlloc<T>
where
    T: ?Sized,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{:?}", self.nnptr)
    }
}

impl<T> Clone for OwnedAlloc<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self::new((**self).clone())
    }
}

impl<T> From<T> for OwnedAlloc<T> {
    fn from(val: T) -> Self {
        Self::new(val)
    }
}

unsafe impl<T> Send for OwnedAlloc<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for OwnedAlloc<T> where T: ?Sized + Sync {}

#[cfg(test)]
mod test {
    use super::OwnedAlloc;

    #[test]
    fn inner_eq() {
        let mut alloc = OwnedAlloc::new(20);

        assert_eq!(*alloc, 20);

        *alloc = 30;

        assert_eq!(*alloc, 30);
    }

    #[test]
    fn move_inner_eq() {
        let alloc = OwnedAlloc::new(20);

        assert_eq!(alloc.move_inner().0, 20);
    }

    #[test]
    fn from_into_std_box() {
        let boxed = unsafe { OwnedAlloc::new([5u128; 32]).into_box() };
        assert_eq!(*boxed, [5; 32]);
        let raw = unsafe { OwnedAlloc::from_box(boxed) };
        assert_eq!(*raw, [5; 32]);
    }
}
