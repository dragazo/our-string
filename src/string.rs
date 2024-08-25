use core::cmp::{Ordering, PartialEq, Eq, PartialOrd, Ord};
use core::fmt::{self, Debug, Display};
use core::borrow::Borrow;
use core::num::NonZero;
use core::ops::Deref;
use core::hash::Hash;

comrade! { StringComrade : str }

/// A customizable immutable shared string.
///
/// Data is backed inline up to `N` bytes (max 255), or stored dynamically by (shared) [`StringComrade`] `T`.
///
/// This type can be constructed via the [`From`] trait given either a `&str` (in which case inlining is attempted but may result in a shared `T` allocation)
/// or a shared handle of type `T` (in which case no inlining is used and the shared handle is simply wrapped).
///
/// Because of this, it is recommended to not use the `T` constructor unless you are already sharing the value around as type `T` elsewhere.
#[derive(Clone)]
pub struct OurString<T: StringComrade, const N: usize>(crate::OurInner<T, N>);

impl<T: StringComrade, const N: usize> OurString<T, N> {
    /// Creates a new empty instance of [`OurString`] with inlined data.
    pub const fn new() -> Self {
        Self(crate::OurInner::Inline { len: NonZero::<u8>::MAX, content: [0; N] })
    }
    /// Converts this [`OurString`] instance into another [`OurString`] type which uses the same shared type `T`.
    ///
    /// If the content of this instance is already allocated via shared handle `T`, that handle will simply be reused without inlining.
    /// Otherwise, re-inlining will be attempted, but may fail if `M < N` and result in a new shared `T` allocation.
    ///
    /// Because of this, it is advised to minimize the use of this function (e.g., by only using one [`OurString`] type throughout your codebase).
    pub fn convert<const M: usize>(self) -> OurString<T, M> {
        match self.0 {
            crate::OurInner::Inline { .. } => OurString::from(self.as_str()),
            crate::OurInner::Outline { content } => OurString::from(content),
        }
    }
    /// Gets a shared reference to the content.
    pub fn as_str(&self) -> &str {
        self
    }
}

impl<T: StringComrade, const N: usize> Deref for OurString<T, N> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match &self.0 {
            crate::OurInner::Inline { len, content } => core::str::from_utf8(&content[..(!len.get()) as usize]).unwrap(),
            crate::OurInner::Outline { content } => content.as_slice(),
        }
    }
}

impl<T: StringComrade, const N: usize> From<&str> for OurString<T, N> {
    fn from(value: &str) -> Self {
        if value.len() <= N && value.len() < u8::MAX as usize {
            let mut content = [0; N];
            content[..value.len()].copy_from_slice(value.as_bytes());
            Self(crate::OurInner::Inline { len: NonZero::new(!(value.len() as u8)).unwrap(), content })
        } else {
            Self(crate::OurInner::Outline { content: T::from_slice(value) })
        }
    }
}

impl<T: StringComrade, const N: usize> From<T> for OurString<T, N> {
    fn from(value: T) -> Self {
        Self(crate::OurInner::Outline { content: value })
    }
}

impl<T: StringComrade, const N: usize> Default for OurString<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: StringComrade, const N: usize> AsRef<str> for OurString<T, N> {
    fn as_ref(&self) -> &str {
        self
    }
}

impl<T: StringComrade, const N: usize> Borrow<str> for OurString<T, N> {
    fn borrow(&self) -> &str {
        self
    }
}

impl<T: StringComrade, const N: usize> Debug for OurString<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <str as Debug>::fmt(&**self, f)
    }
}

impl<T: StringComrade, const N: usize> Display for OurString<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <str as Display>::fmt(&**self, f)
    }
}

impl<T: StringComrade, const N: usize> Hash for OurString<T, N> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

impl<U: Deref<Target = str>, T: StringComrade, const N: usize> PartialEq<U> for OurString<T, N> {
    fn eq(&self, other: &U) -> bool {
        (**self).eq(&**other)
    }
}

impl<T: StringComrade, const N: usize> PartialEq<OurString<T, N>> for &str {
    fn eq(&self, other: &OurString<T, N>) -> bool {
        (**self).eq(&**other)
    }
}

impl<T: StringComrade, const N: usize> Eq for OurString<T, N> {}

impl<U: Deref<Target = str>, T: StringComrade, const N: usize> PartialOrd<U> for OurString<T, N> {
    fn partial_cmp(&self, other: &U) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T: StringComrade, const N: usize> PartialOrd<OurString<T, N>> for &str {
    fn partial_cmp(&self, other: &OurString<T, N>) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T: StringComrade, const N: usize> Ord for OurString<T, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}
