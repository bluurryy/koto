use core::slice;
use std::ops::{Deref, DerefMut};

use smallvec::SmallVec as Inner;

use crate::feature_select;

/// A convenience macro for initializing an [`SmallVec`]
#[macro_export]
macro_rules! smallvec {
    ($($tt:tt)*) => {
        $crate::SmallVec($crate::__private::smallvec::smallvec!($($tt)*))
    };
}

/// Wrapper around [`smallvec::SmallVec`] to implement [`KotoTrace`](crate::KotoTrace)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SmallVec<T, const N: usize>(pub Inner<[T; N]>);

impl<T, const N: usize> SmallVec<T, N> {
    /// See [`smallvec::SmallVec::new`].
    pub fn new() -> Self {
        Self(Inner::new())
    }

    /// See [`smallvec::SmallVec::with_capacity`].
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Inner::with_capacity(capacity))
    }

    /// See [`smallvec::SmallVec::from_vec`].
    pub fn from_vec(vec: Vec<T>) -> Self {
        Self(Inner::from_vec(vec))
    }
}

impl<A, const N: usize> FromIterator<A> for SmallVec<A, N> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self(Inner::from_iter(iter))
    }
}

impl<T, const N: usize> Deref for SmallVec<T, N> {
    type Target = Inner<[T; N]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for SmallVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a SmallVec<T, N> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

feature_select! {
    "gc" | "agc" => {
        use dumpster::{TraceWith, Visitor};

        unsafe impl<V: Visitor, T: TraceWith<V>, const N: usize> TraceWith<V> for SmallVec<T, N> {
            fn accept(&self, visitor: &mut V) -> Result<(), ()> {
                for item in self.0.iter() {
                    item.accept(visitor)?;
                }

                Ok(())
            }
        }
    }
}
