use rxy_core::{prelude::ViewKey, RendererNodeId, RendererWorld};

use super::NativeRenderer;

impl ViewKey<NativeRenderer> for hecs::Entity {
    fn remove(self, world: &mut RendererWorld<NativeRenderer>) {
        let _ = world.world.despawn(self);
        //   world.entity_mut(self).despawn_recursive();
    }

    #[inline]
    fn insert_before(
        &self,
        world: &mut RendererWorld<NativeRenderer>,
        parent: Option<&RendererNodeId<NativeRenderer>>,
        before_node_id: Option<&RendererNodeId<NativeRenderer>>,
    ) {
        //   world.insert_before(parent, before_node_id, std::slice::from_ref(self));
    }
    #[inline]
    fn set_visibility(&self, world: &mut RendererWorld<NativeRenderer>, hidden: bool) {
        //   world.set_visibility(hidden, self)
    }

    #[inline]
    fn state_node_id(&self) -> Option<RendererNodeId<NativeRenderer>> {
        Some(*self)
    }

    #[inline]
    fn reserve_key(world: &mut RendererWorld<NativeRenderer>, _will_rebuild: bool) -> Self {
        world.world.reserve_entity()
    }

    fn first_node_id(
        &self,
        _world: &RendererWorld<NativeRenderer>,
    ) -> Option<RendererNodeId<NativeRenderer>> {
        Some(*self)
    }
}
