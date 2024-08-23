use core::cmp::{Ordering, PartialEq, Eq, PartialOrd, Ord};
use core::fmt::{self, Debug};
use core::borrow::Borrow;
use core::num::NonZero;
use core::ops::Deref;
use core::hash::Hash;

comrade! { BytesComrade : [u8] }

/// A customizable immutable shared byte collection.
///
/// Data is backed inline up to `N` bytes (max 255), or stored dynamically by [`BytesComrade`] `T`.
#[derive(Clone)]
pub struct OurBytes<T: BytesComrade, const N: usize>(crate::OurInner<T, N>);

impl<T: BytesComrade, const N: usize> OurBytes<T, N> {
    /// Creates a new empty instance of [`OurBytes`] with inlined data.
    pub const fn new() -> Self {
        Self(crate::OurInner::Inline { len: NonZero::<u8>::MAX, content: [0; N] })
    }
    /// Attempts to create a new instance of [`OurBytes`] with inlined data.
    pub fn new_inline(s: &[u8]) -> Option<Self> {
        if s.len() <= N && s.len() < u8::MAX as usize {
            let mut content = [0; N];
            content[..s.len()].copy_from_slice(s);
            Some(Self(crate::OurInner::Inline { len: NonZero::new(!(s.len() as u8)).unwrap(), content }))
        } else {
            None
        }
    }
    /// Gets a shared reference to the content.
    pub fn as_slice(&self) -> &[u8] {
        self
    }
}

impl<T: BytesComrade, const N: usize> Deref for OurBytes<T, N> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        match &self.0 {
            crate::OurInner::Inline { len, content } => &content[..(!len.get()) as usize],
            crate::OurInner::Outline { content } => content.as_slice(),
        }
    }
}

impl<T: BytesComrade, const N: usize> From<&[u8]> for OurBytes<T, N> {
    fn from(value: &[u8]) -> Self {
        Self::new_inline(value).unwrap_or_else(|| Self(crate::OurInner::Outline { content: T::from_slice(value) }))
    }
}
impl<T: BytesComrade, const N: usize> From<T> for OurBytes<T, N> {
    fn from(value: T) -> Self {
        Self::new_inline(value.as_slice()).unwrap_or_else(|| Self(crate::OurInner::Outline { content: value }))
    }
}

impl<T: BytesComrade, const N: usize> Default for OurBytes<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: BytesComrade, const N: usize> AsRef<[u8]> for OurBytes<T, N> {
    fn as_ref(&self) -> &[u8] {
        self
    }
}

impl<T: BytesComrade, const N: usize> Borrow<[u8]> for OurBytes<T, N> {
    fn borrow(&self) -> &[u8] {
        self
    }
}

impl<T: BytesComrade, const N: usize> Debug for OurBytes<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <[u8] as Debug>::fmt(&**self, f)
    }
}

impl<T: BytesComrade, const N: usize> Hash for OurBytes<T, N> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

impl<U: Deref<Target = [u8]>, T: BytesComrade, const N: usize> PartialEq<U> for OurBytes<T, N> {
    fn eq(&self, other: &U) -> bool {
        (**self).eq(&**other)
    }
}
impl<T: BytesComrade, const N: usize> PartialEq<OurBytes<T, N>> for &[u8] {
    fn eq(&self, other: &OurBytes<T, N>) -> bool {
        (**self).eq(&**other)
    }
}
impl<T: BytesComrade, const N: usize> Eq for OurBytes<T, N> {}

impl<U: Deref<Target = [u8]>, T: BytesComrade, const N: usize> PartialOrd<U> for OurBytes<T, N> {
    fn partial_cmp(&self, other: &U) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}
impl<T: BytesComrade, const N: usize> PartialOrd<OurBytes<T, N>> for &[u8] {
    fn partial_cmp(&self, other: &OurBytes<T, N>) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T: BytesComrade, const N: usize> Ord for OurBytes<T, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}
