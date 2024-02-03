use core::fmt::Debug;
use core::hash::Hash;

use crate::{MaybeFromReflect, MaybeReflect, MaybeSend, MaybeSync, MaybeTypePath, NodeTree, Renderer, RendererNodeId, RendererWorld, ViewCtx};

// impl MutableView for tuple ?

pub trait MutableView<R: Renderer>: MaybeSend + 'static {
    type Key: MutableViewKey<R>;

    fn no_placeholder_when_no_rebuild() -> bool;

    /// .
    /// if state_node_id is none then will_rebuild is false
    fn build(self, ctx: ViewCtx<R>, placeholder_node_id: Option<RendererNodeId<R>>) -> Self::Key;

    fn rebuild(
        self,
        ctx: ViewCtx<R>,
        key: Self::Key,
        placeholder_node_id: RendererNodeId<R>,
    ) -> Option<Self::Key>;
}

pub trait MutableViewKey<R: Renderer>:
    MaybeReflect
    + MaybeFromReflect
    + MaybeTypePath
    + MaybeSend
    + MaybeSync
    + Clone
    + Hash
    + Debug
    + 'static
{
    fn remove(self, world: &mut RendererWorld<R>);

    fn insert_before(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
    );

    fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool);

    fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>>;

    // Unlike ViewKey, you can change it
    fn state_node_id(&self) -> Option<RendererNodeId<R>>;
}

#[allow(unused_variables)]
impl<R> MutableViewKey<R> for ()
where
    R: Renderer,
{
    fn remove(self, world: &mut RendererWorld<R>) {}

    fn insert_before(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
    ) {
    }

    fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool) {}

    fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
        None
    }

    fn state_node_id(&self) -> Option<RendererNodeId<R>> {
        None
    }
}

pub struct MutableKeySelfStatedWrapper<T>(pub T);

pub fn mutable_view_rebuild<R: Renderer, V: MutableView<R>>(
    view: V,
    ctx: ViewCtx<R>,
    state_node_id: R::NodeId,
) {
    let key = ctx
        .world
        .get_node_state_ref::<MutableKeySelfStatedWrapper<V::Key>>(&state_node_id)
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
            Some(state_node_id.clone()),
        );
        Some(key)
    };
    if let Some(new_key) = new_key {
        ctx.world
            .set_node_state(&state_node_id, MutableKeySelfStatedWrapper(new_key));
    }
}
