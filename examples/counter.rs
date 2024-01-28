use bevy::prelude::*;
use rxy_bevy::prelude::*;
use rxy_bevy_style::prelude::*;
use xy_reactive::prelude::*;

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
    commands.spawn_rxy_ui(counter_by_signal);
}

fn counter_by_signal() -> impl IntoView<BevyRenderer> {
    let count = use_rw_signal(0);

    div().size_screen().flex().center().children(
        div().flex_col().items_center().gap(16).children((
            div().children(("Counter: ", rx(move || count.get().to_string()))),
            div()
                .children("Increment")
                .style((
                    x().px(16).py(8).bg_color(Color::DARK_GRAY),
                    x_hover().bg_color(Color::GRAY),
                    x_active().outline_color(Color::BLUE).outline_width(2),
                ))
                .on_pointer_click(move || count.update(|signal| *signal += 1)),
        )),
    )
}
