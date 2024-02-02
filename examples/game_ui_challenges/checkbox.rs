use crate::FocusStyle;
use crate::XConfirm;
use crate::COLOR_PRIMARY;
use bevy::prelude::*;
use rxy_ui::prelude::*;

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
            FocusStyle,
        )
    });
    button()
        .name("checkbox")
        .style(CheckboxStyle)
        .bg_color(rx(move || is_checked.get().then_some(COLOR_PRIMARY)))
        .rx_member(move || {
            (!readonly.get()).then_some(().on(XConfirm, move || {
                is_checked.update(|is_checked| *is_checked = !*is_checked);
            }))
        })
}
