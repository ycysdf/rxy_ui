use std::any::TypeId;
use std::future::Future;

use bevy_ecs::prelude::Entity;
use bevy_ecs::world::World;

pub use layout::*;
pub use node_bundles::*;
use rxy_core::{Element, NodeTree, Renderer, RendererNodeId};
pub use transform::*;
pub use visibility::*;

pub mod common_renderer;
pub mod elements;

mod attr_values;
pub mod attrs;
mod composite_attrs;
mod layout;
mod node_bundles;
pub mod node_tree;
mod taffy;
mod tailwind_attrs;
mod transform;
pub mod ui_node;
mod view_key;
mod visibility;

use crate::user_event::UserEventSender;
pub use composite_attrs::*;
pub use tailwind_attrs::*;

rxy_bevy_ecs::define_bevy_ces_renderer! {
   #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
   pub struct NativeRenderer;
}

pub type NativeElement<E, VM> = Element<NativeRenderer, E, VM>;

impl EcsRendererAssociated for NativeRenderer {
   type AssociatedNodeTreeScoped = UserEventSender;

   cfg_if::cfg_if! {
       if #[cfg(feature = "tokio")] {
           type AssociatedTask<T: Send + 'static> = tokio::task::JoinHandle<T>;
       } /*else if #[cfg(feature = "compio")] {
           type AssociatedTask<T: Send + 'static> = compio::runtime::Task<T>;
           } */else {
           compile_error!("No runtime feature enabled");
       }
   }
}

impl EcsRenderer for NativeRenderer {
   fn spawn_task<T: Send + 'static>(
      future: impl Future<Output = T> + Send + 'static,
   ) -> <Self as Renderer>::Task<T> {
      cfg_if::cfg_if! {
         if #[cfg(feature = "tokio")] {
             tokio::task::spawn(future)
         } /*else if #[cfg(feature = "compio")] {
             compio::runtime::spawn(future)
             } */else {
             panic!("No runtime feature enabled")
         }
      }
   }

   fn deferred_world_scoped(world: &World) -> Self::AssociatedNodeTreeScoped {
      world.non_send_resource::<UserEventSender>().clone()
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

   fn scoped_type_state<S: Send + Sync + Clone + 'static, U>(
      world: &World,
      type_id: TypeId,
      f: impl FnOnce(Option<&S>) -> U,
   ) -> U {
      {
         #[cfg(feature = "reflect")]
         f(world
            .resource::<bevy_ecs::prelude::AppTypeRegistry>()
            .read()
            .get_type_data::<S>(type_id))
      }
      {
         #[cfg(not(feature = "reflect"))]
         unimplemented!()
      }
   }
}
