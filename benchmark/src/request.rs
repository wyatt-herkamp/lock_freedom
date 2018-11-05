extern crate lockfree;

use lockfree::{map::Map as LfMap, queue::Queue as LfQueue};
use std::{
    collections::{HashMap, LinkedList, VecDeque},
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

    fn send_default_requests(
        &self,
        step1: usize,
        step2: usize,
        nest1: usize,
        nest2: usize,
        sleep1: u64,
        sleep2: u64,
    ) {
        let mut pairs = Vec::with_capacity(step1);
        for i in 0 .. step1 {
            let mut key = String::with_capacity(step1 * nest1 * 2 + 1);
            for j in 0 .. nest1 {
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

        thread::sleep(Duration::from_millis(sleep1));

        for i in 0 .. step2 {
            let mut key = String::with_capacity(nest2 * step2 * 2 + 1);
            for j in 0 .. nest2 {
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

        thread::sleep(Duration::from_millis(sleep2));
        self.send_request(Request::Delete(Arc::from("not in the map")));
    }

    fn await_threads(&mut self) {
        self.shared.die_flag.store(true, Release);
        while let Some(handle) = self.pool.pop() {
            handle.joiner.join().expect("thread failed");
        }
    }

    fn measure(
        threads: usize,
        step1: usize,
        step2: usize,
        nest1: usize,
        nest2: usize,
        sleep1: u64,
        sleep2: u64,
    ) -> Duration {
        let mut this = Self::new(threads);
        let then = Instant::now();
        this.send_default_requests(step1, step2, nest1, nest2, sleep1, sleep2);
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

impl Queue for Mutex<LinkedList<Request>> {
    fn push(&self, request: Request) {
        self.lock().unwrap().push_back(request)
    }

    fn pop(&self) -> Option<Request> {
        self.lock().unwrap().pop_front()
    }
}

fn main() {
    println!("A program simulating a concurrent server.");

    const STEP1: usize = 0x20000;
    const STEP2: usize = 0x4000;
    const NEST1: usize = 5;
    const NEST2: usize = 10;
    const SLEEP1: u64 = 62;
    const SLEEP2: u64 = 2;

    for &nthread in &[2, 4, 8, 16] {
        println!();

        let mut deque = Server::<Mutex<VecDeque<_>>, Mutex<_>>::measure(
            nthread, STEP1, STEP2, NEST1, NEST2, SLEEP1, SLEEP2,
        );
        let mut linked = Server::<Mutex<LinkedList<_>>, Mutex<_>>::measure(
            nthread, STEP1, STEP2, NEST1, NEST2, SLEEP1, SLEEP2,
        );
        let mut lockfree = Server::<LfQueue<_>, LfMap<_, _>>::measure(
            nthread, STEP1, STEP2, NEST1, NEST2, SLEEP1, SLEEP2,
        );

        println!(
            "Mutexed HashMap and VecDeque with {} threads total time: {:?}",
            nthread, deque
        );
        println!(
            "Mutexed HashMap and LinkedList with {} threads total time: {:?}",
            nthread, linked
        );
        println!(
            "Lockfree structures with {} threads total time: {:?}",
            nthread, lockfree
        );
    }
}
