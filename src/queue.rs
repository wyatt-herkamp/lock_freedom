use incinerator::Incinerator;
use owned_alloc::OwnedAlloc;
use std::{
    fmt,
    iter::FromIterator,
    ptr::{null_mut, NonNull},
    sync::atomic::{AtomicPtr, Ordering::*},
};

/// A lock-free queue. FIFO semanthics are fully respected.
/// It can be used as multi-producer and multi-consumer channel.
pub struct Queue<T> {
    front: AtomicPtr<Node<T>>,
    back: AtomicPtr<Node<T>>,
    incin: Incinerator<OwnedAlloc<Node<T>>>,
}

impl<T> Queue<T> {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        Self {
            front: AtomicPtr::new(null_mut()),
            back: AtomicPtr::new(null_mut()),
            incin: Incinerator::new(),
        }
    }

    /// Pushes a value into the back of the queue. This operation is also
    /// wait-free.
    pub fn push(&self, val: T) {
        let node = Node::new(val);
        let alloc = OwnedAlloc::new(node);
        let node_ptr = alloc.into_raw().as_ptr();
        // Very simple schema: let's replace the back with our node, and then...
        self.incin.pause_with(|| {
            let ptr = self.back.swap(node_ptr, AcqRel);
            if let Some(back) = unsafe { ptr.as_ref() } {
                // ...put our node as the "next" of the previous back, if it
                // was not null...
                let _next = back.next.swap(node_ptr, Release);
                debug_assert!(_next.is_null());
            } else {
                // ...otherwise, if it was null, front will also be null. We
                // need to update front.
                self.front.compare_and_swap(null_mut(), node_ptr, Release);
            }
        })
    }

    /// Takes a value from the front of the queue, if it is avaible.
    pub fn pop(&self) -> Option<T> {
        loop {
            let PopIterRes { item, node } = self.incin.pause_with(|| {
                // First, let's load the current pointer.
                let ptr = self.front.load(Acquire);
                // Then, if it is null, the queue never ever had an element.
                let nnptr = match NonNull::new(ptr) {
                    Some(nnptr) => nnptr,
                    None => return PopIterRes { item: Err(true), node: None },
                };

                // We are really interested in this pointer
                let item_ptr = unsafe { nnptr.as_ref() }.val.load(Acquire);

                // If it is null, this item already was removed. We need to
                // clean it.
                let item_nnptr = match NonNull::new(item_ptr) {
                    Some(nnptr) => nnptr,
                    _ => return unsafe { self.clean_front_first(nnptr) },
                };

                // To remove, we simply set the item to null.
                let res = unsafe { nnptr.as_ref() }.val.compare_and_swap(
                    item_ptr,
                    null_mut(),
                    Release,
                );

                if res == item_ptr {
                    PopIterRes {
                        item: Ok(unsafe { OwnedAlloc::from_raw(item_nnptr) }),
                        // let's be polite and clean it up anyway.
                        node: unsafe { self.clean_front_first(nnptr) }.node,
                    }
                } else {
                    PopIterRes { item: Err(false), node: None }
                }
            });

            if let Some(node) = node {
                self.incin.add(node);
            }

            match item {
                Ok(alloc) => {
                    break Some({
                        let (item, _) = alloc.move_inner();
                        item
                    })
                },

                Err(true) => break None,

                Err(false) => (),
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

    unsafe fn clean_front_first(
        &self,
        expected: NonNull<Node<T>>,
    ) -> PopIterRes<T> {
        let next = expected.as_ref().next.load(Acquire);
        if next.is_null() {
            PopIterRes { item: Err(true), node: None }
        } else {
            let res =
                self.front.compare_and_swap(expected.as_ptr(), next, Release);

            PopIterRes {
                item: Err(false),
                node: if res == expected.as_ptr() {
                    Some(OwnedAlloc::from_raw(expected))
                } else {
                    None
                },
            }
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
            unsafe {
                OwnedAlloc::from_raw(nnptr);
            }
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

impl<T> fmt::Debug for Queue<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str("Queue")
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
struct PopIterRes<T> {
    item: Result<OwnedAlloc<T>, bool>,
    node: Option<OwnedAlloc<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    val: AtomicPtr<T>,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            val: AtomicPtr::new(OwnedAlloc::new(val).into_raw().as_ptr()),
            next: AtomicPtr::new(null_mut()),
        }
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
