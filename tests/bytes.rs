use std::cmp::{Ordering, PartialEq, Eq, PartialOrd, Ord};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::borrow::Borrow;
use std::mem::size_of;
use std::ops::Deref;
use std::fmt::Debug;
use std::sync::Arc;
use std::rc::Rc;

use our_string::{OurBytes, BytesComrade};

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn is_inline<T: BytesComrade, const N: usize>(v: &OurBytes<T, N>) -> bool {
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
proptest::proptest! {
    #[test]
    fn proptest_debug(s: Vec<u8>) {
        assert_eq!(format!("{:?}", OurBytes::<Rc<[u8]>, 4>::from(Rc::from(s.as_slice()))), format!("{s:?}"));
        assert_eq!(format!("{:?}", OurBytes::<Arc<[u8]>, 4>::from(Arc::from(s.as_slice()))), format!("{s:?}"));
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
proptest::proptest! {
    #[test]
    fn proptest_comparison(a: Vec<u8>, b: Vec<u8>) {
        fn partial_cmp<A, B>(a: &A, b: &B) -> Option<Ordering> where A: PartialOrd<B> {
            a.partial_cmp(b)
        }
        fn cmp<A>(a: &A, b: &A) -> Ordering where A: Ord {
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
