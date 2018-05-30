extern crate lockfree;

use lockfree::hazard::{later_drop, HazardPtr, Ordering::*};
use std::{ptr::NonNull, sync::Arc, thread};

// Run with sanitizer.
#[test]
fn hazard() {
    fn alloc_boxed<T>(val: T) -> *mut T {
        let ptr = Box::into_raw(Box::new(val));
        ptr
    }

    fn drop_boxed<T>(ptr: NonNull<T>) {
        unsafe {
            Box::from_raw(ptr.as_ptr());
        }
    }

    let haz = Arc::new(HazardPtr::new(alloc_boxed(0u64)));

    let mut threads = Vec::with_capacity(16);

    for i in 0..16 {
        let haz = haz.clone();

        threads.push(thread::spawn(move || {
            for j in 0..256 {
                loop {
                    let (ptr, val) = haz.load(SeqCst, |p| unsafe { (p, *p) });
                    let new = alloc_boxed(val.wrapping_add((i + 1) * (j + 1)));
                    let res = haz.compare_and_swap(ptr, new, SeqCst, {
                        |res| unsafe {
                            if res == ptr {
                                later_drop(
                                    NonNull::new_unchecked(res),
                                    drop_boxed,
                                );
                            } else {
                                later_drop(
                                    NonNull::new_unchecked(new),
                                    drop_boxed,
                                );
                            }
                            res == ptr
                        }
                    });
                    if res {
                        break;
                    }
                }
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    unsafe {
        later_drop(haz.load(SeqCst, |x| NonNull::new(x).unwrap()), drop_boxed);
    }
}
