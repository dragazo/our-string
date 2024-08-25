use std::cmp::{Ordering, PartialEq, Eq, PartialOrd, Ord};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::fmt::{Debug, Display};
use std::borrow::Borrow;
use std::mem::size_of;
use std::ops::Deref;
use std::sync::Arc;
use std::rc::Rc;

use our_string::{OurString, StringComrade};

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn is_inline<T: StringComrade, const N: usize>(v: &OurString<T, N>) -> bool {
    let l = v.len();
    let s = v.as_str() as *const str as *const () as usize;
    let v = v as *const OurString<T, N> as *const () as usize;
    s >= v && s + l <= v + size_of::<OurString<T, N>>()
}

#[test]
fn test_sizes() {
    assert_eq!(size_of::<OurString<Rc<String>, { size_of::<String>() - 1 }>>(), size_of::<String>());
    assert_eq!(size_of::<OurString<Rc<String>, { size_of::<String>() - 1 - size_of::<usize>() }>>(), size_of::<String>() - size_of::<usize>());

    assert_eq!(size_of::<OurString<Arc<String>, { size_of::<String>() - 1 }>>(), size_of::<String>());
    assert_eq!(size_of::<OurString<Arc<String>, { size_of::<String>() - 1 - size_of::<usize>() }>>(), size_of::<String>() - size_of::<usize>());

    assert_eq!(size_of::<OurString<Rc<str>, { size_of::<String>() - 1 }>>(), size_of::<String>());

    assert_eq!(size_of::<OurString<Arc<str>, { size_of::<String>() - 1 }>>(), size_of::<String>());
}

#[test]
fn test_traits() {
    macro_rules! assert_impl {
        ($t:ty : $($tr:tt)*) => {{
            fn checker<T: $($tr)*>() {}
            checker::<$t>();
        }};
    }

    assert_impl!(OurString<Rc<String>, 8> : Hash + Clone + Debug + Display + PartialEq + Eq + PartialOrd + Ord + Default + AsRef<str> + Borrow<str> + Deref<Target = str> + for<'a> From<&'a str> + From<Rc<String>>);
    assert_impl!(OurString<Rc<str>,    8> : Hash + Clone + Debug + Display + PartialEq + Eq + PartialOrd + Ord + Default + AsRef<str> + Borrow<str> + Deref<Target = str> + for<'a> From<&'a str> + From<Rc<str>>);

    assert_impl!(OurString<Arc<String>, 8> : Send + Sync + Hash + Clone + Debug + Display + PartialEq + Eq + PartialOrd + Ord + Default + AsRef<str> + Borrow<str> + Deref<Target = str> + for<'a> From<&'a str> + From<Arc<String>>);
    assert_impl!(OurString<Arc<str>,    8> : Send + Sync + Hash + Clone + Debug + Display + PartialEq + Eq + PartialOrd + Ord + Default + AsRef<str> + Borrow<str> + Deref<Target = str> + for<'a> From<&'a str> + From<Arc<str>>);
}

#[test]
fn test_clone() {
    let a = OurString::<Rc<String>, 5>::from("hello world again");
    let b = a.clone();
    assert_eq!(a, b);
    assert_eq!(a.as_str(), b.as_str());
    assert_eq!(a.as_str() as *const str, b.as_str() as *const str);
}
proptest::proptest! {
    #[test]
    fn proptest_clone(s: String) {
        let a = OurString::<Rc<String>, 5>::from(s.as_str());
        let b = a.clone();
        assert_eq!(a, b);
        assert_eq!(a.as_str(), b.as_str());
        if a.len() > 5 {
            assert_eq!(a.as_str() as *const str, b.as_str() as *const str);
        }
    }
}

#[test]
fn test_new_default() {
    const X: OurString<Rc<String>, 10> = OurString::new();
    const Y: OurString<Arc<str>, 10> = OurString::new();

    assert_eq!(X.len(), 0);
    assert_eq!(X.is_empty(), true);
    assert_eq!(X.as_str().is_empty(), true);

    assert_eq!(Y.len(), 0);
    assert_eq!(Y.is_empty(), true);
    assert_eq!(Y.as_str().is_empty(), true);

    assert_eq!(OurString::<Rc<String>, 10>::default().len(), 0);
    assert_eq!(OurString::<Rc<String>, 10>::default().is_empty(), true);
    assert_eq!(OurString::<Rc<String>, 10>::default().as_str().is_empty(), true);

    assert_eq!(OurString::<Arc<str>, 10>::default().len(), 0);
    assert_eq!(OurString::<Arc<str>, 10>::default().is_empty(), true);
    assert_eq!(OurString::<Arc<str>, 10>::default().as_str().is_empty(), true);
}

#[test]
fn test_from_slice_inlining() {
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from("")), true);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from("h")), true);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from("he")), true);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from("hel")), true);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from("hell")), true);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from("hello")), true);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from("hello ")), true);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from("hello from")), true);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from("hello from ")), false);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from("hello from the")), false);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from("hello from the other")), false);

    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from("")), true);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from("h")), true);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from("he")), true);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from("hel")), true);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from("hell")), true);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from("hello")), false);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from("hello ")), false);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from("hello from")), false);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from("hello from ")), false);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from("hello from the")), false);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from("hello from the other")), false);
}

#[test]
fn test_from_slice() {
    assert_eq!(&*OurString::<Rc<String>, 10>::from("") as &str, "");
    assert_eq!(&*OurString::<Rc<String>, 10>::from("h") as &str, "h");
    assert_eq!(&*OurString::<Rc<String>, 10>::from("he") as &str, "he");
    assert_eq!(&*OurString::<Rc<String>, 10>::from("hel") as &str, "hel");
    assert_eq!(&*OurString::<Rc<String>, 10>::from("hell") as &str, "hell");
    assert_eq!(&*OurString::<Rc<String>, 10>::from("hello") as &str, "hello");
    assert_eq!(&*OurString::<Rc<String>, 10>::from("hello ") as &str, "hello ");
    assert_eq!(&*OurString::<Rc<String>, 10>::from("hello from") as &str, "hello from");
    assert_eq!(&*OurString::<Rc<String>, 10>::from("hello from ") as &str, "hello from ");
    assert_eq!(&*OurString::<Rc<String>, 10>::from("hello from the") as &str, "hello from the");
    assert_eq!(&*OurString::<Rc<String>, 10>::from("hello from the other") as &str, "hello from the other");

    assert_eq!(&*OurString::<Arc<String>, 4>::from("") as &str, "");
    assert_eq!(&*OurString::<Arc<String>, 4>::from("h") as &str, "h");
    assert_eq!(&*OurString::<Arc<String>, 4>::from("he") as &str, "he");
    assert_eq!(&*OurString::<Arc<String>, 4>::from("hel") as &str, "hel");
    assert_eq!(&*OurString::<Arc<String>, 4>::from("hell") as &str, "hell");
    assert_eq!(&*OurString::<Arc<String>, 4>::from("hello") as &str, "hello");
    assert_eq!(&*OurString::<Arc<String>, 4>::from("hello ") as &str, "hello ");
    assert_eq!(&*OurString::<Arc<String>, 4>::from("hello from") as &str, "hello from");
    assert_eq!(&*OurString::<Arc<String>, 4>::from("hello from ") as &str, "hello from ");
    assert_eq!(&*OurString::<Arc<String>, 4>::from("hello from the") as &str, "hello from the");
    assert_eq!(&*OurString::<Arc<String>, 4>::from("hello from the other") as &str, "hello from the other");
}
proptest::proptest! {
    #[test]
    fn proptest_from_slice(s: String) {
        assert_eq!(&*OurString::<Rc<String>, 10>::from(s.as_str()) as &str, s.as_str());
        assert_eq!(&*OurString::<Arc<String>, 10>::from(s.as_str()) as &str, s.as_str());
        assert_eq!(&*OurString::<Rc<str>, 10>::from(s.as_str()) as &str, s.as_str());
        assert_eq!(&*OurString::<Arc<str>, 10>::from(s.as_str()) as &str, s.as_str());

        assert_eq!(OurString::<Rc<String>, 10>::from(s.as_str()).as_str() as &str, s.as_str());
        assert_eq!(OurString::<Arc<String>, 10>::from(s.as_str()).as_str() as &str, s.as_str());
        assert_eq!(OurString::<Rc<str>, 10>::from(s.as_str()).as_str() as &str, s.as_str());
        assert_eq!(OurString::<Arc<str>, 10>::from(s.as_str()).as_str() as &str, s.as_str());

        assert_eq!(OurString::<Rc<String>, 10>::from(s.as_str()).as_ref() as &str, s.as_str());
        assert_eq!(OurString::<Arc<String>, 10>::from(s.as_str()).as_ref() as &str, s.as_str());
        assert_eq!(OurString::<Rc<str>, 10>::from(s.as_str()).as_ref() as &str, s.as_str());
        assert_eq!(OurString::<Arc<str>, 10>::from(s.as_str()).as_ref() as &str, s.as_str());

        assert_eq!(<_ as AsRef<str>>::as_ref(&OurString::<Rc<String>, 10>::from(s.as_str())) as &str, s.as_str());
        assert_eq!(<_ as AsRef<str>>::as_ref(&OurString::<Arc<String>, 10>::from(s.as_str())) as &str, s.as_str());
        assert_eq!(<_ as AsRef<str>>::as_ref(&OurString::<Rc<str>, 10>::from(s.as_str())) as &str, s.as_str());
        assert_eq!(<_ as AsRef<str>>::as_ref(&OurString::<Arc<str>, 10>::from(s.as_str())) as &str, s.as_str());

        assert_eq!(OurString::<Rc<String>, 10>::from(s.as_str()).borrow() as &str, s.as_str());
        assert_eq!(OurString::<Arc<String>, 10>::from(s.as_str()).borrow() as &str, s.as_str());
        assert_eq!(OurString::<Rc<str>, 10>::from(s.as_str()).borrow() as &str, s.as_str());
        assert_eq!(OurString::<Arc<str>, 10>::from(s.as_str()).borrow() as &str, s.as_str());

        assert_eq!(<_ as Borrow<str>>::borrow(&OurString::<Rc<String>, 10>::from(s.as_str())) as &str, s.as_str());
        assert_eq!(<_ as Borrow<str>>::borrow(&OurString::<Arc<String>, 10>::from(s.as_str())) as &str, s.as_str());
        assert_eq!(<_ as Borrow<str>>::borrow(&OurString::<Rc<str>, 10>::from(s.as_str())) as &str, s.as_str());
        assert_eq!(<_ as Borrow<str>>::borrow(&OurString::<Arc<str>, 10>::from(s.as_str())) as &str, s.as_str());
    }
}

#[test]
fn test_hash() {
    assert_eq!(hash(&OurString::<Arc<String>, 4>::from("")), hash(&""));
    assert_eq!(hash(&OurString::<Arc<String>, 4>::from("x")), hash(&"x"));
    assert_eq!(hash(&OurString::<Arc<String>, 4>::from("xy")), hash(&"xy"));
    assert_eq!(hash(&OurString::<Arc<String>, 4>::from("hello world")), hash(&"hello world"));
}
proptest::proptest! {
    #[test]
    fn proptest_hash(s: String) {
        assert_eq!(hash(&OurString::<Rc<String>, 4>::from(s.as_str())), hash(&s.as_str()));
        assert_eq!(hash(&OurString::<Rc<str>, 4>::from(s.as_str())), hash(&s.as_str()));
        assert_eq!(hash(&OurString::<Arc<String>, 4>::from(s.as_str())), hash(&s.as_str()));
        assert_eq!(hash(&OurString::<Arc<str>, 4>::from(s.as_str())), hash(&s.as_str()));
    }
}

#[test]
fn test_from_comrade_inlining() {
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from(Rc::new(String::from("")))), false);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from(Rc::new(String::from("h")))), false);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from(Rc::new(String::from("he")))), false);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from(Rc::new(String::from("hel")))), false);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from(Rc::new(String::from("hell")))), false);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from(Rc::new(String::from("hello")))), false);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from(Rc::new(String::from("hello ")))), false);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from(Rc::new(String::from("hello from")))), false);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from(Rc::new(String::from("hello from ")))), false);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from(Rc::new(String::from("hello from the")))), false);
    assert_eq!(is_inline(&OurString::<Rc<String>, 10>::from(Rc::new(String::from("hello from the other")))), false);

    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from(Arc::new(String::from("")))), false);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from(Arc::new(String::from("h")))), false);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from(Arc::new(String::from("he")))), false);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from(Arc::new(String::from("hel")))), false);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from(Arc::new(String::from("hell")))), false);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from(Arc::new(String::from("hello")))), false);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from(Arc::new(String::from("hello ")))), false);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from(Arc::new(String::from("hello from")))), false);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from(Arc::new(String::from("hello from ")))), false);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from(Arc::new(String::from("hello from the")))), false);
    assert_eq!(is_inline(&OurString::<Arc<String>, 4>::from(Arc::new(String::from("hello from the other")))), false);
}

#[test]
fn test_from_comrade() {
    assert_eq!(&*OurString::<Rc<String>, 10>::from(Rc::new(String::from(""))) as &str, "");
    assert_eq!(&*OurString::<Rc<String>, 10>::from(Rc::new(String::from("h"))) as &str, "h");
    assert_eq!(&*OurString::<Rc<String>, 10>::from(Rc::new(String::from("he"))) as &str, "he");
    assert_eq!(&*OurString::<Rc<String>, 10>::from(Rc::new(String::from("hel"))) as &str, "hel");
    assert_eq!(&*OurString::<Rc<String>, 10>::from(Rc::new(String::from("hell"))) as &str, "hell");
    assert_eq!(&*OurString::<Rc<String>, 10>::from(Rc::new(String::from("hello"))) as &str, "hello");
    assert_eq!(&*OurString::<Rc<String>, 10>::from(Rc::new(String::from("hello "))) as &str, "hello ");
    assert_eq!(&*OurString::<Rc<String>, 10>::from(Rc::new(String::from("hello from"))) as &str, "hello from");
    assert_eq!(&*OurString::<Rc<String>, 10>::from(Rc::new(String::from("hello from "))) as &str, "hello from ");
    assert_eq!(&*OurString::<Rc<String>, 10>::from(Rc::new(String::from("hello from the"))) as &str, "hello from the");
    assert_eq!(&*OurString::<Rc<String>, 10>::from(Rc::new(String::from("hello from the other"))) as &str, "hello from the other");

    assert_eq!(&*OurString::<Arc<String>, 4>::from(Arc::new(String::from(""))) as &str, "");
    assert_eq!(&*OurString::<Arc<String>, 4>::from(Arc::new(String::from("h"))) as &str, "h");
    assert_eq!(&*OurString::<Arc<String>, 4>::from(Arc::new(String::from("he"))) as &str, "he");
    assert_eq!(&*OurString::<Arc<String>, 4>::from(Arc::new(String::from("hel"))) as &str, "hel");
    assert_eq!(&*OurString::<Arc<String>, 4>::from(Arc::new(String::from("hell"))) as &str, "hell");
    assert_eq!(&*OurString::<Arc<String>, 4>::from(Arc::new(String::from("hello"))) as &str, "hello");
    assert_eq!(&*OurString::<Arc<String>, 4>::from(Arc::new(String::from("hello "))) as &str, "hello ");
    assert_eq!(&*OurString::<Arc<String>, 4>::from(Arc::new(String::from("hello from"))) as &str, "hello from");
    assert_eq!(&*OurString::<Arc<String>, 4>::from(Arc::new(String::from("hello from "))) as &str, "hello from ");
    assert_eq!(&*OurString::<Arc<String>, 4>::from(Arc::new(String::from("hello from the"))) as &str, "hello from the");
    assert_eq!(&*OurString::<Arc<String>, 4>::from(Arc::new(String::from("hello from the other"))) as &str, "hello from the other");
}
proptest::proptest! {
    #[test]
    fn proptest_from_comrade(s: String) {
        assert_eq!(&*OurString::<Rc<String>, 10>::from(Rc::new(s.clone())) as &str, s.as_str());
        assert_eq!(&*OurString::<Arc<String>, 10>::from(Arc::new(s.clone())) as &str, s.as_str());
        assert_eq!(&*OurString::<Rc<str>, 10>::from(Rc::from(s.as_str())) as &str, s.as_str());
        assert_eq!(&*OurString::<Arc<str>, 10>::from(Arc::from(s.as_str())) as &str, s.as_str());
    }
}

#[test]
fn test_debug_display() {
    assert_eq!(format!("{:?}", OurString::<Rc<str>, 4>::from("wh?")), format!("{:?}", "wh?"));
    assert_eq!(format!("{:?}", OurString::<Rc<String>, 4>::from("wha?")), format!("{:?}", "wha?"));
    assert_eq!(format!("{:?}", OurString::<Arc<String>, 4>::from("come again?")), format!("{:?}", "come again?"));

    assert_eq!(format!("{}", OurString::<Rc<str>, 4>::from("wh?")), format!("{}", "wh?"));
    assert_eq!(format!("{}", OurString::<Rc<String>, 4>::from("wha?")), format!("{}", "wha?"));
    assert_eq!(format!("{}", OurString::<Arc<String>, 4>::from("come again?")), format!("{}", "come again?"));
}
proptest::proptest! {
    #[test]
    fn proptest_debug_display(s: String) {
        assert_eq!(format!("{:?}", OurString::<Rc<str>, 4>::from(Rc::from(s.as_str()))), format!("{s:?}"));
        assert_eq!(format!("{:?}", OurString::<Arc<str>, 4>::from(Arc::from(s.as_str()))), format!("{s:?}"));

        assert_eq!(format!("{}", OurString::<Rc<str>, 4>::from(Rc::from(s.as_str()))), format!("{s}"));
        assert_eq!(format!("{}", OurString::<Arc<str>, 4>::from(Arc::from(s.as_str()))), format!("{s}"));
    }
}

#[test]
fn test_comparison() {
    let a = "help me";
    let a1 = OurString::<Rc<String>, 10>::from(a);
    let a2 = OurString::<Arc<String>, 3>::from(a);
    let a3 = String::from(a);

    let b = "please";
    let b1 = OurString::<Rc<str>, 7>::from(b);
    let b2 = OurString::<Rc<str>, 2>::from(b);
    let b3 = String::from(b);

    assert_eq!(a1, a ); assert_eq!(a1, a1); assert_eq!(a1, a2); assert_eq!(a1, a3); assert_eq!(a2, a );
    assert_eq!(a2, a1); assert_eq!(a2, a2); assert_eq!(a2, a3); assert_eq!(a,  a1); assert_eq!(a,  a2);

    assert_ne!(b1, a ); assert_ne!(b1, a1); assert_ne!(b1, a2); assert_ne!(b1, a3); assert_ne!(b2, a );
    assert_ne!(b2, a1); assert_ne!(b2, a2); assert_ne!(b2, a3); assert_ne!(b,  a1); assert_ne!(b,  a2);

    assert_ne!(a1, b ); assert_ne!(a1, b1); assert_ne!(a1, b2); assert_ne!(a1, b3); assert_ne!(a2, b );
    assert_ne!(a2, b1); assert_ne!(a2, b2); assert_ne!(a2, b3); assert_ne!(a,  b1); assert_ne!(a,  b2);

    assert_eq!(b1, b ); assert_eq!(b1, b1); assert_eq!(b1, b2); assert_eq!(b1, b3); assert_eq!(b2, b );
    assert_eq!(b2, b1); assert_eq!(b2, b2); assert_eq!(b2, b3); assert_eq!(b,  b1); assert_eq!(b,  b2);

    assert_eq!(a1 <= a , true); assert_eq!(a1 <= a1, true); assert_eq!(a1 <= a2, true); assert_eq!(a1 <= a3, true); assert_eq!(a2 <= a , true);
    assert_eq!(a2 <= a1, true); assert_eq!(a2 <= a2, true); assert_eq!(a2 <= a3, true); assert_eq!(a  <= a1, true); assert_eq!(a  <= a2, true);
    assert_eq!(a1 >= a , true); assert_eq!(a1 >= a1, true); assert_eq!(a1 >= a2, true); assert_eq!(a1 >= a3, true); assert_eq!(a2 >= a , true);
    assert_eq!(a2 >= a1, true); assert_eq!(a2 >= a2, true); assert_eq!(a2 >= a3, true); assert_eq!(a  >= a1, true); assert_eq!(a  >= a2, true);
    assert_eq!(a1 < a , false); assert_eq!(a1 < a1, false); assert_eq!(a1 < a2, false); assert_eq!(a1 < a3, false); assert_eq!(a2 < a , false);
    assert_eq!(a2 < a1, false); assert_eq!(a2 < a2, false); assert_eq!(a2 < a3, false); assert_eq!(a  < a1, false); assert_eq!(a  < a2, false);
    assert_eq!(a1 > a , false); assert_eq!(a1 > a1, false); assert_eq!(a1 > a2, false); assert_eq!(a1 > a3, false); assert_eq!(a2 > a , false);
    assert_eq!(a2 > a1, false); assert_eq!(a2 > a2, false); assert_eq!(a2 > a3, false); assert_eq!(a  > a1, false); assert_eq!(a  > a2, false);

    assert_eq!(b1 <= a , false); assert_eq!(b1 <= a1, false); assert_eq!(b1 <= a2, false); assert_eq!(b1 <= a3, false); assert_eq!(b2 <= a , false);
    assert_eq!(b2 <= a1, false); assert_eq!(b2 <= a2, false); assert_eq!(b2 <= a3, false); assert_eq!(b  <= a1, false); assert_eq!(b  <= a2, false);
    assert_eq!(b1 >= a , true ); assert_eq!(b1 >= a1, true ); assert_eq!(b1 >= a2, true ); assert_eq!(b1 >= a3, true ); assert_eq!(b2 >= a , true );
    assert_eq!(b2 >= a1, true ); assert_eq!(b2 >= a2, true ); assert_eq!(b2 >= a3, true ); assert_eq!(b  >= a1, true ); assert_eq!(b  >= a2, true );
    assert_eq!(b1 < a ,  false); assert_eq!(b1 < a1,  false); assert_eq!(b1 < a2,  false); assert_eq!(b1 < a3,  false); assert_eq!(b2 < a ,  false);
    assert_eq!(b2 < a1,  false); assert_eq!(b2 < a2,  false); assert_eq!(b2 < a3,  false); assert_eq!(b  < a1,  false); assert_eq!(b  < a2,  false);
    assert_eq!(b1 > a ,  true ); assert_eq!(b1 > a1,  true ); assert_eq!(b1 > a2,  true ); assert_eq!(b1 > a3,  true ); assert_eq!(b2 > a ,  true );
    assert_eq!(b2 > a1,  true ); assert_eq!(b2 > a2,  true ); assert_eq!(b2 > a3,  true ); assert_eq!(b  > a1,  true ); assert_eq!(b  > a2,  true );
}
proptest::proptest! {
    #[test]
    fn proptest_comparison(a: String, b: String) {
        fn partial_cmp<A, B>(a: &A, b: &B) -> Option<Ordering> where A: PartialOrd<B> {
            a.partial_cmp(b)
        }
        fn cmp<A>(a: &A, b: &A) -> Ordering where A: Ord {
            a.cmp(b)
        }

        assert_eq!(partial_cmp(&a.as_str(), &OurString::<Rc<str>, 4>::from(b.as_str())), partial_cmp(&a, &b));
        assert_eq!(partial_cmp(&OurString::<Rc<str>, 4>::from(a.as_str()), &b), partial_cmp(&a, &b));
        assert_eq!(partial_cmp(&OurString::<Rc<str>, 4>::from(a.as_str()), &OurString::<Rc<str>, 4>::from(b.as_str())), partial_cmp(&a, &b));
        assert_eq!(partial_cmp(&OurString::<Rc<str>, 4>::from(a.as_str()), &OurString::<Rc<str>, 8>::from(b.as_str())), partial_cmp(&a, &b));
        assert_eq!(partial_cmp(&OurString::<Rc<str>, 8>::from(a.as_str()), &OurString::<Rc<str>, 4>::from(b.as_str())), partial_cmp(&a, &b));

        assert_eq!(cmp(&OurString::<Rc<str>, 4>::from(a.as_str()), &OurString::<Rc<str>, 4>::from(b.as_str())), cmp(&a, &b));
    }
}

#[test]
fn test_index() {
    let a = OurString::<Rc<String>, 8>::from("test message thing");
    assert_eq!(a[..], a.as_str()[..]);
    assert_eq!(a[3..], a.as_str()[3..]);
    assert_eq!(a[..7], a.as_str()[..7]);
    assert_eq!(a[3..7], a.as_str()[3..7]);
}

#[test]
fn test_as_bytes() {
    let a = OurString::<Rc<String>, 8>::from("test message thing");
    assert_eq!(a.as_bytes() as &[u8], b"test message thing");
}

#[test]
fn test_convert() {
    let a = OurString::<Rc<str>, 8>::from("hello");
    assert_eq!(is_inline(&a), true);
    let b: OurString<Rc<str>, 5> = a.convert();
    assert_eq!(is_inline(&b), true);
    let c: OurString<Rc<str>, 10> = b.convert();
    assert_eq!(is_inline(&c), true);
    let d: OurString<Rc<str>, 4> = c.convert();
    assert_eq!(is_inline(&d), false);
    let e: OurString<Rc<str>, 10> = d.clone().convert();
    assert_eq!(is_inline(&e), false);
    assert_eq!(d.as_str().as_ptr(), e.as_str().as_ptr());
}
