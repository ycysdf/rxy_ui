use core::ops::{Deref, DerefMut};
use std::borrow::Cow;

use rxy_core::{
    prelude::{DeferredNodeTreeScoped, ElementAttrType},
    AttrIndex, NodeTree, RendererNodeId, RendererWorld,
};
use winit::event_loop::EventLoopProxy;

use crate::tt::{UserEventSender, XyWindow};

use super::NativeRenderer;

impl NodeTree<NativeRenderer> for XyWindow {
    fn prepare_set_attr_and_get_is_init(
        &mut self,
        node_id: &RendererNodeId<NativeRenderer>,
        attr_index: AttrIndex,
    ) -> bool {
        todo!()
    }

    fn build_attr<A: ElementAttrType<NativeRenderer>>(
        &mut self,
        node_id: RendererNodeId<NativeRenderer>,
        value: A::Value,
    ) {
        todo!()
    }

    fn rebuild_attr<A: ElementAttrType<NativeRenderer>>(
        &mut self,
        node_id: RendererNodeId<NativeRenderer>,
        value: A::Value,
    ) {
        todo!()
    }

    fn unbuild_attr<A: ElementAttrType<NativeRenderer>>(
        &mut self,
        node_id: RendererNodeId<NativeRenderer>,
    ) {
        todo!()
    }

    fn deferred_world_scoped(&mut self) -> impl DeferredNodeTreeScoped<NativeRenderer> {
        todo!()
    }

    fn get_node_state_mut<S: Send + Sync + 'static>(
        &mut self,
        node_id: &RendererNodeId<NativeRenderer>,
    ) -> Option<NativeRenderer::StateMutRef<'_, S>> {
        let result = self.world.get::<&mut S>(*node_id).ok();
        todo!()
    }

    fn get_node_state_ref<S: Send + Sync + 'static>(
        &self,
        node_id: &RendererNodeId<NativeRenderer>,
    ) -> Option<NativeRenderer::StateRef<'_, S>> {
        todo!()
    }

    fn take_node_state<S: Send + Sync + 'static>(
        &mut self,
        node_id: &RendererNodeId<NativeRenderer>,
    ) -> Option<S> {
        todo!()
    }

    fn set_node_state<S: Send + Sync + 'static>(
        &mut self,
        node_id: &RendererNodeId<NativeRenderer>,
        state: S,
    ) {
        todo!()
    }

    fn exist_node_id(&mut self, node_id: &RendererNodeId<NativeRenderer>) -> bool {
        todo!()
    }

    fn reserve_node_id(&mut self) -> RendererNodeId<NativeRenderer> {
        todo!()
    }

    fn spawn_placeholder(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        parent: Option<&RendererNodeId<NativeRenderer>>,
        reserve_node_id: Option<RendererNodeId<NativeRenderer>>,
    ) -> RendererNodeId<NativeRenderer> {
        todo!()
    }

    fn ensure_spawn(&mut self, reserve_node_id: RendererNodeId<NativeRenderer>) {
        todo!()
    }

    fn spawn_empty_node(
        &mut self,
        parent: Option<&RendererNodeId<NativeRenderer>>,
        reserve_node_id: Option<RendererNodeId<NativeRenderer>>,
    ) -> RendererNodeId<NativeRenderer> {
        todo!()
    }

    fn spawn_data_node(&mut self) -> RendererNodeId<NativeRenderer> {
        todo!()
    }

    fn get_parent(
        &self,
        node_id: &RendererNodeId<NativeRenderer>,
    ) -> Option<RendererNodeId<NativeRenderer>> {
        todo!()
    }

    fn remove_node(&mut self, node_id: &RendererNodeId<NativeRenderer>) {
        todo!()
    }

    fn insert_before(
        &mut self,
        parent: Option<&RendererNodeId<NativeRenderer>>,
        before_node_id: Option<&RendererNodeId<NativeRenderer>>,
        inserted_node_ids: &[RendererNodeId<NativeRenderer>],
    ) {
        todo!()
    }

    fn set_visibility(&mut self, hidden: bool, node_id: &RendererNodeId<NativeRenderer>) {
        todo!()
    }

    fn get_visibility(&self, node_id: &RendererNodeId<NativeRenderer>) -> bool {
        todo!()
    }
}

impl DeferredNodeTreeScoped<NativeRenderer> for UserEventSender {
    fn scoped(&self, f: impl FnOnce(&mut RendererWorld<NativeRenderer>) + Send + 'static) {
        self.send(f);
    }
}
