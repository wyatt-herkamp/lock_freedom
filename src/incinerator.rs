use std::{
    cell::Cell,
    fmt,
    mem::replace,
    ptr::null,
    sync::{
        atomic::{AtomicUsize, Ordering::*},
        Arc,
    },
};
use tls::ThreadLocal;

pub use compat::incinerator::*;

#[derive(Debug, Clone)]
pub struct Incinerator<'garbage> {
    inner: Arc<IncinInner<'garbage>>,
}

impl<'garbage> Incinerator<'garbage> {
    pub fn new() -> Self {
        Self { inner: Arc::new(IncinInner::new()) }
    }

    pub fn pause<'incin>(&'incin self) -> Pause<'incin, 'garbage> {
        loop {
            let init = self.inner.counter.load(Acquire);
            if init == usize::max_value() {
                panic!("Too many pauses");
            }

            let res =
                self.inner.counter.compare_and_swap(init, init + 1, Release);
            if res == init {
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
        T: Garbage + 'garbage,
    {
        if self.inner.counter.load(Acquire) == 0 {
            self.inner.tls_list.with(GarbageList::clear);
        } else {
            self.inner
                .tls_list
                .with_init(GarbageList::new, |list| list.add(Box::new(val)));
        }
    }

    pub fn try_clear(&self) {
        if self.inner.counter.load(Acquire) == 0 {
            self.inner.tls_list.with(GarbageList::clear);
        }
    }
}

#[derive(Debug)]
pub struct Pause<'incin, 'garbage> {
    incin: &'incin Incinerator<'garbage>,
}

impl<'incin, 'garbage> Pause<'incin, 'garbage> {
    fn resume(self) {}
}

impl<'incin, 'garbage> Drop for Pause<'incin, 'garbage> {
    fn drop(&mut self) {
        self.incin.inner.counter.fetch_sub(1, Release);
    }
}

impl<'garbage> Default for Incinerator<'garbage> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Garbage {
    fn destroy(self: Box<Self>);
}

impl<F> Garbage for F
where
    F: FnOnce(),
{
    fn destroy(self: Box<Self>) {
        (*self)()
    }
}

impl<'garbage> fmt::Debug for Box<dyn Garbage + 'garbage> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{:p}", self)
    }
}

#[derive(Debug)]
struct IncinInner<'garbage> {
    counter: AtomicUsize,
    tls_list: ThreadLocal<GarbageList<'garbage>>,
}

impl<'garbage> IncinInner<'garbage> {
    fn new() -> Self {
        IncinInner {
            counter: AtomicUsize::new(0),
            tls_list: ThreadLocal::new(),
        }
    }
}

struct GarbageList<'garbage> {
    list: Cell<Vec<Box<dyn Garbage + 'garbage>>>,
}

impl<'garbage> GarbageList<'garbage> {
    fn new() -> Self {
        Self { list: Cell::new(Vec::new()) }
    }

    fn add(&self, obj: Box<dyn Garbage + 'garbage>) {
        let mut list = self.list.replace(Vec::new());
        list.push(obj);
        self.list.set(list);
    }

    fn clear(&self) {
        let list = self.list.replace(Vec::new());

        for garbage in list {
            garbage.destroy();
        }
    }
}

impl<'garbage> fmt::Debug for GarbageList<'garbage> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let list = self.list.replace(Vec::new());
        write!(fmtr, "{:?}", list)?;
        self.list.set(list);
        Ok(())
    }
}

impl<'garbage> Drop for GarbageList<'garbage> {
    fn drop(&mut self) {
        self.clear();
    }
}
