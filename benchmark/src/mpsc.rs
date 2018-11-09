#[macro_use]
extern crate benchsuite;
extern crate lockfree;

use benchsuite::exec::Target;
use lockfree::channel::mpsc;
use std::{
    collections::VecDeque,
    sync::{mpsc as std_mpsc, Arc, Mutex},
};

#[derive(Debug)]
struct MutexTarget {
    inner: Arc<Mutex<VecDeque<u8>>>,
    has_recv: bool,
}

impl Default for MutexTarget {
    fn default() -> Self {
        Self {
            inner: Arc::new(Mutex::new(VecDeque::new())),
            has_recv: true,
        }
    }
}

impl Clone for MutexTarget {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            has_recv: false,
        }
    }
}

impl Target for MutexTarget {
    #[inline(always)]
    fn round(&mut self) {
        if self.has_recv {
            self.inner.lock().unwrap().pop_front();
        }
        let mut queue = self.inner.lock().unwrap();
        queue.push_back(234);
        queue.push_back(235);
    }
}

#[derive(Debug)]
struct StdTarget {
    sender: std_mpsc::Sender<u8>,
    receiver: Option<std_mpsc::Receiver<u8>>,
}

impl Default for StdTarget {
    fn default() -> Self {
        let (sender, receiver) = std_mpsc::channel();
        Self {
            sender,
            receiver: Some(receiver),
        }
    }
}

impl Clone for StdTarget {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: None,
        }
    }
}

impl Target for StdTarget {
    #[inline(always)]
    #[allow(unused_must_use)]
    fn round(&mut self) {
        if let Some(receiver) = &self.receiver {
            receiver.try_recv();
        }
        self.sender.send(234);
        self.sender.send(235);
    }
}

#[derive(Debug)]
struct LockfreeTarget {
    sender: mpsc::Sender<u8>,
    receiver: Option<mpsc::Receiver<u8>>,
}

impl Default for LockfreeTarget {
    fn default() -> Self {
        let (sender, receiver) = mpsc::create();
        Self {
            sender,
            receiver: Some(receiver),
        }
    }
}

impl Clone for LockfreeTarget {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: None,
        }
    }
}

impl Target for LockfreeTarget {
    #[inline(always)]
    #[allow(unused_must_use)]
    fn round(&mut self) {
        if let Some(receiver) = &mut self.receiver {
            receiver.recv();
        }
        self.sender.send(234);
        self.sender.send(235);
    }
}

fn main() {
    bench! {
        levels 2, 4, 8, 16;
        "mutex vector" => MutexTarget::default(),
        "std's mpsc" => StdTarget::default(),
        "lockfree mpsc" => LockfreeTarget::default(),
    }
}
