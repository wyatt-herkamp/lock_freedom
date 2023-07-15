use super::{OwnedAlloc, UninitAlloc};
use std::fmt;

/// Pointer to memory allocaation that might be either initialized or
/// uninitialized. For the drop checker, the type acts as if it contains a `T`
/// due to usage of `PhantomData<T>`.
pub enum MaybeUninitAlloc<T>
where
    T: ?Sized,
{
    /// Initialized allocation.
    Init(OwnedAlloc<T>),

    /// Uninitialized allocation.
    Uninit(UninitAlloc<T>),
}

impl<T> MaybeUninitAlloc<T> {
    /// If the allocation was initialized, this is a no-op. If it wasn't, the
    /// passed function is called and its return value is used to initialize the
    /// memory. In both cases, an allocation considered initialized is returned.
    pub fn or_init<F>(self, init: F) -> OwnedAlloc<T>
    where
        F: FnOnce() -> T,
    {
        match self {
            MaybeUninitAlloc::Init(ptr) => ptr,
            MaybeUninitAlloc::Uninit(ptr) => ptr.init(init()),
        }
    }
}

impl<T> MaybeUninitAlloc<T>
where
    T: ?Sized,
{
    /// If the allocation was initialized, this is a no-op. If it wasn't, the
    /// passed function is called with a mutable reference to the uninitialized
    /// memory and the function is expected to initialize the memory. In both
    /// cases, an allocation considered initialized is returned.
    ///
    /// # Safety
    /// This function is `unsafe` because the passed function might not
    /// initialize the memory correctly.
    pub unsafe fn or_init_in_place<F>(self, init: F) -> OwnedAlloc<T>
    where
        F: FnOnce(&mut T),
    {
        match self {
            MaybeUninitAlloc::Init(ptr) => ptr,
            MaybeUninitAlloc::Uninit(ptr) => ptr.init_in_place(init),
        }
    }

    /// Tests if the allocation is initialized.
    pub fn is_initialized(&self) -> bool {
        match self {
            MaybeUninitAlloc::Init(_) => true,
            MaybeUninitAlloc::Uninit(_) => false,
        }
    }

    /// Tests if the allocation is uninitialized.
    pub fn is_uninitialized(&self) -> bool {
        match self {
            MaybeUninitAlloc::Init(_) => true,
            MaybeUninitAlloc::Uninit(_) => false,
        }
    }

    /// If the memory is initialized, this function drops its content. In any
    /// case, the allocation now with uninitialized content is returned.
    pub fn drop_in_place(self) -> UninitAlloc<T> {
        match self {
            MaybeUninitAlloc::Init(ptr) => ptr.drop_in_place(),
            MaybeUninitAlloc::Uninit(ptr) => ptr,
        }
    }

    /// Encodes this type as a `Result` with an `OwnedAlloc` as `Ok`.
    pub fn init_as_ok(self) -> Result<OwnedAlloc<T>, UninitAlloc<T>> {
        match self {
            MaybeUninitAlloc::Init(ptr) => Ok(ptr),
            MaybeUninitAlloc::Uninit(ptr) => Err(ptr),
        }
    }

    /// Encodes this type as a `Result` with an `UninitAlloc` as `Ok`.
    pub fn uninit_as_ok(self) -> Result<UninitAlloc<T>, OwnedAlloc<T>> {
        match self {
            MaybeUninitAlloc::Init(ptr) => Err(ptr),
            MaybeUninitAlloc::Uninit(ptr) => Ok(ptr),
        }
    }

    /// If the memory is uninitialized, `None` is returned. If it is
    /// initialized, the passed function is called with a mutable reference to
    /// the allocation, and its return value is wrapped into a `Some`.
    pub fn modify<F, A>(&mut self, visit: F) -> Option<A>
    where
        F: FnOnce(&mut T) -> A,
    {
        match self {
            MaybeUninitAlloc::Init(ptr) => Some(visit(&mut **ptr)),
            MaybeUninitAlloc::Uninit(_) => None,
        }
    }
}

impl<T> From<T> for MaybeUninitAlloc<T> {
    fn from(val: T) -> Self {
        MaybeUninitAlloc::Init(OwnedAlloc::new(val))
    }
}

impl<T> From<OwnedAlloc<T>> for MaybeUninitAlloc<T>
where
    T: ?Sized,
{
    fn from(alloc: OwnedAlloc<T>) -> Self {
        MaybeUninitAlloc::Init(alloc)
    }
}

impl<T> From<UninitAlloc<T>> for MaybeUninitAlloc<T>
where
    T: ?Sized,
{
    fn from(alloc: UninitAlloc<T>) -> Self {
        MaybeUninitAlloc::Uninit(alloc)
    }
}

impl<T> fmt::Debug for MaybeUninitAlloc<T>
where
    T: ?Sized,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MaybeUninitAlloc::Init(ptr) => write!(fmtr, "Init({:?})", ptr),
            MaybeUninitAlloc::Uninit(ptr) => write!(fmtr, "Uninit({:?})", ptr),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{super::UninitAlloc, MaybeUninitAlloc};

    #[test]
    fn or_init_is_noop_if_initialized() {
        let init = MaybeUninitAlloc::from(90);

        assert_eq!(*init.or_init(|| 50), 90);
    }

    #[test]
    fn or_init_calls_if_uninit() {
        let init = MaybeUninitAlloc::from(UninitAlloc::new());

        assert_eq!(*init.or_init(|| 50), 50);
    }

    #[test]
    fn modifies() {
        let mut init = MaybeUninitAlloc::from(20);

        assert!(init.modify(|addr| *addr = 2).is_some());
        assert_eq!(*init.init_as_ok().unwrap(), 2);
    }
}
