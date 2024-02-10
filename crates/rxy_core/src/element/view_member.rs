use crate::element::ElementAttrType;
use core::marker::PhantomData;

pub trait ElementAttrMember<R, EA>:
    ViewMember<R> + ViewMemberOrigin<R, Origin = ElementAttr<R, EA>>
where
    R: Renderer,
    EA: ElementAttrType<R>,
{
}

impl<T, R, EA> ElementAttrMember<R, EA> for T
where
    T: ViewMember<R> + ViewMemberOrigin<R, Origin = ElementAttr<R, EA>>,
    R: Renderer,
    EA: ElementAttrType<R>,
{
}

#[derive(Clone, Debug)]
pub struct ElementAttr<R, EA, M = ()>(pub EA::Value, PhantomData<M>)
where
    R: Renderer,
    EA: ElementAttrType<R>;

use crate::{
    AttrValue, NodeTree, Renderer, ViewMember, ViewMemberCtx, ViewMemberIndex, ViewMemberOrigin,
};

impl<R, EA> PartialEq for ElementAttr<R, EA>
where
    R: Renderer,
    EA: ElementAttrType<R>,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<R, EA> ElementAttr<R, EA>
where
    R: Renderer,
    EA: ElementAttrType<R>,
{
    #[inline]
    pub fn new(value: EA::Value) -> Self {
        Self(value, Default::default())
    }
}

impl<R, EA> ViewMemberOrigin<R> for ElementAttr<R, EA>
where
    R: Renderer,
    EA: ElementAttrType<R>,
{
    type Origin = Self;
}

impl<R, EA> ViewMember<R> for ElementAttr<R, EA>
where
    R: Renderer,
    EA: ElementAttrType<R>,
{
    fn count() -> ViewMemberIndex {
        1
    }

    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
        if view_removed {
            return;
        }
        ctx.world.unbuild_attr::<EA>(ctx.node_id);
    }

    fn build(self, ctx: ViewMemberCtx<R>, _will_rebuild: bool) {
        ctx.world.build_attr::<EA>(ctx.node_id, self.0);
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        ctx.world.rebuild_attr::<EA>(ctx.node_id, self.0);
    }
}
