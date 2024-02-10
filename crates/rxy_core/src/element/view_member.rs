use crate::element::ElementAttr;
use crate::{
    AttrValue, ElementAttrMember, NodeTree, Renderer, ViewMember, ViewMemberCtx,
    ViewMemberIndex, ViewMemberOrigin
};

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
    #[inline]
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


impl<R, EA> ViewMemberOrigin<R> for ElementAttrViewMember<R, EA>
where
    R: Renderer,
    EA: ElementAttr<R>,
{
    type Origin = Self;
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
// impl<R, T, EA> Mapper<MapToAttrMarker<EA>> for MapToAttrMarkerWrapper<T, EA>
// where
//     T: Into<AttrValueWrapper<EA::Value>>,
//     EA: ElementAttr<R>,
//     R: Renderer,
// {
//     type To = ElementAttrViewMember<R, EA>;
//
//     fn map(self) -> Self::To {
//         ElementAttrViewMember(self.0.into().0)
//     }
// }

// pub struct MapToAttrMarkerWrapper<T, M = ()>(pub T, PhantomData<M>);


// impl<R, T> XNest<R> for MapToAttrMarkerWrapper<T, ()>
// where
//     R: Renderer,
//     T: XNest<R>,
// {
//     type InnerMember = T::InnerMember;
//     type MapMember<M> = T::MapMember<M>;
//
//     fn map_inner<M>(self) -> Self::MapMember<M> {
//         self.0.map_inner::<M>()
//     }
// }

// impl<EA, R, T> XNest<R> for MapToAttrMarkerWrapper<T, EA>
// where
//     R: Renderer,
//     EA: ElementAttr<R>,
//     T: Into<AttrValueWrapper<EA::Value>> + 'static,
// {
//     type InnerMember = Self;
//     type MapMember<M> = ElementAttrViewMember<R, EA>;
//
//     fn map_inner<M>(self) -> Self::MapMember<M> {
//         <Self as Mapper<MapToAttrMarker<R>>>::map(self)
//     }
//
//     // type MapMember = ElementAttrViewMember<R, EA>;
//     //
//     // fn into_member(self) -> ElementAttrViewMember<R, EA> {
//     //     ElementAttrViewMember(self.0.into().0)
//     // }
// }
