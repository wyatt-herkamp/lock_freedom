use alloc::*;
use std::{
    fmt,
    marker::PhantomData,
    mem,
    ptr::{null_mut, NonNull},
    sync::atomic::{AtomicPtr, Ordering::*},
};

const BITS: usize = 8;

pub struct ThreadLocal<T> {
    top: NonNull<Table<T>>,
}

impl<T> ThreadLocal<T> {
    pub fn new() -> Self {
        unsafe {
            let mut nnptr = alloc_uninit::<Table<T>>();
            nnptr.as_mut().init();
            Self { top: nnptr }
        }
    }

    pub fn with<F, A>(&self, reader: F) -> A
    where
        T: Default,
        F: FnOnce(&T) -> A,
    {
        self.with_init(T::default, reader)
    }

    pub fn with_init<I, F, A>(&self, init: I, reader: F) -> A
    where
        I: FnOnce() -> T,
        F: FnOnce(&T) -> A,
    {
        with_thread_id(|id| {
            let mut table = unsafe { &*self.top.as_ptr() };
            let mut depth = 1;
            let mut shifted = id;
            let mut init = LazyInit::Pending(move || Entry { id, val: init() });
            let mut tbl_cache = CachedAlloc::<Table<T>>::empty();

            loop {
                let index = shifted & (1 << BITS) - 1;
                let in_place = table.nodes[index].atomic.load(Acquire);

                if in_place.is_null() {
                    let nnptr = init.get();
                    let res = table.nodes[index].atomic.compare_and_swap(
                        in_place,
                        nnptr.as_ptr() as *mut (),
                        Release,
                    );

                    if res.is_null() {
                        break reader(unsafe { &nnptr.as_ref().val });
                    }
                } else if in_place as usize & 1 == 0 {
                    let entry = unsafe { &*(in_place as *mut Entry<T>) };
                    if entry.id == id {
                        debug_assert!(init.is_pending());
                        break reader(&entry.val);
                    }

                    let new_tbl_ptr = tbl_cache
                        .get_or(|mut nnptr| unsafe { nnptr.as_mut().init() });

                    let other_shifted = entry.id >> depth * BITS;
                    let other_index = other_shifted & (1 << BITS) - 1;

                    unsafe { new_tbl_ptr.as_ref() }.nodes[other_index]
                        .atomic
                        .store(in_place, Relaxed);

                    let res = table.nodes[index].atomic.compare_and_swap(
                        in_place,
                        (new_tbl_ptr.as_ptr() as usize | 1) as *mut (),
                        Release,
                    );

                    if res == in_place {
                        tbl_cache.take();
                        table = unsafe { &*new_tbl_ptr.as_ptr() };
                        depth += 1;
                        shifted >>= BITS;
                    }
                } else {
                    let table_ptr = (in_place as usize & !1) as *mut Table<T>;
                    table = unsafe { &*table_ptr };
                    depth += 1;
                    shifted >>= BITS;
                }
            }
        })
    }
}

impl<T> Drop for ThreadLocal<T> {
    fn drop(&mut self) {
        let mut tables = vec![self.top];

        while let Some(table_nnptr) = tables.pop() {
            for node in &unsafe { table_nnptr.as_ref() }.nodes as &[Node<_>] {
                let ptr = node.atomic.load(Relaxed);
                if ptr.is_null() {
                    continue;
                }

                if ptr as usize & 1 == 0 {
                    unsafe {
                        dealloc(NonNull::new_unchecked(ptr as *mut Entry<T>))
                    }
                } else {
                    let table_ptr = (ptr as usize & !1) as *mut Table<T>;

                    debug_assert!(!table_ptr.is_null());
                    tables.push(unsafe { NonNull::new_unchecked(table_ptr) });
                }
            }

            unsafe { dealloc(table_nnptr) }
        }
    }
}

unsafe impl<T> Send for ThreadLocal<T> {}
unsafe impl<T> Sync for ThreadLocal<T> {}

impl<T> fmt::Debug for ThreadLocal<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str("ThreadLocal")
    }
}

impl<T> Default for ThreadLocal<T> {
    fn default() -> Self {
        Self::new()
    }
}

struct Node<T> {
    // lower bit marked 0 for Entry, 1 for Table
    atomic: AtomicPtr<()>,
    _marker: PhantomData<T>,
}

#[repr(align(/* at least */ 2))]
struct Table<T> {
    nodes: [Node<T>; 1 << BITS],
}

impl<T> Table<T> {
    unsafe fn init(&mut self) {
        for node_ref in &mut self.nodes as &mut [_] {
            (node_ref as *mut Node<T>).write(Node {
                atomic: AtomicPtr::new(null_mut()),
                _marker: PhantomData,
            })
        }
    }
}

#[repr(align(/* at least */ 2))]
struct Entry<T> {
    id: usize,
    val: T,
}

enum LazyInit<T, F> {
    Done(NonNull<T>),
    Pending(F),
}

impl<T, F> LazyInit<T, F>
where
    F: FnOnce() -> T,
{
    fn is_pending(&self) -> bool {
        match self {
            LazyInit::Pending(_) => true,
            _ => false,
        }
    }

    fn get(&mut self) -> NonNull<T> {
        let old = mem::replace(self, LazyInit::Done(NonNull::dangling()));

        let ptr = match old {
            LazyInit::Done(ptr) => ptr,

            LazyInit::Pending(init) => unsafe { alloc(init()) },
        };

        *self = LazyInit::Done(ptr);
        ptr
    }
}

union IdMaker {
    _usize: usize,
    _f64: f64,
}

thread_local! {
    static ID: IdMaker = IdMaker {
        _usize: 0,
    };
}

fn with_thread_id<F, T>(exec: F) -> T
where
    F: FnOnce(usize) -> T,
{
    ID.with(|tpl| {
        let word = tpl as *const _ as usize;
        let align = mem::align_of::<IdMaker>();
        exec(word >> align.trailing_zeros())
    })
}

#[cfg(test)]
mod test {
    use super::ThreadLocal;
    use std::{sync::Arc, thread};

    #[test]
    fn threads_with_their_id() {
        let tls = Arc::new(ThreadLocal::new());
        let mut threads = Vec::with_capacity(32);

        for i in 0 .. 32 {
            let tls = tls.clone();
            threads.push(thread::spawn(move || {
                tls.with_init(|| i, |&x| assert_eq!(x, i));
            }))
        }

        for thread in threads {
            thread.join().unwrap();
        }
    }
}
