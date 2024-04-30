use core::cmp::Ordering;
use std::any::TypeId;
use std::borrow::Cow;

use bevy_ecs::prelude::World;
use bevy_ecs::prelude::{Component, Entity, EntityWorldMut};
use bevy_hierarchy::{BuildWorldChildren, DespawnRecursiveExt};
use bevy_hierarchy::{Children, Parent};
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;
#[cfg(feature = "reflect")]
use bevy_ecs::prelude::AppTypeRegistry;
// use bevy_render::prelude::Visibility;
// use bevy_ui::prelude::NodeBundle;
// use bevy_ui::Display;
// use bevy_ui::Style;

use rxy_core::{
   AttrIndex, DeferredNodeTreeScoped, ElementAttrType, ElementType, NodeTree, RendererNodeId,
   RendererWorld, ViewKey,
};

use crate::renderer::visibility::Visibility;
use crate::renderer::NativeRenderer;
use crate::user_event::UserEventSender;

#[derive(Component)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Name(pub String);

impl Name {
   pub fn new(name: impl Into<String>) -> Self {
      Self(name.into())
   }
}
/*

#[derive(Component, Clone)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct RendererState<T: Send + Sync + 'static>(pub T);

impl NodeTree<NativeRenderer> for World {
   type NodeTreeScoped = UserEventSender;

   fn recycle_node<K: ViewKey<NativeRenderer>>(&mut self, key: &K) {
      todo!()
   }

   fn cancel_recycle_node<K: ViewKey<NativeRenderer>>(&mut self, key: &K) {
      todo!()
   }

   fn set_attr<A: ElementAttrType<NativeRenderer>>(
      &mut self,
      entity: RendererNodeId<NativeRenderer>,
      value: A::Value,
   ) {
      A::update_value(self, entity, value);
      let Some(mut entity_world_mut) = self.get_entity_mut(entity) else {
         return;
      };
   }

   fn unset_attr<A: ElementAttrType<NativeRenderer>>(
      &mut self,
      entity: RendererNodeId<NativeRenderer>,
   ) {
      A::set_value(self, entity, None::<A::Value>);
      let Some(mut entity_world_mut) = self.get_entity_mut(entity) else {
         return;
      };
   }
   fn world_scoped(&self) -> Self::NodeTreeScoped {
      self.non_send_resource::<UserEventSender>().clone()
   }

   fn get_node_state_mut<S: Send + Sync + 'static>(
      &mut self,
      node_id: &RendererNodeId<NativeRenderer>,
   ) -> Option<&mut S> {
      self
         .get_mut::<RendererState<S>>(*node_id)
         .map(|n| &mut n.into_inner().0)
   }

   fn get_node_state_ref<S: Send + Sync + 'static>(
      &self,
      node_id: &RendererNodeId<NativeRenderer>,
   ) -> Option<&S> {
      self.get::<RendererState<S>>(*node_id).map(|n| &n.0)
   }

   fn take_node_state<S: Send + Sync + 'static>(
      &mut self,
      node_id: &RendererNodeId<NativeRenderer>,
   ) -> Option<S> {
      self
         .entity_mut(*node_id)
         .take::<RendererState<S>>()
         .map(|n| n.0)
   }

   fn set_node_state<S: Send + Sync + 'static>(
      &mut self,
      node_id: &RendererNodeId<NativeRenderer>,
      state: S,
   ) {
      self.entity_mut(*node_id).insert(RendererState(state));
   }

   fn scoped_type_state<S: Send + Sync + Clone + 'static, U>(
      &self,
      type_id: TypeId,
      f: impl FnOnce(Option<&S>) -> U,
   ) -> U {
      #[cfg(feature = "reflect")]
      {
         f(self
             .resource::<AppTypeRegistry>()
             .read()
             .get_type_data::<S>(type_id))
      }
      #[cfg(not(feature = "reflect"))]
      {
         todo!()
      }
   }

   fn exist_node_id(&mut self, node_id: &RendererNodeId<NativeRenderer>) -> bool {
      self.entities().contains(*node_id)
   }

   fn reserve_node_id(&mut self) -> RendererNodeId<NativeRenderer> {
      self.entities().reserve_entity()
   }

   fn spawn_placeholder(
      &mut self,
      name: impl Into<Cow<'static, str>>,
      parent: Option<&RendererNodeId<NativeRenderer>>,
      reserve_node_id: Option<RendererNodeId<NativeRenderer>>,
   ) -> RendererNodeId<NativeRenderer> {
      let mut entity_mut = match reserve_node_id {
         None => self.spawn_empty(),
         Some(node_id) => self.get_or_spawn(node_id).unwrap(),
      };
      let entity = entity_mut.id();
      entity_mut.insert((
         // NodeBundle {
         //     visibility: Visibility::Hidden,
         //     style: Style {
         //         display: Display::None,
         //         ..Default::default()
         //     },
         //     ..Default::default()
         // },
         Name::new(format!("{} ({:?})", name.into(), entity)),
      ));
      if let Some(parent) = parent {
         entity_mut.set_parent(*parent);
      }
      entity_mut.id()
   }

   fn spawn_data_node(&mut self) -> RendererNodeId<NativeRenderer> {
      // spawn to container
      self.spawn((Name::new("[DATA]"),)).id()
   }

   fn spawn_node<E: ElementType<NativeRenderer>>(
      &mut self,
      parent: Option<&RendererNodeId<NativeRenderer>>,
      reserve_node_id: Option<RendererNodeId<NativeRenderer>>,
   ) -> RendererNodeId<NativeRenderer> {
      let node_id = E::spawn(self, parent, reserve_node_id);
      {
         // let entity_extra_data = ElementEntityExtraData::new(E::get());
         // self.entity_mut(node_id).insert(entity_extra_data);
      };
      node_id
   }

   fn get_parent(
      &self,
      node_id: &RendererNodeId<NativeRenderer>,
   ) -> Option<RendererNodeId<NativeRenderer>> {
      self.get::<Parent>(*node_id).map(|n| n.get())
   }

   #[inline]
   fn remove_node(&mut self, _node_id: &RendererNodeId<NativeRenderer>) {
      self.entity_mut(*_node_id).despawn_recursive();
   }

   #[inline]
   fn insert_before(
      &mut self,
      parent: Option<&RendererNodeId<NativeRenderer>>,
      before_node_id: Option<&RendererNodeId<NativeRenderer>>,
      inserted_node_ids: &[RendererNodeId<NativeRenderer>],
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
                     parent_ref
                        .insert_children(entity_index - less_count, core::slice::from_ref(x));
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

   fn set_visibility(&mut self, hidden: bool, node_id: &RendererNodeId<NativeRenderer>) {
      if let Some(mut visibility) = self.get_mut::<Visibility>(*node_id) {
         *visibility = if hidden {
            Visibility::Hidden
         } else {
            Visibility::Inherited
         };
      }
   }

   fn get_visibility(&self, node_id: &RendererNodeId<NativeRenderer>) -> bool {
      self
         .get::<Visibility>(*node_id)
         .is_some_and(|n| *n == Visibility::Hidden)
   }

   fn prepare_set_attr_and_get_is_init(
      &mut self,
      node_id: &RendererNodeId<NativeRenderer>,
      attr_index: AttrIndex,
   ) -> bool {
      true
   }
}

pub trait NodeTreeWorldExt {
   fn get_or_spawn_empty(
      &mut self,
      parent: Option<&RendererNodeId<NativeRenderer>>,
      reserve_node_id: Option<RendererNodeId<NativeRenderer>>,
   ) -> EntityWorldMut<'_>;
}

impl NodeTreeWorldExt for World {
   fn get_or_spawn_empty(
      &mut self,
      parent: Option<&RendererNodeId<NativeRenderer>>,
      reserve_node_id: Option<RendererNodeId<NativeRenderer>>,
   ) -> EntityWorldMut<'_> {
      let mut entity_world_mut = match reserve_node_id {
         None => self.spawn_empty(),
         Some(reserve_node_id) => self.get_or_spawn(reserve_node_id).unwrap(),
      };
      if let Some(parent) = parent {
         entity_world_mut.set_parent(*parent);
      }
      entity_world_mut
   }
}
*/