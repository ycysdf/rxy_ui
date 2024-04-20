use rxy_core::impl_attr_value_and_wrapper;
use vello::kurbo::{Point, Rect, Shape};
use rxy_core::{
    impl_attr_value, impl_x_value_wrappers, smallbox, AttrValue,
    SmallBox, XValueWrapper, S1,
};

impl_attr_value_and_wrapper! {
    crate::Val,
    crate::Display,
    crate::PositionType,
    crate::Direction,
    crate::AlignItems,
    crate::JustifyItems,
    crate::AlignSelf,
    crate::JustifySelf,
    crate::AlignContent,
    crate::JustifyContent,
    crate::FlexDirection,
    crate::FlexWrap,
    // crate::GridAutoFlow,
    // crate::RepeatedGridTrack => crate::RepeatedGridTrack::auto(1),
    // crate::GridTrack,
    // crate::GridPlacement,
    crate::Visibility,
    crate::OverflowAxis
}


// impl Into<XValueWrapper<crate::Val>> for i32 {
//     fn into(self) -> XValueWrapper<crate::Val> {
//         XValueWrapper(crate::Val::Px(self as _))
//     }
// }
//
// impl Into<XValueWrapper<crate::Val>> for f32 {
//     fn into(self) -> XValueWrapper<crate::Val> {
//         XValueWrapper(crate::Val::Px(self))
//     }
// }
//
// impl Into<XValueWrapper<crate::Visibility>> for bool {
//     fn into(self) -> XValueWrapper<crate::Visibility> {
//         XValueWrapper(
//             self
//                 .then(|| crate::Visibility::Visible)
//                 .unwrap_or(crate::Visibility::Hidden),
//         )
//     }
// }


// impl Into<XValueWrapper<bevy_ui::ZIndex>> for i32 {
//     fn into(self) -> XValueWrapper<bevy_ui::ZIndex> {
//         XValueWrapper(bevy_ui::ZIndex::Global(self))
//     }
// }
// impl AttrValue for bevy_ui::ZIndex {
//     fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
//         smallbox!(*self)
//     }
//
//     fn default_value() -> Self {
//         <Self as Default>::default()
//     }
//
//     fn eq(&self, other: &Self) -> bool {
//         match self {
//             bevy_ui::ZIndex::Local(i) => match other {
//                 bevy_ui::ZIndex::Local(o_i) => i == o_i,
//                 bevy_ui::ZIndex::Global(_) => false,
//             },
//             bevy_ui::ZIndex::Global(i) => match other {
//                 bevy_ui::ZIndex::Local(_) => false,
//                 bevy_ui::ZIndex::Global(o_i) => i == o_i,
//             },
//         }
//     }
// }