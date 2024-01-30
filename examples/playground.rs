#![allow(unused_variables)]

use bevy::core::FrameCount;
use bevy::ecs::{entity::Entities, world};
use bevy::prelude::*;
use rxy_ui::{
    bevy::{system, system_once},
    prelude::*,
};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::synccell::SyncCell;
use futures_lite::{FutureExt, StreamExt};
use rxy_bevy::x_bundle;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        RxyPlugin::default(),
        RxyStyleSheetPlugin::default(),
        // WorldInspectorPlugin::new(),
    ))
    .add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn_rxy_ui(sample_bundle);
}

#[schema]
fn schema_tt(_ctx: SchemaCtx, value: ReadSignal<bool>) -> impl IntoView<BevyRenderer> {
    (checkbox(), rx(move || value.get().to_string()))
}

fn sample_bug() -> impl IntoView<BevyRenderer> {
    tt()
}

fn signal_sample() -> impl IntoView<BevyRenderer> {
    let count = use_rw_signal(0);
    div()
        .children(rx(move || count.get().to_string()))
        .padding(20)
        .bg_color(Color::BLUE)
        .border_y(count)
        .border_color(Color::RED)
        .width(100)
        .height(rx(move || count.get() * 4 + 100))
        .on_pointer_click(move || {
            count.update(|x| *x += 1);
        })
}

fn btn(is_show: RwSignal<bool>) -> impl IntoView<BevyRenderer> {
    div().children("Btn").padding(20).bg_color(Color::GRAY).on_pointer_click(move || {
        is_show.update(|x| *x = !*x);
    })
}

fn sample_option_view() -> impl IntoView<BevyRenderer> {
    let is_show = use_rw_signal(false);
    (
        btn(is_show),
        rx(move || {
            let view = div().padding(10).bg_color(Color::BLUE).flex().center().children("Show");
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
        div().padding(10).flex().center().children("Show").member(rx(move || {
            is_show
                .get()
                .then_some(().bg_color(Color::BLUE).outline_color(Color::RED).outline_width(2))
        })), // 下面写法与上面写法是等价的
             // .rx_member(move || {
             //     is_show.get().then_some(
             //         ().bg_color(Color::BLUE)
             //             .outline_color(Color::RED)
             //             .outline_width(2),
             //     )
             // }),
    )
}

fn sample_either_view() -> impl IntoView<BevyRenderer> {
    let is_show = use_rw_signal(false);
    (
        btn(is_show),
        rx(move || {
            let left_view = "Left";
            let right_view = "Right";
            if is_show.get() {
                left_view.either_left() // 这等同于 Either::Left(left_view)
            } else {
                right_view.either_right() // 这等同于 Either::Right(right_view)
            }
        }),
    )
}

fn sample_either_view_member() -> impl IntoView<BevyRenderer> {
    let is_show = use_rw_signal(false);

    (
        btn(is_show),
        div().padding(10).flex().center().children("Show").member(rx(move || {
            let left_vm = ().bg_color(Color::BLUE);
            let right_vm = ().outline_color(Color::RED).outline_width(2);
            if is_show.get() {
                left_vm.either_left() // 这等同于 Either::Left(left_view)
            } else {
                right_vm.either_right() // 这等同于 Either::Right(right_view)
            }
        })),
    )
}

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
        // 非 Boxed Future 使用 x_future
        x_future(async {
            let r = receiver.await;
            div().padding(20).children(format!("Future: {:?}", r))
        }),
        // Boxed Future 可以直接作为 View
        async move {
            let color = receiver2.recv().await;
            div().padding(20).children(format!("Color: {:?}", color))
        }
        .boxed(),
    )
}

fn sample_x_if() -> impl IntoView<BevyRenderer> {
    let is_show = use_rw_signal(false);
    (
        div().children("Btn").padding(20).bg_color(Color::GRAY).on_pointer_click(move || {
            is_show.update(|x| *x = !*x);
        }),
        x_if(is_show, span("Show1").padding(10).text_color(Color::RED)),
        x_if(
            is_show,
            view_builder(|_, _| div().padding(10).flex().center().children("Show2")),
        ),
    )
}

fn sample_x_iter() -> impl IntoView<BevyRenderer> {
    div().flex_col().gap(8).children(x_iter((0..10).map(|n| format!("Item: {}", n))))
}

fn sample_x_iter_keyed() -> impl IntoView<BevyRenderer> {
    div().flex_col().gap(1).children(x_iter_keyed((0..25).map(|n| {
        Keyed(
            n,
            span(format!("Item: {}", n)).padding(10).text_color(Color::rgb_u8(n * 10, 255, 255)),
        )
    })))
}

fn sample_x_iter_keyed_rx() -> impl IntoView<BevyRenderer> {
    let signal = use_rw_signal(3);
    (
        div().padding(10).bg_color(Color::BLUE).children("Add").on_pointer_click(move || {
            signal.update(|x| *x += 1);
        }),
        rx(move || {
            div().flex_col().gap(1).children(x_iter_keyed((0..signal.get()).map(|n| {
                Keyed(
                    n,
                    span(format!("Item: {}", n)).padding(10).text_color(Color::rgb_u8(
                        n * 2,
                        255,
                        255,
                    )),
                )
            })))
        }),
    )
}

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

fn sample_dynamic_style_sheet() -> impl IntoView<BevyRenderer> {
    let signal = use_rw_signal(false);
    div()
        .on_pointer_click(move || {
            signal.update(|n| *n = !*n);
        })
        .style(Some((x().bg_color(Color::GRAY).height(100.).width(100.),)))
        .style_rx(move || {
            signal.get().then_some((x().bg_color(Color::RED), x_hover().bg_color(Color::WHITE)))
        })
}

fn sample_shared_typed_style_sheet() -> impl IntoView<BevyRenderer> {
    #[derive(TypedStyle)]
    struct MenuBtnStyle;
    (
        MenuBtnStyle::def((
            x().width(160).py(8).flex().center().bg_color(Color::DARK_GRAY),
            x_hover().bg_color(Color::GRAY),
            x_active().outline_color(Color::GREEN).outline_width(2),
        )),
        div().padding(50).gap(10).flex_col().children((
            div().style(MenuBtnStyle).children("Button 1"),
            div().style((MenuBtnStyle, x().bg_color(Color::RED))).children("Button 2"),
            div().style(MenuBtnStyle).children("Button 3"),
        )),
    )
}

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
            x().center().size(size).border(1).border_color(Color::DARK_GRAY),
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

fn sample_checkbox() -> impl IntoView<BevyRenderer> {
    let signal = use_rw_signal(false);
    (checkbox()
        .padding(50)
        .style(x().border_color(Color::RED).border(2))
        .on_pointer_up(|| {
            println!("checkbox pointer up");
        })
        .value(Some(rx(move || signal.get())))
        .readonly(false)
        .onchange(|value| {
            println!("checkbox value: {}", value);
        }),)
}

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
    sample().slot_head(div().children("Head")).slot_foot(div().children("Foot"))
}

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
            SampleState::Loading => {
                div().padding(30).bg_color(Color::RED).children("Loading").into_dynamic()
            }
            SampleState::Loaded => span("Loaded").margin(30).text_color(Color::BLUE).into_dynamic(),
        }),
    )
}

fn sample_system_once() -> impl IntoView<BevyRenderer> {
    system_once(|entities: &Entities, cmd_sender: Res<CmdSender>| {
        let cmd_sender = cmd_sender.clone();
        (
            div().font_size(30).children(format!("Entites Count: {}", entities.len())),
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

fn sample_x_res() -> impl IntoView<BevyRenderer> {
    div().gap(10).p(20).flex_col().children((
        // x_res(|time: &Time| span(format!("Time: {:.1}", time.elapsed_seconds())).p(20).font_size(30)),
        x_res(|frame_count: &FrameCount| {
            span(format!("FrameCount: {:?}", frame_count.0)).p(20).font_size(30)
        }),
        div().p(30).member(x_res(|time: &Time| {
            let t = time.elapsed_seconds();
            ().bg_color(Color::rgb(t.sin(), t.cos(), t.sin() * t.cos()))
        })),
    ))
}

fn sample_view_builder() -> impl IntoView<BevyRenderer> {
    view_builder(|ctx, flags| {
        let parent = ctx.parent;
        format!("Parent: {:?}, Flags: {:?}", parent, flags)
    })
}

#[derive(TypedStyle)]
struct BtnStyle;
#[derive(Clone)]
struct MyContext {
    signal: RwSignal<bool>,
}

#[schema]
fn schema_context_sample(Context(my_context): Context<MyContext>) -> impl IntoView<BevyRenderer> {
    div().children("Set Signal To True").style(BtnStyle).on_pointer_click(move || {
        my_context.signal.set(true);
    })
}

fn sample_context() -> impl IntoView<BevyRenderer> {
    let signal = use_rw_signal(false);
    (
        BtnStyle::def((
            x().width(160).py(8).flex().center().bg_color(Color::DARK_GRAY),
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
                div().children("Set Signal To False").style(BtnStyle).on_pointer_click(move || {
                    signal.set(false);
                }),
                context_sample(),
            )),
        ),
    )
}

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

fn sample_bundle() -> impl IntoView<BevyRenderer> {
    let signal = use_rw_signal(false);

    #[derive(Component)]
    struct CustomComponent;
    #[derive(Component)]
    struct CustomComponent2(bool);
    #[derive(Component)]
    struct CustomComponent3;
    div()
        .bundle(CustomComponent3) // 等同于：.member(x_bundle(CustomComponent))
        .rx_member(move || signal.get().then_some(x_bundle(CustomComponent)))
        .rx_member(move || x_bundle(CustomComponent2(signal.get())))
}
