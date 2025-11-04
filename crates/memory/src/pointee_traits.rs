use crate::{KotoTrace, macros::exclusive_feature_select};

macro_rules! pointee_traits_docs {
    () => {
        "A given set of traits that a `T` of `Ptr<T>` must implement.\n\
        \n\
        - **`rc`** — does not require extra traits\n\
        - **`arc`** — does not require extra traits\n\
        - **`gc`** — requires `'static`\n\
        - **`agc`** — requires `Send + Sync + 'static`\n\
        "
    };
}

exclusive_feature_select! {
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
