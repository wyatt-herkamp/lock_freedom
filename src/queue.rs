use std::{
    alloc::{oom as alloc_oom, Alloc, AllocErr, Global},
    fmt,
    iter::FromIterator,
    ptr::{null_mut, read, write, NonNull},
    sync::atomic::{AtomicPtr, Ordering::*},
};

fn oom<T>(e: AllocErr) -> T {
    eprintln!("{}", e);
    alloc_oom()
}

/// A lock-free concurrent queue, but without FIFO garantees on multithreaded
/// environments. Single thread environments still have FIFO garantees. The
/// queue is based on subqueues which threads try to take, modify and then
/// publish. If necessary, subqueues are joint.
pub struct Queue<T> {
    sub: AtomicPtr<SubQueue<T>>,
}

impl<T> Queue<T> {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        Self {
            sub: AtomicPtr::new(null_mut()),
        }
    }

    /// Pushes a value in the back of the queue.
    pub fn push(&self, val: T) {
        let mut sub = self.sub.swap(null_mut(), SeqCst);
        if sub.is_null() {
            sub = SubQueue::alloc().as_ptr();
        }
        unsafe {
            (*sub).push(val);
            self.reinsert(sub);
        }
    }

    /// Pops a value from the front of the queue.
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

    /// Extends the queue from an iterator.
    pub fn extend<I>(&self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        let mut sub = self.sub.swap(null_mut(), SeqCst);
        if sub.is_null() {
            sub = SubQueue::alloc().as_ptr();
        }
        unsafe {
            for item in iterable {
                (*sub).push(item);
            }
            self.reinsert(sub);
        }
    }

    /// Creates an inspector on the current subqueue.
    pub fn inspect<'a>(&'a self) -> Inspector<'a, T> {
        let sub = self.sub.swap(null_mut(), SeqCst);
        Inspector {
            queue: self,
            sub,
            curr: unsafe { sub.as_ref().map(|x| x.front) },
        }
    }

    /// Creates a drainer on the current subqueue. The drainer takes the
    /// subqueue for itself and restores it on drop.
    pub fn drain<'a>(&'a self) -> Drainer<'a, T> {
        let sub = self.sub.swap(null_mut(), SeqCst);
        Drainer {
            queue: self,
            sub,
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

impl<'a, T> IntoIterator for &'a Queue<T> {
    type Item = T;

    type IntoIter = Drainer<'a, T>;

    fn into_iter(self) -> Self::IntoIter { self.drain() }
}

impl<T> fmt::Debug for Queue<T>
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

/// An iterator which inspects a subqueue.
pub struct Inspector<'a, T>
where
    T: 'a,
{
    queue: &'a Queue<T>,
    sub: *mut SubQueue<T>,
    curr: Option<*mut Node<T>>,
}

impl<'a, T> Iterator for Inspector<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = unsafe { self.curr?.as_ref()? };
        self.curr = Some(curr.next);
        Some(&curr.val)
    }
}

impl<'a, T> Drop for Inspector<'a, T> {
    fn drop(&mut self) {
        if !self.sub.is_null() {
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
    queue: &'a Queue<T>,
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
        let sub = Global.alloc_one().unwrap_or_else(oom);
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
