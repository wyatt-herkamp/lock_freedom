use prelude::*;
use std::{
    alloc::{Alloc, Global},
    iter::FromIterator,
    ptr::{null_mut, read, write, NonNull},
};

/// A strict FIFO semanthics queue. This queue uses the hazard API.
/// It can be used as multi-producer and multi-consumer channel.
pub struct Queue<T> {
    back: HazardPtr<Node<T>>,
    front: HazardPtr<Node<T>>,
}

impl<T> Queue<T> {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        Self {
            back: HazardPtr::new(Node::drop_ptr, null_mut()),
            front: HazardPtr::new(Node::drop_ptr, null_mut()),
        }
    }

    /// Pushes a value into the back of the queue. This operation is also
    /// wait-free.
    pub fn push(&self, val: T) {
        let node =
            Node::new_ptr(val, HazardPtr::new(Node::drop_ptr, null_mut()));

        // This actually pretty simple: let's put our node as in place of back,
        // and also put our node as next of whatever was inside back, if it was.
        self.back.swap(node, SeqCst, |ptr| {
            if let Some(back) = unsafe { ptr.as_ref() } {
                // Putting our node as next of the other back, if back wasn't
                // null.
                back.next
                    .swap(node, SeqCst, |next| debug_assert!(next.is_null()));
            } else {
                // If back was null, then front needs to be updated.
                self.front
                    .swap(node, SeqCst, |next| debug_assert!(next.is_null()));
            }
        });
    }

    /// Takes a value from the front of the queue, if it is avaible.
    pub fn pop(&self) -> Option<T> {
        loop {
            let res = self.front.load(SeqCst, |ptr| {
                if ptr.is_null() {
                    // If front is null, the queue is empty. We have no
                    // element, but we're done.
                    Some(None)
                } else {
                    // We succeed if we can update front with the old front's
                    // next. This is only possible because of automatic hazard
                    // pointers.
                    let success = self.front.compare_and_swap(
                        ptr,
                        unsafe { (*ptr).next.load(SeqCst, |x| x) },
                        SeqCst,
                        |res| res == ptr,
                    );

                    if success {
                        // Important! First we need to clean the next's hazard
                        // pointer, in order not to have a double free.
                        unsafe { (*ptr).next.store(null_mut(), SeqCst) }
                        // Effectively obtain val.
                        let elem = unsafe { read(ptr) }.val;
                        // If back was the same as front, then back also needs
                        // to be null.
                        self.back.compare_and_swap(
                            ptr,
                            null_mut(),
                            SeqCst,
                            |_| {},
                        );
                        // Finally, let's drop the pointer now or later.
                        unsafe { self.front.apply_dropper(ptr) }
                        // And we're done. With an element.
                        Some(Some(elem))
                    } else {
                        // Not done yet. No success.
                        None
                    }
                }
            });

            if let Some(maybe_elem) = res {
                break maybe_elem;
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

impl<T> Drop for Queue<T> {
    fn drop(&mut self) { while let Some(_) = self.pop() {} }
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

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

unsafe impl<T> Send for Queue<T>
where
    T: Send,
{
}

unsafe impl<T> Sync for Queue<T>
where
    T: Sync + Send,
{
}

/// An iterator based on `pop` operation of the `Queue`.
pub struct Iter<'a, T>
where
    T: 'a,
{
    queue: &'a Queue<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> { self.queue.pop() }
}

struct Node<T> {
    val: T,
    next: HazardPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new_ptr(val: T, next: HazardPtr<Node<T>>) -> *mut Self {
        let ptr = Global.alloc_one().unwrap_or_else(::oom).as_ptr();
        unsafe {
            write(
                ptr,
                Node {
                    val,
                    next,
                },
            )
        };
        ptr
    }

    fn drop_ptr(ptr: *mut Self) {
        NonNull::new(ptr).map(|x| unsafe { Global.dealloc_one(x) });
    }
}
