use alloc::string::String;
use alloc::borrow::Cow;
use core::fmt::Debug;
use core::ops::Deref;
use crate::{MaybeFromReflect, MaybeReflect, MaybeSend, MaybeSync, MaybeTypePath, smallbox};
use crate::smallbox::{S1, SmallBox};
use crate::element::AttrValueWrapper;
use crate::element::ElementAttr;
use crate::renderer::Renderer;

pub trait AttrValue: MaybeReflect + MaybeSend + MaybeSync + Debug + 'static
{
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


// todo: generic params
macro_rules! impl_into_attr_value_wrapper {
    ($ty:ty) => {
        impl<R, EA> Into<AttrValueWrapper<R, EA>> for $ty
        where
            EA: ElementAttr<R>,
            EA::Value: From<$ty>,
            R: Renderer,
        {
            fn into(self) -> AttrValueWrapper<R, EA> {
                AttrValueWrapper(self.into())
            }
        }
    };
}
#[macro_export]
macro_rules! impl_default_attr_values {
    ($($ty:ty $(:$value:stmt)?),*) => {
        $(
            impl_default_attr_value!($ty $(,$value)?);
        )*
    };
}

#[macro_export]
macro_rules! impl_default_attr_value {
    ($ty:ty) => {
        impl_default_attr_value!($ty, <Self as Default>::default());
    };
    ($ty:ty,$value:stmt) => {
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

            #[cfg(not(feature = "bevy_reflect"))]
            fn as_any(&self) -> &dyn core::any::Any {
                self
            }
        }
        impl_into_attr_value_wrapper!($ty);
    };
}

impl_default_attr_values! {
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
    // &'static str,
    Cow<'static, str>
}

#[cfg(feature = "bevy_ui")]
impl_default_attr_values! {
    bevy_ui::prelude::Val
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

// impl AttrValue for Cow<'static, str> {
//     fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
//         smallbox!(self.clone())
//     }
//
//     fn default_value() -> Self
//     where
//         Self: Sized,
//     {
//         <Self as Default>::default()
//     }
//
//     fn eq(&self, other: &Self) -> bool {
//         self == other
//     }
//
//     #[cfg(not(feature = "bevy_reflect"))]
//     fn as_any(&self) -> &dyn core::any::Any {
//         self
//     }
// }
