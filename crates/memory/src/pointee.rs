use crate::{KotoTrace, macros::exactly_one_feature_select};

macro_rules! pointee_docs {
    () => {
        "\
A set of bounds that a `T` of `Ptr<T>` must satify

- **`rc`** — requires `KotoTrace`
- **`arc`** — requires `KotoTrace`
- **`gc`** — requires `KotoTrace + 'static`
- **`agc`** — requires `KotoTrace + Send + Sync + 'static`
"
    };
}

exactly_one_feature_select! {
    "gc" => {
        #[doc = pointee_docs!()]
        pub trait Pointee: KotoTrace + 'static {}
        impl<T: ?Sized + KotoTrace + 'static> Pointee for T {}
    }
    "agc" => {
        #[doc = pointee_docs!()]
        pub trait Pointee: KotoTrace + Send + Sync + 'static {}
        impl<T: ?Sized + KotoTrace + Send + Sync + 'static> Pointee for T {}
    }
    _ => {
        #[doc = pointee_docs!()]
        pub trait Pointee: KotoTrace {}
        impl<T: ?Sized + KotoTrace> Pointee for T {}
    }
}
