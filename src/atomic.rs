use alloc::*;
use incinerator;
pub use std::sync::atomic::Ordering;
use std::{
    fmt,
    ptr::{null_mut, NonNull},
    sync::atomic::{
        AtomicBool,
        AtomicIsize,
        AtomicPtr,
        AtomicUsize,
        Ordering::*,
    },
};

/// A box which can atomically load/store non-word-sized but Copy values.
/// # Example
/// ```rust
/// extern crate lockfree;
/// use lockfree::atomic::{Atomic, AtomicBox, Ordering::*};
/// use std::{sync::Arc, thread};
///
/// let atomic = Arc::new(AtomicBox::new((0u128, 2u128)));
/// let mut threads = Vec::new();
/// for i in 1 ..= 10 {
///     let atomic = atomic.clone();
///     threads.push(thread::spawn(move || {
///         let loaded = atomic.load(Acquire);
///         atomic.cas_loop(loaded, |(x, y)| Some((x + i, y + 1 + i)), AcqRel);
///     }));
/// }
///
/// for thread in threads {
///     thread.join().expect("thread failed");
/// }
/// assert_eq!(atomic.load(Relaxed), (55, 67));
/// ```
pub struct AtomicBox<T> {
    ptr: AtomicPtr<T>,
}

/// Specifies conversion between a type and its atomic form.
pub trait IntoAtomic: Copy + Eq {
    /// The target atomic type, which must have itself as inner type.
    type Target: Atomic<Inner = Self>;

    /// Performs the conversion.
    fn into_atomic(self) -> Self::Target {
        Self::Target::new(self)
    }
}

/// Speicifies atomic operations on a given type.
pub trait Atomic: Send + Sync {
    /// The inner type of atomic operations.
    type Inner: Copy + Eq;

    /// Creates a new atomic value from the inner type's value.
    fn new(init: Self::Inner) -> Self;

    /// Loads the inner value with the given ordering. Has the same semantics
    /// as stdlib atomics.
    fn load(&self, ord: Ordering) -> Self::Inner;

    /// Stores the inner value with the given ordering. Has the same semantics
    /// as stdlib atomics.
    fn store(&self, val: Self::Inner, ord: Ordering);

    /// Sets the inner value and returns the old value with the given ordering.
    /// Has the same semantics as stdlib atomics.
    fn swap(&self, val: Self::Inner, ord: Ordering) -> Self::Inner;

    /// With the given ordering, tests the stored value with an expected one,
    /// if succeeds, sets the inner value. Returns the old value anyway. Has
    /// the same semantics as stdlib atomics.
    fn compare_and_swap(
        &self,
        curr: Self::Inner,
        new: Self::Inner,
        ord: Ordering,
    ) -> Self::Inner;

    /// With the first ordering for success and the second for failures, tests
    /// the stored value with an expected one, if succeeds, sets the inner
    /// value. Returns the old value as either `Ok` or `Err`. Has the same
    /// semantics as stdlib atomics.
    fn compare_exchange(
        &self,
        curr: Self::Inner,
        new: Self::Inner,
        succ: Ordering,
        fail: Ordering,
    ) -> Result<Self::Inner, Self::Inner>;

    /// Same as `compare_exchange`, but weaker. This means the swap may fail
    /// even if the comparison succeeds. Has the same semantics as stdlib
    /// atomics.
    fn compare_exchange_weak(
        &self,
        curr: Self::Inner,
        new: Self::Inner,
        succ: Ordering,
        fail: Ordering,
    ) -> Result<Self::Inner, Self::Inner>;

    /// Abstraction over a loop with `compare_and_swap` method. The function
    /// `fun`, which receives the current state, processes it, and if a
    /// compare and swap is needed, the function returns `Some(new_value)`,
    /// otherwise, `None`. This method loops until either `None` is returned (a
    /// failure), or compare and swap succeeds (a success). An initial state
    /// is given, and it is updated with `compare_and_swap` return value.
    fn cas_loop<F>(
        &self,
        mut loaded: Self::Inner,
        mut fun: F,
        cmp_swap: Ordering,
    ) -> Result<Self::Inner, Self::Inner>
    where
        F: FnMut(Self::Inner) -> Option<Self::Inner>,
    {
        loop {
            let new = match fun(loaded) {
                Some(val) => val,
                None => break Err(loaded),
            };
            let res = self.compare_and_swap(loaded, new, cmp_swap);
            if res == loaded {
                break Ok(loaded);
            }
            loaded = res;
        }
    }

    /// Abstraction over a loop with `load` and `compare_and_swap` methods. The
    /// main difference between this method and `cas_loop` is that instead of
    /// taking an initial value and updating it with cas, it always get the
    /// value from the `load` method.
    fn load_cas_loop<F>(
        &self,
        mut fun: F,
        load: Ordering,
        cmp_swap: Ordering,
    ) -> Result<Self::Inner, Self::Inner>
    where
        F: FnMut(Self::Inner) -> Option<Self::Inner>,
    {
        loop {
            let loaded = self.load(load);
            let new = match fun(loaded) {
                Some(val) => val,
                None => break Err(loaded),
            };
            let res = self.compare_and_swap(loaded, new, cmp_swap);
            if res == loaded {
                break Ok(loaded);
            }
        }
    }
}

macro_rules! impl_atomic {
    (<$($params:ident),*> $atom_type:ty, $inner_type:ty) => {
        impl<$($params),*> Atomic for $atom_type {
            type Inner = $inner_type;

            fn new(init: Self::Inner) -> Self {
                Self::new(init)
            }

            fn load(&self, ord: Ordering) -> Self::Inner {
                self.load(ord)
            }

            fn store(&self, val: Self::Inner, ord: Ordering) {
                self.store(val, ord)
            }

            fn swap(&self, val: Self::Inner, ord: Ordering) -> Self::Inner {
                self.swap(val, ord)
            }

            fn compare_and_swap(
                &self,
                curr: Self::Inner,
                new: Self::Inner,
                ord: Ordering,
            ) -> Self::Inner {
                self.compare_and_swap(curr, new, ord)
            }

            fn compare_exchange(
                &self,
                curr: Self::Inner,
                new: Self::Inner,
                succ: Ordering,
                fail: Ordering,
            ) -> Result<Self::Inner, Self::Inner> {
                self.compare_exchange(curr, new, succ, fail)
            }

            fn compare_exchange_weak(
                &self,
                curr: Self::Inner,
                new: Self::Inner,
                succ: Ordering,
                fail: Ordering,
            ) -> Result<Self::Inner, Self::Inner> {
                self.compare_exchange_weak(curr, new, succ, fail)
            }
        }
    };
}

impl_atomic! { <> AtomicBool, bool }
impl_atomic! { <> AtomicUsize, usize }
impl_atomic! { <> AtomicIsize, isize }
impl_atomic! { <T> AtomicPtr<T>, *mut T }

impl IntoAtomic for bool {
    type Target = AtomicBool;
}

impl IntoAtomic for usize {
    type Target = AtomicUsize;
}

impl IntoAtomic for isize {
    type Target = AtomicIsize;
}

impl<T> IntoAtomic for *mut T {
    type Target = AtomicPtr<T>;
}

impl<T> Atomic for AtomicBox<T>
where
    T: Copy + Eq,
{
    type Inner = T;

    fn new(init: Self::Inner) -> Self {
        Self { ptr: unsafe { alloc(init) }.as_ptr().into_atomic() }
    }

    fn load(&self, ord: Ordering) -> Self::Inner {
        incinerator::pause(|| unsafe { *self.ptr.load(ord) })
    }

    fn store(&self, val: Self::Inner, ord: Ordering) {
        self.swap(val, ord);
    }

    fn swap(&self, val: Self::Inner, ord: Ordering) -> Self::Inner {
        let ptr = self.ptr.swap(unsafe { alloc(val).as_ptr() }, ord);
        let res = unsafe { *ptr };
        unsafe { incinerator::add(NonNull::new_unchecked(ptr), dealloc) }
        res
    }

    fn compare_and_swap(
        &self,
        curr: Self::Inner,
        new: Self::Inner,
        ord: Ordering,
    ) -> Self::Inner {
        let load_ord = match ord {
            Acquire | AcqRel | Release => Acquire,
            _ => ord,
        };
        match self.load_cas_loop(
            |val| if val == curr { Some(new) } else { None },
            load_ord,
            ord,
        ) {
            Ok(val) => val,
            Err(val) => val,
        }
    }

    fn compare_exchange(
        &self,
        curr: Self::Inner,
        new: Self::Inner,
        succ: Ordering,
        fail: Ordering,
    ) -> Result<Self::Inner, Self::Inner> {
        let new_ptr = unsafe { alloc(new).as_ptr() };

        let load_ord = match succ {
            Acquire | AcqRel | Release => Acquire,
            _ => succ,
        };

        let (result, ptr) = incinerator::pause(|| {
            let mut loaded_ptr = self.ptr.load(load_ord);

            loop {
                let loaded = unsafe { *loaded_ptr };
                if loaded == curr {
                    match self
                        .ptr
                        .compare_exchange(loaded_ptr, new_ptr, succ, fail)
                    {
                        Ok(res_ptr) => break (Ok(loaded), res_ptr),
                        Err(res_ptr) => loaded_ptr = res_ptr,
                    }
                } else {
                    unsafe { dealloc(NonNull::new_unchecked(new_ptr)) }
                    break (Err(loaded), null_mut());
                }
            }
        });

        if let Some(ptr) = NonNull::new(ptr) {
            unsafe { incinerator::add(ptr, dealloc) }
        }

        result
    }

    fn compare_exchange_weak(
        &self,
        curr: Self::Inner,
        new: Self::Inner,
        succ: Ordering,
        fail: Ordering,
    ) -> Result<Self::Inner, Self::Inner> {
        let new_ptr = unsafe { alloc(new).as_ptr() };

        let load_ord = match succ {
            Acquire | AcqRel | Release => Acquire,
            _ => succ,
        };

        let (result, ptr) = incinerator::pause(|| {
            let loaded_ptr = self.ptr.load(load_ord);

            let loaded = unsafe { *loaded_ptr };
            if loaded == curr {
                match self.ptr.compare_exchange(loaded_ptr, new_ptr, succ, fail)
                {
                    Ok(res_ptr) => (Ok(loaded), res_ptr),
                    Err(_) => {
                        unsafe { dealloc(NonNull::new_unchecked(new_ptr)) }
                        (Err(loaded), null_mut())
                    },
                }
            } else {
                unsafe { dealloc(NonNull::new_unchecked(new_ptr)) }
                (Err(loaded), null_mut())
            }
        });

        if let Some(ptr) = NonNull::new(ptr) {
            unsafe { incinerator::add(ptr, dealloc) }
        }

        result
    }

    fn load_cas_loop<F>(
        &self,
        mut update: F,
        load_ord: Ordering,
        cas_ord: Ordering,
    ) -> Result<Self::Inner, Self::Inner>
    where
        F: FnMut(Self::Inner) -> Option<Self::Inner>,
    {
        let new_ptr = unsafe { alloc_uninit() }.as_ptr();

        let (result, ptr) = incinerator::pause(|| {
            let mut loaded_ptr = self.ptr.load(load_ord);

            loop {
                let loaded = unsafe { *loaded_ptr };
                if let Some(new) = update(loaded) {
                    unsafe { *new_ptr = new }
                    let res_ptr =
                        self.ptr.compare_and_swap(loaded_ptr, new_ptr, cas_ord);
                    if res_ptr == loaded_ptr {
                        break (Ok(loaded), res_ptr);
                    } else {
                        loaded_ptr = res_ptr;
                    }
                } else {
                    unsafe { dealloc(NonNull::new_unchecked(new_ptr)) }
                    break (Err(loaded), null_mut());
                }
            }
        });

        if let Some(ptr) = NonNull::new(ptr) {
            unsafe { incinerator::add(ptr, dealloc) }
        }

        result
    }
}

impl<T> Drop for AtomicBox<T> {
    fn drop(&mut self) {
        unsafe { dealloc(NonNull::new_unchecked(self.ptr.load(Relaxed))) }
    }
}

impl<T> fmt::Debug for AtomicBox<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str("AtomicBox")
    }
}

impl<T> From<T> for AtomicBox<T>
where
    T: Copy + Eq,
{
    fn from(val: T) -> Self {
        Self::new(val)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{sync::Arc, thread};

    #[test]
    #[allow(unused_must_use)]
    fn cas_loop_multithreaded() {
        let atomic = Arc::new(AtomicUsize::new(0));
        let mut threads = Vec::new();
        for i in 1 ..= 10 {
            let atomic = atomic.clone();
            threads.push(thread::spawn(move || {
                let loaded = atomic.load(Acquire);
                atomic.cas_loop(loaded, |x| Some(x + i), AcqRel);
            }));
        }

        for thread in threads {
            thread.join().expect("thread failed");
        }
        assert_eq!(atomic.load(Relaxed), 55);
    }

    #[test]
    #[allow(unused_must_use)]
    fn load_cas_loop_multithreaded() {
        let atomic = Arc::new(AtomicUsize::new(0));
        let mut threads = Vec::new();
        for i in 1 ..= 10 {
            let atomic = atomic.clone();
            threads.push(thread::spawn(move || {
                atomic.load_cas_loop(|x| Some(x + i), Acquire, Release);
            }));
        }

        for thread in threads {
            thread.join().expect("thread failed");
        }
        assert_eq!(atomic.load(Relaxed), 55);
    }

    #[test]
    fn box_load() {
        let atomic = AtomicBox::new(14235u128);
        assert_eq!(atomic.load(Relaxed), 14235u128);
    }

    #[test]
    fn box_store() {
        let atomic = AtomicBox::new(0u128);
        atomic.store(14235u128, Relaxed);
        assert_eq!(atomic.load(Relaxed), 14235u128);
    }

    #[test]
    fn box_swap() {
        let atomic = AtomicBox::new(0u128);
        assert_eq!(atomic.swap(5u128, Relaxed), 0u128);
        assert_eq!(atomic.swap(14235u128, Relaxed), 5u128);
        assert_eq!(atomic.load(Relaxed), 14235u128);
    }

    #[test]
    fn box_cas() {
        let atomic = AtomicBox::new(0u128);
        assert_eq!(atomic.compare_and_swap(5u128, 14235u128, Relaxed), 0u128);
        assert_eq!(atomic.compare_and_swap(5u128, 14235u128, Relaxed), 0u128);
        assert_eq!(atomic.compare_and_swap(0u128, 14235u128, Relaxed), 0u128);
        assert_eq!(
            atomic.compare_and_swap(0u128, 14235u128, Relaxed),
            14235u128
        );
        assert_eq!(
            atomic.compare_and_swap(14235u128, 5u128, Relaxed),
            14235u128
        );
        assert_eq!(atomic.compare_and_swap(14235u128, 5u128, Relaxed), 5u128);
        assert_eq!(atomic.load(Relaxed), 5u128);
    }

    #[test]
    fn box_cmp_xchg() {
        let atomic = AtomicBox::new(0u128);
        assert_eq!(
            atomic.compare_exchange(5u128, 14235u128, Relaxed, Relaxed),
            Err(0u128)
        );
        assert_eq!(
            atomic.compare_exchange(5u128, 14235u128, Relaxed, Relaxed),
            Err(0u128)
        );
        assert_eq!(
            atomic.compare_exchange(0u128, 14235u128, Relaxed, Relaxed),
            Ok(0u128)
        );
        assert_eq!(
            atomic.compare_exchange(0u128, 14235u128, Relaxed, Relaxed),
            Err(14235u128)
        );
        assert_eq!(
            atomic.compare_exchange(14235u128, 5u128, Relaxed, Relaxed),
            Ok(14235u128)
        );
        assert_eq!(
            atomic.compare_exchange(14235u128, 5u128, Relaxed, Relaxed),
            Err(5u128)
        );
        assert_eq!(atomic.load(Relaxed), 5u128);
    }

    #[test]
    fn box_cmp_xchg_weak() {
        let atomic = AtomicBox::new(0u128);
        assert_eq!(
            atomic.compare_exchange_weak(5u128, 14235u128, Relaxed, Relaxed),
            Err(0u128)
        );
        assert_eq!(
            atomic.compare_exchange_weak(5u128, 14235u128, Relaxed, Relaxed),
            Err(0u128)
        );
        match atomic.compare_exchange_weak(0u128, 14235u128, Relaxed, Relaxed) {
            Ok(x) | Err(x) => assert_eq!(x, 0u128),
        }
    }

    #[test]
    fn box_load_cas_loop() {
        let atomic = AtomicBox::new(0u128);
        assert_eq!(
            atomic.load_cas_loop(|x| Some(x + 2), Relaxed, Relaxed),
            Ok(0u128)
        );
        assert_eq!(atomic.load(Relaxed), 2u128);
    }

    #[test]
    #[allow(unused_must_use)]
    fn box_multithreaded() {
        let atomic = Arc::new(AtomicBox::new(0u128));
        let mut threads = Vec::new();
        for i in 1 ..= 10 {
            let atomic = atomic.clone();
            threads.push(thread::spawn(move || {
                let loaded = atomic.load(Acquire);
                atomic.cas_loop(loaded, |x| Some(x + i), AcqRel);
            }));
        }

        for thread in threads {
            thread.join().expect("thread failed");
        }
        assert_eq!(atomic.load(Relaxed), 55);
    }
}
