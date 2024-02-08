use crate::element::ElementAttr;
use crate::{
    AttrValue, ElementAttrMember, IntoViewMember, NodeTree, Renderer, ViewMember, ViewMemberCtx,
    ViewMemberIndex, ViewMemberOrigin,
};
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct ElementAttrViewMember<R, EA>(pub EA::Value)
where
    R: Renderer,
    EA: ElementAttr<R>;

impl<R, EA> PartialEq for ElementAttrViewMember<R, EA>
where
    R: Renderer,
    EA: ElementAttr<R>,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<R, EA> ElementAttrViewMember<R, EA>
where
    R: Renderer,
    EA: ElementAttr<R>,
{
    pub fn new(value: EA::Value) -> Self {
        ElementAttrViewMember(value)
    }
}

impl<R, EA> ElementAttrMember<R> for ElementAttrViewMember<R, EA>
where
    R: Renderer,
    EA: ElementAttr<R>,
{
    type EA = EA;
    type Attr<OEA: ElementAttr<R, Value = EA::Value>> = ElementAttrViewMember<R, OEA>;

    fn into_other_attr<OEA: ElementAttr<R, Value = EA::Value>>(self) -> Self::Attr<OEA> {
        ElementAttrViewMember(self.0)
    }
}

impl<R, EA> IntoViewMember<R> for ElementAttrViewMember<R, EA>
where
    R: Renderer,
    EA: ElementAttr<R>,
{
    type Member = Self;

    fn into_member(self) -> Self {
        self
    }
}

// impl<R, EA> ViewMemberOrigin<R> for ElementAttrViewMember<R, EA>
// where
//     R: Renderer,
//     EA: ElementAttr<R>,
// {
//     type Origin = Self;
// }

impl<R, EA> ViewMember<R> for ElementAttrViewMember<R, EA>
where
    R: Renderer,
    EA: ElementAttr<R>,
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

pub struct WW<T, M>(pub T, PhantomData<M>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttrValueWrapper<T>(pub T);

impl<EA, R, T> IntoViewMember<R> for WW<T, EA>
where
    R: Renderer,
    EA: ElementAttr<R>,
    T: Into<AttrValueWrapper<EA::Value>>,
{
    type Member = ElementAttrViewMember<R, EA>;

    fn into_member(self) -> ElementAttrViewMember<R, EA> {
        ElementAttrViewMember(self.0.into().0)
    }
}
impl<R, T> IntoViewMember<R> for T
where
    R: Renderer,
    T: Into<AttrValueWrapper<T>>,
{
    type Member = AttrValueWrapper<T>;

    fn into_member(self) -> AttrValueWrapper<T> {
        self.into()
    }
}
