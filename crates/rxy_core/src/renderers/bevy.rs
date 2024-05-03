use crate::{
   impl_attr_value, impl_attr_value_and_wrapper, impl_x_value_wrappers, smallbox, AttrValue,
   SmallBox, XValueWrapper, S1,
};
use bevy_asset::Handle;
use bevy_color::*;

impl_attr_value_and_wrapper! {
    Color => Color::srgba_u8(0, 0, 0, 0),
    bevy_text::BreakLineOn => bevy_text::BreakLineOn::WordBoundary,
    bevy_text::JustifyText => bevy_text::JustifyText::Left,
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
    bevy_ui::GridAutoFlow,
    bevy_ui::RepeatedGridTrack => bevy_ui::RepeatedGridTrack::auto(1),
    bevy_ui::GridTrack,
    bevy_ui::GridPlacement,
    bevy_render::prelude::Visibility,
    // bevy_transform::prelude::Transform,
    glam::Quat,
    glam::Vec3,
    bevy_ui::OverflowAxis
}

impl_x_value_wrappers!(
   Srgba
   Srgba=>Color
   LinearRgba
   LinearRgba=>Color
   Hsla
   Hsla=>Color
   Hsva
   Hsva=>Color
   Hwba
   Hwba=>Color
   Laba
   Laba=>Color
   Lcha
   Lcha=>Color
   Oklaba
   Oklaba=>Color
   Oklcha
   Oklcha=>Color
   Xyza
   Xyza=>Color
);

impl Into<XValueWrapper<bevy_ui::ZIndex>> for i32 {
   fn into(self) -> XValueWrapper<bevy_ui::ZIndex> {
      XValueWrapper(bevy_ui::ZIndex::Global(self))
   }
}

impl Into<XValueWrapper<bevy_ui::Val>> for i32 {
   fn into(self) -> XValueWrapper<bevy_ui::Val> {
      XValueWrapper(bevy_ui::Val::Px(self as _))
   }
}

impl Into<XValueWrapper<bevy_ui::Val>> for f32 {
   fn into(self) -> XValueWrapper<bevy_ui::Val> {
      XValueWrapper(bevy_ui::Val::Px(self))
   }
}

impl Into<XValueWrapper<i32>> for f32 {
   fn into(self) -> XValueWrapper<i32> {
      XValueWrapper(self as _)
   }
}

impl Into<XValueWrapper<f32>> for i32 {
   fn into(self) -> XValueWrapper<f32> {
      XValueWrapper(self as _)
   }
}

impl Into<XValueWrapper<bevy_render::prelude::Visibility>> for bool {
   fn into(self) -> XValueWrapper<bevy_render::prelude::Visibility> {
      XValueWrapper(
         self
            .then(|| bevy_render::prelude::Visibility::Inherited)
            .unwrap_or(bevy_render::prelude::Visibility::Hidden),
      )
   }
}

impl Into<XValueWrapper<glam::Quat>> for f32 {
   fn into(self) -> XValueWrapper<glam::Quat> {
      XValueWrapper(glam::Quat::from_rotation_z(self))
   }
}

impl Into<XValueWrapper<glam::Vec3>> for f32 {
   fn into(self) -> XValueWrapper<glam::Vec3> {
      XValueWrapper(glam::Vec3::new(self, self, self))
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

impl<T> Into<XValueWrapper<Handle<T>>> for Handle<T>
where
   T: bevy_asset::Asset,
{
   fn into(self) -> XValueWrapper<Handle<T>> {
      XValueWrapper(self)
   }
}
