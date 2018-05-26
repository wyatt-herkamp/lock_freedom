use std::{
    alloc::{oom as alloc_oom, Alloc, AllocErr, Global},
    ptr::{null_mut, read, write, NonNull},
    sync::atomic::{AtomicPtr, Ordering::*},
};

fn oom<T>(e: AllocErr) -> T {
    eprintln!("{}", e);
    alloc_oom()
}

pub struct Queue<T> {
    sub: AtomicPtr<SubQueue<T>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            sub: AtomicPtr::new(null_mut()),
        }
    }

    pub fn push(&self, val: T) {
        let mut sub = self.sub.swap(null_mut(), SeqCst);
        if sub.is_null() {
            sub = Global.alloc_one().unwrap_or_else(oom).as_ptr();
            unsafe {
                write(
                    sub,
                    SubQueue {
                        front: null_mut(),
                        back: null_mut(),
                    },
                );
            }
        }
        unsafe {
            (*sub).push(val);
            self.reinsert(sub);
        }
    }

    pub fn pop(&self) -> Option<T> {
        let sub = self.sub.swap(null_mut(), SeqCst);
        if sub.is_null() {
            return None;
        }
        unsafe {
            let res = (*sub).pop();
            self.reinsert(sub);
            res
        }
    }

    unsafe fn reinsert(&self, sub: *mut SubQueue<T>) {
        loop {
            if self.sub.compare_and_swap(null_mut(), sub, SeqCst).is_null() {
                break;
            }
            let other = self.sub.swap(null_mut(), SeqCst);
            if other.is_null() {
                continue;
            }
            (*sub).join(read(other));
            Global.dealloc_one(NonNull::new_unchecked(other));
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        let sub = self.sub.load(SeqCst);
        if !sub.is_null() {
            unsafe {
                while let Some(_) = (*sub).pop() {}
                Global.dealloc_one(NonNull::new_unchecked(sub));
            }
        }
    }
}

struct SubQueue<T> {
    front: *mut Node<T>,
    back: *mut Node<T>,
}

impl<T> SubQueue<T> {
    fn join(&mut self, other: Self) {
        if self.back.is_null() {
            debug_assert!(self.front.is_null());
            *self = other;
        } else if other.back.is_null() {
            debug_assert!(other.front.is_null());
        } else {
            debug_assert!(unsafe { (*self.back).next.is_null() });
            unsafe {
                (*self.back).next = other.front;
                self.back = other.back;
            }
        }
    }

    fn push(&mut self, val: T) {
        let node = Global.alloc_one().unwrap_or_else(oom).as_ptr();
        unsafe {
            write(
                node,
                Node {
                    val,
                    next: null_mut(),
                },
            );
        }
        if self.back.is_null() {
            debug_assert!(self.front.is_null());
            self.back = node;
            self.front = node;
        } else {
            unsafe { (*self.back).next = node };
            self.back = node;
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.front.is_null() {
            return None;
        }
        let front = self.front;
        let val = unsafe { read(&mut (*front).val as *mut _) };
        self.front = unsafe { (*front).next };
        if self.front.is_null() {
            debug_assert!(self.back == front);
            self.back = self.front;
        }
        unsafe {
            Global.dealloc_one(NonNull::new_unchecked(front));
        }
        Some(val)
    }
}

struct Node<T> {
    val: T,
    next: *mut Node<T>,
}
