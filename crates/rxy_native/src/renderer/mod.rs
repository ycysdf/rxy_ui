use rxy_core::{Element, NodeTree, Renderer};
use std::future::Future;
use bevy_ecs::prelude::Entity;
use bevy_ecs::world::World;
use winit::event_loop::EventLoopProxy;

use crate::tt::XyWindow;

pub mod node_tree;
mod view_key;
pub mod common_renderer;
pub mod elements;
mod layout;
pub mod ui_node;
pub mod geometry;
mod visibility;
pub mod node_bundles;

pub type NativeElement<E, VM> = Element<NativeRenderer, E, VM>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
