use alloc::borrow::Cow;
use core::any::TypeId;
use core::fmt::Debug;
use core::future::Future;

use oneshot::Sender;

use crate::{
    MaybeReflect, MaybeTypePath, MutableView, RendererElementType, View,
    ViewKey, ViewMember,
};

pub type RendererNodeId<R> = <R as Renderer>::NodeId;
pub type RendererWorld<R> = <R as Renderer>::World;

pub struct ViewCtx<'a, R: Renderer> {
    pub world: &'a mut RendererWorld<R>,
    pub parent: RendererNodeId<R>,
}

pub type ViewMemberIndex = u32;

pub struct ViewMemberCtx<'a, R: Renderer> {
    pub index: ViewMemberIndex,
    // todo: remove type_id
    pub type_id: TypeId,
    pub world: &'a mut RendererWorld<R>,
    pub node_id: RendererNodeId<R>,
}
pub enum ContainerType {
    // UiContainer,
    SlotContainer,
}

pub trait DeferredWorldScoped<R>: Clone + Send + Sync + Sized + 'static
where
    R: Renderer,
{
    fn deferred_world(&self, f: impl FnOnce(&mut RendererWorld<R>) + Send + 'static);
}

// todo: refactor, extract methods to World
pub trait Renderer:
    MaybeReflect + MaybeTypePath + Clone + Debug + Send + Sync + Sized + 'static
{
    type NodeId: ViewKey<Self>;
    type World;

    type ViewReBuilder: ViewReBuilder<Self>;
    type MemberReBuilder: MemberReBuilder<Self>;
    type Task<T: Send + 'static>: Send + 'static;
    fn get_or_insert_default_state<'a, S: Default + Send + Sync + 'static>(
        world: &'a mut RendererWorld<Self>,
        node_id: &RendererNodeId<Self>,
    ) -> &'a mut S;
    fn state_scoped<S: Send + Sync + 'static, U>(
        world: &mut RendererWorld<Self>,
        node_id: &RendererNodeId<Self>,
        f: impl FnOnce(&mut RendererWorld<Self>, &mut S) -> U,
    ) -> Option<U> {
        Self::take_state(world, node_id).map(|mut n| {
            let r = f(world, &mut n);
            Self::set_state(world, node_id, n);
            r
        })
    }
    fn try_state_scoped<S: Send + Sync + 'static, U>(
        world: &mut RendererWorld<Self>,
        node_id: &RendererNodeId<Self>,
        f: impl FnOnce(&mut RendererWorld<Self>, Option<&mut S>) -> U,
    ) -> U {
        match Self::take_state(world, node_id) {
            Some(mut n) => {
                let r = f(world, Some(&mut n));
                Self::set_state(world, node_id, n);
                r
            }
            None => {
                
                f(world, None)
            }
        }
    }
    fn deferred_world_scoped(world: &mut RendererWorld<Self>) -> impl DeferredWorldScoped<Self>;

    fn get_view_re_builder(ctx: ViewCtx<Self>) -> Self::ViewReBuilder;

    fn get_container_node_id(
        world: &mut RendererWorld<Self>,
        container_type: ContainerType,
    ) -> RendererNodeId<Self>;

    fn get_member_re_builder(
        ctx: ViewMemberCtx<Self>,
        is_already_build: bool,
    ) -> Self::MemberReBuilder;

    fn spawn_placeholder(
        world: &mut RendererWorld<Self>,
        name: impl Into<Cow<'static, str>>,
        parent: Option<&RendererNodeId<Self>>,
        reserve_node_id: Option<RendererNodeId<Self>>,
    ) -> RendererNodeId<Self>;

    fn spawn_data_node(world: &mut RendererWorld<Self>) -> RendererNodeId<Self>;

    fn get_parent(
        world: &RendererWorld<Self>,
        node_id: &RendererNodeId<Self>,
    ) -> Option<RendererNodeId<Self>>;

    fn ensure_spawn(world: &mut RendererWorld<Self>, reserve_node_id: RendererNodeId<Self>);
    fn spawn_node<E: RendererElementType<Self>>(
        world: &mut RendererWorld<Self>,
        parent: Option<RendererNodeId<Self>>,
        reserve_node_id: Option<RendererNodeId<Self>>,
    ) -> RendererNodeId<Self> {
        E::spawn(world, parent, reserve_node_id)
    }

    fn exist_node_id(world: &mut RendererWorld<Self>, node_id: &RendererNodeId<Self>) -> bool;

    fn get_state_mut<'w, S: Send + Sync + 'static>(
        world: &'w mut RendererWorld<Self>,
        node_id: &RendererNodeId<Self>,
    ) -> Option<&'w mut S>;

    fn get_state_ref<'w, S: Send + Sync + 'static>(
        world: &'w RendererWorld<Self>,
        node_id: &RendererNodeId<Self>,
    ) -> Option<&'w S>;

    fn take_state<S: Send + Sync + 'static>(
        world: &mut RendererWorld<Self>,
        node_id: &RendererNodeId<Self>,
    ) -> Option<S>;

    fn set_state<S: Send + Sync + 'static>(
        world: &mut RendererWorld<Self>,
        node_id: &RendererNodeId<Self>,
        state: S,
    );

    fn reserve_node_id(world: &mut RendererWorld<Self>) -> RendererNodeId<Self>;
    fn spawn_and_detach(future: impl Future<Output = ()> + Send + 'static);
    fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> Self::Task<T>;

    fn remove_node(world: &mut RendererWorld<Self>, node_id: &RendererNodeId<Self>);

    fn insert_before(
        world: &mut RendererWorld<Self>,
        parent: Option<&RendererNodeId<Self>>,
        before_node_id: Option<&RendererNodeId<Self>>,
        inserted_node_ids: &[RendererNodeId<Self>],
    );

    fn set_visibility(
        world: &mut RendererWorld<Self>,
        hidden: bool,
        node_id: &RendererNodeId<Self>,
    );

    fn get_is_hidden(world: &RendererWorld<Self>, node_id: &RendererNodeId<Self>) -> bool;
}

pub enum BuildState<K> {
    AlreadyBuild(K),
    NoBuild(Sender<K>),
    NoBuildWithReserveKey(K),
}

impl<K> BuildState<K> {
    pub fn try_clone(&self) -> Option<Self>
    where
        K: Clone,
    {
        match self {
            BuildState::AlreadyBuild(n) => Some(BuildState::AlreadyBuild(n.clone())),
            BuildState::NoBuild(_) => None,
            BuildState::NoBuildWithReserveKey(n) => {
                Some(BuildState::NoBuildWithReserveKey(n.clone()))
            }
        }
    }
}

pub trait ViewReBuilder<R: Renderer>: Send {
    fn rebuild<V: View<R>>(&self, view: V, is_build: BuildState<V::Key>);
    fn mutable_rebuild<V: MutableView<R>>(&mut self, view: V, node_id: &R::NodeId);
}

pub trait MemberReBuilder<R: Renderer>: Send {
    fn rebuild<VM: ViewMember<R>>(&self, member: VM, index: ViewMemberIndex);
}
