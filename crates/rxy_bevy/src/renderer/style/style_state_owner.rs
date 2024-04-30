use bevy_ecs::prelude::{Entity, Query};
use bevy_ecs::query::QueryFilter;
use bevy_ecs::world::World;

use rxy_core::AttrIndex;
use rxy_core::style::{
   NodeStyleItemId,
   NodeStyleSheetId, StyleItemValue, StyleSheetDefinition,
   StyleSheetLocation,
};

use super::{StyleEntityRefExt, StyleError};
use super::{EntityWorldRef, Result};
use super::node_style_state::NodeStyleSheetsState;
use super::rxy_bevy_crate::RendererState;

pub(crate) trait StyleStateOwner<'a, 's>: Sized {
   fn get_style_sheets_state(&'s self, entity: Entity) -> Result<&'a NodeStyleSheetsState>;

   fn get_style_item_attr_id(
      &'s self,
      entity: Entity,
      style_item_id: impl Into<NodeStyleItemId>,
   ) -> Result<AttrIndex> {
      let style_item_id: NodeStyleItemId = style_item_id.into();
      self
         .get_style_sheet_definition(entity, style_item_id)
         .and_then(|n| {
            n.items
               .get(style_item_id.item_index as usize)
               .ok_or(StyleError::NoFoundStyleItemId {
                  item_id: style_item_id,
               })
               .map(|n| n.attr_id)
         })
   }

   #[inline]
   fn get_style_item_value(
      &'s self,
      entity: Entity,
      style_item_id: impl Into<NodeStyleItemId>,
   ) -> Result<&'a StyleItemValue> {
      let style_item_id: NodeStyleItemId = style_item_id.into();
      self
         .get_style_sheet_definition(entity, style_item_id)
         .and_then(|n| {
            n.items
               .get(style_item_id.item_index as usize)
               .ok_or(StyleError::NoFoundStyleItemId {
                  item_id: style_item_id,
               })
         })
   }

   fn get_style_sheet_definition(
      &'s self,
      entity: Entity,
      style_sheet_id: impl Into<NodeStyleSheetId>,
   ) -> Result<&'a StyleSheetDefinition> {
      let style_sheet_id: NodeStyleSheetId = style_sheet_id.into();
      let style_sheets_state = self.get_style_sheets_state(entity)?;
      match style_sheet_id.location {
         StyleSheetLocation::Inline => {
            style_sheets_state.get_inline_style_sheet(style_sheet_id.index)
         }
         StyleSheetLocation::Shared => {
            let style_sheet_id =
               style_sheets_state.get_shared_style_sheet_id(style_sheet_id.index)?;

            let node_id = style_sheet_id.node_id;
            self.get_style_sheet_definition(node_id, style_sheet_id)
         }
      }
   }
}

pub(crate) trait NodeStyleStateOwner<'a, 's>: StyleStateOwner<'a, 's> {
   fn get_current_entity(&'s self) -> Entity;

   #[inline]
   fn get_current_style_item_value(
      &'s self,
      style_item_id: impl Into<NodeStyleItemId>,
   ) -> Result<&'a StyleItemValue> {
      self.get_style_item_value(self.get_current_entity(), style_item_id)
   }

   fn get_current_style_sheet_definition(
      &'s self,
      style_sheet_id: impl Into<NodeStyleSheetId>,
   ) -> Result<&'a StyleSheetDefinition> {
      self.get_style_sheet_definition(self.get_current_entity(), style_sheet_id)
   }
}

impl<'a> StyleStateOwner<'a, '_> for &'a World {
   fn get_style_sheets_state(&self, entity: Entity) -> Result<&'a NodeStyleSheetsState> {
      self
         .get_entity(entity)
         .ok_or(StyleError::NoFoundNode { node_id: entity })?
         .get_style_sheets_state()
   }
}

impl<'a> StyleStateOwner<'a, '_> for EntityWorldRef<'a> {
   fn get_style_sheets_state(&self, entity: Entity) -> Result<&'a NodeStyleSheetsState> {
      self
         .world
         .get_entity(entity)
         .ok_or(StyleError::NoFoundNode { node_id: entity })?
         .get_style_sheets_state()
   }
}

impl<'a> NodeStyleStateOwner<'a, '_> for EntityWorldRef<'a> {
   fn get_current_entity(&self) -> Entity {
      self.entity_ref.id()
   }
}

pub struct EntityStyleWorldQuery<'a, 'world, 'state, F: QueryFilter> {
   pub query: Query<'world, 'state, &'a RendererState<NodeStyleSheetsState>, F>,
   pub current_entity: Entity,
}

impl<'a, 'world, 'state, F: QueryFilter> StyleStateOwner<'a, 'a>
   for EntityStyleWorldQuery<'world, 'state, 'a, F>
{
   fn get_style_sheets_state(&'a self, entity: Entity) -> Result<&'a NodeStyleSheetsState> {
      self
         .query
         .get(entity)
         .map(|n| &n.0)
         .map_err(move |_| StyleError::NoFoundStyleSheetsState { node_id: entity })
   }
}

impl<'a, 'world, 'state, F: QueryFilter> NodeStyleStateOwner<'a, 'a>
   for EntityStyleWorldQuery<'world, 'state, 'a, F>
{
   fn get_current_entity(&self) -> Entity {
      self.current_entity
   }
}
