use std::any::TypeId;

pub use attr_iter::EntityStyleAttrInfoIterArgs;
pub(crate) use style_state_owner::StyleStateOwner;
pub use attr_syncer::EntityAttrSyncer;
pub use element_view_ext::*;
pub use entity_world_ref::*;
pub use interaction_style::interaction_to_style_interaction;
pub use plugin::{Previous, RxyStyleSheetPlugin};
use rxy_bevy_crate::BevyRenderer;
use rxy_core::style::{AppliedStyleSheet, StyleSheetCtx, StyleSheetsInfo};
pub use shared_style_sheets::SharedStyleState;
pub use shared_style_view::*;

pub use crate as rxy_bevy_crate;

mod attr_iter;
mod attr_syncer;
mod element_view_ext;
mod entity_world_ref;
mod focus_style;
mod interaction_style;
mod node_style_state;
mod node_tree;
mod plugin;
mod shared_style_sheets;
mod shared_style_view;
mod res_style_sheets;
mod style_state_owner;

pub type Result<T = ()> = rxy_core::style::Result<BevyRenderer, T>;
pub type StyleError = rxy_core::style::StyleError<BevyRenderer>;

pub mod prelude {
   pub use super::{
      typed_shared_style_sheets, RxyStyleSheetPlugin, SchemaCtxExt, StyleError,
      TypedStyleLabel,
   };
}

pub fn typed_shared_style_sheets(
   type_id: TypeId,
   ctx: StyleSheetCtx<BevyRenderer>,
) -> (
   impl Iterator<Item = AppliedStyleSheet<BevyRenderer>> + Send + 'static,
   StyleSheetsInfo,
) {
   let entity = ctx.world.get_typed_entity(type_id).unwrap();
   {
      let mut entity_world_mut = ctx.world.entity_mut(entity);
      let shared_style_sheets = entity_world_mut.get_shared_style_state().unwrap();
      shared_style_sheets.add_subscriber(ctx.node_id);
   }
   let mut entity_world_mut = ctx.world.entity_mut(entity);

   let style_sheets_state = entity_world_mut.get_style_sheets_state().unwrap();
   (
      style_sheets_state.apply_as_shared(entity, ctx.shared_style_sheet_index),
      style_sheets_state.style_sheets_info(),
   )
}
