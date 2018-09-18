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
            unsafe { Node::new_ptr(alloc(val).as_ptr(), null_mut()).as_ptr() };
        // Very simple schema: let's replace the back with our node, and then...
        incinerator::pause(|| {
            let ptr = self.back.swap(node, AcqRel);
            if let Some(back) = unsafe { ptr.as_ref() } {
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
            let result = incinerator::pause(|| unsafe {
                // First, let's load the current pointer.
                let ptr = self.front.load(Acquire);
                // Then, if it is null, the queue never ever had an element.
                let nnptr = match NonNull::new(ptr) {
                    Some(nnptr) => nnptr,
                    None => return Some(null_mut()),
                };

                // We are really interested in this pointer
                let item_ptr = nnptr.as_ref().val.load(Acquire);

                // If it is null, this item already was removed. We need to
                // clean it.
                if item_ptr.is_null() {
                    return if self.clean_front_first(nnptr) {
                        None
                    } else {
                        Some(null_mut())
                    };
                }

                // To remove, we simply set the item to null.
                let res = nnptr.as_ref().val.compare_and_swap(
                    item_ptr,
                    null_mut(),
                    Release,
                );

                if res == item_ptr {
                    // let's be polite and clean it up anyway.
                    self.clean_front_first(nnptr);
                    Some(item_ptr)
                } else {
                    None
                }
            });

            if let Some(ptr) = result {
                break NonNull::new(ptr).map(|nnptr| {
                    // Also, we have to take out the value.
                    let val = unsafe { nnptr.as_ptr().read() };
                    unsafe {
                        // Now it is OK to dealloc. If someone loaded the
                        // pointer, the thread will also block effectively
                        // memory reclamation.
                        incinerator::add(nnptr, dealloc_moved)
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

    unsafe fn clean_front_first(&self, expected: NonNull<Node<T>>) -> bool {
        let next = (*expected.as_ptr()).next.load(Acquire);
        if next.is_null() {
            false
        } else {
            let res =
                self.front.compare_and_swap(expected.as_ptr(), next, Release);
            if res == expected.as_ptr() {
                incinerator::add(NonNull::new_unchecked(res), dealloc);
            }
            true
        }
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
        if let Some(nnptr) = NonNull::new(self.front.load(Acquire)) {
            unsafe { dealloc(nnptr) }
        }
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
    val: AtomicPtr<T>,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    unsafe fn new_ptr(val: *mut T, next: *mut Self) -> NonNull<Self> {
        alloc(Self { val: AtomicPtr::new(val), next: AtomicPtr::new(next) })
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
