#![no_std]
#![doc = include_str!("../README.md")]

extern crate alloc;

/// Represents a socialist data container.
///
/// This should only be implemented on types that have semantics similar to [`Rc`](alloc::rc::Rc) or [`Arc`](alloc::sync::Arc).
pub trait Comrade {
    fn from_slice(s: &[u8]) -> Self;
    fn as_slice(&self) -> &[u8];
}

macro_rules! impl_comrade {
    ($($(#[$a:meta])* $t:ident$(::$tt:ident)*),*) => {$(
        $(#[$a])* impl<T: core::ops::Deref<Target = [u8]> + for<'a> From<&'a [u8]>> Comrade for $t$(::$tt)*<T> {
            fn from_slice(s: &[u8]) -> Self { $t$(::$tt)*::new(T::from(s)) }
            fn as_slice(&self) -> &[u8] { self }
        }
        $(#[$a])* impl Comrade for $t$(::$tt)*<[u8]> {
            fn from_slice(s: &[u8]) -> Self { $t$(::$tt)*::from(s) }
            fn as_slice(&self) -> &[u8] { self }
        }
    )*};
}
impl_comrade! { alloc::rc::Rc, #[cfg(target_has_atomic = "ptr")] alloc::sync::Arc }

mod bytes;
mod string;
pub mod comrades;

pub use bytes::*;
pub use string::*;
