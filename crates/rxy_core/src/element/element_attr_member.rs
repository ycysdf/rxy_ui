use crate::element::ElementAttr;
use crate::{
    x_future, Either, InnerIvmToVm, MaybeSend, Renderer, ViewMember, ViewMemberCtx,
    ViewMemberIndex, ViewMemberOrigin, XFuture,
};
use core::future::Future;
use core::marker::PhantomData;
use futures_lite::FutureExt;
use futures_lite::StreamExt;

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
    OOEA: ElementAttr<R, Value = EA::Value>,
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

impl<R, T> ElementAttrMember<R> for futures_lite::future::Boxed<T>
where
    R: Renderer,
    T: ElementAttrMember<R> + 'static,
{
    type EA = T::EA;
    type Attr<OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>> =
        futures_lite::future::Boxed<T::Attr<OEA>>;

    fn into_other_attr<OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>>(
        self,
    ) -> Self::Attr<OEA> {
        async move { self.await.into_other_attr() }.boxed()
    }
}

impl<R, TO, VM, OEA, EA> ViewMemberOrigin<R> for ElementAttrMemberWrapper<XFuture<TO>, OEA>
where
    OEA: ElementAttr<R, Value = EA::Value>,
    EA: ElementAttr<R>,
    R: Renderer,
    TO: Future<Output = VM> + MaybeSend + 'static,
    VM: ElementAttrMember<R, EA = EA> + ViewMemberOrigin<R>,
{
    type Origin = VM::Origin;
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

impl<R, T> ElementAttrMember<R> for futures_lite::stream::Boxed<T>
where
    R: Renderer,
    T: ElementAttrMember<R> + 'static,
{
    type EA = T::EA;
    type Attr<OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>> =
        futures_lite::stream::Boxed<T::Attr<OEA>>;

    fn into_other_attr<OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>>(
        self,
    ) -> Self::Attr<OEA> {
        self.map(|n| n.into_other_attr()).boxed()
    }
}

#[cfg(feature = "xy_reactive")]
const _: () = {
    use crate::Reactive;
    use crate::{IntoViewMember, MaybeSync};
    use xy_reactive::prelude::Memo;
    use xy_reactive::prelude::ReadSignal;
    use xy_reactive::prelude::RwSignal;
    use xy_reactive::prelude::SignalGet;

    impl<R, T, EA, OEA, VM> ViewMemberOrigin<R> for ElementAttrMemberWrapper<T, OEA>
    where
        R: Renderer,
        T: SignalGet<Value = VM> + MaybeSend + 'static,
        VM: ElementAttrMember<R, EA = EA> + MaybeSync + Clone + ViewMemberOrigin<R>,
        EA: ElementAttr<R>,
        OEA: ElementAttr<R, Value = EA::Value>,
    {
        type Origin = VM::Origin;
    }

    impl<R, T, EA, OEA, VM> ViewMember<R> for ElementAttrMemberWrapper<T, OEA>
    where
        R: Renderer,
        T: SignalGet<Value = VM> + MaybeSend + 'static,
        VM: ElementAttrMember<R, EA = EA> + MaybeSync + Clone,
        EA: ElementAttr<R>,
        OEA: ElementAttr<R, Value = EA::Value>,
    {
        fn count() -> ViewMemberIndex {
            VM::count()
        }

        fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
            VM::unbuild(ctx, view_removed)
        }

        fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
            let reactive = crate::rx(move || self.0.get().into_other_attr::<OEA>());
            reactive.build(ctx, will_rebuild)
        }

        fn rebuild(self, ctx: ViewMemberCtx<R>) {
            let reactive = crate::rx(move || self.0.get().into_other_attr::<OEA>());
            reactive.rebuild(ctx)
        }
    }

    impl<R, F, EA, OEA, VM> ViewMemberOrigin<R> for ElementAttrMemberWrapper<Reactive<F, VM>, OEA>
    where
        R: Renderer,
        F: Fn() -> VM + MaybeSend + 'static,
        VM: ElementAttrMember<R, EA = EA> + MaybeSend + ViewMemberOrigin<R>,
        EA: ElementAttr<R>,
        OEA: ElementAttr<R, Value = EA::Value>,
    {
        type Origin = VM::Origin;
    }

    impl<R, F, EA, OEA, VM> ViewMember<R> for ElementAttrMemberWrapper<Reactive<F, VM>, OEA>
    where
        R: Renderer,
        F: Fn() -> VM + MaybeSend + 'static,
        VM: ElementAttrMember<R, EA = EA> + MaybeSend,
        EA: ElementAttr<R>,
        OEA: ElementAttr<R, Value = EA::Value>,
    {
        fn count() -> ViewMemberIndex {
            VM::count()
        }

        fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
            VM::unbuild(ctx, view_removed)
        }

        fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
            let reactive = crate::rx(move || self.0 .0().into_other_attr::<OEA>());
            reactive.build(ctx, will_rebuild)
        }

        fn rebuild(self, ctx: ViewMemberCtx<R>) {
            let reactive = crate::rx(move || self.0 .0().into_other_attr::<OEA>());
            reactive.rebuild(ctx)
        }
    }

    impl<R, T, EA, OOEA, VM> ElementAttrMember<R> for ElementAttrMemberWrapper<T, OOEA>
    where
        R: Renderer,
        T: SignalGet<Value = VM> + MaybeSend + 'static,
        VM: ElementAttrMember<R, EA = EA> + MaybeSync + Clone,
        EA: ElementAttr<R>,
        OOEA: ElementAttr<R, Value = EA::Value>,
        OOEA: ElementAttr<R>,
    {
        type EA = OOEA;
        type Attr<OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>> =
            ElementAttrMemberWrapper<T, OEA>;

        fn into_other_attr<OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>>(
            self,
        ) -> Self::Attr<OEA> {
            ElementAttrMemberWrapper::new(self.0)
        }
    }

    impl<R, F, EA, OOEA, VM> ElementAttrMember<R> for ElementAttrMemberWrapper<Reactive<F, VM>, OOEA>
    where
        R: Renderer,
        F: Fn() -> VM + MaybeSend + 'static,
        VM: ElementAttrMember<R, EA = EA> + MaybeSend,
        EA: ElementAttr<R>,
        OOEA: ElementAttr<R, Value = EA::Value>,
        OOEA: ElementAttr<R>,
    {
        type EA = OOEA;
        type Attr<OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>> =
            ElementAttrMemberWrapper<Reactive<F, VM>, OEA>;

        fn into_other_attr<OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>>(
            self,
        ) -> Self::Attr<OEA> {
            ElementAttrMemberWrapper::new(self.0)
        }
    }

    impl<R, F, VM> ElementAttrMember<R> for Reactive<F, VM>
    where
        R: Renderer,
        F: Fn() -> VM + MaybeSend + 'static,
        VM: ElementAttrMember<R>,
    {
        type EA = VM::EA;
        type Attr<OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>> =
            ElementAttrMemberWrapper<Self, OEA>;

        fn into_other_attr<OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>>(
            self,
        ) -> Self::Attr<OEA> {
            ElementAttrMemberWrapper::new(self)
        }
    }

    macro_rules! impl_element_view_member_for_signal_get {
        ($ident:ident) => {
            impl<R, VM> ElementAttrMember<R> for $ident<VM>
            where
                R: Renderer,
                VM: ElementAttrMember<R> + MaybeSync + Clone,
            {
                type EA = VM::EA;
                type Attr<OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>> =
                    ElementAttrMemberWrapper<Self, OEA>;

                fn into_other_attr<
                    OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>,
                >(
                    self,
                ) -> Self::Attr<OEA> {
                    ElementAttrMemberWrapper::new(self)
                }
            }
        };
    }
    impl_element_view_member_for_signal_get!(Memo);
    impl_element_view_member_for_signal_get!(ReadSignal);
    impl_element_view_member_for_signal_get!(RwSignal);

    impl<R, F, IVM, VM> ElementAttrMember<R> for InnerIvmToVm<Reactive<F, IVM>, VM>
    where
        R: Renderer,
        F: Fn() -> IVM + MaybeSend + 'static,
        IVM: IntoViewMember<R, Member=VM> + MaybeSend + MaybeSync + Clone + 'static,
        VM: ElementAttrMember<R>,
    {
        type EA = VM::EA;
        type Attr<OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>> =
            Reactive<Box<dyn Fn() -> VM::Attr<OEA> + MaybeSend>, VM::Attr<OEA>>;

        fn into_other_attr<OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>>(
            self,
        ) -> Self::Attr<OEA> {
            crate::rx(Box::new(move || {
                self.0 .0().into_member().into_other_attr::<OEA>()
            }))
        }
    }

    impl<R, T, VM, IVM> ElementAttrMember<R> for InnerIvmToVm<T, VM>
    where
        R: Renderer,
        T: SignalGet<Value = IVM> + MaybeSend + 'static,
        IVM: IntoViewMember<R,Member= VM> + MaybeSync + Clone + 'static,
        VM: ElementAttrMember<R>,
    {
        type EA = VM::EA;
        type Attr<OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>> =
            Reactive<Box<dyn Fn() -> VM::Attr<OEA> + MaybeSend>, VM::Attr<OEA>>;

        fn into_other_attr<OEA: ElementAttr<R, Value = <Self::EA as ElementAttr<R>>::Value>>(
            self,
        ) -> Self::Attr<OEA> {
            crate::rx(Box::new(move || {
                self.0.get().into_member().into_other_attr::<OEA>()
            }))
        }
    }
};

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
        assert_not_impl_all!(Either<VM2,VM2>: ElementAttrMember<()>);
        assert_impl_all!(Option<VM1>: ElementAttrMember<()>);
        assert_impl_all!(Option<Option<VM2 >>: ElementAttrMember<()>);
        assert_impl_all!(Either<VM1,VM1>: ElementAttrMember<()>);
        assert_impl_all!(Either<VM1,Either<VM1,VM1>>: ElementAttrMember<()>);
        assert_impl_all!(Option<Either<VM1,VM1>>: ElementAttrMember<()>);
    }
}
