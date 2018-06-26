use queue::Queue;
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering::*},
        Arc,
    },
    thread,
    time::Duration,
};

/// Lock-free event notifying. This structure is intended to be used as a
/// replacement for `std::sync::CondVar` or `std::thread::park`. It allows
/// precise thread parking while having no locks. You may want to use it with
/// an `Arc`.
#[derive(Debug)]
pub struct Event {
    fresh: Queue<(Arc<AtomicUsize>, thread::Thread)>,
    waiting: Queue<(Arc<AtomicUsize>, thread::Thread)>,
}

const PARKED: usize = 0;
const PENDING: usize = 1;
const UNPARKED: usize = 2;

impl Event {
    /// Creates a new `Event`.
    pub fn new() -> Self {
        Self {
            fresh: Queue::new(),
            waiting: Queue::new(),
        }
    }

    /// Waits until a notification of this event is sent or forever.
    pub fn wait(&self) {
        let status = Arc::new(AtomicUsize::new(PARKED));
        self.fresh.push((status.clone(), thread::current()));
        loop {
            thread::park();
            if status.compare_and_swap(PENDING, UNPARKED, Release) == PENDING {
                break;
            }
        }
    }

    /// Waits until either a notification of this event is sent or it timeouts.
    /// Returns `true` if a notification was sent, and `false` if the wait
    /// timeouted.
    pub fn wait_timeout(&self, dur: Duration) -> bool {
        let status = Arc::new(AtomicUsize::new(PARKED));
        self.fresh.push((status.clone(), thread::current()));
        thread::park_timeout(dur);
        status.swap(UNPARKED, Release) == PENDING
    }

    /// Notifies only one thread. Returns `true` if there was a thread waiting,
    /// `false` otherwise.
    pub fn notify_one(&self) -> bool {
        let res = self.notify_one_raw();
        self.poll();
        res
    }

    /// Notifies all awaiting threads and return how many threads were notified.
    pub fn notify_all(&self) -> usize {
        let mut count = 0;
        while self.notify_one_raw() {
            count += 1;
        }
        self.poll();
        count
    }

    fn notify_one_raw(&self) -> bool {
        loop {
            break if let Some((status, thread)) = self.fresh.pop() {
                if status.compare_and_swap(PARKED, PENDING, AcqRel) != PARKED {
                    continue;
                }
                thread.unpark();
                self.waiting.push((status, thread));
                true
            } else {
                false
            };
        }
    }

    fn poll(&self) {
        let queue = Queue::new();
        while let Some((status, thread)) = self.waiting.pop() {
            if status.load(Acquire) != UNPARKED {
                thread.unpark();
                queue.push((status, thread));
            }
        }
        while let Some(x) = queue.pop() {
            self.waiting.push(x);
        }
    }
}

impl Default for Event {
    fn default() -> Self {
        Self::new()
    }
}
