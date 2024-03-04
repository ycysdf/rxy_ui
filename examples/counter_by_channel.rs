use bevy::prelude::*;
use rxy_ui::prelude::*;

use core::sync::atomic::{AtomicUsize, Ordering};
use futures_lite::StreamExt;
use std::sync::Arc;

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
    commands.spawn_view_on_root(counter());
}

fn counter() -> impl IntoView<BevyRenderer> {
    let count = Arc::new(AtomicUsize::new(0));
    let (sender, receiver) = async_channel::unbounded();
    sender.send_blocking(()).unwrap();

    div().size_screen().flex().center().children(
        div().flex_col().items_center().gap(16).children((
            div().children((
                "Counter: ",
                receiver
                    .map({
                        let count = count.clone();
                        move |_| count.load(Ordering::Relaxed).to_string()
                    })
                    .boxed(),
            )),
            div()
                .children("Increment")
                .style((
                    x().px(16).py(8).bg_color(Color::DARK_GRAY),
                    x_hover().bg_color(Color::GRAY),
                    x_active().outline_color(Color::BLUE).outline_width(2),
                ))
                .on_pointer_click(move || {
                    count.fetch_add(1, Ordering::Relaxed);
                    sender.send_blocking(()).unwrap();
                }),
        )),
    )
}
