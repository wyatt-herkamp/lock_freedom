use std::{
    alloc::{Layout, LayoutError as StdLayoutErr},
    fmt,
};

/// Error returned from the allocator.
#[derive(Debug, Clone)]
pub struct AllocErr {
    /// The requested layout.
    pub layout: Layout,
}

impl fmt::Display for AllocErr {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "the allocator failed for the layout of size {}, align {}",
            self.layout.size(),
            self.layout.align()
        )
    }
}

/// Error caused by invalid size or alignment.
#[derive(Debug, Clone)]
pub struct LayoutErr;

impl fmt::Display for LayoutErr {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str("invalid layout parameters")
    }
}

impl From<StdLayoutErr> for LayoutErr {
    fn from(_err: StdLayoutErr) -> Self {
        LayoutErr
    }
}

/// Errors returned by the `RawVec`.
#[derive(Debug, Clone)]
pub enum RawVecErr {
    /// Allocation error.
    Alloc(AllocErr),
    /// Layout error.
    Layout(LayoutErr),
}

impl fmt::Display for RawVecErr {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RawVecErr::Alloc(err) => write!(fmtr, "{}", err),
            RawVecErr::Layout(err) => write!(fmtr, "{}", err),
        }
    }
}

impl From<AllocErr> for RawVecErr {
    fn from(err: AllocErr) -> Self {
        RawVecErr::Alloc(err)
    }
}

impl From<LayoutErr> for RawVecErr {
    fn from(err: LayoutErr) -> Self {
        RawVecErr::Layout(err)
    }
}
