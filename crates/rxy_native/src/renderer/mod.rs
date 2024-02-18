use hecs::World;
use rxy_core::{NodeTree, Renderer};
use std::future::Future;
use winit::event_loop::EventLoopProxy;

use crate::tt::XyWindow;

pub mod node_tree;
mod view_key;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeRenderer;

impl Renderer for NativeRenderer {
    type NodeId = hecs::Entity;
    type NodeTree = XyWindow;
    type StateMutRef<'a, S: Send + Sync + 'static> = hecs::RefMut<'a, S>
        where
            Self: 'a;
    type StateRef<'a, S: Send + Sync + 'static> = hecs::Ref<'a, S>
        where
            Self: 'a;

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
