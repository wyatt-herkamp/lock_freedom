use std::{
    cell::UnsafeCell,
    mem::{replace, transmute},
    process::abort,
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering::*},
};

static PAUSED_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Adds the given pointer and drop function to the local deletion list.
/// If there is no critical code executing (i.e. the incinerator is not
/// paused), all local list items are deleted. This function is unsafe because
/// pointers must be correctly dropped such as no "use after free" or "double
/// free" happens. You may want to call this function only after you replaced
/// the pointer from the shared context (or there aren't active threads).
pub unsafe fn add<T>(ptr: NonNull<T>, dropper: unsafe fn(NonNull<T>)) {
    LOCAL_DELETION.with(|list| {
        if PAUSED_COUNT.load(Acquire) == 0 {
            // Please, note that we check for the counter AFTER the given
            // pointer was removed from the shared context. Also, we are the
            // only ones that can add something to the list. No pointer is
            // added to the list after the check, since the list is
            // thread-local.
            dropper(ptr);
            list.delete();
        } else {
            // If we cannot delete the pointer, we have to add it to the list.
            list.add(Garbage { ptr: ptr.cast(), dropper: transmute(dropper) });
        }
    })
}

/// Tries to force deletion of all local queue items. Only succeeds
/// if there are no pauses when checking for them before the deletion.
/// Returns true in case of success, false otherwise. Please note this
/// functions is not strictly need to be called, but it may help on releasing
/// garbage if you added a lot of them during a pause. These are some situations
/// in which `try_force` can be helpful:
/// 1. Your application exits from a concurrent context, and then you want to
/// clean    a possibly non-empty deletion queue for the main thread.
/// 2. Your application's threads might sleep for some time and you want to
/// clean    garbage up and free memory.
pub fn try_force() -> bool {
    LOCAL_DELETION.with(|queue| {
        let success = PAUSED_COUNT.load(Acquire) == 0;
        if success {
            // No problem to change the status while deleting.
            // No pointer is added to the queue during the change.
            queue.delete();
        }
        success
    })
}

/// Pauses the incinerator and executes the given function as critical code.
/// No deletions of new queues will start during the execution of the given
/// function. Inside the passed function is a good place to load and read
/// atomic pointers.
#[inline]
pub fn pause<F, T>(exec: F) -> T
where
    F: FnOnce() -> T,
{
    // Do not allow deletions, but allow adding pointers to the local queues.
    let paused = Pause::new();
    let res = exec();
    // After the execution, everything is fine.
    drop(paused);
    res
}

struct Pause;

impl Pause {
    fn new() -> Self {
        // prevent count from overflowing and creating bugs
        if PAUSED_COUNT.fetch_add(1, Acquire) == usize::max_value() {
            abort();
        }
        Pause
    }
}

impl Drop for Pause {
    fn drop(&mut self) {
        PAUSED_COUNT.fetch_sub(1, Release);
    }
}

struct Garbage {
    ptr: NonNull<u8>,
    dropper: unsafe fn(NonNull<u8>),
}

struct GarbageList {
    inner: UnsafeCell<Vec<Garbage>>,
}

impl GarbageList {
    fn new() -> Self {
        Self { inner: UnsafeCell::new(Vec::new()) }
    }

    fn add(&self, garbage: Garbage) {
        unsafe { &mut *self.inner.get() }.push(garbage);
    }

    fn delete(&self) {
        let mut list = replace(unsafe { &mut *self.inner.get() }, Vec::new());
        while let Some(garbage) = list.pop() {
            unsafe {
                (garbage.dropper)(garbage.ptr);
            }
        }
    }
}

impl Drop for GarbageList {
    fn drop(&mut self) {
        while PAUSED_COUNT.load(Acquire) != 0 {}
        self.delete();
    }
}

thread_local! {
    static LOCAL_DELETION: GarbageList = GarbageList::new();
}

// Testing the safety of `unsafe` in this module is done with random operations
// via fuzzing
#[cfg(test)]
mod test {
    use super::*;
    use alloc::*;
    use std::thread;

    #[test]
    fn try_force_succeeds_in_single_threaded() {
        assert!(try_force());

        const COUNT: usize = 16;

        let mut allocs = Vec::with_capacity(COUNT);

        for i in 0 .. COUNT {
            allocs.push(unsafe { alloc(i) });
        }

        pause(|| ());

        for ptr in allocs {
            unsafe {
                add(ptr, dealloc);
            }
        }

        assert!(try_force());
    }

    #[test]
    fn count_is_gt_0_when_pausing() {
        const NTHREADS: usize = 20;
        let mut threads = Vec::with_capacity(NTHREADS);
        for _ in 0 .. NTHREADS {
            threads.push(thread::spawn(|| {
                pause(|| {
                    assert!(PAUSED_COUNT.load(SeqCst) > 0);
                })
            }));
        }
        for thread in threads {
            thread.join().expect("sub-thread panicked");
        }
    }
}
