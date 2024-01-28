use std::alloc::{self, Layout};
use std::any::Any;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{self, Hash};
use std::marker::PhantomData;
#[cfg(feature = "coerce")]
use std::marker::Unsize;
use std::mem::{self, MaybeUninit};
use std::ops;
#[cfg(feature = "coerce")]
use std::ops::CoerceUnsized;
use std::ops::Deref;
use std::ptr;

use crate::element_core::AttrValue;

/*#[cfg(not(any(feature = "std", doctest)))]
use ::core::alloc::{self, Layout};
#[cfg(any(feature = "std", doctest))]*/
#[cfg(feature = "coerce")]
impl<T: ?Sized + Unsize<U>, U: ?Sized, Space> CoerceUnsized<SmallBox<U, Space>>
    for SmallBox<T, Space>
{
}

#[macro_export]
macro_rules! smallbox {
    ( $e: expr ) => {{
        let val = $e;
        let ptr = &val as *const _;
        #[allow(unsafe_code)]
        unsafe {
            $crate::SmallBox::new_unchecked(val, ptr)
        }
    }};
}

/// An optimized box that store value on stack or on heap depending on its size
pub struct SmallBox<T: ?Sized, Space> {
    space: MaybeUninit<Space>,
    ptr: *const T,
    _phantom: PhantomData<T>,
}

impl<T: ?Sized, Space> SmallBox<T, Space> {
    #[inline(always)]
    pub fn new(val: T) -> SmallBox<T, Space>
    where
        T: Sized,
    {
        smallbox!(val)
    }

    #[doc(hidden)]
    #[inline]
    pub unsafe fn new_unchecked<U>(val: U, ptr: *const T) -> SmallBox<T, Space>
    where
        U: Sized,
    {
        let result = Self::new_copy(&val, ptr);
        mem::forget(val);
        result
    }

    pub fn resize<ToSpace>(self) -> SmallBox<T, ToSpace> {
        unsafe {
            let result = if self.is_heap() {
                // don't change anything if data is already on heap
                let space = MaybeUninit::<ToSpace>::uninit();
                SmallBox {
                    space,
                    ptr: self.ptr,
                    _phantom: PhantomData,
                }
            } else {
                let val: &T = &*self;
                SmallBox::<T, ToSpace>::new_copy(val, val as *const T)
            };

            mem::forget(self);

            result
        }
    }

    #[inline]
    pub fn is_heap(&self) -> bool {
        !self.ptr.is_null()
    }

    unsafe fn new_copy<U>(val: &U, ptr: *const T) -> SmallBox<T, Space>
    where
        U: ?Sized,
    {
        let size = mem::size_of_val::<U>(val);
        let align = mem::align_of_val::<U>(val);

        let mut space = MaybeUninit::<Space>::uninit();

        let (ptr_addr, ptr_copy): (*const u8, *mut u8) = if size == 0 {
            (ptr::null(), align as *mut u8)
        } else if size > mem::size_of::<Space>() || align > mem::align_of::<Space>() {
            // Heap
            let layout = Layout::for_value::<U>(val);
            let heap_ptr = alloc::alloc(layout);

            (heap_ptr, heap_ptr)
        } else {
            // Stack
            (ptr::null(), space.as_mut_ptr() as *mut u8)
        };

        // Overwrite the pointer but retain any extra data inside the fat pointer.
        let mut ptr = ptr;
        let ptr_ptr = &mut ptr as *mut _ as *mut usize;
        ptr_ptr.write(ptr_addr as usize);

        ptr::copy_nonoverlapping(val as *const _ as *const u8, ptr_copy, size);

        SmallBox {
            space,
            ptr,
            _phantom: PhantomData,
        }
    }

    unsafe fn downcast_unchecked<U: Any>(self) -> SmallBox<U, Space> {
        let size = mem::size_of::<U>();
        let mut space = MaybeUninit::<Space>::uninit();

        if !self.is_heap() {
            ptr::copy_nonoverlapping(
                self.space.as_ptr() as *const u8,
                space.as_mut_ptr() as *mut u8,
                size,
            );
        };

        let ptr = self.ptr as *const U;

        mem::forget(self);

        SmallBox {
            space,
            ptr,
            _phantom: PhantomData,
        }
    }

    #[inline]
    unsafe fn as_ptr(&self) -> *const T {
        let mut ptr = self.ptr;

        if !self.is_heap() {
            // Overwrite the pointer but retain any extra data inside the fat pointer.
            let ptr_ptr = &mut ptr as *mut _ as *mut usize;
            ptr_ptr.write(self.space.as_ptr() as *const () as usize);
        }

        ptr
    }

    #[inline]
    unsafe fn as_mut_ptr(&mut self) -> *mut T {
        let mut ptr = self.ptr;

        if !self.is_heap() {
            // Overwrite the pointer but retain any extra data inside the fat pointer.
            let ptr_ptr = &mut ptr as *mut _ as *mut usize;
            ptr_ptr.write(self.space.as_mut_ptr() as *mut () as usize);
        }

        ptr as *mut _
    }
    #[inline]
    pub fn into_inner(self) -> T
    where
        T: Sized,
    {
        let ret_val: T = unsafe { self.as_ptr().read() };

        // Just drops the heap without dropping the boxed value
        if self.is_heap() {
            let layout = Layout::new::<T>();
            unsafe {
                alloc::dealloc(self.ptr as *mut u8, layout);
            }
        }
        mem::forget(self);

        ret_val
    }
}

impl<Space> SmallBox<dyn Any, Space> {
    #[inline]
    pub fn downcast<T: Any>(self) -> Result<SmallBox<T, Space>, Self> {
        if self.is::<T>() {
            unsafe { Ok(self.downcast_unchecked()) }
        } else {
            Err(self)
        }
    }
}

impl<Space> SmallBox<dyn AttrValue, Space> {
    #[inline]
    pub fn downcast<T: Any>(self) -> Result<SmallBox<T, Space>, Self> {
        if self.deref().as_any().is::<T>() {
            unsafe { Ok(self.downcast_unchecked()) }
        } else {
            Err(self)
        }
    }
}

impl<Space> SmallBox<dyn Any + Send, Space> {
    #[inline]
    pub fn downcast<T: Any>(self) -> Result<SmallBox<T, Space>, Self> {
        if self.is::<T>() {
            unsafe { Ok(self.downcast_unchecked()) }
        } else {
            Err(self)
        }
    }
}

impl<T: ?Sized, Space> ops::Deref for SmallBox<T, Space> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.as_ptr() }
    }
}

impl<T: ?Sized, Space> ops::DerefMut for SmallBox<T, Space> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.as_mut_ptr() }
    }
}

impl<T: ?Sized, Space> ops::Drop for SmallBox<T, Space> {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::for_value::<T>(&*self);
            ptr::drop_in_place::<T>(&mut **self);
            if self.is_heap() {
                alloc::dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

impl<T: Clone, Space> Clone for SmallBox<T, Space>
where
    T: Sized,
{
    fn clone(&self) -> Self {
        let val: &T = &*self;
        SmallBox::new(val.clone())
    }
}

impl<T: ?Sized + fmt::Display, Space> fmt::Display for SmallBox<T, Space> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T: ?Sized + fmt::Debug, Space> fmt::Debug for SmallBox<T, Space> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized, Space> fmt::Pointer for SmallBox<T, Space> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // It's not possible to extract the inner Unique directly from the Box,
        // instead we cast it to a *const which aliases the Unique
        let ptr: *const T = &**self;
        fmt::Pointer::fmt(&ptr, f)
    }
}

impl<T: ?Sized + PartialEq, Space> PartialEq for SmallBox<T, Space> {
    fn eq(&self, other: &SmallBox<T, Space>) -> bool {
        PartialEq::eq(&**self, &**other)
    }
    fn ne(&self, other: &SmallBox<T, Space>) -> bool {
        PartialEq::ne(&**self, &**other)
    }
}

impl<T: ?Sized + PartialOrd, Space> PartialOrd for SmallBox<T, Space> {
    fn partial_cmp(&self, other: &SmallBox<T, Space>) -> Option<Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
    fn lt(&self, other: &SmallBox<T, Space>) -> bool {
        PartialOrd::lt(&**self, &**other)
    }
    fn le(&self, other: &SmallBox<T, Space>) -> bool {
        PartialOrd::le(&**self, &**other)
    }
    fn ge(&self, other: &SmallBox<T, Space>) -> bool {
        PartialOrd::ge(&**self, &**other)
    }
    fn gt(&self, other: &SmallBox<T, Space>) -> bool {
        PartialOrd::gt(&**self, &**other)
    }
}

impl<T: ?Sized + Ord, Space> Ord for SmallBox<T, Space> {
    fn cmp(&self, other: &SmallBox<T, Space>) -> Ordering {
        Ord::cmp(&**self, &**other)
    }
}

impl<T: ?Sized + Eq, Space> Eq for SmallBox<T, Space> {}

impl<T: ?Sized + Hash, Space> Hash for SmallBox<T, Space> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

unsafe impl<T: ?Sized + Send, Space> Send for SmallBox<T, Space> {}

unsafe impl<T: ?Sized + Sync, Space> Sync for SmallBox<T, Space> {}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use crate::smallbox::space::*;

    use super::SmallBox;

    #[test]
    fn test_basic() {
        let stacked: SmallBox<usize, S1> = SmallBox::new(1234usize);
        assert!(*stacked == 1234);

        let heaped: SmallBox<(usize, usize), S1> = SmallBox::new((0, 1));
        assert!(*heaped == (0, 1));
    }

    #[test]
    fn test_new_unchecked() {
        let val = [0usize, 1];
        let ptr = &val as *const _;

        unsafe {
            let stacked: SmallBox<[usize], S2> = SmallBox::new_unchecked(val, ptr);
            assert!(*stacked == [0, 1]);
            assert!(!stacked.is_heap());
        }

        let val = [0usize, 1, 2];
        let ptr = &val as *const _;

        unsafe {
            let heaped: SmallBox<dyn Any, S2> = SmallBox::new_unchecked(val, ptr);
            assert!(heaped.is_heap());

            if let Some(array) = heaped.downcast_ref::<[usize; 3]>() {
                assert_eq!(*array, [0, 1, 2]);
            } else {
                unreachable!();
            }
        }
    }

    #[test]
    #[deny(unsafe_code)]
    fn test_macro() {
        let stacked: SmallBox<dyn Any, S1> = smallbox!(1234usize);
        if let Some(num) = stacked.downcast_ref::<usize>() {
            assert_eq!(*num, 1234);
        } else {
            unreachable!();
        }

        let heaped: SmallBox<dyn Any, S1> = smallbox!([0usize, 1]);
        if let Some(array) = heaped.downcast_ref::<[usize; 2]>() {
            assert_eq!(*array, [0, 1]);
        } else {
            unreachable!();
        }

        let is_even: SmallBox<dyn Fn(u8) -> bool, S1> = smallbox!(|num: u8| num % 2 == 0);
        assert!(!is_even(5));
        assert!(is_even(6));
    }

    #[test]
    #[cfg(feature = "coerce")]
    fn test_coerce() {
        let stacked: SmallBox<dyn Any, S1> = SmallBox::new(1234usize);
        if let Some(num) = stacked.downcast_ref::<usize>() {
            assert_eq!(*num, 1234);
        } else {
            unreachable!();
        }

        let heaped: SmallBox<dyn Any, S1> = SmallBox::new([0usize, 1]);
        if let Some(array) = heaped.downcast_ref::<[usize; 2]>() {
            assert_eq!(*array, [0, 1]);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_drop() {
        use std::cell::Cell;

        struct Struct<'a>(&'a Cell<bool>, u8);
        impl<'a> Drop for Struct<'a> {
            fn drop(&mut self) {
                self.0.set(true);
            }
        }

        let flag = Cell::new(false);
        let stacked: SmallBox<_, S2> = SmallBox::new(Struct(&flag, 0));
        assert!(!stacked.is_heap());
        assert!(flag.get() == false);
        drop(stacked);
        assert!(flag.get() == true);

        let flag = Cell::new(false);
        let heaped: SmallBox<_, S1> = SmallBox::new(Struct(&flag, 0));
        assert!(heaped.is_heap());
        assert!(flag.get() == false);
        drop(heaped);
        assert!(flag.get() == true);
    }

    #[test]
    fn test_dont_drop_space() {
        struct NoDrop(S1);
        impl Drop for NoDrop {
            fn drop(&mut self) {
                unreachable!();
            }
        }

        drop(SmallBox::<_, NoDrop>::new([true]));
    }

    #[test]
    fn test_oversize() {
        let fit = SmallBox::<_, S1>::new([1usize]);
        let oversize = SmallBox::<_, S1>::new([1usize, 2]);
        assert!(!fit.is_heap());
        assert!(oversize.is_heap());
    }

    #[test]
    fn test_resize() {
        let m = SmallBox::<_, S4>::new([1usize, 2]);
        let l = m.resize::<S8>();
        assert!(!l.is_heap());
        let m = l.resize::<S4>();
        assert!(!m.is_heap());
        let s = m.resize::<S2>();
        assert!(!s.is_heap());
        let xs = s.resize::<S1>();
        assert!(xs.is_heap());
        let m = xs.resize::<S4>();
        assert!(m.is_heap());
        assert_eq!(*m, [1usize, 2]);
    }

    #[test]
    fn test_clone() {
        let stacked: SmallBox<[usize; 2], S2> = smallbox!([1usize, 2]);
        assert_eq!(stacked, stacked.clone())
    }

    #[test]
    fn test_zst() {
        struct ZSpace;

        let zst: SmallBox<[usize], S1> = smallbox!([1usize; 0]);
        assert_eq!(*zst, [1usize; 0]);

        let zst: SmallBox<[usize], ZSpace> = smallbox!([1usize; 0]);
        assert_eq!(*zst, [1usize; 0]);
        let zst: SmallBox<[usize], ZSpace> = smallbox!([1usize; 2]);
        assert_eq!(*zst, [1usize; 2]);
    }

    #[test]
    fn test_downcast() {
        let stacked: SmallBox<dyn Any, S1> = smallbox!(0x01u32);
        assert!(!stacked.is_heap());
        assert_eq!(SmallBox::new(0x01), stacked.downcast::<u32>().unwrap());

        let heaped: SmallBox<dyn Any, S1> = smallbox!([1usize, 2]);
        assert!(heaped.is_heap());
        assert_eq!(
            smallbox!([1usize, 2]),
            heaped.downcast::<[usize; 2]>().unwrap()
        );

        let stacked_send: SmallBox<dyn Any + Send, S1> = smallbox!(0x01u32);
        assert!(!stacked_send.is_heap());
        assert_eq!(SmallBox::new(0x01), stacked_send.downcast::<u32>().unwrap());

        let heaped_send: SmallBox<dyn Any + Send, S1> = smallbox!([1usize, 2]);
        assert!(heaped_send.is_heap());
        assert_eq!(
            SmallBox::new([1usize, 2]),
            heaped_send.downcast::<[usize; 2]>().unwrap()
        );

        let mismatched: SmallBox<dyn Any, S1> = smallbox!(0x01u32);
        assert!(mismatched.downcast::<u8>().is_err());
        let mismatched: SmallBox<dyn Any, S1> = smallbox!(0x01u32);
        assert!(mismatched.downcast::<u64>().is_err());
    }

    #[test]
    fn test_option_encoding() {
        let tester: SmallBox<Box<()>, S2> = SmallBox::new(Box::new(()));
        assert!(Some(tester).is_some());
    }

    #[test]
    fn test_into_inner() {
        let tester: SmallBox<_, S1> = SmallBox::new([21usize]);
        let val = tester.into_inner();
        assert_eq!(val[0], 21);

        let tester: SmallBox<_, S1> = SmallBox::new(vec![21, 56, 420]);
        let val = tester.into_inner();
        assert_eq!(val[1], 56);
    }
}
