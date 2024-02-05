use crate::RendererWorld;
use crate::{
    DeferredNodeTreeScoped, NodeTree, Renderer, RendererNodeId, ViewMember, ViewMemberCtx,
    ViewMemberIndex,
};
use crate::{MaybeSend, MaybeSync};
use alloc::borrow::Cow;
use core::future::Future;
use core::hint::unreachable_unchecked;

#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TestRenderer;

impl<R> DeferredNodeTreeScoped<R> for TestRenderer
where
    R: Renderer,
{
    fn scoped(&self, f: impl FnOnce(&mut RendererWorld<R>) + MaybeSend + 'static) {
        unsafe { unreachable_unchecked() }
    }
}

pub struct TestNodeTree;

#[allow(unused_variables)]
impl NodeTree<TestRenderer> for TestNodeTree {
    fn deferred_world_scoped(&mut self) -> impl DeferredNodeTreeScoped<TestRenderer> {
        TestRenderer
    }

    fn get_node_state_mut<S: MaybeSend + MaybeSync + 'static>(
        &mut self,
        node_id: &RendererNodeId<TestRenderer>,
    ) -> Option<&mut S> {
        unsafe { unreachable_unchecked() }
    }

    fn get_node_state_ref<S: MaybeSend + MaybeSync + 'static>(
        &self,
        node_id: &RendererNodeId<TestRenderer>,
    ) -> Option<&S> {
        unsafe { unreachable_unchecked() }
    }

    fn take_node_state<S: MaybeSend + MaybeSync + 'static>(
        &mut self,
        node_id: &RendererNodeId<TestRenderer>,
    ) -> Option<S> {
        unsafe { unreachable_unchecked() }
    }

    fn set_node_state<S: MaybeSend + MaybeSync + 'static>(
        &mut self,
        node_id: &RendererNodeId<TestRenderer>,
        state: S,
    ) {
        unsafe { unreachable_unchecked() }
    }

    fn exist_node_id(&mut self, node_id: &RendererNodeId<TestRenderer>) -> bool {
        unsafe { unreachable_unchecked() }
    }

    fn reserve_node_id(&mut self) -> RendererNodeId<TestRenderer> {
        unsafe { unreachable_unchecked() }
    }

    fn spawn_placeholder(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        parent: Option<&RendererNodeId<TestRenderer>>,
        reserve_node_id: Option<RendererNodeId<TestRenderer>>,
    ) -> RendererNodeId<TestRenderer> {
        unsafe { unreachable_unchecked() }
    }

    fn ensure_spawn(&mut self, reserve_node_id: RendererNodeId<TestRenderer>) {
        unsafe { unreachable_unchecked() }
    }

    fn spawn_empty_node(
        &mut self,
        parent: Option<&RendererNodeId<TestRenderer>>,
        reserve_node_id: Option<RendererNodeId<TestRenderer>>,
    ) -> RendererNodeId<TestRenderer> {
        unsafe { unreachable_unchecked() }
    }

    fn spawn_data_node(&mut self) -> RendererNodeId<TestRenderer> {
        unsafe { unreachable_unchecked() }
    }

    fn get_parent(
        &self,
        node_id: &RendererNodeId<TestRenderer>,
    ) -> Option<RendererNodeId<TestRenderer>> {
        unsafe { unreachable_unchecked() }
    }

    fn remove_node(&mut self, node_id: &RendererNodeId<TestRenderer>) {
        unsafe { unreachable_unchecked() }
    }

    fn insert_before(
        &mut self,
        parent: Option<&RendererNodeId<TestRenderer>>,
        before_node_id: Option<&RendererNodeId<TestRenderer>>,
        inserted_node_ids: &[RendererNodeId<TestRenderer>],
    ) {
        unsafe { unreachable_unchecked() }
    }

    fn set_visibility(&mut self, hidden: bool, node_id: &RendererNodeId<TestRenderer>) {
        unsafe { unreachable_unchecked() }
    }

    fn get_visibility(&self, node_id: &RendererNodeId<TestRenderer>) -> bool {
        unsafe { unreachable_unchecked() }
    }
}

impl Renderer for TestRenderer {
    type NodeId = ();
    type NodeTree = TestNodeTree;
    type Task<T: MaybeSend + 'static> = ();

    fn spawn<T: MaybeSend + 'static>(
        future: impl Future<Output = T> + MaybeSend + 'static,
    ) -> Self::Task<T> {
        unsafe { unreachable_unchecked() }
    }
}

pub struct TestVM<T>(T);

#[allow(unused_variables)]
impl<R, T> ViewMember<R> for TestVM<T>
where
    T: 'static,
    R: Renderer,
{
    fn count() -> ViewMemberIndex {
        unsafe { unreachable_unchecked() }
    }

    fn unbuild(ctx: ViewMemberCtx<R>, _view_removed: bool) {
        unsafe { unreachable_unchecked() }
    }

    fn build(self, ctx: ViewMemberCtx<R>, _will_rebuild: bool) {
        unsafe { unreachable_unchecked() }
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        unsafe { unreachable_unchecked() }
    }
}
