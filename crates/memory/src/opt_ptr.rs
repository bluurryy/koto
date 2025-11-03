use std::fmt;

use crate::{
    KCell, PointeeTraits, Ptr,
    macros::exclusive_feature_select,
    ptr_impl::{OptPtrImpl, PtrImpl},
};

pub type OptPtrMut<T> = OptPtr<KCell<T>>;

/// TODO
pub struct OptPtr<T: PointeeTraits + ?Sized>(OptPtrImpl<T>);

// TODO: Debug, Default, Clone,

impl<T: PointeeTraits> OptPtr<T> {
    /// TODO
    pub const NONE: Self = Self(Self::NONE_IMPL);

    // TODO: make this macro expr compatible, so we can use it inline above
    exclusive_feature_select! {
        "gc" | "agc" => {
            const NONE_IMPL: OptPtrImpl<T> = OptPtrImpl::NONE;
        }
        "rc" | "arc" => {
            const NONE_IMPL: OptPtrImpl<T> = OptPtrImpl::None;
        }
        _ => {
            const NONE_IMPL: OptPtrImpl<T> = OptPtrImpl::None;
        }
    }
}

impl<T: PointeeTraits + ?Sized> OptPtr<T> {
    exclusive_feature_select! {
        "gc" | "agc" => {
            fn some_impl(ptr: Ptr<T>) -> Self {
                Self(OptPtrImpl::some(ptr.into_inner()))
            }
        }
        "rc" | "arc" => {
            fn some_impl(ptr: Ptr<T>) -> Self {
                Self(Some(ptr.into_inner()))
            }
        }
        _ => {
            fn some_impl(ptr: Ptr<T>) -> Self {
                Self(Some(ptr))
            }
        }
    }

    /// Create an `OptPtr<T>` from a `Ptr<T>`.
    #[inline]
    #[must_use]
    pub fn some(ptr: Ptr<T>) -> Self {
        Self::some_impl(ptr)
    }

    #[inline]
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    #[inline]
    pub fn as_ref(&self) -> Option<&Ptr<T>> {
        self.0.as_ref().map(Ptr::from_inner_ref)
    }

    #[inline]
    pub fn as_mut(&mut self) -> Option<&mut Ptr<T>> {
        self.0.as_mut().map(Ptr::from_inner_mut)
    }

    #[inline]
    pub fn into_option(self) -> Option<Ptr<T>> {
        // its not useless for `gc` or `agc`
        #[allow(clippy::useless_conversion)]
        <Option<PtrImpl<T>>>::from(self.0).map(Ptr::from_inner)
    }

    /// Inserts the default value if this option [`is_none`],
    /// then returns a mutable reference to the contained `Ptr<T>`.
    ///
    /// [`is_none`]: Self::is_none
    #[inline]
    pub fn get_or_insert_default(&mut self) -> &mut Ptr<T>
    where
        T: Default,
    {
        self.get_or_insert_with(Default::default)
    }

    /// Inserts a `Ptr<T>` computed from `f` if this option [`is_none`],
    /// then returns a mutable reference to the contained `Ptr<T>`.
    ///
    /// [`is_none`]: Self::is_none
    #[inline]
    pub fn get_or_insert_with(&mut self, f: impl FnOnce() -> Ptr<T>) -> &mut Ptr<T> {
        Ptr::from_inner_mut(self.0.get_or_insert_with(|| f().into_inner()))
    }

    // TODO: unwrap, expect, map, into_option
}

impl<T: PointeeTraits + ?Sized + fmt::Debug> fmt::Debug for OptPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_ref(), f)
    }
}

impl<T: PointeeTraits> Default for OptPtr<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

// TODO: ?Sized
impl<T: PointeeTraits + ?Sized> Clone for OptPtr<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(OptPtrImpl::clone(&self.0))
    }
}

impl<T: PointeeTraits> From<Option<Ptr<T>>> for OptPtr<T> {
    #[inline]
    fn from(value: Option<Ptr<T>>) -> Self {
        Self(OptPtrImpl::from(value.map(Ptr::into_inner)))
    }
}

impl<T: PointeeTraits + ?Sized> From<OptPtr<T>> for Option<Ptr<T>> {
    #[inline]
    fn from(value: OptPtr<T>) -> Self {
        value.into_option()
    }
}

exclusive_feature_select! {
    "gc" | "agc" => {
        unsafe impl<V: dumpster::Visitor, T: PointeeTraits + ?Sized> dumpster::TraceWith<V> for OptPtr<T> {
            #[inline]
            fn accept(&self, visitor: &mut V) -> Result<(), ()> {
                self.0.accept(visitor)
            }
        }
    }
    "rc" | "arc" => {}
    _ => {
        // TODO: make this arm optional
    }
}
