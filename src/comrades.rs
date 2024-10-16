//! Specialized [`Comrade`](crate::Comrade) types.

use core::hash::{Hash, Hasher};
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::ptr::NonNull;
use core::cell::Cell;
use core::ops::Deref;
use core::fmt;

const ALIGN: usize = align_of::<usize>();

trait Counter {
    fn increment(&self);
    fn decrement(&self) -> usize;
}

impl Counter for Cell<usize> {
    fn increment(&self) {
        self.set(self.get() + 1);
    }
    fn decrement(&self) -> usize {
        let res = self.get() - 1;
        self.set(res);
        res
    }
}

#[cfg(target_has_atomic = "ptr")]
impl Counter for core::sync::atomic::AtomicUsize {
    fn increment(&self) {
        self.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    }
    fn decrement(&self) -> usize {
        self.fetch_sub(1, core::sync::atomic::Ordering::AcqRel) - 1
    }
}

macro_rules! make_comrade {
    ($(#[$m:meta])? $vis:vis struct $name:ident : $counter:ty) => {
        $(#[$m])?
        ///
        /// We do not support weak semantics, as all are equal under socialism.
        $vis struct $name(NonNull<u8>);

        impl From<&[u8]> for $name {
            fn from(value: &[u8]) -> Self {
                debug_assert!(align_of::<$counter>() == ALIGN && size_of::<$counter>() == ALIGN);

                unsafe {
                    let ptr = alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(2 * ALIGN + value.len(), ALIGN).unwrap_unchecked());
                    *(ptr as *mut $counter) = <$counter>::new(1);
                    *(ptr.add(ALIGN) as *mut usize) = value.len();
                    ptr.add(2 * ALIGN).copy_from_nonoverlapping(value.as_ptr(), value.len());
                    Self(NonNull::new_unchecked(ptr))
                }
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                unsafe { (*(self.0.as_ptr() as *const $counter)).increment(); }
                Self(self.0)
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    if (*(self.0.as_ptr() as *const $counter)).decrement() == 0 {
                        alloc::alloc::dealloc(self.0.as_ptr(), alloc::alloc::Layout::from_size_align(2 * ALIGN + *(self.0.as_ptr().add(ALIGN) as *const usize), ALIGN).unwrap_unchecked());
                    }
                }
            }
        }

        impl Deref for $name {
            type Target = [u8];
            fn deref(&self) -> &Self::Target {
                unsafe { core::slice::from_raw_parts(self.0.as_ptr().add(2 * ALIGN), *(self.0.as_ptr().add(ALIGN) as *const usize)) }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::from([].as_slice())
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                self
            }
        }

        impl Borrow<[u8]> for $name {
            fn borrow(&self) -> &[u8] {
                self
            }
        }

        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                (**self).hash(state);
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", &**self)
            }
        }

        impl<T: AsRef<[u8]>> PartialEq<T> for $name {
            fn eq(&self, other: &T) -> bool {
                (**self).eq(other.as_ref())
            }
        }

        impl Eq for $name {}

        impl<T: AsRef<[u8]>> PartialOrd<T> for $name {
            fn partial_cmp(&self, other: &T) -> Option<Ordering> {
                (**self).partial_cmp(other.as_ref())
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> Ordering {
                (**self).cmp(&**other)
            }
        }

        impl crate::Comrade for $name {
            fn from_slice(s: &[u8]) -> Self {
                s.into()
            }
            fn as_slice(&self) -> &[u8] {
                self
            }
        }
    };
}
make_comrade!(#[doc = "Basically `Rc<[u8]>` but only takes up half the stack space."] pub struct RcBytes : Cell<usize>);
#[cfg(target_has_atomic = "ptr")]
make_comrade!(#[doc = "Basically `Arc<[u8]>` but only takes up half the stack space."] pub struct ArcBytes : core::sync::atomic::AtomicUsize);

#[cfg(target_has_atomic = "ptr")]
unsafe impl Send for ArcBytes {}
#[cfg(target_has_atomic = "ptr")]
unsafe impl Sync for ArcBytes {}
