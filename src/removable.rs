use std::{
    fmt,
    mem::{replace, uninitialized, ManuallyDrop},
    sync::atomic::{AtomicBool, Ordering::*},
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
        Self {
            item: ManuallyDrop::new(val),
            present: AtomicBool::new(true),
        }
    }

    /// Creates a removable item with no present value.
    pub fn empty() -> Self {
        Self {
            item: ManuallyDrop::new(unsafe { uninitialized() }),
            present: AtomicBool::new(false),
        }
    }

    /// Replaces the stored value with a given one and returns the old value.
    /// Requires a mutable reference since the type of the value might not be
    /// atomic.
    pub fn replace(&mut self, val: T) -> Option<T> {
        if self.present.swap(true, Relaxed) {
            Some(replace(&mut *self.item, val))
        } else {
            unsafe { (&mut *self.item as *mut T).write(val) };
            None
        }
    }

    /// Tries to get a mutable reference to the stored value. If the value was
    /// not present, `None` is returned.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.present.load(Relaxed) {
            Some(&mut *self.item)
        } else {
            None
        }
    }

    /// Tests if the stored value is present. Note that there are no guarantees
    /// that `take` will be successful if this method returns `true` because
    /// some other thread could take the value meanwhile.
    pub fn is_present(&self) -> bool {
        self.present.load(Acquire)
    }

    /// Tries to take the value. If no value was present in first place, `None`
    /// is returned.
    pub fn take(&self) -> Option<T> {
        if self.present.swap(false, AcqRel) {
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
            self.is_present(),
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
        if self.is_present() {
            unsafe { ManuallyDrop::drop(&mut self.item) }
        }
    }
}

unsafe impl<T> Send for Removable<T> where T: Send {}
unsafe impl<T> Sync for Removable<T> where T: Send {}
