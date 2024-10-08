use core::cmp::{Ordering, PartialEq, Eq, PartialOrd, Ord};
use core::fmt::{self, Debug, Display};
use core::borrow::Borrow;
use core::ops::Deref;
use core::hash::Hash;

use crate::Comrade;

/// A customizable immutable shared string.
///
/// Data is backed inline up to `N` bytes (max 254), or stored dynamically by (shared) [`Comrade`] `T`.
///
/// This type can be constructed via the [`From`] trait given either a `&str` (in which case inlining is attempted but may result in a shared `T` allocation)
/// or a shared handle of type `T` (in which case no inlining is used and the shared handle is simply wrapped).
///
/// Because of this, it is recommended to not use the `T` constructor unless you are already sharing the value around as type `T` elsewhere.
#[derive(Default, Clone)]
pub struct OurString<T: Comrade, const N: usize>(crate::OurBytes<T, N>);

impl<T: Comrade, const N: usize> OurString<T, N> {
    /// Creates a new empty instance of [`OurString`] with inlined data.
    pub const fn new() -> Self {
        Self(crate::OurBytes::new())
    }
    /// Converts this [`OurString`] instance into another [`OurString`] type which uses the same shared type `T`.
    ///
    /// If the content of this instance is already allocated via shared handle `T`, that handle will simply be reused without inlining.
    /// Otherwise, re-inlining will be attempted, but may fail if `M < N` and result in a new shared `T` allocation.
    ///
    /// Because of this, it is advised to minimize the use of this function (e.g., by only using one [`OurString`] type throughout your codebase).
    pub fn convert<const M: usize>(self) -> OurString<T, M> {
        OurString(self.0.convert())
    }
    /// Gets a shared reference to the content.
    pub fn as_str(&self) -> &str {
        self
    }
    /// Extracts the underlying shared bytes container.
    pub fn into_bytes(self) -> crate::OurBytes<T, N> {
        self.0
    }
    /// Attempts to construct a new [`OurString`] instance from the underlying shared bytes container.
    pub fn from_utf8(value: crate::OurBytes<T, N>) -> Result<Self, core::str::Utf8Error> {
        core::str::from_utf8(&value)?;
        Ok(Self(value))
    }
}

impl<T: Comrade, const N: usize> Deref for OurString<T, N> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        unsafe { core::str::from_utf8_unchecked(&self.0) }
    }
}

impl<T: Comrade, const N: usize> From<&str> for OurString<T, N> {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().into())
    }
}

impl<T: Comrade, const N: usize> From<T> for OurString<T, N> {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<T: Comrade, const N: usize> AsRef<str> for OurString<T, N> {
    fn as_ref(&self) -> &str {
        self
    }
}

impl<T: Comrade, const N: usize> Borrow<str> for OurString<T, N> {
    fn borrow(&self) -> &str {
        self
    }
}

impl<T: Comrade, const N: usize> Debug for OurString<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <str as Debug>::fmt(&**self, f)
    }
}

impl<T: Comrade, const N: usize> Display for OurString<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <str as Display>::fmt(&**self, f)
    }
}

impl<T: Comrade, const N: usize> Hash for OurString<T, N> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

impl<U: Deref<Target = str>, T: Comrade, const N: usize> PartialEq<U> for OurString<T, N> {
    fn eq(&self, other: &U) -> bool {
        (**self).eq(&**other)
    }
}

impl<T: Comrade, const N: usize> PartialEq<OurString<T, N>> for &str {
    fn eq(&self, other: &OurString<T, N>) -> bool {
        (**self).eq(&**other)
    }
}

impl<T: Comrade, const N: usize> Eq for OurString<T, N> {}

impl<U: Deref<Target = str>, T: Comrade, const N: usize> PartialOrd<U> for OurString<T, N> {
    fn partial_cmp(&self, other: &U) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T: Comrade, const N: usize> PartialOrd<OurString<T, N>> for &str {
    fn partial_cmp(&self, other: &OurString<T, N>) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T: Comrade, const N: usize> Ord for OurString<T, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}
