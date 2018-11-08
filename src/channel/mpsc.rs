pub use super::{NoRecv, RecvErr};
use owned_alloc::OwnedAlloc;
use std::{
    fmt,
    ptr::{null_mut, NonNull},
    sync::{
        atomic::{AtomicPtr, Ordering::*},
        Arc,
    },
};

/// Creates an asynchronous lock-free Multi-Producer-Single-Consumer (MPSC)
/// channel. In order to allow multiple producers, `Sender` is clonable and does
/// not require mutability.
pub fn create<T>() -> (Sender<T>, Receiver<T>) {
    let alloc = OwnedAlloc::new(Node {
        message: None,
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
        back,
        front: single_node,
    };

    (sender, receiver)
}

/// The `Sender` handle of a MPSC channel. Created by `channel` function. It is
/// clonable and does not require mutability.
pub struct Sender<T> {
    inner: Arc<SenderInner<T>>,
}

impl<T> Sender<T> {
    /// Sends a message and if the receiver disconnected, an error is returned.
    pub fn send(&self, message: T) -> Result<(), NoRecv<T>> {
        let alloc = OwnedAlloc::new(Node {
            message: Some(message),
            next: AtomicPtr::new(null_mut()),
        });
        let node = alloc.into_raw();

        loop {
            let loaded = unsafe { self.inner.back.as_ref().ptr.load(Acquire) };

            if loaded as usize & 1 == 1 {
                let mut alloc = unsafe { OwnedAlloc::from_raw(node) };
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
                        delete_before_last(node);
                    }
                }

                break Ok(());
            }
        }
    }

    /// Tests if the `Receiver` is still connected. There are no guarantees that
    /// `send` will succeed if this method returns `true` because the `Receiver`
    /// may disconnect meanwhile.
    pub fn is_connected(&self) -> bool {
        let back = unsafe { self.inner.back.as_ref() };
        back.ptr.load(Acquire) as usize & 1 == 0
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

unsafe impl<T> Send for Sender<T> where T: Send {}
unsafe impl<T> Sync for Sender<T> where T: Send {}

impl<T> fmt::Debug for Sender<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str("mpsc::Sender")
    }
}

/// The `Receiver` handle of a MPSC channel. Created by `channel` function.
pub struct Receiver<T> {
    back: NonNull<SharedBack<T>>,
    front: NonNull<Node<T>>,
}

impl<T> Receiver<T> {
    /// Tries to receive a message. If no message is available,
    /// `Err(RecvErr::NoMessage)` is returned. If the sender disconnected,
    /// `Err(RecvErr::NoSender)` is returned.
    pub fn recv(&mut self) -> Result<T, RecvErr> {
        let mut node = unsafe { &mut *self.front.as_ptr() };
        loop {
            match node.message.take() {
                Some(message) => {
                    let next = node.next.load(Acquire);

                    if let Some(nnptr) = NonNull::new(next) {
                        unsafe {
                            OwnedAlloc::from_raw(self.front);
                        };
                        self.front = nnptr;
                    }

                    break Ok(message);
                },

                None => {
                    let back = unsafe {
                        self.back.as_ref().ptr.load(Acquire) as usize
                    };
                    let next = node.next.load(Acquire);

                    match NonNull::new(next) {
                        None => {
                            break if back & 1 == 0 {
                                Err(RecvErr::NoMessage)
                            } else {
                                Err(RecvErr::NoSender)
                            };
                        },

                        Some(nnptr) => {
                            unsafe {
                                node = &mut *nnptr.as_ptr();
                                OwnedAlloc::from_raw(self.front);
                            };
                            self.front = nnptr;
                        },
                    }
                },
            }
        }
    }

    /// Tests if the `Sender` is still connected. There are no guarantees that
    /// `recv` will succeed if this method returns `true` because the `Receiver`
    /// may disconnect meanwhile. This method may also return `true` if the
    /// `Sender` disconnected but there are messages pending in the buffer.
    pub fn is_connected(&self) -> bool {
        let front = unsafe { self.front.as_ref() };
        let back = unsafe { self.back.as_ref() };
        back.ptr.load(Acquire) as usize & 1 == 0
            || front.message.is_some()
            || !front.next.load(Acquire).is_null()
    }

    unsafe fn delete_all(&self) {
        let mut node_ptr = Some(self.front);

        while let Some(node) = node_ptr {
            node_ptr = NonNull::new(node.as_ref().next.load(Relaxed));
            OwnedAlloc::from_raw(node);
        }
    }
}

impl<T> Drop for Receiver<T> {
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
                unsafe { delete_before_last(self.front) }
            }
        }
    }
}

unsafe impl<T> Send for Receiver<T> where T: Send {}
unsafe impl<T> Sync for Receiver<T> where T: Send {}

impl<T> fmt::Debug for Receiver<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str("mpsc::Receiver")
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

struct SharedBack<T> {
    // lower bit is 0 when both sides connect, 1 when one disconnect
    // never null
    ptr: AtomicPtr<Node<T>>,
}

#[repr(align(/* at least */ 2))]
struct Node<T> {
    message: Option<T>,
    // lower bit is 1 means this node (and its subsequent ones) need to be
    // thrown away.
    next: AtomicPtr<Node<T>>,
}

unsafe fn delete_before_last<T>(mut curr: NonNull<Node<T>>) {
    loop {
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
    use channel::{mpsc, RecvErr};
    use std::thread;

    #[test]
    fn correct_numbers() {
        const THREADS: usize = 8;
        const MSGS_PER_THREAD: usize = 64;
        const MSGS: usize = THREADS * MSGS_PER_THREAD;

        let mut done = [false; MSGS];
        let (sender, mut receiver) = mpsc::create::<usize>();
        let mut threads = Vec::with_capacity(THREADS);

        for i in 0 .. THREADS {
            let sender = sender.clone();
            threads.push(thread::spawn(move || {
                let start = i * MSGS_PER_THREAD;
                for j in start .. start + MSGS_PER_THREAD {
                    sender.send(j).unwrap();
                }
            }))
        }

        drop(sender);

        loop {
            match receiver.recv() {
                Ok(i) => {
                    assert!(!done[i]);
                    done[i] = true;
                },

                Err(RecvErr::NoMessage) => (),

                Err(RecvErr::NoSender) => break,
            }
        }

        for thread in threads {
            thread.join().unwrap();
        }

        for (i, status) in done.iter().enumerate() {
            assert!(*status);
        }
    }
}
