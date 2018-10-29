use super::{
    super::incin::Incinerator,
    Ordering::{self, *},
};
use owned_alloc::OwnedAlloc;
use std::{ptr::NonNull, sync::atomic::AtomicPtr};

pub struct OwnedAccessPtr<T> {
    atomic: AtomicPtr<T>,
}

impl<T> OwnedAccessPtr<T> {
    pub fn new(alloc: OwnedAlloc<T>) -> Self {
        Self::from(alloc)
    }

    pub fn load(&self, ordering: Ordering) -> OwnedAccessPreview<T> {
        OwnedAccessPreview {
            preview: unsafe {
                NonNull::new_unchecked(self.atomic.load(Acquire))
            },
            target: self,
        }
    }

    pub fn swap(
        &self,
        alloc: OwnedAlloc<T>,
        ordering: Ordering,
    ) -> OwnedAlloc<T> {
        let ptr = self.atomic.swap(alloc.into_raw().as_ptr(), ordering);
        unsafe { OwnedAlloc::from_raw(NonNull::new_unchecked(ptr)) }
    }

    pub fn store(&self, alloc: OwnedAlloc<T>, ordering: Ordering) {
        match ordering {
            Acquire => panic!("No such thing as Acquire for a store"),
            AcqRel => panic!("No such thing as AcqRel for a store"),
            _ => (),
        }

        self.swap(alloc, ordering);
    }
}

impl<T> From<T> for OwnedAccessPtr<T> {
    fn from(val: T) -> Self {
        Self::from(OwnedAlloc::new(val))
    }
}

impl<T> From<OwnedAlloc<T>> for OwnedAccessPtr<T> {
    fn from(owned: OwnedAlloc<T>) -> Self {
        Self { atomic: AtomicPtr::new(owned.into_raw().as_ptr()) }
    }
}

pub struct OwnedAccessPreview<'target, T> {
    preview: NonNull<T>,
    target: &'target OwnedAccessPtr<T>,
}
