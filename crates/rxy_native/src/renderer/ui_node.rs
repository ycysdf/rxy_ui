use std::ops::{Deref, DerefMut};

use bevy_ecs::component::Component;
#[cfg(feature = "reflect")]
use bevy_ecs::prelude::*;
use kurbo::{RoundedRectRadii, Vec2};
use vello::{peniko::Color, Scene};

use crate::Val;

#[derive(Component, Default, Clone, Debug)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect), reflect(Component, Default))]
pub struct Node {
   pub(crate) stack_index: u32,
   pub(crate) calculated_size: glam::Vec2,
   pub(crate) outline_width: f32,
   /// The amount of space between the outline and the edge of the node.
   pub(crate) outline_offset: f32,
   pub(crate) unrounded_size: glam::Vec2,
}

#[derive(Component, Clone, Debug)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect), reflect(Component, Default))]
pub struct BorderRadius {
   pub top_left: Val,
   pub top_right: Val,
   pub bottom_right: Val,
   pub bottom_left: Val,
}

impl Default for BorderRadius {
   fn default() -> Self {
      Self::DEFAULT
   }
}

impl BorderRadius {
   pub const DEFAULT: Self = Self::ZERO;

   /// Zero curvature. All the corners will be right-angled.
   pub const ZERO: Self = Self::all(Val::Px(0.));

   /// Maximum curvature. The UI Node will take a capsule shape or circular if width and height are equal.
   pub const MAX: Self = Self::all(Val::Px(f32::MAX));


   #[inline]
   /// Set all four corners to the same curvature.
   pub const fn all(radius: Val) -> Self {
      Self {
         top_left: radius,
         top_right: radius,
         bottom_left: radius,
         bottom_right: radius,
      }
   }

   #[inline]
   pub const fn new(top_left: Val, top_right: Val, bottom_right: Val, bottom_left: Val) -> Self {
      Self {
         top_left,
         top_right,
         bottom_right,
         bottom_left,
      }
   }

   #[inline]
   /// Sets the radii to logical pixel values.
   pub const fn px(top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32) -> Self {
      Self {
         top_left: Val::Px(top_left),
         top_right: Val::Px(top_right),
         bottom_right: Val::Px(bottom_right),
         bottom_left: Val::Px(bottom_left),
      }
   }

   #[inline]
   /// Sets the radii to percentage values.
   pub const fn percent(
      top_left: f32,
      top_right: f32,
      bottom_right: f32,
      bottom_left: f32,
   ) -> Self {
      Self {
         top_left: Val::Px(top_left),
         top_right: Val::Px(top_right),
         bottom_right: Val::Px(bottom_right),
         bottom_left: Val::Px(bottom_left),
      }
   }

   #[inline]
   /// Sets the radius for the top left corner.
   /// Remaining corners will be right-angled.
   pub const fn top_left(radius: Val) -> Self {
      Self {
         top_left: radius,
         ..Self::DEFAULT
      }
   }

   #[inline]
   /// Sets the radius for the top right corner.
   /// Remaining corners will be right-angled.
   pub const fn top_right(radius: Val) -> Self {
      Self {
         top_right: radius,
         ..Self::DEFAULT
      }
   }

   #[inline]
   /// Sets the radius for the bottom right corner.
   /// Remaining corners will be right-angled.
   pub const fn bottom_right(radius: Val) -> Self {
      Self {
         bottom_right: radius,
         ..Self::DEFAULT
      }
   }

   #[inline]
   /// Sets the radius for the bottom left corner.
   /// Remaining corners will be right-angled.
   pub const fn bottom_left(radius: Val) -> Self {
      Self {
         bottom_left: radius,
         ..Self::DEFAULT
      }
   }

   #[inline]
   /// Sets the radii for the top left and bottom left corners.
   /// Remaining corners will be right-angled.
   pub const fn left(radius: Val) -> Self {
      Self {
         top_left: radius,
         bottom_left: radius,
         ..Self::DEFAULT
      }
   }

   #[inline]
   /// Sets the radii for the top right and bottom right corners.
   /// Remaining corners will be right-angled.
   pub const fn right(radius: Val) -> Self {
      Self {
         top_right: radius,
         bottom_right: radius,
         ..Self::DEFAULT
      }
   }

   #[inline]
   /// Sets the radii for the top left and top right corners.
   /// Remaining corners will be right-angled.
   pub const fn top(radius: Val) -> Self {
      Self {
         top_left: radius,
         top_right: radius,
         ..Self::DEFAULT
      }
   }

   #[inline]
   /// Sets the radii for the bottom left and bottom right corners.
   /// Remaining corners will be right-angled.
   pub const fn bottom(radius: Val) -> Self {
      Self {
         bottom_left: radius,
         bottom_right: radius,
         ..Self::DEFAULT
      }
   }

   /// Returns the [`BorderRadius`] with its `top_left` field set to the given value.
   #[inline]
   pub const fn with_top_left(mut self, radius: Val) -> Self {
      self.top_left = radius;
      self
   }

   /// Returns the [`BorderRadius`] with its `top_right` field set to the given value.
   #[inline]
   pub const fn with_top_right(mut self, radius: Val) -> Self {
      self.top_right = radius;
      self
   }

   /// Returns the [`BorderRadius`] with its `bottom_right` field set to the given value.
   #[inline]
   pub const fn with_bottom_right(mut self, radius: Val) -> Self {
      self.bottom_right = radius;
      self
   }

   /// Returns the [`BorderRadius`] with its `bottom_left` field set to the given value.
   #[inline]
   pub const fn with_bottom_left(mut self, radius: Val) -> Self {
      self.bottom_left = radius;
      self
   }

   /// Returns the [`BorderRadius`] with its `top_left` and `bottom_left` fields set to the given value.
   #[inline]
   pub const fn with_left(mut self, radius: Val) -> Self {
      self.top_left = radius;
      self.bottom_left = radius;
      self
   }

   /// Returns the [`BorderRadius`] with its `top_right` and `bottom_right` fields set to the given value.
   #[inline]
   pub const fn with_right(mut self, radius: Val) -> Self {
      self.top_right = radius;
      self.bottom_right = radius;
      self
   }

   /// Returns the [`BorderRadius`] with its `top_left` and `top_right` fields set to the given value.
   #[inline]
   pub const fn with_top(mut self, radius: Val) -> Self {
      self.top_left = radius;
      self.top_right = radius;
      self
   }

   /// Returns the [`BorderRadius`] with its `bottom_left` and `bottom_right` fields set to the given value.
   #[inline]
   pub const fn with_bottom(mut self, radius: Val) -> Self {
      self.bottom_left = radius;
      self.bottom_right = radius;
      self
   }

   pub fn resolve(&self, node_size: Vec2, viewport_size: Vec2, ui_scale: f64) -> RoundedRectRadii {
      fn resolve_val(value: Val, node_size: Vec2, viewport_size: Vec2, ui_scale: f64) -> f64 {
         match value {
            Val::Auto => 0.,
            Val::Px(px) => ui_scale * px as f64,
            Val::Percent(percent) => node_size.x.min(node_size.y) * percent as f64 / 100.,
            Val::Vw(percent) => viewport_size.x * percent as f64 / 100.,
            Val::Vh(percent) => viewport_size.y * percent as f64 / 100.,
            Val::VMin(percent) => viewport_size.x.min(viewport_size.y) * percent as f64 / 100.,
            Val::VMax(percent) => viewport_size.x.max(viewport_size.y) * percent as f64 / 100.,
         }
      }
      RoundedRectRadii {
         top_left: resolve_val(self.top_left, node_size, viewport_size, ui_scale),
         top_right: resolve_val(self.top_right, node_size, viewport_size, ui_scale),
         bottom_right: resolve_val(self.bottom_right, node_size, viewport_size, ui_scale),
         bottom_left: resolve_val(self.bottom_left, node_size, viewport_size, ui_scale),
      }
   }
}

#[derive(Component, Default, Clone)]
pub struct VelloFragment(pub Scene);

impl Deref for VelloFragment {
   type Target = Scene;

   fn deref(&self) -> &Self::Target {
      &self.0
   }
}

impl DerefMut for VelloFragment {
   fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.0
   }
}

#[derive(Component, Copy, Clone, Debug)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect), reflect(Component, Default))]
#[cfg_attr(
   feature = "serialize",
   derive(serde::Serialize, serde::Deserialize),
   reflect(Serialize, Deserialize)
)]
pub struct BackgroundColor(pub Color);

impl BackgroundColor {
   pub const DEFAULT: Self = Self(Color::WHITE);
}

impl Default for BackgroundColor {
   fn default() -> Self {
      Self::DEFAULT
   }
}

impl From<Color> for BackgroundColor {
   fn from(color: Color) -> Self {
      Self(color)
   }
}

/// The border color of the UI node.
#[derive(Component, Copy, Clone, Debug)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect), reflect(Component, Default))]
#[cfg_attr(
   feature = "serialize",
   derive(serde::Serialize, serde::Deserialize),
   reflect(Serialize, Deserialize)
)]
pub struct BorderColor(pub Color);

impl From<Color> for BorderColor {
   fn from(color: Color) -> Self {
      Self(color)
   }
}

impl BorderColor {
   pub const DEFAULT: Self = BorderColor(Color::WHITE);
}

impl Default for BorderColor {
   fn default() -> Self {
      Self::DEFAULT
   }
}

#[derive(Component, Copy, Clone, Default, Debug)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect), reflect(Component, Default))]
#[cfg_attr(
   feature = "serialize",
   derive(serde::Serialize, serde::Deserialize),
   reflect(Serialize, Deserialize)
)]
pub struct Outline {
   /// The width of the outline.
   ///
   /// Percentage `Val` values are resolved based on the width of the outlined [`Node`].
   pub width: Val,
   /// The amount of space between a node's outline the edge of the node.
   ///
   /// Percentage `Val` values are resolved based on the width of the outlined [`Node`].
   pub offset: Val,
   /// The color of the outline.
   ///
   /// If you are frequently toggling outlines for a UI node on and off it is recommended to set `Color::None` to hide the outline.
   /// This avoids the table moves that would occur from the repeated insertion and removal of the `Outline` component.
   pub color: Color,
}

impl Outline {
   /// Create a new outline
   pub const fn new(width: Val, offset: Val, color: Color) -> Self {
      Self {
         width,
         offset,
         color,
      }
   }
}
