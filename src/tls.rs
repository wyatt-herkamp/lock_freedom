use owned_alloc::{Cache, OwnedAlloc, UninitAlloc};
use std::{
    fmt,
    marker::PhantomData,
    mem,
    ptr::{null_mut, NonNull},
    sync::atomic::{AtomicPtr, Ordering::*},
};

const BITS: usize = 8;

/// Per Object Thread Local Storage. The stored data is not dropped on thread
/// exit. It is only dropped when the structure itself is dropped. After the
/// thread exited, the data might be reused for other threads.
///
/// # Example
/// ```
/// extern crate lockfree;
///
/// use lockfree::tls::ThreadLocal;
/// use std::{sync::Arc, thread};
///
/// let tls = Arc::new(ThreadLocal::new());
/// let mut threads = Vec::with_capacity(32);
///
/// for i in 1 ..= 32 {
///     let tls = tls.clone();
///     threads.push(thread::spawn(move || {
///         let stored = if i == 32 {
///             tls.with_default(|&x| {
///                 assert_eq!(x, 0);
///                 x
///             })
///         } else {
///             tls.with_init(
///                 || i,
///                 |&x| {
///                     assert_eq!(x, i);
///                     x
///                 },
///             )
///         };
///
///         if tls.with(|&x| assert_eq!(x, stored)).is_none() {
///             eprintln!("Warning: OS mis-reset the global thread state")
///         }
///     }))
/// }
///
/// for thread in threads {
///     thread.join().unwrap();
/// }
/// ```
pub struct ThreadLocal<T> {
    top: OwnedAlloc<Table<T>>,
}

impl<T> ThreadLocal<T> {
    /// Creates an empty thread local storage.
    pub fn new() -> Self {
        Self { top: Table::new_alloc() }
    }

    /// Removes and drops all entries. The TLS is considered empty then. This
    /// method is only available with exclusive references. This method is
    /// merely for optimization since the TLS is cleared at drop.
    pub fn clear(&mut self) {
        let mut tables = Vec::new();

        unsafe { self.top.clear(&mut tables) }

        while let Some(table) = tables.pop() {
            unsafe { table.free_nodes(&mut tables) }
        }
    }

    /// Accesses the entry for the current thread. No initialization is
    /// performed.
    pub fn with<F, A>(&self, reader: F) -> Option<A>
    where
        F: FnOnce(&T) -> A,
    {
        with_thread_id(|id| {
            let mut table = &*self.top;
            let mut shifted = id;

            loop {
                let index = shifted & (1 << BITS) - 1;
                let in_place = table.nodes[index].atomic.load(Acquire);

                if in_place.is_null() {
                    break None;
                }

                if in_place as usize & 1 == 0 {
                    let entry = unsafe { &*(in_place as *mut Entry<T>) };
                    break if entry.id == id {
                        Some(reader(&entry.val))
                    } else {
                        None
                    };
                }

                let table_ptr = (in_place as usize & !1) as *mut Table<T>;
                table = unsafe { &*table_ptr };
                shifted >>= BITS;
            }
        })
    }

    /// Accesses the entry for the current thread. If necessary, the `init`
    /// closure is called to initialize the entry.
    pub fn with_init<I, F, A>(&self, init: I, reader: F) -> A
    where
        I: FnOnce() -> T,
        F: FnOnce(&T) -> A,
    {
        with_thread_id(|id| {
            let mut table = &*self.top;
            let mut depth = 1;
            let mut shifted = id;
            let mut init = LazyInit::Pending(move || Entry { id, val: init() });
            let mut tbl_cache = Cache::<OwnedAlloc<Table<T>>>::new();

            loop {
                let index = shifted & (1 << BITS) - 1;
                let in_place = table.nodes[index].atomic.load(Acquire);

                if in_place.is_null() {
                    let nnptr = init.get();
                    debug_assert!(nnptr.as_ptr() as usize & 1 == 0);

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

                    let new_tbl = tbl_cache.take_or(Table::new_alloc);

                    let other_shifted = entry.id >> depth * BITS;
                    let other_index = other_shifted & (1 << BITS) - 1;

                    new_tbl.nodes[other_index].atomic.store(in_place, Relaxed);

                    let new_tbl_ptr = new_tbl.into_raw();

                    let res = table.nodes[index].atomic.compare_and_swap(
                        in_place,
                        (new_tbl_ptr.as_ptr() as usize | 1) as *mut (),
                        Release,
                    );

                    if res == in_place {
                        table = unsafe { &*new_tbl_ptr.as_ptr() };
                        depth += 1;
                        shifted >>= BITS;
                    } else {
                        let new_tbl =
                            unsafe { OwnedAlloc::from_raw(new_tbl_ptr) };
                        new_tbl.nodes[other_index]
                            .atomic
                            .store(null_mut(), Relaxed);
                        tbl_cache.store(new_tbl);
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

    /// Accesses the entry for the current thread. If necessary, the entry is
    /// initialized with default value.
    pub fn with_default<F, A>(&self, reader: F) -> A
    where
        T: Default,
        F: FnOnce(&T) -> A,
    {
        self.with_init(T::default, reader)
    }
}

impl<T> Drop for ThreadLocal<T> {
    fn drop(&mut self) {
        let mut tables = Vec::new();

        unsafe { self.top.free_nodes(&mut tables) }

        while let Some(table) = tables.pop() {
            unsafe { table.free_nodes(&mut tables) }
        }
    }
}

unsafe impl<T> Send for ThreadLocal<T> {}
unsafe impl<T> Sync for ThreadLocal<T> {}

impl<T> fmt::Debug for ThreadLocal<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "ThreadLocal {} storage: ", '{')?;
        self.with(|val| write!(fmtr, "Some({:?})", val))
            .unwrap_or_else(|| write!(fmtr, "None"))?;
        write!(fmtr, "{}", '}')
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

impl<T> Node<T> {
    unsafe fn free_ptr(
        ptr: *mut (),
        tbl_stack: &mut Vec<OwnedAlloc<Table<T>>>,
    ) {
        if ptr.is_null() {
            return;
        }

        if ptr as usize & 1 == 0 {
            OwnedAlloc::from_raw(NonNull::new_unchecked(ptr as *mut Entry<T>));
        } else {
            let table_ptr = (ptr as usize & !1) as *mut Table<T>;

            debug_assert!(!table_ptr.is_null());
            tbl_stack
                .push(OwnedAlloc::from_raw(NonNull::new_unchecked(table_ptr)));
        }
    }
}

#[repr(align(/* at least */ 2))]
struct Table<T> {
    nodes: [Node<T>; 1 << BITS],
}

impl<T> Table<T> {
    #[inline]
    fn new_alloc() -> OwnedAlloc<Self> {
        unsafe { UninitAlloc::<Self>::new().init_in_place(|this| this.init()) }
    }

    #[inline]
    unsafe fn init(&mut self) {
        for node_ref in &mut self.nodes as &mut [_] {
            (node_ref as *mut Node<T>).write(Node {
                atomic: AtomicPtr::new(null_mut()),
                _marker: PhantomData,
            })
        }
    }

    #[inline]
    unsafe fn free_nodes(&self, tbl_stack: &mut Vec<OwnedAlloc<Table<T>>>) {
        for node in &self.nodes as &[Node<_>] {
            Node::free_ptr(node.atomic.load(Relaxed), tbl_stack);
        }
    }

    #[inline]
    unsafe fn clear(&self, tbl_stack: &mut Vec<OwnedAlloc<Table<T>>>) {
        for node in &self.nodes as &[Node<_>] {
            Node::free_ptr(node.atomic.swap(null_mut(), Relaxed), tbl_stack);
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

            LazyInit::Pending(init) => OwnedAlloc::new(init()).into_raw(),
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
