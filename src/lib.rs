#![no_std]
#![forbid(unsafe_code)]

#![doc = include_str!("../README.md")]

extern crate alloc;

mod private {
    pub trait Sealed {}
    impl<T: ?Sized> Sealed for alloc::rc::Rc<T> {}
    #[cfg(target_has_atomic = "ptr")]
    impl<T: ?Sized> Sealed for alloc::sync::Arc<T> {}
}

macro_rules! comrade {
    ($n:ident : $r:ty) => {
        comrade! { $n : $r : alloc::rc::Rc, #[cfg(target_has_atomic = "ptr")] alloc::sync::Arc }
    };
    ($n:ident : $r:ty : $($(#[$a:meta])* $t:ident$(::$tt:ident)*),*) => {
        /// Represents a socialist data container.
        ///
        /// This trait has been sealed away by the People to ensure it cannot be implemented on capitalist types like [`Box`](alloc::boxed::Box) and [`Vec`](alloc::vec::Vec).
        pub trait $n: crate::private::Sealed {
            fn from_slice(s: &$r) -> Self;
            fn as_slice(&self) -> &$r;
        }
        $(
            $(#[$a])* impl<T: core::ops::Deref<Target = $r> + for<'a> From<&'a $r>> $n for $t$(::$tt)*<T> {
                fn from_slice(s: &$r) -> Self { $t$(::$tt)*::new(T::from(s)) }
                fn as_slice(&self) -> &$r { self }
            }
            $(#[$a])* impl $n for $t$(::$tt)*<$r> {
                fn from_slice(s: &$r) -> Self { $t$(::$tt)*::from(s) }
                fn as_slice(&self) -> &$r { self }
            }
        )*
    };
}

#[derive(Clone)]
enum OurInner<T, const N: usize> {
    Inline { len: core::num::NonZero<u8>, content: [u8; N] },
    Outline { content: T },
}

mod bytes;

pub use bytes::*;
