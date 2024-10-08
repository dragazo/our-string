use our_string::comrades::{RcBytes, ArcBytes};

#[test]
fn test_traits() {
    macro_rules! assert_impl {
        ($t:ty : $($tr:tt)*) => {{
            fn checker<T: $($tr)*>() {}
            checker::<$t>();
        }};
    }
    macro_rules! assert_not_impl {
        ($t:ty : $($tr:tt)*) => {
            const _: fn() -> () = || {
                struct Check<T: ?Sized>(T);
                trait Foo<A> { fn foo() {} }
                impl<T: ?Sized> Foo<()> for Check<T> {}
                impl<T: ?Sized + $($tr)*> Foo<u8> for Check<T> {}
                <Check::<$t> as Foo<_>>::foo()
            };
        };
    }

    assert_impl!(std::sync::Arc<[u8]> : Send + Sync);
    assert_not_impl!(std::rc::Rc<[u8]> : Send);
    assert_not_impl!(std::rc::Rc<[u8]> : Sync);

    assert_impl!(ArcBytes : Send + Sync + core::fmt::Debug + Default + Clone + PartialEq + Eq + PartialOrd + Ord + core::ops::Deref<Target = [u8]> + AsRef<[u8]> + core::borrow::Borrow<[u8]> + core::hash::Hash);
    assert_impl!(RcBytes : core::fmt::Debug + Default + Clone + PartialEq + Eq + PartialOrd + Ord + core::ops::Deref<Target = [u8]> + AsRef<[u8]> + core::borrow::Borrow<[u8]> + core::hash::Hash);
    assert_not_impl!(RcBytes : Send);
    assert_not_impl!(RcBytes : Sync);
}

#[test]
fn test_rc_bytes() {
    assert_eq!(size_of::<RcBytes>(), size_of::<usize>());
    assert_eq!(size_of::<Option<RcBytes>>(), size_of::<usize>());

    for value in ["".as_bytes(), b"h", b"he", b"hel", b"help", b"help me obi-wan kenobi, you're my only hope"] {
        let v = RcBytes::from(value);
        assert_eq!(v, value);
        assert_eq!(&*v, value);
        assert_ne!(v.as_ptr(), value.as_ptr());
        let vv = v.clone();
        assert_eq!(vv, value);
        assert_eq!(&*vv, value);
        assert_ne!(v.as_ptr(), value.as_ptr());
        assert_eq!(v.as_ptr(), vv.as_ptr());
    }

    let empty = RcBytes::default();
    assert_eq!(empty.is_empty(), true);
    assert_eq!(empty, &[] as &[u8]);
}

#[test]
fn test_arc_bytes() {
    assert_eq!(size_of::<ArcBytes>(), size_of::<usize>());
    assert_eq!(size_of::<Option<ArcBytes>>(), size_of::<usize>());
}
