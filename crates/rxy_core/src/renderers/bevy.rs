use std::borrow::Cow;

use crate::{
    impl_attr_value, impl_attr_value_and_wrapper, impl_into_attr_value_wrappers, smallbox,
    AttrValue, AttrValueWrapper, SmallBox, S1,
};
use bevy_asset::Handle;
use bevy_render::prelude::Color;

impl_attr_value_and_wrapper! {
    Color => Color::rgba_u8(0, 0, 0, 0),
    bevy_text::BreakLineOn => bevy_text::BreakLineOn::WordBoundary,
    bevy_text::TextAlignment => bevy_text::TextAlignment::Left,
    bevy_ui::Val,
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
    // bevy_transform::prelude::Transform,
    glam::Quat,
    glam::Vec3,
    bevy_ui::OverflowAxis
}

impl Into<AttrValueWrapper<bevy_ui::ZIndex>> for i32 {
    fn into(self) -> AttrValueWrapper<bevy_ui::ZIndex> {
        AttrValueWrapper(bevy_ui::ZIndex::Global(self))
    }
}

impl Into<AttrValueWrapper<bevy_ui::Val>> for i32 {
    fn into(self) -> AttrValueWrapper<bevy_ui::Val> {
        AttrValueWrapper(bevy_ui::Val::Px(self as _))
    }
}

impl Into<AttrValueWrapper<bevy_ui::Val>> for f32 {
    fn into(self) -> AttrValueWrapper<bevy_ui::Val> {
        AttrValueWrapper(bevy_ui::Val::Px(self))
    }
}

impl Into<AttrValueWrapper<i32>> for f32 {
    fn into(self) -> AttrValueWrapper<i32> {
        AttrValueWrapper(self as _)
    }
}

impl Into<AttrValueWrapper<f32>> for i32 {
    fn into(self) -> AttrValueWrapper<f32> {
        AttrValueWrapper(self as _)
    }
}

impl Into<AttrValueWrapper<bevy_render::prelude::Visibility>> for bool {
    fn into(self) -> AttrValueWrapper<bevy_render::prelude::Visibility> {
        AttrValueWrapper(
            self.then(|| bevy_render::prelude::Visibility::Visible)
                .unwrap_or(bevy_render::prelude::Visibility::Hidden),
        )
    }
}

impl Into<AttrValueWrapper<Cow<'static, str>>> for &'static str {
    fn into(self) -> AttrValueWrapper<Cow<'static, str>> {
        AttrValueWrapper(self.into())
    }
}

impl Into<AttrValueWrapper<glam::Quat>> for f32 {
    fn into(self) -> AttrValueWrapper<glam::Quat> {
        AttrValueWrapper(glam::Quat::from_rotation_z(self))
    }
}

impl Into<AttrValueWrapper<glam::Vec3>> for f32 {
    fn into(self) -> AttrValueWrapper<glam::Vec3> {
        AttrValueWrapper(glam::Vec3::new(self, self, self))
    }
}

impl AttrValue for bevy_ui::ZIndex {
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(*self)
    }

    fn default_value() -> Self {
        <Self as Default>::default()
    }

    fn eq(&self, other: &Self) -> bool {
        match self {
            bevy_ui::ZIndex::Local(i) => match other {
                bevy_ui::ZIndex::Local(o_i) => i == o_i,
                bevy_ui::ZIndex::Global(_) => false,
            },
            bevy_ui::ZIndex::Global(i) => match other {
                bevy_ui::ZIndex::Local(_) => false,
                bevy_ui::ZIndex::Global(o_i) => i == o_i,
            },
        }
    }
}

impl<T> AttrValue for Handle<T>
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

impl<T> Into<AttrValueWrapper<Handle<T>>> for Handle<T>
where
    T: bevy_asset::Asset,
{
    fn into(self) -> AttrValueWrapper<Handle<T>> {
        AttrValueWrapper(self)
    }
}
