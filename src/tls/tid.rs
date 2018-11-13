use owned_alloc::OwnedAlloc;
use std::{
    marker::PhantomData,
    ptr::null_mut,
    sync::atomic::{AtomicBool, AtomicPtr, AtomicUsize, Ordering::*},
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

fn create_id() -> &'static Node {
    let node = Node {
        bits: ID_COUNTER.fetch_add(1, Relaxed),
        free: AtomicBool::new(false),
        next: AtomicPtr::new(null_mut()),
    };
    let alloc = OwnedAlloc::new(node);
    let nnptr = alloc.into_raw();
    let prev = ID_LIST_BACK.swap(nnptr.as_ptr(), Release);
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
        let mut node = &ID_LIST;
        let back_then = ID_LIST_BACK.load(Acquire);

        loop {
            if node.free.swap(false, Relaxed) {
                break Self { node };
            }

            if node as *const _ == back_then {
                break Self { node: create_id() };
            }

            node = unsafe { &*node.next.load(Acquire) };
        }
    }
}

impl Drop for IdGuard {
    fn drop(&mut self) {
        self.node.free.store(true, Release);
    }
}

#[repr(align(/* at least */ 2))]
struct Node {
    free: AtomicBool,
    bits: usize,
    next: AtomicPtr<Node>,
}
