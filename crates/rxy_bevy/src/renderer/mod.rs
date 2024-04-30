use bevy_core::Name;
use std::any::TypeId;
use std::borrow::Cow;
use std::future::Future;

use bevy_ecs::prelude::{AppTypeRegistry, EntityWorldMut, World};
use bevy_reflect::Reflect;
use bevy_render::prelude::Visibility;
use bevy_tasks::Task;
use bevy_ui::node_bundles::NodeBundle;
use bevy_ui::{Display, Style};

pub use composite_attrs::*;
use rxy_bevy_ecs::define_bevy_ces_renderer;
use rxy_core::{
   AttrIndex, DeferredNodeTreeScoped, Element, ElementAttr, ElementAttrType, ElementType,
   ElementTypeUnTyped, ElementViewChildren, Renderer, RendererNodeId, RendererWorld,
};
#[cfg(feature = "tailwind_aliases")]
pub use tailwind_attrs::*;
pub use text_styled_element::*;

use crate::elements::element_div;
use crate::{CmdSender, ElementEntityExtraData, ElementEntityWorldMutExt, ElementStyleEntityExt};

pub mod attrs;
pub mod common_renderer;
mod composite_attrs;
pub mod elements;
// mod node_tree;
mod text_styled_element;
// mod view_key;

pub mod event;
#[cfg(feature = "style")]
pub mod style;
#[cfg(feature = "tailwind_aliases")]
mod tailwind_attrs;
pub mod view_builder_ext;

#[inline]
pub fn view_element_type() -> &'static dyn ElementTypeUnTyped<BevyRenderer> {
   &element_div
}

pub type BevyElement<E, VM> = Element<BevyRenderer, E, VM>;
#[cfg(feature = "dynamic_element")]
pub type DynamicBevyElement<E> = rxy_core::DynamicElement<BevyRenderer, E>;

pub type BevyElementViewChildren<CV, E, VM> =
   ElementViewChildren<BevyRenderer, Element<BevyRenderer, E, VM>, CV>;

pub type BevyElementAttrMember<EA> = ElementAttr<BevyRenderer, EA>;

pub type TaskState = rxy_core::TaskState<BevyRenderer>;

define_bevy_ces_renderer! {
   #[derive(Reflect)]
   pub struct BevyRenderer;
}

impl EcsRendererAssociated for BevyRenderer {
   #[cfg(not(target_arch = "wasm32"))]
   type AssociatedTask<T: Send + 'static> = Task<T>;

   #[cfg(target_arch = "wasm32")]
   type AssociatedTask<T: Send + 'static> = ();
   type AssociatedNodeTreeScoped = BevyDeferredWorldScoped;
}

impl EcsRenderer for BevyRenderer {
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

   fn deferred_world_scoped(
      world: &World,
   ) -> <Self as EcsRendererAssociated>::AssociatedNodeTreeScoped {
      BevyDeferredWorldScoped {
         cmd_sender: world.resource::<CmdSender>().clone(),
      }
   }

   fn set_visibility(world: &mut World, hidden: bool, node_id: &RendererNodeId<Self>) {
      if let Some(mut visibility) = world.get_mut::<Visibility>(*node_id) {
         *visibility = if hidden {
            Visibility::Hidden
         } else {
            Visibility::Inherited
         };
      }
   }

   fn get_visibility(world: &World, node_id: &RendererNodeId<Self>) -> bool {
      world
         .get::<Visibility>(*node_id)
         .is_some_and(|n| *n == Visibility::Hidden)
   }

   fn on_set_attr<A: ElementAttrType<Self>>(entity_world_mut: &mut EntityWorldMut) {
      entity_world_mut
         .as_entity_mut()
         .get_element_extra_data_mut()
         .unwrap() // todo: error handle
         .set_attr(A::INDEX, true);
   }
   fn on_unset_attr<A: ElementAttrType<Self>>(entity_world_mut: &mut EntityWorldMut) {
      entity_world_mut
         .as_entity_mut()
         .get_element_extra_data_mut()
         .unwrap() // todo: error handle
         .set_attr(A::INDEX, false);
   }

   fn on_spawn_placeholder(name: Cow<'static, str>, entity_world_mut: &mut EntityWorldMut) {
      let entity = entity_world_mut.id();
      entity_world_mut.insert((
         NodeBundle {
            visibility: Visibility::Hidden,
            style: Style {
               display: Display::None,
               ..Default::default()
            },
            ..Default::default()
         },
         Name::new(format!("{} ({:?})", name, entity)),
      ));
   }

   fn on_spawn_data_node(entity_world_mut: &mut EntityWorldMut) {
      entity_world_mut.insert((Name::new("[DATA]"),));
   }

   fn on_spawn_node<E: ElementType<Self>>(entity_world_mut: &mut EntityWorldMut) {
      let entity_extra_data = ElementEntityExtraData::new(E::get());
      entity_world_mut.insert(entity_extra_data);
   }
   fn prepare_set_attr_and_get_is_init(
      world: &mut World,
      node_id: &RendererNodeId<Self>,
      attr_index: AttrIndex,
   ) -> bool {
      let mut entity_mut = world.entity_mut(*node_id);
      let mut extra_data = entity_mut.get_mut::<ElementEntityExtraData>().unwrap();
      let is_init = extra_data.is_init_attr(attr_index);
      if !is_init {
         extra_data.init_attr(attr_index, true);
      }
      is_init
   }

   fn scoped_type_state<S: Send + Sync + Clone + 'static, U>(
      world: &World,
      type_id: TypeId,
      f: impl FnOnce(Option<&S>) -> U,
   ) -> U {
      f(world
         .resource::<AppTypeRegistry>()
         .read()
         .get_type_data::<S>(type_id))
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

//
// use std::future::Future;
//
// use bevy_derive::{Deref, DerefMut};
// use bevy_ecs::component::Component;
// use bevy_ecs::entity::Entity;
// use bevy_ecs::prelude::World;
// use bevy_reflect::Reflect;
// use bevy_tasks::Task;
//
// pub use composite_attrs::*;
// use rxy_core::{
//    DeferredNodeTreeScoped, Element, ElementAttr, ElementTypeUnTyped, ElementViewChildren, Renderer,
//    RendererWorld,
// };
// #[cfg(feature = "tailwind_aliases")]
// pub use tailwind_attrs::*;
// pub use text_styled_element::*;
//
// use crate::elements::element_div;
// use crate::CmdSender;
//
// pub mod attrs;
// pub mod common_renderer;
// mod composite_attrs;
// pub mod elements;
// mod node_tree;
// mod text_styled_element;
// mod view_key;
//
// pub mod event;
// #[cfg(feature = "style")]
// pub mod style;
// #[cfg(feature = "tailwind_aliases")]
// mod tailwind_attrs;
// pub mod view_builder_ext;
//
// #[inline]
// pub fn view_element_type() -> &'static dyn ElementTypeUnTyped<BevyRenderer> {
//    &element_div
// }
//
// pub type BevyElement<E, VM> = Element<BevyRenderer, E, VM>;
// #[cfg(feature = "dynamic_element")]
// pub type DynamicBevyElement<E> = rxy_core::DynamicElement<BevyRenderer, E>;
//
// pub type BevyElementViewChildren<CV, E, VM> =
// ElementViewChildren<BevyRenderer, Element<BevyRenderer, E, VM>, CV>;
//
// pub type BevyElementAttrMember<EA> = ElementAttr<BevyRenderer, EA>;
//
// pub type TaskState = rxy_core::TaskState<BevyRenderer>;
//
// #[derive(Deref, DerefMut, Component, Reflect, Clone)]
// pub struct RendererState<T: Send + Sync + 'static>(pub T);
//
// #[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
// pub struct BevyRenderer;
//
// impl Renderer for BevyRenderer {
//    type NodeId = Entity;
//    type NodeTree = World;
//
//    #[cfg(not(target_arch = "wasm32"))]
//    type Task<T: Send + 'static> = Task<T>;
//
//    #[cfg(target_arch = "wasm32")]
//    type Task<T: Send + 'static> = ();
//
//    #[cfg(any(target_arch = "wasm32", not(feature = "multi-threaded")))]
//    fn spawn_task<T: Send + 'static>(
//       future: impl Future<Output = T> + Send + 'static,
//    ) -> Self::Task<T> {
//       bevy_tasks::AsyncComputeTaskPool::get().spawn(future)
//    }
//
//    #[cfg(target_arch = "wasm32")]
//    fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> Self::Task<T> {
//       bevy_tasks::AsyncComputeTaskPool::get().spawn(future);
//    }
// }
//
// #[derive(Clone)]
// pub struct BevyDeferredWorldScoped {
//    cmd_sender: CmdSender,
// }
//
// impl DeferredNodeTreeScoped<BevyRenderer> for BevyDeferredWorldScoped {
//    fn scoped(&self, f: impl FnOnce(&mut RendererWorld<BevyRenderer>) + Send + 'static) {
//       self.cmd_sender.add(move |world: &mut World| f(world))
//    }
// }
