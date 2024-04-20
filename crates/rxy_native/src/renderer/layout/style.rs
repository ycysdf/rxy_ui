use bevy_ecs::prelude::Component;
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;

#[cfg(any(feature = "flexbox", feature = "grid"))]
pub use crate::{AlignContent, AlignItems, AlignSelf, JustifyContent, JustifyItems, JustifySelf};
#[cfg(feature = "flexbox")]
pub use crate::{FlexDirection, FlexWrap};
use crate::{UiRect, Val};
use crate::renderer::layout::text::Direction;

/// Sets the layout used for the children of this node
///
/// The default values depends on on which feature flags are enabled. The order of precedence is: Flex, Grid, Block, None.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default, PartialEq))]
#[cfg_attr(
   all(feature = "reflect", feature = "serialize"),
   reflect(Serialize, Deserialize)
)]
pub enum Display {
   /// The children will follow the block layout algorithm
   #[cfg(feature = "block_layout")]
   Block,
   /// The children will follow the flexbox layout algorithm
   #[cfg(feature = "flexbox")]
   Flex,
   /// The children will follow the CSS Grid layout algorithm
   #[cfg(feature = "grid")]
   Grid,
   /// The children will not be laid out, and will follow absolute positioning
   None,
}

impl Display {
   /// The default of Display.
   #[cfg(feature = "flexbox")]
   pub const DEFAULT: Display = Display::Flex;

   /// The default of Display.
   #[cfg(all(feature = "grid", not(feature = "flexbox")))]
   pub const DEFAULT: Display = Display::Grid;

   /// The default of Display.
   #[cfg(all(
      feature = "block_layout",
      not(feature = "flexbox"),
      not(feature = "grid")
   ))]
   pub const DEFAULT: Display = Display::Block;

   /// The default of Display.
   #[cfg(all(
      not(feature = "flexbox"),
      not(feature = "grid"),
      not(feature = "block_layout")
   ))]
   pub const DEFAULT: Display = Display::None;
}

impl core::fmt::Display for Display {
   fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      match self {
         Display::None => write!(f, "NONE"),
         #[cfg(feature = "block_layout")]
         Display::Block => write!(f, "BLOCK"),
         #[cfg(feature = "flexbox")]
         Display::Flex => write!(f, "FLEX"),
         #[cfg(feature = "grid")]
         Display::Grid => write!(f, "GRID"),
      }
   }
}

impl Default for Display {
   fn default() -> Self {
      Self::DEFAULT
   }
}

/// The positioning strategy for this item.
///
/// This controls both how the origin is determined for the [`Style::position`] field,
/// and whether or not the item will be controlled by flexbox's layout algorithm.
///
/// WARNING: this enum follows the behavior of [CSS's `position` property](https://developer.mozilla.org/en-US/docs/Web/CSS/position),
/// which can be unintuitive.
///
/// [`PositionType::Relative`] is the default value, in contrast to the default behavior in CSS.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default, PartialEq))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
   all(feature = "reflect", feature = "serialize"),
   reflect(Serialize, Deserialize)
)]
pub enum PositionType {
   /// The offset is computed relative to the final position given by the layout algorithm.
   /// Offsets do not affect the position of any other items; they are effectively a correction factor applied at the end.
   Relative,
   /// The offset is computed relative to this item's closest positioned ancestor, if any.
   /// Otherwise, it is placed relative to the origin.
   /// No space is created for the item in the page layout, and its size will not be altered.
   ///
   /// WARNING: to opt-out of layouting entirely, you must use [`Display::None`] instead on your [`Style`] object.
   Absolute,
}

impl PositionType {
   pub const DEFAULT: Self = Self::Relative;
}

impl Default for PositionType {
   fn default() -> Self {
      Self::DEFAULT
   }
}

/// Whether to show or hide overflowing items
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default, PartialEq))]
#[cfg_attr(
   all(feature = "reflect", feature = "serialize"),
   reflect(Serialize, Deserialize)
)]
pub struct Overflow {
   /// Whether to show or clip overflowing items on the x axis
   pub x: OverflowAxis,
   /// Whether to show or clip overflowing items on the y axis
   pub y: OverflowAxis,
}

impl Overflow {
   pub const DEFAULT: Self = Self {
      x: OverflowAxis::DEFAULT,
      y: OverflowAxis::DEFAULT,
   };

   /// Show overflowing items on both axes
   pub const fn visible() -> Self {
      Self {
         x: OverflowAxis::Visible,
         y: OverflowAxis::Visible,
      }
   }

   /// Clip overflowing items on both axes
   pub const fn clip() -> Self {
      Self {
         x: OverflowAxis::Clip,
         y: OverflowAxis::Clip,
      }
   }

   /// Clip overflowing items on the x axis
   pub const fn clip_x() -> Self {
      Self {
         x: OverflowAxis::Clip,
         y: OverflowAxis::Visible,
      }
   }

   /// Clip overflowing items on the y axis
   pub const fn clip_y() -> Self {
      Self {
         x: OverflowAxis::Visible,
         y: OverflowAxis::Clip,
      }
   }

   /// Overflow is visible on both axes
   pub const fn is_visible(&self) -> bool {
      self.x.is_visible() && self.y.is_visible()
   }
}

impl Default for Overflow {
   fn default() -> Self {
      Self::DEFAULT
   }
}

/// How children overflowing their container should affect layout
///
/// In CSS the primary effect of this property is to control whether contents of a parent container that overflow that container should
/// be displayed anyway, be clipped, or trigger the container to become a scroll container. However it also has secondary effects on layout,
/// the main ones being:
///
///   - The automatic minimum size Flexbox/CSS Grid items with non-`Visible` overflow is `0` rather than being content based
///   - `Overflow::Scroll` nodes have space in the layout reserved for a scrollbar (width controlled by the `scrollbar_width` property)
///
/// In Taffy, we only implement the layout related secondary effects as we are not concerned with drawing/painting. The amount of space reserved for
/// a scrollbar is controlled by the `scrollbar_width` property. If this is `0` then `Scroll` behaves identically to `Hidden`.
///
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow>
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default, PartialEq))]
#[cfg_attr(
   all(feature = "reflect", feature = "serialize"),
   reflect(Serialize, Deserialize)
)]
pub enum OverflowAxis {
   /// The automatic minimum size of this node as a flexbox/grid item should be based on the size of its content.
   /// Content that overflows this node *should* contribute to the scroll region of its parent.
   Visible,
   /// The automatic minimum size of this node as a flexbox/grid item should be based on the size of its content.
   /// Content that overflows this node should *not* contribute to the scroll region of its parent.
   Clip,
   /// The automatic minimum size of this node as a flexbox/grid item should be `0`.
   /// Content that overflows this node should *not* contribute to the scroll region of its parent.
   Hidden,
   /// The automatic minimum size of this node as a flexbox/grid item should be `0`. Additionally, space should be reserved
   /// for a scrollbar. The amount of space reserved is controlled by the `scrollbar_width` property.
   /// Content that overflows this node should *not* contribute to the scroll region of its parent.
   Scroll,
}

impl Default for OverflowAxis {
   fn default() -> Self {
      Self::DEFAULT
   }
}

impl OverflowAxis {
   pub const DEFAULT: Self = Self::Visible;

   pub const fn is_visible(&self) -> bool {
      matches!(self, Self::Visible)
   }

   /// Returns true for overflow modes that contain their contents (`Overflow::Hidden`, `Overflow::Scroll`, `Overflow::Auto`)
   /// or else false for overflow modes that allow their contains to spill (`Overflow::Visible`).
   #[inline(always)]
   pub(crate) fn is_scroll_container(self) -> bool {
      match self {
         Self::Visible | Self::Clip => false,
         Self::Hidden | Self::Scroll => true,
      }
   }

   /// Returns `Some(0.0)` if the overflow mode would cause the automatic minimum size of a Flexbox or CSS Grid item
   /// to be `0`. Else returns None.
   #[inline(always)]
   pub(crate) fn maybe_into_automatic_min_size(self) -> Option<f32> {
      match self.is_scroll_container() {
         true => Some(0.0),
         false => None,
      }
   }
}

/// A typed representation of the CSS style information for a single node.
///
/// The most important idea in flexbox is the notion of a "main" and "cross" axis, which are always perpendicular to each other.
/// The orientation of these axes are controlled via the [`FlexDirection`] field of this struct.
///
/// This struct follows the [CSS equivalent](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Flexible_Box_Layout/Basic_Concepts_of_Flexbox) directly;
/// information about the behavior on the web should transfer directly.
///
/// Detailed information about the exact behavior of each of these fields
/// can be found on [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS) by searching for the field name.
/// The distinction between margin, padding and border is explained well in
/// this [introduction to the box model](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Box_Model/Introduction_to_the_CSS_box_model).
///
/// If the behavior does not match the flexbox layout algorithm on the web, please file a bug!
#[derive(Component, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(
   feature = "reflect",
   derive(Reflect),
   reflect(Component, Default, PartialEq)
)]
#[cfg_attr(
   all(feature = "reflect", feature = "serialize"),
   reflect(Serialize, Deserialize)
)]
pub struct Style {
   /// Which layout algorithm to use when laying out this node's contents:
   ///   - [`Display::Flex`]: Use the Flexbox layout algorithm
   ///   - [`Display::Grid`]: Use the CSS Grid layout algorithm
   ///   - [`Display::None`]: Hide this node and perform layout as if it does not exist.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
   pub display: Display,

   pub scrollbar_width: f32,

   /// Whether a node should be laid out in-flow with, or independently of its siblings:
   ///  - [`PositionType::Relative`]: Layout this node in-flow with other nodes using the usual (flexbox/grid) layout algorithm.
   ///  - [`PositionType::Absolute`]: Layout this node on top and independently of other nodes.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/position>
   pub position_type: PositionType,

   /// Whether overflowing content should be displayed or clipped.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow>
   pub overflow: Overflow,

   /// Defines the text direction. For example, English is written LTR (left-to-right) while Arabic is written RTL (right-to-left).
   ///
   /// Note: the corresponding CSS property also affects box layout order, but this isn't yet implemented in Bevy.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/direction>
   pub direction: Direction,

   /// The horizontal position of the left edge of the node.
   ///  - For relatively positioned nodes, this is relative to the node's position as computed during regular layout.
   ///  - For absolutely positioned nodes, this is relative to the *parent* node's bounding box.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/left>
   pub left: Val,

   /// The horizontal position of the right edge of the node.
   ///  - For relatively positioned nodes, this is relative to the node's position as computed during regular layout.
   ///  - For absolutely positioned nodes, this is relative to the *parent* node's bounding box.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/right>
   pub right: Val,

   /// The vertical position of the top edge of the node.
   ///  - For relatively positioned nodes, this is relative to the node's position as computed during regular layout.
   ///  - For absolutely positioned nodes, this is relative to the *parent* node's bounding box.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/top>
   pub top: Val,

   /// The vertical position of the bottom edge of the node.
   ///  - For relatively positioned nodes, this is relative to the node's position as computed during regular layout.
   ///  - For absolutely positioned nodes, this is relative to the *parent* node's bounding box.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/bottom>
   pub bottom: Val,

   /// The ideal width of the node. `width` is used when it is within the bounds defined by `min_width` and `max_width`.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/width>
   pub width: Val,

   /// The ideal height of the node. `height` is used when it is within the bounds defined by `min_height` and `max_height`.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/height>
   pub height: Val,

   /// The minimum width of the node. `min_width` is used if it is greater than `width` and/or `max_width`.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/min-width>
   pub min_width: Val,

   /// The minimum height of the node. `min_height` is used if it is greater than `height` and/or `max_height`.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/min-height>
   pub min_height: Val,

   /// The maximum width of the node. `max_width` is used if it is within the bounds defined by `min_width` and `width`.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/max-width>
   pub max_width: Val,

   /// The maximum height of the node. `max_height` is used if it is within the bounds defined by `min_height` and `height`.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/max-height>
   pub max_height: Val,

   /// The aspect ratio of the node (defined as `width / height`)
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/aspect-ratio>
   pub aspect_ratio: Option<f32>,

   /// The amount of space around a node outside its border.
   ///
   /// If a percentage value is used, the percentage is calculated based on the width of the parent node.
   ///
   /// # Example
   /// ```
   /// # use rxy_native::{Style, UiRect, Val};
   /// let style = Style {
   ///     margin: UiRect {
   ///         left: Val::Percent(10.),
   ///         right: Val::Percent(10.),
   ///         top: Val::Percent(15.),
   ///         bottom: Val::Percent(15.)
   ///     },
   ///     ..Default::default()
   /// };
   /// ```
   /// A node with this style and a parent with dimensions of 100px by 300px will have calculated margins of 10px on both left and right edges, and 15px on both top and bottom edges.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin>
   pub margin: UiRect,

   /// The amount of space between the edges of a node and its contents.
   ///
   /// If a percentage value is used, the percentage is calculated based on the width of the parent node.
   ///
   /// # Example
   /// ```
   /// # use rxy_native::{Style, UiRect, Val};
   /// let style = Style {
   ///     padding: UiRect {
   ///         left: Val::Percent(1.),
   ///         right: Val::Percent(2.),
   ///         top: Val::Percent(3.),
   ///         bottom: Val::Percent(4.)
   ///     },
   ///     ..Default::default()
   /// };
   /// ```
   /// A node with this style and a parent with dimensions of 300px by 100px will have calculated padding of 3px on the left, 6px on the right, 9px on the top and 12px on the bottom.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/padding>
   pub padding: UiRect,

   /// The amount of space between the margins of a node and its padding.
   ///
   /// If a percentage value is used, the percentage is calculated based on the width of the parent node.
   ///
   /// The size of the node will be expanded if there are constraints that prevent the layout algorithm from placing the border within the existing node boundary.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-width>
   pub border: UiRect,

   /// Whether a Flexbox container should be a row or a column. This property has no effect on Grid nodes.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction>
   #[cfg(feature = "flexbox")]
   pub flex_direction: FlexDirection,

   /// Whether a Flexbox container should wrap its contents onto multiple lines if they overflow. This property has no effect on Grid nodes.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-wrap>
   #[cfg(feature = "flexbox")]
   pub flex_wrap: FlexWrap,

   /// Defines how much a flexbox item should grow if there's space available. Defaults to 0 (don't grow at all).
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-grow>
   #[cfg(feature = "flexbox")]
   pub flex_grow: f32,

   /// Defines how much a flexbox item should shrink if there's not enough space available. Defaults to 1.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-shrink>
   #[cfg(feature = "flexbox")]
   pub flex_shrink: f32,

   /// The initial length of a flexbox in the main axis, before flex growing/shrinking properties are applied.
   ///
   /// `flex_basis` overrides `size` on the main axis if both are set, but it obeys the bounds defined by `min_size` and `max_size`.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-basis>
   #[cfg(feature = "flexbox")]
   pub flex_basis: Val,

   /// Used to control how each individual item is aligned by default within the space they're given.
   /// - For Flexbox containers, sets default cross axis alignment of the child items.
   /// - For CSS Grid containers, controls block (vertical) axis alignment of children of this grid container within their grid areas.
   ///
   /// This value is overridden if [`AlignSelf`] on the child node is set.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-items>
   #[cfg(any(feature = "flexbox", feature = "grid"))]
   pub align_items: AlignItems,

   /// Used to control how the specified item is aligned within the space it's given.
   /// - For Flexbox items, controls cross axis alignment of the item.
   /// - For CSS Grid items, controls block (vertical) axis alignment of a grid item within its grid area.
   ///
   /// If set to `Auto`, alignment is inherited from the value of [`AlignItems`] set on the parent node.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-self>
   #[cfg(any(feature = "flexbox", feature = "grid"))]
   pub align_self: AlignSelf,

   /// Used to control how items are distributed.
   /// - For Flexbox containers, controls alignment of lines if `flex_wrap` is set to [`FlexWrap::Wrap`] and there are multiple lines of items.
   /// - For CSS Grid containers, controls alignment of grid rows.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-content>
   #[cfg(any(feature = "flexbox", feature = "grid"))]
   pub align_content: AlignContent,

   /// Used to control how items are distributed.
   /// - For Flexbox containers, controls alignment of items in the main axis.
   /// - For CSS Grid containers, controls alignment of grid columns.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>
   #[cfg(any(feature = "flexbox", feature = "grid"))]
   pub justify_content: JustifyContent,

   /// The size of the gutters between items in a vertical flexbox layout or between rows in a grid layout.
   ///
   /// Note: Values of `Val::Auto` are not valid and are treated as zero.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/row-gap>
   #[cfg(any(feature = "flexbox", feature = "grid"))]
   pub row_gap: Val,

   /// The size of the gutters between items in a horizontal flexbox layout or between column in a grid layout.
   ///
   /// Note: Values of `Val::Auto` are not valid and are treated as zero.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/column-gap>
   #[cfg(any(feature = "flexbox", feature = "grid"))]
   pub column_gap: Val,

   /// Used to control how each individual item is aligned by default within the space they're given.
   /// - For Flexbox containers, this property has no effect. See `justify_content` for main axis alignment of flex items.
   /// - For CSS Grid containers, sets default inline (horizontal) axis alignment of child items within their grid areas.
   ///
   /// This value is overridden if [`JustifySelf`] on the child node is set.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items>
   #[cfg(feature = "grid")]
   pub justify_items: JustifyItems,

   /// Used to control how the specified item is aligned within the space it's given.
   /// - For Flexbox items, this property has no effect. See `justify_content` for main axis alignment of flex items.
   /// - For CSS Grid items, controls inline (horizontal) axis alignment of a grid item within its grid area.
   ///
   /// If set to `Auto`, alignment is inherited from the value of [`JustifyItems`] set on the parent node.
   ///
   /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self>
   #[cfg(feature = "grid")]
   pub justify_self: JustifySelf,
   // /// Controls whether automatically placed grid items are placed row-wise or column-wise as well as whether the sparse or dense packing algorithm is used.
   // /// Only affects Grid layouts.
   // ///
   // /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow>
   // pub grid_auto_flow: GridAutoFlow,
   //
   // /// Defines the number of rows a grid has and the sizes of those rows. If grid items are given explicit placements then more rows may
   // /// be implicitly generated by items that are placed out of bounds. The sizes of those rows are controlled by `grid_auto_rows` property.
   // ///
   // /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-rows>
   // pub grid_template_rows: Vec<RepeatedGridTrack>,
   //
   // /// Defines the number of columns a grid has and the sizes of those columns. If grid items are given explicit placements then more columns may
   // /// be implicitly generated by items that are placed out of bounds. The sizes of those columns are controlled by `grid_auto_columns` property.
   // ///
   // /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns>
   // pub grid_template_columns: Vec<RepeatedGridTrack>,
   //
   // /// Defines the size of implicitly created rows. Rows are created implicitly when grid items are given explicit placements that are out of bounds
   // /// of the rows explicitly created using `grid_template_rows`.
   // ///
   // /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-rows>
   // pub grid_auto_rows: Vec<GridTrack>,
   // /// Defines the size of implicitly created columns. Columns are created implicitly when grid items are given explicit placements that are out of bounds
   // /// of the columns explicitly created using `grid_template_columns`.
   // ///
   // /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-columns>
   // pub grid_auto_columns: Vec<GridTrack>,
   //
   // /// The row in which a grid item starts and how many rows it spans.
   // ///
   // /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row>
   // pub grid_row: GridPlacement,
   //
   // /// The column in which a grid item starts and how many columns it spans.
   // ///
   // /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-column>
   // pub grid_column: GridPlacement,
}

impl Style {
   /// The [`Default`] layout, in a form that can be used in const functions
   pub const DEFAULT: Self = Self {
      display: Display::DEFAULT,
      scrollbar_width: taffy::Style::DEFAULT.scrollbar_width,
      position_type: PositionType::DEFAULT,
      left: Val::Auto,
      right: Val::Auto,
      top: Val::Auto,
      bottom: Val::Auto,
      direction: Direction::DEFAULT,
      margin: UiRect::DEFAULT,
      padding: UiRect::DEFAULT,
      border: UiRect::DEFAULT,
      flex_basis: Val::Auto,
      width: Val::Auto,
      height: Val::Auto,
      min_width: Val::Auto,
      min_height: Val::Auto,
      max_width: Val::Auto,
      max_height: Val::Auto,
      aspect_ratio: None,
      overflow: Overflow::DEFAULT,
      #[cfg(feature = "flexbox")]
      flex_direction: FlexDirection::DEFAULT,
      #[cfg(feature = "flexbox")]
      flex_wrap: FlexWrap::DEFAULT,
      #[cfg(feature = "flexbox")]
      flex_grow: 0.0,
      #[cfg(feature = "flexbox")]
      flex_shrink: 1.0,
      #[cfg(feature = "flexbox")]
      #[cfg(any(feature = "flexbox", feature = "grid"))]
      align_items: AlignItems::DEFAULT,
      #[cfg(any(feature = "flexbox", feature = "grid"))]
      justify_content: JustifyContent::DEFAULT,
      #[cfg(any(feature = "flexbox", feature = "grid"))]
      align_self: AlignSelf::DEFAULT,
      #[cfg(any(feature = "flexbox", feature = "grid"))]
      align_content: AlignContent::DEFAULT,
      #[cfg(any(feature = "flexbox", feature = "grid"))]
      row_gap: Val::ZERO,
      #[cfg(any(feature = "flexbox", feature = "grid"))]
      column_gap: Val::ZERO,
      #[cfg(feature = "grid")]
      justify_self: JustifySelf::DEFAULT,
      #[cfg(feature = "grid")]
      justify_items: JustifyItems::DEFAULT,
      // grid_auto_flow: GridAutoFlow::DEFAULT,
      // grid_template_rows: Vec::new(),
      // grid_template_columns: Vec::new(),
      // grid_auto_rows: Vec::new(),
      // grid_auto_columns: Vec::new(),
      // grid_column: GridPlacement::DEFAULT,
      // grid_row: GridPlacement::DEFAULT,
   };
}

impl Default for Style {
   fn default() -> Self {
      Style::DEFAULT
   }
}
