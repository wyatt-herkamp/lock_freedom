use super::{NoRecv, RecvErr};
use owned_alloc::OwnedAlloc;
use std::{
    fmt,
    ptr::{null_mut, NonNull},
    sync::atomic::{AtomicPtr, Ordering::*},
};

/// Creates an asynchronous lock-free Single-Producer-Single-Consumer (SPSC)
/// channel.
pub fn create<T>() -> (Sender<T>, Receiver<T>) {
    let alloc = OwnedAlloc::new(Node {
        message: None,
        next: AtomicPtr::new(null_mut()),
    });
    let nnptr = alloc.into_raw();

    (Sender { back: nnptr }, Receiver { front: nnptr })
}

/// The `Sender` handle of a SPSC channel. Created by `channel` function.
pub struct Sender<T> {
    back: NonNull<Node<T>>,
}

impl<T> Sender<T> {
    /// Sends a message and if the receiver disconnected, an error is returned.
    pub fn send(&mut self, message: T) -> Result<(), NoRecv<T>> {
        let alloc = OwnedAlloc::new(Node {
            message: Some(message),
            next: AtomicPtr::new(null_mut()),
        });
        let nnptr = alloc.into_raw();

        let res = unsafe { self.back.as_ref() }.next.compare_and_swap(
            null_mut(),
            nnptr.as_ptr(),
            Release,
        );

        if res.is_null() {
            self.back = nnptr;
            Ok(())
        } else {
            let mut alloc = unsafe { OwnedAlloc::from_raw(nnptr) };
            let message = alloc.message.take().unwrap();
            Err(NoRecv { message })
        }
    }

    /// Tests if the `Receiver` is still connected. There are no guarantees that
    /// `send` will succeed if this method returns `true` because the `Receiver`
    /// may disconnect meanwhile.
    pub fn is_connected(&self) -> bool {
        let back = unsafe { self.back.as_ref() };
        back.next.load(Acquire).is_null()
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let res = unsafe { self.back.as_ref() }.next.compare_and_swap(
            null_mut(),
            (null_mut::<Node<T>>() as usize | 1) as *mut _,
            Release,
        );

        if !res.is_null() {
            unsafe { OwnedAlloc::from_raw(self.back) };
        }
    }
}

unsafe impl<T> Send for Sender<T> where T: Send {}
unsafe impl<T> Sync for Sender<T> where T: Send {}

impl<T> fmt::Debug for Sender<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str("spsc::Senderr")
    }
}

/// The `Receiver` handle of a SPSC channel. Created by `channel` function.
pub struct Receiver<T> {
    front: NonNull<Node<T>>,
}

impl<T> Receiver<T> {
    /// Tries to receive a message. If no message is available,
    /// `Err(RecvErr::NoMessage)` is returned. If the sender disconnected,
    /// `Err(RecvErr::NoSender)` is returned.
    pub fn recv(&mut self) -> Result<T, RecvErr> {
        loop {
            let node = unsafe { &mut *self.front.as_ptr() };

            match node.message.take() {
                Some(message) => {
                    let next = node.next.load(Acquire) as usize;

                    if let Some(nnptr) = NonNull::new((next & !1) as *mut _) {
                        unsafe { OwnedAlloc::from_raw(self.front) };
                        self.front = nnptr;
                    }

                    break Ok(message);
                },

                None => {
                    let next = node.next.load(Acquire);

                    if next as usize & 1 == 0 {
                        match NonNull::new(next) {
                            Some(nnptr) => {
                                unsafe { OwnedAlloc::from_raw(self.front) };
                                self.front = nnptr;
                            },

                            None => break Err(RecvErr::NoMessage),
                        }
                    } else {
                        break Err(RecvErr::NoSender);
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
        front.message.is_some() || front.next.load(Acquire) as usize & 1 == 0
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        loop {
            let next = unsafe { self.front.as_ref() }.next.compare_and_swap(
                null_mut(),
                (null_mut::<Node<T>>() as usize | 1) as *mut _,
                AcqRel,
            );

            let next_nnptr = match NonNull::new(next) {
                Some(nnptr) => nnptr,
                None => break,
            };

            unsafe { OwnedAlloc::from_raw(self.front) };

            if next as usize & 1 == 1 {
                break;
            }

            self.front = next_nnptr;
        }
    }
}

unsafe impl<T> Send for Receiver<T> where T: Send {}
unsafe impl<T> Sync for Receiver<T> where T: Send {}

impl<T> fmt::Debug for Receiver<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str("spsc::Receiver")
    }
}

#[repr(align(/* at least */ 2))]
struct Node<T> {
    message: Option<T>,
    // lower bit is 1 for "disconnected" and 0 for "connected"
    next: AtomicPtr<Node<T>>,
}

#[cfg(test)]
mod test {
    use channel::{spsc, RecvErr};
    use std::thread;

    #[test]
    fn correct_sequence() {
        const MSGS: usize = 512;

        let (mut sender, mut receiver) = spsc::create::<usize>();
        let thread = thread::spawn(move || {
            for i in 0 .. MSGS {
                loop {
                    match receiver.recv() {
                        Ok(j) => {
                            assert_eq!(i, j);
                            break;
                        },

                        Err(RecvErr::NoMessage) => (),

                        _ => unreachable!(),
                    }
                }
            }
        });

        for i in 0 .. MSGS {
            sender.send(i).unwrap();
        }

        thread.join().unwrap();
    }
}
