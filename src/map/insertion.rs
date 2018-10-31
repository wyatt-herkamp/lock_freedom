use super::Removed;

/// The result of an _interactive_ insertion.
// #[derive(Debug, PartialEq, Eq)]
pub enum Insertion<K, V, E> {
    /// If this is returned, an entry for the given key was successfully
    /// created with the provided value i.e. there was no entry before.
    Created,
    /// If this is returned, an entry for the given key already existed and
    /// was successfully updated with the provided value. The field is the old
    /// entry pair.
    Updated(Removed<K, V>),
    /// If this is returned, the insertion failed and no action was done.
    /// Failure may have happened because the given closure rejected the
    /// conditions. The field will depend on the method you called.
    Failed(E),
}

impl<K, V, E> Insertion<K, V, E> {
    /// Is this insertion a creation?
    pub fn created(&self) -> bool {
        match self {
            Insertion::Created => true,
            _ => false,
        }
    }

    /// Is this insertion an update? If so, the return is a reference to the
    /// old value.
    pub fn updated(&self) -> Option<&Removed<K, V>> {
        match self {
            Insertion::Updated(pair) => Some(pair),
            _ => None,
        }
    }

    /// Is this insertion an update? If so, the old value is taken and
    /// returned. Otherwise, the insertion is returned.
    pub fn take_updated(self) -> Result<Removed<K, V>, Self> {
        match self {
            Insertion::Updated(pair) => Ok(pair),
            this => Err(this),
        }
    }

    /// Is this a failure? If so, return a reference to the custom field.
    pub fn failed(&self) -> Option<&E> {
        match self {
            Insertion::Failed(err) => Some(err),
            _ => None,
        }
    }

    /// Is this a failure? If so, the custom field is taken and
    /// returned. Otherwise, the insertion is returned.
    pub fn take_failed(self) -> Result<E, Self> {
        match self {
            Insertion::Failed(e) => Ok(e),
            this => Err(this),
        }
    }
}

/// A preview of the value in an _interactive_ insertion.
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preview<V> {
    /// Discard the current generated value. If there is no currently
    /// generated value, the closure returning this is not generating anyone
    /// (i.e. conditions not met). The inserter may still be consulted with a
    /// different found value, however.
    Discard,
    /// Keep the current genereated value. If there is no currently
    /// generated value, this has the same effect as `Discard` do.
    Keep,
    /// Replace the current value with this new one.
    New(V),
}
