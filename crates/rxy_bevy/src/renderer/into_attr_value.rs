use bevy_reflect::prelude::ReflectDefault;
use bevy_reflect::Reflect;
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};
use bevy_render::prelude::Color;
use bevy_text::{TextSection, TextStyle};
use bevy_ui::Val;
use bevy_ui::{BackgroundColor, BorderColor, OverflowAxis, ZIndex};
use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

use rxy_core::{smallbox, AttrValue, SmallBox, S1};
use rxy_core::{AttrValueWrapper, ElementAttr, Renderer};

#[derive(Copy, Default, Clone, Debug, PartialEq, Eq, Reflect)]
pub struct BevyAttrValue<T>(pub T);

impl<T> From<T> for BevyAttrValue<T>{
    fn from(val: T) -> Self {
        BevyAttrValue(val)
    }
}

macro_rules! impl_into_attr_value_wrapper {
    ($ty:ty) => {
        impl<R, EA> Into<AttrValueWrapper<R, EA>> for BevyAttrValue<$ty>
        where
            EA: ElementAttr<R>,
            EA::Value: From<$ty>,
            R: Renderer,
        {
            fn into(self) -> AttrValueWrapper<R, EA> {
                AttrValueWrapper(self.0.into())
            }
        }
    };
}

macro_rules! impl_default_attr_values {
    ($($ty:ty $(:$value:expr)?),*) => {
        $(
            impl_default_attr_value!($ty $(,$value)?);
        )*
    };
}

macro_rules! impl_default_attr_value {
    ($ty:ty) => {
        impl_default_attr_value!($ty, <$ty as Default>::default());
    };
    ($ty:ty,$value:expr) => {
        impl AttrValue for BevyAttrValue<$ty> {
            fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
                smallbox!(self.clone())
            }
            fn default_value() -> Self {
                BevyAttrValue($value)
            }

            fn eq(&self, other: &Self) -> bool {
                *self == *other
            }
        }
        impl_into_attr_value_wrapper!($ty);
    };
}

impl_default_attr_values! {
    // bevy_render::prelude::Color => attr_values::UiTexture,
    // bevy_render::prelude::Color => bevy_ui::BackgroundColor,
    // bevy_render::prelude::Color => bevy_ui::BorderColor,
    // &'static str => Cow<'static, str>,
    bevy_render::prelude::Color: Color::rgba_u8(0, 0, 0, 0),
    bevy_text::BreakLineOn: bevy_text::BreakLineOn::WordBoundary,
    bevy_text::TextAlignment: bevy_text::TextAlignment::Left,
    Val,
    TextSections,
    UiOptionalRect,
    OptionalOverflow,
    OptionalTransform,
    // bevy_ui::BackgroundColor,
    // bevy_ui::BorderColor,
    bevy_ui::Display,
    bevy_ui::PositionType,
    bevy_ui::Direction,
    bevy_ui::AlignItems,
    bevy_ui::JustifyItems,
    bevy_ui::AlignSelf,
    bevy_ui::JustifySelf,
    bevy_ui::AlignContent,
    bevy_ui::JustifyContent,
    bevy_ui::FlexDirection,
    bevy_ui::FlexWrap,
    bevy_render::prelude::Visibility,
    bevy_transform::prelude::Transform,
    glam::Quat,
    glam::Vec3,
    bevy_ui::OverflowAxis
}

// use crate::BevyWrapper;
// use bevy_render::prelude::Visibility;
// use bevy_ui::{Val, ZIndex};
// use std::borrow::Cow;
//
// #[macro_export]
// macro_rules! impl_bevy_wrapper_into {
//     ($($ty:ty,)*) => {
//         $(
//             impl From<$ty> for BevyWrapper<$ty> {
//                 fn from(val: $ty) -> Self {
//                     BevyWrapper(val)
//                 }
//             }
//         )*
//     };
// }
// #[macro_export]
// macro_rules! impl_into_bevy_wrapper_into {
//     ($($from:ty => $to:ty,)*) => {
//         $(
//             impl From<$from> for BevyWrapper<$to> {
//                 fn from(val: $from) -> Self {
//                     BevyWrapper(val.into())
//                 }
//             }
//         )*
//     };
// }
//
// impl From<i32> for BevyWrapper<ZIndex> {
//     fn from(val: i32) -> Self {
//         BevyWrapper(ZIndex::Global(val))
//     }
// }
//
// impl From<f32> for BevyWrapper<Val> {
//     fn from(val: f32) -> Self {
//         BevyWrapper(Val::Px(val))
//     }
// }
//
// impl From<i32> for BevyWrapper<Val> {
//     fn from(val: i32) -> Self {
//         BevyWrapper(Val::Px(val as _))
//     }
// }
//
// impl From<i32> for BevyWrapper<f32> {
//     fn from(val: i32) -> Self {
//         BevyWrapper(val as _)
//     }
// }
//
// impl From<f32> for BevyWrapper<glam::Quat> {
//     fn from(val: f32) -> Self {
//         BevyWrapper(glam::Quat::from_rotation_z(val))
//     }
// }
//
// impl From<bool> for BevyWrapper<Visibility> {
//     fn from(val: bool) -> Self {
//         BevyWrapper(if val {
//             Visibility::Visible
//         } else {
//             Visibility::Hidden
//         })
//     }
// }
//
// impl_into_bevy_wrapper_into!(
//     bevy_render::prelude::Color => attr_values::UiTexture,
//     bevy_render::prelude::Color => bevy_ui::BackgroundColor,
//     bevy_render::prelude::Color => bevy_ui::BorderColor,
//     &'static str => Cow<'static, str>,
// );
//
// impl_bevy_wrapper_into! {
//     u8,
//     u16,
//     u32,
//     u64,
//     i8,
//     i16,
//     i32,
//     i64,
//     f32,
//     f64,
//     (),
//     bool,
//     usize,
//     isize,
//     String,
//     &'static str,
//     Cow<'static, str>,
// }
//

#[allow(unused_macros)]
macro_rules! downcast_chain {
    ($value:ident,$type:ty,$($candidate_type:ty),*) => {
        {
            let r = <dyn Reflect>::downcast::<$type>($value).map(|n| *n);
            $(
                let r = r.or_else(|value| {
                    <dyn Reflect>::downcast::<$candidate_type>(value)
                                .map(|n| *n)
                                .map(Into::into)
                });
                )*
            r.ok()
        }
    };
    ($type:ty) => {
        downcast_chain!($type,);
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug, Reflect, Serialize, Deserialize)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub struct OptionalOverflow {
    pub x: Option<OverflowAxis>,
    pub y: Option<OverflowAxis>,
}

#[derive(Debug, PartialEq, Default, Clone, Copy, Reflect)]
#[reflect(Default, PartialEq)]
pub struct OptionalTransform {
    pub translation: Option<Vec3>,
    pub rotation: Option<Quat>,
    pub scale: Option<Vec3>,
}

impl OptionalTransform {
    pub fn is_some(&self) -> [bool; 3] {
        [
            self.translation.is_some(),
            self.rotation.is_some(),
            self.scale.is_some(),
        ]
    }
}

#[derive(Copy, Clone, Default, PartialEq, Debug, Reflect)]
#[reflect(PartialEq)]
pub struct UiOptionalRect {
    pub left: Option<Val>,
    pub right: Option<Val>,
    pub top: Option<Val>,
    pub bottom: Option<Val>,
}

impl UiOptionalRect {
    pub fn all(val: Val) -> Self {
        Self {
            left: Some(val),
            right: Some(val),
            top: Some(val),
            bottom: Some(val),
        }
    }
    pub fn values(&self) -> [&Option<Val>; 4] {
        [&self.left, &self.right, &self.top, &self.bottom]
    }
    pub fn zero() -> Self {
        Self {
            left: Some(Val::Px(0.)),
            right: Some(Val::Px(0.)),
            top: Some(Val::Px(0.)),
            bottom: Some(Val::Px(0.)),
        }
    }

    pub const fn new(left: Val, right: Val, top: Val, bottom: Val) -> Self {
        Self {
            left: Some(left),
            right: Some(right),
            top: Some(top),
            bottom: Some(bottom),
        }
    }

    pub const fn px(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left: Some(Val::Px(left)),
            right: Some(Val::Px(right)),
            top: Some(Val::Px(top)),
            bottom: Some(Val::Px(bottom)),
        }
    }

    pub const fn percent(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left: Some(Val::Percent(left)),
            right: Some(Val::Percent(right)),
            top: Some(Val::Percent(top)),
            bottom: Some(Val::Percent(bottom)),
        }
    }

    pub fn horizontal(value: Val) -> Self {
        Self {
            left: Some(value),
            right: Some(value),
            ..Default::default()
        }
    }

    pub fn vertical(value: Val) -> Self {
        Self {
            top: Some(value),
            bottom: Some(value),
            ..Default::default()
        }
    }

    pub fn axes(horizontal: Val, vertical: Val) -> Self {
        Self {
            left: Some(horizontal),
            right: Some(horizontal),
            top: Some(vertical),
            bottom: Some(vertical),
        }
    }

    pub fn left(value: Val) -> Self {
        Self {
            left: Some(value),
            ..Default::default()
        }
    }

    pub fn right(value: Val) -> Self {
        Self {
            right: Some(value),
            ..Default::default()
        }
    }

    pub fn top(value: Val) -> Self {
        Self {
            top: Some(value),
            ..Default::default()
        }
    }

    pub fn bottom(value: Val) -> Self {
        Self {
            bottom: Some(value),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Reflect, Clone)]
pub struct TextSections(pub Vec<TextSection>);

impl PartialEq for TextSections {
    fn eq(&self, other: &Self) -> bool {
        self.reflect_partial_eq(other).unwrap_or(false)
    }
}

impl From<String> for TextSections {
    fn from(value: String) -> Self {
        Self(vec![TextSection::new(value, TextStyle::default())])
    }
}

impl<'a> From<&'a str> for TextSections {
    fn from(value: &'a str) -> Self {
        Self(vec![TextSection::new(value, TextStyle::default())])
    }
}

impl AttrValue for BevyAttrValue<ZIndex> {
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(*self)
    }

    fn default_value() -> Self {
        <Self as Default>::default()
    }

    fn eq(&self, other: &Self) -> bool {
        match self.0 {
            ZIndex::Local(i) => match other.0 {
                ZIndex::Local(o_i) => i == o_i,
                ZIndex::Global(_) => false,
            },
            ZIndex::Global(i) => match other.0 {
                ZIndex::Local(_) => false,
                ZIndex::Global(o_i) => i == o_i,
            },
        }
    }
}
impl_into_attr_value_wrapper!(ZIndex);

impl AttrValue for BevyAttrValue<BorderColor> {
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(*self)
    }

    fn default_value() -> Self {
        BevyAttrValue(BorderColor(Color::rgba_u8(0, 0, 0, 0)))
    }

    fn eq(&self, other: &Self) -> bool {
        self.0 .0 == other.0 .0
    }
}
impl_into_attr_value_wrapper!(BorderColor);

impl AttrValue for BevyAttrValue<BackgroundColor> {
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(*self)
    }

    fn default_value() -> Self {
        BevyAttrValue(BackgroundColor(Color::rgba_u8(0, 0, 0, 0)))
    }

    fn eq(&self, other: &Self) -> bool {
        self.0 .0 == other.0 .0
    }
}
impl_into_attr_value_wrapper!(BackgroundColor);

impl<T> AttrValue for BevyAttrValue<bevy_asset::Handle<T>>
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
}

impl<T, R, EA> Into<AttrValueWrapper<R, EA>> for BevyAttrValue<bevy_asset::Handle<T>>
where
    T: bevy_asset::Asset,
    EA: ElementAttr<R>,
    EA::Value: From<bevy_asset::Handle<T>>,
    R: Renderer,
{
    fn into(self) -> AttrValueWrapper<R, EA> {
        AttrValueWrapper(self.0.into())
    }
}
