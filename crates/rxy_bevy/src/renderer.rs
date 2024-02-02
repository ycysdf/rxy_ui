use bevy_core::Name;
use bevy_derive::{Deref, DerefMut};
use rxy_core::build_info::node_build_times_increment;
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
    mutable_view_rebuild, BuildState, ContainerType, DeferredWorldScoped,
    MutableView, Renderer, RendererElementType, RendererNodeId, RendererWorld, View, ViewCtx,
    ViewKey, ViewMember, ViewMemberCtx, ViewMemberIndex,
};

use crate::{CmdSender, RxyContainerEntity};

#[derive(Reflect, Clone)]
pub struct BevyWrapper<T>(pub T);

#[derive(Deref, DerefMut, Component, Reflect, Clone)]
pub struct RendererState<T: Send + Sync + 'static>(pub T);

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BevyRenderer;

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
            Self::node_state_scoped(world, &entity, |world, state| {
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
            Self::try_state_scoped(world, &entity, |world, state| {
                f(&mut world.entity_mut(entity), state)
            })
        })
    }
}

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

    fn get_or_insert_default_node_state<'a, S: Default + Send + Sync + 'static>(
        world: &'a mut RendererWorld<Self>,
        node_id: &RendererNodeId<Self>,
    ) -> &'a mut S {
        let mut entity_mut = world.entity_mut(*node_id);
        if !entity_mut.contains::<RendererState<S>>() {
            entity_mut.insert(RendererState::<S>(Default::default()));
        }
        world
            .get_mut::<RendererState<S>>(*node_id)
            .map(|n| &mut n.into_inner().0)
            .unwrap()
    }

    fn deferred_world_scoped(world: &mut RendererWorld<Self>) -> impl DeferredWorldScoped<Self> {
        BevyDeferredWorldScoped {
            cmd_sender: world.resource::<CmdSender>().clone(),
        }
    }

    fn get_container_node_id(
        world: &mut RendererWorld<Self>,
        _container_type: ContainerType,
    ) -> RendererNodeId<Self> {
        if let Some(r) = world.get_resource::<RxyContainerEntity>() {
            r.entity
        } else {
            world.init_resource::<RxyContainerEntity>();
            world.resource::<RxyContainerEntity>().entity
        }
    }

    fn spawn_placeholder(
        world: &mut Self::World,
        name: impl Into<Cow<'static, str>>,
        parent: Option<&Self::NodeId>,
        reserve_node_id: Option<RendererNodeId<Self>>,
    ) -> Self::NodeId {
        let mut entity_mut = match reserve_node_id {
            None => world.spawn_empty(),
            Some(node_id) => world.get_or_spawn(node_id).unwrap(),
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

    fn spawn_data_node(world: &mut RendererWorld<Self>) -> RendererNodeId<Self> {
        world.spawn((Name::new("[DATA]"),)).id()
    }

    fn get_parent(
        world: &RendererWorld<Self>,
        node_id: &RendererNodeId<Self>,
    ) -> Option<RendererNodeId<Self>> {
        world.get::<Parent>(*node_id).map(|n| n.get())
    }

    fn ensure_spawn(world: &mut RendererWorld<Self>, reserve_node_id: RendererNodeId<Self>) {
        world.get_or_spawn(reserve_node_id);
    }

    fn spawn_node<E: RendererElementType<Self>>(
        world: &mut Self::World,
        parent: Option<Self::NodeId>,
        reserve_node_id: Option<RendererNodeId<Self>>,
    ) -> RendererNodeId<Self> {
        let node_id = E::spawn(world, parent, reserve_node_id);
        use rxy_bevy_element::ElementEntityExtraData;
        {
            let entity_extra_data = ElementEntityExtraData::new(E::NAME);
            world.entity_mut(node_id).insert(entity_extra_data);
        };
        node_id
    }

    fn exist_node_id(world: &mut RendererWorld<Self>, node_id: &RendererNodeId<Self>) -> bool {
        world.entities().contains(*node_id)
    }

    fn get_node_state_mut<'w, S: Send + Sync + 'static>(
        world: &'w mut RendererWorld<Self>,
        node_id: &RendererNodeId<Self>,
    ) -> Option<&'w mut S> {
        world
            .get_mut::<RendererState<S>>(*node_id)
            .map(|n| &mut n.into_inner().0)
    }

    fn get_node_state_ref<'w, S: Send + Sync + 'static>(
        world: &'w RendererWorld<Self>,
        node_id: &RendererNodeId<Self>,
    ) -> Option<&'w S> {
        world.get::<RendererState<S>>(*node_id).map(|n| &n.0)
    }

    fn take_node_state<S: Send + Sync + 'static>(
        world: &mut RendererWorld<Self>,
        node_id: &RendererNodeId<Self>,
    ) -> Option<S> {
        world
            .entity_mut(*node_id)
            .take::<RendererState<S>>()
            .map(|n| n.0)
    }

    fn set_node_state<S: Send + Sync + 'static>(
        world: &mut RendererWorld<Self>,
        node_id: &RendererNodeId<Self>,
        state: S,
    ) {
        world.entity_mut(*node_id).insert(RendererState(state));
    }

    fn reserve_node_id(world: &mut Self::World) -> Self::NodeId {
        world.entities().reserve_entity()
    }

    fn spawn_and_detach(future: impl Future<Output = ()> + Send + 'static) {
        bevy_tasks::AsyncComputeTaskPool::get()
            .spawn(future)
            .detach();
    }

    fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> Self::Task<T> {
        bevy_tasks::AsyncComputeTaskPool::get().spawn(future)
    }

    #[inline]
    fn remove_node(world: &mut Self::World, _node_id: &Self::NodeId) {
        world.entity_mut(*_node_id).despawn_recursive();
    }

    #[inline]
    fn insert_before(
        world: &mut Self::World,
        parent: Option<&Self::NodeId>,
        before_node_id: Option<&Self::NodeId>,
        inserted_node_ids: &[Self::NodeId],
    ) {
        let parent = parent
            .cloned()
            .or_else(|| before_node_id.and_then(|n| world.get::<Parent>(*n).map(|n| n.get())));
        let Some(parent) = parent else {
            return;
        };
        if let Some(before_node_id) = before_node_id {
            let children: Vec<Entity> = world
                .get::<Children>(parent)
                .unwrap()
                .iter()
                .cloned()
                .collect();
            let entity_index = children.iter().position(|n| n == before_node_id).unwrap();
            let mut parent_ref = world.entity_mut(parent);
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
            world.entity_mut(parent).push_children(inserted_node_ids);
        }
    }

    fn set_visibility(
        world: &mut RendererWorld<Self>,
        hidden: bool,
        node_id: &RendererNodeId<Self>,
    ) {
        if let Some(mut visibility) = world.get_mut::<Visibility>(*node_id) {
            *visibility = if hidden {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            };
        }
    }

    fn get_is_hidden(world: &RendererWorld<Self>, node_id: &RendererNodeId<Self>) -> bool {
        world
            .get::<Visibility>(*node_id)
            .is_some_and(|n| *n == Visibility::Hidden)
    }
}

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
        BevyRenderer::insert_before(world, parent, before_node_id, std::slice::from_ref(self));
    }
    #[inline]
    fn set_visibility(&self, world: &mut RendererWorld<BevyRenderer>, hidden: bool) {
        BevyRenderer::set_visibility(world, hidden, self)
    }

    #[inline]
    fn state_node_id(&self) -> Option<RendererNodeId<BevyRenderer>> {
        Some(*self)
    }

    #[inline]
    fn reserve_key(world: &mut RendererWorld<BevyRenderer>, _will_rebuild: bool) -> Self {
        BevyRenderer::reserve_node_id(world)
    }

    fn first_node_id(
        &self,
        _world: &RendererWorld<BevyRenderer>,
    ) -> Option<RendererNodeId<BevyRenderer>> {
        Some(*self)
    }
}
