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
/// thread exited, the data might be reused for other threads. This TLS's
/// operation are also wait-free.
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
        Self {
            top: Table::new_alloc(),
        }
    }

    /// Removes and drops all entries. The TLS is considered empty then. This
    /// method is only available with exclusive references. This method is
    /// merely for optimization since the TLS is cleared at drop.
    pub fn clear(&mut self) {
        let mut tables = Vec::new();

        // Method clear means we are also resetting all node pointers to null.
        self.top.clear(&mut tables);

        while let Some(mut table) = tables.pop() {
            // Method free_nodes means we are only freeing node pointers but not
            // clearing them.
            table.free_nodes(&mut tables);
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
                // The index of the node for our id.
                let index = shifted & (1 << BITS) - 1;

                // Load what is in there.
                let in_place = table.nodes[index].atomic.load(Acquire);

                // Null means there is nothing.
                if in_place.is_null() {
                    break None;
                }

                // Having in_place's lower bit set to 0 means it is a
                // pointer to entry.
                if in_place as usize & 1 == 0 {
                    let entry = unsafe { &*(in_place as *mut Entry<T>) };
                    break if entry.id == id {
                        // We only have an entry for the thread if the ids
                        // match.
                        Some(reader(&entry.val))
                    } else {
                        None
                    };
                }

                // The remaining case (non-null with lower bit set to 1) means
                // we have a child table.
                // Clear the pointer first lower bit so we can dereference it.
                let table_ptr = (in_place as usize & !1) as *mut Table<T>;
                // Set it as the table to be checked in the next iteration.
                table = unsafe { &*table_ptr };
                // Shift our "hash" for the next level.
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
            // The depth of the iterations.
            let mut depth = 1;
            let mut shifted = id;
            // Using `LazyInit` to make sure we only initialize if there is no
            // entry.
            let mut init = LazyInit::Pending(move || Entry { id, val: init() });
            let mut tbl_cache = Cache::<OwnedAlloc<Table<T>>>::new();

            loop {
                // The index of the node for our id.
                let index = shifted & (1 << BITS) - 1;
                // First of all, let's check what is stored.
                let in_place = table.nodes[index].atomic.load(Acquire);

                if in_place.is_null() {
                    // Null means we have an empty node and also our thread has
                    // not stored anything. Let's initialize.
                    let nnptr = init.get();
                    // First lower bit set to 0 means this is a pointer to
                    // entry. This should be guaranteed by the alignment,
                    // however, always good to ensure it.
                    debug_assert!(nnptr.as_ptr() as usize & 1 == 0);

                    // Trying to publish our freshly created entry.
                    let res = table.nodes[index].atomic.compare_and_swap(
                        in_place,
                        nnptr.as_ptr() as *mut (),
                        Release,
                    );

                    if res.is_null() {
                        // If the stored value still was null, we succeeded.
                        // Let's read the entry.
                        break reader(unsafe { &nnptr.as_ref().val });
                    }
                } else if in_place as usize & 1 == 0 {
                    // First lower bit set to 0 means we have an entry.
                    let entry = unsafe { &*(in_place as *mut Entry<T>) };
                    // If ids match, this is the entry for our thread.
                    if entry.id == id {
                        // There is no possible way we have initialized the
                        // `LazyInit`. It will only happen if we found an empty
                        // node while searching, and the only way of putting a
                        // non-empty node is either we put it or some other
                        // thread (with different id obviously) put it.
                        debug_assert!(init.is_pending());
                        // And let's read it...
                        break reader(&entry.val);
                    }

                    // Get a table allocation from the cache.
                    let new_tbl = tbl_cache.take_or(Table::new_alloc);

                    // Calculate index for the collided entry.
                    let other_shifted = entry.id >> depth * BITS;
                    let other_index = other_shifted & (1 << BITS) - 1;

                    // Pre-insert it in the table from the cache.
                    new_tbl.nodes[other_index].atomic.store(in_place, Relaxed);

                    // Forget about the owned allocation and turn it into a
                    // pointer.
                    let new_tbl_ptr = new_tbl.into_raw();

                    // Let's try to publish our work.
                    let res = table.nodes[index].atomic.compare_and_swap(
                        in_place,
                        // First lower bit set to 1 means it is a table
                        // pointer.
                        (new_tbl_ptr.as_ptr() as usize | 1) as *mut (),
                        Release,
                    );

                    if res == in_place {
                        // If the old node was still stored, we succeeded.
                        // Let's set the new table as the table for the next
                        // iteration.
                        table = unsafe { &*new_tbl_ptr.as_ptr() };
                        // We are going one depth further.
                        depth += 1;
                        // Shift our "hash" for the next level.
                        shifted >>= BITS;
                    } else {
                        // If we failed, let's rebuild the owned allocation.
                        let new_tbl =
                            unsafe { OwnedAlloc::from_raw(new_tbl_ptr) };
                        // Clear that pre-inserted node.
                        new_tbl.nodes[other_index]
                            .atomic
                            .store(null_mut(), Relaxed);

                        // Store it into the cache for later.
                        tbl_cache.store(new_tbl);
                    }
                } else {
                    // The remaining case (non-null with first lower bit set to
                    // 1) is a table. Clear the pointer first lower bit so we
                    // can dereference it.
                    let table_ptr = (in_place as usize & !1) as *mut Table<T>;
                    // Set it as table for the next iteration.
                    table = unsafe { &*table_ptr };
                    // We are going one depth further.
                    depth += 1;
                    // Shift our "hash" for the next level.
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

        // Method free_nodes means we are only freeing node pointers but not
        // clearing them (no need to clear since nobody will ever use them
        // again, we are dropping the TLS).
        self.top.free_nodes(&mut tables);

        while let Some(mut table) = tables.pop() {
            table.free_nodes(&mut tables);
        }
    }
}

unsafe impl<T> Send for ThreadLocal<T> where T: Send {}
unsafe impl<T> Sync for ThreadLocal<T> where T: Send {}

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
    fn free_nodes(&mut self, tbl_stack: &mut Vec<OwnedAlloc<Table<T>>>) {
        for node in &self.nodes as &[Node<_>] {
            unsafe { Node::free_ptr(node.atomic.load(Relaxed), tbl_stack) };
        }
    }

    #[inline]
    fn clear(&mut self, tbl_stack: &mut Vec<OwnedAlloc<Table<T>>>) {
        for node in &self.nodes as &[Node<_>] {
            unsafe {
                Node::free_ptr(
                    node.atomic.swap(null_mut(), Relaxed),
                    tbl_stack,
                );
            }
        }
    }
}

#[repr(align(/* at least */ 64))]
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
