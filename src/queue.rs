use incin::Incinerator;
use owned_alloc::OwnedAlloc;
use std::{
    fmt,
    iter::FromIterator,
    mem::ManuallyDrop,
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
        self.incin.pause_with(|| {
            let ptr = self.back.swap(node_ptr, AcqRel);
            if let Some(back) = unsafe { ptr.as_ref() } {
                let res =
                    back.next.compare_and_swap(null_mut(), node_ptr, Release);
                if !res.is_null() {
                    self.front.store(node_ptr, Release);
                }
            } else {
                self.front.store(node_ptr, Release);
            }
        })
    }

    /// Takes a value from the front of the queue, if it is avaible.
    pub fn pop(&self) -> Option<T> {
        loop {
            let res = self.incin.pause_with(|| {
                let front_ptr = self.front.load(Acquire);
                let front_nnptr = match NonNull::new(front_ptr) {
                    Some(nnptr) => nnptr,
                    None => return Some(None),
                };

                let next_ptr =
                    unsafe { front_nnptr.as_ref() }.next.load(Acquire) as usize;

                if next_ptr & 1 == 0 {
                    let res =
                        unsafe { front_nnptr.as_ref() }.next.compare_and_swap(
                            next_ptr as *mut _,
                            (next_ptr | 1) as *mut _,
                            Release,
                        );

                    if res == next_ptr as *mut _ {
                        self.clear_first(front_ptr, next_ptr as *mut _);
                        Some(Some(front_nnptr))
                    } else {
                        None
                    }
                } else {
                    self.clear_first(front_ptr, (next_ptr & !1) as *mut _);
                    None
                }
            });

            if let Some(maybe_nnptr) = res {
                break maybe_nnptr.map(|mut nnptr| unsafe {
                    let val = (&mut *nnptr.as_mut().val as *mut T).read();
                    self.incin.add(OwnedAlloc::from_raw(nnptr));
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

    fn clear_first(&self, expected: *mut Node<T>, desired: *mut Node<T>) {
        self.front.compare_and_swap(expected, desired, Release);
        self.back.compare_and_swap(expected, desired, Release);
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
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            val: ManuallyDrop::new(val),
            next: AtomicPtr::new(null_mut()),
        }
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        if self.next.load(Acquire) as usize & 1 == 0 {
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
