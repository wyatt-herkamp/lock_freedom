use owned_alloc::OwnedAlloc;
use std::{
    marker::PhantomData,
    ptr::{null_mut, NonNull},
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

static COUNTER: AtomicUsize = AtomicUsize::new(0);
static ID_LIST: AtomicPtr<Node> = AtomicPtr::new(null_mut());

thread_local! {
    static ID: IdGuard = IdGuard::new();
}

struct IdGuard {
    node: &'static Node,
}

impl IdGuard {
    fn new() -> Self {
        let mut loaded = ID_LIST.load(Acquire);

        while let Some(nnptr) = NonNull::new(loaded) {
            let node = unsafe { &*nnptr.as_ptr() };
            if node.free.swap(false, Acquire) {
                return Self { node };
            }
            loaded = node.next.load(Acquire);
        }

        let free = AtomicBool::new(false);
        let bits = COUNTER.fetch_add(1, Relaxed);
        let next = AtomicPtr::new(null_mut());
        let nnptr = OwnedAlloc::new(Node { free, bits, next }).into_raw();
        let node = unsafe { &*nnptr.as_ptr() };

        let in_place = ID_LIST.swap(nnptr.as_ptr(), Relaxed);
        node.next.store(in_place, Release);

        Self { node }
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
