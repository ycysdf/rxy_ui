use bevy::prelude::*;
use rxy_ui::prelude::*;

fn main() {
   let mut app = App::new();
   app.add_plugins((
      DefaultPlugins,
      RxyPlugin::default(),
      RxyStyleSheetPlugin::default(),
   ))
   .add_systems(Startup, setup);

   app.run();
}

fn setup(mut commands: Commands) {
   commands.spawn(Camera2dBundle::default());
   commands.spawn_view_on_root(ui());
}

// fn ui() -> impl IntoView<BevyRenderer> {
//     let condition = use_rw_signal(false);
//     div().class("gap-3 flex-col").children((
//         "--Header--",
//         div()
//             .style_builder(|ctx, _| {
//                 rx(|| {
//                     if true {
//                         normal().bg_color(Color::RED).either_left()
//                     } else {
//                         normal().height(100.).either_right()
//                     }
//                 })
//             })
//             .style(rx(|| {
//                 if true {
//                     normal().bg_color(Color::RED).either_left()
//                 } else {
//                     normal().height(100.).either_right()
//                 }
//             }))
//             .style(Some((
//                 normal().bg_color(Color::RED).height(100.).width(100.),
//                 hover().bg_color(Color::BLUE),
//                 active().bg_color(Color::GREEN),
//             )))
//             .style((
//                 normal().bg_color(Color::RED).height(100.).width(100.),
//                 hover().bg_color(Color::BLUE),
//                 active().bg_color(Color::GREEN),
//             ))
//             .style((
//                 normal().bg_color(Color::WHITE).height(30.).width(30.),
//                 hover().bg_color(Color::PINK),
//                 active().bg_color(Color::GREEN),
//             ))
//             .style((
//                 (
//                     normal()
//                         .bg_color(Color::WHITE)
//                         .height(10.)
//                         .width(10.)
//                         .height(50.)
//                         .width(50.),
//                     hover().bg_color(Color::RED),
//                     active().bg_color(Color::GREEN),
//                 ),
//                 normal()
//                     .bg_color(Color::WHITE)
//                     .height(10.)
//                     .width(10.)
//                     .height(50.)
//                     .width(50.),
//                 hover().bg_color(Color::RED),
//                 active().bg_color(Color::GREEN),
//             ))
//             .style(external_shared_style()),
//         "--Footer--",
//     ))
// }

fn ui() -> impl IntoView<BevyRenderer> {
   let signal = use_rw_signal(false);
   div().style(x().gap(3).flex_col()).children((
      "--Header--",
      div()
         .on_pointer_click(move || {
            signal.update(|n| *n = !*n);
         })
         .style(Some((
            x().bg_color(Color::BLUE).height(100.).width(100.),
            // hover().bg_color(Color::WHITE),
         )))
         .rx_style(move || {
            signal
               .get()
               .then_some((x().bg_color(Color::RED), x_hover().bg_color(Color::WHITE)))
         }),
      "--Footer--",
   ))
}
