use crate::{
    MutableView, MutableViewKey, Renderer, RendererNodeId, RendererWorld, View, ViewCtx, ViewKey,
};

pub fn to_mutable<T>(t: T) -> ToMutableWrapper<T> {
    ToMutableWrapper(t)
}

#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[derive(Hash, Clone, Debug)]
pub struct ToMutableWrapper<T>(pub T);

impl<VK: ViewKey<R>, R: Renderer> MutableViewKey<R> for ToMutableWrapper<VK> {
    fn remove(self, world: &mut RendererWorld<R>, _state_node_id: &RendererNodeId<R>) {
        self.0.remove(world)
    }

    fn insert_before(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
        _state_node_id: &RendererNodeId<R>,
    ) {
        self.0.insert_before(world, parent, before_node_id)
    }

    fn set_visibility(
        &self,
        world: &mut RendererWorld<R>,
        hidden: bool,
        _state_node_id: &RendererNodeId<R>,
    ) {
        self.0.set_visibility(world, hidden)
    }

    fn first_node_id(
        &self,
        world: &RendererWorld<R>,
        _state_node_id: &RendererNodeId<R>,
    ) -> Option<RendererNodeId<R>> {
        self.0.first_node_id(world)
    }
}

impl<R, V> MutableView<R> for ToMutableWrapper<V>
where
    R: Renderer,
    V: View<R>,
{
    type Key = ToMutableWrapper<V::Key>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        will_rebuild: bool,
        _state_node_id: RendererNodeId<R>,
    ) -> Self::Key {
        ToMutableWrapper(self.0.build(ctx, None, will_rebuild))
    }

    fn rebuild(
        self,
        ctx: ViewCtx<R>,
        key: Self::Key,
        _state_node_id: RendererNodeId<R>,
    ) -> Option<Self::Key> {
        self.0.rebuild(ctx, key.0);
        None
    }
}
