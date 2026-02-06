//! Memory management utilities for Koto
//!
//! Currently, only reference-counted pointers without cycle detection are implemented.
//! The intent is that this crate can be expanded in the future with implementations of
//! `Ptr` and `PtrMut` that offer alternative memory management strategies.

#![cfg_attr(docsrs, feature(doc_cfg), doc(auto_cfg))]

mod macros;

use macros::*;

mod address;
mod pointee;
mod ptr;
mod ptr_impl;
mod ptr_mut;
mod small_vec;
mod trace;
mod untrace;

pub use address::Address;
pub use koto_derive::KotoTrace;
pub use pointee::Pointee;
pub use ptr::*;
pub use ptr_mut::*;
pub use small_vec::SmallVec;
pub use trace::KotoTrace;
pub use untrace::Untrace;

// Assert that only one of the pointer features is enabled.
//
// We don't do that assertion when building the documentation, so we can
// build the documentation with all features enabled to show the entire api surface.
#[cfg(not(doc))]
assert_exactly_one_ptr_feature!();

#[doc(hidden)]
pub mod __private {
    pub use ::smallvec;
}

feature_select_item! {
    "gc" | "agc" => {
        /// Re-export of the [`dumpster`] crate
        ///
        /// This is the garbage collection implementation.
        pub use ::dumpster;
    }
}

feature_select_item! {
    "gc" | "agc" => {
        /// Performs a garbage-collection pass
        ///
        /// Detects and frees `Ptr` instances that are no longer reachable except through reference cycles.
        ///
        /// - **gc** — collects all allocations local to this thread
        /// - **agc** — might not collect every allocation, but often the ones on this thread
        pub fn collect_garbage() {
            exactly_one_feature_match_or_panic! {
                "gc" => { dumpster::unsync::collect() }
                "agc" => { dumpster::sync::collect() }
            }
        }
    }
}
