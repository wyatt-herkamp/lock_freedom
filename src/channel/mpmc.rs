pub use super::{
    NoRecv,
    RecvErr::{self, *},
};
use incin::{Incinerator, Pause};
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

/// Creates an asynchronous lock-free Multi-Producer-Multi-Consumer (MPMC)
/// channel. In order to allow multiple producers and multiple receivers,
/// [`Sender`] and [`Receiver`] are clonable and do not require mutability.
pub fn create<T>() -> (Sender<T>, Receiver<T>) {
    with_incin(SharedIncin::new())
}

/// Same as [`create`], but use a passed incinerator instead of creating a new
/// one.
pub fn with_incin<T>(incin: SharedIncin<T>) -> (Sender<T>, Receiver<T>) {
    let alloc = OwnedAlloc::new(Node {
        message: Removable::empty(),
        next: AtomicPtr::new(null_mut()),
    });
    let single_node = alloc.into_raw();

    let shared = SharedBack {
        ptr: AtomicPtr::new(single_node.as_ptr()),
    };
    let alloc = OwnedAlloc::new(shared);
    let back = alloc.into_raw();

    let sender = Sender {
        inner: Arc::new(SenderInner { back }),
    };
    let receiver = Receiver {
        inner: Arc::new(ReceiverInner {
            front: AtomicPtr::new(single_node.as_ptr()),
            back,
            incin,
        }),
    };

    (sender, receiver)
}

/// The [`Sender`] handle of a MPMC channel. Created by [`create`] or
/// [`with_incin`] function. It is clonable and does not require mutability.
pub struct Sender<T> {
    inner: Arc<SenderInner<T>>,
}

impl<T> Sender<T> {
    /// Sends a message and if the receiver disconnected, an error is returned.
    pub fn send(&self, message: T) -> Result<(), NoRecv<T>> {
        let alloc = OwnedAlloc::new(Node {
            message: Removable::new(message),
            next: AtomicPtr::new(null_mut()),
        });
        let node = alloc.into_raw();

        loop {
            let loaded = unsafe { self.inner.back.as_ref().ptr.load(Acquire) };

            if loaded as usize & 1 == 1 {
                let alloc = unsafe { OwnedAlloc::from_raw(node) };
                let message = alloc.message.take().unwrap();
                break Err(NoRecv { message });
            }

            let res = unsafe {
                self.inner.back.as_ref().ptr.compare_and_swap(
                    loaded,
                    node.as_ptr(),
                    Release,
                )
            };

            if res == loaded {
                debug_assert!(!loaded.is_null());
                let prev = unsafe { NonNull::new_unchecked(loaded) };
                let res = unsafe {
                    prev.as_ref().next.compare_and_swap(
                        null_mut(),
                        node.as_ptr(),
                        Release,
                    )
                };

                if !res.is_null() {
                    unsafe {
                        OwnedAlloc::from_raw(prev);
                        delete_before_last(node, None);
                    }
                }

                break Ok(());
            }
        }
    }

    /// Tests if there are any [`Receiver`]s still connected. There are no
    /// guarantees that [`send`](Sender::send) will succeed if this method
    /// returns `true` because the [`Receiver`] may disconnect meanwhile.
    pub fn is_connected(&self) -> bool {
        let back = unsafe { self.inner.back.as_ref() };
        back.ptr.load(Acquire) as usize & 1 == 0
    }
}

unsafe impl<T> Send for Sender<T> where T: Send {}
unsafe impl<T> Sync for Sender<T> where T: Send {}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> fmt::Debug for Sender<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str("mpmc::Sender")
    }
}

/// The [`Receiver`] handle of a MPMC channel. Created by [`create`] or
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
        let pause = self.inner.incin.inner.pause();
        loop {
            let front_nnptr = unsafe {
                let ptr = self.inner.front.load(Acquire);
                debug_assert!(!ptr.is_null());
                NonNull::new_unchecked(ptr)
            };

            match unsafe { front_nnptr.as_ref() }.message.take() {
                Some(val) => {
                    unsafe { self.try_clear_first(front_nnptr, &pause) };
                    break Ok(val);
                },

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
        let _pause = self.inner.incin.inner.pause();
        let front = unsafe { &*self.inner.front.load(Acquire) };
        front.message.is_present() || front.next.load(Acquire) as usize & 1 == 0
    }

    /// The shared incinerator used by this [`Receiver`].
    pub fn incin(&self) -> SharedIncin<T> {
        self.inner.incin.clone()
    }

    unsafe fn try_clear_first(
        &self,
        expected: NonNull<Node<T>>,
        pause: &Pause<OwnedAlloc<Node<T>>>,
    ) -> Result<(), RecvErr> {
        let next = expected.as_ref().next.load(Acquire);

        if !next.is_null() {
            let ptr = expected.as_ptr();
            let res = self.inner.front.compare_and_swap(ptr, next, Release);
            if res == expected.as_ptr() {
                pause.add_to_incin(OwnedAlloc::from_raw(expected));
            }
            Ok(())
        } else if self.inner.back.as_ref().ptr.load(Acquire) as usize & 1 == 1 {
            Err(RecvErr::NoSender)
        } else {
            Err(RecvErr::NoMessage)
        }
    }
}

unsafe impl<T> Send for Receiver<T> where T: Send {}
unsafe impl<T> Sync for Receiver<T> where T: Send {}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> fmt::Debug for Receiver<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str("mpmc::Receiver")
    }
}

struct SenderInner<T> {
    back: NonNull<SharedBack<T>>,
}

impl<T> Drop for SenderInner<T> {
    fn drop(&mut self) {
        loop {
            let ptr = unsafe { self.back.as_ref().ptr.load(Acquire) };

            if ptr as usize & 1 == 1 {
                let ptr = (ptr as usize & !1) as *mut Node<T>;
                debug_assert!(!ptr.is_null());
                unsafe {
                    OwnedAlloc::from_raw(NonNull::new_unchecked(ptr));
                    OwnedAlloc::from_raw(self.back);
                };
                break;
            }

            let res = unsafe {
                self.back.as_ref().ptr.compare_and_swap(
                    ptr,
                    (ptr as usize | 1) as *mut _,
                    Release,
                )
            };

            if res == ptr {
                break;
            }
        }
    }
}

struct ReceiverInner<T> {
    // never null
    front: AtomicPtr<Node<T>>,
    back: NonNull<SharedBack<T>>,
    incin: SharedIncin<T>,
}

impl<T> ReceiverInner<T> {
    unsafe fn delete_all(&self) {
        let mut node_ptr = NonNull::new(self.front.load(Relaxed));

        while let Some(node) = node_ptr {
            node_ptr = NonNull::new(node.as_ref().next.load(Relaxed));
            OwnedAlloc::from_raw(node);
        }
    }
}

impl<T> Drop for ReceiverInner<T> {
    fn drop(&mut self) {
        loop {
            let ptr = unsafe { self.back.as_ref().ptr.load(Acquire) };

            if ptr as usize & 1 == 1 {
                unsafe {
                    self.delete_all();
                    OwnedAlloc::from_raw(self.back);
                }
                break;
            }

            let res = unsafe {
                self.back.as_ref().ptr.compare_and_swap(
                    ptr,
                    (ptr as usize | 1) as *mut _,
                    Release,
                )
            };

            if res == ptr {
                debug_assert!(!ptr.is_null());
                unsafe {
                    delete_before_last(
                        NonNull::new_unchecked(self.front.load(Relaxed)),
                        NonNull::new(ptr),
                    )
                }
                break;
            }
        }
    }
}

struct SharedBack<T> {
    // lower bit is 0 when both sides connect, 1 when one disconnect
    // never null
    ptr: AtomicPtr<Node<T>>,
}

#[repr(align(/* at least */ 2))]
struct Node<T> {
    message: Removable<T>,
    // lower bit is 1 means this node (and its subsequent ones) need to be
    // thrown away.
    next: AtomicPtr<Node<T>>,
}

make_shared_incin! {
    { "`mpmc::Receiver`" }
    pub SharedIncin<T> of OwnedAlloc<Node<T>>
}

unsafe fn delete_before_last<T>(
    mut curr: NonNull<Node<T>>,
    last: Option<NonNull<Node<T>>>,
) {
    while last != Some(curr) {
        let next = curr.as_ref().next.compare_and_swap(
            null_mut(),
            (null_mut::<Node<T>>() as usize | 1) as *mut _,
            Release,
        );

        match NonNull::new(next) {
            Some(next) => {
                OwnedAlloc::from_raw(curr);
                curr = next;
            },

            None => break,
        }
    }
}

#[cfg(test)]
mod test {
    use channel::mpmc;
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
        const MSGS_PER_THREAD: usize = 64;
        const MSGS: usize = THREADS * MSGS_PER_THREAD;

        let mut done = Vec::with_capacity(MSGS);
        for _ in 0 .. MSGS {
            done.push(AtomicBool::new(false));
        }
        let done = Arc::<[AtomicBool]>::from(done);
        let (sender, receiver) = mpmc::create::<usize>();
        let mut threads = Vec::with_capacity(THREADS);

        for i in 0 .. THREADS {
            let sender = sender.clone();
            threads.push(thread::spawn(move || {
                let start = i * MSGS_PER_THREAD;
                for j in start .. start + MSGS_PER_THREAD {
                    sender.send(j).unwrap();
                }
            }));

            let receiver = receiver.clone();
            let done = done.clone();
            threads.push(thread::spawn(move || loop {
                match receiver.recv() {
                    Ok(i) => assert!(!done[i].swap(true, AcqRel)),

                    Err(mpmc::NoMessage) => (),

                    Err(mpmc::NoSender) => break,
                }
            }));
        }

        drop(sender);
        drop(receiver);

        for thread in threads {
            thread.join().unwrap();
        }

        for status in done.iter() {
            assert!(status.load(Relaxed));
        }
    }
}
