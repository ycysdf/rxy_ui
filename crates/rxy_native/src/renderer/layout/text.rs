#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;

/// Defines the text direction.
///
/// For example, English is written LTR (left-to-right) while Arabic is written RTL (right-to-left).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default, PartialEq))]
#[cfg_attr(
   all(feature = "reflect", feature = "serialize"),
   reflect(Serialize, Deserialize)
)]
pub enum Direction {
   /// Inherit from parent node.
   Inherit,
   /// Text is written left to right.
   LeftToRight,
   /// Text is written right to left.
   RightToLeft,
}

impl Direction {
   pub const DEFAULT: Self = Self::Inherit;
}

impl Default for Direction {
   fn default() -> Self {
      Self::DEFAULT
   }
}
