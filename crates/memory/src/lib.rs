//! Memory management utilities for Koto
//!
//! Currently, only reference-counted pointers without cycle detection are implemented.
//! The intent is that this crate can be expanded in the future with implementations of
//! `Ptr` and `PtrMut` that offer alternative memory management strategies.

mod macros;

use macros::*;

assert_valid_features! {
    mod address;
    mod pointee_traits;
    mod ptr;
    mod ptr_impl;
    mod ptr_mut;
    mod send_sync;
    mod small_vec;
    mod untrace;
    mod trace;

    pub use address::Address;
    pub use koto_derive::KotoTrace;
    pub use pointee_traits::PointeeTraits;
    pub use ptr::*;
    pub use ptr_mut::*;
    pub use send_sync::{KotoSend, KotoSync};
    pub use small_vec::SmallVec;
    pub use untrace::Untrace;
    pub use trace::KotoTrace;

    #[doc(hidden)]
    pub mod __private {
        pub use ::smallvec;
    }

    feature_select! {
        "gc" | "agc" => {
            /// Re-export of the [`dumpster`] crate
            ///
            /// This is the garbage collection implementation.
            pub use ::dumpster;
        }
    }
}
