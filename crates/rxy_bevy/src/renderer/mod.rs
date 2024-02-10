use std::future::Future;

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::World;
use bevy_reflect::Reflect;
use bevy_tasks::Task;

pub use composite_attrs::*;
use rxy_core::{
    DeferredNodeTreeScoped, Element, ElementAttr, ElementTypeUnTyped,
    ElementViewChildren, Renderer, RendererWorld,
};
pub use text_styled_element::*;

use crate::elements::element_div;
use crate::CmdSender;

pub mod attrs;
mod composite_attrs;
pub mod elements;
mod node_tree;
mod text_styled_element;
mod view_key;
pub mod common_renderer;

#[inline(always)]
pub fn view_element_type() -> &'static dyn ElementTypeUnTyped<BevyRenderer> {
    &element_div
}

pub type BevyElement<E, VM> = Element<BevyRenderer, E, VM>;

pub type BevyElementViewChildren<CV, E, VM> =
    ElementViewChildren<BevyRenderer, Element<BevyRenderer, E, VM>, CV>;

pub type BevyElementAttrMember<EA> = ElementAttr<BevyRenderer, EA>;

pub type TaskState = rxy_core::TaskState<BevyRenderer>;

#[derive(Deref, DerefMut, Component, Reflect, Clone)]
pub struct RendererState<T: Send + Sync + 'static>(pub T);

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BevyRenderer;

impl Renderer for BevyRenderer {
    type NodeId = Entity;
    type NodeTree = World;

    #[cfg(not(target_arch = "wasm32"))]
    type Task<T: Send + 'static> = Task<T>;

    #[cfg(target_arch = "wasm32")]
    type Task<T: Send + 'static> = ();

    #[cfg(any(target_arch = "wasm32", not(feature = "multi-threaded")))]
    fn spawn_task<T: Send + 'static>(
        future: impl Future<Output = T> + Send + 'static,
    ) -> Self::Task<T> {
        bevy_tasks::AsyncComputeTaskPool::get().spawn(future)
    }

    #[cfg(target_arch = "wasm32")]
    fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> Self::Task<T> {
        bevy_tasks::AsyncComputeTaskPool::get().spawn(future);
    }
}

#[derive(Clone)]
pub struct BevyDeferredWorldScoped {
    cmd_sender: CmdSender,
}

impl DeferredNodeTreeScoped<BevyRenderer> for BevyDeferredWorldScoped {
    fn scoped(&self, f: impl FnOnce(&mut RendererWorld<BevyRenderer>) + Send + 'static) {
        self.cmd_sender.add(move |world: &mut World| f(world))
    }
}
