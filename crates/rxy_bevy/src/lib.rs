#![allow(clippy::type_complexity)]

pub use cmd::*;
pub use command::*;
pub use element::*;
pub use focusable::*;
pub use plugin::*;
pub use renderer::*;
pub use res_change_observe::*;
pub use rxy_bevy_element::all_attrs;
pub use rxy_bevy_element::elements;
pub use rxy_bevy_element::ElementType;
use rxy_core::{CloneableSchemaSlot, FnSchema, IntoViewSchemaFnWrapper, RebuildFnReceiver, RenderSchemaCtx, SchemaSlot, SchemaView};
pub use view::*;
pub use view_member::*;

mod cmd;
mod command;
mod element;
mod focusable;
pub mod navigation;
mod plugin;
#[cfg(feature = "xy_reactive")]
mod reactive;
mod renderer;
mod res_change_observe;
mod view;
mod view_member;

pub type FnSchemaView<F, P = ()> =
SchemaView<BevyRenderer, FnSchema<IntoViewSchemaFnWrapper<F, BevyRenderer>, P>, (), ()>;

pub type SchemaCtx = RenderSchemaCtx<BevyRenderer>;

pub type ReceiverProp<T> = RebuildFnReceiver<BevyRenderer, T>;

pub type Slot = SchemaSlot<BevyRenderer>;
pub type CloneableSlot = CloneableSchemaSlot<BevyRenderer>;

pub mod prelude {
    pub use bevy_ui::prelude::Val;

    pub use super::{button, div, span};
    pub use super::{
        BevyElement, BevyRenderer, BevyWrapper, CloneableSlot, CmdReceiver, CmdSender,
        CommonAttrsViewBuilder, CompositeAttrs, element::event::*, ElementType, FnSchemaView,
        Focusable, MemberOwnerBundleExt, ReceiverProp, ResChangeWorldExt, RxyPlugin,
        RxyUiCommandExt, SchemaCtx, Slot, system_once, x_res,
    };
}
