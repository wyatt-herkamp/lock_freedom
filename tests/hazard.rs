extern crate lockfree;

use lockfree::prelude::*;
use std::{sync::Arc, thread};

// Run with sanitizer.
#[test]
fn hazard() {
    fn alloc_boxed<T>(val: T) -> *mut T {
        let ptr = Box::into_raw(Box::new(val));
        ptr
    }

    fn drop_boxed<T>(ptr: *mut T) {
        unsafe {
            Box::from_raw(ptr);
        }
    }

    let haz = Arc::new(HazardPtr::new(drop_boxed, alloc_boxed(0u64)));

    let mut threads = Vec::with_capacity(16);

    for i in 0..16 {
        let haz = haz.clone();

        threads.push(thread::spawn(move || {
            for j in 0..256 {
                loop {
                    let (ptr, val) = haz.load(SeqCst, |p| unsafe { (p, *p) });
                    let new = alloc_boxed(val.wrapping_add((i + 1) * (j + 1)));
                    let res = haz.compare_and_swap(ptr, new, SeqCst, {
                        let haz = haz.clone();
                        move |res| unsafe {
                            if res == ptr {
                                haz.apply_dropper(res);
                            } else {
                                haz.apply_dropper(new);
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
}
