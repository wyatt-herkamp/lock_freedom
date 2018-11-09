use incin::Incinerator;
use owned_alloc::OwnedAlloc;
use removable::Removable;
use std::{
    fmt,
    iter::FromIterator,
    ptr::{null_mut, NonNull},
    sync::{
        atomic::{AtomicPtr, Ordering::*},
        Arc,
    },
};

/// A lock-free queue. FIFO semanthics are fully respected.
/// It can be used as multi-producer and multi-consumer channel.
pub struct Queue<T> {
    front: AtomicPtr<Node<T>>,
    back: AtomicPtr<Node<T>>,
    incin: SharedIncin<T>,
}

impl<T> Queue<T> {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        Self::with_incin(SharedIncin::new())
    }

    /// Creates an empty queue using the passed shared incinerator.
    pub fn with_incin(incin: SharedIncin<T>) -> Self {
        let node = Node::new(Removable::empty());
        let sentinel = OwnedAlloc::new(node).into_raw().as_ptr();
        Self {
            front: AtomicPtr::new(sentinel),
            back: AtomicPtr::new(sentinel),
            incin,
        }
    }

    /// Returns the shared incinerator used by this `Queue`.
    pub fn incin(&self) -> SharedIncin<T> {
        self.incin.clone()
    }

    /// Creates an iterator over `T`s, based on `pop` operation of the queue.
    pub fn pop_iter<'origin>(&'origin self) -> PopIter<'origin, T> {
        PopIter { queue: self }
    }

    /// Pushes a value into the back of the queue. This operation is also
    /// wait-free.
    pub fn push(&self, item: T) {
        let node = Node::new(Removable::new(item));
        let alloc = OwnedAlloc::new(node);
        let node_ptr = alloc.into_raw().as_ptr();
        let prev_back = self.back.swap(node_ptr, AcqRel);
        unsafe {
            (*prev_back).next.store(node_ptr, Release);
        }
    }

    /// Takes a value from the front of the queue, if it is avaible.
    pub fn pop(&self) -> Option<T> {
        loop {
            let res = self.incin.inner.pause_with(|| {
                let front_ptr = self.front.load(Acquire);
                let front_nnptr = match NonNull::new(front_ptr) {
                    Some(nnptr) => nnptr,
                    None => return Some(None),
                };

                match unsafe { front_nnptr.as_ref() }.item.take() {
                    Some(val) => {
                        unsafe { self.try_clear_first(front_nnptr) };
                        Some(Some(val))
                    },

                    None if unsafe { self.try_clear_first(front_nnptr) } => {
                        None
                    },

                    None => Some(None),
                }
            });

            if let Some(ret) = res {
                break ret;
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

    unsafe fn try_clear_first(&self, expected: NonNull<Node<T>>) -> bool {
        let next = expected.as_ref().next.load(Acquire);

        if next.is_null() {
            false
        } else {
            let ptr = expected.as_ptr();
            let res = self.front.compare_and_swap(ptr, next, Release);
            if res == expected.as_ptr() {
                self.incin.inner.add(OwnedAlloc::from_raw(expected));
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
        let mut node_ptr = self.front.load(Relaxed);
        while let Some(nnptr) = NonNull::new(node_ptr) {
            let node = unsafe { OwnedAlloc::from_raw(nnptr) };
            node_ptr = node.next.load(Relaxed);
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

impl<T> fmt::Debug for Queue<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "Queue {} front: {:?}, back: {:?}, incin: {:?} {}",
            '{', self.front, self.back, self.incin, '}'
        )
    }
}

unsafe impl<T> Send for Queue<T> where T: Send {}
unsafe impl<T> Sync for Queue<T> where T: Send {}

/// An iterator based on `pop` operation of the `Queue`.
pub struct PopIter<'origin, T>
where
    T: 'origin,
{
    queue: &'origin Queue<T>,
}

impl<'origin, T> Iterator for PopIter<'origin, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop()
    }
}

impl<'origin, T> fmt::Debug for PopIter<'origin, T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "PopIter {} queue: {:?} {}", '{', self.queue, '}')
    }
}

make_shared_incin! {
    { "`Queue`" }
    pub SharedIncin<T> of OwnedAlloc<Node<T>>
}

impl<T> fmt::Debug for SharedIncin<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "SharedIncin {} inner: {:?} {}", '{', self.inner, '}')
    }
}

#[repr(align(/* at least */ 2))]
struct Node<T> {
    item: Removable<T>,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new(item: Removable<T>) -> Self {
        Self {
            item,
            next: AtomicPtr::new(null_mut()),
        }
    }
}

// Testing the safety of `unsafe` in this module is done with random operations
// via fuzzing
#[cfg(test)]
mod test {
    use super::*;
    use std::{
        sync::{atomic::AtomicUsize, Arc},
        thread,
    };

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
        let removed = Arc::new(AtomicUsize::new(0));

        for i in 0 .. NTHREAD {
            let removed = removed.clone();
            let queue = queue.clone();
            handles.push(thread::spawn(move || {
                for j in 0 .. NITER {
                    let val = (i * NITER) + j;
                    queue.push(val);
                    if (val + 1) % NMOD == 0 {
                        if let Some(val) = queue.pop() {
                            removed.fetch_add(1, Relaxed);
                            assert!(val < NITER * NTHREAD);
                        }
                    }
                }
            }));
        }

        for handle in handles {
            handle.join().expect("thread failed");
        }

        let expected = NITER * NTHREAD - removed.load(Relaxed);
        let mut res = 0;
        while let Some(val) = queue.pop() {
            assert!(val < NITER * NTHREAD);
            res += 1;
        }

        assert_eq!(res, expected);
    }
}
