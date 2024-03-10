use bevy_ecs::world::EntityWorldMut;
use rxy_core::prelude::Either;
use rxy_core::style::{NodeInterStyleItemId, NodeStyleAttrInfo, NodeStyleItemId};
use crate::attrs::get_attr_by_index;

use super::{Result, StateOwner};

pub trait EntityAttrSyncer {
    fn sync_attr_value_to_element(
        self,
        entity_world_mut: &mut EntityWorldMut,
    ) -> Result;
}

impl<L, R> EntityAttrSyncer for Either<L, R>
where
    L: EntityAttrSyncer,
    R: EntityAttrSyncer,
{
    fn sync_attr_value_to_element(self, entity_world_mut: &mut EntityWorldMut) -> Result {
        match self {
            Either::Left(n) => n.sync_attr_value_to_element(entity_world_mut),
            Either::Right(n) => n.sync_attr_value_to_element(entity_world_mut),
        }
    }
}

impl<'a> EntityAttrSyncer for &'a NodeStyleAttrInfo {
    fn sync_attr_value_to_element(self, entity_world_mut: &mut EntityWorldMut) -> Result {
        self.top_item_id()
            .sync_attr_value_to_element(entity_world_mut)
    }
}

impl EntityAttrSyncer for NodeStyleItemId {
    fn sync_attr_value_to_element(self, entity_world_mut: &mut EntityWorldMut) -> Result {
        let node_id = entity_world_mut.id();
        let attr_index = entity_world_mut
            .world()
            .get_style_item_attr_id(node_id, self)?;
        let value = entity_world_mut
            .world()
            .get_style_item_value(node_id, self)
            .map(|n| n.clone().value)?;
        entity_world_mut.world_scope(|world| {
            get_attr_by_index(attr_index)
                .set_value(world, node_id, Some(value));
        });
        Ok(())
    }
}

impl EntityAttrSyncer for NodeInterStyleItemId {
    #[inline]
    fn sync_attr_value_to_element(self, entity_world_mut: &mut EntityWorldMut) -> Result {
        self.style_item_id
            .sync_attr_value_to_element(entity_world_mut)
    }
}
