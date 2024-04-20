//! Style types for Flexbox layout

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;

/// Controls whether flex items are forced onto one line or can wrap onto multiple lines.
///
/// Defaults to [`FlexWrap::NoWrap`]
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#flex-wrap-property)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default, PartialEq))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
   all(feature = "reflect", feature = "serialize"),
   reflect(Serialize, Deserialize)
)]
pub enum FlexWrap {
   /// Items will not wrap and stay on a single line
   NoWrap,
   /// Items will wrap according to this item's [`FlexDirection`]
   Wrap,
   /// Items will wrap in the opposite direction to this item's [`FlexDirection`]
   WrapReverse,
}

impl FlexWrap {
   pub const DEFAULT: Self = Self::NoWrap;
}

impl Default for FlexWrap {
   fn default() -> Self {
      Self::DEFAULT
   }
}

/// The direction of the flexbox layout main axis.
///
/// There are always two perpendicular layout axes: main (or primary) and cross (or secondary).
/// Adding items will cause them to be positioned adjacent to each other along the main axis.
/// By varying this value throughout your tree, you can create complex axis-aligned layouts.
///
/// Items are always aligned relative to the cross axis, and justified relative to the main axis.
///
/// The default behavior is [`FlexDirection::Row`].
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#flex-direction-property)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default, PartialEq))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
   all(feature = "reflect", feature = "serialize"),
   reflect(Serialize, Deserialize)
)]
pub enum FlexDirection {
   /// Defines +x as the main axis
   ///
   /// Items will be added from left to right in a row.
   Row,
   /// Defines +y as the main axis
   ///
   /// Items will be added from top to bottom in a column.
   Column,
   /// Defines -x as the main axis
   ///
   /// Items will be added from right to left in a row.
   RowReverse,
   /// Defines -y as the main axis
   ///
   /// Items will be added from bottom to top in a column.
   ColumnReverse,
}

impl Default for FlexDirection {
   fn default() -> Self {
      Self::DEFAULT
   }
}

impl FlexDirection {
   pub const DEFAULT: Self = Self::Row;

   #[inline]
   /// Is the direction [`FlexDirection::Row`] or [`FlexDirection::RowReverse`]?
   pub fn is_row(self) -> bool {
      matches!(self, Self::Row | Self::RowReverse)
   }

   #[inline]
   /// Is the direction [`FlexDirection::Column`] or [`FlexDirection::ColumnReverse`]?
   pub fn is_column(self) -> bool {
      matches!(self, Self::Column | Self::ColumnReverse)
   }

   #[inline]
   /// Is the direction [`FlexDirection::RowReverse`] or [`FlexDirection::ColumnReverse`]?
   pub fn is_reverse(self) -> bool {
      matches!(self, Self::RowReverse | Self::ColumnReverse)
   }

   // #[inline]
   // /// The `AbsoluteAxis` that corresponds to the main axis
   // pub(crate) fn main_axis(self) -> AbsoluteAxis {
   //    match self {
   //       Self::Row | Self::RowReverse => AbsoluteAxis::Horizontal,
   //       Self::Column | Self::ColumnReverse => AbsoluteAxis::Vertical,
   //    }
   // }
   //
   // #[inline]
   // /// The `AbsoluteAxis` that corresponds to the cross axis
   // pub(crate) fn cross_axis(self) -> AbsoluteAxis {
   //    match self {
   //       Self::Row | Self::RowReverse => AbsoluteAxis::Vertical,
   //       Self::Column | Self::ColumnReverse => AbsoluteAxis::Horizontal,
   //    }
   // }
}
