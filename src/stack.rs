use incin::Incinerator;
use owned_alloc::{OwnedAlloc, UninitAlloc};
use std::{
    fmt,
    iter::FromIterator,
    ptr::{null_mut, NonNull},
    sync::{
        atomic::{AtomicPtr, Ordering::*},
        Arc,
    },
};

/// A lock-free stack. LIFO/FILO semanthics are fully respected.
pub struct Stack<T> {
    top: AtomicPtr<Node<T>>,
    incin: SharedIncin<T>,
}

impl<T> Stack<T> {
    /// Creates a new empty stack.
    pub fn new() -> Self {
        Self::with_incin(SharedIncin::new())
    }

    /// Creates an empty queue using the passed shared incinerator.
    pub fn with_incin(incin: SharedIncin<T>) -> Self {
        Self {
            top: AtomicPtr::new(null_mut()),
            incin,
        }
    }

    /// Returns the shared incinerator used by this `Stack`.
    pub fn incin(&self) -> SharedIncin<T> {
        self.incin.clone()
    }

    /// Pushes a new value onto the top of the stack.
    pub fn push(&self, val: T) {
        let mut target = OwnedAlloc::new(Node::new(val, null_mut())).into_raw();
        loop {
            // Load current top as our "next".
            let next = self.top.load(Acquire);
            // Put our "next" into the new top.
            unsafe { target.as_mut().next = next }
            let inner =
                self.top.compare_and_swap(next, target.as_ptr(), Release);
            // We will succeed if our "next" still was the top.
            if inner == next {
                break;
            }
        }
    }

    /// Pops a single element from the top of the stack.
    pub fn pop(&self) -> Option<T> {
        let result = self.incin.inner.pause_with(|| loop {
            // First, let's load our top.
            let top = self.top.load(Acquire);
            match NonNull::new(top) {
                // If top is null, we have nothing. We're done without
                // elements.
                None => break None,

                Some(nnptr) => {
                    // The replacement for top is its "next".
                    // This is only possible because of hazard pointers.
                    // Otherwise, we would face the "ABA problem".
                    let res = self.top.compare_and_swap(
                        top,
                        unsafe { nnptr.as_ref().next },
                        Release,
                    );

                    // We succeed if top still was the loaded pointer.
                    if res == top {
                        // Done with an element.
                        break Some(nnptr);
                    }
                    // Not done.
                },
            }
        });

        result.map(|mut nnptr| {
            // Let's first get the "val" to be returned.
            let val = unsafe { (&mut nnptr.as_mut().val as *mut T).read() };
            // Then, let's dealloc (now or later).
            self.incin
                .inner
                .add(unsafe { UninitAlloc::from_raw(nnptr) });
            val
        })
    }

    /// Extends the stack from a given iterable. All values are pushed.
    pub fn extend<I>(&self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        for elem in iterable {
            self.push(elem);
        }
    }

    /// Creates an iterator over `T`s, based on `pop` operation of the stack.
    pub fn iter<'a>(&'a self) -> PopIter<'a, T> {
        PopIter { stack: self }
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut node_ptr = self.top.load(Relaxed);

        while let Some(nnptr) = NonNull::new(node_ptr) {
            let node = unsafe { OwnedAlloc::from_raw(nnptr) };
            node_ptr = node.next;
        }
    }
}

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let this = Self::new();
        this.extend(iterable);
        this
    }
}

impl<T> fmt::Debug for Stack<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "Stack {} top: {:?}, incin: {:?} {}",
            '{', self.top, self.incin, '}'
        )
    }
}

unsafe impl<T> Send for Stack<T> where T: Send {}
unsafe impl<T> Sync for Stack<T> where T: Send {}

/// An iterator based on `pop` operation of the `Stack`.
pub struct PopIter<'a, T>
where
    T: 'a,
{
    stack: &'a Stack<T>,
}

impl<'a, T> Iterator for PopIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

make_shared_incin! {
    { "`Stack`" }
    pub SharedIncin<T> of UninitAlloc<Node<T>>
}

impl<T> fmt::Debug for SharedIncin<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "SharedIncin {} inner: {:?} {}", '{', self.inner, '}')
    }
}

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(val: T, next: *mut Self) -> Self {
        Self { val, next }
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
        let stack = Stack::<usize>::new();
        assert!(stack.pop().is_none());
    }

    #[test]
    fn on_empty_last_pop_is_none() {
        let stack = Stack::new();
        stack.push(3);
        stack.push(1234);
        stack.pop();
        stack.pop();
        assert!(stack.pop().is_none());
    }

    #[test]
    fn order() {
        let stack = Stack::new();
        stack.push(3);
        stack.push(5);
        stack.push(6);
        assert_eq!(stack.pop(), Some(6));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(3));
    }

    #[test]
    fn no_data_corruption() {
        const NTHREAD: usize = 20;
        const NITER: usize = 800;
        const NMOD: usize = 55;

        let stack = Arc::new(Stack::new());
        let mut handles = Vec::with_capacity(NTHREAD);

        for i in 0 .. NTHREAD {
            let stack = stack.clone();
            handles.push(thread::spawn(move || {
                for j in 0 .. NITER {
                    let val = (i * NITER) + j;
                    stack.push(val);
                    if (val + 1) % NMOD == 0 {
                        if let Some(val) = stack.pop() {
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
        while let Some(val) = stack.pop() {
            assert!(val < NITER * NTHREAD);
            res += 1;
        }

        assert_eq!(res, expected);
    }
}
