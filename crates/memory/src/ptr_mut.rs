use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use crate::{
    PointeeTraits, Ptr,
    macros::{exclusive_feature_select, exclusive_feature_select_expr},
    ptr_impl::{BorrowImpl, BorrowMutImpl, CellImpl},
};

/// Makes a PtrMut, with support for casting to trait objects
///
/// Although `PtrMut::from` is available, the challenge comes when a trait object needs to be used as
/// the pointer type. Until the `CoerceUnized` trait is stabilized, casting from a concrete type to
/// `dyn Trait` needs to be performed on the inner pointer. This macro encapsulates the casting to
/// make life easier at the call site.
#[macro_export]
macro_rules! make_ptr_mut {
    ($value:expr) => {
        $crate::make_ptr!($crate::KCell::from($value))
    };
}

/// A mutable pointer to a value in allocated memory
pub type PtrMut<T> = Ptr<KCell<T>>;

impl<T: PointeeTraits> From<T> for PtrMut<T> {
    fn from(value: T) -> Self {
        Ptr::from(KCell::from(value))
    }
}

/// A mutable value with borrowing checked at runtime
#[derive(Debug, Default)]
pub struct KCell<T: ?Sized>(CellImpl<T>);

exclusive_feature_select! {
    "gc" | "agc" => {
        unsafe impl<V: dumpster::Visitor, T: dumpster::TraceWith<V> + ?Sized> dumpster::TraceWith<V> for KCell<T> {
            #[inline]
            fn accept(&self, visitor: &mut V) -> Result<(), ()> {
                self.try_borrow().ok_or(())?.accept(visitor)
            }
        }
    }
}

impl<T> From<T> for KCell<T> {
    fn from(value: T) -> Self {
        Self(CellImpl::from(value))
    }
}

impl<T: ?Sized> KCell<T> {
    /// Immutably borrows the wrapped value.
    ///
    /// Multiple immutable borrows can be made at the same time.
    ///
    /// # Feature-specific behavior
    ///
    /// If the value is currently mutably borrowed then
    /// - with the "rc" feature, this will panic
    /// - with the "arc" feature, this will block
    ///
    /// See `try_borrow` for a non-panicking/non-blocking version.
    pub fn borrow(&self) -> Borrow<'_, T> {
        exclusive_feature_select_expr! {
            "rc" | "gc" => {
                Borrow(self.0.borrow())
            }
            "arc" | "agc" => {
                Borrow(parking_lot::RwLockReadGuard::map(self.0.read(), |x| x))
            }
        }
    }

    /// Attempts to mutably borrow the wrapped value.
    ///
    /// Returns an error if the value is currently mutably borrowed.
    pub fn try_borrow(&self) -> Option<Borrow<'_, T>> {
        exclusive_feature_select_expr! {
            "rc" | "gc" => {
                self.0.try_borrow().ok().map(Borrow)
            }
            "arc" | "agc" => {
                self.0.try_read().map(|g| Borrow(parking_lot::RwLockReadGuard::map(g, |x| x)))
            }
        }
    }

    /// Mutably borrows the wrapped value.
    ///
    /// # Feature-specific behavior
    ///
    /// If the value is currently borrowed then
    /// - with the "rc" feature, this will panic
    /// - with the "arc" feature, this will block
    ///
    /// See `try_borrow_mut` for a non-panicking version.
    pub fn borrow_mut(&self) -> BorrowMut<'_, T> {
        exclusive_feature_select_expr! {
            "rc" | "gc" => {
                BorrowMut(self.0.borrow_mut())
            }
            "arc" | "agc" => {
                BorrowMut(parking_lot::RwLockWriteGuard::map(self.0.write(), |x| x))
            }
        }
    }

    /// Attempts to mutably borrow the wrapped value.
    ///
    /// Returns an error if the value is currently mutably borrowed.
    pub fn try_borrow_mut(&self) -> Option<BorrowMut<'_, T>> {
        exclusive_feature_select_expr! {
            "rc" | "gc" => {
                self.0.try_borrow_mut().ok().map(BorrowMut)
            }
            "arc" | "agc" => {
                self.0.try_write().map(|g| BorrowMut(parking_lot::RwLockWriteGuard::map(g, |x| x)))
            }
        }
    }
}

/// An immutably borrowed reference to a value borrowed from a [PtrMut]
pub struct Borrow<'a, T: ?Sized>(BorrowImpl<'a, T>);

impl<'a, T: ?Sized> Borrow<'a, T> {
    /// Makes a new Borrow for an optional component of the borrowed data.
    /// If the closure returns None then the original borrow is returned as the error.
    pub fn filter_map<U, F>(borrowed: Self, f: F) -> Result<Borrow<'a, U>, Self>
    where
        F: FnOnce(&T) -> Option<&U>,
        U: ?Sized,
    {
        exclusive_feature_select_expr! {
            "rc" | "gc" => {
                BorrowImpl::filter_map(borrowed.0, f).map(Borrow).map_err(Borrow)
            }
            "arc" | "agc" => {
                BorrowImpl::try_map(borrowed.0, f).map(Borrow).map_err(Borrow)
            }
        }
    }
}

impl<T: ?Sized> Deref for Borrow<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.0.deref()
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for Borrow<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A mutably borrowed reference to a value borrowed from a [PtrMut]
pub struct BorrowMut<'a, T: ?Sized>(BorrowMutImpl<'a, T>);

impl<'a, T: ?Sized> BorrowMut<'a, T> {
    /// Makes a new BorrowMut for an optional component of the borrowed data.
    /// If the closure returns None then the original borrow is returned as the error.
    pub fn filter_map<U, F>(borrowed: Self, f: F) -> Result<BorrowMut<'a, U>, Self>
    where
        F: FnOnce(&mut T) -> Option<&mut U>,
        U: ?Sized,
    {
        exclusive_feature_select_expr! {
            "rc" | "gc" => {
                BorrowMutImpl::filter_map(borrowed.0, f).map(BorrowMut).map_err(BorrowMut)
            }
            "arc" | "agc" => {
                BorrowMutImpl::try_map(borrowed.0, f).map(BorrowMut).map_err(BorrowMut)
            }
        }
    }
}

impl<T: ?Sized> Deref for BorrowMut<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.0.deref()
    }
}

impl<T: ?Sized> DerefMut for BorrowMut<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.0.deref_mut()
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for BorrowMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
