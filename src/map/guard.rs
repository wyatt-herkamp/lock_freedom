use super::bucket::Garbage;
use incin::Pause;

#[derive(Debug)]
pub struct ReadGuard<'origin, K, V> {
    pair: &'origin (K, V),
    pause: Pause<'origin, Garbage<K, V>>,
}

impl<'origin, K, V> ReadGuard<'origin, K, V> {
    pub(super) fn new(
        pair: &'origin (K, V),
        pause: Pause<'origin, Garbage<K, V>>,
    ) -> Self {
        Self { pair, pause }
    }
}
