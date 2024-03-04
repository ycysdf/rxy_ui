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

fn ui() -> impl IntoView<BevyRenderer> {
    let (ops, source) = use_list([1, 2, 3]);

    let container_style = x()
        .flex()
        .size_screen()
        .gap(16)
        .py(16)
        .justify_center()
        .items_start();

    #[derive(TypedStyle)]
    struct BtnStyle;

    (
        BtnStyle::def((
            x().py(8).px(16).center().bg_color(Color::DARK_GRAY),
            x_hover().bg_color(Color::GRAY),
        )),
        div().style(container_style).children((
            div().flex_col().gap(8).children((
                div().children("Push").style(BtnStyle).on_pointer_click({
                    let ops = ops.clone();
                    move || {
                        ops.callback(|vec| {
                            vec.push(vec.len() as u32);
                        });
                    }
                }),
                div()
                    .children("Update First")
                    .style(BtnStyle)
                    .on_pointer_click({
                        let ops = ops.clone();
                        move || {
                            ops.callback(|vec| {
                                if vec.is_empty() {
                                    return;
                                }
                                vec.update(0, 100);
                            });
                        }
                    }),
                div()
                    .children("Patch Last")
                    .style(BtnStyle)
                    .on_pointer_click({
                        let ops = ops.clone();
                        move || {
                            ops.callback(|vec| {
                                if vec.is_empty() {
                                    return;
                                }
                                let last = vec.len() - 1;
                                vec.update(last, vec[last] + 10)
                            })
                        }
                    }),
                div().children("Pop").style(BtnStyle).on_pointer_click({
                    let ops = ops.clone();
                    move || {
                        ops.pop();
                    }
                }),
                div().children("Clear").style(BtnStyle).on_pointer_click({
                    let ops = ops.clone();
                    move || {
                        ops.clear();
                    }
                }),
            )),
            div().flex_col().gap(8).children((
                "--Header--",
                x_iter_source(source, |n,_| n.to_string()),
                "--Footer--",
            )),
        )),
    )
}
