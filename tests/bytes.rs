use std::cmp::{PartialEq, Eq, PartialOrd, Ord};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::borrow::Borrow;
use std::mem::size_of;
use std::ops::Deref;
use std::fmt::Debug;
use std::sync::Arc;
use std::rc::Rc;

use our_string::{OurBytes, Comrade};

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn is_inline<T: Comrade, const N: usize>(v: &OurBytes<T, N>) -> bool {
    let l = v.len();
    let s = v.as_slice() as *const [u8] as *const () as usize;
    let v = v as *const OurBytes<T, N> as *const () as usize;
    s >= v && s + l <= v + size_of::<OurBytes<T, N>>()
}

#[test]
fn test_sizes() {
    assert_eq!(size_of::<Option<OurBytes<Rc<Vec<u8>>, { size_of::<String>() - 2 }>>>(), size_of::<String>());
    assert_eq!(size_of::<OurBytes<Rc<Vec<u8>>, { size_of::<String>() - 1 }>>(), size_of::<String>());
    assert_eq!(size_of::<OurBytes<Rc<Vec<u8>>, { size_of::<String>() - 1 - size_of::<usize>() }>>(), size_of::<String>() - size_of::<usize>());

    assert_eq!(size_of::<Option<OurBytes<Arc<Vec<u8>>, { size_of::<String>() - 2 }>>>(), size_of::<String>());
    assert_eq!(size_of::<OurBytes<Arc<Vec<u8>>, { size_of::<String>() - 1 }>>(), size_of::<String>());
    assert_eq!(size_of::<OurBytes<Arc<Vec<u8>>, { size_of::<String>() - 1 - size_of::<usize>() }>>(), size_of::<String>() - size_of::<usize>());

    assert_eq!(size_of::<OurBytes<Rc<[u8]>, { size_of::<String>() - 1 }>>(), size_of::<String>());

    assert_eq!(size_of::<OurBytes<Arc<[u8]>, { size_of::<String>() - 1 }>>(), size_of::<String>());
}

#[test]
fn test_traits() {
    macro_rules! assert_impl {
        ($t:ty : $($tr:tt)*) => {{
            fn checker<T: $($tr)*>() {}
            checker::<$t>();
        }};
    }

    assert_impl!(OurBytes<Rc<Vec<u8>>, 8> : Hash + Clone + Debug + PartialEq + Eq + PartialOrd + Ord + Default + AsRef<[u8]> + Borrow<[u8]> + Deref<Target = [u8]> + for<'a> From<&'a [u8]> + From<Rc<Vec<u8>>>);
    assert_impl!(OurBytes<Rc<[u8]>,    8> : Hash + Clone + Debug + PartialEq + Eq + PartialOrd + Ord + Default + AsRef<[u8]> + Borrow<[u8]> + Deref<Target = [u8]> + for<'a> From<&'a [u8]> + From<Rc<[u8]>>);

    assert_impl!(OurBytes<Arc<Vec<u8>>, 8> : Send + Sync + Hash + Clone + Debug + PartialEq + Eq + PartialOrd + Ord + Default + AsRef<[u8]> + Borrow<[u8]> + Deref<Target = [u8]> + for<'a> From<&'a [u8]> + From<Arc<Vec<u8>>>);
    assert_impl!(OurBytes<Arc<[u8]>,    8> : Send + Sync + Hash + Clone + Debug + PartialEq + Eq + PartialOrd + Ord + Default + AsRef<[u8]> + Borrow<[u8]> + Deref<Target = [u8]> + for<'a> From<&'a [u8]> + From<Arc<[u8]>>);
}

#[test]
fn test_clone() {
    let a = OurBytes::<Rc<Vec<u8>>, 5>::from([5u8, 2, 7, 5, 2, 5, 4, 1, 7, 5].as_slice());
    let b = a.clone();
    assert_eq!(a, b);
    assert_eq!(a.as_slice(), b.as_slice());
    assert_eq!(a.as_slice() as *const [u8], b.as_slice() as *const [u8]);
}
#[cfg(not(miri))]
proptest::proptest! {
    #[test]
    fn proptest_clone(s: Vec<u8>) {
        let a = OurBytes::<Rc<Vec<u8>>, 5>::from(s.as_slice());
        let b = a.clone();
        assert_eq!(a, b);
        assert_eq!(a.as_slice(), b.as_slice());
        if a.len() > 5 {
            assert_eq!(a.as_slice() as *const [u8], b.as_slice() as *const [u8]);
        }
    }
}

#[test]
fn test_new_default() {
    const X: OurBytes<Rc<Vec<u8>>, 10> = OurBytes::new();
    const Y: OurBytes<Arc<[u8]>, 10> = OurBytes::new();

    assert_eq!(X.len(), 0);
    assert_eq!(X.is_empty(), true);
    assert_eq!(X.as_slice().is_empty(), true);

    assert_eq!(Y.len(), 0);
    assert_eq!(Y.is_empty(), true);
    assert_eq!(Y.as_slice().is_empty(), true);

    assert_eq!(OurBytes::<Rc<Vec<u8>>, 10>::default().len(), 0);
    assert_eq!(OurBytes::<Rc<Vec<u8>>, 10>::default().is_empty(), true);
    assert_eq!(OurBytes::<Rc<Vec<u8>>, 10>::default().as_slice().is_empty(), true);

    assert_eq!(OurBytes::<Arc<[u8]>, 10>::default().len(), 0);
    assert_eq!(OurBytes::<Arc<[u8]>, 10>::default().is_empty(), true);
    assert_eq!(OurBytes::<Arc<[u8]>, 10>::default().as_slice().is_empty(), true);
}

#[test]
fn test_from_slice_inlining() {
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(&[] as &[u8])), true);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8] as &[u8])), true);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6] as &[u8])), true);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6, 1] as &[u8])), true);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6, 1, 84] as &[u8])), true);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6, 1, 84, 255] as &[u8])), true);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6, 1, 84, 255, 12] as &[u8])), true);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23] as &[u8])), true);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45] as &[u8])), false);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65] as &[u8])), false);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65, 56, 23, 76, 45, 98, 23, 56] as &[u8])), false);

    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(&[] as &[u8])), true);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8] as &[u8])), true);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6] as &[u8])), true);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6, 1] as &[u8])), true);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6, 1, 84] as &[u8])), true);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6, 1, 84, 255] as &[u8])), false);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6, 1, 84, 255, 12] as &[u8])), false);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23] as &[u8])), false);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45] as &[u8])), false);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65] as &[u8])), false);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65, 56, 23, 76, 45, 98, 23, 56] as &[u8])), false);
}

#[test]
fn test_from_slice() {
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(&[] as &[u8]) as &[u8], &[] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8] as &[u8]) as &[u8], &[4u8] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6] as &[u8]) as &[u8], &[4u8, 6] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6, 1] as &[u8]) as &[u8], &[4u8, 6, 1] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6, 1, 84] as &[u8]) as &[u8], &[4u8, 6, 1, 84] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6, 1, 84, 255] as &[u8]) as &[u8], &[4u8, 6, 1, 84, 255] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6, 1, 84, 255, 12] as &[u8]) as &[u8], &[4u8, 6, 1, 84, 255, 12] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23] as &[u8]) as &[u8], &[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45] as &[u8]) as &[u8], &[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65] as &[u8]) as &[u8], &[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(&[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65, 56, 23, 76, 45, 98, 23, 56] as &[u8]) as &[u8], &[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65, 56, 23, 76, 45, 98, 23, 56] as &[u8]);

    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(&[] as &[u8]) as &[u8], &[] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8] as &[u8]) as &[u8], &[4u8] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6] as &[u8]) as &[u8], &[4u8, 6] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6, 1] as &[u8]) as &[u8], &[4u8, 6, 1] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6, 1, 84] as &[u8]) as &[u8], &[4u8, 6, 1, 84] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6, 1, 84, 255] as &[u8]) as &[u8], &[4u8, 6, 1, 84, 255] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6, 1, 84, 255, 12] as &[u8]) as &[u8], &[4u8, 6, 1, 84, 255, 12] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23] as &[u8]) as &[u8], &[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45] as &[u8]) as &[u8], &[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65] as &[u8]) as &[u8], &[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65, 56, 23, 76, 45, 98, 23, 56] as &[u8]) as &[u8], &[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65, 56, 23, 76, 45, 98, 23, 56] as &[u8]);
}
#[cfg(not(miri))]
proptest::proptest! {
    #[test]
    fn proptest_from_slice(s: Vec<u8>) {
        assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(s.as_slice()) as &[u8], s.as_slice());
        assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 10>::from(s.as_slice()) as &[u8], s.as_slice());
        assert_eq!(&*OurBytes::<Rc<[u8]>, 10>::from(s.as_slice()) as &[u8], s.as_slice());
        assert_eq!(&*OurBytes::<Arc<[u8]>, 10>::from(s.as_slice()) as &[u8], s.as_slice());

        assert_eq!(OurBytes::<Rc<Vec<u8>>, 10>::from(s.as_slice()).as_slice() as &[u8], s.as_slice());
        assert_eq!(OurBytes::<Arc<Vec<u8>>, 10>::from(s.as_slice()).as_slice() as &[u8], s.as_slice());
        assert_eq!(OurBytes::<Rc<[u8]>, 10>::from(s.as_slice()).as_slice() as &[u8], s.as_slice());
        assert_eq!(OurBytes::<Arc<[u8]>, 10>::from(s.as_slice()).as_slice() as &[u8], s.as_slice());

        assert_eq!(OurBytes::<Rc<Vec<u8>>, 10>::from(s.as_slice()).as_ref() as &[u8], s.as_slice());
        assert_eq!(OurBytes::<Arc<Vec<u8>>, 10>::from(s.as_slice()).as_ref() as &[u8], s.as_slice());
        assert_eq!(OurBytes::<Rc<[u8]>, 10>::from(s.as_slice()).as_ref() as &[u8], s.as_slice());
        assert_eq!(OurBytes::<Arc<[u8]>, 10>::from(s.as_slice()).as_ref() as &[u8], s.as_slice());

        assert_eq!(<_ as AsRef<[u8]>>::as_ref(&OurBytes::<Rc<Vec<u8>>, 10>::from(s.as_slice())) as &[u8], s.as_slice());
        assert_eq!(<_ as AsRef<[u8]>>::as_ref(&OurBytes::<Arc<Vec<u8>>, 10>::from(s.as_slice())) as &[u8], s.as_slice());
        assert_eq!(<_ as AsRef<[u8]>>::as_ref(&OurBytes::<Rc<[u8]>, 10>::from(s.as_slice())) as &[u8], s.as_slice());
        assert_eq!(<_ as AsRef<[u8]>>::as_ref(&OurBytes::<Arc<[u8]>, 10>::from(s.as_slice())) as &[u8], s.as_slice());

        assert_eq!(OurBytes::<Rc<Vec<u8>>, 10>::from(s.as_slice()).borrow() as &[u8], s.as_slice());
        assert_eq!(OurBytes::<Arc<Vec<u8>>, 10>::from(s.as_slice()).borrow() as &[u8], s.as_slice());
        assert_eq!(OurBytes::<Rc<[u8]>, 10>::from(s.as_slice()).borrow() as &[u8], s.as_slice());
        assert_eq!(OurBytes::<Arc<[u8]>, 10>::from(s.as_slice()).borrow() as &[u8], s.as_slice());

        assert_eq!(<_ as Borrow<[u8]>>::borrow(&OurBytes::<Rc<Vec<u8>>, 10>::from(s.as_slice())) as &[u8], s.as_slice());
        assert_eq!(<_ as Borrow<[u8]>>::borrow(&OurBytes::<Arc<Vec<u8>>, 10>::from(s.as_slice())) as &[u8], s.as_slice());
        assert_eq!(<_ as Borrow<[u8]>>::borrow(&OurBytes::<Rc<[u8]>, 10>::from(s.as_slice())) as &[u8], s.as_slice());
        assert_eq!(<_ as Borrow<[u8]>>::borrow(&OurBytes::<Arc<[u8]>, 10>::from(s.as_slice())) as &[u8], s.as_slice());
    }
}

#[test]
fn test_hash() {
    assert_eq!(hash(&OurBytes::<Arc<Vec<u8>>, 4>::from(&[] as &[u8])), hash(&[].as_slice() as &&[u8]));
    assert_eq!(hash(&OurBytes::<Arc<Vec<u8>>, 4>::from(&[7u8] as &[u8])), hash(&[7u8].as_slice()));
    assert_eq!(hash(&OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6] as &[u8])), hash(&[4u8, 6].as_slice()));
    assert_eq!(hash(&OurBytes::<Arc<Vec<u8>>, 4>::from(&[4u8, 6, 7, 4, 5, 2, 4, 3, 2, 255, 160, 2] as &[u8])), hash(&[4u8, 6, 7, 4, 5, 2, 4, 3, 2, 255, 160, 2].as_slice()));
}
#[cfg(not(miri))]
proptest::proptest! {
    #[test]
    fn proptest_hash(s: Vec<u8>) {
        assert_eq!(hash(&OurBytes::<Rc<Vec<u8>>, 4>::from(s.as_slice())), hash(&s.as_slice()));
        assert_eq!(hash(&OurBytes::<Rc<[u8]>, 4>::from(s.as_slice())), hash(&s.as_slice()));
        assert_eq!(hash(&OurBytes::<Arc<Vec<u8>>, 4>::from(s.as_slice())), hash(&s.as_slice()));
        assert_eq!(hash(&OurBytes::<Arc<[u8]>, 4>::from(s.as_slice())), hash(&s.as_slice()));
    }
}

#[test]
fn test_from_comrade_inlining() {
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6, 1] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6, 1, 84] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6, 1, 84, 255] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6, 1, 84, 255, 12] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65, 56, 23, 76, 45, 98, 23, 56] as Vec<u8>))), false);

    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6, 1] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6, 1, 84] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6, 1, 84, 255] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6, 1, 84, 255, 12] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65] as Vec<u8>))), false);
    assert_eq!(is_inline(&OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65, 56, 23, 76, 45, 98, 23, 56] as Vec<u8>))), false);
}

#[test]
fn test_from_comrade() {
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![] as Vec<u8>)) as &[u8], &[] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8] as Vec<u8>)) as &[u8], &[4u8] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6] as Vec<u8>)) as &[u8], &[4u8, 6] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6, 1] as Vec<u8>)) as &[u8], &[4u8, 6, 1] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6, 1, 84] as Vec<u8>)) as &[u8], &[4u8, 6, 1, 84] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6, 1, 84, 255] as Vec<u8>)) as &[u8], &[4u8, 6, 1, 84, 255] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6, 1, 84, 255, 12] as Vec<u8>)) as &[u8], &[4u8, 6, 1, 84, 255, 12] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23] as Vec<u8>)) as &[u8], &[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45] as Vec<u8>)) as &[u8], &[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65] as Vec<u8>)) as &[u8], &[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65] as &[u8]);
    assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(vec![4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65, 56, 23, 76, 45, 98, 23, 56] as Vec<u8>)) as &[u8], &[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65, 56, 23, 76, 45, 98, 23, 56] as &[u8]);

    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![] as Vec<u8>)) as &[u8], &[] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8] as Vec<u8>)) as &[u8], &[4u8] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6] as Vec<u8>)) as &[u8], &[4u8, 6] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6, 1] as Vec<u8>)) as &[u8], &[4u8, 6, 1] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6, 1, 84] as Vec<u8>)) as &[u8], &[4u8, 6, 1, 84] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6, 1, 84, 255] as Vec<u8>)) as &[u8], &[4u8, 6, 1, 84, 255] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6, 1, 84, 255, 12] as Vec<u8>)) as &[u8], &[4u8, 6, 1, 84, 255, 12] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23] as Vec<u8>)) as &[u8], &[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45] as Vec<u8>)) as &[u8], &[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65] as Vec<u8>)) as &[u8], &[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65] as &[u8]);
    assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 4>::from(Arc::new(vec![4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65, 56, 23, 76, 45, 98, 23, 56] as Vec<u8>)) as &[u8], &[4u8, 6, 1, 84, 255, 12, 23, 98, 169, 23, 45, 65, 56, 23, 76, 45, 98, 23, 56] as &[u8]);
}
#[cfg(not(miri))]
proptest::proptest! {
    #[test]
    fn proptest_from_comrade(s: Vec<u8>) {
        assert_eq!(&*OurBytes::<Rc<Vec<u8>>, 10>::from(Rc::new(s.clone())) as &[u8], s.as_slice());
        assert_eq!(&*OurBytes::<Arc<Vec<u8>>, 10>::from(Arc::new(s.clone())) as &[u8], s.as_slice());
        assert_eq!(&*OurBytes::<Rc<[u8]>, 10>::from(Rc::from(s.as_slice())) as &[u8], s.as_slice());
        assert_eq!(&*OurBytes::<Arc<[u8]>, 10>::from(Arc::from(s.as_slice())) as &[u8], s.as_slice());
    }
}

#[test]
fn test_debug() {
    assert_eq!(format!("{:?}", OurBytes::<Rc<[u8]>, 4>::from(&[1u8, 2, 7] as &[u8])), format!("{:?}", &[1u8, 2, 7]));
    assert_eq!(format!("{:?}", OurBytes::<Rc<Vec<u8>>, 4>::from(&[1u8, 2, 3, 4] as &[u8])), format!("{:?}", &[1u8, 2, 3, 4]));
    assert_eq!(format!("{:?}", OurBytes::<Arc<Vec<u8>>, 4>::from(&[1u8, 2, 3, 4, 9, 1, 3, 255] as &[u8])), format!("{:?}", &[1u8, 2, 3, 4, 9, 1, 3, 255]));
}
#[cfg(not(miri))]
proptest::proptest! {
    #[test]
    fn proptest_debug(s: Vec<u8>) {
        assert_eq!(format!("{:?}", OurBytes::<Rc<[u8]>, 4>::from(Rc::from(s.as_slice()))), format!("{s:?}"));
        assert_eq!(format!("{:?}", OurBytes::<Arc<[u8]>, 4>::from(Arc::from(s.as_slice()))), format!("{s:?}"));
    }
}

#[test]
fn test_limits() {
    const STORY: &[u8] = b"the quick brown fox jumped over the lazy dog and a lot of other things that will make this sufficiently long enough for all of the tests to have enough space to slice and all that and stuff. except that this is not nearly as long as it needs to be and it really needs to be around double the size that i had origionally planned for in order to accomodate all of the larger slicing tests that will happen to this and all of that and stuff. ok plz be long enough now. nope not quite, but it's getting pretty close at this point and now this should really be it for real this time!";
    fn check<const N: usize>(inline: bool) {
        let story = &STORY[..N];
        let s = OurBytes::<Rc<[u8]>, N>::from(story);
        assert_eq!(s.len(), N);
        assert_eq!(s, story);
        assert_eq!(is_inline(&s), inline);
    }
    macro_rules! checks {
        ($v:literal => $($l:literal),*$(,)?) => { $(check::<$l>($v));* };
    }
    checks! {
        true =>
          0,   1,   2,   3,   4,   5,   6,   7,   8,   9,  10,  11,  12,  13,  14,  15,
         16,  17,  18,  19,  20,  21,  22,  23,  24,  25,  26,  27,  28,  29,  30,  31,
         32,  33,  34,  35,  36,  37,  38,  39,  40,  41,  42,  43,  44,  45,  46,  47,
         48,  49,  50,  51,  52,  53,  54,  55,  56,  57,  58,  59,  60,  61,  62,  63,
         64,  65,  66,  67,  68,  69,  70,  71,  72,  73,  74,  75,  76,  77,  78,  79,
         80,  81,  82,  83,  84,  85,  86,  87,  88,  89,  90,  91,  92,  93,  94,  95,
         96,  97,  98,  99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
        112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
        128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
        144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
        160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175,
        176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191,
        192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207,
        208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223,
        224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239,
        240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254,
    }
    checks! {
        false => 255,
        256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 270, 271,
        272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287,
        288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303,
        304, 305, 306, 307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319,
        320, 321, 322, 323, 324, 325, 326, 327, 328, 329, 330, 331, 332, 333, 334, 335,
        336, 337, 338, 339, 340, 341, 342, 343, 344, 345, 346, 347, 348, 349, 350, 351,
        352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363, 364, 365, 366, 367,
        368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382, 383,
        384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399,
        400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415,
        416, 417, 418, 419, 420, 421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431,
        432, 433, 434, 435, 436, 437, 438, 439, 440, 441, 442, 443, 444, 445, 446, 447,
        448, 449, 450, 451, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461, 462, 463,
        464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477, 478, 479,
        480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495,
        496, 497, 498, 499, 500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511,
    }
}

#[test]
fn test_comparison() {
    let a = [6u8, 2, 3, 9, 3].as_slice();
    let a1 = OurBytes::<Rc<Vec<u8>>, 10>::from(a);
    let a2 = OurBytes::<Arc<Vec<u8>>, 3>::from(a);
    let a3 = Vec::from(a);

    let b = [7u8, 2, 7, 5].as_slice();
    let b1 = OurBytes::<Rc<[u8]>, 7>::from(b);
    let b2 = OurBytes::<Rc<[u8]>, 2>::from(b);
    let b3 = Vec::from(b);

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
#[cfg(not(miri))]
proptest::proptest! {
    #[test]
    fn proptest_comparison(a: Vec<u8>, b: Vec<u8>) {
        fn partial_cmp<A, B>(a: &A, b: &B) -> Option<core::cmp::Ordering> where A: PartialOrd<B> {
            a.partial_cmp(b)
        }
        fn cmp<A>(a: &A, b: &A) -> core::cmp::Ordering where A: Ord {
            a.cmp(b)
        }

        assert_eq!(partial_cmp(&a.as_slice(), &OurBytes::<Rc<[u8]>, 4>::from(b.as_slice())), partial_cmp(&a, &b));
        assert_eq!(partial_cmp(&OurBytes::<Rc<[u8]>, 4>::from(a.as_slice()), &b), partial_cmp(&a, &b));
        assert_eq!(partial_cmp(&OurBytes::<Rc<[u8]>, 4>::from(a.as_slice()), &OurBytes::<Rc<[u8]>, 4>::from(b.as_slice())), partial_cmp(&a, &b));
        assert_eq!(partial_cmp(&OurBytes::<Rc<[u8]>, 4>::from(a.as_slice()), &OurBytes::<Rc<[u8]>, 8>::from(b.as_slice())), partial_cmp(&a, &b));
        assert_eq!(partial_cmp(&OurBytes::<Rc<[u8]>, 8>::from(a.as_slice()), &OurBytes::<Rc<[u8]>, 4>::from(b.as_slice())), partial_cmp(&a, &b));

        assert_eq!(cmp(&OurBytes::<Rc<[u8]>, 4>::from(a.as_slice()), &OurBytes::<Rc<[u8]>, 4>::from(b.as_slice())), cmp(&a, &b));
    }
}

#[test]
fn test_index() {
    let a = OurBytes::<Rc<Vec<u8>>, 8>::from([5u8, 3, 7, 12, 174, 255, 34].as_slice());
    assert_eq!(a[..], a.as_slice()[..]);
    assert_eq!(a[3..], a.as_slice()[3..]);
    assert_eq!(a[..7], a.as_slice()[..7]);
    assert_eq!(a[3..7], a.as_slice()[3..7]);
    assert_eq!(a[5], a.as_slice()[5]);
    assert_eq!(a[3], a.as_slice()[3]);
    assert_eq!(a[0], a.as_slice()[0]);
}

#[test]
fn test_convert() {
    let a = OurBytes::<Rc<[u8]>, 8>::from([5u8, 1, 6, 3, 6].as_slice());
    assert_eq!(is_inline(&a), true);
    let b: OurBytes<Rc<[u8]>, 5> = a.convert();
    assert_eq!(is_inline(&b), true);
    let c: OurBytes<Rc<[u8]>, 10> = b.convert();
    assert_eq!(is_inline(&c), true);
    let d: OurBytes<Rc<[u8]>, 4> = c.convert();
    assert_eq!(is_inline(&d), false);
    let e: OurBytes<Rc<[u8]>, 10> = d.clone().convert();
    assert_eq!(is_inline(&e), false);
    assert_eq!(d.as_slice().as_ptr(), e.as_slice().as_ptr());
}
