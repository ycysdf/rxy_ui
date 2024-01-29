# Rxy UI

<div>
  <h6>
    <a href="https://github.com/ycysdf/rxy_ui/blob/main/README-zh-Hans.md"> 中文 </a>
  </h6>
</div>

> The project is ongoing and in its early stages, and is not recommended for use in a production environment. A large number of test cases are missing.

Inspired by projects such as [xilem](https://github.com/linebender/xilem),[tachys](https://github.com/gbj/tachys).

## Features:

- Compile-time view, No procedural  macro, Use tuple Type Builder to build views, members, and styles
- Performance: Views are constructed only once by default, and reconstruction occurs only within the controllable range of changes in reactive data. Compiled-time views are utilized, and can be placed on the stack.
- Fine-grained reactivity, support signals（fork from [tachy_reaccy](https://github.com/gbj/tachys/tree/main/tachy_reaccy)）
- Componentization: static prop, signal prop, events, slots
- Control flow
- Cascading style

## Goal

- Flexible and scalable
- High reuse, composable 
- High performance, zero cost abstraction
- Minimal boilerplate
- Support multiple renderer (currently only support [Bevy](https://github.com/bevyengine/bevy), make its can be used in a variety of scenarios, such as: games, desktop software, embedded platform, the native UI, etc
- Supports mainstream operating systems and Web platforms

## The plan

- Text Edit
- More UI components and examples
- More Debug features, more test cases, Element inspector
- Other Renderers
- Bevy deeper integration, as a scenario to use? (like '@react-three/fiber'), Schema as Prefab?
- Theme，tailwind
- DSL, style hot reload, dynamic styles
- View Designer

## License

MIT License ([LICENSE-MIT](https://github.com/ycysdf/rxy_ui/blob/main/LICENSE-MIT))

## Contributions

Code sharing is welcome!

## Examples

Counter

<img src="./assets/counter.gif" />

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

Custom Checkbox UI Component

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
    ctx.default_typed_style(CheckboxStyle, || {
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

Game Menu and Setting

<img src="./assets/game_menu.gif" />

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

## More Examples

- Counter：[examples/counter](examples/counter.rs)
- [10 Challenges for Bevy UI Frameworks](https://github.com/bevyengine/bevy/discussions/11100)
   - 1. Game Menu：[examples/game_ui_challenges/game_menu](examples/game_ui_challenges/game_menu.rs)
   - `todo!()`
- [7GUIs](https://eugenkiss.github.io/7guis/)
   - `todo!()`
- TodoList: `todo!()`

## Tutorials

### Basics

Rxy UI's view is composed of a tuple of `View` elements. Any type that implements `IntoView` can be a part of the view.

As long as all members of the tuple implement `View`, the tuple itself also implements `View`, allowing views to be composed in this way.

Since strings implement `IntoView`, they can be directly included in the view.

`div` is the most commonly used view type (analogous to `NodeBundle` in Bevy and similar to the HTML div).

```rust
fn my_view() -> impl IntoView<BevyRenderer> {
    (
        div().children("Hello World"),
        "Hello World",
    )
}
```

Adding members to the `View` type through chained calls, the `children` method is used to configure the view's child views.

You can also use the `erasure_children` method to set a view's child views, The only difference from `children` method is that `erasure_children` performs type erasure on the child views, which can help avoid overly complex types.

> Currently, the `view_erasure` feature is enabled by default, forcing all child views to under go erasure and be placed on the heap. This is becasue if erasure is not performed. the types will become overly complex, causing the compiler to fail and type evaluation to overflow. You can also disable this feature and manually use `erasure_children` to erasure some child views, Avoid making your types too complex and causing compilation failures.

You can set the attrs of the view using attributes such as `width`, `height`, `flex`, `border`, `outline`, and so on.

> a complete list of currently supported attributes: [attrs](https://github.com/ycysdf/rxy_ui/blob/main/crates/rxy_bevy_element/src/element_attrs/attrs.rs),[composite_attrs](https://github.com/ycysdf/rxy_ui/blob/main/crates/rxy_bevy/src/view_member/composite_attrs.rs),[tailwind_attrs](https://github.com/ycysdf/rxy_ui/blob/main/crates/rxy_bevy_style/src/tailwind_attrs.rs)

Any type that implements `ViewMember` can be used as a member of the view, and members can be manually added using the `member` method.

Similar to `View`, as long as all members of the tuple implement `ViewMember`, the tuple itself also implements `ViewMember`.

```rust
fn my_view() -> impl IntoView<BevyRenderer> {
    let my_member = ().width(100).height(100);
    (
        div().member(my_member.clone()),
        div().member(my_member)
    )
}
```

### Event

You can add events using methods like `on_pointer_click`, and the callback function provided is a Bevy `System`. This means you can use `Res`, `Commands`, `EventWriter`, etc. as parameters for the callback function.

```rust
fn signal_example()-> impl IntoView<BevyRenderer> {
    div().children("Button")
        .on_pointer_click(|res: Res<TestRes>, event_writer: EventWriter<TestEvent>, commands: Commands| {
            println!("click")
        })
}
```

### Signal

> The current signal implementation at [xy_reactive](https://github.com/ycysdf/xy_reactive), is forked from [tachy_reaccy](https://github.com/gbj/tachys/tree/main/tachy_reaccy). It serves as the next-generation signal library for leptos, and its usage is generally similar to the leptos_reactive library.

Rxy UI supports the use of signals to rebuild views and their members.

An important function is `rx`, which requires passing a closure. Inside this closure, you can use the get method of the signal to retrieve its value. `rx` returns a type of `Reactive`.

When the value of the signal changes, the associated `View` or `ViewMember` will be rebuilt.

If the closure's return value implements `IntoView`, then the `Reactive` type implements `View`, and the `rx` function can be directly used in view,

If the closure's return value implements `IntoViewAttrMember`, then the `Reactive` type implements `ViewMember`, and the `rx` function can be directly used in view member.

If the signal types `RwSignal<T>` or `ReadSignal`, where `T` implements `IntoView` or `IntoViewAttrMember`, then the signal also implements `View` or `ViewMember`.

> `IntoViewAttrMember` represents types that can be converted to `ViewMember`. For example, the width attribute value requires a type of [Val](https://docs.rs/bevy/latest/bevy/ui/enum.Val.html), but you can pass an `i32`, and `IntoViewAttrMember` will internally convert it to `Val::Px(100)` for you.

```rust
fn signal_example() -> impl IntoView<BevyRenderer> {
    let count = use_rw_signal(0);
    div()
        .children(rx(move || count.get().to_string())) // `Reactive` 作为 `View`
        .padding(20)
        .bg_color(Color::BLUE)
        .border_y(count) // 直接将 `RwSignal` 信号 作为 `ViewMember`
        .border_color(Color::RED)
        .width(100)
        .height(rx(move || count.get() * 4 + 100)) // `Reactive` 作为 `ViewMember`
        .on_pointer_click(move || {
            count.update(|x| *x += 1);
        })
}
```

### Option、Either、Stream、Future

`Option<T>`,`Either<A,B>`,`Stream<T>`、`Future<T>`, and similar types all implement `IntoView` or `IntoViewAttrMember`. Thereforce, they can be used directly as views or view members.

For examples:

Use case for `Option`: Controlling whether to build a view or view member

> while `rx` combined with `Option` can control the visibility of the view, `x_if` is better choice for this purpose, and it will be introduced later.

```rust
fn btn(is_show: RwSignal<bool>) -> impl IntoView<BevyRenderer> {
    div()
        .children("Btn")
        .padding(20)
        .bg_color(Color::GRAY)
        .on_pointer_click(move || {
            is_show.update(|x| *x = !*x);
        })
}

fn sample_option_view() -> impl IntoView<BevyRenderer> {
    let is_show = use_rw_signal(false);
    (
        btn(is_show),
        rx(move || {
            let view = div()
                .padding(10)
                .bg_color(Color::BLUE)
                .flex()
                .center()
                .children("Show");
            if is_show.get() {
                Some(view)
            } else {
                None
            }

            // is_show.get().then_some(view) 这种写法也可以
        }),
    )
}

fn sample_option_view_member() -> impl IntoView<BevyRenderer> {
    let is_show = use_rw_signal(false);
    (
        btn(is_show),
        div()
            .padding(10)
            .flex()
            .center()
            .children("Show")
            .member(rx(move || {
                is_show.get().then_some(
                    ().bg_color(Color::BLUE)
                        .outline_color(Color::RED)
                        .outline_width(2),
                )
            }))
            // The following syntax is equivalent to the one above.
            // .rx_member(move || {
            //     is_show.get().then_some(
            //         ().bg_color(Color::BLUE)
            //             .outline_color(Color::RED)
            //             .outline_width(2),
            //     )
            // }),
    )
}
```
Use case for `Either`: Chossing which view or view member to build

> While `rx` combined with `Either` can achieve switching between views, `x_if_else` is a better choice for this purpose, and it will be introduced later.

```rust
fn sample_either_view() -> impl IntoView<BevyRenderer> {
    let is_show = use_rw_signal(false);
    (
        btn(is_show),
        rx(move || {
            let left_view = "Left";
            let right_view = "Right";
            if is_show.get() {
                left_view.either_left() // This is equivalent to Either::Left(left_view)
            } else {
                right_view.either_right() // This is equivalent to Either::Right(right_view)
            }
        }),
    )
}

fn sample_either_view_member() -> impl IntoView<BevyRenderer> {
    let is_show = use_rw_signal(false);
    (
        btn(is_show),
        div()
            .padding(10)
            .flex()
            .center()
            .children("Show")
            .member(rx(move || {
                let left_vm = ().bg_color(Color::BLUE);
                let right_vm = ().outline_color(Color::RED).outline_width(2);
                if is_show.get() {
                    left_vm.either_left() // This is equivalent to Either::Left(left_vm)
                } else {
                    right_vm.either_right() // This is equivalent to Either::Right(right_vm)
                }
            }))
    )
}
```

`Future` Example:

```rust
fn sample_future() -> impl IntoView<BevyRenderer> {
    let (sender, receiver) = oneshot::channel();
    let mut sender = SyncCell::new(Some(sender));

    let (sender2, receiver2) = async_channel::unbounded();

    (
        div()
            .padding(20)
            .bg_color({
                let receiver2 = receiver2.clone();
                async move { receiver2.recv().await.unwrap() }.boxed()
            })
            .children("Ok")
            .on_pointer_click(move || {
                let _ = sender2.try_send(Color::RED);
                if let Some(sender) = sender.get().take() {
                    sender.send("Ok").unwrap();
                }
            }),
        // For non-Boxed Future, you can use rx_future
        x_future(async {
            let r = receiver.await;
            div().padding(20).children(format!("Future: {:?}", r))
        }),
        // a Boxed Future can be used directly as a view
        async move {
            let color = receiver2.recv().await;
            div().padding(20).children(format!("Color: {:?}", color))
        }
        .boxed(),
    )
}
```

`Stream` Example:

[counter_by_channel](examples/counter_by_channel.rs)

### Control flow：x_if、x_iter、x_iter_keyed、x_iter_source

`x_if` is used to control whether to build the view. The first parameter passed to `x_if` is similar to the view member, it can be `bool`, `Reactive`, `ReadSingal`, etc.

```rust
fn sample_x_if() -> impl IntoView<BevyRenderer> {
    let is_show = use_rw_signal(false);
    (
        div()
            .children("Btn")
            .padding(20)
            .bg_color(Color::GRAY)
            .on_pointer_click(move || {
                is_show.update(|x| *x = !*x);
            }),
        x_if(is_show, span("Show1").padding(10).text_color(Color::RED)),
        x_if(
            is_show,
            view_builder(|_, _| div().padding(10).flex().center().children("Show2")),
        ),
    )
}
```

> The second use of `x_if` with `view_buillder` is because currently, views with children are not `Clone` (due to certain reasons that might be improved later). However, `x_if` requires views to implement `Clone`. Thereforce, in this case, `view_builder` is used, and a callback function is passed to make it cloneable.

`x_iter` is used to build list views, It takes an `IntoIterator`, and the `Item` needs to implement `IntoView`. It uses the item's index as the key.

`x_iter_keyed` requires manually specifying the key and expects `Item` to be of type `Keyed<K, IV>`. It has two members, the first being the key (which should implement `Hash`), and the second being the view.

```rust 
fn sample_x_iter() -> impl IntoView<BevyRenderer> {
    div()
        .flex_col()
        .gap(8)
        .children(x_iter((0..10).map(|n| format!("Item: {}", n))))
}

fn sample_x_iter_keyed() -> impl IntoView<BevyRenderer> {
    div()
        .flex_col()
        .gap(1)
        .children(x_iter_keyed((0..25).map(|n| {
            Keyed(
                n,
                span(format!("Item: {}", n))
                    .padding(10)
                    .text_color(Color::rgb_u8(n * 10, 0, 0)),
            )
        })))
}
```

If you need to update the list, you can use `rx` to wrap it.

```rust
fn sample_x_iter_keyed_rx() -> impl IntoView<BevyRenderer> {
    let signal = use_rw_signal(3);
    (
        div()
            .padding(10)
            .bg_color(Color::BLUE)
            .children("Add")
            .on_pointer_click(move || {
                signal.update(|x| *x += 1);
            }),
        rx(move || {
            div()
                .flex_col()
                .gap(1)
                .children(x_iter_keyed((0..signal.get()).map(|n| {
                    Keyed(
                        n,
                        span(format!("Item: {}", n))
                            .padding(10)
                            .text_color(Color::rgb_u8(n * 2, 255, 255)),
                    )
                })))
        }),
    )
}
```

For frequently changing list views, you can also use `x_iter_source` with `use_list`. It is more efficient at updating lists than `x_iter_keyed` and performs well in terms of performance.

Example:

```rust
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
                x_iter_source(source, |n| n.to_string()),
                "--Footer--",
            )),
        )),
    )
}
```

### Cascading Style Sheets

In the previous examples, attrs style where directly written in the views, and such styles cannot cascade. Although it's possible to override them (with some problems), it  doesn't support interactive styles like hover and press(active).

Cascading styles can indeed stack, and they have priorities, including directly set attributes. Their priorities are as follows:

Directly set attrs > Interactive styles > Inline styles > Shared styles > Styles positioned later

To add styles using the `style` method, similar to `View` and `ViewMember`, it accepts a tuple where you can include multiple style sheets.

Normal styles are constructed using `x()`, and interactive styles are construct using `x_hover()`,``

```rust
fn sample_style_sheet() -> impl IntoView<BevyRenderer> {
    div()
        .margin(50)
        .style((
            x().py(8 * 2).px(16 * 2).center().bg_color(Color::DARK_GRAY),
            x_hover().bg_color(Color::GRAY),
            x_active().outline_color(Color::GREEN).outline_width(2),
        ))
        .children("Button")
}
```

Currently, styles within the style sheet are static and do not allow the use of `rx`, `Future`, `Stream`, `Option`, etc, However, you can call `style` method multiple times to add styles.

There are also convenient methods like `style_rx`, `style_builder`, `style_option`, `style_stream`, etc (it's perferable to use these methods first, using `style_future`)

```rust
fn sample_dynamic_style_sheet() -> impl IntoView<BevyRenderer> {
    let signal = use_rw_signal(false);
    div()
        .on_pointer_click(move || {
            signal.update(|n| *n = !*n);
        })
        .style(Some((x().bg_color(Color::GRAY).height(100.).width(100.),)))
        .style_rx(move || {
            signal
                .get()
                .then_some((x().bg_color(Color::RED), x_hover().bg_color(Color::WHITE)))
        })
}
```

### Shared typed cascading styles

Define a shared style identifier struct by using the `TypedStyle` derive macro in the structure. Then, in the view, use the `def` method to define a shared typed cascading style.

To use the shared style, simply call `style` method and pass in the type.

```rust
fn sample_shared_typed_style_sheet() -> impl IntoView<BevyRenderer> {
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
            x_active().outline_color(Color::GREEN).outline_width(2),
        )),
        div().padding(50).gap(10).flex_col().children((
            div().style(MenuBtnStyle).children("Button 1"),
            div()
                .style((MenuBtnStyle, x().bg_color(Color::RED)))
                .children("Button 2"),
            div().style(MenuBtnStyle).children("Button 3"),
        )),
    )
}
```
> Currently, there are no means to modify styles in the shared style sheet, but there will be added means for modification in the future.

### Schema ( Componentization )

Firstly, you can directly reuse views through functions. However, there are some problems:

- Rust lacks optional parameters.
- The function may be called multiple times (especially if it is within an `rx`, where it is called each time the signal changes).
- It's just an ordinary function, lacking context, making it challenging to perform some encapsulation.
- There are some other problems as well, not to go into further detail.

So, we need a better way to reuse views, and that's where `Schema` comes in. This is a tentative name, and if you have a better one, fell free to suggest.


> It's preferable for the name to be a single word and not use "Component" because it's already used by ECS. Avoiding "Widget" is because I plan to use it for custom-drawn controls in future.

> In the future, we will also explore the possibility of using `Schema` as perfab for game scenes.

`Schema` essentially addresses all the problems mentioned above:

- `Schema` ensures that the function runs only once, meaning the views inside it will be builded only once.
- Default properties are optional, and it supports required properties.
- It supports slots, events, schema context, and more

Conventions for defining a `Schema`:

- Use the attribute macro `schema`.
- Function names should start with `schema_`
- The return type must be either `impl IntoView<Renderer>` or `impl IntoElementView<Renderer>` (the difference between `IntoElementView` and `IntoView` will be explained later)`)
- Valid parameter types define attributes, events, slots, etc. for the `Schema` throught the function parameters.

There are the following types of Props:

- `ReadSignal`: Signal
- `ReceiverProp`: Simple reactive prop, Currently not recommended for use because it is weaker and less user-friendly compared to signals.
- `Static`: Static prop that does not update

By default, props are optional, If you want them to be required, you can use `Required` to wrap the type (more on this later)

Event type: `Sender`, representing an event (actually a re-export of `async-channel`'s `Sender` type).

Slot types: `Slot`, a slot; `CloneableSlot`, a cloneable slot. Slots implement `IntoView` and can be directly placed in the view.

`SchemaCtx`: Context type. Currently, it can be used to:

- Obtain the `World`
- define shared styles or default shared styles outside the view.
- Use `use_controlled_state` to get al controlled state.

Below is an example code for custom `Checkbox`:

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
    let is_checked = ctx.use_controlled_state(value, onchange); // 获取一个受控的状态
    ctx.default_typed_style(CheckboxStyle, || { // 定义默认共享样式
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
        .bg_color(rx(move || is_checked.get().then_some(Color::BLUE)))
        .rx_member(move || {
            (!readonly.get()).then_some(().on_pointer_click(move || {
                is_checked.update(|is_checked| *is_checked = !*is_checked);
            }))
        })
}
```

>If you are using you custom type in prop,such as `ReadSignal<CustomType>`, you need to add the `PropValueWrapper` derive macro to `CustomType`. This restriction may be removed in the future.

In the xample above, `use_controlled_state` is used, which, by taking a signal and an event as inputs, returns a new signal.

When the input signal or the retured signal changes. it sends an event with the latest value as as parameter.

What does the `schema` attribute macro do? It doesn't modify the original function. Instead, it based on the function information, it generates a new function. The original function with with the `schema_` prefix removed is its name. Additionally,  it adds an implementation of a trait for the return type of this function (for details, you can check the code generated by the macro).

Code Example: 

```rust
fn sample_checkbox() -> impl IntoView<BevyRenderer> {
    checkbox()
        .padding(50)
        .style(x().border_color(Color::RED).border(2))
        .on_pointer_up(|| {
            println!("checkbox pointer up");
        })
        .value(false)
        .readonly(false)
        .onchange(|value| {
            println!("checkbox value: {}", value);
        })
}
```

The `checkbox` function is generated by `schema_checkbox`.

You can pass attributes or events like `padding`,`style`,`on_pointer_up`, etc. to the root element of `schema_checkbox`. These attributes or events will be forwarded to the root element.

In addition to these, there are attributes like `value`, `readonly`, `onchange`, etc. You can use them to set the corresponding props or events of `schema_checkbox`

Apart from accepting static values, you can also wrap values with reactive types like `Option`, `Reactive`, `ReadSignal`, etc.

> Unlike `ViewMember`, schema props currently cannot nest `Option` within reactive types like `Reactive`, `Memo`, etc.

### Differernce between IntoView and IntoElementView

In `IntoView`, the root element can have zero to many children, while in `IntoElementView`, it can have only one root element.

If the return type of a `Schema` is `IntoView`, external code cannot append members or envets to its root element because the root element may have multiple or zero children.

### Schema Slot

After defining a slot in `Schema`, you can use the `slot_<slot_name>` method to specify the content of the slot. It takes the type `IntoView`. If the slot is of type `CloneableSlot`, then the `IntoView` type it takes needs to implement `Clone`. Many views do not implement `Clone`, and in such cases, you can use `view_builder` to wrap it.

Code Example:

```rust
#[schema]
fn schema_sample(head: Slot, foot: Slot) -> impl IntoView<BevyRenderer> {
    div()
        .children((
            head,
            div().bg_color(Color::GRAY).p(20).children("Body"),
            foot,
        ))
        .p(20)
        .flex_col()
        .gap(8)
}

fn sample_schema_sample() -> impl IntoView<BevyRenderer> {
    sample()
        .slot_head(div().children("Head"))
        .slot_foot(div().children("Foot"))
}
```


### Schema Required

By default, props in `Schema` are optional. Therefore, the types of prop values, like `T` in `ReadSignal<T>`, must implement `Default`.

You can use `Required` to wrap prop such as `Static`,`ReadSignal`,`ReceiverProp`,`Slot`,`CloneableSlot`, etc. to indicate that they are required. In this case, the prop value `T` does not need to implement `Defeault`.

After marking an prop as required, the function generated by `Schema` will no longer be parameterless. Instead, you will need to pass the required parameters in order.

Code Example:

```rust
#[schema]
fn schema_required_sample(
    Required(content): Required<Slot>,
    Required(static_p): Required<Static<bool>>,
) -> impl IntoView<BevyRenderer> {
    div().children((content, "schema_required_sample"))
}

fn sample_schema_required_sample() -> impl IntoView<BevyRenderer> {
    required_sample("Content Slot", true)
}
```

### dynamic

Sometimes you need to return different types, but `impl` doesn't allow returning different types.

For two different types, you can use `Either<LeftView, RightView>` to wrap them to solve the problem.

For multiple different types, you can also use `Either<LeftView, Either<RightView, Either<...>>>` to solve this problem, but this approach can be cumbersome.

> Note: The type of `View` wraps `ViewMember`, meaning different `ViewMember` members will result in diferent `View` types

Code Example:

```rust
#[derive(Clone, Debug)]
enum SampleState {
    Init,
    Loading,
    Loaded,
}

fn sample_dynamic() -> impl IntoView<BevyRenderer> {
    let signal = use_rw_signal(SampleState::Init);
    (
        div()
            .children("Change State")
            .style((
                x().padding(20).flex().center().bg_color(Color::DARK_GRAY),
                x_hover().bg_color(Color::GRAY),
                x_active().outline_color(Color::GREEN).outline_width(2),
            ))
            .on_pointer_click(move || {
                signal.update(|state| match state {
                    SampleState::Init => *state = SampleState::Loading,
                    SampleState::Loading => *state = SampleState::Loaded,
                    SampleState::Loaded => *state = SampleState::Init,
                })
            }),
        rx(move || match signal.get() {
            SampleState::Init => span("Init").margin(30).into_dynamic(),
            SampleState::Loading => div()
                .padding(30)
                .bg_color(Color::RED)
                .children("Loading")
                .into_dynamic(),
            SampleState::Loaded => span("Loaded")
                .margin(30)
                .text_color(Color::BLUE)
                .into_dynamic(),
        }),
    )
}
```

### Bevy `system_once`

`system_once` will run the proveded `System` once when it is builded (if it's within an `rx`, then it will run each time the signal changes).

```rust
fn sample_system_once() -> impl IntoView<BevyRenderer> {
    system_once(|entities: &Entities, cmd_sender: Res<CmdSender>| {
        let cmd_sender = cmd_sender.clone();
        (
            div()
                .font_size(30)
                .children(format!("Entites Count: {}", entities.len())),
            div()
                .children("Spawn 2d Material")
                .style((
                    x().padding(20).flex().center().bg_color(Color::DARK_GRAY),
                    x_hover().bg_color(Color::GRAY),
                    x_active().outline_color(Color::GREEN).outline_width(2),
                ))
                .on_pointer_click(move || {
                    cmd_sender.add(|world: &mut World| {
                        let mut meshes = world.resource_mut::<Assets<Mesh>>();
                        let mesh = meshes.add(shape::Quad::default().into()).into();

                        let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
                        let material = materials.add(Color::PURPLE.into());

                        world.spawn(MaterialMesh2dBundle {
                            mesh,
                            transform: Transform::default().with_scale(Vec3::splat(128.)),
                            material,
                            ..default()
                        });
                    });
                }),
        )
    })
}
```

### Bevy `system`

By using `system`, you can treat a Bevy `System` as a view. You can use `configure` to configure the `System`

However, this method is currently unsafe because there is currently no means of removing a `System` In Bevy, So you have to make sure that this view is always there

```rust
fn sample_system() -> impl IntoView<BevyRenderer> {
    div().flex_col().gap(10).children(unsafe {
        system(Update, |query: Query<Entity, With<Style>>| {
            x_iter_keyed(query.iter().map(|entity| {
                Keyed(
                    entity,
                    span(format!("Style Entity: {:?}", entity)).margin(10),
                )
            }))
        })
        .configure(|config| config.run_if(|| true))
    })
}
```

### Bevy `x_res`

`x_res` retrieves a resource of type `T` and  rebuilds when the resource changes.

In the following code example, the view will be rebuild every time the `FrameCount` resource changes:

```rust
fn sample_x_res() -> impl IntoView<BevyRenderer> {
    div().gap(10).p(20).flex_col().children((
        x_res(|frame_count: &FrameCount| {
            span(format!("FrameCount: {:?}", frame_count.0))
                .p(20)
                .font_size(30)
        }),
    ))
}
```

### view_builder

`view_builder`, the view builder,accepts a callback function that takes two parameters: the first is `ViewCtx`, and the second is `BuildFlags`.

You can use `ViewCtx` within the callback to construct your view.

As mentioned earlier, `view_builder` also allow views that cannot be `Clone` to be placed inside it, The type returned by `view_builder` implements `Clone`.

```rust
fn sample_view_builder() -> impl IntoView<BevyRenderer> {
    view_builder(|ctx, flags| {
        let parent = ctx.parent;
        // let world = ctx.world;
        format!("Parent: {:?}, Flags: {:?}", parent, flags)
    })
}
```

### `provide_context`

You can provide context using `provide_context`, which supplies the context type to its child views.

Child views can obtain the context through the `context`-related methods of the first parameter, `ViewCtx`, in `view_builder`.

In `Schema`, you can directly obtain the context through the `Context` parameter.

Code Example：

```rust
#[derive(TypedStyle)]
struct BtnStyle;
#[derive(Clone)]
struct MyContext {
    signal: RwSignal<bool>,
}

#[schema]
fn schema_context_sample(Context(my_context): Context<MyContext>) -> impl IntoView<BevyRenderer> {
    div()
        .children("Set Signal To True")
        .style(BtnStyle)
        .on_pointer_click(move || {
            my_context.signal.set(true);
        })
}

fn sample_context() -> impl IntoView<BevyRenderer> {
    let signal = use_rw_signal(false);
    (
        BtnStyle::def((
            x().width(160)
                .py(8)
                .flex()
                .center()
                .bg_color(Color::DARK_GRAY),
            x_hover().bg_color(Color::GRAY),
            x_active().outline_color(Color::GREEN).outline_width(2),
        )),
        provide_context(
            MyContext { signal },
            div().p(20).flex().gap(10).children((
                view_builder(|ctx, _| {
                    let my_context = ctx.context::<MyContext>();
                    rx(move || format!("Signal: {}", my_context.signal.get()))
                }),
                x_if(signal, "Signal Is True"),
                div()
                    .children("Set Signal To False")
                    .style(BtnStyle)
                    .on_pointer_click(move || {
                        signal.set(false);
                    }),
                context_sample(),
            )),
        ),
    )
}
```

### Generic Schema

`schema` support function to use generic parameter

Code Example：

```rust
#[schema]
pub fn schema_select<T>(
    mut ctx: SchemaCtx,
    content: CloneableSlot,
    value: ReadSignal<T>,
    readonly: ReadSignal<bool>,
    onchange: Sender<T>,
) -> impl IntoElementView<BevyRenderer>
where
    T: Default + Debug + Send + Sync + PartialEq + Clone + 'static,
{
    ///...
}

fn sample_schema_select() -> impl IntoView<BevyRenderer> {
    enum SelectValue {
        A,
        B,
        C,
    }
    schema_select::<SelectValue>()
    // ...
}
```
