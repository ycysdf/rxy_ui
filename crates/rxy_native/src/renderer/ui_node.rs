use crate::renderer::geometry::Val;
use bevy_ecs::component::Component;
use kurbo::{RoundedRectRadii, Vec2};
use std::ops::{Deref, DerefMut};
use vello::{peniko::Color, Scene};

#[derive(Component, Default, Clone, Debug /*, Reflect*/)]
pub struct Node {
    pub(crate) stack_index: u32,
    pub(crate) calculated_size: Vec2,
}

#[derive(Component, Clone, Debug /*, Reflect*/)]
pub struct BorderRadius {
    pub top_left: Val,
    pub top_right: Val,
    pub bottom_right: Val,
    pub bottom_left: Val,
}

impl BorderRadius {
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

#[derive(Component,Default, Clone)]
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

#[derive(Component, Copy, Clone, Debug /*, Reflect*/)]
// #[reflect(Component, Default)]
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
#[derive(Component, Copy, Clone, Debug /*, Reflect*/)]
// #[reflect(Component, Default)]
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

#[derive(Component, Copy, Clone, Default, Debug /*, Reflect*/)]
// #[reflect(Component, Default)]
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
