use crate::BevyWrapper;
use bevy_render::prelude::Visibility;
use bevy_ui::{Val, ZIndex};
use rxy_bevy_element::attr_values;
use std::borrow::Cow;

#[macro_export]
macro_rules! impl_bevy_wrapper_into {
    ($($ty:ty,)*) => {
        $(
            impl From<$ty> for BevyWrapper<$ty> {
                fn from(val: $ty) -> Self {
                    BevyWrapper(val)
                }
            }
        )*
    };
}
#[macro_export]
macro_rules! impl_into_bevy_wrapper_into {
    ($($from:ty => $to:ty,)*) => {
        $(
            impl From<$from> for BevyWrapper<$to> {
                fn from(val: $from) -> Self {
                    BevyWrapper(val.into())
                }
            }
        )*
    };
}

impl From<i32> for BevyWrapper<ZIndex> {
    fn from(val: i32) -> Self {
        BevyWrapper(ZIndex::Global(val))
    }
}

impl From<f32> for BevyWrapper<Val> {
    fn from(val: f32) -> Self {
        BevyWrapper(Val::Px(val))
    }
}

impl From<i32> for BevyWrapper<Val> {
    fn from(val: i32) -> Self {
        BevyWrapper(Val::Px(val as _))
    }
}

impl From<f32> for BevyWrapper<glam::Quat> {
    fn from(val: f32) -> Self {
        BevyWrapper(glam::Quat::from_rotation_z(val))
    }
}

impl From<bool> for BevyWrapper<Visibility> {
    fn from(val: bool) -> Self {
        BevyWrapper(if val {
            Visibility::Visible
        } else {
            Visibility::Hidden
        })
    }
}

impl_into_bevy_wrapper_into!(
    bevy_render::prelude::Color => attr_values::UiTexture,
    bevy_render::prelude::Color => bevy_ui::BackgroundColor,
    bevy_render::prelude::Color => bevy_ui::BorderColor,
    &'static str => Cow<'static, str>,
);

impl_bevy_wrapper_into! {
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
    &'static str,
    Cow<'static, str>,
}

impl_bevy_wrapper_into! {
    Val,
    attr_values::UiTexture,
    attr_values::TextSections,
    attr_values::UiOptionalRect,
    attr_values::OptionalOverflow,
    attr_values::OptionalTransform,
    bevy_render::prelude::Color,
    bevy_text::BreakLineOn,
    bevy_text::TextAlignment,
    bevy_ui::BackgroundColor,
    bevy_ui::BorderColor,
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
    bevy_ui::OverflowAxis,
    bevy_ui::ZIndex,
}
