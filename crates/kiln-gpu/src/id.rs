use std::{
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};

pub trait HasId<T = ()> {
    fn id(&self) -> Id<T>;
}

/// A generator for ids.
pub struct IdSource<T = ()> {
    inner: AtomicU64,
    marker: PhantomData<*const T>,
}

unsafe impl<T> Send for IdSource<T> {}
unsafe impl<T> Sync for IdSource<T> {}

impl<T> IdSource<T> {
    pub fn generate(&self) -> Id<T> {
        let inner = self.inner.fetch_add(1, Ordering::AcqRel);
        Id::from_raw(inner)
    }
}

impl<T> Default for IdSource<T> {
    fn default() -> Self {
        Self {
            inner: AtomicU64::new(0),
            marker: PhantomData,
        }
    }
}

/// A generic id used to track various resources.
pub struct Id<T = ()> {
    inner: u64,
    marker: PhantomData<*const T>,
}

unsafe impl<T> Send for Id<T> {}
unsafe impl<T> Sync for Id<T> {}

impl<T> Id<T> {
    /// Creates a new [`Id`] from a [`u64`].
    ///
    /// * Note
    /// This should rarely if never be used, as it often leads to problems.
    pub const fn from_raw(inner: u64) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }

    /// Casts self into any other id.
    ///
    /// * Note
    /// This should rarely if never be used, as it often leads to problems.
    pub const fn cast<U>(self) -> Id<U> {
        Id {
            inner: self.inner,
            marker: PhantomData,
        }
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            marker: PhantomData,
        }
    }
}

impl<T> Copy for Id<T> {}

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id<{}>({})", std::any::type_name::<T>(), self.inner)
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T> Eq for Id<T> {}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T> HasId<T> for Id<T> {
    fn id(&self) -> Id<T> {
        *self
    }
}
