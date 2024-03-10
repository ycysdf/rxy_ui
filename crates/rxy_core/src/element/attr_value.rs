use crate::smallbox::{SmallBox, S1};
use crate::{impl_x_value_wrappers, XValueWrapper};
use crate::{smallbox, MaybeFromReflect, MaybeReflect, MaybeSend, MaybeSync, MaybeTypePath};
use alloc::borrow::Cow;
use alloc::string::String;
use core::fmt::Debug;
use core::ops::Deref;
use alloc::vec::Vec;

pub trait AttrValue: MaybeReflect + MaybeSend + MaybeSync + Debug + 'static {
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1>;
    fn default_value() -> Self
    where
        Self: Sized;

    #[cfg(not(feature = "bevy_reflect"))]
    fn as_any(&self) -> &dyn core::any::Any;

    fn eq(&self, other: &Self) -> bool
    where
        Self: Sized;
}

impl Clone for SmallBox<dyn AttrValue, S1> {
    fn clone(&self) -> Self {
        self.deref().clone_att_value()
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

#[macro_export]
macro_rules! impl_attr_value_and_wrapper {
    ($($ty:ty $(=> $value:expr)?),*) => {
        $(
            impl_attr_value!($ty $(=> $value)?);
            impl_x_value_wrappers!($ty);
        )*
    };
}
#[macro_export]
macro_rules! impl_attr_values {
    ($($ty:ty $(=> $value:expr)?),*) => {
        $(
            impl_attr_value!($ty $(=> $value)?);
        )*
    };
}

#[macro_export]
macro_rules! impl_attr_value {
    ($ty:ty) => {
        impl_attr_value!($ty => <Self as Default>::default());
    };
    ($ty:ty => $value:expr) => {
        impl AttrValue for $ty {
            fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
                smallbox!(self.clone())
            }
            fn default_value() -> Self {
                $value
            }

            fn eq(&self, other: &Self) -> bool {
                *self == *other
            }

            #[cfg(all(not(feature = "bevy_reflect"),not(feature = "bevy")))]
            fn as_any(&self) -> &dyn core::any::Any {
                self
            }
        }
    };
}


impl<T> Into<XValueWrapper<Vec<T>>> for Vec<T>
{
    fn into(self) -> XValueWrapper<Vec<T>> {
        XValueWrapper(self)
    }
}

impl_attr_value_and_wrapper! {
    u8,
    u16,
    u32,
    u64,
    i8,
    i16,
    i32,
    i64,
    f32,
    f64,
    (),
    bool,
    usize,
    isize,
    String,
    Cow<'static, str>
}

impl_x_value_wrappers!(&'static str);

impl Into<XValueWrapper<Cow<'static, str>>> for String {
    fn into(self) -> XValueWrapper<Cow<'static, str>> {
        XValueWrapper(self.into())
    }
}

impl Into<XValueWrapper<Cow<'static, str>>> for &'static str {
    fn into(self) -> XValueWrapper<Cow<'static, str>> {
        XValueWrapper(self.into())
    }
}

#[cfg(feature = "web_dom")]
impl Into<XValueWrapper<Cow<'static, str>>> for i32 {
    fn into(self) -> XValueWrapper<Cow<'static, str>> {
        XValueWrapper(alloc::format!("{}px",self).into())
    }
}

#[cfg(feature = "web_dom")]
impl Into<XValueWrapper<Cow<'static, str>>> for f32 {
    fn into(self) -> XValueWrapper<Cow<'static, str>> {
        XValueWrapper(alloc::format!("{}px",self).into())
    }
}

impl<T> AttrValue for Option<T>
where
    T: AttrValue + Clone + PartialEq + MaybeTypePath + MaybeFromReflect,
{
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(self.clone())
    }

    fn default_value() -> Self
    where
        Self: Sized,
    {
        <Self as Default>::default()
    }

    #[cfg(not(feature = "bevy_reflect"))]
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }

    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl<T> AttrValue for Vec<T>
where
    T: AttrValue + Clone + PartialEq + MaybeTypePath + MaybeFromReflect,
{
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(self.clone())
    }

    fn default_value() -> Self
    where
        Self: Sized,
    {
        <Self as Default>::default()
    }

    #[cfg(not(feature = "bevy_reflect"))]
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }

    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
