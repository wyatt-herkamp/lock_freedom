extern crate lockfree;

use lockfree::{map::Map as LfMap, queue::Queue as LfQueue};
use std::{
    collections::{HashMap, VecDeque},
    fmt::Write,
    mem::uninitialized,
    sync::{
        atomic::{AtomicBool, Ordering::*},
        Arc,
        Mutex,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

fn prevent_opt<T>(val: T) {
    unsafe {
        let mut _local = uninitialized();
        (&mut _local as *mut T).write_volatile(val);
    }
}

#[derive(Debug)]
enum Request {
    Get(Arc<str>),
    Put(Arc<str>, Arc<str>),
    Delete(Arc<str>),
}

#[derive(Debug)]
struct ThreadHandle {
    joiner: JoinHandle<()>,
}

#[derive(Debug)]
struct Server<Q, M>
where
    Q: Queue + Send + Sync + 'static,
    M: Map + Send + Sync + 'static,
{
    pool: Vec<ThreadHandle>,
    shared: Arc<Shared<Q, M>>,
}

impl<Q, M> Server<Q, M>
where
    Q: Queue + Send + Sync + 'static,
    M: Map + Send + Sync + 'static,
{
    fn new(threads: usize) -> Self {
        if threads == 0 {
            panic!("Cannot have the server with 0 threads")
        }

        let mut this = Self {
            pool: Vec::with_capacity(threads),
            shared: Arc::new(Shared::default()),
        };

        for _ in 0 .. threads {
            let shared = this.shared.clone();
            let joiner = thread::spawn(move || {
                while !shared.die_flag.load(Acquire) {
                    while let Some(request) = shared.requests.pop() {
                        match request {
                            Request::Put(key, val) => shared.map.put(key, val),

                            Request::Get(key) => {
                                prevent_opt(shared.map.get(&key))
                            },

                            Request::Delete(key) => {
                                prevent_opt(shared.map.delete(&key))
                            },
                        }
                    }
                }
            });

            this.pool.push(ThreadHandle { joiner });
        }

        this
    }

    fn send_request(&self, request: Request) {
        self.shared.requests.push(request)
    }

    fn send_default_requests(&self) {
        const STEP1: usize = 2000;
        const STEP2: usize = 200;
        const SLEEP1: u64 = 52;
        const SLEEP2: u64 = 2;

        let mut pairs = Vec::with_capacity(STEP1);
        for i in 0 .. STEP1 {
            let mut key = String::with_capacity(STEP1 * 2);
            for j in 0 .. STEP1 {
                write!(key, "{}{}", i * j, (i + j) as u8 as char).unwrap();
            }
            let mut val = format!("{}", i);
            pairs.push((key, val));
        }

        while let Some((key, val)) = pairs.pop() {
            let key = Arc::<str>::from(key);
            self.send_request(Request::Put(key.clone(), val.into()));
            self.send_request(Request::Get(key.clone()));
            self.send_request(Request::Get(key.clone()));
            self.send_request(Request::Get(key.clone()));
            self.send_request(Request::Delete(key.clone()));
            self.send_request(Request::Delete(key));
        }

        thread::sleep(Duration::from_millis(SLEEP1));

        for i in 0 .. STEP2 {
            let mut key = String::with_capacity(STEP2 * 2);
            for j in 0 .. STEP2 {
                write!(key, "{}{}", (i + j) as u8 as char, (i + j * 2))
                    .unwrap();
            }
            let mut val = format!("{}", i);
            pairs.push((key, val));
        }

        while let Some((key, val)) = pairs.pop() {
            let key = Arc::<str>::from(key);
            self.send_request(Request::Put(key.clone(), val.into()));
            self.send_request(Request::Get(key.clone()));
            self.send_request(Request::Get(key.clone()));
            self.send_request(Request::Delete(key.clone()));
            self.send_request(Request::Delete(key));
        }

        thread::sleep(Duration::from_millis(SLEEP2));
        self.send_request(Request::Delete(Arc::from("not in the map")));
    }

    fn await_threads(&mut self) {
        self.shared.die_flag.store(true, Release);
        while let Some(handle) = self.pool.pop() {
            handle.joiner.join().expect("thread failed");
        }
    }

    fn measure(threads: usize) -> Duration {
        let mut this = Self::new(threads);
        let then = Instant::now();
        this.send_default_requests();
        this.await_threads();
        then.elapsed()
    }
}

impl<Q, M> Drop for Server<Q, M>
where
    Q: Queue + Send + Sync + 'static,
    M: Map + Send + Sync + 'static,
{
    fn drop(&mut self) {
        self.await_threads()
    }
}

#[derive(Default, Debug)]
struct Shared<Q, M> {
    requests: Q,
    map: M,
    die_flag: AtomicBool,
}

trait Map: Default {
    fn put(&self, key: Arc<str>, val: Arc<str>);
    fn get(&self, key: &str) -> Option<Arc<str>>;
    fn delete(&self, key: &str) -> Option<Arc<str>>;
}

impl Map for LfMap<Arc<str>, Arc<str>> {
    fn put(&self, key: Arc<str>, val: Arc<str>) {
        self.insert(key, val);
    }

    fn get(&self, key: &str) -> Option<Arc<str>> {
        self.get(key).map(|guard| guard.key().clone())
    }

    fn delete(&self, key: &str) -> Option<Arc<str>> {
        self.remove(key).map(|rem| rem.key().clone())
    }
}

impl Map for Mutex<HashMap<Arc<str>, Arc<str>>> {
    fn put(&self, key: Arc<str>, val: Arc<str>) {
        self.lock().unwrap().insert(key, val);
    }

    fn get(&self, key: &str) -> Option<Arc<str>> {
        self.lock().unwrap().get(key).map(Clone::clone)
    }

    fn delete(&self, key: &str) -> Option<Arc<str>> {
        self.lock().unwrap().remove(key)
    }
}

trait Queue: Default {
    fn push(&self, request: Request);
    fn pop(&self) -> Option<Request>;
}

impl Queue for LfQueue<Request> {
    fn push(&self, request: Request) {
        self.push(request)
    }

    fn pop(&self) -> Option<Request> {
        self.pop()
    }
}

impl Queue for Mutex<VecDeque<Request>> {
    fn push(&self, request: Request) {
        self.lock().unwrap().push_back(request)
    }

    fn pop(&self) -> Option<Request> {
        self.lock().unwrap().pop_front()
    }
}

fn main() {
    println!("A program simulating a concurrent server.");

    for &nthread in &[2, 4, 8, 16] {
        println!();
        println!(
            "Mutexed structures with {} threads total time: {:?}",
            nthread,
            Server::<Mutex<_>, Mutex<_>>::measure(nthread)
        );
        println!(
            "Lockfree structures with {} threads total time: {:?}",
            nthread,
            Server::<LfQueue<_>, LfMap<_, _>>::measure(nthread)
        );
    }
}
