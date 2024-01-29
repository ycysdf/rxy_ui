use std::borrow::Cow;
use std::fmt::Debug;
use std::ops::Deref;

use bevy_reflect::prelude::*;
use bevy_reflect::{FromReflect, TypePath};
use bevy_ui::Val;

use crate::smallbox;
use crate::smallbox::S1;
use crate::SmallBox;

pub trait AttrValue: Reflect + Send + Sync + 'static // where Option<Self>: From<DomAttributeValue>
{
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1>;
    fn default_value() -> Self
    where
        Self: Sized;
    fn merge_value(&mut self, _value: Self)
    where
        Self: Sized,
    {
    }

    fn eq(&self, other: &Self) -> bool
    where
        Self: Sized;
}

impl Debug for dyn AttrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.debug(f)
    }
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
    String
}

impl_default_attr_values! {
    Val
}

impl<T: AttrValue + TypePath + FromReflect + Clone + PartialEq> AttrValue for Option<T> {
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
}

// #[cfg(feature = "bevy_asset")]
impl<T: bevy_asset::Asset> AttrValue for bevy_asset::Handle<T> {
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
}
