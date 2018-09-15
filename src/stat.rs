macro_rules! with_stat {
    ($($item:item)*) => {
        $(#[cfg(lockfree_stats)] $item)*
    };
}

macro_rules! without_stat {
    ($($item:item)*) => {
        $(#[cfg(not(lockfree_stats))] $item)*
    };
}

with_stat! {
    use atomic::*;
    use std::sync::atomic::{AtomicUsize, Ordering::*};

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Mean {
        rate: f64,
        total: usize,
    }

    #[derive(Debug)]
    pub struct Stat {
        tag: &'static str,
        min: AtomicUsize,
        max: AtomicUsize,
        mean: AtomicBox<Mean>,
    }

    #[derive(Debug)]
    pub struct Session<'stat> {
        stat: &'stat Stat,
        count: usize,
    }

    impl Stat {
        pub fn new(tag: &'static str) -> Self {
            Self {
                tag,
                min: AtomicUsize::new(!0),
                max: AtomicUsize::new(0),
                mean: AtomicBox::new(Mean { rate: 0.0, total: 0 }),
            }
        }

        pub fn session<'stat>(&'stat self) -> Session<'stat> {
            Session { stat: self, count: 0 }
        }

        pub fn tag(&self) -> &'static str {
            self.tag
        }

        pub fn min(&self) -> usize {
            self.min.load(Relaxed)
        }

        pub fn max(&self) -> usize {
            self.max.load(Relaxed)
        }

        pub fn mean(&self) -> f64 {
            self.mean.load(Relaxed).rate
        }

        pub fn total(&self) -> usize {
            self.mean.load(Relaxed).total
        }
    }

    impl Drop for Stat {
        fn drop(&mut self) {
            println!(
                "stat {}: min = {}, max = {}, mean = {:.3}, total = {}",
                self.tag(),
                self.min(),
                self.max(),
                self.mean(),
                self.total()
            )
        }
    }

    impl<'stat> Session<'stat> {
        pub fn tick(&mut self) {
            self.count += 1;
        }
    }

    #[allow(unused_must_use)]
    impl<'stat> Drop for Session<'stat> {
        fn drop(&mut self) {
            self.stat.min.load_cas_loop(
                |i| Some(i.min(self.count)),
                Acquire,
                Release,
            );
            self.stat.max.load_cas_loop(
                |i| Some(i.max(self.count)),
                Acquire,
                Release,
            );
            self.stat.mean.load_cas_loop(
                |mut mean| {
                    mean.total += 1;
                    mean.rate +=
                        self.count as u64 as f64 / mean.total as u64 as f64;
                    Some(mean)
                },
                Acquire,
                Release,
            );
        }
    }
}

without_stat! {
    use std::marker::PhantomData;

    #[derive(Debug)]
    pub struct Stat(());

    #[derive(Debug)]
    pub struct Session<'stat>(PhantomData<&'stat Stat>);

    impl Stat {
        #[inline(always)]
        pub fn new(_tag: &'static str) -> Self {
            Stat(())
        }

        #[inline(always)]
        pub fn session<'stat>(&'stat self) -> Session<'stat> {
            Session(PhantomData)
        }

        #[inline(always)]
        pub fn tag(&self) -> &'static str {
            ""
        }

        #[inline(always)]
        pub fn min(&self) -> usize {
            !0
        }

        #[inline(always)]
        pub fn max(&self) -> usize {
            0
        }

        #[inline(always)]
        pub fn mean(&self) -> f64 {
            0.0
        }

        #[inline(always)]
        pub fn total(&self) -> usize {
            0
        }
    }

    impl<'stat> Session<'stat> {
        #[inline(always)]
        pub fn tick(&mut self) {}
    }
}
