#[macro_export]
macro_rules! code_node_tree {
   ($renderer:ident) => {
      #[derive(bevy_ecs::prelude::Component)]
      pub struct RecycledNode {
         placeholder: bevy_ecs::prelude::Entity,
      }

      #[derive(bevy_ecs::prelude::Resource, Copy, Clone)]
      pub struct RecycleNodeContainer {
         entity: bevy_ecs::prelude::Entity,
      }

      const _: () = {
         #[cfg(feature = "reflect")]
         use bevy_ecs::prelude::AppTypeRegistry;
         use bevy_ecs::prelude::{Entity, EntityWorldMut};
         use bevy_ecs::prelude::{FromWorld, World};
         use bevy_hierarchy::{BuildWorldChildren, DespawnRecursiveExt};
         use bevy_hierarchy::{Children, Parent};
         use core::cmp::Ordering;
         use std::any::TypeId;
         use std::borrow::Cow;

         use rxy_core::{
            AttrIndex, ElementAttrType, ElementType, NodeTree, Renderer, RendererNodeId, ViewKey,
         };

         impl FromWorld for RecycleNodeContainer
         {
            fn from_world(world: &mut World) -> Self {
               let entity = world.spawn_empty().id();
               world.set_visibility(true, &entity);
               Self {
                  entity
               }
            }
         }

         impl NodeTree<$renderer> for World
         {
            type NodeTreeScoped = <$renderer as EcsRendererAssociated>::AssociatedNodeTreeScoped;

            fn recycle_node<K: ViewKey<$renderer>>(&mut self, key: &K) {
               let Some(first_node) = key.first_node_id(self) else {
                  return;
               };
               let parent =
                  <World as NodeTree<$renderer>>::get_parent(self, &first_node).unwrap();
               let placeholder = self.spawn_empty().id();
               <$renderer as EcsRenderer>::set_visibility(self, true, &placeholder);
               <World as NodeTree<$renderer>>::insert_before(
                  self,
                  Some(&parent),
                  Some(&first_node),
                  &[placeholder],
               );
               self.init_resource::<RecycleNodeContainer>();
               let recycle_node_container = self
                  .resource::<RecycleNodeContainer>()
                  .entity;
               <World as NodeTree<$renderer>>::set_node_state::<RecycledNode>(
                  self,
                  &first_node,
                  RecycledNode { placeholder },
               );
               key.set_visibility(self, true);
               key.insert_before(self, Some(&recycle_node_container), None);
            }

            fn cancel_recycle_node<K: ViewKey<$renderer>>(&mut self, key: &K) {
               let Some(first_node) = key.first_node_id(self) else {
                  return;
               };
               let placeholder = <World as NodeTree<$renderer>>::take_node_state::<
                  RecycledNode,
               >(self, &first_node)
               .unwrap()
               .placeholder;
               key.insert_before(self, None, Some(&placeholder));
               key.set_visibility(self, false);
            }

            fn set_attr<A: ElementAttrType<$renderer>>(
               &mut self,
               entity: RendererNodeId<$renderer>,
               value: A::Value,
            ) {
               A::update_value(self, entity, value);
               let Some(mut entity_world_mut) = self.get_entity_mut(entity) else {
                  return;
               };
               <$renderer as EcsRenderer>::on_set_attr::<A>(&mut entity_world_mut);
            }

            fn unset_attr<A: ElementAttrType<$renderer>>(
               &mut self,
               entity: RendererNodeId<$renderer>,
            ) {
               A::set_value(self, entity, None::<A::Value>);
               let Some(mut entity_world_mut) = self.get_entity_mut(entity) else {
                  return;
               };
               <$renderer as EcsRenderer>::on_unset_attr::<A>(&mut entity_world_mut);
            }

            fn world_scoped(&self) -> Self::NodeTreeScoped {
               <$renderer as EcsRenderer>::deferred_world_scoped(self)
            }

            fn get_node_state_mut<S: Send + Sync + 'static>(
               &mut self,
               node_id: &RendererNodeId<$renderer>,
            ) -> Option<&mut S> {
               self
                  .get_mut::<RendererState<S>>(*node_id)
                  .map(|n| &mut n.into_inner().0)
            }

            fn get_node_state_ref<S: Send + Sync + 'static>(
               &self,
               node_id: &RendererNodeId<$renderer>,
            ) -> Option<&S> {
               self.get::<RendererState<S>>(*node_id).map(|n| &n.0)
            }

            fn take_node_state<S: Send + Sync + 'static>(
               &mut self,
               node_id: &RendererNodeId<$renderer>,
            ) -> Option<S> {
               self
                  .entity_mut(*node_id)
                  .take::<RendererState<S>>()
                  .map(|n| n.0)
            }

            fn set_node_state<S: Send + Sync + 'static>(
               &mut self,
               node_id: &RendererNodeId<$renderer>,
               state: S,
            ) {
               self.entity_mut(*node_id).insert(RendererState(state));
            }

            fn scoped_type_state<S: Send + Sync + Clone + 'static, U>(
               &self,
               type_id: TypeId,
               f: impl FnOnce(Option<&S>) -> U,
            ) -> U {
               <$renderer as EcsRenderer>::scoped_type_state(self,type_id,f)
            }

            fn exist_node_id(&mut self, node_id: &RendererNodeId<$renderer>) -> bool {
               self.entities().contains(*node_id)
            }

            fn reserve_node_id(&mut self) -> RendererNodeId<$renderer> {
               self.entities().reserve_entity()
            }

            fn spawn_placeholder(
               &mut self,
               name: impl Into<Cow<'static, str>>,
               parent: Option<&RendererNodeId<$renderer>>,
               reserve_node_id: Option<RendererNodeId<$renderer>>,
            ) -> RendererNodeId<$renderer> {
               let mut entity_mut = match reserve_node_id {
                  None => self.spawn_empty(),
                  Some(node_id) => self.get_or_spawn(node_id).unwrap(),
               };
               <$renderer as EcsRenderer>::on_spawn_placeholder(name.into(), &mut entity_mut);
               if let Some(parent) = parent {
                  entity_mut.set_parent(*parent);
               }
               entity_mut.id()
            }

            fn spawn_data_node(&mut self) -> RendererNodeId<$renderer> {
               let mut entity_world_mut = self.spawn_empty();
               <$renderer as EcsRenderer>::on_spawn_data_node(&mut entity_world_mut);
               entity_world_mut.id()
            }

            fn spawn_node<E: ElementType<$renderer>>(
               &mut self,
               parent: Option<&RendererNodeId<$renderer>>,
               reserve_node_id: Option<RendererNodeId<$renderer>>,
            ) -> RendererNodeId<$renderer> {
               let node_id = E::spawn(self, parent, reserve_node_id);
               <$renderer as EcsRenderer>::on_spawn_node::<E>(&mut self.entity_mut(node_id));
               node_id
            }

            fn get_parent(
               &self,
               node_id: &RendererNodeId<$renderer>,
            ) -> Option<RendererNodeId<$renderer>> {
               self.get::<Parent>(*node_id).map(|n| n.get())
            }

            #[inline]
            fn remove_node(&mut self, _node_id: &RendererNodeId<$renderer>) {
               self.entity_mut(*_node_id).despawn_recursive();
            }

            #[inline]
            fn insert_before(
               &mut self,
               parent: Option<&RendererNodeId<$renderer>>,
               before_node_id: Option<&RendererNodeId<$renderer>>,
               inserted_node_ids: &[RendererNodeId<$renderer>],
            ) {
               let parent = parent
                  .cloned()
                  .or_else(|| before_node_id.and_then(|n| self.get::<Parent>(*n).map(|n| n.get())));
               let Some(parent) = parent else {
                  return;
               };
               if let Some(before_node_id) = before_node_id {
                  let children: Vec<Entity> = self
                     .get::<Children>(parent)
                     .unwrap()
                     .iter()
                     .cloned()
                     .collect();
                  let entity_index = children.iter().position(|n| n == before_node_id).unwrap();
                  let mut parent_ref = self.entity_mut(parent);
                  let mut less_count = 0;
                  for x in inserted_node_ids {
                     if let Some(i) = children.iter().position(|n| n == x) {
                        match i.cmp(&entity_index) {
                           Ordering::Less => {
                              less_count += 1;
                              parent_ref.insert_children(
                                 entity_index - less_count,
                                 core::slice::from_ref(x),
                              );
                           }
                           Ordering::Equal => {}
                           Ordering::Greater => {
                              parent_ref.insert_children(entity_index, core::slice::from_ref(x));
                           }
                        }
                     } else {
                        parent_ref.insert_children(entity_index, core::slice::from_ref(x));
                     }
                  }
               } else {
                  self.entity_mut(parent).push_children(inserted_node_ids);
               }
            }

            fn set_visibility(
               &mut self,
               hidden: bool,
               node_id: &RendererNodeId<$renderer>,
            ) {
               <$renderer as EcsRenderer>::set_visibility(self, hidden, node_id)
            }

            fn get_visibility(&self, node_id: &RendererNodeId<$renderer>) -> bool {
               <$renderer as EcsRenderer>::get_visibility(self, node_id)
            }

            fn prepare_set_attr_and_get_is_init(
               &mut self,
               node_id: &RendererNodeId<$renderer>,
               attr_index: AttrIndex,
            ) -> bool {
               <$renderer as EcsRenderer>::prepare_set_attr_and_get_is_init(self, node_id,attr_index)
            }
         }
      };
   };
}
