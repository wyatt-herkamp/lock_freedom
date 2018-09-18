use alloc::*;
use std::{
    process::abort,
    ptr::{null_mut, NonNull},
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering::*},
};

type JunkYard<'drop> = Vec<Box<Dropper + 'drop>>;

/// The incinerator is a structure which concurrently reclaims resources or
/// execute other sensitive code. If needed, reclamation is deferred until
/// threads allow it to occur. The deferring happens by the creation of pauses.
/// When all pauses are dropped, the reclamation will continue to happen.
///
/// # Example
/// ```
/// extern crate lockfree;
///
/// use lockfree::prelude::*;
/// use std::{
///     ptr::{null_mut, NonNull},
///     sync::{
///         atomic::{AtomicPtr, Ordering::*},
///         Arc,
///     },
///     thread,
/// };
///
/// unsafe fn create_ptr<T>(val: T) -> NonNull<T> {
///     NonNull::new_unchecked(Box::into_raw(Box::new(val)))
/// }
///
/// unsafe fn drop_ptr<T>(ptr: NonNull<T>) {
///     Box::from_raw(ptr.as_ptr());
/// }
///
/// struct Shared {
///     inc: Incinerator<'static>,
///     ptr: AtomicPtr<usize>,
/// }
///
/// let ptr = unsafe { create_ptr(55).as_ptr() };
/// let dummy_state =
///     Arc::new(Shared { ptr: AtomicPtr::new(ptr), inc: Incinerator::new() });
/// let mut threads = Vec::with_capacity(16);
///
/// for i in 0 .. 16 {
///     let state = dummy_state.clone();
///     threads.push(thread::spawn(move || {
///         let _pause = state.inc.pause();
///         let loaded = state.ptr.load(SeqCst);
///         let new = if let Some(num) = unsafe { loaded.as_ref() } {
///             i + num
///         } else {
///             i
///         };
///         let new_ptr = unsafe { create_ptr(new).as_ptr() };
///         let ptr = state.ptr.swap(new_ptr, SeqCst);
///
///         if let Some(nnptr) = NonNull::new(ptr) {
///             // dropping
///             unsafe { state.inc.add_non_send(move || drop_ptr(nnptr)) }
///         }
///     }));
/// }
///
/// for thread in threads {
///     thread.join().unwrap();
/// }
///
/// if let Some(ptr) = NonNull::new(dummy_state.ptr.load(SeqCst)) {
///     assert!(unsafe { *ptr.as_ref() } <= 15 * 15);
///     unsafe { drop_ptr(ptr) }
/// }
/// ```
#[derive(Debug, Default)]
pub struct Incinerator<'drop> {
    paused: AtomicUsize,
    pool: AtomicPtr<JunkYard<'drop>>,
}

impl<'drop> Incinerator<'drop> {
    /// Creates a new empty incinerator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a pause. While the incinerator is paused, no reclamation
    /// function is executed.
    pub fn pause<'incin>(&'incin self) -> Pause<'incin, 'drop> {
        if self.paused.fetch_add(1, AcqRel) == usize::max_value() {
            eprintln!("Too much pauses");
            abort();
        }
        Pause { inc: self }
    }

    /// Adds a `Send`able reclamation function to the incinerator. If there is
    /// no pauses currently, the function is immediatly executed as well the
    /// functions in the "junk yard". If there is at least one pause executing,
    /// the function is added to the junk yard.
    pub fn add<F>(&self, dropper: F)
    where
        F: FnOnce() + Send + 'drop,
    {
        unsafe { self.add_non_send(dropper) }
    }

    /// The same as `add`, but allows the function to contain non-send types
    /// such as pointers, which makes this method unsafe.
    pub unsafe fn add_non_send<F>(&self, dropper: F)
    where
        F: FnOnce() + 'drop,
    {
        let maybe_sub = self.take_pool();
        if self.paused.load(Acquire) == 0 {
            dropper();

            if let Some(nnptr) = maybe_sub {
                let vec = &mut *nnptr.as_ptr();
                while let Some(dropper) = vec.pop() {
                    dropper.drop()
                }
                dealloc(nnptr)
            }
        } else {
            let mut nnptr = match maybe_sub {
                Some(nnptr) => nnptr,
                None => alloc(JunkYard::new()),
            };

            nnptr.as_mut().push(Box::new(dropper));
            self.reinsert(nnptr)
        }
    }

    fn take_pool(&self) -> Option<NonNull<JunkYard<'drop>>> {
        NonNull::new(self.pool.swap(null_mut(), AcqRel))
    }

    unsafe fn reinsert(&self, mut nnptr: NonNull<JunkYard<'drop>>) {
        while !self
            .pool
            .compare_and_swap(null_mut(), nnptr.as_ptr(), Release)
            .is_null()
        {
            let in_place = self.pool.swap(null_mut(), AcqRel);
            let mut in_place = match NonNull::new(in_place) {
                Some(nnptr) => nnptr,
                None => continue,
            };
            nnptr.as_mut().append(in_place.as_mut());
            dealloc(in_place);
        }
    }
}

impl<'drop> Drop for Incinerator<'drop> {
    fn drop(&mut self) {
        let maybe_sub = self.take_pool();
        if let Some(nnptr) = maybe_sub {
            let vec = unsafe { &mut *nnptr.as_ptr() };
            while let Some(dropper) = vec.pop() {
                dropper.drop()
            }
            unsafe { dealloc(nnptr) }
        }
    }
}

/// A pause on the incinerator. When this type is dropped, the pause is ended.
/// The drop implementation of this type will also try to execute all
/// reclamation functions on incinerator.
#[derive(Debug)]
pub struct Pause<'incin, 'drop> {
    inc: &'incin Incinerator<'drop>,
}

impl<'incin, 'drop> Drop for Pause<'incin, 'drop> {
    fn drop(&mut self) {
        let maybe_sub = self.inc.take_pool();

        if self.inc.paused.fetch_sub(1, AcqRel) == 0 {
            if let Some(nnptr) = maybe_sub {
                let vec = unsafe { &mut *nnptr.as_ptr() };
                while let Some(dropper) = vec.pop() {
                    dropper.drop()
                }

                unsafe { dealloc(nnptr) }
            }
        } else {
            if let Some(nnptr) = maybe_sub {
                unsafe { self.inc.reinsert(nnptr) }
            }
        }
    }
}

trait Dropper {
    fn drop(self: Box<Self>);
}

impl<F> Dropper for F
where
    F: FnOnce(),
{
    fn drop(self: Box<Self>) {
        (*self)()
    }
}
