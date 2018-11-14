pub use super::{
    NoRecv,
    RecvErr::{self, *},
};
use incin::Pause;
use owned_alloc::OwnedAlloc;
use removable::Removable;
use std::{
    fmt,
    ptr::{null_mut, NonNull},
    sync::{
        atomic::{AtomicPtr, Ordering::*},
        Arc,
    },
};

/// Creates an asynchronous lock-free Single-Producer-Multi-Consumer (SPMC)
/// channel. In order to allow multiple consumers, [`Receiver`] is clonable and
/// does not require mutability.
pub fn create<T>() -> (Sender<T>, Receiver<T>) {
    with_incin(SharedIncin::new())
}

/// Same as [`create`], but use a passed incinerator instead of creating a new
/// one.
pub fn with_incin<T>(incin: SharedIncin<T>) -> (Sender<T>, Receiver<T>) {
    // First we create a single node shared between two ends.
    let alloc = OwnedAlloc::new(Node {
        message: Removable::empty(),
        next: AtomicPtr::new(null_mut()),
    });
    let single_node = alloc.into_raw();

    // Then put it on back and on the front.
    let sender = Sender { back: single_node };
    let receiver = Receiver {
        inner: Arc::new(ReceiverInner {
            front: AtomicPtr::new(single_node.as_ptr()),
            incin,
        }),
    };

    (sender, receiver)
}

/// The [`Sender`] handle of a SPMC channel. Created by [`create`] or
/// [`with_incin`] function.
pub struct Sender<T> {
    back: NonNull<Node<T>>,
}

impl<T> Sender<T> {
    /// Sends a message and if the receiver disconnected, an error is returned.
    pub fn send(&mut self, message: T) -> Result<(), NoRecv<T>> {
        // First we allocate the node for our message.
        let alloc = OwnedAlloc::new(Node {
            message: Removable::new(message),
            next: AtomicPtr::new(null_mut()),
        });
        let nnptr = alloc.into_raw();

        // This dereferral is safe because the queue has at least one node. We
        // possess a single node in the back, and if the queue has just one
        // node, it is stored in the back (and in the front). Also, we are the
        // only ones with access to the back.
        let res = unsafe {
            // We try to update the back's next pointer. We want to catch any
            // bit marking here. A marked lower bit means the receiver
            // disconnected.
            self.back.as_ref().next.compare_and_swap(
                null_mut(),
                nnptr.as_ptr(),
                Release,
            )
        };

        if res.is_null() {
            // If we succeeded, let's update the back so we keep the invariant
            // "the back has a single node".
            self.back = nnptr;
            Ok(())
        } else {
            // If we failed, receiver disconnected. It is safe to dealloc
            // because this is the node we just allocated, and we did not share
            // it with anyone (cas failed).
            let mut alloc = unsafe { OwnedAlloc::from_raw(nnptr) };
            let message = alloc.message.replace(None).unwrap();
            Err(NoRecv { message })
        }
    }

    /// Tests if there are any [`Receiver`]s still connected. There are no
    /// guarantees that [`send`](Sender::send) will succeed if this method
    /// returns `true` because the [`Receiver`] may disconnect meanwhile.
    pub fn is_connected(&self) -> bool {
        // Safe because we always have at least one node, which is only dropped
        // in the last side to disconnect's drop.
        let back = unsafe { self.back.as_ref() };
        back.next.load(Acquire).is_null()
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        // This dereferral is safe because the queue always have at least one
        // node. This single node is only dropped when the last side to
        // disconnect drops.
        let res = unsafe {
            // Let's try to mark next's bit so that receiver will see we
            // disconnected, if it hasn't disconnected by itself. It is ok to
            // just swap, since we have only two possible values (null and
            // null | 1) and we everyone will be setting to the same value
            // (null | 1).
            self.back
                .as_ref()
                .next
                .swap((null_mut::<Node<T>>() as usize | 1) as *mut _, AcqRel)
        };

        // If the previously stored value was not null, receiver has already
        // disconnected. It is safe to drop because we are the only ones that
        // have a pointer to the node.
        if !res.is_null() {
            unsafe { OwnedAlloc::from_raw(self.back) };
        }
    }
}

impl<T> fmt::Debug for Sender<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str("spmc::Sender")
    }
}

unsafe impl<T> Send for Sender<T> where T: Send {}
unsafe impl<T> Sync for Sender<T> where T: Send {}

/// The [`Receiver`] handle of a SPMC channel. Created by [`create`] or
/// [`with_incin`] function. It is clonable and does not require mutability.
pub struct Receiver<T> {
    inner: Arc<ReceiverInner<T>>,
}

impl<T> Receiver<T> {
    /// Tries to receive a message. If no message is available,
    /// [`Err`]`(`[`RecvErr::NoMessage`]`)` is returned. If the sender
    /// disconnected, [`Err`]`(`[`RecvErr::NoSender`]`)` is returned.
    #[allow(unused_must_use)]
    pub fn recv(&self) -> Result<T, RecvErr> {
        // We have to pause the incinerator due to ABA problem. This channel
        // suffers from it, yeah.
        let pause = self.inner.incin.inner.pause();
        loop {
            // Bypassing null check is safe because we never store null in
            // the front.
            let front_nnptr = unsafe {
                // First we load pointer stored in the front.
                let ptr = self.inner.front.load(Acquire);
                debug_assert!(!ptr.is_null());
                NonNull::new_unchecked(ptr)
            };

            // Let's remove the node logically first. Safe to derefer this
            // pointer because we paused the incinerator and we only
            // delete nodes via incinerator.
            match unsafe { front_nnptr.as_ref().message.take(Release) } {
                Some(val) => {
                    // Safe to call because we passed a pointer from the front
                    // which was loaded during the very same pause we are
                    // passing.
                    unsafe { self.try_clear_first(front_nnptr, &pause) };
                    break Ok(val);
                },

                // Safe to call because we passed a pointer from the front
                // which was loaded during the very same pause we are
                // passing.
                None => unsafe { self.try_clear_first(front_nnptr, &pause)? },
            }
        }
    }

    /// Tests if there are any [`Sender`]s still connected. There are no
    /// guarantees that [`recv`](Receiver::recv) will succeed if this method
    /// returns `true` because the [`Receiver`] may disconnect meanwhile.
    /// This method may also return `true` if the [`Sender`] disconnected
    /// but there are messages pending in the buffer. Note that another
    /// [`Receiver`] may pop out the pending messages after this method was
    /// called.
    pub fn is_connected(&self) -> bool {
        // We need this pause because of use-after-free.
        let _pause = self.inner.incin.inner.pause();
        // Safe to derefer this pointer because we paused the incinerator and we
        // only delete nodes via incinerator.
        let front = unsafe { &*self.inner.front.load(Acquire) };
        front.message.is_present(Acquire)
            || front.next.load(Acquire) as usize & 1 == 0
    }

    /// The shared incinerator used by this [`Receiver`].
    pub fn incin(&self) -> SharedIncin<T> {
        self.inner.incin.clone()
    }

    // This function is unsafe because passing the wrong pointer will lead to
    // undefined behavior. The pointer must have been loaded from the front
    // during the passed pause.
    unsafe fn try_clear_first(
        &self,
        expected: NonNull<Node<T>>,
        pause: &Pause<OwnedAlloc<Node<T>>>,
    ) -> Result<(), RecvErr> {
        let next = expected.as_ref().next.load(Acquire);

        if next as usize & 1 == 1 {
            // If the next is bit flagged, sender disconnected, no more messages
            // ever.
            Err(RecvErr::NoSender)
        } else if next.is_null() {
            // No bit flag means sender is still there but we have no message.
            Err(RecvErr::NoMessage)
        } else {
            let ptr = expected.as_ptr();
            let res = self.inner.front.compare_and_swap(ptr, next, Release);
            // We are not oblied to succeed. This is just cleanup and some other
            // thread might do it.
            if res == expected.as_ptr() {
                // Only deleting nodes via incinerator due to ABA problem and
                // use-after-frees.
                pause.add_to_incin(OwnedAlloc::from_raw(expected));
            }
            // Then we *might* have some message since we found another node.
            Ok(())
        }
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl<T> fmt::Debug for Receiver<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str("spmc::Receiver")
    }
}

unsafe impl<T> Send for Receiver<T> where T: Send {}
unsafe impl<T> Sync for Receiver<T> where T: Send {}

struct ReceiverInner<T> {
    // never null
    front: AtomicPtr<Node<T>>,
    incin: SharedIncin<T>,
}

impl<T> Drop for ReceiverInner<T> {
    fn drop(&mut self) {
        loop {
            // This null-check-by-pass is safe because we never store null in
            // the front.
            let front =
                unsafe { NonNull::new_unchecked(self.front.load(Acquire)) };
            // This is safe because we are the only receiver left and the list
            // will always have at least one node, even in the drop. Of course,
            // unless we are the last side to drop (then we do drop it all).
            let next = unsafe {
                // Let's try to mark the next (which means we disconnected). We
                // might fail because either this is not the last node or the
                // sender already disconnected and marked this pointer.
                front.as_ref().next.compare_and_swap(
                    null_mut(),
                    (null_mut::<Node<T>>() as usize | 1) as *mut _,
                    AcqRel,
                )
            };

            // If the next is null, we are the first side to disconnect and we
            // must keep at least one node in the queue.
            if next.is_null() {
                break;
            }

            // Ok, safe to deallocate the front now. We already loaded the next
            // field and it is not null. Either the queue won't be empty or the
            // sender disconnected.
            unsafe { OwnedAlloc::from_raw(front) };

            // This means the sender disconnected we reached the end of the
            // queue.
            if next as usize & 1 == 1 {
                break;
            }

            // Now let's keep going until the list is empty.
            self.front.store(next, Relaxed);
        }
    }
}

#[repr(align(/* at least */ 2))]
struct Node<T> {
    message: Removable<T>,
    // lower bit is 1 if the other side disconnected, 0 means nothing
    next: AtomicPtr<Node<T>>,
}

make_shared_incin! {
    { "`spmc::Receiver`" }
    pub SharedIncin<T> of OwnedAlloc<Node<T>>
}

#[cfg(test)]
mod test {
    use channel::spmc;
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering::*},
            Arc,
        },
        thread,
    };

    #[test]
    fn correct_numbers() {
        const THREADS: usize = 8;
        const MSGS: usize = 512;

        let mut done = Vec::with_capacity(MSGS);
        for _ in 0 .. MSGS {
            done.push(AtomicBool::new(false));
        }
        let done = Arc::<[AtomicBool]>::from(done);

        let (mut sender, receiver) = spmc::create::<usize>();
        let mut threads = Vec::with_capacity(THREADS);

        for _ in 0 .. THREADS {
            let done = done.clone();
            let receiver = receiver.clone();
            threads.push(thread::spawn(move || loop {
                match receiver.recv() {
                    Ok(i) => assert!(!done[i].swap(true, AcqRel)),
                    Err(spmc::NoSender) => break,
                    Err(spmc::NoMessage) => (),
                }
            }))
        }

        for i in 0 .. MSGS {
            sender.send(i).unwrap();
        }

        drop(sender);

        for thread in threads {
            thread.join().unwrap();
        }

        for status in done.iter() {
            assert!(status.load(Relaxed));
        }
    }
}
