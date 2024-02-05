#![allow(clippy::type_complexity)]

pub use cmd::*;
pub use command::*;
pub use element::*;
pub use entity_extra_data::*;
pub use focusable::*;
pub use plugin::*;
pub use renderer::*;
pub use res_change_observe::*;
// pub use rxy_bevy_element::all_attrs;
// pub use rxy_bevy_element::elements;
// pub use rxy_bevy_element::ElementType;
use rxy_core::{
    CloneableSchemaSlot, FnSchema, IntoViewSchemaFnWrapper, RebuildFnReceiver, RenderSchemaCtx,
    SchemaSlot, SchemaView,
};
pub use view::*;
pub use view_member::*;
pub use world_ext::*;

mod cmd;
mod command;
mod element;
mod entity_extra_data;
mod event;
mod focusable;
pub mod navigation;
mod plugin;
#[cfg(feature = "xy_reactive")]
mod reactive;
mod renderer;
mod res_change_observe;
mod view;
mod view_member;
mod world_ext;

pub type FnSchemaView<F, P = ()> =
    SchemaView<BevyRenderer, FnSchema<IntoViewSchemaFnWrapper<F, BevyRenderer>, P>, (), ()>;

pub type SchemaCtx = RenderSchemaCtx<BevyRenderer>;

pub type ReceiverProp<T> = RebuildFnReceiver<BevyRenderer, T>;

pub type Slot = SchemaSlot<BevyRenderer>;
pub type CloneableSlot = CloneableSchemaSlot<BevyRenderer>;

pub mod all_attrs {
    pub use crate::attrs::*;
    // pub use crate::elements::input_attrs::*;
    pub use crate::elements::span_attrs::*;
}

pub mod prelude {
    pub use bevy_ui::prelude::Val;

    pub use crate::renderer::{button, div, span, BevyElement};

    pub use super::{
        event::*, system_once, x_res, BevyRenderer, BevyWrapper, CloneableSlot, CmdReceiver,
        CmdSender, ElementKeyboardEvents, FnSchemaView, Focusable, MemberOwnerBundleExt,
        ReceiverProp, ResChangeWorldExt, RxyPlugin, RxyUiCommandExt, SchemaCtx, Slot,
    };
}
