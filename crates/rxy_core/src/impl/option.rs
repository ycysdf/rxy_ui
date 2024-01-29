use crate::{
    to_mutable, virtual_container, IntoView, MutableView, MutableViewKey, Renderer, RendererNodeId,
    RendererWorld, ToMutableWrapper, ViewCtx, ViewKey, ViewMember, ViewMemberCtx, VirtualContainer,
};

impl<R, V> MutableView<R> for Option<V>
where
    R: Renderer,
    V: MutableView<R>,
{
    type Key = Option<V::Key>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        will_rebuild: bool,
        state_node_id: RendererNodeId<R>,
    ) -> Self::Key {
        self.map(|n| n.build(ctx, will_rebuild, state_node_id))
    }

    fn rebuild(
        self,
        ctx: ViewCtx<R>,
        key: Self::Key,
        state_node_id: RendererNodeId<R>,
    ) -> Option<Self::Key> {
        match (key, self) {
            (Some(key), Some(new)) => {
                new.rebuild(ctx, key, state_node_id);
                None
            }
            (key, None) => {
                key.remove(&mut *ctx.world, &state_node_id);
                Some(None)
            }
            (None, Some(new)) => {
                let parent = ctx.parent;
                let key = new.build(
                    ViewCtx {
                        world: &mut *ctx.world,
                        parent: parent.clone(),
                    },
                    true,
                    state_node_id.clone(),
                );
                key.insert_before(
                    &mut *ctx.world,
                    Some(&parent),
                    Some(&state_node_id),
                    &state_node_id,
                );
                // if R::get_is_hidden(&mut *ctx.world, &state_node_id) {
                //     key.set_visibility(&mut *ctx.world, true, &state_node_id);
                // }
                Some(Some(key))
            }
        }
    }
}

impl<K, R> MutableViewKey<R> for Option<K>
where
    R: Renderer,
    K: MutableViewKey<R>,
{
    fn remove(self, world: &mut RendererWorld<R>, state_node_id: &RendererNodeId<R>) {
        if let Some(n) = self {
            n.remove(world, state_node_id)
        }
    }

    fn insert_before(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
        state_node_id: &RendererNodeId<R>,
    ) {
        if let Some(n) = self.as_ref() {
            n.insert_before(world, parent, before_node_id, state_node_id)
        }
    }

    fn set_visibility(
        &self,
        world: &mut RendererWorld<R>,
        hidden: bool,
        state_node_id: &RendererNodeId<R>,
    ) {
        if let Some(n) = self.as_ref() {
            n.set_visibility(world, hidden, state_node_id)
        }
    }

    fn first_node_id(
        &self,
        world: &RendererWorld<R>,
        state_node_id: &RendererNodeId<R>,
    ) -> Option<RendererNodeId<R>> {
        self.as_ref()
            .and_then(|n| n.first_node_id(world, state_node_id))
    }
}

impl<R, IV> IntoView<R> for Option<IV>
where
    R: Renderer,
    IV: IntoView<R> + Send + 'static,
{
    type View = VirtualContainer<R, Option<ToMutableWrapper<IV::View>>>;

    fn into_view(self) -> Self::View {
        virtual_container(
            self.map(|n| to_mutable(n.into_view())),
            "[Option Placeholder]",
        )
    }
}

impl<R, VM> ViewMember<R> for Option<VM>
where
    R: Renderer,
    VM: ViewMember<R>,
{
    fn count() -> u8 {
        VM::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>) {
        VM::unbuild(ctx);
    }

    fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
        if let Some(n) = self {
            n.build(ctx, will_rebuild)
        }
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        match self {
            None => {
                VM::unbuild(ctx);
            }
            Some(vm) => {
                vm.build(ctx, true);
            }
        }
    }
}

impl<K, R> ViewKey<R> for Option<K>
where
    R: Renderer,
    K: ViewKey<R>,
{
    fn remove(self, world: &mut RendererWorld<R>) {
        if let Some(n) = self {
            n.remove(world)
        }
    }

    fn insert_before(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
    ) {
        if let Some(n) = self.as_ref() {
            n.insert_before(world, parent, before_node_id)
        }
    }

    fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool) {
        if let Some(n) = self.as_ref() {
            n.set_visibility(world, hidden)
        }
    }

    fn state_node_id(&self) -> Option<RendererNodeId<R>> {
        match self {
            Some(n) => n.state_node_id(),
            None => None,
        }
    }

    fn reserve_key(world: &mut RendererWorld<R>, will_rebuild: bool) -> Self {
        Some(K::reserve_key(world, will_rebuild))
    }

    fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
        self.as_ref().and_then(|n| n.first_node_id(world))
    }
}
