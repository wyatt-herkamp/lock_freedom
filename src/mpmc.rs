use event::Event;
use queue::Queue;
use std::{sync::Arc, time::Duration};

#[derive(Debug)]
struct Channel<T> {
    evt: Event,
    items: Queue<T>,
}

/// A producer-handle.
#[derive(Debug, Clone)]
pub struct Sender<T> {
    channel: Arc<Channel<T>>,
}

/// A receiver-handle.
#[derive(Debug, Clone)]
pub struct Receiver<T> {
    channel: Arc<Channel<T>>,
}

/// Creates a multi-producer/multi-consumer channel.
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let channel = Arc::new(Channel {
        evt: Event::new(),
        items: Queue::new(),
    });
    (
        Sender {
            channel: channel.clone(),
        },
        Receiver {
            channel,
        },
    )
}

impl<T> Sender<T> {
    /// Sends a value through the channel.
    pub fn send(&self, val: T) {
        self.channel.items.push(val);
        self.channel.evt.notify_one();
    }
}

impl<T> Receiver<T> {
    /// Receives a value in lock-free style (i.e. non blocking).
    pub fn recv(&self) -> Option<T> {
        self.channel.items.pop()
    }

    /// Waits for a value or forever. This is blocking thus not lock-free.
    pub fn recv_wait(&self) -> T {
        loop {
            if let Some(x) = self.recv() {
                break x;
            }
            self.channel.evt.wait()
        }
    }

    /// Waits for a value or until it timeouts. This is blocking thus not
    /// lock-free.
    pub fn recv_timeout(&self, dur: Duration) -> Option<T> {
        self.recv().or_else(|| {
            if self.channel.evt.wait_timeout(dur) {
                self.recv()
            } else {
                None
            }
        })
    }
}
