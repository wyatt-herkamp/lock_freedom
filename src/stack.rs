use alloc::*;
use prelude::*;
use std::{
    iter::FromIterator,
    ptr::{null_mut, read, NonNull},
};

/// A lock-free stack. LIFO/FILO semanthics are fully respected.
#[derive(Debug)]
pub struct Stack<T> {
    top: HazardPtr<Node<T>>,
}

impl<T> Stack<T> {
    /// Creates a new empty stack.
    pub fn new() -> Self {
        Self {
            top: HazardPtr::new(Node::drop_ptr, null_mut()),
        }
    }

    /// Pushes a new value onto the top of the stack.
    pub fn push(&self, val: T) {
        let mut target = Node::new_ptr(val, null_mut());
        loop {
            // Load current top as our "next".
            let next = self.top.load(SeqCst, |ptr| ptr);
            // Put our "next" into the new top.
            unsafe { target.as_mut().next = next }
            let success = self.top.compare_and_swap(
                next,
                target.as_ptr(),
                SeqCst,
                // We will succeed if our "next" still was the top.
                |inner| inner == next,
            );
            if success {
                break;
            }
        }
    }

    /// Pops a single element from the top of the stack.
    pub fn pop(&self) -> Option<T> {
        loop {
            // First, let's load our top.
            let result = self.top.load(SeqCst, |top| {
                if top.is_null() {
                    // If top is null, we have nothing. We're done without
                    // elements.
                    Some(None)
                } else {
                    // The replacement for top is its "next".
                    // This is only possible because of hazard pointers.
                    // Otherwise, we would face the "ABA problem".
                    let success = self.top.compare_and_swap(
                        top,
                        unsafe { (*top).next },
                        SeqCst,
                        // We succeed if top still was the loaded pointer.
                        |ptr| top == ptr,
                    );

                    if success {
                        // Done with an element.
                        Some(Some(top))
                    } else {
                        // Not done.
                        None
                    }
                }
            });

            if let Some(maybe_ptr) = result {
                break maybe_ptr.map(|ptr| {
                    // Let's first get the "val" to be returned.
                    let val = unsafe { read(&mut (*ptr).val as *mut _) };
                    unsafe {
                        // Then, let's dealloc (now or later).
                        self.top.apply_dropper(NonNull::new_unchecked(ptr))
                    }
                    val
                });
            }
        }
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
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            stack: self,
        }
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) { while let Some(_) = self.pop() {} }
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

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

unsafe impl<T> Send for Stack<T>
where
    T: Send,
{
}

unsafe impl<T> Sync for Stack<T>
where
    T: Sync + Send,
{
}

/// An iterator based on `pop` operation of the `Stack`.
pub struct Iter<'a, T>
where
    T: 'a,
{
    stack: &'a Stack<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> { self.stack.pop() }
}

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    fn new_ptr(val: T, next: *mut Node<T>) -> NonNull<Node<T>> {
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
