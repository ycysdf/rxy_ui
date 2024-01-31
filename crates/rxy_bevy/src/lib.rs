#![allow(clippy::type_complexity)]

pub use cmd::*;
pub use command::*;
pub use element::*;
pub use plugin::*;
pub use renderer::*;
pub use res_change_observe::*;
pub use rxy_bevy_element::all_attrs;
pub use rxy_bevy_element::elements;
pub use rxy_bevy_element::ElementType;
use rxy_core::{CloneableSchemaSlot, RebuildFnReceiver, RenderSchemaCtx, SchemaSlot};
pub use view::*;
pub use view_member::*;
mod cmd;
mod command;
mod element;
mod plugin;
#[cfg(feature = "xy_reactive")]
mod reactive;
mod renderer;
mod res_change_observe;
mod view;
mod view_member;
pub mod wrapper;
pub mod navigation;

pub use wrapper::{pl_schema_view, FnSchemaView, SchemaIntoViewFn};

pub type SchemaCtx = RenderSchemaCtx<BevyRenderer>;

pub type ReceiverProp<T> = RebuildFnReceiver<BevyRenderer, T>;

pub type Slot = SchemaSlot<BevyRenderer>;
pub type CloneableSlot = CloneableSchemaSlot<BevyRenderer>;

pub mod prelude {
    pub use super::{
        div, element::event::*, pl_schema_view, span, system_once, x_res, BevyElement,
        BevyRenderer, BevyWrapper, CloneableSlot, CmdReceiver, CmdSender, CommonAttrsViewBuilder,
        CompositeAttrs, ElementType, FnSchemaView, MemberOwnerBundleExt, MemberOwnerFocusExt,
        ReceiverProp, ResChangeWorldExt, RxyPlugin, RxyUiCommandExt, SchemaCtx, SchemaIntoViewFn,
        Slot,
    };
    pub use bevy_ui::prelude::Val;
}
