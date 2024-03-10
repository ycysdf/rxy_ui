use crate::BevyRenderer;
use bevy_ecs::prelude::Entity;
use bevy_hierarchy::DespawnRecursiveExt;
use rxy_core::{NodeTree, RendererNodeId, RendererWorld, ViewKey};

impl ViewKey<BevyRenderer> for Entity {
    fn remove(self, world: &mut RendererWorld<BevyRenderer>) {
        world.entity_mut(self).despawn_recursive();
    }

    #[inline]
    fn insert_before(
        &self,
        world: &mut RendererWorld<BevyRenderer>,
        parent: Option<&RendererNodeId<BevyRenderer>>,
        before_node_id: Option<&RendererNodeId<BevyRenderer>>,
    ) {
        world.insert_before(parent, before_node_id, std::slice::from_ref(self));
    }
    #[inline]
    fn set_visibility(&self, world: &mut RendererWorld<BevyRenderer>, hidden: bool) {
        world.set_visibility(hidden, self)
    }

    #[inline]
    fn state_node_id(&self) -> Option<RendererNodeId<BevyRenderer>> {
        Some(*self)
    }

    #[inline]
    fn reserve_key(
        world: &mut RendererWorld<BevyRenderer>,
        _will_rebuild: bool,
        parent: RendererNodeId<BevyRenderer>,
        spawn: bool,
    ) -> Self {
        world.reserve_node_id_or_spawn(parent,spawn)
    }

    fn first_node_id(
        &self,
        _world: &RendererWorld<BevyRenderer>,
    ) -> Option<RendererNodeId<BevyRenderer>> {
        Some(*self)
    }
}
