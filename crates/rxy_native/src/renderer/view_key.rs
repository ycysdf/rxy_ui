use bevy_ecs::prelude::Entity;
use bevy_hierarchy::DespawnRecursiveExt;
use rxy_core::{NodeTree, RendererNodeId, RendererWorld, ViewKey};
use crate::renderer::NativeRenderer;

impl ViewKey<NativeRenderer> for Entity {
    fn remove(self, world: &mut RendererWorld<NativeRenderer>) {
        world.entity_mut(self).despawn_recursive();
    }

    #[inline]
    fn insert_before(
        &self,
        world: &mut RendererWorld<NativeRenderer>,
        parent: Option<&RendererNodeId<NativeRenderer>>,
        before_node_id: Option<&RendererNodeId<NativeRenderer>>,
    ) {
        world.insert_before(parent, before_node_id, std::slice::from_ref(self));
    }
    #[inline]
    fn set_visibility(&self, world: &mut RendererWorld<NativeRenderer>, hidden: bool) {
        world.set_visibility(hidden, self)
    }

    #[inline]
    fn state_node_id(&self) -> Option<RendererNodeId<NativeRenderer>> {
        Some(*self)
    }

    fn reserve_key(world: &mut RendererWorld<NativeRenderer>, _will_rebuild: bool, _parent: RendererNodeId<NativeRenderer>, spawn: bool) -> Self {
        world.reserve_node_id()
    }

    fn first_node_id(
        &self,
        _world: &RendererWorld<NativeRenderer>,
    ) -> Option<RendererNodeId<NativeRenderer>> {
        Some(*self)
    }
}
