use alloc::borrow::Cow;
use core::fmt::Debug;
use core::ops::Deref;

use rxy_core::{MaybeFromReflect, MaybeReflect, MaybeSend, MaybeSync, MaybeTypePath};

use crate::smallbox;
use crate::smallbox::S1;
use crate::SmallBox;

pub trait AttrValue: MaybeReflect + MaybeSend + MaybeSync + Debug + 'static
// where Option<Self>: From<DomAttributeValue>
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

#[macro_export]
macro_rules! impl_default_attr_values {
    ($($type:ty $(:$value:stmt)?),*) => {
        $(
            impl_default_attr_value!($type $(,$value)?);
        )*
    };
}

#[macro_export]
macro_rules! impl_default_attr_value {
    ($type:ty) => {
        impl_default_attr_value!($type, <Self as Default>::default());
    };
    ($type:ty,$value:stmt) => {
        impl AttrValue for $type {
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
    alloc::string::String
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

#[cfg(feature = "bevy_asset")]
impl<T> AttrValue for bevy_asset::Handle<T>
where
    T: bevy_asset::Asset,
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

    fn eq(&self, other: &Self) -> bool {
        self == other
    }

    #[cfg(not(feature = "bevy_reflect"))]
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

impl AttrValue for Cow<'static, str> {
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(self.clone())
    }

    fn default_value() -> Self
    where
        Self: Sized,
    {
        <Self as Default>::default()
    }

    fn eq(&self, other: &Self) -> bool {
        self == other
    }

    #[cfg(not(feature = "bevy_reflect"))]
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}
