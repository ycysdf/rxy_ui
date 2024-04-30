
mod node_tree;
mod view_key;

#[macro_export]
macro_rules! code_ecs_renderer {
   ($renderer:ident) => {

      #[derive(bevy_ecs::prelude::Component, Clone)]
      #[cfg_attr(feature = "reflect", derive(Reflect))]
      pub struct RendererState<T: Send + Sync + 'static>(pub T);

      impl<T: Send + Sync + 'static> ::std::ops::Deref for RendererState<T> {
         type Target = T;
         fn deref(&self) -> &Self::Target { &self.0 }
      }
      impl<T: Send + Sync + 'static> ::std::ops::DerefMut for RendererState<T> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 } }

      pub trait EcsRendererAssociated: {
         type AssociatedTask<T: rxy_core::MaybeSend + 'static>: rxy_core::MaybeSend + 'static;
         type AssociatedNodeTreeScoped: rxy_core::DeferredNodeTreeScoped<$renderer>;
      }

      trait EcsRenderer:
         rxy_core::Renderer+EcsRendererAssociated
      {
         fn spawn_task<T: rxy_core::MaybeSend + 'static>(
            future: impl std::future::Future<Output = T> + rxy_core::MaybeSend + 'static,
         ) -> <Self as Renderer>::Task<T>;

         fn deferred_world_scoped(world: &bevy_ecs::prelude::World) -> <Self as EcsRendererAssociated>::AssociatedNodeTreeScoped;

         fn on_spawn_placeholder(
            _name: std::borrow::Cow<'static, str>,
            _entity_world_mut: &mut bevy_ecs::prelude::EntityWorldMut,
         ) {
         }

         fn on_spawn_data_node(_entity_world_mut: &mut bevy_ecs::prelude::EntityWorldMut) {}

         fn on_spawn_node<E: rxy_core::ElementType<Self>>(
            _entity_world_mut: &mut bevy_ecs::prelude::EntityWorldMut,
         ) {
         }

         fn set_visibility(
            world: &mut bevy_ecs::prelude::World,
            hidden: bool,
            node_id: &rxy_core::RendererNodeId<Self>,
         );

         fn get_visibility(
            world: &bevy_ecs::prelude::World,
            node_id: &rxy_core::RendererNodeId<Self>,
         ) -> bool;

         fn on_set_attr<A: rxy_core::ElementAttrType<Self>>(
            _entity_world_mut: &mut bevy_ecs::prelude::EntityWorldMut,
         ) {
         }

         fn on_unset_attr<A: rxy_core::ElementAttrType<Self>>(
            _entity_world_mut: &mut bevy_ecs::prelude::EntityWorldMut,
         ) {
         }

         fn prepare_set_attr_and_get_is_init(
            world: &mut bevy_ecs::prelude::World,
            node_id: &rxy_core::RendererNodeId<Self>,
            attr_index: rxy_core::AttrIndex,
         ) -> bool {
            true
         }


         fn scoped_type_state<S: Send + Sync + Clone + 'static, U>(
            world: &bevy_ecs::prelude::World,
            type_id: core::any::TypeId,
            f: impl FnOnce(Option<&S>) -> U,
         ) -> U;
      }

      const _: () = {
         use bevy_ecs::prelude::{Entity, World};
         use rxy_core::{MaybeSend, NodeTree, Renderer};
         use std::future::Future;
         impl Renderer for $renderer
         {
            type NodeId = Entity;
            type NodeTree = World;
            type Task<T: MaybeSend + 'static> = <$renderer as EcsRendererAssociated>::AssociatedTask<T>;

            fn spawn_task<T: MaybeSend + 'static>(
               future: impl Future<Output = T> + MaybeSend + 'static,
            ) -> Self::Task<T> {
               <$renderer as EcsRenderer>::spawn_task(future)
            }
         }
      };
   };
}

#[macro_export]
macro_rules! define_bevy_ces_renderer {
   ($(#[$meta:meta])? $vis:vis struct $renderer:ident;) => {
      #[derive(Debug, Clone, Copy, PartialEq, Eq)]
      $(#[$meta])?
      $vis struct $renderer;

      $crate::code_ecs_renderer!($renderer);
      $crate::code_node_tree!($renderer);
      $crate::code_view_key!($renderer);
   };
}
use bevy_ecs::prelude::{Entity, EntityWorldMut, World};
use bevy_hierarchy::BuildWorldChildren;

pub trait NodeTreeWorldExt {
   fn get_or_spawn_empty(
      &mut self,
      parent: Option<&Entity>,
      reserve_node_id: Option<Entity>,
   ) -> EntityWorldMut<'_>;
}

impl NodeTreeWorldExt for World {
   fn get_or_spawn_empty(
      &mut self,
      parent: Option<&Entity>,
      reserve_node_id: Option<Entity>,
   ) -> EntityWorldMut<'_> {
      let mut entity_world_mut = match reserve_node_id {
         None => self.spawn_empty(),
         Some(reserve_node_id) => self.get_or_spawn(reserve_node_id).unwrap(),
      };
      if let Some(parent) = parent {
         entity_world_mut.set_parent(parent.clone());
      }
      entity_world_mut
   }
}
