# Rxy UI

> 项目还在进行中，处于早期阶段，不建议在生产环境中使用。目前缺少大量测试用例。

受 [xilem](https://github.com/linebender/xilem) [tachys](https://github.com/gbj/tachys) 等项目的启发

## 特征：

- 编译时视图、无过程宏：通过元组 Type Builder 的方式去构建视图、成员、样式
- 性能：视图仅运行一次，仅在反应式数据发生变化的可控范围里进行更新
- 细粒度更新、支持信号 （fork 自 [tachy_reaccy](https://github.com/gbj/tachys/tree/main/tachy_reaccy)）
- 使用 `Taffy` 进行 Flexbox 布局
- 组件化：静态属性、信号属性、事件、槽
- 控制流
- 层叠样式

### Rxy UI 的目标

- 灵活、可扩展
- 高性能、零成本抽象
- 最小的样板
- 支持多个渲染器 ( 目前只支持 [Bevy](https://github.com/bevyengine/bevy) )，使其能够应用在多种场景中，比如：游戏、桌面软件、嵌入式等
- 支持主流操作系统、Web 平台

## 示例

计数器

```rust
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
```

自定义 Checkbox UI 组件

```rust
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
    ctx.default_tpyed_style(CheckboxStyle, || {
        let size = 20;
        (
            x().center()
                .size(size)
                .border(1)
                .border_color(Color::DARK_GRAY),
            x_hover().bg_color(Color::DARK_GRAY),
        )
    });
    div()
        .name("checkbox")
        .style(CheckboxStyle)
        .bg_color(rx(move || is_checked.get().then_some(COLOR_PRIMARY)))
        .rx_member(move || {
            (!readonly.get()).then_some(().on_pointer_click(move || {
                is_checked.update(|is_checked| *is_checked = !*is_checked);
            }))
        })
}
```

游戏菜单与设置

```rust
use bevy::prelude::*;
use rxy_bevy::prelude::*;
use rxy_bevy_style::prelude::*;
use xy_reactive::prelude::*;

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
    ))
    .add_state::<GameState>()
    .add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn_rxy_ui(game_ui);
}

fn game_ui() -> impl IntoView<BevyRenderer> {
    x_res(|state: &State<GameState>| match state.get() {
        GameState::MainMenu => main_menu().into_dynamic(),
        GameState::Setting => setting().into_dynamic(),
        GameState::InGame => in_game().into_dynamic(),
    })
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
        )),
        div().style(x().size_screen().center()).children(
            div().style(x().flex_col().gap(8).padding(20)).children((
                div()
                    .style(MenuBtnStyle)
                    .children("New Game")
                    .on_pointer_click(|mut next_state: ResMut<NextState<GameState>>| {
                        next_state.set(GameState::InGame);
                    }),
                div()
                    .style(MenuBtnStyle)
                    .children("Setting")
                    .on_pointer_click(|mut next_state: ResMut<NextState<GameState>>| {
                        next_state.set(GameState::Setting);
                    }),
                div().style(MenuBtnStyle).children("Exit").on_pointer_click(
                    |mut app_exit: EventWriter<AppExit>| {
                        app_exit.send(AppExit);
                    },
                ),
            )),
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

    fn label(str: &str) -> impl IntoView<BevyRenderer> {
        span(str).font_size(17.)
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
                                div()
                                    .style((
                                        x().flex().py(6).center(),
                                        x_hover().bg_color(Color::DARK_GRAY),
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
                    div()
                        .style((
                            x().flex().py(8).px(16).center(),
                            x_hover().bg_color(Color::DARK_GRAY),
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
```

## 更多示例

- 计数器：[examples/counter](examples/counter.rs)
- [Bevy UI 框架的 10 个挑战](https://github.com/bevyengine/bevy/discussions/11100)
   - 1. 游戏菜单与自定义 UI 组件：[examples/game_ui_challenges/game_menu](examples/game_ui_challenges/game_menu.rs)
   - `todo!()`
- [7GUIs](https://eugenkiss.github.io/7guis/)
   - `todo!()`
- TodoList: `todo!()`

## 教程

`todo!()`

## 计划

- Text Edit
- 更多的 UI 组件 与 示例
- 更多 Debug 功能、更多的测试用例、Element inspector
- 其他渲染器：wgpu (vello)、html、嵌入式 
- bevy 更加深入的集成
- 主题，tailwind
- DSL、样式热更新、动态样式
- 视图设计器

## 许可证

MIT License ([LICENSE-MIT](./LICENSE-MIT))

## 贡献

欢迎共享代码！

