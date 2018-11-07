use incin::Incinerator;
use owned_alloc::OwnedAlloc;
use std::{
    fmt,
    iter::FromIterator,
    mem::{uninitialized, ManuallyDrop},
    ptr::{null_mut, NonNull},
    sync::atomic::{AtomicBool, AtomicPtr, Ordering::*},
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
        let sentinel = OwnedAlloc::new(Node::empty()).into_raw().as_ptr();
        Self {
            front: AtomicPtr::new(sentinel),
            back: AtomicPtr::new(sentinel),
            incin: Incinerator::new(),
        }
    }

    /// Pushes a value into the back of the queue. This operation is also
    /// wait-free.
    pub fn push(&self, val: T) {
        let node = Node::new(val);
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
            let res = self.incin.pause_with(|| {
                let front_ptr = self.front.load(Acquire);
                let mut front_nnptr = match NonNull::new(front_ptr) {
                    Some(nnptr) => nnptr,
                    None => return Some(None),
                };

                let prev_removed =
                    unsafe { front_nnptr.as_ref() }.empty.swap(true, AcqRel);

                if !prev_removed {
                    unsafe {
                        let ptr = &mut *front_nnptr.as_mut().val as *mut T;
                        let val = ptr.read();
                        self.try_clear_first(front_nnptr);
                        Some(Some(val))
                    }
                } else if unsafe { self.try_clear_first(front_nnptr) } {
                    None
                } else {
                    Some(None)
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

    /// Creates an iterator over `T`s, based on `pop` operation of the queue.
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { queue: self }
    }

    unsafe fn try_clear_first(&self, expected: NonNull<Node<T>>) -> bool {
        let next = expected.as_ref().next.load(Acquire);

        if next.is_null() {
            false
        } else {
            let ptr = expected.as_ptr();
            let res = self.front.compare_and_swap(ptr, next, Release);
            if res == expected.as_ptr() {
                self.incin.add(OwnedAlloc::from_raw(expected));
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

impl<'a, T> IntoIterator for &'a Queue<T> {
    type Item = T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> fmt::Debug for Queue<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "Queue {} front: {:?}, back: {:?} {}",
            '{', self.front, self.back, '}'
        )
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

#[repr(align(/* at least */ 2))]
struct Node<T> {
    val: ManuallyDrop<T>,
    empty: AtomicBool,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            val: ManuallyDrop::new(val),
            empty: AtomicBool::new(false),
            next: AtomicPtr::new(null_mut()),
        }
    }

    fn empty() -> Self {
        Self {
            val: ManuallyDrop::new(unsafe { uninitialized() }),
            empty: AtomicBool::new(true),
            next: AtomicPtr::new(null_mut()),
        }
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        if !self.empty.load(Acquire) {
            unsafe { ManuallyDrop::drop(&mut self.val) }
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
