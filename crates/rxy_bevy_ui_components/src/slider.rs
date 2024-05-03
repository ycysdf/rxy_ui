use std::fmt::Debug;

use bevy::{
    app::AppExit,
    prelude::{Color, Res},
};
use bevy::prelude::*;
use bevy_mod_picking::prelude::On;
use rxy_ui::prelude::*;

use super::{COLOR_PRIMARY, FocusStyle, XConfirm};

#[derive(TypedStyle)]
pub struct SliderRootStyle;
#[derive(TypedStyle)]
pub struct SliderBgStyle;
#[derive(TypedStyle)]
pub struct SliderIndicatorStyle;
#[derive(TypedStyle)]
pub struct SliderThumbStyle;

#[schema]
pub fn schema_slider(
   mut ctx: SchemaCtx,
   value: ReadSignal<f32>,
   readonly: ReadSignal<bool>,
   onchange: Sender<f32>,
) -> impl IntoElementView<BevyRenderer> {
   let value = ctx.use_controlled_state(value, onchange);
   let width = use_memo(move |_| Val::Percent(value.get() * 100.0));
   let is_drag = use_rw_signal(false);

   {
      let height = 10.;

      let indicator_height = 30.;
      let indicator_width = 30.;
      let indicator_spacing = 20.;

      let thumb_height = 26.;
      let thumb_width = 26.;

      ctx.default_typed_style(SliderRootStyle, || {
         x().h(height)
            .flex()
            .min_w(150.)
            .bg_color(Color::WHITE)
            .relative()
            .items_center()
      });
      ctx.default_typed_style(SliderBgStyle, || {
         x().h_full()
            .bg_color(COLOR_PRIMARY)
            .absolute()
            .left(0)
            .top(0)
      });
      ctx.default_typed_style(SliderIndicatorStyle, || {
         x().h(indicator_height)
            .w(indicator_width)
            .ml(-indicator_width / 2.0)
            .center()
            .top(-(indicator_height + thumb_height) / 2.0 - indicator_spacing)
      });
      ctx.default_typed_style(SliderThumbStyle, || {
         (
            x().bg_color(tailwind::GRAY_600)
               .h(thumb_height)
               .w(thumb_width)
               .top(-8)
               .ml(-thumb_width / 2.0)
               .absolute(),
            x_hover().bg_color(tailwind::GRAY_500),
            FocusStyle,
         )
      });
   }

   div().name("slider").style(SliderRootStyle).children((
      div().name("slider-bg").style(SliderBgStyle).width(width),
      x_if(
         is_drag,
         view_builder(move |_, _| {
            div()
               .name("slider-indicator")
               .style(SliderIndicatorStyle)
               .left(width)
               .children(rx(move || format!("{:.1}", value.get())))
         }),
      ),
      button()
         .name("slider-thumb")
         .style(SliderThumbStyle)
         .left(width)
         .rx_member(move || {
            (!readonly.get()).then_some(
               ().on_pointer_drag(move |e: Res<ListenerInputPointerDrag>| {
                  value.update(|value| {
                     *value = (*value + e.delta.x / 150.0).clamp(0.0, 1.0);
                  });
               })
               .on_pointer_drag_start(move || {
                  is_drag.set(true);
               })
               .on_pointer_drag_end(move || {
                  is_drag.set(false);
               }),
            )
         }),
   ))
}
