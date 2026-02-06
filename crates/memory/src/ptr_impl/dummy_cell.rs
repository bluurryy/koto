use std::{
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Default)]
pub struct DummyCell<T: ?Sized>(PhantomData<fn() -> T>);

impl<T> From<T> for DummyCell<T> {
    fn from(_value: T) -> Self {
        unimplemented!()
    }
}

pub struct DummyBorrow<'a, T: ?Sized>(PhantomData<fn() -> &'a T>);

impl<T: ?Sized> Deref for DummyBorrow<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for DummyBorrow<'_, T> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

pub struct DummyBorrowMut<'a, T: ?Sized>(PhantomData<fn() -> &'a mut T>);

impl<T: ?Sized> Deref for DummyBorrowMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}

impl<T: ?Sized> DerefMut for DummyBorrowMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unimplemented!()
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for DummyBorrowMut<'_, T> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
