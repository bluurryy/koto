//! Definitions of Send and Sync used in the Koto runtime
//!
//! When Koto is being used in a single-threaded context [KotoSend] and [KotoSync] are empty
//! traits implemented for all types.

macro_rules! send_sync_docs {
    ($send_or_sync:literal) => {
        concat!("\
An alias for the `", $send_or_sync ,"` trait or an empty trait implemented for all types

In multi-threaded contexts (**\"arc\"** or **\"agc\"**) this is an alias for 
[`", $send_or_sync, "`]. Otherwise it's an empty trait, implemented for all types.

")
    };
}

#[allow(unused_macros)]
macro_rules! inherited_docs_note {
    ($send_or_sync:literal) => {
        concat!(
            "\
**What follows is the documentation of the [`",
            $send_or_sync,
            "`] trait.**

---

"
        )
    };
}

#[cfg(any(feature = "rc", feature = "gc"))]
mod inner {
    #[doc = send_sync_docs!("Send")]
    pub trait KotoSend {}
    impl<T: ?Sized> KotoSend for T {}

    #[cfg(any(feature = "rc", feature = "gc"))]
    #[doc = send_sync_docs!("Sync")]
    pub trait KotoSync {}
    impl<T: ?Sized> KotoSync for T {}
}

#[cfg(not(any(feature = "rc", feature = "gc")))]
mod inner {
    #[doc = send_sync_docs!("Send")]
    #[doc = inherited_docs_note!("Send")]
    pub use Send as KotoSend;

    #[cfg(not(any(feature = "rc", feature = "gc")))]
    #[doc = send_sync_docs!("Sync")]
    #[doc = inherited_docs_note!("Sync")]
    pub use Sync as KotoSync;
}

pub use inner::{KotoSend, KotoSync};
