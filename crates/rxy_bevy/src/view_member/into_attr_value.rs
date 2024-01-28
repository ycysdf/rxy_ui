use crate::BevyWrapper;
use bevy_render::prelude::Visibility;
use bevy_ui::{Val, ZIndex};
use rxy_bevy_element::attr_values;
use std::borrow::Cow;

#[macro_export]
macro_rules! impl_bevy_wrapper_into {
    ($($ty:ty,)*) => {
        $(
            impl Into<BevyWrapper<Self>> for $ty {
                fn into(self) -> BevyWrapper<Self> {
                    BevyWrapper(self)
                }
            }
        )*
    };
}
#[macro_export]
macro_rules! impl_into_bevy_wrapper_into {
    ($($from:ty => $to:ty,)*) => {
        $(
            impl Into<BevyWrapper<$to>> for $from {
                fn into(self) -> BevyWrapper<$to> {
                    BevyWrapper(self.into())
                }
            }
        )*
    };
}

impl Into<BevyWrapper<ZIndex>> for i32 {
    fn into(self) -> BevyWrapper<ZIndex> {
        BevyWrapper(ZIndex::Global(self))
    }
}

impl Into<BevyWrapper<Val>> for f32 {
    fn into(self) -> BevyWrapper<Val> {
        BevyWrapper(Val::Px(self))
    }
}

impl Into<BevyWrapper<Val>> for i32 {
    fn into(self) -> BevyWrapper<Val> {
        BevyWrapper(Val::Px(self as _))
    }
}

impl Into<BevyWrapper<glam::Quat>> for f32 {
    fn into(self) -> BevyWrapper<glam::Quat> {
        BevyWrapper(glam::Quat::from_rotation_z(self))
    }
}

impl Into<BevyWrapper<Visibility>> for bool {
    fn into(self) -> BevyWrapper<Visibility> {
        BevyWrapper(if self {
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
