use crate::{KotoTrace, feature_select};

macro_rules! pointee_traits_docs {
    () => {
        "\
A set of traits that a `T` of `Ptr<T>` must implement

The set of traits depends on the active feature:
- **`rc`** — requires no extra traits
- **`arc`** — requires no extra traits
- **`gc`** — requires `'static`
- **`agc`** — requires `Send + Sync + 'static`
"
    };
}

feature_select! {
    "gc" => {
        #[doc = pointee_traits_docs!()]
        pub trait PointeeTraits: KotoTrace + 'static {}
        impl<T: ?Sized + KotoTrace + 'static> PointeeTraits for T {}
    }
    "agc" => {
        #[doc = pointee_traits_docs!()]
        pub trait PointeeTraits: KotoTrace + Send + Sync + 'static {}
        impl<T: ?Sized + KotoTrace + Send + Sync + 'static> PointeeTraits for T {}
    }
    _ => {
        #[doc = pointee_traits_docs!()]
        pub trait PointeeTraits: KotoTrace {}
        impl<T: ?Sized + KotoTrace> PointeeTraits for T {}
    }
}
