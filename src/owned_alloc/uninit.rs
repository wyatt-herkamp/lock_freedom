use super::{AllocErr, OwnedAlloc, RawVec};
use std::{
    alloc::{alloc, dealloc, handle_alloc_error, Layout},
    fmt,
    marker::PhantomData,
    mem,
    ptr::NonNull,
};

/// Dynamic allocation of a `T` whose memory is considered uninitialized. The
/// allocation is freed on `drop`. If the size of the allocation is zero, no
/// allocation is performed and a dangling pointer is used (just like in `std`).
/// For the drop checker, the type acts as if it contains a `T` due to usage of
/// `PhantomData<T>`.
pub struct UninitAlloc<T>
    where
        T: ?Sized,
{
    nnptr: NonNull<T>,
    _marker: PhantomData<T>,
}

impl<T> Default for UninitAlloc<T>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> UninitAlloc<T> {
    /// Creates room for a `T`. In case of allocation error, the handler
    /// registered via stdlib is called.
    pub fn new() -> Self {
        Self::try_new().unwrap_or_else(|err| handle_alloc_error(err.layout))
    }

    /// Creates room for a `T`. In case of allocation error, `Err` is returned.
    pub fn try_new() -> Result<Self, AllocErr> {
        let layout = Layout::new::<T>();

        let res = if layout.size() == 0 {
            Ok(NonNull::dangling())
        } else {
            NonNull::new(unsafe { alloc(layout) })
                .map(NonNull::cast::<T>)
                .ok_or(AllocErr { layout })
        };

        res.map(|nnptr| Self { nnptr, _marker: PhantomData })
    }

    /// Initializes the memory and returns the allocation now considered
    /// initialized.
    pub fn init(self, val: T) -> OwnedAlloc<T> {
        let raw = self.into_raw();
        unsafe {
            raw.as_ptr().write(val);
            OwnedAlloc::from_raw(raw)
        }
    }
}

impl<T> UninitAlloc<T>
    where
        T: ?Sized,
{
    /// Calls a function with a mutable reference to uninitialized memory and
    /// returns the allocation now considered initialized. The passed function
    /// is expected to initialize the memory.
    ///
    /// # Safety
    /// This function is `unsafe` because the passed function might not
    /// initialize the memory correctly.
    pub unsafe fn init_in_place<F>(self, init: F) -> OwnedAlloc<T>
        where
            F: FnOnce(&mut T),
    {
        let mut raw = self.into_raw();
        init(raw.as_mut());
        OwnedAlloc::from_raw(raw)
    }

    /// Recreate the `UninitAlloc` from a raw non-null pointer.
    ///
    /// # Safety
    /// This functions is `unsafe` because passing the wrong pointer leads to
    /// undefined behaviour.
    pub unsafe fn from_raw(nnptr: NonNull<T>) -> Self {
        Self { nnptr, _marker: PhantomData }
    }

    /// Returns the raw non-null pointer of the allocation.
    pub fn raw(&self) -> NonNull<T> {
        self.nnptr
    }

    /// "Forgets" dropping the allocation and returns its raw non-null pointer.
    pub fn into_raw(self) -> NonNull<T> {
        let nnptr = self.nnptr;
        mem::forget(self);
        nnptr
    }
}

impl<T> Drop for UninitAlloc<T>
    where
        T: ?Sized,
{
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::for_value(self.nnptr.as_ref());

            if layout.size() != 0 {
                dealloc(self.nnptr.cast().as_ptr(), layout);
            }
        }
    }
}

impl<T> fmt::Debug for UninitAlloc<T>
    where
        T: ?Sized,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{:?}", self.nnptr)
    }
}

impl<T> From<RawVec<T>> for UninitAlloc<[T]> {
    fn from(alloc: RawVec<T>) -> Self {
        Self { nnptr: alloc.into_raw_slice(), _marker: PhantomData }
    }
}

unsafe impl<T> Send for UninitAlloc<T> where T: ?Sized + Send {}

unsafe impl<T> Sync for UninitAlloc<T> where T: ?Sized + Sync {}

#[cfg(test)]
mod test {
    use super::UninitAlloc;

    #[test]
    fn into_from_raw() {
        let alloc = UninitAlloc::<usize>::new();
        let raw_borrowed = alloc.raw();
        let raw = alloc.into_raw();

        assert_eq!(raw, raw_borrowed);

        let alloc = unsafe { UninitAlloc::from_raw(raw) };
        assert_eq!(alloc.raw(), raw_borrowed);
    }
}
