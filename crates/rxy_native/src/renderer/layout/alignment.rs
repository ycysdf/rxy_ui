use crate::{FlexDirection, FlexWrap};
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;

/// Used to control how each individual item is aligned by default within the space they're given.
/// - For Flexbox containers, sets default cross axis alignment of the child items.
/// - For CSS Grid containers, controls block (vertical) axis alignment of children of this grid container within their grid areas.
///
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-items>
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default, PartialEq))]
#[cfg_attr(
   all(feature = "reflect", feature = "serialize"),
   reflect(Serialize, Deserialize)
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum AlignItems {
   /// The items are packed in their default position as if no alignment was applied.
   Default,
   /// The items are packed towards the start of the axis.
   Start,
   /// The items are packed towards the end of the axis.
   End,
   /// The items are packed towards the start of the axis, unless the flex direction is reversed;
   /// then they are packed towards the end of the axis.
   FlexStart,
   /// The items are packed towards the end of the axis, unless the flex direction is reversed;
   /// then they are packed towards the start of the axis.
   FlexEnd,
   /// The items are packed along the center of the axis.
   Center,
   /// The items are packed such that their baselines align.
   Baseline,
   /// The items are stretched to fill the space they're given.
   Stretch,
}

impl AlignItems {
   pub const DEFAULT: Self = Self::Default;
}

impl Default for AlignItems {
   fn default() -> Self {
      Self::DEFAULT
   }
}

/// Used to control how each individual item is aligned by default within the space they're given.
/// - For Flexbox containers, this property has no effect. See `justify_content` for main axis alignment of flex items.
/// - For CSS Grid containers, sets default inline (horizontal) axis alignment of child items within their grid areas.
///
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items>
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default, PartialEq))]
#[cfg_attr(
   all(feature = "reflect", feature = "serialize"),
   reflect(Serialize, Deserialize)
)]
pub enum JustifyItems {
   /// The items are packed in their default position as if no alignment was applied.
   Default,
   /// The items are packed towards the start of the axis.
   Start,
   /// The items are packed towards the end of the axis.
   End,
   /// The items are packed along the center of the axis
   Center,
   /// The items are packed such that their baselines align.
   Baseline,
   /// The items are stretched to fill the space they're given.
   Stretch,
}

impl JustifyItems {
   pub const DEFAULT: Self = Self::Default;
}

impl Default for JustifyItems {
   fn default() -> Self {
      Self::DEFAULT
   }
}

/// Used to control how the specified item is aligned within the space it's given.
/// - For Flexbox items, controls cross axis alignment of the item.
/// - For CSS Grid items, controls block (vertical) axis alignment of a grid item within its grid area.
///
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-self>
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default, PartialEq))]
#[cfg_attr(
   all(feature = "reflect", feature = "serialize"),
   reflect(Serialize, Deserialize)
)]
pub enum AlignSelf {
   /// Use the parent node's [`AlignItems`] value to determine how this item should be aligned.
   Auto,
   /// This item will be aligned with the start of the axis.
   Start,
   /// This item will be aligned with the end of the axis.
   End,
   /// This item will be aligned with the start of the axis, unless the flex direction is reversed;
   /// then it will be aligned with the end of the axis.
   FlexStart,
   /// This item will be aligned with the end of the axis, unless the flex direction is reversed;
   /// then it will be aligned with the start of the axis.
   FlexEnd,
   /// This item will be aligned along the center of the axis.
   Center,
   /// This item will be aligned at the baseline.
   Baseline,
   /// This item will be stretched to fill the container.
   Stretch,
}

impl AlignSelf {
   pub const DEFAULT: Self = Self::Auto;
}

impl Default for AlignSelf {
   fn default() -> Self {
      Self::DEFAULT
   }
}

/// Used to control how the specified item is aligned within the space it's given.
/// - For Flexbox items, this property has no effect. See `justify_content` for main axis alignment of flex items.
/// - For CSS Grid items, controls inline (horizontal) axis alignment of a grid item within its grid area.
///
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self>
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default, PartialEq))]
#[cfg_attr(
   all(feature = "reflect", feature = "serialize"),
   reflect(Serialize, Deserialize)
)]
pub enum JustifySelf {
   /// Use the parent node's [`JustifyItems`] value to determine how this item should be aligned.
   Auto,
   /// This item will be aligned with the start of the axis.
   Start,
   /// This item will be aligned with the end of the axis.
   End,
   /// This item will be aligned along the center of the axis.
   Center,
   /// This item will be aligned at the baseline.
   Baseline,
   /// This item will be stretched to fill the space it's given.
   Stretch,
}

impl JustifySelf {
   pub const DEFAULT: Self = Self::Auto;
}

impl Default for JustifySelf {
   fn default() -> Self {
      Self::DEFAULT
   }
}

/// Used to control how items are distributed.
/// - For Flexbox containers, controls alignment of lines if `flex_wrap` is set to [`FlexWrap::Wrap`] and there are multiple lines of items.
/// - For CSS Grid containers, controls alignment of grid rows.
///
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-content>
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default, PartialEq))]
#[cfg_attr(
   all(feature = "reflect", feature = "serialize"),
   reflect(Serialize, Deserialize)
)]
pub enum AlignContent {
   /// The items are packed in their default position as if no alignment was applied.
   Default,
   /// The items are packed towards the start of the axis.
   Start,
   /// The items are packed towards the end of the axis.
   End,
   /// The items are packed towards the start of the axis, unless the flex direction is reversed;
   /// then the items are packed towards the end of the axis.
   FlexStart,
   /// The items are packed towards the end of the axis, unless the flex direction is reversed;
   /// then the items are packed towards the start of the axis.
   FlexEnd,
   /// The items are packed along the center of the axis.
   Center,
   /// The items are stretched to fill the container along the axis.
   Stretch,
   /// The items are distributed such that the gap between any two items is equal.
   SpaceBetween,
   /// The items are distributed such that the gap between and around any two items is equal.
   SpaceEvenly,
   /// The items are distributed such that the gap between and around any two items is equal, with half-size gaps on either end.
   SpaceAround,
}

impl AlignContent {
   pub const DEFAULT: Self = Self::Default;
}

impl Default for AlignContent {
   fn default() -> Self {
      Self::DEFAULT
   }
}

/// Used to control how items are distributed.
/// - For Flexbox containers, controls alignment of items in the main axis.
/// - For CSS Grid containers, controls alignment of grid columns.
///
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default, PartialEq))]
#[cfg_attr(
   all(feature = "reflect", feature = "serialize"),
   reflect(Serialize, Deserialize)
)]
pub enum JustifyContent {
   /// The items are packed in their default position as if no alignment was applied.
   Default,
   /// The items are packed towards the start of the axis.
   Start,
   /// The items are packed towards the end of the axis.
   End,
   /// The items are packed towards the start of the axis, unless the flex direction is reversed;
   /// then the items are packed towards the end of the axis.
   FlexStart,
   /// The items are packed towards the end of the axis, unless the flex direction is reversed;
   /// then the items are packed towards the start of the axis.
   FlexEnd,
   /// The items are packed along the center of the axis.
   Center,
   /// The items are stretched to fill the container along the axis.
   Stretch,
   /// The items are distributed such that the gap between any two items is equal.
   SpaceBetween,
   /// The items are distributed such that the gap between and around any two items is equal.
   SpaceEvenly,
   /// The items are distributed such that the gap between and around any two items is equal, with half-size gaps on either end.
   SpaceAround,
}

impl JustifyContent {
   pub const DEFAULT: Self = Self::Default;
}

impl Default for JustifyContent {
   fn default() -> Self {
      Self::DEFAULT
   }
}




impl From<AlignItems> for Option<taffy::style::AlignItems> {
   fn from(value: AlignItems) -> Self {
      match value {
         AlignItems::Default => None,
         AlignItems::Start => taffy::style::AlignItems::Start.into(),
         AlignItems::End => taffy::style::AlignItems::End.into(),
         AlignItems::FlexStart => taffy::style::AlignItems::FlexStart.into(),
         AlignItems::FlexEnd => taffy::style::AlignItems::FlexEnd.into(),
         AlignItems::Center => taffy::style::AlignItems::Center.into(),
         AlignItems::Baseline => taffy::style::AlignItems::Baseline.into(),
         AlignItems::Stretch => taffy::style::AlignItems::Stretch.into(),
      }
   }
}

impl From<JustifyItems> for Option<taffy::style::JustifyItems> {
   fn from(value: JustifyItems) -> Self {
      match value {
         JustifyItems::Default => None,
         JustifyItems::Start => taffy::style::JustifyItems::Start.into(),
         JustifyItems::End => taffy::style::JustifyItems::End.into(),
         JustifyItems::Center => taffy::style::JustifyItems::Center.into(),
         JustifyItems::Baseline => taffy::style::JustifyItems::Baseline.into(),
         JustifyItems::Stretch => taffy::style::JustifyItems::Stretch.into(),
      }
   }
}

impl From<AlignSelf> for Option<taffy::style::AlignSelf> {
   fn from(value: AlignSelf) -> Self {
      match value {
         AlignSelf::Auto => None,
         AlignSelf::Start => taffy::style::AlignSelf::Start.into(),
         AlignSelf::End => taffy::style::AlignSelf::End.into(),
         AlignSelf::FlexStart => taffy::style::AlignSelf::FlexStart.into(),
         AlignSelf::FlexEnd => taffy::style::AlignSelf::FlexEnd.into(),
         AlignSelf::Center => taffy::style::AlignSelf::Center.into(),
         AlignSelf::Baseline => taffy::style::AlignSelf::Baseline.into(),
         AlignSelf::Stretch => taffy::style::AlignSelf::Stretch.into(),
      }
   }
}

impl From<JustifySelf> for Option<taffy::style::JustifySelf> {
   fn from(value: JustifySelf) -> Self {
      match value {
         JustifySelf::Auto => None,
         JustifySelf::Start => taffy::style::JustifySelf::Start.into(),
         JustifySelf::End => taffy::style::JustifySelf::End.into(),
         JustifySelf::Center => taffy::style::JustifySelf::Center.into(),
         JustifySelf::Baseline => taffy::style::JustifySelf::Baseline.into(),
         JustifySelf::Stretch => taffy::style::JustifySelf::Stretch.into(),
      }
   }
}

impl From<AlignContent> for Option<taffy::style::AlignContent> {
   fn from(value: AlignContent) -> Self {
      match value {
         AlignContent::Default => None,
         AlignContent::Start => taffy::style::AlignContent::Start.into(),
         AlignContent::End => taffy::style::AlignContent::End.into(),
         AlignContent::FlexStart => taffy::style::AlignContent::FlexStart.into(),
         AlignContent::FlexEnd => taffy::style::AlignContent::FlexEnd.into(),
         AlignContent::Center => taffy::style::AlignContent::Center.into(),
         AlignContent::Stretch => taffy::style::AlignContent::Stretch.into(),
         AlignContent::SpaceBetween => taffy::style::AlignContent::SpaceBetween.into(),
         AlignContent::SpaceAround => taffy::style::AlignContent::SpaceAround.into(),
         AlignContent::SpaceEvenly => taffy::style::AlignContent::SpaceEvenly.into(),
      }
   }
}

impl From<JustifyContent> for Option<taffy::style::JustifyContent> {
   fn from(value: JustifyContent) -> Self {
      match value {
         JustifyContent::Default => None,
         JustifyContent::Start => taffy::style::JustifyContent::Start.into(),
         JustifyContent::End => taffy::style::JustifyContent::End.into(),
         JustifyContent::FlexStart => taffy::style::JustifyContent::FlexStart.into(),
         JustifyContent::FlexEnd => taffy::style::JustifyContent::FlexEnd.into(),
         JustifyContent::Center => taffy::style::JustifyContent::Center.into(),
         JustifyContent::Stretch => taffy::style::JustifyContent::Stretch.into(),
         JustifyContent::SpaceBetween => taffy::style::JustifyContent::SpaceBetween.into(),
         JustifyContent::SpaceAround => taffy::style::JustifyContent::SpaceAround.into(),
         JustifyContent::SpaceEvenly => taffy::style::JustifyContent::SpaceEvenly.into(),
      }
   }
}

impl From<FlexDirection> for taffy::style::FlexDirection {
   fn from(value: FlexDirection) -> Self {
      match value {
         FlexDirection::Row => taffy::style::FlexDirection::Row,
         FlexDirection::Column => taffy::style::FlexDirection::Column,
         FlexDirection::RowReverse => taffy::style::FlexDirection::RowReverse,
         FlexDirection::ColumnReverse => taffy::style::FlexDirection::ColumnReverse,
      }
   }
}
impl From<FlexWrap> for taffy::style::FlexWrap {
   fn from(value: FlexWrap) -> Self {
      match value {
         FlexWrap::NoWrap => taffy::style::FlexWrap::NoWrap,
         FlexWrap::Wrap => taffy::style::FlexWrap::Wrap,
         FlexWrap::WrapReverse => taffy::style::FlexWrap::WrapReverse,
      }
   }
}