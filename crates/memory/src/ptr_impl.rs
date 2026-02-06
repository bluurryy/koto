use crate::{feature_select, macros::exactly_one_feature_select};

exactly_one_feature_select! {
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
    _ => {
        mod dummy_ptr;
        pub(crate) type PtrImpl<T> = dummy_ptr::DummyPtr<T>;
    }
}

exactly_one_feature_select! {
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
    _ => {
        mod dummy_cell;
        pub(crate) type CellImpl<T> = dummy_cell::DummyCell<T>;
        pub(crate) type BorrowImpl<'a, T> = dummy_cell::DummyBorrow<'a, T>;
        pub(crate) type BorrowMutImpl<'a, T> = dummy_cell::DummyBorrowMut<'a, T>;
    }
}

// implementation for `make_ptr!`
feature_select! {
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
}

// implementation for `lazy!`
feature_select! {
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
                // A garbage collected pointer must not be stored in a thread local, because it can lead to a panic.
                //
                // The pointer registry of the garbage collector is also a thread local which can get destructed
                // before a thread local pointer. The pointer may try to access the pointer registry when it drops
                // which would then can cause a panic if the registry is already destructed.
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
}
