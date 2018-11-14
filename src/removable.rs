use std::{
    fmt,
    mem::{replace, uninitialized, ManuallyDrop},
    sync::atomic::{
        AtomicBool,
        Ordering::{self, *},
    },
};

/// A shared removable value. You can only take values from this type (no
/// insertion allowed). No extra allocation is necessary. It may be useful for
/// things like shared `thread::JoinHandle`s.
pub struct Removable<T> {
    item: ManuallyDrop<T>,
    present: AtomicBool,
}

impl<T> Removable<T> {
    /// Creates a removable item with the passed argument as a present value.
    pub fn new(val: T) -> Self {
        Self { item: ManuallyDrop::new(val), present: AtomicBool::new(true) }
    }

    /// Creates a removable item with no present value.
    pub fn empty() -> Self {
        Self {
            // This is safe because we will only read from the item if present
            // is true. Present will only be true if we write to it.
            item: ManuallyDrop::new(unsafe { uninitialized() }),
            present: AtomicBool::new(false),
        }
    }

    /// Replaces the stored value with a given one and returns the old value.
    /// Requires a mutable reference since the type of the value might not be
    /// atomic.
    pub fn replace(&mut self, val: Option<T>) -> Option<T> {
        let present = self.present.get_mut();

        match val {
            Some(val) => {
                if *present {
                    Some(replace(&mut *self.item, val))
                } else {
                    // Safe because we get the pointer from a valid reference
                    // and present will only be false if item is uninitialized.
                    *present = true;
                    unsafe { (&mut *self.item as *mut T).write(val) };
                    None
                }
            },

            None if *present => {
                // Safe because we get the pointer from a valid reference
                // and present will only be false if item is uninitialized.
                *present = false;
                Some(unsafe { (&*self.item as *const T).read() })
            },

            None => None,
        }
    }

    /// Tries to get a mutable reference to the stored value. If the value was
    /// not present, `None` is returned.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if *self.present.get_mut() {
            Some(&mut *self.item)
        } else {
            None
        }
    }

    /// Tests if the stored value is present. Note that there are no guarantees
    /// that `take` will be successful if this method returns `true` because
    /// some other thread could take the value meanwhile.
    pub fn is_present(&self, ordering: Ordering) -> bool {
        self.present.load(ordering)
    }

    /// Tries to take the value. If no value was present in first place, `None`
    /// is returned.
    pub fn take(&self, ordering: Ordering) -> Option<T> {
        if self.present.swap(false, ordering) {
            // Safe because if present was true, the memory was initialized. All
            // other reads won't happen because we set present to false.
            Some(unsafe { (&*self.item as *const T).read() })
        } else {
            None
        }
    }
}

impl<T> fmt::Debug for Removable<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "Removable {} present: {:?} {}",
            '{',
            self.is_present(Relaxed),
            '}'
        )
    }
}

impl<T> Default for Removable<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> Drop for Removable<T> {
    fn drop(&mut self) {
        if *self.present.get_mut() {
            // Safe because present will only be true when the memory is
            // initialized. And now we are at drop.
            unsafe { ManuallyDrop::drop(&mut self.item) }
        }
    }
}

impl<T> From<Option<T>> for Removable<T> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(item) => Self::new(item),
            None => Self::empty(),
        }
    }
}

unsafe impl<T> Send for Removable<T> where T: Send {}
unsafe impl<T> Sync for Removable<T> where T: Send {}
