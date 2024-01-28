use std::sync::OnceLock;

use bevy_ecs::world::EntityWorldMut;
use derive_more::IntoIterator;

use bevy_ui::Interaction;
use rxy_bevy::elements::view_element_type;
use rxy_core::prelude::Either;
use rxy_style::{NodeInterStyleAttrInfo, NodeInterStyleItemId, NodeStyleAttrInfo, NodeStyleItemId};

use crate::{
    attr_iter::EntityStyleAttrInfoIterItem, interaction_to_style_kind, Result, StateOwner,
};

pub trait EntityAttrSyncer {
    fn sync_attr_value_to_element<'w>(
        self,
        entity_world_mut: &'w mut EntityWorldMut<'w>,
        // context: &mut SetAttrValueContext,
        // entity_style_state: &NodeStyleState,
        // shared_style_sheets: &SharedStyleSheets,
    ) -> Result;
}

impl<L, R> EntityAttrSyncer for Either<L, R>
where
    L: EntityAttrSyncer,
    R: EntityAttrSyncer,
{
    fn sync_attr_value_to_element<'w>(
        self,
        entity_world_mut: &'w mut EntityWorldMut<'w>,
    ) -> Result {
        match self {
            Either::Left(n) => n.sync_attr_value_to_element(entity_world_mut),
            Either::Right(n) => n.sync_attr_value_to_element(entity_world_mut),
        }
    }
}

impl<'a> EntityAttrSyncer for &'a NodeStyleAttrInfo {
    fn sync_attr_value_to_element<'w>(
        self,
        entity_world_mut: &'w mut EntityWorldMut<'w>,
    ) -> Result {
        self.eval_current_item_id()
            .sync_attr_value_to_element(entity_world_mut)
    }
}
impl<'a> EntityAttrSyncer for &'a NodeInterStyleAttrInfo {
    fn sync_attr_value_to_element<'w>(
        self,
        entity_world_mut: &'w mut EntityWorldMut<'w>,
    ) -> Result {
        let Some(interaction) = entity_world_mut.get::<Interaction>() else {
            return Ok(());
        };
        let Some(interaction) = interaction_to_style_kind(*interaction) else {
            return Ok(());
        };
        let Some(style_item_id) = self.eval_current_item_id(interaction, false) else {
            return Ok(());
        };

        style_item_id.sync_attr_value_to_element(entity_world_mut)
    }
}

impl EntityAttrSyncer for NodeStyleItemId {
    fn sync_attr_value_to_element<'w>(
        self,
        entity_world_mut: &'w mut EntityWorldMut<'w>,
    ) -> Result {
        let node_id = entity_world_mut.id();
        let attr_id = entity_world_mut
            .world()
            .get_style_item_attr_id(node_id, self)?;
        let value = entity_world_mut
            .world()
            .get_style_item_value(node_id, self)
            .map(|n| n.clone().value)?;
        view_element_type()
            .attr_by_index(attr_id)
            .init_or_set(entity_world_mut, Some(value));
        Ok(())
    }
}

impl EntityAttrSyncer for NodeInterStyleItemId {
    #[inline(always)]
    fn sync_attr_value_to_element<'w>(
        self,
        entity_world_mut: &'w mut EntityWorldMut<'w>,
    ) -> Result {
        self.style_item_id
            .sync_attr_value_to_element(entity_world_mut)
    }
}

#[derive(IntoIterator)]
pub struct SyncerWrapper<T>(pub T);

impl<'a, T> EntityAttrSyncer for SyncerWrapper<T>
where
    T: Iterator<Item = EntityStyleAttrInfoIterItem<'a>>,
{
    #[inline(always)]
    fn sync_attr_value_to_element<'w>(
        self,
        entity_world_mut: &'w mut EntityWorldMut<'w>,
    ) -> Result {
        let interaction = OnceLock::new();
        for (attr_id, attr_info) in self {
            let item_id = match attr_info {
                Either::Left(n) => n.eval_current_item_id(),
                Either::Right(n) => {
                    let Some(interaction) = interaction.get_or_init(|| {
                        entity_world_mut
                            .get::<Interaction>()
                            .cloned()
                            .and_then(interaction_to_style_kind)
                    }) else {
                        return Ok(());
                    };
                    let Some(item_id) = n.eval_current_item_id(*interaction, false) else {
                        return Ok(());
                    };
                    item_id.item_id
                }
            };
            let entity = entity_world_mut.id();
            let value = entity_world_mut
                .world()
                .get_style_item_value(entity, item_id)
                .map(|n| n.clone().value)?;
            view_element_type()
                .attr_by_index(attr_id)
                .init_or_set(entity_world_mut, Some(value));
        }
        Ok(())
    }
}
