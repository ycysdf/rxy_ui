use crate::element::ElementAttr;
use crate::{
    x_future, Either, MaybeSend, Renderer, ViewMember, ViewMemberCtx, ViewMemberIndex, XFuture,
};
use core::future::Future;
use core::marker::PhantomData;

pub trait ElementAttrMember<R>: ViewMember<R>
where
    R: Renderer,
{
    type EA: ElementAttr<R>;

    type Attr<OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>>: ElementAttrMember<
        R,
        EA = OEA,
    >;

    fn into_other_attr<OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>>(
        self,
    ) -> Self::Attr<OEA>;
}

impl<R, EA, T> ElementAttrMember<R> for Option<T>
where
    R: Renderer,
    EA: ElementAttr<R>,
    T: ElementAttrMember<R, EA = EA>,
{
    type EA = T::EA;
    type Attr<OEA: ElementAttr<R, Value = EA::Value>> = Option<T::Attr<OEA>>;

    fn into_other_attr<OEA: ElementAttr<R, Value = EA::Value>>(self) -> Self::Attr<OEA> {
        self.map(|n| n.into_other_attr())
    }
}

impl<R, EA, LT, RT> ElementAttrMember<R> for Either<LT, RT>
where
    R: Renderer,
    EA: ElementAttr<R>,
    LT: ElementAttrMember<R, EA = EA>,
    RT: ElementAttrMember<R, EA = EA>,
{
    type EA = LT::EA;
    type Attr<OEA: ElementAttr<R, Value = EA::Value>> = Either<LT::Attr<OEA>, RT::Attr<OEA>>;

    fn into_other_attr<OEA: ElementAttr<R, Value = EA::Value>>(self) -> Self::Attr<OEA> {
        match self {
            Either::Left(l) => Either::Left(l.into_other_attr()),
            Either::Right(r) => Either::Right(r.into_other_attr()),
        }
    }
}

pub struct ElementAttrMemberWrapper<T, M>(pub T, PhantomData<M>);

impl<T, M> ElementAttrMemberWrapper<T, M> {
    pub fn new(value: T) -> Self {
        Self(value, PhantomData)
    }
}

impl<R, TO, VM, EA, OOEA> ElementAttrMember<R> for ElementAttrMemberWrapper<XFuture<TO>, OOEA>
where
    EA: ElementAttr<R>,
    OOEA: ElementAttr<R,Value=EA::Value>,
    R: Renderer,
    TO: Future<Output = VM> + MaybeSend + 'static,
    VM: ElementAttrMember<R, EA = EA>,
{
    type EA = OOEA;
    type Attr<OEA: ElementAttr<R, Value = EA::Value>> = ElementAttrMemberWrapper<XFuture<TO>, OEA>;

    fn into_other_attr<OEA: ElementAttr<R, Value = EA::Value>>(self) -> Self::Attr<OEA> {
        ElementAttrMemberWrapper::new(self.0)
    }
}

impl<R, TO, VM, OEA, EA> ViewMember<R> for ElementAttrMemberWrapper<XFuture<TO>, OEA>
where
    OEA: ElementAttr<R, Value = EA::Value>,
    EA: ElementAttr<R>,
    R: Renderer,
    TO: Future<Output = VM> + MaybeSend + 'static,
    VM: ElementAttrMember<R, EA = EA>,
{
    fn count() -> ViewMemberIndex {
        VM::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
        VM::unbuild(ctx, view_removed)
    }

    fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
        let reactive = x_future(async move { self.0 .0.await.into_other_attr::<OEA>() });
        reactive.build(ctx, will_rebuild)
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        let reactive = x_future(async move { self.0 .0.await.into_other_attr::<OEA>() });
        reactive.rebuild(ctx)
    }
}

impl<R, T, EA> ElementAttrMember<R> for XFuture<T>
where
    R: Renderer,
    EA: ElementAttr<R>,
    T: Future + Send + 'static,
    T::Output: ElementAttrMember<R, EA = EA> + Send + 'static,
{
    type EA = EA;
    type Attr<OEA: ElementAttr<R, Value = EA::Value>> = ElementAttrMemberWrapper<Self, OEA>;

    fn into_other_attr<OEA: ElementAttr<R, Value = EA::Value>>(self) -> Self::Attr<OEA> {
        ElementAttrMemberWrapper::new(self)
    }
}

#[cfg(test)]
mod test {
    use crate::element::{ElementAttr, ElementAttrMember};
    use crate::{Either, Renderer};
    use static_assertions::*;

    fn test<
        R: Renderer,
        EA1: ElementAttr<R, Value = u32>,
        EA2: ElementAttr<R, Value = bool>,
        VM1: ElementAttrMember<R, EA = EA1>,
        VM2: ElementAttrMember<R, EA = EA2>,
    >() {
        assert_not_impl_all!(Either<VM1,VM2>: ElementAttrMember<()>);
        assert_impl_all!(Option<VM1>: ElementAttrMember<()>);
        assert_impl_all!(Option<Option<VM2 >>: ElementAttrMember<()>);
        assert_impl_all!(Either<VM1,VM1>: ElementAttrMember<()>);
        assert_impl_all!(Either<VM1,Either<VM1,VM1>>: ElementAttrMember<()>);
        assert_impl_all!(Option<Either<VM1,VM1>>: ElementAttrMember<()>);
    }
}
