use core::cmp::{Ordering, PartialEq, Eq, PartialOrd, Ord};
use core::fmt::{self, Debug};
use core::borrow::Borrow;
use core::num::NonZero;
use core::ops::Deref;
use core::hash::Hash;

use crate::Comrade;

#[derive(Clone)]
enum OurInner<T, const N: usize> {
    Inline { len: core::num::NonZero<u8>, content: [u8; N] },
    Outline { content: T },
}

/// A customizable immutable shared byte collection.
///
/// Data is backed inline up to `N` bytes (max 254), or stored dynamically by (shared) [`Comrade`] `T`.
///
/// This type can be constructed via the [`From`] trait given either a `&[u8]` (in which case inlining is attempted but may result in a shared `T` allocation)
/// or a shared handle of type `T` (in which case no inlining is used and the shared handle is simply wrapped).
///
/// Because of this, it is recommended to not use the `T` constructor unless you are already sharing the value around as type `T` elsewhere.
#[derive(Clone)]
pub struct OurBytes<T: Comrade, const N: usize>(OurInner<T, N>);

impl<T: Comrade, const N: usize> OurBytes<T, N> {
    /// Creates a new empty instance of [`OurBytes`] with inlined data.
    pub const fn new() -> Self {
        Self(OurInner::Inline { len: NonZero::<u8>::MAX, content: [0; N] })
    }
    /// Converts this [`OurBytes`] instance into another [`OurBytes`] type which uses the same shared type `T`.
    ///
    /// If the content of this instance is already allocated via shared handle `T`, that handle will simply be reused without inlining.
    /// Otherwise, re-inlining will be attempted, but may fail if `M < N` and result in a new shared `T` allocation.
    ///
    /// Because of this, it is advised to minimize the use of this function (e.g., by only using one [`OurBytes`] type throughout your codebase).
    pub fn convert<const M: usize>(self) -> OurBytes<T, M> {
        match self.0 {
            OurInner::Inline { .. } => OurBytes::from(self.as_slice()),
            OurInner::Outline { content } => OurBytes::from(content),
        }
    }
    /// Gets a shared reference to the content.
    pub fn as_slice(&self) -> &[u8] {
        self
    }
}

impl<T: Comrade, const N: usize> Deref for OurBytes<T, N> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        match &self.0 {
            OurInner::Inline { len, content } => &content[..(!len.get()) as usize],
            OurInner::Outline { content } => content.as_slice(),
        }
    }
}

impl<T: Comrade, const N: usize> From<&[u8]> for OurBytes<T, N> {
    fn from(value: &[u8]) -> Self {
        if value.len() <= N && value.len() < u8::MAX as usize {
            let mut content = [0; N];
            content[..value.len()].copy_from_slice(value);
            Self(OurInner::Inline { len: NonZero::new(!(value.len() as u8)).unwrap(), content })
        } else {
            Self(OurInner::Outline { content: T::from_slice(value) })
        }
    }
}

impl<T: Comrade, const N: usize> From<T> for OurBytes<T, N> {
    fn from(content: T) -> Self {
        Self(OurInner::Outline { content })
    }
}

impl<T: Comrade, const N: usize> Default for OurBytes<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Comrade, const N: usize> AsRef<[u8]> for OurBytes<T, N> {
    fn as_ref(&self) -> &[u8] {
        self
    }
}

impl<T: Comrade, const N: usize> Borrow<[u8]> for OurBytes<T, N> {
    fn borrow(&self) -> &[u8] {
        self
    }
}

impl<T: Comrade, const N: usize> Debug for OurBytes<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <[u8] as Debug>::fmt(&**self, f)
    }
}

impl<T: Comrade, const N: usize> Hash for OurBytes<T, N> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

impl<U: Deref<Target = [u8]>, T: Comrade, const N: usize> PartialEq<U> for OurBytes<T, N> {
    fn eq(&self, other: &U) -> bool {
        (**self).eq(&**other)
    }
}

impl<T: Comrade, const N: usize> PartialEq<OurBytes<T, N>> for &[u8] {
    fn eq(&self, other: &OurBytes<T, N>) -> bool {
        (**self).eq(&**other)
    }
}

impl<T: Comrade, const N: usize> Eq for OurBytes<T, N> {}

impl<U: Deref<Target = [u8]>, T: Comrade, const N: usize> PartialOrd<U> for OurBytes<T, N> {
    fn partial_cmp(&self, other: &U) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T: Comrade, const N: usize> PartialOrd<OurBytes<T, N>> for &[u8] {
    fn partial_cmp(&self, other: &OurBytes<T, N>) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T: Comrade, const N: usize> Ord for OurBytes<T, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}
