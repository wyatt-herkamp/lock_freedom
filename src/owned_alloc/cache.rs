/// A general purpouse cache suitable for saving discarted memory allocations in
/// a tight loop.
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
