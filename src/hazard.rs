pub use std::sync::atomic::Ordering::{self, *};
use std::{
    cell::RefCell,
    collections::VecDeque,
    mem::transmute,
    sync::atomic::{AtomicPtr, AtomicUsize},
};

/// A hazard atomic pointer. It keeps a destructor with itself.
/// The destruction and loading of this pointer uses the hazard API of this
/// module. Because it (possibly later) calls the destructor on drop, be very
/// careful. In general, if the pointer is consumed, you may want to fill the
/// hazard pointer with `std::ptr::null_mut()` and check it on your drop.
#[derive(Debug)]
pub struct HazardPtr<T> {
    dropper: fn(*mut T),
    ptr: AtomicPtr<T>,
}

impl<T> HazardPtr<T> {
    /// Creates a new pointer from the given dropper and initial ptr.
    /// The dropper is run at the `HazardPtr<T>`'s `Drop`. Also, it may
    /// be used for dropping intermediate pointers explicitly.
    pub const fn new(dropper: fn(*mut T), ptr: *mut T) -> Self {
        Self {
            dropper,
            ptr: AtomicPtr::new(ptr),
        }
    }

    /// Loads the pointer atomically as critical code.
    pub fn load<F, A>(&self, ord: Ordering, exec: F) -> A
    where
        F: FnOnce(*mut T) -> A,
    {
        critical(|| exec(self.ptr.load(ord)))
    }

    /// Stores the pointer atomically.
    pub fn store(&self, ptr: *mut T, ord: Ordering) {
        self.ptr.store(ptr, ord);
    }

    /// Swaps the pointer atomically as critical code.
    pub fn swap<F, A>(&self, ptr: *mut T, ord: Ordering, exec: F) -> A
    where
        F: FnOnce(*mut T) -> A,
    {
        critical(|| exec(self.ptr.swap(ptr, ord)))
    }

    /// Compares the given `curr` argument with the actual stored pointer,
    /// and, if the comparison succeeds, swaps it with the given `new` pointer.
    /// The whole operation is atomic and is run as critical code.
    pub fn compare_and_swap<F, A>(
        &self,
        curr: *mut T,
        new: *mut T,
        ord: Ordering,
        exec: F,
    ) -> A
    where
        F: FnOnce(*mut T) -> A,
    {
        critical(|| exec(self.ptr.compare_and_swap(curr, new, ord)))
    }

    /// Compares the given `curr` argument with the actual stored pointer,
    /// and, if the comparison succeeds, swaps it with the given `new` pointer.
    /// The whole operation is atomic and is run as critical code. This method
    /// accepts two orderings: one for success and one for failure.
    pub fn compare_exchange<F, A>(
        &self,
        curr: *mut T,
        new: *mut T,
        succ_ord: Ordering,
        fail_ord: Ordering,
        exec: F,
    ) -> A
    where
        F: FnOnce(Result<*mut T, *mut T>) -> A,
    {
        critical(|| {
            exec(self.ptr.compare_exchange(curr, new, succ_ord, fail_ord))
        })
    }

    /// Same as `compare_exchange`, but with weaker semanthics (it might
    /// perform better).
    pub fn compare_exchange_weak<F, A>(
        &self,
        curr: *mut T,
        new: *mut T,
        succ_ord: Ordering,
        fail_ord: Ordering,
        exec: F,
    ) -> A
    where
        F: FnOnce(Result<*mut T, *mut T>) -> A,
    {
        critical(|| {
            exec(self.ptr.compare_exchange_weak(curr, new, succ_ord, fail_ord))
        })
    }

    /// Applies the dropper to some pointer. This function is unsafe because
    /// incorrectly applying the destructor may result in "use after free" or
    /// "double free".
    pub unsafe fn apply_dropper(&self, ptr: *mut T) {
        later_drop(ptr, self.dropper)
    }
}

impl<T> Drop for HazardPtr<T> {
    fn drop(&mut self) {
        unsafe { later_drop(self.ptr.load(Relaxed), self.dropper) }
    }
}

/// Adds the given pointer and drop function to the local deletion queue.
/// If there is no critical code executing, the local queue items are deleted.
/// The function is unsafe because pointers must be correctly dropped such as
/// no "use after free" or "double free" happens.
pub unsafe fn later_drop<T>(ptr: *mut T, dropper: fn(*mut T)) {
    LOCAL_DELETION.with(|queue| {
        // First of all, let's put it on the queue because of a possible
        // obstruction when deleting.
        queue.add(Garbage {
            ptr: ptr as *mut u8,
            dropper: transmute(dropper),
        });
        if DELETION_STATUS.load(SeqCst) == 0 {
            // Please, note that we check for the status AFTER the enqueueing.
            // This ensures that no pointer is added after a possible status
            // change. All pointers deleted here were already added
            // to the queue.
            queue.delete();
        }
    })
}

/// Tries to delete the local queue items.
pub fn try_delete_local() -> Result<(), ()> {
    LOCAL_DELETION.with(|queue| {
        if DELETION_STATUS.load(SeqCst) == 0 {
            // No problem to change the status while deleting.
            // No pointer is added to the queue during the change.
            queue.delete();
            Ok(())
        } else {
            Err(())
        }
    })
}

/// Executes the given function as critical code.
/// No deletions of new queues will start during this execution.
pub fn critical<F, T>(exec: F) -> T
where
    F: FnOnce() -> T,
{
    // Do not allow deletions, but allow adding pointers to the local queues.
    DELETION_STATUS.fetch_add(1, SeqCst);
    let res = exec();
    // After the execution, everything is fine.
    DELETION_STATUS.fetch_sub(1, SeqCst);
    res
}

struct Garbage {
    ptr: *mut u8,
    dropper: fn(*mut u8),
}

struct GarbageQueue {
    inner: RefCell<VecDeque<Garbage>>,
}

impl GarbageQueue {
    fn new() -> Self {
        Self {
            inner: RefCell::new(VecDeque::with_capacity(16)),
        }
    }

    fn add(&self, garbage: Garbage) {
        self.inner.borrow_mut().push_back(garbage);
    }

    fn delete(&self) {
        let mut deque = self.inner.borrow_mut();
        while let Some(garbage) = deque.pop_front() {
            (garbage.dropper)(garbage.ptr);
        }
    }
}

impl Drop for GarbageQueue {
    fn drop(&mut self) {
        while DELETION_STATUS.load(SeqCst) != 0 {}
        self.delete();
    }
}

thread_local! {
    static LOCAL_DELETION: GarbageQueue = GarbageQueue::new();
}

static DELETION_STATUS: AtomicUsize = AtomicUsize::new(0);
