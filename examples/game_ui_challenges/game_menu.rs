#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use bevy::prelude::*;
use rxy_bevy::navigation::RxyKeyboardNavigationPlugin;
use rxy_ui::prelude::*;
use bevy::app::AppExit;

use std::fmt::Debug;

mod checkbox;
mod select;
mod slider;

use checkbox::*;
use select::*;
use slider::*;

pub const COLOR_PRIMARY: Color = Color::BLUE;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    MainMenu,
    Setting,
    InGame,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        RxyPlugin::default(),
        RxyStyleSheetPlugin::default(),
        RxyKeyboardNavigationPlugin::default(),
    ))
    .init_state::<GameState>()
    .add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn_rxy_ui(game_ui);
}

#[derive(TypedStyle)]
struct FocusStyle;

fn game_ui() -> impl IntoView<BevyRenderer> {
    (
        FocusStyle::def(
            x_focus()
                .outline_width(2)
                .outline_offset(2)
                .outline_color(COLOR_PRIMARY),
        ),
        x_res(|state: &State<GameState>| match state.get() {
            GameState::MainMenu => main_menu().into_dynamic(),
            GameState::Setting => setting().into_dynamic(),
            GameState::InGame => in_game().into_dynamic(),
        }),
    )
}

#[derive(Copy, Clone, Debug)]
pub struct XConfirm;

impl ElementEventIds for XConfirm {
    fn iter_event_ids(self) -> impl Iterator<Item = ElementEventId> + Send + 'static {
        (
            x_just_pressed(KeyCode::Enter),
            x_just_pressed(GamepadButton::new(Gamepad::new(1), GamepadButtonType::West)),
            x_pointer_click(),
        )
            .iter_event_ids()
    }
}

#[schema]
fn schema_main_menu() -> impl IntoView<BevyRenderer> {
    #[derive(TypedStyle)]
    struct MenuBtnStyle;

    (
        MenuBtnStyle::def((
            x().width(160)
                .py(8)
                .flex()
                .center()
                .bg_color(Color::DARK_GRAY),
            x_hover().bg_color(Color::GRAY),
            x_active().bg_color(COLOR_PRIMARY),
            FocusStyle,
        )),
        div().style(x().size_screen().center()).children(
            div().style(x().flex_col().gap(8).padding(20)).children({
                (
                    button().style(MenuBtnStyle).children("New Game").on(
                        XConfirm,
                        |mut next_state: ResMut<NextState<GameState>>| {
                            next_state.set(GameState::InGame);
                        },
                    ),
                    button().style(MenuBtnStyle).children("Setting").on(
                        XConfirm,
                        |mut next_state: ResMut<NextState<GameState>>| {
                            next_state.set(GameState::Setting);
                        },
                    ),
                    button().style(MenuBtnStyle).children("Exit").on(
                        XConfirm,
                        |mut app_exit: EventWriter<AppExit>| {
                            app_exit.send(AppExit);
                        },
                    ),
                )
            }),
        ),
    )
}

#[schema]
fn schema_setting() -> impl IntoView<BevyRenderer> {
    let options = ["Option 1", "Option 2", "Option 3"];

    fn setting_item(
        label: impl IntoView<BevyRenderer>,
        content: impl IntoView<BevyRenderer>,
    ) -> impl IntoView<BevyRenderer> {
        div()
            .style((
                x().flex()
                    .min_h(45)
                    .justify_between()
                    .items_center()
                    .gap(20)
                    .py(8)
                    .px(16),
                x_hover().bg_color(Color::rgba(0.25, 0.25, 0.25, 0.4)),
            ))
            .children((label, content))
    }

    fn label(str: impl Into<String>) -> impl IntoView<BevyRenderer> {
        span(str.into()).font_size(17.)
    }

    div().style(x().size_screen().center()).children(
        div().style(x().min_w(500).gap(8).flex_col()).children((
            span("Game Setting").font_size(24.).mb(20),
            setting_item(
                label("Select"),
                select::<&'static str>()
                    .value(options[0])
                    .slot_content(view_builder(move |_, _| {
                        x_iter(options.map(|n| {
                            selection_item(n, |item| {
                                button()
                                    .style((
                                        x().flex().py(6).center(),
                                        x_hover().bg_color(Color::DARK_GRAY),
                                        FocusStyle,
                                    ))
                                    .bg_color(item.is_selected.then_some(Color::BLUE))
                                    .children((item.value,))
                            })
                        }))
                    })),
            ),
            setting_item(
                label("CheckBox"),
                checkbox().value(true).onchange(|_value| {
                    println!("checkbox value: {}", _value);
                }),
            ),
            setting_item(label("Slider"), slider().value(0.3)),
            setting_item(label("Select Item"), {
                let section_item = |item: SelectionItem<&'static str>| {
                    button()
                        .style((
                            x().flex().py(8).px(16).center(),
                            x_hover().bg_color(Color::DARK_GRAY),
                            FocusStyle,
                        ))
                        .bg_color(item.is_selected.then_some(Color::BLUE))
                        .children((item.value,))
                };
                selection_list::<&'static str>()
                    .style(x().flex_row().py(4))
                    .value("One")
                    .slot_content((
                        selection_item("One", section_item),
                        selection_item("Two", section_item),
                    ))
            }),
        )),
    )
}

#[schema]
fn schema_in_game() -> impl IntoView<BevyRenderer> {
    div()
        .style(x().size_screen().center().flex_col().gap(8))
        .children("InGame")
}
