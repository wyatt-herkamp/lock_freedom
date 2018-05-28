use std::{
    alloc::{Alloc, Global},
    fmt,
    iter::FromIterator,
    ptr::{null_mut, read, write, NonNull},
    sync::atomic::{AtomicPtr, Ordering::*},
};

/// A lock-free concurrent queue, but without FIFO garantees on multithreaded
/// environments. Single thread environments still have FIFO garantees. The
/// queue is based on subqueues which threads try to take, modify and then
/// publish. If necessary, subqueues are appended.
/// # Example
/// ```rust
/// extern crate lockfree;
/// use lockfree::prelude::*;
/// use std::{sync::Arc, thread};
///
/// let queue = Arc::new(LooseQueue::new());
/// let mut producers = Vec::with_capacity(4);
/// for i in 0..3 {
///     let queue = queue.clone();
///     producers.push(thread::spawn(move || {
///         for j in i * 100..(i + 1) * 100 {
///             queue.push(j);
///             if j % 7 == 0 {
///                 if let Some(elem) = queue.pop() {
///                     queue.push(elem + 1);
///                 }
///             }
///         }
///     }));
/// }
/// let mut consumers = Vec::with_capacity(8);
/// for _ in 0..8 {
///     let queue = queue.clone();
///     consumers.push(thread::spawn(move || {
///         while let Some(x) = queue.pop() {
///             assert!(x < 800);
///         }
///     }));
/// }
/// for producer in producers {
///     producer.join().unwrap();
/// }
/// for consumer in consumers {
///     consumer.join().unwrap();
/// }
/// while let Some(x) = queue.pop() {
///     assert!(x < 800);
/// }
/// ```
pub struct LooseQueue<T> {
    sub: AtomicPtr<SubQueue<T>>,
}

impl<T> LooseQueue<T> {
    /// Creates a new empty queue.
    pub const fn new() -> Self {
        Self {
            sub: AtomicPtr::new(null_mut()),
        }
    }

    /// Pushes a value in the back of the queue.
    pub fn push(&self, val: T) {
        // We are taking this subqueue for ourselves.
        // We are also leaving a null pointer in its place.
        // When we reinsert the subqueue, we expect to find the null pointer.
        let mut sub = self.sub.swap(null_mut(), SeqCst);
        if sub.is_null() {
            // Somebody already took the subqueue? No problem! Let's create a
            // new one. Note that initially there is no subqueue.
            sub = SubQueue::alloc().as_ptr();
        }
        unsafe {
            // This is the part in which we really do the work! Push into the
            // subqueue.
            (*sub).push(val);
            // And then... reinsert the subqueue. This function does not expect
            // null.
            self.reinsert(sub);
        }
    }

    /// Pops a value from the front of the queue.
    pub fn pop(&self) -> Option<T> {
        // We are taking this subqueue for ourselves.
        // We are also leaving a null pointer in its place.
        // When we reinsert the subqueue, we expect to find the null pointer.
        let sub = self.sub.swap(null_mut(), SeqCst);
        if sub.is_null() {
            // No subqueue? Then we have nothing.
            return None;
        }
        unsafe {
            // We do the real work in here! Pop from the subqueue.
            let res = (*sub).pop();
            // Reinsert the popped subqueue. This function does not expect null.
            self.reinsert(sub);
            // Whatever we got; Some, None; return it!
            res
        }
    }

    /// Appends some other queue to the end of this one.
    pub fn append(&self, other: &Self) {
        // Let's take the other's subqueue and not give it back! :P
        let other = other.sub.swap(null_mut(), SeqCst);
        if other.is_null() {
            // Well.. If other's is null, we have nothing to do!
            return;
        }
        // Let's take our subqueue. Same schema here. We are placing a null,
        // and we expect to find a null when reinserting.
        let sub = self.sub.swap(null_mut(), SeqCst);
        if sub.is_null() {
            // If ours is null, we can "re"insert the other list.
            unsafe { self.reinsert(other) }
        } else {
            unsafe {
                // Otherwise let's append both subqueue.
                (*sub).append(read(other));
                // Dealloc the other's pointer.
                Global.dealloc_one(NonNull::new_unchecked(other));
                // And reinsert sub!
                self.reinsert(sub);
            }
        }
    }

    /// Extends the queue from an iterator.
    pub fn extend<I>(&self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        // Very simple: take our queue (same schema).
        let mut sub = self.sub.swap(null_mut(), SeqCst);
        // If there is no queue, allocate one!
        if sub.is_null() {
            sub = SubQueue::alloc().as_ptr();
        }
        unsafe {
            for item in iterable {
                // Do the job on the subqueue.
                (*sub).push(item);
            }
            // And then reinsert.
            self.reinsert(sub);
        }
    }

    /// Creates an inspector on the current subqueue. The inspector takes the
    /// subqueue for itself and restores it on drop.
    pub fn inspect<'a>(&'a self) -> Inspector<'a, T> {
        // Let's literally borrow the subqueue for a while.
        let sub = self.sub.swap(null_mut(), SeqCst);
        Inspector {
            queue: self,
            sub,
            curr: unsafe { sub.as_ref().map(|x| x.front) },
        }
    }

    /// Creates a drainer on the current subqueue. The drainer takes the
    /// subqueue for itself and restores what is left of it on drop.
    pub fn drain<'a>(&'a self) -> Drainer<'a, T> {
        // Let's literally borrow the subqueue for a while.
        let sub = self.sub.swap(null_mut(), SeqCst);
        Drainer {
            queue: self,
            sub,
        }
    }

    unsafe fn reinsert(&self, sub: *mut SubQueue<T>) {
        loop {
            // We expect null pointer. If it is there, we swap and done!
            if self.sub.compare_and_swap(null_mut(), sub, SeqCst).is_null() {
                break;
            }
            // Well, if it wasn't null, take it.
            let other = self.sub.swap(null_mut(), SeqCst);
            if other.is_null() {
                // Somebody took it before us? No problem, let's try the CAS
                // again!
                continue;
            }
            // Append the subqueue with the freshly took subqueue.
            (*sub).append(read(other));
            // Dealloc the other's pointer.
            Global.dealloc_one(NonNull::new_unchecked(other));
        }
    }
}

impl<T> Drop for LooseQueue<T> {
    fn drop(&mut self) {
        // No other threads will drop, no need to take.
        let sub = self.sub.load(SeqCst);
        if !sub.is_null() {
            // We have only work to do if it is not null, of course.
            unsafe {
                // Pop everything to call destructor on T and deallocate
                // the nodes.
                while let Some(_) = (*sub).pop() {}
                // And then deallocate the subqueue.
                Global.dealloc_one(NonNull::new_unchecked(sub));
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a LooseQueue<T> {
    type Item = T;

    type IntoIter = Drainer<'a, T>;

    fn into_iter(self) -> Self::IntoIter { self.drain() }
}

impl<T> FromIterator<T> for LooseQueue<T> {
    fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let this = Self::new();
        this.extend(iterable);
        this
    }
}

impl<T> fmt::Debug for LooseQueue<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "front <= ")?;
        for elem in self.inspect() {
            write!(fmtr, "{:?} <= ", elem)?;
        }
        write!(fmtr, "back")
    }
}

unsafe impl<T> Send for LooseQueue<T>
where
    T: Send + Sync,
{
}

unsafe impl<T> Sync for LooseQueue<T>
where
    T: Sync,
{
}

/// An iterator which inspects a subqueue.
pub struct Inspector<'a, T>
where
    T: 'a,
{
    queue: &'a LooseQueue<T>,
    sub: *mut SubQueue<T>,
    curr: Option<*mut Node<T>>,
}

impl<'a, T> Iterator for Inspector<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // If curr were None or Some(null), ? operator does all the job.
        let curr = unsafe { self.curr?.as_ref()? };
        // Put next into curr.
        self.curr = Some(curr.next);
        // And we're done!
        Some(&curr.val)
    }
}

impl<'a, T> Drop for Inspector<'a, T> {
    fn drop(&mut self) {
        if !self.sub.is_null() {
            // If there was a subqueue, let's put it back.
            unsafe {
                self.queue.reinsert(self.sub);
            }
        }
    }
}

impl<'a, T> fmt::Debug for Inspector<'a, T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "Inspector {} node: {:?} {}", '{', self.curr, '}')
    }
}

/// A drainer over the queue.
pub struct Drainer<'a, T>
where
    T: 'a,
{
    queue: &'a LooseQueue<T>,
    sub: *mut SubQueue<T>,
}

impl<'a, T> Iterator for Drainer<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe { self.sub.as_mut()?.pop() }
    }
}

impl<'a, T> Drop for Drainer<'a, T> {
    fn drop(&mut self) {
        if !self.sub.is_null() {
            // If there was a subqueue, put it back.
            unsafe {
                self.queue.reinsert(self.sub);
            }
        }
    }
}

impl<'a, T> fmt::Debug for Drainer<'a, T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "Drainer {} sub: {:?} {}", '{', self.sub, '}')
    }
}

struct SubQueue<T> {
    front: *mut Node<T>,
    back: *mut Node<T>,
}

impl<T> SubQueue<T> {
    fn alloc() -> NonNull<Self> {
        let sub = Global.alloc_one().unwrap_or_else(::oom);
        unsafe {
            write(
                sub.as_ptr(),
                SubQueue {
                    front: null_mut(),
                    back: null_mut(),
                },
            );
        }
        sub
    }

    fn append(&mut self, other: Self) {
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
        let node = Global.alloc_one().unwrap_or_else(::oom).as_ptr();
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
