use owned_alloc::OwnedAlloc;
use std::{
    marker::PhantomData,
    ptr::null_mut,
    sync::atomic::{fence, AtomicBool, AtomicPtr, AtomicUsize, Ordering::*},
};

/// A cached thread-id. Repeated calls to [`ThreadLocal`](super::ThreadLocal)'s
/// methods with cached IDs should be faster than reloading the ID everytime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreadId {
    bits: usize,
    _non_tsafe: PhantomData<*mut ()>,
}

impl ThreadId {
    /// Loads the ID for this thread.
    #[inline]
    pub fn current() -> Self {
        ID.with(|id| Self {
            bits: id.node.bits,
            _non_tsafe: PhantomData,
        })
    }

    pub(super) fn bits(self) -> usize {
        self.bits
    }
}

impl Default for ThreadId {
    fn default() -> Self {
        Self::current()
    }
}

static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

static ID_LIST: Node = Node {
    bits: 0,
    free: AtomicBool::new(true),
    next: AtomicPtr::new(null_mut()),
};

static ID_LIST_BACK: AtomicPtr<Node> =
    AtomicPtr::new(&ID_LIST as *const _ as *mut _);

thread_local! {
    static ID: IdGuard = IdGuard::new();
}

fn create_node() -> &'static Node {
    let node = Node {
        bits: 0,
        free: AtomicBool::new(false),
        next: AtomicPtr::new(null_mut()),
    };

    let mut alloc = OwnedAlloc::new(node);

    // Let's change the counter after the allocation, so memory will be
    // exhausted before the counter overflows.
    alloc.bits = ID_COUNTER.fetch_add(1, Relaxed);

    let nnptr = alloc.into_raw();

    let prev = ID_LIST_BACK.swap(nnptr.as_ptr(), Release);

    // Ok because the list back is never null. Also, nodes are either static
    // variables or heap-allocations turned into static variables.
    unsafe {
        (*prev).next.store(nnptr.as_ptr(), Release);
        &*nnptr.as_ptr()
    }
}

struct IdGuard {
    node: &'static Node,
}

impl IdGuard {
    fn new() -> Self {
        // We load the back at a given time so our thread-id can be truly
        // wait-free. We won't go further than this node before creating a new
        // one. Remember nodes are never deleted.
        let back_then = ID_LIST_BACK.load(Relaxed);
        fence(Acquire);

        let mut node = &ID_LIST;

        loop {
            // First we try to acquire the current node.
            if node.free.swap(false, Relaxed) {
                break Self { node };
            }

            // Then we check if we reached the limited we loaded previously.
            if node as *const _ == back_then {
                // If so, we create a new node.
                break Self {
                    node: create_node(),
                };
            }

            // Ok because we only reach null when the test above is true and the
            // branch for the test breaks the loop. Also, nodes are either
            // static variables or heap-allocations turned into
            // static variables.
            node = unsafe { &*node.next.load(Relaxed) };
            fence(Acquire);
        }
    }
}

impl Drop for IdGuard {
    fn drop(&mut self) {
        self.node.free.store(true, Relaxed);
    }
}

#[repr(align(/* at least */ 2))]
struct Node {
    free: AtomicBool,
    bits: usize,
    next: AtomicPtr<Node>,
}
