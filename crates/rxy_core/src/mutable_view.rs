use core::fmt::Debug;
use core::hash::Hash;

use crate::{
    MaybeFromReflect, MaybeReflect, MaybeTypePath, Renderer, RendererNodeId, RendererWorld, ViewCtx,
};

pub trait MutableView<R: Renderer>: Send + 'static {
    type Key: MutableViewKey<R>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        will_rebuild: bool,
        state_node_id: RendererNodeId<R>,
    ) -> Self::Key;

    fn rebuild(
        self,
        ctx: ViewCtx<R>,
        key: Self::Key,
        state_node_id: RendererNodeId<R>,
    ) -> Option<Self::Key>;
}

pub trait MutableViewKey<R: Renderer>:
    MaybeReflect + MaybeFromReflect + MaybeTypePath + Send + Sync + Clone + Hash + Debug + 'static
{
    fn remove(self, world: &mut RendererWorld<R>, state_node_id: &RendererNodeId<R>);

    fn insert_before(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
        state_node_id: &RendererNodeId<R>,
    );

    fn set_visibility(
        &self,
        world: &mut RendererWorld<R>,
        hidden: bool,
        state_node_id: &RendererNodeId<R>,
    );

    fn first_node_id(
        &self,
        world: &RendererWorld<R>,
        state_node_id: &RendererNodeId<R>,
    ) -> Option<RendererNodeId<R>>;
}

impl<R> MutableViewKey<R> for ()
where
    R: Renderer,
{
    fn remove(self, _world: &mut RendererWorld<R>, _state_node_id: &RendererNodeId<R>) {}

    fn insert_before(
        &self,
        _world: &mut RendererWorld<R>,
        _parent: Option<&RendererNodeId<R>>,
        _before_node_id: Option<&RendererNodeId<R>>,
        _state_node_id: &RendererNodeId<R>,
    ) {
    }

    fn set_visibility(
        &self,
        _world: &mut RendererWorld<R>,
        _hidden: bool,
        _state_node_id: &RendererNodeId<R>,
    ) {
    }

    fn first_node_id(
        &self,
        _world: &RendererWorld<R>,
        _state_node_id: &RendererNodeId<R>,
    ) -> Option<RendererNodeId<R>> {
        None
    }
}

pub struct MutableKeySelfStatedWrapper<T>(pub T);

pub fn mutable_view_rebuild<R: Renderer, V: MutableView<R>>(
    view: V,
    ctx: ViewCtx<R>,
    state_node_id: R::NodeId,
) {
    let key =
        R::get_state_ref::<MutableKeySelfStatedWrapper<V::Key>>(ctx.world, &state_node_id)
            .map(|n| &n.0)
            .cloned();
    let new_key = if let Some(key) = key {
        view.rebuild(
            ViewCtx {
                world: &mut *ctx.world,
                parent: ctx.parent.clone(),
            },
            key,
            state_node_id.clone(),
        )
    } else {
        let key = view.build(
            ViewCtx {
                world: &mut *ctx.world,
                parent: ctx.parent.clone(),
            },
            true,
            state_node_id.clone(),
        );
        Some(key)
    };
    if let Some(new_key) = new_key {
        R::set_state(
            ctx.world,
            &state_node_id,
            MutableKeySelfStatedWrapper(new_key),
        );
    }
}
