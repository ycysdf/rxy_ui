use alloc::borrow::Cow;
use bevy_utils::synccell::SyncCell;
use core::fmt::Debug;
use core::future::Future;

use crate::{MaybeReflect, MaybeTypePath, RendererElementType, ViewKey};

pub type RendererNodeId<R> = <R as Renderer>::NodeId;
pub type RendererWorld<R> = <R as Renderer>::NodeTree;

pub struct ViewCtx<'a, R: Renderer> {
    pub world: &'a mut RendererWorld<R>,
    pub parent: RendererNodeId<R>,
}

pub type ViewMemberIndex = u32;

pub struct ViewMemberCtx<'a, R: Renderer> {
    pub index: ViewMemberIndex,
    pub world: &'a mut RendererWorld<R>,
    pub node_id: RendererNodeId<R>,
}

pub trait DeferredNodeTreeScoped<R>: Clone + Send + Sync + Sized + 'static
where
    R: Renderer,
{
    fn scoped(&self, f: impl FnOnce(&mut RendererWorld<R>) + Send + 'static);
}

pub trait Renderer:
    MaybeReflect + MaybeTypePath + Clone + Debug + Send + Sync + Sized + 'static
{
    type NodeId: ViewKey<Self>;
    type NodeTree: NodeTree<Self>;

    type Task<T: Send + 'static>: Send + 'static;

    fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> Self::Task<T>;
}

pub trait NodeTree<R>
where
    R: Renderer<NodeTree= Self>,
{
    fn deferred_world_scoped(&mut self) -> impl DeferredNodeTreeScoped<R>;
    fn get_node_state_mut<S: Send + Sync + 'static>(
        &mut self,
        node_id: &RendererNodeId<R>,
    ) -> Option<&mut S>;

    fn get_node_state_ref<S: Send + Sync + 'static>(
        &self,
        node_id: &RendererNodeId<R>,
    ) -> Option<&S>;

    fn take_node_state<S: Send + Sync + 'static>(
        &mut self,
        node_id: &RendererNodeId<R>,
    ) -> Option<S>;

    fn set_node_state<S: Send + Sync + 'static>(&mut self, node_id: &RendererNodeId<R>, state: S);

    fn exist_node_id(&mut self, node_id: &RendererNodeId<R>) -> bool;

    fn reserve_node_id(&mut self) -> RendererNodeId<R>;

    fn spawn_placeholder(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        parent: Option<&RendererNodeId<R>>,
        reserve_node_id: Option<RendererNodeId<R>>,
    ) -> RendererNodeId<R>;

    fn ensure_spawn(&mut self, reserve_node_id: RendererNodeId<R>);

    fn spawn_empty_node(
        &mut self,
        parent: Option<&RendererNodeId<R>>,
        reserve_node_id: Option<RendererNodeId<R>>,
    ) -> RendererNodeId<R>;

    fn spawn_data_node(&mut self) -> RendererNodeId<R>;

    fn spawn_node<E: RendererElementType<R>>(
        &mut self,
        parent: Option<RendererNodeId<R>>,
        reserve_node_id: Option<RendererNodeId<R>>,
    ) -> RendererNodeId<R> {
        E::spawn(self, parent, reserve_node_id)
    }

    fn get_parent(&self, node_id: &RendererNodeId<R>) -> Option<RendererNodeId<R>>;

    fn remove_node(&mut self, node_id: &RendererNodeId<R>);

    fn insert_before(
        &mut self,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
        inserted_node_ids: &[RendererNodeId<R>],
    );

    fn set_visibility(&mut self, hidden: bool, node_id: &RendererNodeId<R>);

    fn get_visibility(&self, node_id: &RendererNodeId<R>) -> bool;

    fn node_state_scoped<S: Send + Sync + 'static, U>(
        &mut self,
        node_id: &RendererNodeId<R>,
        f: impl FnOnce(&mut Self, &mut S) -> U,
    ) -> Option<U> {
        Self::take_node_state(self, node_id).map(|mut n| {
            let r = f(self, &mut n);
            Self::set_node_state(self, node_id, n);
            r
        })
    }
    fn try_state_scoped<S: Send + Sync + 'static, U>(
        &mut self,
        node_id: &RendererNodeId<R>,
        f: impl FnOnce(&mut Self, Option<&mut S>) -> U,
    ) -> U {
        match Self::take_node_state(self, node_id) {
            Some(mut n) => {
                let r = f(self, Some(&mut n));
                Self::set_node_state(self, node_id, n);
                r
            }
            None => f(self, None),
        }
    }

    fn get_or_insert_default_node_state<S: Default + Send + Sync + 'static>(
        &mut self,
        node_id: &RendererNodeId<R>,
    ) -> &mut S {
        if self.get_node_state_mut::<S>(node_id).is_none() {
            self.set_node_state(node_id, S::default());
        }
        self.get_node_state_mut::<S>(node_id).unwrap()
    }
}

pub struct TaskState<R>(#[allow(dead_code)] pub SyncCell<R::Task<()>>)
where
    R: Renderer;

impl<R> TaskState<R>
where
    R: Renderer,
{
    pub fn new(task: R::Task<()>) -> Self {
        Self(SyncCell::new(task))
    }
}
