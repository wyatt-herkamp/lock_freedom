use std::{
    cell::UnsafeCell,
    mem::replace,
    sync::atomic::{AtomicUsize, Ordering::*},
};
use tls::ThreadLocal;

pub use compat::incinerator::*;

#[derive(Debug)]
pub struct Incinerator<G>
where
    G: GarbageList,
{
    counter: AtomicUsize,
    tls_list: ThreadLocal<UnsafeCell<G>>,
}

impl<G> Incinerator<G>
where
    G: GarbageList,
{
    pub fn new() -> Self {
        Self { counter: AtomicUsize::new(0), tls_list: ThreadLocal::new() }
    }

    pub fn pause(&self) -> Pause<G> {
        loop {
            let init = self.counter.load(Acquire);
            if init == usize::max_value() {
                panic!("Too many pauses");
            }
            if self.counter.compare_and_swap(init, init + 1, Release) == init {
                break Pause { incin: self };
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

    pub fn add<T>(&self, val: T)
    where
        T: Into<G::Garbage>,
    {
        if self.counter.load(Acquire) == 0 {
            self.tls_list.with(|cell| {
                let mut list = replace(unsafe { &mut *cell.get() }, G::empty());
                list.clear();
            });
        } else {
            self.tls_list.with_init(
                || UnsafeCell::new(G::empty()),
                |cell| {
                    let list = unsafe { &mut *cell.get() };
                    list.add(val.into());
                },
            );
        }
    }

    pub fn try_clear(&self) {
        if self.counter.load(Acquire) == 0 {
            self.tls_list.with(|cell| unsafe { &mut *cell.get() }.clear());
        }
    }
}

#[derive(Debug)]
pub struct Pause<'incin, G>
where
    G: GarbageList + 'incin,
{
    incin: &'incin Incinerator<G>,
}

impl<'incin, G> Pause<'incin, G>
where
    G: GarbageList,
{
    fn resume(self) {}
}

impl<'incin, G> Drop for Pause<'incin, G>
where
    G: GarbageList,
{
    fn drop(&mut self) {
        self.incin.counter.fetch_sub(1, Release);
    }
}

impl<G> Default for Incinerator<G>
where
    G: GarbageList,
{
    fn default() -> Self {
        Self::new()
    }
}

pub unsafe trait GarbageList: Sized {
    type Garbage;

    fn empty() -> Self;

    fn clear(&mut self);

    fn add(&mut self, val: Self::Garbage);
}

unsafe impl<T> GarbageList for Vec<T> {
    type Garbage = T;

    fn empty() -> Self {
        Self::new()
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn add(&mut self, val: Self::Garbage) {
        self.push(val);
    }
}
