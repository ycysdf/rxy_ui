use bevy_a11y::Focus;
use bevy_core::Name;
use bevy_derive::{Deref, DerefMut};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::future::Future;

use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::World;
use bevy_ecs::world::EntityWorldMut;
use bevy_hierarchy::{BuildWorldChildren, Children, DespawnRecursiveExt, Parent};
use bevy_reflect::Reflect;
use bevy_render::view::Visibility;
use bevy_tasks::Task;
use bevy_ui::node_bundles::NodeBundle;
use bevy_ui::{Display, Style};

use rxy_core::{
    DeferredWorldScoped, NodeTree, Renderer, RendererElementType, RendererNodeId, RendererWorld,
    ViewKey,
};

use crate::CmdSender;

#[derive(Reflect, Clone)]
pub struct BevyWrapper<T>(pub T);

#[derive(Deref, DerefMut, Component, Reflect, Clone)]
pub struct RendererState<T: Send + Sync + 'static>(pub T);

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BevyRenderer;

#[derive(Clone)]
pub struct BevyDeferredWorldScoped {
    cmd_sender: CmdSender,
}

pub type TaskState = rxy_core::TaskState<BevyRenderer>;

impl DeferredWorldScoped<BevyRenderer> for BevyDeferredWorldScoped {
    fn scoped(&self, f: impl FnOnce(&mut RendererWorld<BevyRenderer>) + Send + 'static) {
        self.cmd_sender.add(move |world: &mut World| f(world))
    }
}
impl Renderer for BevyRenderer {
    type NodeId = Entity;
    type World = World;

    type Task<T: Send + 'static> = Task<T>;

    fn spawn_and_detach(future: impl Future<Output = ()> + Send + 'static) {
        bevy_tasks::AsyncComputeTaskPool::get()
            .spawn(future)
            .detach();
    }

    fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> Self::Task<T> {
        bevy_tasks::AsyncComputeTaskPool::get().spawn(future)
    }
}

impl ViewKey<BevyRenderer> for Entity {
    fn remove(self, world: &mut RendererWorld<BevyRenderer>) {
        let mut focus = world.resource_mut::<Focus>();
        if focus.0 == Some(self) {
            focus.0 = None;
        }
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
    fn reserve_key(world: &mut RendererWorld<BevyRenderer>, _will_rebuild: bool) -> Self {
        world.reserve_node_id()
    }

    fn first_node_id(
        &self,
        _world: &RendererWorld<BevyRenderer>,
    ) -> Option<RendererNodeId<BevyRenderer>> {
        Some(*self)
    }
}

impl NodeTree<BevyRenderer> for World {
    fn deferred_world_scoped(&mut self) -> impl DeferredWorldScoped<BevyRenderer> {
        BevyDeferredWorldScoped {
            cmd_sender: self.resource::<CmdSender>().clone(),
        }
    }

    fn get_node_state_mut<S: Send + Sync + 'static>(
        &mut self,
        node_id: &RendererNodeId<BevyRenderer>,
    ) -> Option<&mut S> {
        self.get_mut::<RendererState<S>>(*node_id)
            .map(|n| &mut n.into_inner().0)
    }

    fn get_node_state_ref<S: Send + Sync + 'static>(
        &self,
        node_id: &RendererNodeId<BevyRenderer>,
    ) -> Option<&S> {
        self.get::<RendererState<S>>(*node_id).map(|n| &n.0)
    }

    fn take_node_state<S: Send + Sync + 'static>(
        &mut self,
        node_id: &RendererNodeId<BevyRenderer>,
    ) -> Option<S> {
        self.entity_mut(*node_id)
            .take::<RendererState<S>>()
            .map(|n| n.0)
    }

    fn set_node_state<S: Send + Sync + 'static>(
        &mut self,
        node_id: &RendererNodeId<BevyRenderer>,
        state: S,
    ) {
        self.entity_mut(*node_id).insert(RendererState(state));
    }

    fn exist_node_id(&mut self, node_id: &RendererNodeId<BevyRenderer>) -> bool {
        self.entities().contains(*node_id)
    }

    fn reserve_node_id(&mut self) -> RendererNodeId<BevyRenderer> {
        self.entities().reserve_entity()
    }

    fn spawn_placeholder(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        parent: Option<&RendererNodeId<BevyRenderer>>,
        reserve_node_id: Option<RendererNodeId<BevyRenderer>>,
    ) -> RendererNodeId<BevyRenderer> {
        let mut entity_mut = match reserve_node_id {
            None => self.spawn_empty(),
            Some(node_id) => self.get_or_spawn(node_id).unwrap(),
        };
        let entity = entity_mut.id();
        entity_mut.insert((
            NodeBundle {
                visibility: Visibility::Hidden,
                style: Style {
                    display: Display::None,
                    ..Default::default()
                },
                ..Default::default()
            },
            Name::new(format!("{} ({:?})", name.into(), entity)),
        ));
        if let Some(parent) = parent {
            entity_mut.set_parent(*parent);
        }
        entity_mut.id()
    }

    fn ensure_spawn(&mut self, reserve_node_id: RendererNodeId<BevyRenderer>) {
        self.get_or_spawn(reserve_node_id);
    }

    fn spawn_empty_node(
        &mut self,
        parent: Option<RendererNodeId<BevyRenderer>>,
        reserve_node_id: Option<RendererNodeId<BevyRenderer>>,
    ) -> RendererNodeId<BevyRenderer> {
        let mut entity_world_mut = match reserve_node_id {
            None => self.spawn_empty(),
            Some(reserve_node_id) => self.get_or_spawn(reserve_node_id).unwrap(),
        };
        if let Some(parent) = parent {
            entity_world_mut.set_parent(parent);
        }
        entity_world_mut.id()
    }

    fn spawn_data_node(&mut self) -> RendererNodeId<BevyRenderer> {
        self.spawn((Name::new("[DATA]"),)).id()
    }

    fn spawn_node<E: RendererElementType<BevyRenderer>>(
        &mut self,
        parent: Option<RendererNodeId<BevyRenderer>>,
        reserve_node_id: Option<RendererNodeId<BevyRenderer>>,
    ) -> RendererNodeId<BevyRenderer> {
        let node_id = E::spawn(self, parent, reserve_node_id);
        use rxy_bevy_element::ElementEntityExtraData;
        {
            let entity_extra_data = ElementEntityExtraData::new(E::NAME);
            self.entity_mut(node_id).insert(entity_extra_data);
        };
        node_id
    }

    fn get_parent(
        &self,
        node_id: &RendererNodeId<BevyRenderer>,
    ) -> Option<RendererNodeId<BevyRenderer>> {
        self.get::<Parent>(*node_id).map(|n| n.get())
    }

    #[inline]
    fn remove_node(&mut self, _node_id: &RendererNodeId<BevyRenderer>) {
        self.entity_mut(*_node_id).despawn_recursive();
    }

    #[inline]
    fn insert_before(
        &mut self,
        parent: Option<&RendererNodeId<BevyRenderer>>,
        before_node_id: Option<&RendererNodeId<BevyRenderer>>,
        inserted_node_ids: &[RendererNodeId<BevyRenderer>],
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

    fn set_visibility(&mut self, hidden: bool, node_id: &RendererNodeId<BevyRenderer>) {
        if let Some(mut visibility) = self.get_mut::<Visibility>(*node_id) {
            *visibility = if hidden {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            };
        }
    }

    fn get_visibility(&self, node_id: &RendererNodeId<BevyRenderer>) -> bool {
        self.get::<Visibility>(*node_id)
            .is_some_and(|n| *n == Visibility::Hidden)
    }
}

impl BevyRenderer {
    pub fn get_or_insert_default_state_by_entity_mut<'a, S: Default + Send + Sync + 'static>(
        entity_world_mut: &'a mut EntityWorldMut,
    ) -> &'a mut S {
        if !entity_world_mut.contains::<RendererState<S>>() {
            entity_world_mut.insert(RendererState::<S>(Default::default()));
        }
        entity_world_mut
            .get_mut::<RendererState<S>>()
            .map(|n| &mut n.into_inner().0)
            .unwrap()
    }

    pub fn entity_state_scoped<S: Send + Sync + 'static, U>(
        entity_world_mut: &mut EntityWorldMut,
        f: impl FnOnce(&mut EntityWorldMut, &mut S) -> U,
    ) -> Option<U> {
        let entity = entity_world_mut.id();
        entity_world_mut.world_scope(|world| {
            world.node_state_scoped(&entity, |world, state| {
                f(&mut world.entity_mut(entity), state)
            })
        })
    }
    pub fn try_entity_state_scoped<S: Send + Sync + 'static, U>(
        entity_world_mut: &mut EntityWorldMut,
        f: impl FnOnce(&mut EntityWorldMut, Option<&mut S>) -> U,
    ) -> U {
        let entity = entity_world_mut.id();
        entity_world_mut.world_scope(|world| {
            world.try_state_scoped(&entity, |world, state| {
                f(&mut world.entity_mut(entity), state)
            })
        })
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
