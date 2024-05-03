use bevy::prelude::*;
use rxy_ui::prelude::*;
use bevy::color::palettes::tailwind;

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

fn ui() -> impl IntoView<BevyRenderer> {
   let condition = use_rw_signal(false);
   div().size_screen().flex().center().children(
      div().flex_col().gap(24).children((
         "--Header--",
         div()
            .style((
               x().flex().py(8).px(16).center().bg_color(tailwind::GRAY_600),
               x_hover().bg_color(tailwind::GRAY_500),
            ))
            .on_pointer_click(move || {
               condition.update(|n| *n = !*n);
            })
            .children(rx(move || {
               if condition.get() {
                  "rx: True"
               } else {
                  "rx: False"
               }
            })),
         x_if_else(condition, "x_if_else: True", "Else"),
         x_if(condition, "x_if: True"),
         "--Footer--",
      )),
   )
}
