use std::ops::DerefMut;

use bevy_ecs::component::Component;
use bevy_ecs::prelude::{EntityMut, EntityWorldMut, Mut};
use bevy_ecs::world::World;
use bevy_hierarchy::{BuildWorldChildren, Parent};
use bevy_ui::Style;

use rxy_core::{NodeTree, RendererNodeId};

use crate::{BevyRenderer, ElementEntityExtraData, RendererState};

pub trait BevyWorldExt {
   fn get_or_spawn_empty(
      &mut self,
      parent: Option<&RendererNodeId<BevyRenderer>>,
      reserve_node_id: Option<RendererNodeId<BevyRenderer>>,
   ) -> EntityWorldMut<'_>;
}

impl BevyWorldExt for World {
   fn get_or_spawn_empty(
      &mut self,
      parent: Option<&RendererNodeId<BevyRenderer>>,
      reserve_node_id: Option<RendererNodeId<BevyRenderer>>,
   ) -> EntityWorldMut<'_> {
      let mut entity_world_mut = match reserve_node_id {
         None => self.spawn_empty(),
         Some(reserve_node_id) => self.get_or_spawn(reserve_node_id).unwrap(),
      };
      let old_parent = entity_world_mut.get::<Parent>();
      match (old_parent, parent) {
         (None, None) => {}
         (None, Some(parent)) => {
            entity_world_mut.set_parent(*parent);
         }
         (Some(old_parent), Some(parent)) => {
            if old_parent.get() != *parent {
               entity_world_mut.set_parent(*parent);
            }
         }
         (Some(_), None) => {
            entity_world_mut.remove_parent();
         }
      }
      entity_world_mut
   }
}

pub trait ElementStyleEntityExt {
   fn try_set_style(&mut self, set_f: impl FnOnce(&mut Style));
   fn try_set<T: Component>(&mut self, set_f: impl FnOnce(&mut T));
   fn get_element_extra_data_mut(&mut self) -> Option<Mut<'_, ElementEntityExtraData>>;
}

impl ElementStyleEntityExt for EntityMut<'_> {
   #[inline]
   fn try_set_style(&mut self, set_f: impl FnOnce(&mut Style)) {
      if let Some(mut style) = self.get_mut::<Style>() {
         set_f(style.deref_mut());
      }
   }
   #[inline]
   fn try_set<T: Component>(&mut self, set_f: impl FnOnce(&mut T)) {
      if let Some(mut component) = self.get_mut::<T>() {
         set_f(component.deref_mut());
      }
   }
   fn get_element_extra_data_mut(&mut self) -> Option<Mut<'_, ElementEntityExtraData>> {
      self.get_mut::<ElementEntityExtraData>()
   }
}

impl ElementStyleEntityExt for EntityWorldMut<'_> {
   #[inline]
   fn try_set_style(&mut self, set_f: impl FnOnce(&mut Style)) {
      self.as_entity_mut().try_set_style(set_f);
   }
   #[inline]
   fn try_set<T: Component>(&mut self, set_f: impl FnOnce(&mut T)) {
      self.as_entity_mut().try_set(set_f);
   }
   fn get_element_extra_data_mut(&mut self) -> Option<Mut<'_, ElementEntityExtraData>> {
      self.get_mut::<ElementEntityExtraData>()
   }
}

pub trait ElementEntityWorldMutExt {
   fn as_entity_mut(&mut self) -> EntityMut<'_>;
}

impl ElementEntityWorldMutExt for EntityWorldMut<'_> {
   fn as_entity_mut(&mut self) -> EntityMut<'_> {
      self.into()
   }
}

pub trait EntityWorldMutExt {
   fn insert_if_not_exist<C>(&mut self, component: C)
   where
      C: Component;
   fn get_or_default<S>(&mut self) -> &mut S
   where
      S: Default + Send + Sync + 'static;
   fn state_scoped<S, U>(&mut self, f: impl FnOnce(&mut EntityWorldMut, &mut S) -> U) -> Option<U>
   where
      S: Send + Sync + 'static;
   fn try_state_scoped<S, U>(
      &mut self,
      f: impl FnOnce(&mut EntityWorldMut, Option<&mut S>) -> U,
   ) -> U
   where
      S: Send + Sync + 'static;
}

impl EntityWorldMutExt for EntityWorldMut<'_> {
   fn insert_if_not_exist<C>(&mut self, component: C)
   where
      C: Component,
   {
      if !self.contains::<C>() {
         self.insert(component);
      }
   }
   fn get_or_default<S>(&mut self) -> &mut S
   where
      S: Default + Send + Sync + 'static,
   {
      if !self.contains::<RendererState<S>>() {
         self.insert(RendererState::<S>(Default::default()));
      }
      self
         .get_mut::<RendererState<S>>()
         .map(|n| &mut n.into_inner().0)
         .unwrap()
   }

   fn state_scoped<S, U>(&mut self, f: impl FnOnce(&mut EntityWorldMut, &mut S) -> U) -> Option<U>
   where
      S: Send + Sync + 'static,
   {
      let entity = self.id();
      self.world_scope(|world| {
         world.node_state_scoped(&entity, |world, state| {
            f(&mut world.entity_mut(entity), state)
         })
      })
   }

   fn try_state_scoped<S, U>(
      &mut self,
      f: impl FnOnce(&mut EntityWorldMut, Option<&mut S>) -> U,
   ) -> U
   where
      S: Send + Sync + 'static,
   {
      let entity = self.id();
      self.world_scope(|world| {
         world.try_state_scoped(&entity, |world, state| {
            f(&mut world.entity_mut(entity), state)
         })
      })
   }
}
