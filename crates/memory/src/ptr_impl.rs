use crate::macros::exclusive_feature_select;

exclusive_feature_select! {
    "rc" => {
        pub(crate) type PtrImpl<T> = std::rc::Rc<T>;
    }
    "arc" => {
        pub(crate) type PtrImpl<T> = std::sync::Arc<T>;
    }
    "gc" => {
        pub(crate) type PtrImpl<T> = dumpster::unsync::Gc<T>;
    }
    "agc" => {
        pub(crate) type PtrImpl<T> = dumpster::sync::Gc<T>;
    }
}

exclusive_feature_select! {
    "rc" | "gc" => {
        pub(crate) type CellImpl<T> = std::cell::RefCell<T>;
        pub(crate) type BorrowImpl<'a, T> = std::cell::Ref<'a, T>;
        pub(crate) type BorrowMutImpl<'a, T> = std::cell::RefMut<'a, T>;
    }
    "arc" | "agc" => {
        pub(crate) type CellImpl<T> = parking_lot::RwLock<T>;
        pub(crate) type BorrowImpl<'a, T> = parking_lot::MappedRwLockReadGuard<'a, T>;
        pub(crate) type BorrowMutImpl<'a, T> = parking_lot::MappedRwLockWriteGuard<'a, T>;
    }
}

// implementation for `make_ptr!`
exclusive_feature_select! {
    "rc" => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __make_ptr {
            ($value:expr) => {
                ::std::rc::Rc::new($value) as ::std::rc::Rc<_>
            };
        }
    }
    "arc" => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __make_ptr {
            ($value:expr) => {
                ::std::sync::Arc::new($value) as ::std::sync::Arc<_>
            };
        }
    }
    "gc" => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __make_ptr {
            ($value:expr) => {
                $crate::dumpster::unsync::coerce_gc!($crate::dumpster::unsync::Gc::new($value))
            };
        }
    }
    "agc" => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __make_ptr {
            ($value:expr) => {
                $crate::dumpster::sync::coerce_gc!($crate::dumpster::sync::Gc::new($value))
            };
        }
    }
    _ => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __make_ptr {
            ($value:expr) => {
                match $value { _ => unimplemented!() }
            };
        }
    }
}

// implementation for `lazy!`
exclusive_feature_select! {
    "rc" => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __lazy {
            ($ty:ty; $expr:expr) => {{
                thread_local! {
                    static VALUE: $ty = $expr.into();
                }
                VALUE.with(Clone::clone)
            }};
        }
    }
    "arc" => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __lazy {
            ($ty:ty; $expr:expr) => {{
                static VALUE: ::std::sync::LazyLock<$ty> = ::std::sync::LazyLock::new(|| $expr.into());
                ::std::sync::LazyLock::force(&VALUE).clone()
            }};
        }
    }
    "gc" => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __lazy {
            ($ty:ty; $expr:expr) => {{
                let value: $ty = $expr.into();
                value
            }};
        }
    }
    "agc" => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __lazy {
            ($ty:ty; $expr:expr) => {{
                static VALUE: ::std::sync::LazyLock<$ty> = ::std::sync::LazyLock::new(|| $expr.into());
                ::std::sync::LazyLock::force(&VALUE).clone()
            }};
        }
    }
    _ => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __lazy {
            ($ty:ty; $expr:expr) => {{
                unimplemented!()
            }};
        }
    }
}
