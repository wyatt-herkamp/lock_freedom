use alloc::*;
use incinerator;
use std::{
    iter::FromIterator,
    ptr::{null_mut, read, NonNull},
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
        let node = Node::new_ptr(val, AtomicPtr::new(null_mut())).as_ptr();
        // Very simple schema: let's replace the back with our node, and then...
        incinerator::pause(|| {
            let ptr = self.back.swap(node, SeqCst);
            if let Some(back) = unsafe { ptr.as_mut() } {
                // ...put our node as the "next" of the previous back, if it
                // was not null...
                let _next = back.next.swap(node, SeqCst);
                debug_assert!(_next.is_null());
            } else {
                // ...otherwise, if it was null, front will also be null. We
                // need to update front.
                self.front.compare_and_swap(null_mut(), node, SeqCst);
            }
        })
    }

    /// Takes a value from the front of the queue, if it is avaible.
    pub fn pop(&self) -> Option<T> {
        loop {
            let result = incinerator::pause(|| {
                // load "ptr"
                let ptr = self.front.load(SeqCst);
                if ptr.is_null() {
                    // If front is null, then the queue is empty (for now).
                    // We're done with no elements.
                    Some(None)
                } else {
                    let next = unsafe { (*ptr).next.load(SeqCst) };
                    let res = self.front.compare_and_swap(ptr, next, SeqCst);
                    if res == ptr {
                        // If the loaded pointer "ptr" still was the
                        // front, we have an element and we're done.
                        Some(Some((ptr, next)))
                    } else {
                        // Otherwise, we are not done. Let's try again.
                        None
                    }
                }
            });

            if let Some(maybe_ptr) = result {
                break maybe_ptr.map(|(ptr, next)| {
                    // Critical! We have to first replace the back's pointer
                    // before deallocating our freshly front-removed pointer.
                    // Of course, we only need to replace if back and front
                    // were the same.
                    self.back.compare_and_swap(ptr, null_mut(), SeqCst);

                    // The back might have pushed a new value before we
                    // swaped int the code above.
                    // So, let's check if we don't need to
                    // update the front.
                    // This will only be needed if the stored "next" was null
                    // AND if we reload the next we get
                    // null.
                    unsafe {
                        if next.is_null() {
                            incinerator::pause(|| {
                                let next = (*ptr).next.load(Acquire);
                                if !next.is_null() {
                                    self.front.compare_and_swap(
                                        null_mut(),
                                        next,
                                        SeqCst,
                                    );
                                }
                            });
                        }
                    };

                    // Also, we have to take out the value.
                    let val = unsafe { read(&mut (*ptr).val as *mut _) };
                    unsafe {
                        // Now it is OK to dealloc. If someone loaded the
                        // pointer, the thread will also block effectively
                        // memory reclamation.
                        incinerator::add(NonNull::new_unchecked(ptr), Node::drop_ptr)
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
        Iter {
            queue: self,
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

unsafe impl<T> Sync for Queue<T> where T: Sync + Send {}

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
    fn new_ptr(val: T, next: AtomicPtr<Node<T>>) -> NonNull<Node<T>> {
        unsafe {
            alloc(Node {
                val,
                next,
            })
        }
    }

    fn drop_ptr(ptr: NonNull<Node<T>>) {
        unsafe {
            dealloc_moved(ptr);
        }
    }
}
