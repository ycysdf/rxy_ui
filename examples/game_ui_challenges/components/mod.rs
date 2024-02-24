use bevy::prelude::Color;
use rxy_bevy::event::{ElementEventId, ElementEventIds};
use rxy_macro::TypedStyle;

mod checkbox;
mod select;
mod slider;

use bevy::prelude::{Gamepad, GamepadButton, GamepadButtonType, KeyCode};
pub use checkbox::*;
use rxy_bevy::style::{DefaultStyleDef, TypedSharedStyleView};
use rxy_bevy::BevyRenderer;
use rxy_core::prelude::x_focus;
use rxy_core::style::StyleSheets;
use rxy_core::IntoView;
use rxy_ui::prelude::*;
pub use select::*;
pub use slider::*;

pub const COLOR_PRIMARY: Color = Color::BLUE;

#[derive(Copy, Clone, Debug)]
pub struct XConfirm;

impl ElementEventIds for XConfirm {
    fn iter_event_ids(self) -> impl Iterator<Item = ElementEventId> + Send + 'static {
        (
            x_just_pressed(KeyCode::Return),
            x_just_pressed(GamepadButton::new(Gamepad::new(1), GamepadButtonType::West)),
            x_pointer_click(),
        )
            .iter_event_ids()
    }
}

#[derive(TypedStyle)]
pub struct FocusStyle;

impl DefaultStyleDef for FocusStyle {
    fn def_default() -> impl IntoView<BevyRenderer> {
        FocusStyle::def(
            x_focus()
                .outline_width(2)
                .outline_offset(2)
                .outline_color(COLOR_PRIMARY),
        )
    }
}
