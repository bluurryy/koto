use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::Deref,
};

use crate::macros::feature_select_item;

#[derive(Debug, Default)]
pub struct DummyPtr<T: ?Sized>(PhantomData<fn() -> T>);

feature_select_item! {
    "gc" | "agc" => {
        unsafe impl<V: dumpster::Visitor, T: crate::Pointee + ?Sized> dumpster::TraceWith<V> for DummyPtr<T> {
            fn accept(&self, _visitor: &mut V) -> Result<(), ()> {
                unimplemented!()
            }
        }
    }
}

impl<T> From<T> for DummyPtr<T> {
    fn from(_value: T) -> Self {
        unimplemented!()
    }
}

impl<T> From<Box<T>> for DummyPtr<T> {
    fn from(_value: Box<T>) -> Self {
        unimplemented!()
    }
}

impl<T: ?Sized> DummyPtr<T> {
    pub(crate) fn ptr_eq(_this: &Self, _other: &Self) -> bool {
        unimplemented!()
    }

    pub(crate) fn as_ptr(_this: &Self) -> *const T {
        unimplemented!()
    }

    pub(crate) fn make_mut(_this: &mut Self) -> &mut T {
        unimplemented!()
    }
}

impl<T: ?Sized> Deref for DummyPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}

impl<T: ?Sized> Clone for DummyPtr<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: Clone> From<&[T]> for DummyPtr<[T]> {
    fn from(_value: &[T]) -> Self {
        unimplemented!()
    }
}

impl<T> From<Vec<T>> for DummyPtr<[T]> {
    fn from(_value: Vec<T>) -> Self {
        unimplemented!()
    }
}

impl From<&str> for DummyPtr<str> {
    fn from(_value: &str) -> Self {
        unimplemented!()
    }
}

impl From<String> for DummyPtr<str> {
    fn from(_value: String) -> Self {
        unimplemented!()
    }
}

impl<T: ?Sized + PartialEq> PartialEq for DummyPtr<T> {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}

impl<T: ?Sized + Eq> Eq for DummyPtr<T> {}

impl<T: ?Sized + PartialOrd> PartialOrd for DummyPtr<T> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        unimplemented!()
    }
}

impl<T: ?Sized + Ord> Ord for DummyPtr<T> {
    fn cmp(&self, _other: &Self) -> Ordering {
        unimplemented!()
    }
}

impl<T: ?Sized + Hash> Hash for DummyPtr<T> {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        unimplemented!()
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for DummyPtr<T> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
