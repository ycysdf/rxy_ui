use core::fmt::Display;
use std::fmt::Debug;

use bevy::prelude::*;
use bevy::render::color::Color;
use rxy_core::{ElementAttrViewMember, fn_schema_view, XNest};

use rxy_ui::prelude::*;

use crate::FocusStyle;
use crate::XConfirm;

#[derive(TypedStyle)]
pub struct SelectStyle;

#[derive(TypedStyle)]
pub struct SelectSelectionListStyle;

#[schema]
pub fn schema_select<T>(
    mut ctx: SchemaCtx,
    content: CloneableSlot,
    value: ReadSignal<T>,
    readonly: ReadSignal<bool>,
    onchange: Sender<T>,
) -> impl IntoElementView<BevyRenderer>
where
    T: Default + Debug + Display + Send + Sync + PartialEq + Clone + 'static,
{
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

    button()
        .name("select")
        .style(SelectStyle)
        .rx_member(move || {
            (!readonly.get()).then_some(().on(XConfirm, move || {
                is_open.update(|is_open| *is_open = !*is_open);
            }))
        })
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
                }),
        ))
}

#[derive(Clone)]
pub struct SelectionListContext<T: Send + Sync + 'static> {
    value_signal: RwSignal<T>,
}

#[schema]
pub fn schema_selection_list<T: Default + Debug + Send + Sync + PartialEq + Clone + 'static>(
    mut ctx: SchemaCtx,
    content: Slot,
    value: ReadSignal<T>,
    readonly: ReadSignal<T>,
    onchange: Sender<T>,
) -> impl IntoElementView<BevyRenderer> {
    let value_signal = ctx.use_controlled_state(value, onchange);
    provide_context(
        SelectionListContext { value_signal },
        div().style(x().flex_col().py(4)).children(content),
    )
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
            let ctx = ctx.context::<SelectionListContext<T>>();
            let is_selected = use_memo({
                let value = value.clone();
                let value_signal = ctx.value_signal;
                move |_| value_signal.get() == value
            });

            rx(move || {
                add_members(
                    f(SelectionItem {
                        value: value.clone(),
                        is_selected: is_selected.get(),
                    }),
                    ().on(XConfirm, {
                        let value_signal = ctx.value_signal;
                        let value = value.clone();
                        move || {
                            value_signal.set(value.clone());
                        }
                    }),
                )
            })
        })
    })
}
