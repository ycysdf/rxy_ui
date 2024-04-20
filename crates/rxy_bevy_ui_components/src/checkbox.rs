use bevy_render::prelude::Color;

use rxy_ui::prelude::*;

// use bevy::prelude::*;
use crate::{x_theme_once, x_ui_setting_once};

#[derive(TypedStyle)]
pub struct CheckboxStyle;

#[schema]
pub fn schema_checkbox(
   mut ctx: SchemaCtx,
   value: ReadSignal<bool>,
   readonly: ReadSignal<bool>,
   onchange: Sender<bool>,
) -> impl IntoElementView<BevyRenderer> {
   let is_checked = ctx.use_controlled_state(value, onchange);
   ctx.default_typed_style(CheckboxStyle, || {
      let size = 20;
      (
         x().center()
            .size(size)
            .border(1)
            .border_color(Color::DARK_GRAY),
         x_hover().bg_color(Color::DARK_GRAY),
         // FocusStyle,
      )
   });

   button()
      .name("checkbox")
      .style(CheckboxStyle)
      .bg_color(rx(move || {
         is_checked
            .get()
            .then_some(x_theme_once(|n| n.primary_color))
      }))
      .rx_member(move || {
         readonly.not_then_some(x_ui_setting_once(move |n| {
            ().on(n.confirm_event_ids.clone(), move || {
               is_checked.update(|is_checked| *is_checked = !*is_checked);
            })
         }))
      })
}
