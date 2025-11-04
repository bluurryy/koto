//! Memory management utilities for Koto
//!
//! Currently, only reference-counted pointers without cycle detection are implemented.
//! The intent is that this crate can be expanded in the future with implementations of
//! `Ptr` and `PtrMut` that offer alternative memory management strategies.
//!
//! Making custom GC types that support trait objects or other DSTs is currently only
//! possible with nightly Rust, while the stabilization of DST custom coercions
//! [is pending](https://github.com/rust-lang/rust/issues/18598)
//!
//! Until then, GC implementations for Ptr/PtrMut could be introduced with a nightly-only feature.

assert_mutually_exclusive_features!("arc", "rc", "agc", "gc");

mod address;
mod macros;
mod pointee_traits;
mod ptr;
mod ptr_impl;
mod ptr_mut;
mod send_sync;
mod small_vec;

pub use address::Address;
pub use koto_derive::KotoTrace;
pub use pointee_traits::PointeeTraits;
pub use ptr::*;
pub use ptr_mut::*;
pub use send_sync::{KotoSend, KotoSync};
pub use small_vec::SmallVec;

use macros::{
    assert_mutually_exclusive_features, exclusive_feature_select, exclusive_feature_select_expr,
};

exclusive_feature_select! {
    "gc" | "agc" => {
        pub use dumpster::{Trace, TraceWith, Visitor};

        unsafe impl<V: dumpster::Visitor, T: ?Sized> dumpster::TraceWith<V> for Untrace<T> {
            fn accept(&self, _visitor: &mut V) -> Result<(), ()> {
                Ok(())
            }
        }
    }
}

exclusive_feature_select! {
    "gc" | "agc" => {
        /// A wrapper trait for [`dumpster::Trace`].
        pub trait KotoTrace: dumpster::Trace + 'static {}
        impl<T: ?Sized + dumpster::Trace + 'static> KotoTrace for T {}
    }
    _ => {
        /// An empty trait for non-garbage-collected contexts, implemented for all types
        pub trait KotoTrace {}
        impl<T: ?Sized> KotoTrace for T {}
    }
}

#[doc(hidden)]
pub mod __private {
    pub use ::smallvec;
}

exclusive_feature_select! {
    "gc" | "agc" => {
        /// Re-export of the [`dumpster`] crate
        ///
        /// This is the garbage collection implementation.
        pub use ::dumpster;
    }
}

/// TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Untrace<T: ?Sized>(pub T);

impl<T: Iterator> Iterator for Untrace<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<T: DoubleEndedIterator> DoubleEndedIterator for Untrace<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<T: std::io::Read> std::io::Read for Untrace<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
    // TODO: more impls?
}

impl<T: std::io::Write> std::io::Write for Untrace<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
    // TODO: more impls?
}

impl<T: std::io::Seek> std::io::Seek for Untrace<T> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.0.seek(pos)
    }

    // TODO: more impls?
}
