use crate::element::{ElementAttr, ElementAttrMember};
use crate::{IntoViewMember, NodeTree, Renderer, ViewMember, ViewMemberCtx, ViewMemberIndex};

pub struct ElementAttrViewMember<R, EA>(pub EA::Value)
where
    R: Renderer,
    EA: ElementAttr<R>;

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

impl<R, EA> IntoViewMember<R, Self> for ElementAttrViewMember<R, EA>
where
    R: Renderer,
    EA: ElementAttr<R>,
{
    fn into_member(self) -> Self {
        self
    }
}

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

pub struct AttrValueWrapper<R, EA>(pub EA::Value)
where
    R: Renderer,
    EA: ElementAttr<R>;

impl<EA, R, T> IntoViewMember<R, ElementAttrViewMember<R, EA>> for T
where
    R: Renderer,
    EA: ElementAttr<R>,
    T: Into<AttrValueWrapper<R, EA>>,
{
    fn into_member(self) -> ElementAttrViewMember<R, EA> {
        ElementAttrViewMember(self.into().0)
    }
}
