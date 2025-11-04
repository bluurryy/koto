//! Definitions of Send and Sync used in the Koto runtime
//!
//! When Koto is being used in a single-threaded context [KotoSend] and [KotoSync] are empty
//! traits implemented for all types.

#[cfg(any(feature = "rc", feature = "gc"))]
mod traits {
    /// An empty trait for single-threaded contexts, implemented for all types
    pub trait KotoSend {}
    impl<T: ?Sized> KotoSend for T {}

    /// An empty trait for single-threaded contexts, implemented for all types
    pub trait KotoSync {}
    impl<T: ?Sized> KotoSync for T {}
}

#[cfg(not(any(feature = "rc", feature = "gc")))]
mod traits {
    pub use Send as KotoSend;
    pub use Sync as KotoSync;
}

pub use traits::*;
