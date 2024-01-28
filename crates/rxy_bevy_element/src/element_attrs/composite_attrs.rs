// use bevy_transform::prelude::*;
// use bevy_ui::{UiRect, Val};
// use smallvec::SmallVec;
//
// use crate::attr_values::{OptionalTransform, UiOptionalRect};
// use crate::CompositeValue;
// use crate::{all_attrs, ElementAttr, ElementCompositeAttr};
// use crate::{ElementUnitAttrUntyped, SetAttrValueContext, StyleEntityExt};
//
// impl CompositeValue for UiOptionalRect {
//     fn get_count(&self) -> u8 {
//         4
//     }
//     fn get_valid_indices_bits(&self) -> u8 {
//         let mut r = 0u8;
//         if self.left.is_some() {
//             r |= 1 << 0;
//         }
//         if self.right.is_some() {
//             r |= 1 << 1;
//         }
//         if self.top.is_some() {
//             r |= 1 << 2;
//         }
//         if self.bottom.is_some() {
//             r |= 1 << 3;
//         }
//         r
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct margin;
//
// impl ElementCompositeAttr for margin {
//     const ATTRS: &'static [&'static dyn ElementUnitAttrUntyped] = &[
//         &all_attrs::margin_left,
//         &all_attrs::margin_right,
//         &all_attrs::margin_top,
//         &all_attrs::margin_bottom,
//     ];
// }
//
// impl ElementAttr for margin {
//     type Value = UiOptionalRect;
//
//     const NAME: &'static str = stringify!(margin);
//
//     fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
//         context.entity_mut.try_set_style(|style| {
//             let value = value.into();
//             style.margin = UiRect {
//                 left: value.left.unwrap_or(style.margin.left),
//                 right: value.right.unwrap_or(style.margin.right),
//                 top: value.top.unwrap_or(style.margin.top),
//                 bottom: value.bottom.unwrap_or(style.margin.bottom),
//             };
//         });
//     }
// }
//
// pub struct TwoValueWrapper<T>(T);
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct margin_horizontal;
//
// impl ElementCompositeAttr for margin_horizontal {
//     const ATTRS: &'static [&'static dyn ElementUnitAttrUntyped] =
//         &[&all_attrs::margin_left, &all_attrs::margin_right];
//
//     fn get_attrs(
//         &self,
//         _value: &dyn CompositeValue,
//     ) -> SmallVec<[&'static dyn ElementUnitAttrUntyped; 4]> {
//         SmallVec::from(Self::ATTRS)
//     }
// }
//
// impl ElementAttr for margin_horizontal {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(margin_horizontal);
//
//     fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
//         context.entity_mut.try_set_style(|style| {
//             let value = value.into();
//             style.margin = UiRect {
//                 left: value,
//                 right: value,
//                 top: style.margin.top,
//                 bottom: style.margin.bottom,
//             };
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct margin_vertical;
//
// impl ElementCompositeAttr for margin_vertical {
//     const ATTRS: &'static [&'static dyn ElementUnitAttrUntyped] =
//         &[&all_attrs::margin_top, &all_attrs::margin_bottom];
//
//     fn get_attrs(
//         &self,
//         _value: &dyn CompositeValue,
//     ) -> SmallVec<[&'static dyn ElementUnitAttrUntyped; 4]> {
//         SmallVec::from(Self::ATTRS)
//     }
// }
//
// impl ElementAttr for margin_vertical {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(margin_vertical);
//
//     fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
//         context.entity_mut.try_set_style(|style| {
//             let value = value.into();
//             style.margin = UiRect {
//                 left: style.margin.left,
//                 right: style.margin.right,
//                 top: value,
//                 bottom: value,
//             };
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct padding;
//
// impl ElementCompositeAttr for padding {
//     const ATTRS: &'static [&'static dyn ElementUnitAttrUntyped] = &[
//         &all_attrs::padding_left,
//         &all_attrs::padding_right,
//         &all_attrs::padding_top,
//         &all_attrs::padding_bottom,
//     ];
// }
//
// impl ElementAttr for padding {
//     type Value = UiOptionalRect;
//
//     const NAME: &'static str = stringify!(padding);
//
//     fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
//         context.entity_mut.try_set_style(|style| {
//             let value = value.into();
//             style.padding = UiRect {
//                 left: value.left.unwrap_or(style.padding.left),
//                 right: value.right.unwrap_or(style.padding.right),
//                 top: value.top.unwrap_or(style.padding.top),
//                 bottom: value.bottom.unwrap_or(style.padding.bottom),
//             };
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct padding_horizontal;
//
// impl ElementCompositeAttr for padding_horizontal {
//     const ATTRS: &'static [&'static dyn ElementUnitAttrUntyped] =
//         &[&all_attrs::padding_left, &all_attrs::padding_right];
//
//     fn get_attrs(
//         &self,
//         _value: &dyn CompositeValue,
//     ) -> SmallVec<[&'static dyn ElementUnitAttrUntyped; 4]> {
//         SmallVec::from(Self::ATTRS)
//     }
// }
//
// impl ElementAttr for padding_horizontal {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(padding_horizontal);
//
//     fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
//         context.entity_mut.try_set_style(|style| {
//             let value = value.into();
//             style.padding = UiRect {
//                 left: value,
//                 right: value,
//                 top: style.padding.top,
//                 bottom: style.padding.bottom,
//             };
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct padding_vertical;
//
// impl ElementCompositeAttr for padding_vertical {
//     const ATTRS: &'static [&'static dyn ElementUnitAttrUntyped] =
//         &[&all_attrs::padding_top, &all_attrs::padding_bottom];
//
//     fn get_attrs(
//         &self,
//         _value: &dyn CompositeValue,
//     ) -> SmallVec<[&'static dyn ElementUnitAttrUntyped; 4]> {
//         SmallVec::from(Self::ATTRS)
//     }
// }
//
// impl ElementAttr for padding_vertical {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(padding_vertical);
//
//     fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
//         context.entity_mut.try_set_style(|style| {
//             let value = value.into();
//             style.padding = UiRect {
//                 left: style.padding.left,
//                 right: style.padding.right,
//                 top: value,
//                 bottom: value,
//             };
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct border;
//
// impl ElementCompositeAttr for border {
//     const ATTRS: &'static [&'static dyn ElementUnitAttrUntyped] = &[
//         &all_attrs::border_left,
//         &all_attrs::border_right,
//         &all_attrs::border_top,
//         &all_attrs::border_bottom,
//     ];
// }
//
// impl ElementAttr for border {
//     type Value = UiOptionalRect;
//
//     const NAME: &'static str = stringify!(border);
//
//     fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
//         context.entity_mut.try_set_style(|style| {
//             let value = value.into();
//             style.border = UiRect {
//                 left: value.left.unwrap_or(style.border.left),
//                 right: value.right.unwrap_or(style.border.right),
//                 top: value.top.unwrap_or(style.border.top),
//                 bottom: value.bottom.unwrap_or(style.border.bottom),
//             };
//         });
//     }
// }
//
// impl CompositeValue for OptionalTransform {
//     fn get_count(&self) -> u8 {
//         3
//     }
//
//     fn get_valid_indices_bits(&self) -> u8 {
//         let mut r = 0u8;
//         if self.translation.is_some() {
//             r |= 1 << 0;
//         }
//         if self.rotation.is_some() {
//             r |= 1 << 1;
//         }
//         if self.scale.is_some() {
//             r |= 1 << 2;
//         }
//         r
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct transform;
//
// impl ElementCompositeAttr for transform {
//     const ATTRS: &'static [&'static dyn ElementUnitAttrUntyped] = &[
//         &all_attrs::translation,
//         &all_attrs::rotation,
//         &all_attrs::scale,
//     ];
// }
//
// impl ElementAttr for transform {
//     type Value = OptionalTransform;
//
//     const NAME: &'static str = stringify!(transform);
//
//     fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
//         let value = value.into();
//         let Some(mut t) = context.entity_mut.get_mut::<Transform>() else {
//             return;
//         };
//         t.translation = value.translation.unwrap_or(t.translation);
//         t.rotation = value.rotation.unwrap_or(t.rotation);
//         t.scale = value.scale.unwrap_or(t.scale);
//     }
// }
