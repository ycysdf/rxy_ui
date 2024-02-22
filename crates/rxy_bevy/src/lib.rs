#![allow(clippy::type_complexity)]

pub use cmd::*;
pub use command::*;
pub use entity_extra_data::*;
pub use focusable::*;
pub use plugin::*;
pub use renderer::*;
pub use res_change_observe::*;
use rxy_core::{
    CloneableSchemaSlot, FnSchema, IntoViewSchemaFnWrapper, RebuildFnReceiver, RenderSchemaCtx,
    SchemaSlot, SchemaView,
};
pub use view::*;
pub use view_member::*;
pub use world_ext::*;

mod cmd;
mod command;
mod entity_extra_data;
mod event;
mod focusable;
pub mod navigation;
mod nest;
mod plugin;
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
    pub use crate::elements::attrs::*;
}

pub use crate::attrs::element_view_builder;

pub mod prelude {
    pub use bevy_ui::prelude::Val;

    pub use crate::attrs::element_view_builder::*;
    pub use crate::renderer::common_renderer::*;
    pub use crate::renderer::BevyElement;

    pub use super::all_attrs::{
        CommonAttrsElementViewBuilder, CommonAttrsViewBuilder, ElementSpanAttrsElementViewBuilder,
        ElementSpanAttrsViewBuilder,
    };
    #[cfg(feature = "tailwind_aliases")]
    pub use super::renderer::{ElementViewTailwindAttrs, MemberOwnerTailwindAttrs};
    pub use super::{
        event::*, system_once, x_res, BevyRenderer, CloneableSlot, CmdReceiver, CmdSender,
        FnSchemaView, Focusable, ReceiverProp,
        ResChangeWorldExt, RxyPlugin, RxyUiCommandExt, SchemaCtx, Slot,
    };
    pub use super::renderer::view_builder_ext::*;
    pub use super::renderer::event::*;
    pub use super::{ElementViewCompositeAttrs, MemberOwnerCompositeAttrs};

    #[cfg(feature = "style")]
    pub use super::style::prelude::StyleError;
    #[cfg(feature = "style")]
    pub use super::style::prelude::*;
}
