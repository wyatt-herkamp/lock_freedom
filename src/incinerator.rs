use std::sync::{
    atomic::{AtomicUsize, Ordering::*},
    Arc,
};

pub(crate) use compat::incinerator::*;

#[derive(Debug)]
pub struct Pause<'counter> {
    counter: &'counter PauseCounter,
}

impl<'counter> Pause<'counter> {
    pub fn resume(self) {}
}

impl<'counter> Drop for Pause<'counter> {
    fn drop(&mut self) {
        let _res = self.counter.pauses.fetch_sub(1, Release);
        debug_assert!(_res > 0);
    }
}

#[derive(Debug, Clone, Default)]
pub struct PauseCounter {
    pauses: Arc<AtomicUsize>,
}

impl PauseCounter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pause(&self) -> Pause {
        loop {
            let init = self.pauses.load(Acquire);
            if init == usize::max_value() {
                panic!("Too much pauses")
            }
            if self.pauses.compare_and_swap(init, init + 1, Release) == init {
                break Pause { counter: self };
            }
        }
    }

    pub fn pause_with<F, T>(&self, exec: F) -> T
    where
        F: FnOnce() -> T,
    {
        let pause = self.pause();
        let ret = exec();
        pause.resume();
        ret
    }
}

#[derive(Debug, Default)]
pub struct Incinerator<G> {
    counter: PauseCounter,
    list: Vec<G>,
}

impl<G> Incinerator<G> {
    pub fn new(counter: PauseCounter) -> Self {
        Self { counter, list: Vec::new() }
    }

    pub fn add(&mut self, garbage: G) {
        if !self.try_delete() {
            // prevent garbage from being dropped
            self.list.push(garbage);
        }
    }

    pub fn try_delete(&mut self) -> bool {
        let success = self.counter.pauses.load(Acquire) == 0;
        if success {
            self.list.clear();
        }
        success
    }

    pub fn counter(&self) -> &PauseCounter {
        &self.counter
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;

    #[test]
    fn try_delete_succeeds_in_single_threaded() {
        let counter = PauseCounter::new();
        let mut incin = Incinerator::<Box<usize>>::new(counter.clone());

        assert!(incin.try_delete());

        const COUNT: usize = 16;

        let mut allocs = Vec::with_capacity(COUNT);

        for i in 0 .. COUNT {
            allocs.push(Box::new(i));
        }

        counter.pause_with(|| assert!(!incin.try_delete()));

        for boxed in allocs {
            incin.add(boxed);
        }

        assert!(incin.try_delete());
    }

    #[test]
    fn count_is_gt_0_when_pausing() {
        const NTHREADS: usize = 20;
        let counter = PauseCounter::new();
        let mut threads = Vec::with_capacity(NTHREADS);
        for _ in 0 .. NTHREADS {
            let counter = counter.clone();
            threads.push(thread::spawn(move || {
                counter.pause_with(|| {
                    assert!(counter.pauses.load(SeqCst) > 0);
                })
            }));
        }
        for thread in threads {
            thread.join().expect("sub-thread panicked");
        }
    }
}
