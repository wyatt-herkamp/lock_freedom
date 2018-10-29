use std::{
    cell::UnsafeCell,
    mem::replace,
    sync::atomic::{AtomicUsize, Ordering::*},
};
use tls::ThreadLocal;

pub use compat::incinerator::*;

#[derive(Debug)]
pub struct Incinerator<T> {
    counter: AtomicUsize,
    tls_list: ThreadLocal<UnsafeCell<Vec<T>>>,
}

impl<T> Incinerator<T> {
    pub fn new() -> Self {
        Self { counter: AtomicUsize::new(0), tls_list: ThreadLocal::new() }
    }

    pub fn pause(&self) -> Pause<T> {
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

    pub fn pause_with<F, A>(&self, exec: F) -> A
    where
        F: FnOnce() -> A,
    {
        let pause = self.pause();
        let ret = exec();
        pause.resume();
        ret
    }

    pub fn add<U>(&self, val: U)
    where
        U: Into<T>,
    {
        if self.counter.load(Acquire) == 0 {
            self.tls_list.with(|cell| {
                let mut list = replace(unsafe { &mut *cell.get() }, Vec::new());
                list.clear();
            });
        } else {
            self.tls_list.with_init(
                || UnsafeCell::new(Vec::new()),
                |cell| {
                    let list = unsafe { &mut *cell.get() };
                    list.push(val.into());
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
pub struct Pause<'incin, T>
where
    T: 'incin,
{
    incin: &'incin Incinerator<T>,
}

impl<'incin, T> Pause<'incin, T> {
    fn resume(self) {}
}

impl<'incin, T> Drop for Pause<'incin, T> {
    fn drop(&mut self) {
        self.incin.counter.fetch_sub(1, Release);
    }
}

impl<T> Default for Incinerator<T> {
    fn default() -> Self {
        Self::new()
    }
}
