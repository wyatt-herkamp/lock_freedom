use incinerator;
use std::{
    ptr::NonNull,
    sync::{
        atomic::{AtomicPtr, Ordering::*},
        Arc,
    },
};

/// Darc: Doubly atomic reference counter. `Darc` is an atomic pointer which
/// stores `Arc`s.
#[derive(Debug)]
pub struct Darc<T> {
    ptr: AtomicPtr<T>,
}

impl<T> Darc<T> {
    /// Creates a new `Darc` from the given `Arc`.
    pub fn new(arc: Arc<T>) -> Self {
        Self { ptr: AtomicPtr::new(Arc::into_raw(arc) as *mut _) }
    }

    /// Loads the `Darc` into an `Arc`.
    pub fn load(&self) -> Arc<T> {
        incinerator::pause(|| {
            let ptr = self.ptr.load(Relaxed);
            let arc = unsafe { Arc::from_raw(ptr) };
            // We are cloning the stored arc pointer. Therefore,
            // we must increase its reference counter.
            Arc::into_raw(arc.clone());
            arc
        })
    }

    /// Stores an `Arc` inconditionally. This is the same as swaping with
    /// unused return.
    pub fn store(&self, new: Arc<T>) {
        // We have to swap it, anyway.
        self.swap(new);
    }

    /// Swaps the inner `Arc` with the argument `new` inconditionally.
    pub fn swap(&self, new: Arc<T>) -> Arc<T> {
        let new = Arc::into_raw(new) as *mut _;
        let ptr = self.ptr.swap(new, Relaxed);
        // You may think "We could just return the arc without cloning".
        // Well, we need to clone because the arc may be dropped right
        // after the return of the method, so, we need clone and the
        // later_drop above to ensure no use-after free.
        let arc = unsafe { Arc::from_raw(ptr) };
        Arc::into_raw(arc.clone());
        unsafe { incinerator::add(NonNull::new_unchecked(ptr), drop_arc) };
        arc
    }

    /// Compares the inner `Arc` with `curr`, and if they are the same pointer,
    /// the inner `Arc` is swapped with `new`. To test the result, use
    /// `Arc::ptr_eq(&curr, &ret)`.
    pub fn compare_and_swap(&self, curr: Arc<T>, new: Arc<T>) -> Arc<T> {
        let curr = Arc::into_raw(curr) as *mut _;
        let new = Arc::into_raw(new) as *mut _;

        let res = incinerator::pause(|| {
            let ptr = self.ptr.compare_and_swap(curr, new, Relaxed);
            if ptr == curr {
                // Behaves as a swap.
                // We need to later_drop the loaded pointer for the same reason
                // as in swap.
                unsafe {
                    incinerator::add(NonNull::new_unchecked(ptr), drop_arc);
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

        incinerator::try_force();
        res
    }

    /// Same as `compare_and_swap` but it returns a `Result` instead.
    pub fn compare_exchange(
        &self,
        curr: Arc<T>,
        new: Arc<T>,
    ) -> Result<Arc<T>, Arc<T>> {
        let curr = Arc::into_raw(curr) as *mut _;
        let new = Arc::into_raw(new) as *mut _;
        let res = incinerator::pause(|| {
            let res = self.ptr.compare_exchange(curr, new, Relaxed, Relaxed);
            match res {
                Ok(ptr) => {
                    // We need to later_drop the loaded pointer for the same
                    // reason as in swap.
                    unsafe {
                        incinerator::add(NonNull::new_unchecked(ptr), drop_arc);
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
        incinerator::try_force();
        res
    }

    /// Same as `compare_exchange` but with weaker semanthics.
    pub fn compare_exchange_weak(
        &self,
        curr: Arc<T>,
        new: Arc<T>,
    ) -> Result<Arc<T>, Arc<T>> {
        let curr = Arc::into_raw(curr) as *mut _;
        let new = Arc::into_raw(new) as *mut _;
        let res = incinerator::pause(|| {
            let res =
                self.ptr.compare_exchange_weak(curr, new, Relaxed, Relaxed);
            match res {
                Ok(ptr) => {
                    // We need to later_drop the loaded pointer for the same
                    // reason as in swap.
                    unsafe {
                        incinerator::add(NonNull::new_unchecked(ptr), drop_arc);
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
        incinerator::try_force();
        res
    }
}

impl<T> Drop for Darc<T> {
    fn drop(&mut self) {
        unsafe {
            incinerator::add(
                NonNull::new_unchecked(self.ptr.load(Relaxed)),
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

impl<T> Default for Darc<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(Arc::default())
    }
}

fn drop_arc<T>(ptr: NonNull<T>) {
    unsafe {
        Arc::from_raw(ptr.as_ptr());
    }
}

unsafe impl<T> Send for Darc<T> where T: Send + Sync {}

unsafe impl<T> Sync for Darc<T> where T: Send + Sync {}

// Testing the safety of `unsafe` in this module is done with random operations
// via fuzzing
#[cfg(test)]
mod test {
    use super::*;
    use std::{sync::Arc, thread};

    #[test]
    fn load() {
        let arc = Arc::new(5);
        let darc = Darc::new(arc.clone());
        assert!(Arc::ptr_eq(&arc, &darc.load()));
        assert_eq!(*darc.load(), 5);
    }

    #[test]
    fn store() {
        let arc = Arc::new(5);
        let darc = Darc::new(arc.clone());
        let new_arc = Arc::new(6);
        darc.store(new_arc.clone());
        assert!(!Arc::ptr_eq(&arc, &darc.load()));
        assert!(Arc::ptr_eq(&new_arc, &darc.load()));
        assert_ne!(*darc.load(), 5);
        assert_eq!(*darc.load(), 6);

        let newer = Arc::new(6);
        darc.store(newer.clone());
        assert!(!Arc::ptr_eq(&new_arc, &darc.load()));
        assert!(Arc::ptr_eq(&newer, &darc.load()));
        assert_ne!(*darc.load(), 5);
        assert_eq!(*darc.load(), 6);
    }

    #[test]
    fn swap() {
        let arc = Arc::new(5);
        let darc = Darc::new(arc.clone());
        let new_arc = Arc::new(6);
        let swapped = darc.swap(new_arc.clone());
        assert!(!Arc::ptr_eq(&swapped, &new_arc));
        assert!(Arc::ptr_eq(&swapped, &arc));
        assert!(!Arc::ptr_eq(&arc, &darc.load()));
        assert!(Arc::ptr_eq(&new_arc, &darc.load()));
        assert_ne!(*darc.load(), 5);
        assert_eq!(*darc.load(), 6);

        let newer = Arc::new(6);
        let swapped = darc.swap(newer.clone());
        assert!(!Arc::ptr_eq(&swapped, &newer));
        assert!(Arc::ptr_eq(&swapped, &new_arc));
        assert!(!Arc::ptr_eq(&new_arc, &darc.load()));
        assert!(Arc::ptr_eq(&newer, &darc.load()));
        assert_ne!(*darc.load(), 5);
        assert_eq!(*darc.load(), 6);
    }

    #[test]
    fn compare_and_swap() {
        let x = Arc::new(5);
        let y = Arc::new(6);
        let z = Arc::new(7);
        let darc = Darc::new(x.clone());
        assert!(!Arc::ptr_eq(&y, &darc.compare_and_swap(y.clone(), z.clone())));
        assert!(Arc::ptr_eq(&x, &darc.compare_and_swap(x.clone(), z.clone())));
        assert!(Arc::ptr_eq(&z, &darc.load()));
    }

    #[test]
    fn compare_exchange() {
        let x = Arc::new(5);
        let y = Arc::new(6);
        let z = Arc::new(7);
        let darc = Darc::new(x.clone());
        assert!(darc.compare_exchange(y.clone(), z.clone()).is_err());
        assert!(darc.compare_exchange(x.clone(), z.clone()).is_ok());
        assert!(Arc::ptr_eq(&z, &darc.load()));
    }

    #[test]
    fn compare_exchange_weak() {
        let x = Arc::new(5);
        let y = Arc::new(6);
        let z = Arc::new(7);
        let darc = Darc::new(x.clone());
        assert!(darc.compare_exchange_weak(y.clone(), z.clone()).is_err());
        if darc.compare_exchange_weak(x.clone(), z.clone()).is_ok() {
            assert!(Arc::ptr_eq(&z, &darc.load()));
        }
    }

    #[test]
    fn no_data_corruption() {
        const NTHREADS: usize = 20;
        let darc = Arc::new(Darc::new(Arc::new(12)));
        let mut threads = Vec::with_capacity(NTHREADS);
        for i in 0 .. NTHREADS {
            let darc = darc.clone();
            threads.push(thread::spawn(move || loop {
                let inner = darc.load();
                let new = Arc::new(*inner + i);
                let res = darc.compare_and_swap(inner.clone(), new);
                if Arc::ptr_eq(&res, &inner) {
                    break;
                }
            }));
        }

        let sum = (0 .. NTHREADS).sum::<usize>() + 12;

        for thread in threads {
            thread.join().expect("sub-thread failed");
        }

        assert_eq!(*darc.load(), sum);
    }
}
