use super::bucket::Garbage;
use incin::Pause;

#[derive(Debug)]
pub struct ReadGuard<'origin, K, V> {
    pair: &'origin (K, V),
    pause: Pause<'origin, Garbage<K, V>>,
}
