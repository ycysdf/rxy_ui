#![allow(clippy::clone_on_copy)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
extern crate alloc;

mod attr_iter;
mod attr_syncer;
mod element_view_ext;
mod entity_world_ref;
mod focus_style;
mod interaction_style;
mod node_style_state;
mod plugin;
mod shared_style_sheets;
mod shared_style_view;
mod style_sheets;
#[cfg(feature = "tailwind_aliases")]
mod tailwind_attrs;
// mod view_member;

#[cfg(feature = "tailwind_aliases")]
pub use crate::tailwind_attrs::TailwindAttrs;
use std::any::TypeId;

pub use crate::attr_iter::EntityStyleAttrInfoIterArgs;
pub(crate) use crate::attr_iter::StateOwner;
pub use crate::element_view_ext::ElementStyleExt;
pub use crate::interaction_style::interaction_to_style_interaction;
pub use crate::plugin::{Previous, RxyStyleSheetPlugin};
pub use crate::style_sheets::res;
pub use attr_syncer::EntityAttrSyncer;
pub use entity_world_ref::*;
use rxy_bevy::BevyRenderer;
use rxy_core::style::{AppliedStyleSheet, StyleSheetCtx, StyleSheetsInfo};
pub use shared_style_sheets::SharedStyleState;
pub use shared_style_view::*;

// pub use view_member::{ApplyStyleSheets, ApplyStyleSheetsMemberState};

pub type Result<T = ()> = rxy_core::style::Result<BevyRenderer, T>;
pub type StyleError = rxy_core::style::StyleError<BevyRenderer>;
pub use rxy_style_macro;

pub mod prelude {
    #[cfg(feature = "tailwind_aliases")]
    pub use super::TailwindAttrs;
    pub use super::{
        res, rxy_style_macro::TypedStyle, typed_shared_style_sheets, ElementStyleExt,
        RxyStyleSheetPlugin, SchemaCtxExt, StyleError, TypedStyleLabel,
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
