use std::{
    fmt,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering::*},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

pub const ITER_PER_TRY: usize = 0x400;

pub trait Target: Clone + Send + 'static {
    #[inline(always)]
    fn round(&mut self);
}

pub trait TargetSet {
    fn run(&self, executor: &mut Executor);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Stat {
    duration: Duration,
    rounds: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Executor {
    stats: Vec<Stat>,
    threads: usize,
}

impl Stat {
    pub fn duration(self) -> Duration {
        self.duration
    }

    pub fn rounds(self) -> usize {
        self.rounds
    }

    pub fn seconds(self) -> f64 {
        let secs = self.duration.as_secs() as f64;
        let nanos = self.duration.subsec_nanos() as u64 as f64;
        secs + nanos / 1_000_000_000.0
    }

    pub fn rounds_per_sec(self) -> f64 {
        self.rounds as u64 as f64 / self.seconds()
    }
}

impl fmt::Display for Stat {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "mean of {:.3} r/s ({} rounds in {:.3} seconds)",
            self.rounds_per_sec(),
            self.rounds,
            self.seconds()
        )
    }
}

impl Executor {
    pub fn new(threads: usize) -> Self {
        if threads == 0 {
            panic!("Cannot have zero threads to be benchmarked")
        }
        if threads == usize::max_value() {
            panic!("too much threads")
        }
        Self {
            stats: Vec::new(),
            threads,
        }
    }

    pub fn threads(&self) -> usize {
        self.threads
    }

    pub fn stats(&self) -> &[Stat] {
        &self.stats
    }

    pub fn run<T>(&mut self, target: &T)
    where
        T: Target,
    {
        let mut threads = Vec::new();
        let count = Arc::new(AtomicUsize::new(0));
        let exit = Arc::new(AtomicBool::new(false));

        for _ in 0 .. self.threads {
            let mut target = target.clone();
            let exit = exit.clone();
            let count = count.clone();
            threads.push(thread::spawn(move || {
                while !exit.load(Acquire) {
                    for _ in 0 .. ITER_PER_TRY {
                        target.round();
                    }
                    count.fetch_add(ITER_PER_TRY, Relaxed);
                }
            }))
        }

        let start = Instant::now();
        thread::sleep(Duration::from_millis(1250));
        exit.store(true, Release);
        for thread in threads {
            thread.join().unwrap()
        }
        let total = start.elapsed();

        self.stats.push(Stat {
            duration: total,
            rounds: count.load(Relaxed),
        });
    }
}

impl<A> TargetSet for A
where
    A: Target,
{
    fn run(&self, executor: &mut Executor) {
        executor.run(self)
    }
}

macro_rules! make_field {
    () => {};
    ($field:expr $(, $fields:expr)*) => {
        make_field!($($fields),*);
        $field;
    }
}

macro_rules! impl_tuple {
    ($($ty:ident : $field:tt),*) => {
        impl<$($ty),*> TargetSet for ($($ty,)*)
        where
            $($ty: Target,)*
        {
            fn run(&self, _executor: &mut Executor) {
                make_field!($(_executor.run(&self.$field)),*);
            }
        }
    };
}

macro_rules! impl_tuples {
    () => {
        impl_tuple!();
    };

    ($ty:ident : $field:tt $(, $tys:ident : $fields:tt)*) => {
        impl_tuple!($ty : $field $(, $tys : $fields)*);
        impl_tuples!($($tys : $fields),*);
    };
}
impl_tuples! {
    P : 15,
    O : 14,
    N : 13,
    M : 12,
    L : 11,
    K : 10,
    J : 9,
    I : 8,
    H : 7,
    G : 6,
    F : 5,
    E : 4,
    D : 3,
    C : 2,
    B : 1,
    A : 0
}
