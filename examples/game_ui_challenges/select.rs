use core::fmt::Display;
use std::fmt::Debug;

use bevy::prelude::*;
use bevy::render::color::Color;
use rxy_bevy::FocusedEntity;
use rxy_bevy::RendererState;
use rxy_core::utils::SyncCell;
use rxy_core::{fn_schema_view, ElementAttr, NodeTree, RendererNodeId, SchemaElementView, XNest};
use rxy_ui::prelude::*;

use crate::FocusStyle;
use crate::XConfirm;

#[derive(TypedStyle)]
pub struct SelectStyle;

#[derive(TypedStyle)]
pub struct SelectSelectionListStyle;

#[derive(ElementSchema)]
pub struct Select<T>
where
    T: Default + Debug + Display + Send + Sync + PartialEq + Clone + 'static,
{
    ctx: SchemaCtx,
    content: CloneableSlot,
    value: ReadSignal<T>,
    readonly: ReadSignal<bool>,
    onchange: Sender<T>,
}

impl<T> SchemaElementView<BevyRenderer> for Select<T>
where
    T: Default + Debug + Display + Send + Sync + PartialEq + Clone + 'static,
{
    fn view(self) -> impl IntoElementView<BevyRenderer> {
        let Select {
            content,
            value,
            mut ctx,
            onchange,
            readonly,
        } = self;
        let value = ctx.use_controlled_state(value, onchange);
        let is_open = use_rw_signal(false);

        ctx.default_typed_style(SelectStyle, || {
            (
                x().flex()
                    .border(1)
                    .border_color(Color::WHITE)
                    .center()
                    .relative()
                    .py(8)
                    .min_w(150),
                x_hover().bg_color(Color::DARK_GRAY),
                FocusStyle,
            )
        });
        ctx.default_typed_style(SelectSelectionListStyle, || {
            x().absolute()
                .z(1)
                .top(Val::Percent(100.))
                .bg_color(Color::GRAY)
                .w_full()
        });
        let (id_sender, id_receiver) = oneshot::channel();

        button()
            .name("select")
            .style(SelectStyle)
            .children((
                rx(move || {
                    format!("{}", value.get())
                        .into_view()
                        .text_color(Color::WHITE)
                }),
                selection_list::<T>()
                    .style(SelectSelectionListStyle)
                    .slot_content(content)
                    .visibility(is_open)
                    .value(value)
                    .onchange(move |new_value: T| {
                        is_open.set(false);
                        value.set(new_value);
                    })
                    .on_build(|member_ctx: ViewMemberCtx<BevyRenderer>, _| {
                        let _ = id_sender.send(member_ctx.node_id);
                    }),
            ))
            .on_build_after_children(move |ctx: ViewMemberCtx<BevyRenderer>, _| {
                let select_entity = ctx.node_id;
                let selection_list_entity = id_receiver.try_recv().unwrap();
                rx(move || {
                    (!readonly.get()).then_some(().on(
                        XConfirm,
                        move |query: Query<&RendererState<Context<SelectionListContext<T>>>>,
                              cmd_sender: Res<CmdSender>| {
                            is_open.update(|is_open| *is_open = !*is_open);
                            let selection_list_ctx =
                                &query.get(selection_list_entity).unwrap().0 .0;
                            if let Some(selected_entity) = selection_list_ctx.selected_entity {
                                cmd_sender.add(move |world: &mut World| {
                                    let mut focused_entity = world.resource_mut::<FocusedEntity>();
                                    focused_entity.0 = Some(selected_entity);
                                })
                            }
                        },
                    ))
                })
            })
    }
}

#[derive(Clone)]
pub struct SelectionListContext<T: Send + Sync + 'static> {
    value_signal: RwSignal<T>,
    selected_entity: Option<RendererNodeId<BevyRenderer>>,
}

#[derive(ElementSchema)]
pub struct SelectionList<T: Default + Debug + Send + Sync + PartialEq + Clone + 'static> {
    ctx: SchemaCtx,
    content: Slot,
    value: ReadSignal<T>,
    readonly: ReadSignal<bool>,
    onchange: Sender<T>,
}

impl<T> SchemaElementView<BevyRenderer> for SelectionList<T>
where
    T: Default + Debug + Send + Sync + PartialEq + Clone + 'static,
{
    fn view(self) -> impl IntoElementView<BevyRenderer> {
        let SelectionList {
            mut ctx,
            value,
            onchange,
            content,
            ..
        } = self;
        let value_signal = ctx.use_controlled_state(value, onchange);
        provide_context(
            SelectionListContext {
                value_signal,
                selected_entity: None,
            },
            div().style(x().flex_col().py(4)).children(content),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectionItem<T> {
    pub value: T,
    pub is_selected: bool,
}

pub fn selection_item<T, V>(
    value: T,
    f: impl Fn(SelectionItem<T>) -> V + Send + 'static,
) -> FnSchemaView<impl SchemaIntoViewFn<BevyRenderer>>
where
    T: Default + Send + Sync + PartialEq + Clone + 'static,
    V: IntoElementView<BevyRenderer> + Send,
{
    fn_schema_view(move || {
        view_builder(|ctx, _| {
            let parent = ctx.parent;
            let selection_list = ctx.context::<SelectionListContext<T>>();
            let is_selected = use_memo({
                let value = value.clone();
                let value_signal = selection_list.value_signal;
                move |_| value_signal.get() == value
            });

            rx(move || {
                let element_view = f(SelectionItem {
                    value: value.clone(),
                    is_selected: is_selected.get(),
                })
                .into_element_view();
                into_view(
                    element_view
                        .rx_member(move || {
                            is_selected.get().then_some(member_builder(
                                move |member_ctx: ViewMemberCtx<BevyRenderer>, _| {
                                    if let Some(selection_list) = member_ctx
                                        .world
                                        .get_node_state_mut::<Context<SelectionListContext<T>>>(
                                            &parent,
                                        )
                                    {
                                        selection_list.0.selected_entity = Some(member_ctx.node_id);
                                        println!("selected entity: {:?}", member_ctx.node_id);
                                    }
                                },
                            ))
                        })
                        .on(XConfirm, {
                            let value_signal = selection_list.value_signal;
                            let value = value.clone();
                            move |cmd_sender: Res<CmdSender>| {
                                value_signal.set(value.clone());
                                cmd_sender.add(move |world: &mut World| {
                                    let select_entity = world.get::<Parent>(parent).unwrap().get();
                                    let mut focused_entity = world.resource_mut::<FocusedEntity>();
                                    focused_entity.0 = Some(select_entity);
                                })
                            }
                        }),
                )
            })
        })
    })
}
