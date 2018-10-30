use super::{
    owned_alloc_to_raw,
    raw_to_owned_alloc,
    CasError,
    Ordering::{self, *},
};
use owned_alloc::OwnedAlloc;
use std::{fmt, ptr::NonNull, sync::atomic::AtomicPtr};

pub type OwnedCasError<'target, T> =
    CasError<OwnedAlloc<T>, OwnedPreview<'target, T>>;

pub type OptionOwnedCasError<'target, T> =
    CasError<Option<OwnedAlloc<T>>, OptionOwnedPreview<'target, T>>;

pub struct OwnedAccessPtr<T> {
    atomic: AtomicPtr<T>,
}

impl<T> OwnedAccessPtr<T> {
    pub fn new(alloc: OwnedAlloc<T>) -> Self {
        Self { atomic: AtomicPtr::new(alloc.into_raw().as_ptr()) }
    }

    pub fn load(&self, ordering: Ordering) -> OwnedPreview<T> {
        OwnedPreview {
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
    fn from(alloc: OwnedAlloc<T>) -> Self {
        Self::new(alloc)
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

pub struct OwnedPreview<'target, T> {
    preview: NonNull<T>,
    target: &'target OwnedAccessPtr<T>,
}

impl<'target, T> OwnedPreview<'target, T> {
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

impl<'target, T> fmt::Debug for OwnedPreview<'target, T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "OwnedPreview {} preview: {:?}, target {:?} {}",
            '{', self.preview, self.target, '}'
        )
    }
}

pub struct OptionOwnedAccessPtr<T> {
    atomic: AtomicPtr<T>,
}

impl<T> OptionOwnedAccessPtr<T> {
    pub fn new(alloc: Option<OwnedAlloc<T>>) -> Self {
        Self { atomic: AtomicPtr::new(owned_alloc_to_raw(alloc)) }
    }

    pub fn load(&self, ordering: Ordering) -> OptionOwnedPreview<T> {
        OptionOwnedPreview { preview: self.atomic.load(ordering), target: self }
    }

    pub fn swap(
        &self,
        alloc: Option<OwnedAlloc<T>>,
        ordering: Ordering,
    ) -> Option<OwnedAlloc<T>> {
        let ptr = self.atomic.swap(owned_alloc_to_raw(alloc), ordering);
        unsafe { raw_to_owned_alloc(ptr) }
    }

    pub fn store(&self, alloc: Option<OwnedAlloc<T>>, ordering: Ordering) {
        match ordering {
            Acquire => panic!("No such thing as Acquire for a store"),
            AcqRel => panic!("No such thing as AcqRel for a store"),
            _ => (),
        }

        self.swap(alloc, ordering);
    }
}

impl<T> From<T> for OptionOwnedAccessPtr<T> {
    fn from(val: T) -> Self {
        Self::from(OwnedAlloc::new(val))
    }
}

impl<T> From<Option<T>> for OptionOwnedAccessPtr<T> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(val) => Self::from(val),
            None => Self::new(None),
        }
    }
}

impl<T> From<OwnedAlloc<T>> for OptionOwnedAccessPtr<T> {
    fn from(alloc: OwnedAlloc<T>) -> Self {
        Self::from(Some(alloc))
    }
}

impl<T> From<Option<OwnedAlloc<T>>> for OptionOwnedAccessPtr<T> {
    fn from(alloc: Option<OwnedAlloc<T>>) -> Self {
        Self::new(alloc)
    }
}

impl<T> Drop for OptionOwnedAccessPtr<T> {
    fn drop(&mut self) {
        unsafe {
            raw_to_owned_alloc(self.atomic.load(Relaxed));
        }
    }
}

impl<T> fmt::Debug for OptionOwnedAccessPtr<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "OptionOwnedAccessPtr {} atomic: {:?} {}",
            '{', self.atomic, '}'
        )
    }
}

pub struct OptionOwnedPreview<'target, T> {
    preview: *mut T,
    target: &'target OptionOwnedAccessPtr<T>,
}

impl<'target, T> OptionOwnedPreview<'target, T> {
    pub fn pointer(&self) -> *mut T {
        self.preview
    }

    pub fn target(&self) -> &OptionOwnedAccessPtr<T> {
        self.target
    }

    pub fn compare_and_swap(
        self,
        new_value: Option<OwnedAlloc<T>>,
        ordering: Ordering,
    ) -> Result<Option<OwnedAlloc<T>>, OptionOwnedCasError<'target, T>> {
        let new_raw = owned_alloc_to_raw(new_value);
        let res = self.target.atomic.compare_and_swap(
            self.preview,
            new_raw,
            ordering,
        );

        if res == self.preview {
            Ok(unsafe { raw_to_owned_alloc(self.preview) })
        } else {
            Err(CasError {
                found: Self { preview: res, target: self.target },
                expected: self,
                desired: unsafe { raw_to_owned_alloc(new_raw) },
            })
        }
    }
}

impl<'target, T> fmt::Debug for OptionOwnedPreview<'target, T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "OwnedPreview {} preview: {:?}, target {:?} {}",
            '{', self.preview, self.target, '}'
        )
    }
}
