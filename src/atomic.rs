use owned_alloc::{Cache, OwnedAlloc, UninitAlloc};
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

/// Specifies conversion between a type and its atomic form.
pub trait IntoAtomic: Copy + PartialEq {
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
    type Inner: Copy + PartialEq;

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
    /// value. Returns the old value as either [`Ok`] or [`Err`]. Has the same
    /// semantics as stdlib atomics.
    fn compare_exchange(
        &self,
        curr: Self::Inner,
        new: Self::Inner,
        succ: Ordering,
        fail: Ordering,
    ) -> Result<Self::Inner, Self::Inner>;

    /// Same as [`compare_exchange`](Atomic::compare_exchange), but weaker. This
    /// means the swap may fail even if the comparison succeeds. Has the
    /// same semantics as stdlib atomics.
    fn compare_exchange_weak(
        &self,
        curr: Self::Inner,
        new: Self::Inner,
        succ: Ordering,
        fail: Ordering,
    ) -> Result<Self::Inner, Self::Inner>;

    /// Abstraction over a loop with
    /// [`compare_and_swap`](Atomic::compare_and_swap) method. The function
    /// `fun`, which receives the current state, processes it, and if a
    /// compare and swap is needed, the function returns `Some(new_value)`,
    /// otherwise, `None`. This method loops until either `None` is returned (a
    /// failure), or compare and swap succeeds (a success). An initial state
    /// is given, and it is updated with
    /// [`compare_and_swap`](Atomic::compare_and_swap) return value.
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

    /// Abstraction over a loop with [`load`](Atomic::load) and
    /// [`compare_and_swap`](Atomic::compare_and_swap) methods. The main
    /// difference between this method and `cas_loop` is that instead of
    /// taking an initial value and updating it with cas, it always get the
    /// value from the [`load`](Atomic::load) method.
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

    /// Returns the inner value stored within this atomic.
    fn into_inner(self) -> Self::Inner
    where
        Self: Sized,
    {
        self.load(Relaxed)
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
    incin: BoxSharedIncin<T>,
}

impl<T> AtomicBox<T>
where
    T: Copy + PartialEq + Send,
{
    /// Creates the [`AtomicBox`] with an initial value and a given shared
    /// incinerator.
    pub fn with_incin(init: T, incin: BoxSharedIncin<T>) -> Self {
        Self {
            ptr: OwnedAlloc::new(init).into_raw().as_ptr().into_atomic(),
            incin,
        }
    }

    /// The shared incinerator used by this [`AtomicBox`].
    pub fn incin(&self) -> BoxSharedIncin<T> {
        self.incin.clone()
    }

    /// Returns a mutable reference to the stored data.
    /// This is safe because it requires a mutable (thus exclusive) reference.
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr.load(Relaxed) }
    }
}

impl<T> Atomic for AtomicBox<T>
where
    T: Copy + PartialEq + Send,
{
    type Inner = T;

    fn new(init: Self::Inner) -> Self {
        Self::with_incin(init, BoxSharedIncin::new())
    }

    fn load(&self, ord: Ordering) -> Self::Inner {
        self.incin
            .inner
            .pause_with(|| unsafe { *self.ptr.load(ord) })
    }

    fn store(&self, val: Self::Inner, ord: Ordering) {
        self.swap(val, ord);
    }

    fn swap(&self, val: Self::Inner, ord: Ordering) -> Self::Inner {
        let ptr = self.ptr.swap(OwnedAlloc::new(val).into_raw().as_ptr(), ord);
        let res = unsafe { *ptr };
        self.incin
            .inner
            .add(unsafe { OwnedAlloc::from_raw(NonNull::new_unchecked(ptr)) });
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
        let new_nnptr = OwnedAlloc::new(new).into_raw();

        let load_ord = match succ {
            Acquire | AcqRel | Release => Acquire,
            _ => succ,
        };

        let (result, ptr) = self.incin.inner.pause_with(|| {
            let mut loaded_ptr = self.ptr.load(load_ord);
            loop {
                let loaded = unsafe { *loaded_ptr };

                if loaded == curr {
                    match self.ptr.compare_exchange(
                        loaded_ptr,
                        new_nnptr.as_ptr(),
                        succ,
                        fail,
                    ) {
                        Ok(res_ptr) => {
                            let to_drop = Some(unsafe {
                                NonNull::new_unchecked(res_ptr)
                            });
                            break (Ok(loaded), to_drop);
                        },

                        Err(res_ptr) => loaded_ptr = res_ptr,
                    }
                } else {
                    unsafe { OwnedAlloc::from_raw(new_nnptr) };

                    break (Err(loaded), None);
                }
            }
        });

        if let Some(nnptr) = ptr {
            self.incin.inner.add(unsafe { OwnedAlloc::from_raw(nnptr) });
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
        let new_nnptr = OwnedAlloc::new(new).into_raw();

        let load_ord = match succ {
            Acquire | AcqRel | Release => Acquire,
            _ => succ,
        };

        let (result, ptr) = self.incin.inner.pause_with(|| {
            let loaded_ptr = self.ptr.load(load_ord);

            let loaded = unsafe { *loaded_ptr };
            if loaded == curr {
                match self.ptr.compare_exchange_weak(
                    loaded_ptr,
                    new_nnptr.as_ptr(),
                    succ,
                    fail,
                ) {
                    Ok(res_ptr) => (
                        Ok(loaded),
                        Some(unsafe { NonNull::new_unchecked(res_ptr) }),
                    ),

                    Err(_) => {
                        unsafe { OwnedAlloc::from_raw(new_nnptr) };
                        (Err(loaded), None)
                    },
                }
            } else {
                unsafe { OwnedAlloc::from_raw(new_nnptr) };
                (Err(loaded), None)
            }
        });

        if let Some(nnptr) = ptr {
            self.incin.inner.add(unsafe { OwnedAlloc::from_raw(nnptr) });
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
        let mut new_cache = Cache::new();

        let (result, ptr) = self.incin.inner.pause_with(|| {
            let mut loaded_ptr = self.ptr.load(load_ord);

            loop {
                let loaded = unsafe { *loaded_ptr };
                if let Some(new) = update(loaded) {
                    let new_alloc =
                        new_cache.take_or(|| UninitAlloc::new()).init(new);
                    let new_nnptr = new_alloc.into_raw();

                    let res_ptr = self.ptr.compare_and_swap(
                        loaded_ptr,
                        new_nnptr.as_ptr(),
                        cas_ord,
                    );
                    if res_ptr == loaded_ptr {
                        let to_drop =
                            Some(unsafe { NonNull::new_unchecked(res_ptr) });

                        break (Ok(loaded), to_drop);
                    } else {
                        new_cache
                            .store(unsafe { UninitAlloc::from_raw(new_nnptr) });
                        loaded_ptr = res_ptr;
                    }
                } else {
                    break (Err(loaded), None);
                }
            }
        });

        if let Some(nnptr) = ptr {
            self.incin.inner.add(unsafe { OwnedAlloc::from_raw(nnptr) });
        }

        result
    }
}

impl<T> Drop for AtomicBox<T> {
    fn drop(&mut self) {
        unsafe {
            OwnedAlloc::from_raw(NonNull::new_unchecked(
                self.ptr.load(Relaxed),
            ));
        }
    }
}

impl<T> fmt::Debug for AtomicBox<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "AtomicBox {} ptr: {:?} {}", '{', self.ptr, '}')
    }
}

impl<T> Default for AtomicBox<T>
where
    T: Copy + PartialEq + Default + Send,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> From<T> for AtomicBox<T>
where
    T: Copy + PartialEq + Send,
{
    fn from(val: T) -> Self {
        Self::new(val)
    }
}

/// Atomic Optional Box of `T`. Similar to [`AtomicBox`], but the pointer of
/// [`AtomicOptionBox`] can be null.
pub struct AtomicOptionBox<T> {
    ptr: AtomicPtr<T>,
    incin: BoxSharedIncin<T>,
}

impl<T> AtomicOptionBox<T>
where
    T: Copy + PartialEq + Send,
{
    /// Creates the [`AtomicOptionBox`] with an initial value and a given shared
    /// incinerator.
    pub fn with_incin(init: Option<T>, incin: BoxSharedIncin<T>) -> Self {
        Self {
            ptr: Self::make_ptr(init).into_atomic(),
            incin,
        }
    }

    /// The shared incinerator used by this [`AtomicOptionBox`].
    pub fn incin(&self) -> BoxSharedIncin<T> {
        self.incin.clone()
    }

    /// Returns a mutable reference to the stored data, if any.
    /// This is safe because it requires a mutable (thus exclusive) reference.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        unsafe { self.ptr.load(Relaxed).as_mut() }
    }

    fn make_ptr(val: Option<T>) -> *mut T {
        val.map_or(null_mut(), |val| OwnedAlloc::new(val).into_raw().as_ptr())
    }

    unsafe fn make_val(ptr: *mut T) -> Option<T> {
        ptr.as_ref().map(|&x| x)
    }

    unsafe fn make_val_and_dealloc(&self, ptr: *mut T) -> Option<T> {
        NonNull::new(ptr).map(|nnptr| {
            let val = *nnptr.as_ref();
            self.incin.inner.add(OwnedAlloc::from_raw(nnptr));
            val
        })
    }
}

impl<T> Atomic for AtomicOptionBox<T>
where
    T: Copy + PartialEq + Send,
{
    type Inner = Option<T>;

    fn new(init: Self::Inner) -> Self {
        Self::with_incin(init, BoxSharedIncin::new())
    }

    fn load(&self, ord: Ordering) -> Self::Inner {
        self.incin
            .inner
            .pause_with(|| unsafe { Self::make_val(self.ptr.load(ord)) })
    }

    fn store(&self, val: Self::Inner, ord: Ordering) {
        self.swap(val, ord);
    }

    fn swap(&self, val: Self::Inner, ord: Ordering) -> Self::Inner {
        let ptr = self.ptr.swap(Self::make_ptr(val), ord);
        unsafe { self.make_val_and_dealloc(ptr) }
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
        let new_ptr = Self::make_ptr(new);

        let load_ord = match succ {
            Acquire | AcqRel | Release => Acquire,
            _ => succ,
        };

        let (result, ptr) = self.incin.inner.pause_with(|| {
            let mut loaded_ptr = self.ptr.load(load_ord);

            loop {
                let loaded = unsafe { Self::make_val(loaded_ptr) };
                if loaded == curr {
                    match self
                        .ptr
                        .compare_exchange(loaded_ptr, new_ptr, succ, fail)
                    {
                        Ok(res_ptr) => {
                            break (Ok(loaded), NonNull::new(res_ptr))
                        },

                        Err(res_ptr) => loaded_ptr = res_ptr,
                    }
                } else {
                    if let Some(nnptr) = NonNull::new(new_ptr) {
                        unsafe { OwnedAlloc::from_raw(nnptr) };
                    }

                    break (Err(loaded), None);
                }
            }
        });

        if let Some(nnptr) = ptr {
            self.incin.inner.add(unsafe { OwnedAlloc::from_raw(nnptr) });
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
        let new_ptr = Self::make_ptr(new);

        let load_ord = match succ {
            Acquire | AcqRel | Release => Acquire,
            _ => succ,
        };

        let (result, ptr) = self.incin.inner.pause_with(|| {
            let loaded_ptr = self.ptr.load(load_ord);

            let loaded = unsafe { Self::make_val(loaded_ptr) };
            if loaded == curr {
                match self
                    .ptr
                    .compare_exchange_weak(loaded_ptr, new_ptr, succ, fail)
                {
                    Ok(res_ptr) => (Ok(loaded), NonNull::new(res_ptr)),

                    Err(_) => {
                        if let Some(nnptr) = NonNull::new(new_ptr) {
                            unsafe { OwnedAlloc::from_raw(nnptr) };
                        }
                        (Err(loaded), None)
                    },
                }
            } else {
                if let Some(nnptr) = NonNull::new(new_ptr) {
                    unsafe { OwnedAlloc::from_raw(nnptr) };
                }

                (Err(loaded), None)
            }
        });

        if let Some(nnptr) = ptr {
            self.incin.inner.add(unsafe { OwnedAlloc::from_raw(nnptr) });
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
        let mut new_cache = Cache::new();

        let (result, ptr) = self.incin.inner.pause_with(|| {
            let mut loaded_ptr = self.ptr.load(load_ord);

            loop {
                let loaded = unsafe { Self::make_val(loaded_ptr) };
                if let Some(new) = update(loaded) {
                    let new_ptr = match new {
                        Some(val) => {
                            let alloc = new_cache
                                .take_or(|| UninitAlloc::new())
                                .init(val);
                            alloc.into_raw().as_ptr()
                        },

                        None => null_mut(),
                    };

                    let res_ptr =
                        self.ptr.compare_and_swap(loaded_ptr, new_ptr, cas_ord);

                    if res_ptr == loaded_ptr {
                        break (Ok(loaded), NonNull::new(res_ptr));
                    } else {
                        if let Some(nnptr) = NonNull::new(new_ptr) {
                            new_cache
                                .store(unsafe { UninitAlloc::from_raw(nnptr) });
                        }
                        loaded_ptr = res_ptr;
                    }
                } else {
                    break (Err(loaded), None);
                }
            }
        });

        if let Some(nnptr) = ptr {
            self.incin.inner.add(unsafe { OwnedAlloc::from_raw(nnptr) });
        }

        result
    }
}

impl<T> Drop for AtomicOptionBox<T> {
    fn drop(&mut self) {
        if let Some(nnptr) = NonNull::new(self.ptr.load(Relaxed)) {
            unsafe { OwnedAlloc::from_raw(nnptr) };
        }
    }
}

impl<T> fmt::Debug for AtomicOptionBox<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "AtomicOptionBox {} ptr: {:?} {}", '{', self.ptr, '}')
    }
}

impl<T> Default for AtomicOptionBox<T>
where
    T: Copy + PartialEq + Send,
{
    fn default() -> Self {
        Self::new(None)
    }
}

impl<T> From<T> for AtomicOptionBox<T>
where
    T: Copy + PartialEq + Send,
{
    fn from(val: T) -> Self {
        Self::new(Some(val))
    }
}

impl<T> From<Option<T>> for AtomicOptionBox<T>
where
    T: Copy + PartialEq + Send,
{
    fn from(val: Option<T>) -> Self {
        Self::new(val)
    }
}

make_shared_incin! {
    { "[`AtomicBox`] and [`AtomicOptionBox`]" }
    pub BoxSharedIncin<T> of OwnedAlloc<T>
}

impl<T> fmt::Debug for BoxSharedIncin<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "BoxSharedIncin {} inner: {:?} {}",
            '{', self.inner, '}'
        )
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

    #[test]
    fn opt_load() {
        let atomic = AtomicOptionBox::new(Some(14235u128));
        assert_eq!(atomic.load(Relaxed), Some(14235u128));
        let atomic = AtomicOptionBox::<[u128; 8]>::new(None);
        assert_eq!(atomic.load(Relaxed), None);
    }

    #[test]
    fn opt_store() {
        let atomic = AtomicOptionBox::new(Some(0u128));
        atomic.store(None, Relaxed);
        assert_eq!(atomic.load(Relaxed), None);
        atomic.store(Some(14235u128), Relaxed);
        assert_eq!(atomic.load(Relaxed), Some(14235u128));
    }

    #[test]
    fn opt_swap() {
        let atomic = AtomicOptionBox::new(Some(0u128));
        assert_eq!(atomic.swap(Some(5u128), Relaxed), Some(0u128));
        assert_eq!(atomic.swap(None, Relaxed), Some(5u128));
        assert_eq!(atomic.swap(Some(14235u128), Relaxed), None);
        assert_eq!(atomic.load(Relaxed), Some(14235u128));
    }

    #[test]
    fn opt_cas() {
        let atomic = AtomicOptionBox::new(Some(0u128));
        assert_eq!(
            atomic.compare_and_swap(Some(5u128), Some(14235u128), Relaxed),
            Some(0u128)
        );
        assert_eq!(
            atomic.compare_and_swap(Some(5u128), None, Relaxed),
            Some(0u128)
        );
        assert_eq!(
            atomic.compare_and_swap(Some(0u128), Some(14235u128), Relaxed),
            Some(0u128)
        );
        assert_eq!(
            atomic.compare_and_swap(Some(0u128), Some(14235u128), Relaxed),
            Some(14235u128)
        );
        assert_eq!(
            atomic.compare_and_swap(Some(14235u128), None, Relaxed),
            Some(14235u128)
        );
        assert_eq!(
            atomic.compare_and_swap(Some(14235u128), Some(5u128), Relaxed),
            None
        );
        assert_eq!(atomic.load(Relaxed), None);
    }

    #[test]
    fn opt_cmp_xchg() {
        let atomic = AtomicOptionBox::new(Some(0u128));
        assert_eq!(
            atomic.compare_exchange(
                Some(5u128),
                Some(14235u128),
                Relaxed,
                Relaxed
            ),
            Err(Some(0u128))
        );
        assert_eq!(
            atomic.compare_exchange(
                Some(5u128),
                Some(14235u128),
                Relaxed,
                Relaxed
            ),
            Err(Some(0u128))
        );
        assert_eq!(
            atomic.compare_exchange(Some(0u128), None, Relaxed, Relaxed),
            Ok(Some(0u128))
        );
        assert_eq!(
            atomic.compare_exchange(Some(0u128), None, Relaxed, Relaxed),
            Err(None)
        );
        assert_eq!(
            atomic.compare_exchange(None, Some(5u128), Relaxed, Relaxed),
            Ok(None)
        );
        assert_eq!(
            atomic.compare_exchange(
                Some(14235u128),
                Some(5u128),
                Relaxed,
                Relaxed
            ),
            Err(Some(5u128))
        );
        assert_eq!(atomic.load(Relaxed), Some(5u128));
    }

    #[test]
    fn opt_cmp_xchg_weak() {
        let atomic = AtomicOptionBox::new(None);
        assert_eq!(
            atomic.compare_exchange_weak(
                Some(5u128),
                Some(14235u128),
                Relaxed,
                Relaxed
            ),
            Err(None)
        );
        assert_eq!(
            atomic.compare_exchange_weak(
                Some(5u128),
                Some(14235u128),
                Relaxed,
                Relaxed
            ),
            Err(None)
        );
        match atomic.compare_exchange_weak(
            Some(0u128),
            Some(14235u128),
            Relaxed,
            Relaxed,
        ) {
            Ok(x) | Err(x) => assert_eq!(x, None),
        }
    }

    #[test]
    fn opt_load_cas_loop() {
        let atomic = AtomicOptionBox::new(Some(0u128));
        assert_eq!(
            atomic.load_cas_loop(
                |opt| Some(opt.map(|x| x + 2)),
                Relaxed,
                Relaxed
            ),
            Ok(Some(0u128))
        );
        assert_eq!(atomic.load(Relaxed), Some(2u128));
    }

    #[test]
    #[allow(unused_must_use)]
    fn opt_multithreaded() {
        let atomic = Arc::new(AtomicOptionBox::new(Some(0u128)));
        let mut threads = Vec::new();
        for i in 1 ..= 10 {
            let atomic = atomic.clone();
            threads.push(thread::spawn(move || {
                let loaded = atomic.load(Acquire);
                atomic.cas_loop(loaded, |opt| Some(opt.map(|x| x + i)), AcqRel);
            }));
        }

        for thread in threads {
            thread.join().expect("thread failed");
        }
        assert_eq!(atomic.load(Relaxed), Some(55));
    }
}
