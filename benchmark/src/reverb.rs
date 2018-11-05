extern crate lockfree;

use lockfree::queue::Queue as LfQueue;
use std::{
    collections::{LinkedList, VecDeque},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

const SAMPLES: usize = 10;
const ITER: u128 = 0x4000;

fn measure<Q>(nthread: usize) -> Duration
where
    Q: Queue + Send + Sync + 'static,
{
    let channel = Arc::new(Q::default());
    let then = Instant::now();
    let mut threads = Vec::with_capacity(nthread);

    for _ in 0 .. nthread {
        let channel = channel.clone();
        threads.push(thread::spawn(move || {
            for j in 0 .. ITER {
                let popped = channel.pop();
                channel.push(j);
                if let Some(num) = popped {
                    channel.push(num + j);
                }
            }

            while let Some(_) = channel.pop() {}
        }))
    }

    for thread in threads {
        thread.join().expect("thread failed");
    }

    then.elapsed()
}

trait Queue: Default {
    fn push(&self, num: u128);
    fn pop(&self) -> Option<u128>;
}

impl Queue for LfQueue<u128> {
    fn push(&self, num: u128) {
        self.push(num)
    }

    fn pop(&self) -> Option<u128> {
        self.pop()
    }
}

impl Queue for Mutex<VecDeque<u128>> {
    fn push(&self, num: u128) {
        self.lock().unwrap().push_back(num)
    }

    fn pop(&self) -> Option<u128> {
        self.lock().unwrap().pop_front()
    }
}

impl Queue for Mutex<LinkedList<u128>> {
    fn push(&self, num: u128) {
        self.lock().unwrap().push_back(num)
    }

    fn pop(&self) -> Option<u128> {
        self.lock().unwrap().pop_front()
    }
}

fn main() {
    println!("A program which reverberates messages");

    for &nthread in &[2, 4, 8, 16] {
        println!();

        let mut deque = Duration::default();
        let mut linked = Duration::default();
        let mut lockfree = Duration::default();

        for _ in 0 .. SAMPLES {
            deque += measure::<Mutex<VecDeque<_>>>(nthread);
            linked += measure::<Mutex<LinkedList<_>>>(nthread);
            lockfree += measure::<LfQueue<_>>(nthread);
        }

        println!(
            "Mutexed VecDeque with {} threads total time: {:?}",
            nthread, deque
        );
        println!(
            "Mutexed LinkedList with {} threads total time: {:?}",
            nthread, linked
        );
        println!(
            "Lockfree Queue with {} threads total time: {:?}",
            nthread, lockfree
        );
    }
}
