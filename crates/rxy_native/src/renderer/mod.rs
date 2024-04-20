use std::future::Future;

use bevy_ecs::prelude::Entity;
use bevy_ecs::world::World;

pub use layout::*;
pub use node_bundles::*;
use rxy_core::{Element, NodeTree, Renderer};
pub use transform::*;
pub use visibility::*;

pub mod common_renderer;
pub mod elements;

mod layout;
mod node_bundles;
pub mod node_tree;
mod taffy;
mod transform;
pub mod ui_node;
mod view_key;
mod visibility;
pub mod attrs;
mod attr_values;
mod composite_attrs;
mod tailwind_attrs;

pub use composite_attrs::*;
pub use tailwind_attrs::*;

pub type NativeElement<E, VM> = Element<NativeRenderer, E, VM>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct NativeRenderer;

impl Renderer for NativeRenderer {
   type NodeId = Entity;
   type NodeTree = World;

   cfg_if::cfg_if! {
       if #[cfg(feature = "tokio")] {
           type Task<T: Send + 'static> = tokio::task::JoinHandle<T>;
       } else if #[cfg(feature = "compio")] {
           type Task<T: Send + 'static> = compio::runtime::Task<T>;
       } else {
           compile_error!("No runtime feature enabled");
       }
   }

   fn spawn_task<T: Send + 'static>(
      future: impl Future<Output = T> + Send + 'static,
   ) -> Self::Task<T> {
      cfg_if::cfg_if! {
         if #[cfg(feature = "tokio")] {
             tokio::task::spawn(future)
         } else if #[cfg(feature = "compio")] {
             compio::runtime::spawn(future)
         } else {
             panic!("No runtime feature enabled")
         }
      }
   }
}
