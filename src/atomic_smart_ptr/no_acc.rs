use super::{
    CasErr,
    Ordering::{self, *},
    Pointer,
};
use std::{fmt, sync::atomic::AtomicPtr};

pub type NoAccCasErr<'origin, 'expected, P> =
    CasErr<P, &'expected P, NoAccPreview<'origin, P>>;
pub type NoAccPreviewCasErr<'origin, P> =
    CasErr<P, NoAccPreview<'origin, P>, NoAccPreview<'origin, P>>;

pub struct NoAccPtr<P>
where
    P: Pointer,
{
    atomic: AtomicPtr<P::Target>,
}

impl<P> NoAccPtr<P>
where
    P: Pointer,
{
    pub fn new(ptr: P) -> Self {
        Self { atomic: AtomicPtr::new(ptr.into_raw()) }
    }

    pub fn into_inner(self) -> P {
        unsafe { P::from_raw(self.atomic.load(Relaxed)) }
    }

    pub fn load(&self, ordering: Ordering) -> NoAccPreview<P> {
        NoAccPreview { preview: self.atomic.load(ordering), origin: self }
    }

    pub fn store(&self, new: P, ordering: Ordering) {
        match ordering {
            Acquire => panic!("There is no such thing as Acquire store"),
            AcqRel => panic!("There is no such thing as AcqRel store"),
            _ => (),
        }

        self.swap(new, ordering);
    }

    pub fn swap(&self, new: P, ordering: Ordering) -> P {
        unsafe { P::from_raw(self.atomic.swap(new.into_raw(), ordering)) }
    }

    pub fn compare_and_swap<'origin, 'expected>(
        &'origin self,
        expected: &'expected P,
        new: P,
        ordering: Ordering,
    ) -> Result<P, NoAccCasErr<'origin, 'expected, P>> {
        let expected_raw = expected.as_raw();
        let new_raw = new.into_raw();
        let found =
            self.atomic.compare_and_swap(expected_raw, new_raw, ordering);

        if found == expected_raw {
            Ok(unsafe { P::from_raw(found) })
        } else {
            Err(CasErr {
                desired: unsafe { P::from_raw(new_raw) },
                expected,
                found: NoAccPreview { preview: found, origin: self },
            })
        }
    }

    pub fn compare_exchange<'origin, 'expected>(
        &'origin self,
        expected: &'expected P,
        new: P,
        success: Ordering,
        failure: Ordering,
    ) -> Result<P, NoAccCasErr<'origin, 'expected, P>> {
        let expected_raw = expected.as_raw();
        let new_raw = new.into_raw();

        match self.atomic.compare_exchange(
            expected_raw,
            new_raw,
            success,
            failure,
        ) {
            Ok(found) => Ok(unsafe { P::from_raw(found) }),
            Err(found) => Err(CasErr {
                desired: unsafe { P::from_raw(new_raw) },
                expected,
                found: NoAccPreview { preview: found, origin: self },
            }),
        }
    }

    pub fn compare_exchange_weak<'origin, 'expected>(
        &'origin self,
        expected: &'expected P,
        new: P,
        success: Ordering,
        failure: Ordering,
    ) -> Result<P, NoAccCasErr<'origin, 'expected, P>> {
        let expected_raw = expected.as_raw();
        let new_raw = new.into_raw();

        match self.atomic.compare_exchange_weak(
            expected_raw,
            new_raw,
            success,
            failure,
        ) {
            Ok(found) => Ok(unsafe { P::from_raw(found) }),
            Err(found) => Err(CasErr {
                desired: unsafe { P::from_raw(new_raw) },
                expected,
                found: NoAccPreview { preview: found, origin: self },
            }),
        }
    }
}

impl<P> fmt::Debug for NoAccPtr<P>
where
    P: Pointer,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "NoAccPtr {} atomic: {:?} {}", '{', self.atomic, '}')
    }
}

impl<P> Drop for NoAccPtr<P>
where
    P: Pointer,
{
    fn drop(&mut self) {
        unsafe {
            P::from_raw(self.atomic.load(Relaxed));
        }
    }
}

pub struct NoAccPreview<'origin, P>
where
    P: Pointer,
{
    preview: *mut P::Target,
    origin: &'origin NoAccPtr<P>,
}

impl<'origin, P> NoAccPreview<'origin, P>
where
    P: Pointer,
{
    pub fn preview(&self) -> *mut P::Target {
        self.preview
    }

    pub fn origin(&self) -> &NoAccPtr<P> {
        self.origin
    }

    pub fn compare_and_swap(
        self,
        new: P,
        ordering: Ordering,
    ) -> Result<P, NoAccPreviewCasErr<'origin, P>> {
        let new_raw = new.into_raw();
        let found = self.origin.atomic.compare_and_swap(
            self.preview,
            new_raw,
            ordering,
        );

        if found == self.preview {
            Ok(unsafe { P::from_raw(self.preview) })
        } else {
            Err(CasErr {
                desired: unsafe { P::from_raw(new_raw) },
                found: Self { preview: found, origin: self.origin },
                expected: self,
            })
        }
    }

    pub fn compare_exchange(
        self,
        new: P,
        success: Ordering,
        failure: Ordering,
    ) -> Result<P, NoAccPreviewCasErr<'origin, P>> {
        let new_raw = new.into_raw();

        match self.origin.atomic.compare_exchange(
            self.preview,
            new_raw,
            success,
            failure,
        ) {
            Ok(_) => Ok(unsafe { P::from_raw(self.preview) }),
            Err(found) => Err(CasErr {
                desired: unsafe { P::from_raw(new_raw) },
                found: Self { preview: found, origin: self.origin },
                expected: self,
            }),
        }
    }

    pub fn compare_exchange_weak(
        self,
        new: P,
        success: Ordering,
        failure: Ordering,
    ) -> Result<P, NoAccPreviewCasErr<'origin, P>> {
        let new_raw = new.into_raw();

        match self.origin.atomic.compare_exchange_weak(
            self.preview,
            new_raw,
            success,
            failure,
        ) {
            Ok(_) => Ok(unsafe { P::from_raw(self.preview) }),
            Err(found) => Err(CasErr {
                desired: unsafe { P::from_raw(new_raw) },
                found: Self { preview: found, origin: self.origin },
                expected: self,
            }),
        }
    }
}

impl<'origin, P> fmt::Debug for NoAccPreview<'origin, P>
where
    P: Pointer,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "NoAccPreview {} preview: {:?}, origin: {:?} {}",
            '{', self.preview, self.origin, '}'
        )
    }
}

impl<'origin, P> Clone for NoAccPreview<'origin, P>
where
    P: Pointer,
{
    fn clone(&self) -> Self {
        Self { preview: self.preview, origin: self.origin }
    }
}
