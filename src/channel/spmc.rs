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

/// Creates an asynchronous lock-free Single-Producer-Multi-Consumer (SPMC)
/// channel. In order to allow multiple consumers, `Receiver` is clonable and
/// does not require mutability.
pub fn create<T>() -> (Sender<T>, Receiver<T>) {
    with_incin(SharedIncin::new())
}

/// Same as `create`, but use a passed incinerator instead of creating a new
/// one.
pub fn with_incin<T>(incin: SharedIncin<T>) -> (Sender<T>, Receiver<T>) {
    let alloc = OwnedAlloc::new(Node {
        message: Removable::empty(),
        next: AtomicPtr::new(null_mut()),
    });
    let nnptr = alloc.into_raw();

    let sender = Sender { back: nnptr };
    let receiver = Receiver {
        inner: Arc::new(ReceiverInner {
            front: AtomicPtr::new(nnptr.as_ptr()),
            incin,
        }),
    };

    (sender, receiver)
}

/// The `Sender` handle of a SPMC channel. Created by `channel` or `with_incin`
/// function.
pub struct Sender<T> {
    back: NonNull<Node<T>>,
}

impl<T> Sender<T> {
    /// Sends a message and if the receiver disconnected, an error is returned.
    pub fn send(&mut self, message: T) -> Result<(), NoRecv<T>> {
        let alloc = OwnedAlloc::new(Node {
            message: Removable::new(message),
            next: AtomicPtr::new(null_mut()),
        });
        let nnptr = alloc.into_raw();

        let res = unsafe {
            self.back.as_ref().next.compare_and_swap(
                null_mut(),
                nnptr.as_ptr(),
                Release,
            )
        };

        if res.is_null() {
            self.back = nnptr;
            Ok(())
        } else {
            let alloc = unsafe { OwnedAlloc::from_raw(nnptr) };
            let message = alloc.message.take().unwrap();
            Err(NoRecv { message })
        }
    }

    /// Tests if there are any `Receiver`s still connected. There are no
    /// guarantees that `send` will succeed if this method returns `true`
    /// because the `Receiver` may disconnect meanwhile.
    pub fn is_connected(&self) -> bool {
        let back = unsafe { self.back.as_ref() };
        back.next.load(Acquire).is_null()
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let res = unsafe {
            self.back.as_ref().next.compare_and_swap(
                null_mut(),
                (null_mut::<Node<T>>() as usize | 1) as *mut _,
                Release,
            )
        };

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

/// The `Receiver` handle of a SPMC channel. Created by `channel` function. It
/// is clonable and does not require mutability.
pub struct Receiver<T> {
    inner: Arc<ReceiverInner<T>>,
}

impl<T> Receiver<T> {
    /// Tries to receive a message. If no message is available,
    /// `Err(RecvErr::NoMessage)` is returned. If the sender disconnected,
    /// `Err(RecvErr::NoSender)` is returned.
    #[allow(unused_must_use)]
    pub fn recv(&self) -> Result<T, RecvErr> {
        loop {
            let pause = self.inner.incin.inner.pause();
            let front_nnptr = unsafe {
                let ptr = self.inner.front.load(Acquire);
                debug_assert!(!ptr.is_null());
                NonNull::new_unchecked(ptr)
            };

            match unsafe { front_nnptr.as_ref() }.message.take() {
                Some(val) => {
                    unsafe { self.try_clear_first(front_nnptr, pause) };
                    break Ok(val);
                },

                None => unsafe { self.try_clear_first(front_nnptr, pause)? },
            }
        }
    }

    /// Tests if there are any `Sender`s still connected. There are no
    /// guarantees that `recv` will succeed if this method returns `true`
    /// because the `Receiver` may disconnect meanwhile. This method may
    /// also return `true` if the `Sender` disconnected but there are
    /// messages pending in the buffer. Note that another `Receiver` may pop
    /// out the pending messages after this method was called.
    pub fn is_connected(&self) -> bool {
        let _pause = self.inner.incin.inner.pause();
        let front = unsafe { &*self.inner.front.load(Acquire) };
        front.message.is_present() || front.next.load(Acquire) as usize & 1 == 0
    }

    /// The shared incinerator used by this `Receiver`.
    pub fn incin(&self) -> SharedIncin<T> {
        self.inner.incin.clone()
    }

    unsafe fn try_clear_first(
        &self,
        expected: NonNull<Node<T>>,
        pause: Pause<OwnedAlloc<Node<T>>>,
    ) -> Result<(), RecvErr> {
        let next = expected.as_ref().next.load(Acquire);

        if next as usize & 1 == 1 {
            Err(RecvErr::NoSender)
        } else if next.is_null() {
            Err(RecvErr::NoMessage)
        } else {
            let ptr = expected.as_ptr();
            let res = self.inner.front.compare_and_swap(ptr, next, Release);
            pause.resume();
            if res == expected.as_ptr() {
                self.inner.incin.inner.add(OwnedAlloc::from_raw(expected));
            }
            Ok(())
        }
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> fmt::Debug for Receiver<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str("spmc::Receiver")
    }
}

struct ReceiverInner<T> {
    // never null
    front: AtomicPtr<Node<T>>,
    incin: SharedIncin<T>,
}

impl<T> Drop for ReceiverInner<T> {
    fn drop(&mut self) {
        loop {
            let front =
                unsafe { NonNull::new_unchecked(self.front.load(Acquire)) };
            let next = unsafe {
                front.as_ref().next.compare_and_swap(
                    null_mut(),
                    (null_mut::<Node<T>>() as usize | 1) as *mut _,
                    AcqRel,
                )
            };

            if next.is_null() {
                break;
            }

            unsafe { OwnedAlloc::from_raw(front) };

            if next as usize & 1 == 1 {
                break;
            }

            self.front.store(next, Release);
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
        const MESSAGES: usize = 512;

        let mut done = Vec::with_capacity(MESSAGES);
        for _ in 0 .. MESSAGES {
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

        for i in 0 .. MESSAGES {
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
