use alloc::*;
use incinerator;
use std::{
    iter::FromIterator,
    ptr::{null_mut, NonNull},
    sync::atomic::{AtomicPtr, Ordering::*},
};

/// A lock-free queue. FIFO semanthics are fully respected.
/// It can be used as multi-producer and multi-consumer channel.
#[derive(Debug)]
pub struct Queue<T> {
    front: AtomicPtr<Node<T>>,
    back: AtomicPtr<Node<T>>,
}

impl<T> Queue<T> {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        Self {
            front: AtomicPtr::new(null_mut()),
            back: AtomicPtr::new(null_mut()),
        }
    }

    /// Pushes a value into the back of the queue. This operation is also
    /// wait-free.
    pub fn push(&self, val: T) {
        let node =
            unsafe { Node::new_ptr(val, AtomicPtr::new(null_mut())).as_ptr() };
        // Very simple schema: let's replace the back with our node, and then...
        incinerator::pause(|| {
            let ptr = self.back.swap(node, AcqRel);
            if let Some(back) = unsafe { ptr.as_mut() } {
                // ...put our node as the "next" of the previous back, if it
                // was not null...
                let _next = back.next.swap(node, Release);
                debug_assert!(_next.is_null());
            } else {
                // ...otherwise, if it was null, front will also be null. We
                // need to update front.
                self.front.compare_and_swap(null_mut(), node, Release);
            }
        })
    }

    /// Takes a value from the front of the queue, if it is avaible.
    pub fn pop(&self) -> Option<T> {
        loop {
            let result = incinerator::pause(|| {
                // load "ptr"
                let ptr = self.front.load(Acquire);
                if ptr.is_null() {
                    // If front is null, then the queue is empty (for now).
                    // We're done with no elements.
                    return Some(ptr);
                }

                let next = unsafe { (*ptr).next.load(Acquire) };
                let res = self.front.compare_and_swap(ptr, next, Release);

                if res != ptr {
                    return None;
                }

                // If the loaded pointer "ptr" still was the
                // front, we have an element and we're done.

                // Critical! We have to first replace the back's pointer
                // before deallocating our freshly front-removed
                // pointer. Of course, we only
                // need to replace if back and front
                // were the same.
                self.back.compare_and_swap(ptr, null_mut(), Release);

                // The back might have pushed a new value before we
                // swaped int the code above.
                // So, let's check if we don't need to
                // update the front.
                // This will only be needed if the stored "next" was
                // null AND if we reload the
                // next we get null.
                if next.is_null() {
                    let next = unsafe { (*ptr).next.load(Acquire) };
                    self.front.compare_and_swap(null_mut(), next, Release);
                }

                Some(ptr)
            });

            if let Some(ptr) = result {
                break NonNull::new(ptr).map(|nnptr| {
                    // Also, we have to take out the value.
                    let val =
                        unsafe { (&nnptr.as_ref().val as *const T).read() };
                    unsafe {
                        // Now it is OK to dealloc. If someone loaded the
                        // pointer, the thread will also block effectively
                        // memory reclamation.
                        incinerator::add(nnptr, Node::drop_ptr)
                    }
                    val
                });
            }
        }
    }

    /// Extends the queue from a given iterable.
    pub fn extend<I>(&self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        for elem in iterable {
            self.push(elem);
        }
    }

    /// Creates an iterator over `T`s, based on `pop` operation of the queue.
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { queue: self }
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T> FromIterator<T> for Queue<T> {
    fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let this = Self::new();
        this.extend(iterable);
        this
    }
}

impl<'a, T> IntoIterator for &'a Queue<T> {
    type Item = T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

unsafe impl<T> Send for Queue<T> where T: Send {}

unsafe impl<T> Sync for Queue<T> where T: Send {}

/// An iterator based on `pop` operation of the `Queue`.
pub struct Iter<'a, T>
where
    T: 'a,
{
    queue: &'a Queue<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop()
    }
}

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    unsafe fn new_ptr(val: T, next: AtomicPtr<Self>) -> NonNull<Self> {
        alloc(Self { val, next })
    }

    unsafe fn drop_ptr(ptr: NonNull<Self>) {
        dealloc_moved(ptr);
    }
}

// Testing the safety of `unsafe` in this module is done with random operations
// via fuzzing
#[cfg(test)]
mod test {
    use super::*;
    use std::{sync::Arc, thread};

    #[test]
    fn on_empty_first_pop_is_none() {
        let queue = Queue::<usize>::new();
        assert!(queue.pop().is_none());
    }

    #[test]
    fn on_empty_last_pop_is_none() {
        let queue = Queue::new();
        queue.push(3);
        queue.push(1234);
        queue.pop();
        queue.pop();
        assert!(queue.pop().is_none());
    }

    #[test]
    fn order() {
        let queue = Queue::new();
        queue.push(3);
        queue.push(5);
        queue.push(6);
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(5));
        assert_eq!(queue.pop(), Some(6));
    }

    #[test]
    fn no_data_corruption() {
        const NTHREAD: usize = 20;
        const NITER: usize = 800;
        const NMOD: usize = 55;

        let queue = Arc::new(Queue::new());
        let mut handles = Vec::with_capacity(NTHREAD);

        for i in 0 .. NTHREAD {
            let queue = queue.clone();
            handles.push(thread::spawn(move || {
                for j in 0 .. NITER {
                    let val = (i * NITER) + j;
                    queue.push(val);
                    if (val + 1) % NMOD == 0 {
                        if let Some(val) = queue.pop() {
                            assert!(val < NITER * NTHREAD);
                        }
                    }
                }
            }));
        }

        for handle in handles {
            handle.join().expect("thread failed");
        }

        let expected = NITER * NTHREAD - NITER * NTHREAD / NMOD;
        let mut res = 0;
        while let Some(val) = queue.pop() {
            assert!(val < NITER * NTHREAD);
            res += 1;
        }

        assert_eq!(res, expected);
    }
}
