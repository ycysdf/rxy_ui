use core::iter;

use bevy_ecs::prelude::Entity;
use bevy_ecs::world::{EntityRef, EntityWorldMut};
use bevy_ui::Interaction;

use rxy_core::prelude::EitherExt;
use rxy_core::style::{
   IterExt, NodeInterStyleAttrInfos, NodeStyleAttrInfo, NodeStyleAttrInfos, PipeOp,
   StyleInteraction,
};
use rxy_core::AttrIndex;

use super::interaction_style::AttrSetBitsIterExt;
use super::rxy_bevy_crate::{
   AttrSetBits, ElementEntityExtraData, ElementEntityWorldMutExt, FocusedEntity, RendererState,
};
use super::Result;
use super::{interaction_to_style_interaction, EntityAttrSyncer};

#[derive(Default)]
pub struct EntityStyleAttrInfoIterArgs<'a> {
   pub iter_normal_style_sheet: bool,
   pub iter_inter_style_sheet: bool,
   pub limit_attr_ids: Option<&'a [AttrIndex]>,
}

impl<'a> EntityStyleAttrInfoIterArgs<'a> {
   pub fn new() -> Self {
      Self::default()
   }
   pub fn normal() -> Self {
      Self {
         iter_normal_style_sheet: true,
         ..Default::default()
      }
   }
   pub fn inter() -> Self {
      Self {
         iter_inter_style_sheet: true,
         ..Default::default()
      }
   }
   pub fn all_kind() -> Self {
      Self {
         iter_inter_style_sheet: true,
         iter_normal_style_sheet: true,
         ..Default::default()
      }
   }
   pub fn iter_and_sync_set(
      self,
      mut entity_world_mut: EntityWorldMut, // strict_match: bool,
   ) -> Result {
      let focus = entity_world_mut.world().resource::<FocusedEntity>().0;
      let entity_ref = entity_world_mut.as_entity_mut();
      let item_ids = self
         .iter_match_attrs(
            unsafe { core::mem::transmute(entity_ref.as_readonly()) },
            focus,
            false,
         )
         .map(|n| (n.0, n.1.top_item_id()))
         .collect::<Vec<_>>();

      let mut attr_bits: AttrSetBits = 0;
      for (attr_index, top_item_id) in item_ids {
         if (attr_bits >> attr_index) & 1 == 1 {
            continue;
         }
         attr_bits |= 1 << attr_index;
         top_item_id.sync_attr_value_to_element(&mut entity_world_mut)?;
      }

      Ok(())
   }

   pub fn iter_match_attrs(
      self,
      entity_ref: EntityRef<'a>,
      focused_entity: Option<Entity>,
      strict_match: bool,
   ) -> impl Iterator<Item = (AttrIndex, &NodeStyleAttrInfo)> {
      let limit_attr_bits = entity_ref
         .get::<ElementEntityExtraData>()
         .map(|n| n.attr_is_set);
      let r = iter::empty();

      let attr_infos = || {
         let entity_style_state = entity_ref
            .get_ref::<RendererState<NodeStyleAttrInfos>>()
            .map(|n| n.into_inner())
            .unwrap();
         match self.limit_attr_ids {
            Some(n) => n
               .iter()
               .filter_map(|id| entity_style_state.get(id).map(|n| (*id, n)))
               .either_left(),
            None => entity_style_state
               .iter()
               .map(|n| (*n.0, n.1))
               .either_right(),
         }
      };

      let inter_attr_infos = || {
         let r = iter::empty();
         r.option_map(
            entity_ref
               .get_ref::<RendererState<NodeInterStyleAttrInfos>>()
               .map(|n| n.into_inner()),
            |_, entity_inter_style_state| {
               let mut node_interaction = entity_ref
                  .get::<Interaction>()
                  .cloned()
                  .and_then(interaction_to_style_interaction)
                  .unwrap_or(StyleInteraction::empty());
               if focused_entity == Some(entity_ref.id()) {
                  node_interaction |= StyleInteraction::Focus;
               }

               match self.limit_attr_ids {
                  Some(n) => n
                     .iter()
                     .cloned()
                     .filter_map(move |id| {
                        let attr_info =
                           entity_inter_style_state.match_attr(id, node_interaction, strict_match);
                        attr_info.map(|n| (id, n))
                     })
                     .either_left(),
                  None => entity_inter_style_state
                     .iter_match_attr(Some(node_interaction), strict_match)
                     .map(|n| (n.0, n.1))
                     .either_right(),
               }
            },
         )
      };

      r.chain_option(self.iter_inter_style_sheet.then(inter_attr_infos))
         .chain_option(self.iter_normal_style_sheet.then(attr_infos))
         .option_map(limit_attr_bits, |n, limit_attr_bits| {
            n.filter_attr_already_set(limit_attr_bits)
         })
   }
}
