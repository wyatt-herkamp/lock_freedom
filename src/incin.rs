use std::{
    cell::Cell,
    fmt,
    sync::atomic::{AtomicUsize, Ordering::*},
};
use tls::ThreadLocal;

/// The incinerator. It is an API used to solve the infamous ABA problem. It
/// basically consists of a counter and a list of garbage. Before a thread
/// begins a suffering-from-ABA operation, it should start a new pause, and keep
/// the incinerator paused while it is performing the operation.
///
/// When a thread wants to drop an allocation that might affect other threads
/// with ABA problem or uses-after-free, it should `add` it to the incinerator's
/// garbage list. The incinerator will only execute the `drop` of its type `T`
/// when the pause counter is zero.
///
/// When the incinerator is dropped, all the garbage is automatically dropped
/// too.
///
/// C11 Implementation: <https://gitlab.com/bzim/c11-incinerator/>
///
/// # Example
/// ```rust
/// extern crate lockfree;
///
/// use lockfree::prelude::*;
/// use std::{
///     ptr::{null_mut, NonNull},
///     sync::{
///         atomic::{AtomicPtr, Ordering::*},
///         Arc,
///     },
///     thread,
/// };
///
/// let incin = Arc::new(Incinerator::<Box<u128>>::new());
/// let ptr = Box::into_raw(Box::new(55u128));
/// let dummy_state = Arc::new(AtomicPtr::new(ptr));
///
/// let mut threads = Vec::with_capacity(16);
///
/// for i in 0 .. 16 {
///     let state = dummy_state.clone();
///     let incin = incin.clone();
///     threads.push(thread::spawn(move || {
///         let ptr = incin.pause_with(|| {
///             let loaded = state.load(SeqCst);
///             let new = unsafe { *loaded + i };
///             state.swap(Box::into_raw(Box::new(new)), SeqCst)
///         });
///
///         // dropping
///         incin.add(unsafe { Box::from_raw(ptr) })
///     }));
/// }
///
/// for thread in threads {
///     thread.join().unwrap();
/// }
///
/// let boxed = unsafe { Box::from_raw(dummy_state.load(SeqCst)) };
/// assert!(*boxed <= 15 * 15);
/// ```
#[derive(Debug)]
pub struct Incinerator<T> {
    counter: AtomicUsize,
    tls_list: ThreadLocal<GarbageList<T>>,
}

impl<T> Incinerator<T> {
    /// Creates a new incinerator, with no pauses and empty garbage list.
    pub fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
            tls_list: ThreadLocal::new(),
        }
    }

    /// Increments the pause counter and creates a pause associated with this
    /// incinerator. Only after creating the pause you should perform atomic
    /// operations such as `load` and any other operation affected by ABA
    /// problem.
    pub fn pause(&self) -> Pause<T> {
        loop {
            let init = self.counter.load(Acquire);
            // Sanity check.
            if init == usize::max_value() {
                panic!("Too many pauses");
            }
            // Simply try to increment it. This will be decremented at
            // `Pause::drop`. Nobody will be able to drop stuff while this is
            // not 0.
            if self.counter.compare_and_swap(init, init + 1, Release) == init {
                break Pause { incin: self };
            }
        }
    }

    /// Creates a pause before executing the given closure and resumes the
    /// incinerator only after executing the closure. You should execute the
    /// whole ABA-problem-suffering cycle of `load` and `compare_and_swap`
    /// inside the closure. See documentation for `Incinerator::pause` and
    /// `Pause::resume` for more details.
    pub fn pause_with<F, A>(&self, exec: F) -> A
    where
        F: FnOnce() -> A,
    {
        let pause = self.pause();
        let ret = exec();
        pause.resume();
        ret
    }

    /// Adds the given value to the garbage list. The value is only dropped when
    /// the counter is zero. If the counter is zero when the method is called,
    /// the value is immediately dropped and the garbage list is cleared. You
    /// must remove the resource from shared context before calling this method.
    pub fn add(&self, val: T) {
        if self.counter.load(Acquire) == 0 {
            // Safe to drop it all. Note that we check the counter after the
            // resource was removed from shared context. Since we use Thread
            // Local Storage, nobody can add something to the list meanwhile
            // besides us.
            self.tls_list.with(GarbageList::clear);
            drop(val);
        } else {
            // Not safe to drop. We have to save the value in the garbage list.
            self.tls_list
                .with_init(GarbageList::new, |list| list.add(val));
        }
    }

    /// Tries to delete the garbage list associated with this thread. The
    /// garbage list is only cleared if the counter is zero. In case of success,
    /// `true` is returned.
    pub fn try_clear(&self) -> bool {
        if self.counter.load(Acquire) == 0 {
            // It is only safe to drop if there are no active pauses. Remember
            // nobody can add something to this specific list besides us because
            // it is thread local.
            self.tls_list.with(GarbageList::clear);
            true
        } else {
            false
        }
    }

    /// Clears everything that is in the inicinerator regardless of pauses.
    /// Exclusive reference is required.
    pub fn clear(&mut self) {
        self.tls_list.clear();
    }
}

impl<T> Default for Incinerator<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// An active incinerator pause. When a value of this type is alive, no
/// sensitive data is dropped in the incinerator. When a value of this type is
/// dropped, the incinerator counter is decremented.
#[derive(Debug)]
pub struct Pause<'incin, T>
where
    T: 'incin,
{
    incin: &'incin Incinerator<T>,
}

impl<'incin, T> Pause<'incin, T> {
    /// Returns the incinerator on which this pause acts.
    pub fn incin(&self) -> &Incinerator<T> {
        self.incin
    }

    /// Forces drop and decrements the incinerator counter. If the counter
    /// becomes 0, the list associated with this thread is cleared. This method
    /// does not need to be called because the incinerator counter is
    /// decremented when the pause is dropped.
    pub fn resume(self) {}
}

impl<'incin, T> Drop for Pause<'incin, T> {
    fn drop(&mut self) {
        if self.incin.counter.fetch_sub(1, AcqRel) == 1 {
            // If the previous value was 1, this means now it is 0 and... we can
            // delete our local list.
            self.incin.tls_list.with(GarbageList::clear);
        }
    }
}

impl<'incin, T> Clone for Pause<'incin, T> {
    fn clone(&self) -> Self {
        self.incin.pause()
    }
}

struct GarbageList<T> {
    list: Cell<Vec<T>>,
}

impl<T> GarbageList<T> {
    fn new() -> Self {
        Self {
            list: Cell::new(Vec::new()),
        }
    }

    fn add(&self, val: T) {
        let mut list = self.list.replace(Vec::new());
        list.push(val);
        self.list.replace(list);
    }

    fn clear(&self) {
        self.list.replace(Vec::new());
    }
}

impl<T> fmt::Debug for GarbageList<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let list = self.list.replace(Vec::new());
        write!(fmtr, "{:?}", list)?;

        let mut tmp = self.list.replace(list);

        // A totally weird corner case, but we have to handle it.
        if tmp.len() > 0 {
            let mut list = self.list.replace(Vec::new());
            list.append(&mut tmp);
            self.list.replace(list);
        }

        Ok(())
    }
}

macro_rules! doc {
    ($doc:expr ; $($target:tt)*) => {
        #[doc = $doc]
        $($target)*
    };
}

macro_rules! make_shared_incin {
    (
        { $target:expr }
        $(#[$meta:meta])*
        $vis:vis $name:ident<$($params:ident),*> of $garbage:ty
    ) => {
        doc! {
            concat!("The shared incinerator used by ", $target, ".");
            $(#[$meta])*
            $vis struct $name<$($params),*> {
                inner: Arc<Incinerator<$garbage>>,
            }
        }

        impl<$($params),*> $name<$($params),*> {
            doc! {
                concat!("Creates a new shared incinerator for ", $target, ".");
                $vis fn new() -> Self {
                    Self {
                        inner: Arc::new(Incinerator::new()),
                    }
                }
            }

            fn clear(&mut self) {
                if let Some(incin) = Arc::get_mut(&mut self.inner) {
                    incin.clear();
                    return;
                }
                self.inner.try_clear();
            }
        }

        impl<$($params),*> Default for $name<$($params),*> {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<$($params),*> Clone for $name<$($params),*> {
            fn clone(&self) -> Self {
                Self {
                    inner: self.inner.clone(),
                }
            }
        }
    };
}
