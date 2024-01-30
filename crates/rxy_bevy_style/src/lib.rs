#![allow(clippy::clone_on_copy)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
extern crate alloc;

mod attr_iter;
mod attr_style_owner;
mod attr_syncer;
mod element_view_ext;
mod entity_world_ref;
mod into_view_member;
mod node_style_state;
mod plugin;
mod shared_style_sheets;
mod shared_style_view;
mod style_sheet_definition;
mod style_sheet_items;
mod style_sheets;
#[cfg(feature = "tailwind_aliases")]
mod tailwind_attrs;
mod view_member;

#[cfg(feature = "tailwind_aliases")]
pub use crate::tailwind_attrs::TailwindAttrs;

pub use crate::attr_iter::EntityStyleAttrInfoIterArgs;
pub(crate) use crate::attr_iter::StateOwner;
pub use crate::element_view_ext::ElementStyleExt;
pub use crate::into_view_member::IntoViewMemberWithOrigin;
pub use crate::plugin::{
    interaction_to_style_kind, AppliedStyleSheet, Previous, RxyStyleSheetPlugin, StyleItemValue,
    StyleSheetsInfo,
};
pub use crate::style_sheet_items::StyleSheetItems;
pub use crate::style_sheets::{res, typed_shared_style_sheets, StyleSheets};
pub use attr_style_owner::{AttrStyleOwner, NodeStyleStateRefVariant};
pub use attr_syncer::{EntityAttrSyncer, SyncerWrapper};
pub use entity_world_ref::*;
pub use node_style_state::NodeStyleState;
use rxy_bevy::BevyRenderer;
pub use shared_style_sheets::SharedStyleState;
pub use shared_style_view::*;
pub use style_sheet_definition::StyleSheetDefinition;

pub use view_member::{ApplyStyleSheets, ApplyStyleSheetsMemberState};

pub type Result<T = ()> = rxy_style::Result<BevyRenderer, T>;
pub type StyleError = rxy_style::StyleError<BevyRenderer>;
pub use rxy_style_macro;

pub mod prelude {
    pub use super::{
        res, rxy_style_macro::TypedStyle, ElementStyleExt, RxyStyleSheetPlugin, SchemaCtxExt,
        StyleError, StyleSheets, TailwindAttrs, TypedStyleLabel,
    };
}
