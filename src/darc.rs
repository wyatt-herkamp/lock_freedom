pub use hazard::Ordering::{self, *};
use hazard::{later_drop, try_delete_local, HazardPtr};
use std::{ptr::NonNull, sync::Arc};

/// Darc: Doubly atomic reference counter. `Darc` is an atomic pointer which
/// stores `Arc`s.
#[derive(Debug)]
pub struct Darc<T> {
    ptr: HazardPtr<T>,
}

impl<T> Darc<T> {
    /// Creates a new `Darc` from the given `Arc`.
    pub fn new(arc: Arc<T>) -> Self {
        Self {
            ptr: HazardPtr::new(Arc::into_raw(arc) as *mut _),
        }
    }

    /// Loads the `Darc` into an `Arc`.
    pub fn load(&self, ord: Ordering) -> Arc<T> {
        self.ptr.load(ord, |ptr| {
            let arc = unsafe { Arc::from_raw(ptr) };
            // We are cloning the stored arc pointer. Therefore,
            // we must increase its reference counter.
            Arc::into_raw(arc.clone());
            arc
        })
    }

    /// Stores an `Arc` inconditionally. This is the same as swaping with
    /// unused return.
    pub fn store(&self, new: Arc<T>, ord: Ordering) {
        // We have to swap it, anyway.
        self.swap(new, ord);
    }

    /// Swaps the inner `Arc` with the argument `new` inconditionally.
    pub fn swap(&self, new: Arc<T>, ord: Ordering) -> Arc<T> {
        let new = Arc::into_raw(new) as *mut _;
        let ptr = self.ptr.swap(new, ord, |ptr| ptr);
        // You may think "We could just return the arc without cloning".
        // Well, we need to clone because the arc may be dropped right
        // after the return of the method, so, we need clone and the
        // later_drop above to ensure no use-after free.
        let arc = unsafe { Arc::from_raw(ptr) };
        Arc::into_raw(arc.clone());
        unsafe { later_drop(NonNull::new_unchecked(ptr), drop_arc) };
        arc
    }

    /// Compares the inner `Arc` with `curr`, and if they are the same pointer,
    /// the inner `Arc` is swapped with `new`. To test the result, use
    /// `Arc::ptr_eq(&curr, &ret)`.
    #[allow(unused_must_use)]
    pub fn compare_and_swap(
        &self,
        curr: Arc<T>,
        new: Arc<T>,
        ord: Ordering,
    ) -> Arc<T> {
        let curr = Arc::into_raw(curr) as *mut _;
        let new = Arc::into_raw(new) as *mut _;
        let res = self.ptr.compare_and_swap(curr, new, ord, |ptr| {
            if ptr == curr {
                // Behaves as a swap.
                // We need to later_drop the loaded pointer for the same reason
                // as in swap.
                unsafe {
                    later_drop(NonNull::new_unchecked(ptr), drop_arc);
                }
                unsafe { Arc::from_raw(ptr) }
            } else {
                // Behaves as a load.
                // No need to later_drop new, since it was not atomically
                // stored.
                unsafe {
                    Arc::from_raw(new);
                }
                // No need to later_drop curr, since it was not atomically
                // stored.
                unsafe {
                    Arc::from_raw(curr);
                }
                let arc = unsafe { Arc::from_raw(ptr) };
                // This clone is needed for the same reason as in swap.
                Arc::into_raw(arc.clone());
                arc
            }
        });
        try_delete_local();
        res
    }

    /// Same as `compare_and_swap` but accepts two `Ordering`s: one for failure
    /// and one for success. Also, it returns a `Result` instead.
     #[allow(unused_must_use)]
    pub fn compare_exchange(
        &self,
        curr: Arc<T>,
        new: Arc<T>,
        succ: Ordering,
        fail: Ordering,
    ) -> Result<Arc<T>, Arc<T>> {
        let curr = Arc::into_raw(curr) as *mut _;
        let new = Arc::into_raw(new) as *mut _;
        let res = self.ptr.compare_exchange(curr, new, succ, fail, |res| {
            match res {
                Ok(ptr) => {
                    // We need to later_drop the loaded pointer for the same
                    // reason as in swap.
                    unsafe {
                        later_drop(NonNull::new_unchecked(ptr), drop_arc);
                    }
                    Ok(unsafe { Arc::from_raw(ptr) })
                },
                Err(ptr) => {
                    // No need to later_drop new, since it was not atomically
                    // stored.
                    unsafe {
                        Arc::from_raw(new);
                    }
                    // No need to later_drop curr, since it was not atomically
                    // stored.
                    unsafe {
                        Arc::from_raw(curr);
                    }
                    let arc = unsafe { Arc::from_raw(ptr) };
                    // This clone is needed for the same reason as in swap.
                    Arc::into_raw(arc.clone());
                    Err(arc)
                },
            }
        });
        try_delete_local();
        res
    }

    /// Same as `compare_exchange` but with weaker semanthics.
     #[allow(unused_must_use)]
    pub fn compare_exchange_weak(
        &self,
        curr: Arc<T>,
        new: Arc<T>,
        succ: Ordering,
        fail: Ordering,
    ) -> Result<Arc<T>, Arc<T>> {
        let curr = Arc::into_raw(curr) as *mut _;
        let new = Arc::into_raw(new) as *mut _;
        let res =
            self.ptr.compare_exchange_weak(curr, new, succ, fail, |res| {
                match res {
                    Ok(ptr) => {
                        // We need to later_drop the loaded pointer for the same
                        // reason as in swap.
                        unsafe {
                            later_drop(NonNull::new_unchecked(ptr), drop_arc);
                        }
                        Ok(unsafe { Arc::from_raw(ptr) })
                    },
                    Err(ptr) => {
                        // No need to later_drop new, since it was not
                        // atomically stored.
                        unsafe {
                            Arc::from_raw(new);
                        }
                        // No need to later_drop curr, since it was not
                        // atomically stored.
                        unsafe {
                            Arc::from_raw(curr);
                        }
                        let arc = unsafe { Arc::from_raw(ptr) };
                        // This clone is needed for the same reason as in swap.
                        Arc::into_raw(arc.clone());
                        Err(arc)
                    },
                }
            });
        try_delete_local();
        res
    }
}

impl<T> Drop for Darc<T> {
    fn drop(&mut self) {
        unsafe {
            later_drop(
                NonNull::new_unchecked(self.ptr.load(Relaxed, |x| x)),
                drop_arc,
            );
        }
    }
}

impl<T> From<T> for Darc<T> {
    fn from(val: T) -> Self {
        Self::new(Arc::new(val))
    }
}

fn drop_arc<T>(ptr: NonNull<T>) {
    unsafe {
        Arc::from_raw(ptr.as_ptr());
    }
}

unsafe impl<T> Send for Darc<T>
where
    T: Send + Sync
{

}

unsafe impl<T> Sync for Darc<T>
where
    T: Send + Sync
{

}
