#[macro_export]
macro_rules! code_view_key {
   ($renderer:ident) => {
      const _: () = {
         use bevy_ecs::prelude::{Entity, World};
         use bevy_hierarchy::DespawnRecursiveExt;
         use rxy_core::{NodeTree, RendererNodeId, RendererWorld, ViewKey};

         impl ViewKey<$renderer> for Entity
         {
            fn remove(self, world: &mut RendererWorld<$renderer>) {
               world.entity_mut(self).despawn_recursive();
            }

            #[inline]
            fn insert_before(
               &self,
               world: &mut RendererWorld<$renderer>,
               parent: Option<&RendererNodeId<$renderer>>,
               before_node_id: Option<&RendererNodeId<$renderer>>,
            ) {
               world.insert_before(
                  parent,
                  before_node_id,
                  std::slice::from_ref(self),
               );
            }
            #[inline]
            fn set_visibility(
               &self,
               world: &mut RendererWorld<$renderer>,
               hidden: bool,
            ) {
               <World as NodeTree<$renderer>>::set_visibility(world, hidden, self)
            }

            #[inline]
            fn state_node_id(&self) -> Option<RendererNodeId<$renderer>> {
               Some(*self)
            }

            fn reserve_key(
               world: &mut RendererWorld<$renderer>,
               _will_rebuild: bool,
               parent: RendererNodeId<$renderer>,
               spawn: bool,
            ) -> Self {
               world.reserve_node_id_or_spawn(parent, spawn)
            }

            fn first_node_id(
               &self,
               _world: &RendererWorld<$renderer>,
            ) -> Option<RendererNodeId<$renderer>> {
               Some(*self)
            }
         }
      };
   };
}
