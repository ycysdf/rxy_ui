mod view;
mod element;
mod node_tree;

pub use element::*;
pub use view::*;

use bevy_derive::{Deref, DerefMut};
use std::future::Future;

use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::World;
use bevy_hierarchy::DespawnRecursiveExt;
use bevy_reflect::Reflect;
use bevy_tasks::Task;
use rxy_core::{
    DeferredNodeTreeScoped, NodeTree, Renderer, RendererNodeId, RendererWorld,
    ViewKey,
};

use crate::CmdSender;

#[derive(Reflect, Clone)]
pub struct BevyWrapper<T>(pub T);

#[derive(Deref, DerefMut, Component, Reflect, Clone)]
pub struct RendererState<T: Send + Sync + 'static>(pub T);

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BevyRenderer;

#[derive(Clone)]
pub struct BevyDeferredWorldScoped {
    cmd_sender: CmdSender,
}

pub type TaskState = rxy_core::TaskState<BevyRenderer>;

impl DeferredNodeTreeScoped<BevyRenderer> for BevyDeferredWorldScoped {
    fn scoped(&self, f: impl FnOnce(&mut RendererWorld<BevyRenderer>) + Send + 'static) {
        self.cmd_sender.add(move |world: &mut World| f(world))
    }
}
impl Renderer for BevyRenderer {
    type NodeId = Entity;
    type NodeTree = World;

    type Task<T: Send + 'static> = Task<T>;

    fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> Self::Task<T> {
        bevy_tasks::AsyncComputeTaskPool::get().spawn(future)
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
