/// A general purpouse cache suitable for saving discarted memory allocations in
/// a tight loop.
///
/// # Dummy Example
/// ```rust
/// extern crate owned_alloc;
///
/// use owned_alloc::{Cache, RawVec, UninitAlloc};
///
/// fn do_some_stuff(iter: usize, n: usize) -> usize {
///     let mut cache = Cache::new();
///     let mut res = 0usize;
///
///     for i in 1 ..= iter {
///         let alloc =
///             cache.take_or(|| UninitAlloc::from(RawVec::with_capacity(n)));
///
///         let inited = unsafe {
///             alloc.init_in_place(|slice| {
///                 for j in 0 .. slice.len() {
///                     (&mut slice[j] as *mut usize)
///                         .write(i.wrapping_mul(j + 1))
///                 }
///             })
///         };
///
///         for &item in &*inited {
///             res = res.wrapping_add(item);
///         }
///
///         cache.store(inited.drop_in_place());
///     }
///
///     res
/// }
///
/// assert_eq!(do_some_stuff(2, 3), 1 + 2 + 3 + 2 + 4 + 6);
/// ```

#[derive(Debug)]
pub struct Cache<A> {
    stored: Option<A>,
}

impl<A> Cache<A> {
    /// Creates a new cache with no data.
    pub fn new() -> Self {
        Self { stored: None }
    }

    /// Stores data into the cache.
    pub fn store(&mut self, val: A) {
        self.stored = Some(val);
    }

    /// Takes the data from the cache.
    pub fn take(&mut self) -> Option<A> {
        self.stored.take()
    }

    /// Takes the data from the cache. If there was no data, the passed closure
    /// is called to produce the returned data.
    pub fn take_or<F>(&mut self, create: F) -> A
    where
        F: FnOnce() -> A,
    {
        self.take().unwrap_or_else(create)
    }
}

impl<A> Default for Cache<A> {
    fn default() -> Self {
        Self::new()
    }
}
