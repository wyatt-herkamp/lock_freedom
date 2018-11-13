use owned_alloc::{Cache, OwnedAlloc, UninitAlloc};
use std::{
    marker::PhantomData,
    ptr::null_mut,
    sync::atomic::{AtomicBool, AtomicPtr, Ordering::*},
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

static ID_LIST: Node = Node {
    bits: 0,
    free: AtomicBool::new(true),
    next: AtomicPtr::new(null_mut()),
};

thread_local! {
    static ID: IdGuard = IdGuard::new();
}

struct IdGuard {
    node: &'static Node,
}

impl IdGuard {
    fn new() -> Self {
        let mut cache = Cache::new();
        let mut node = &ID_LIST;

        loop {
            if node.free.swap(false, Acquire) {
                break Self { node };
            }

            match unsafe { node.next.load(Acquire).as_ref() } {
                Some(refr) => node = refr,
                None => {
                    let alloc = cache.take_or(UninitAlloc::new).init(Node {
                        bits: node
                            .bits
                            .checked_add(1)
                            .expect("too much threads"),
                        next: AtomicPtr::new(null_mut()),
                        free: AtomicBool::new(false),
                    });

                    let nnptr = alloc.into_raw();
                    let res = node.next.compare_and_swap(
                        null_mut(),
                        nnptr.as_ptr(),
                        Release,
                    );

                    if res.is_null() {
                        break Self {
                            node: unsafe { &*nnptr.as_ptr() },
                        };
                    }

                    let alloc = unsafe { OwnedAlloc::from_raw(nnptr) };
                    cache.store(alloc.drop_in_place());
                    node = unsafe { &*res };
                },
            }
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
