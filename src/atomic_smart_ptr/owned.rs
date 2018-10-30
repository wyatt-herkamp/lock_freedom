use super::{
    CasError,
    Ordering::{self, *},
};
use owned_alloc::OwnedAlloc;
use std::{fmt, ptr::NonNull, sync::atomic::AtomicPtr};

pub type OwnedCasError<'target, T> =
    CasError<OwnedAlloc<T>, OwnedAccessPreview<'target, T>>;

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
                NonNull::new_unchecked(self.atomic.load(ordering))
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

impl<T> Drop for OwnedAccessPtr<T> {
    fn drop(&mut self) {
        unsafe {
            OwnedAlloc::from_raw(NonNull::new_unchecked(
                self.atomic.load(Relaxed),
            ));
        }
    }
}

impl<T> fmt::Debug for OwnedAccessPtr<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "OwnedAccessPtr {} atomic: {:?} {}", '{', self.atomic, '}')
    }
}

pub struct OwnedAccessPreview<'target, T> {
    preview: NonNull<T>,
    target: &'target OwnedAccessPtr<T>,
}

impl<'target, T> OwnedAccessPreview<'target, T> {
    pub fn pointer(&self) -> NonNull<T> {
        self.preview
    }

    pub fn target(&self) -> &OwnedAccessPtr<T> {
        self.target
    }

    pub fn compare_and_swap(
        self,
        new_value: OwnedAlloc<T>,
        ordering: Ordering,
    ) -> Result<OwnedAlloc<T>, OwnedCasError<'target, T>> {
        let new_raw = new_value.into_raw();
        let res = self.target.atomic.compare_and_swap(
            self.preview.as_ptr(),
            new_raw.as_ptr(),
            ordering,
        );

        if res == self.preview.as_ptr() {
            Ok(unsafe { OwnedAlloc::from_raw(self.preview) })
        } else {
            Err(CasError {
                found: Self {
                    preview: unsafe { NonNull::new_unchecked(res) },
                    target: self.target,
                },
                expected: self,
                desired: unsafe { OwnedAlloc::from_raw(new_raw) },
            })
        }
    }

    pub fn compare_exchange(
        self,
        new_value: OwnedAlloc<T>,
        success: Ordering,
        failure: Ordering,
    ) -> Result<OwnedAlloc<T>, OwnedCasError<'target, T>> {
        let new_raw = new_value.into_raw();

        match self.target.atomic.compare_exchange(
            self.preview.as_ptr(),
            new_raw.as_ptr(),
            success,
            failure,
        ) {
            Ok(_) => Ok(unsafe { OwnedAlloc::from_raw(self.preview) }),
            Err(res) => Err(CasError {
                found: Self {
                    preview: unsafe { NonNull::new_unchecked(res) },
                    target: self.target,
                },
                expected: self,
                desired: unsafe { OwnedAlloc::from_raw(new_raw) },
            }),
        }
    }

    pub fn compare_exchange_weak(
        self,
        new_value: OwnedAlloc<T>,
        success: Ordering,
        failure: Ordering,
    ) -> Result<OwnedAlloc<T>, OwnedCasError<'target, T>> {
        let new_raw = new_value.into_raw();

        match self.target.atomic.compare_exchange_weak(
            self.preview.as_ptr(),
            new_raw.as_ptr(),
            success,
            failure,
        ) {
            Ok(_) => Ok(unsafe { OwnedAlloc::from_raw(self.preview) }),
            Err(res) => Err(CasError {
                found: Self {
                    preview: unsafe { NonNull::new_unchecked(res) },
                    target: self.target,
                },
                expected: self,
                desired: unsafe { OwnedAlloc::from_raw(new_raw) },
            }),
        }
    }
}

impl<'target, T> fmt::Debug for OwnedAccessPreview<'target, T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "OwnedAccessPreview {} preview: {:?}, target {:?} {}",
            '{', self.preview, self.target, '}'
        )
    }
}
