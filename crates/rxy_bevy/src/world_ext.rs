use bevy_ecs::prelude::EntityWorldMut;
use bevy_ecs::component::Component;
use rxy_core::NodeTree;
use crate::RendererState;

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
        self.get_mut::<RendererState<S>>()
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
