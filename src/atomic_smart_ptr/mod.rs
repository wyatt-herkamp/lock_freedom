mod owned;

pub use std::sync::atomic::Ordering;

pub use self::owned::{OwnedAccessPreview, OwnedAccessPtr, OwnedCasError};

#[derive(Debug)]
pub struct CasError<T, P> {
    pub desired: T,
    pub expected: P,
    pub found: P,
}
