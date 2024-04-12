#![allow(clippy::type_complexity)]

pub use cmd::*;
pub use command::*;
pub use entity_extra_data::*;
pub use focusable::*;
pub use plugin::*;
pub use renderer::*;
pub use res::*;
pub use res_change_observe::*;
use rxy_core::{
   CloneableSchemaSlot, FnSchema, IntoViewSchemaFnWrapper, RebuildFnReceiver, RenderSchemaCtx,
   RendererSchemaView, SchemaSlot,
};
pub use view::*;
pub use view_member::*;
pub use world_ext::*;

mod cmd;
mod command;
mod entity_extra_data;
pub mod event;
mod focusable;
pub mod navigation;
mod nest;
mod plugin;
mod renderer;
mod res;
mod res_change_observe;
pub mod vec_data_source;
mod view;
mod view_member;
mod world_ext;

pub type FnSchemaView<F, P = ()> =
   RendererSchemaView<BevyRenderer, FnSchema<IntoViewSchemaFnWrapper<F, BevyRenderer>, P>, (), ()>;

pub type SchemaCtx = RenderSchemaCtx<BevyRenderer>;

pub type ReceiverProp<T> = RebuildFnReceiver<BevyRenderer, T>;

pub type Slot = SchemaSlot<BevyRenderer>;
pub type CloneableSlot = CloneableSchemaSlot<BevyRenderer>;

pub mod all_attrs {
   pub use crate::attrs::*;
}

pub mod prelude {
   pub use bevy_ui::prelude::Val;

   pub use rxy_bevy_macro::{ElementSchema, Schema};

   pub use crate::elements::prelude::*;
   pub use crate::renderer::common_renderer::*;
   #[cfg(feature = "style")]
   pub use crate::renderer::style::ElementViewStyleExt;
   pub use crate::renderer::BevyElement;
   pub use crate::x_res_once;

   pub use super::all_attrs::{CommonAttrsElementViewBuilder, CommonAttrsViewBuilder};
   pub use super::renderer::event::*;
   pub use super::renderer::view_builder_ext::*;
   #[cfg(feature = "tailwind_aliases")]
   pub use super::renderer::{ElementViewTailwindAttrs, MemberOwnerTailwindAttrs};
   #[cfg(feature = "style")]
   pub use super::style::prelude::StyleError;
   #[cfg(feature = "style")]
   pub use super::style::prelude::*;
   pub use super::{
      event::*, system_once, x_res, BevyRenderer, CloneableSlot, CmdReceiver, CmdSender,
      FnSchemaView, Focusable, ReceiverProp, ResChangeWorldExt, RxyPlugin, RxyViewSpawner,
      SchemaCtx, Slot,
   };
   pub use super::{ElementViewCompositeAttrs, MemberOwnerCompositeAttrs};
}
