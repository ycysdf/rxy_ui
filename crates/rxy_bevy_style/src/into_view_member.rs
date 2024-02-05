use crate::style_sheets::StyleSheets;
use crate::ApplyStyleSheets;
use futures_lite::FutureExt;
use rxy_bevy::BevyRenderer;
use rxy_core::{
    rx, x_future, Either, MaybeSend, Reactive, RebuildFnReceiver, Renderer, ViewMember,
    ViewMemberCtx, ViewMemberIndex, XFuture,
};
use std::future::Future;
use std::marker::PhantomData;

pub trait ViewMemberWithOrigin<R, VM>: 'static
where
    R: Renderer,
    VM: ViewMember<R>,
{
    type Origin: ViewMember<R>;

    fn into_view_member(self) -> VM;
}

impl<T> ViewMemberWithOrigin<BevyRenderer, Self> for ApplyStyleSheets<T>
where
    T: StyleSheets<BevyRenderer>,
{
    type Origin = ApplyStyleSheets<T>;

    fn into_view_member(self) -> Self {
        self
    }
}

impl<T> ViewMemberWithOrigin<BevyRenderer, ApplyStyleSheets<T>> for T
where
    T: StyleSheets<BevyRenderer>,
{
    type Origin = ApplyStyleSheets<T>;

    fn into_view_member(self) -> ApplyStyleSheets<T> {
        ApplyStyleSheets(self)
    }
}

impl<R, VMO, VM> ViewMemberWithOrigin<R, Option<VM>> for Option<VMO>
where
    R: Renderer,
    VM: ViewMember<R>,
    VMO: ViewMemberWithOrigin<R, VM>,
{
    type Origin = VMO::Origin;

    fn into_view_member(self) -> Option<VM> {
        self.map(|n| n.into_view_member())
    }
}

impl<R, LVM, RVM, LVMO, RVMO> ViewMemberWithOrigin<R, Either<LVM, RVM>> for Either<LVMO, RVMO>
where
    R: Renderer,
    LVM: ViewMember<R>,
    RVM: ViewMember<R>,
    LVMO: ViewMemberWithOrigin<R, LVM>,
    RVMO: ViewMemberWithOrigin<R, RVM>,
{
    type Origin = Either<LVMO::Origin, RVMO::Origin>;

    fn into_view_member(self) -> Either<LVM, RVM> {
        match self {
            Either::Left(n) => Either::Left(n.into_view_member()),
            Either::Right(n) => Either::Right(n.into_view_member()),
        }
    }
}

pub struct ViewMemberWithOriginWrapper<T, M>(pub T, PhantomData<M>);

impl<T, M> ViewMemberWithOriginWrapper<T, M> {
    #[inline]
    pub fn new(t: T) -> Self {
        Self(t, Default::default())
    }
}

impl<R, F, VMO, VM> ViewMember<R> for ViewMemberWithOriginWrapper<Reactive<F, VMO>, VM>
where
    R: Renderer,
    F: Fn() -> VMO + MaybeSend + 'static,
    VM: ViewMember<R>,
    VMO: ViewMemberWithOrigin<R, VM> + MaybeSend,
{
    fn count() -> ViewMemberIndex {
        VM::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
        VM::unbuild(ctx, view_removed)
    }

    fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
        let reactive = rx(move || self.0 .0().into_view_member());
        reactive.build(ctx, will_rebuild)
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        let reactive = rx(move || self.0 .0().into_view_member());
        reactive.rebuild(ctx)
    }
}

impl<R, F, VMO, VM> ViewMemberWithOrigin<R, ViewMemberWithOriginWrapper<Self, VM>>
    for Reactive<F, VMO>
where
    R: Renderer,
    F: Fn() -> VMO + MaybeSend + 'static,
    VM: ViewMember<R>,
    VMO: ViewMemberWithOrigin<R, VM> + MaybeSend,
{
    type Origin = VMO::Origin;

    fn into_view_member(self) -> ViewMemberWithOriginWrapper<Self, VM> {
        ViewMemberWithOriginWrapper::new(self)
    }
}

impl<R, VM, VMO> ViewMemberWithOrigin<R, RebuildFnReceiver<R, VM>> for RebuildFnReceiver<R, VMO>
where
    R: Renderer,
    VM: ViewMember<R>,
    VMO: ViewMemberWithOrigin<R, VM> + 'static,
{
    type Origin = VMO::Origin;

    fn into_view_member(self) -> RebuildFnReceiver<R, VM> {
        self.map(|n| n.into_view_member())
    }
}

impl<R, VM, VMO> ViewMemberWithOrigin<R, futures_lite::stream::Boxed<VM>>
    for futures_lite::stream::Boxed<VMO>
where
    R: Renderer,
    VM: ViewMember<R>,
    VMO: ViewMemberWithOrigin<R, VM> + 'static,
{
    type Origin = VMO::Origin;

    fn into_view_member(self) -> futures_lite::stream::Boxed<VM> {
        use futures_lite::StreamExt;
        self.map(|n| n.into_view_member()).boxed()
    }
}

impl<R, TO, VM> ViewMember<R> for ViewMemberWithOriginWrapper<XFuture<TO>, VM>
where
    R: Renderer,
    TO: Future + MaybeSend + 'static,
    VM: ViewMember<R>,
    TO::Output: ViewMemberWithOrigin<R, VM> + MaybeSend + 'static,
{
    fn count() -> ViewMemberIndex {
        VM::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
        VM::unbuild(ctx, view_removed)
    }

    fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
        let reactive = x_future(async move { self.0 .0.await.into_view_member() });
        reactive.build(ctx, will_rebuild)
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        let reactive = x_future(async move { self.0 .0.await.into_view_member() });
        reactive.rebuild(ctx)
    }
}

impl<R, TO, VM> ViewMemberWithOrigin<R, ViewMemberWithOriginWrapper<Self, VM>> for XFuture<TO>
where
    R: Renderer,
    TO: Future + MaybeSend + 'static,
    VM: ViewMember<R>,
    TO::Output: ViewMemberWithOrigin<R, VM> + MaybeSend + 'static,
{
    type Origin = <TO::Output as ViewMemberWithOrigin<R, VM>>::Origin;

    fn into_view_member(self) -> ViewMemberWithOriginWrapper<Self, VM> {
        ViewMemberWithOriginWrapper::new(self)
    }
}

impl<R, VMO, VM> ViewMemberWithOrigin<R, futures_lite::future::Boxed<VM>>
    for futures_lite::future::Boxed<VMO>
where
    R: Renderer,
    VM: ViewMember<R>,
    VMO: ViewMemberWithOrigin<R, VM> + MaybeSend + 'static,
{
    type Origin = VMO::Origin;

    fn into_view_member(self) -> futures_lite::future::Boxed<VM> {
        let pin = (async move { self.await.into_view_member() }).boxed();
        pin
    }
}

// pub trait IntoViewMemberWithOrigin<R>
//     where
//         R: Renderer,
// {
//     type Origin;
//     type VM: ViewMember<R>;
//
//     fn into_view_member(self) -> Self::VM;
// }
//
// impl<T> IntoViewMemberWithOrigin<BevyRenderer> for T
//     where
//         T: StyleSheets<BevyRenderer>,
// {
//     type Origin = ApplyStyleSheets<T>;
//     type VM = ApplyStyleSheets<Self>;
//
//     fn into_view_member(self) -> Self::VM {
//         ApplyStyleSheets(self)
//     }
// }
//
// impl<T> IntoViewMemberWithOrigin<BevyRenderer> for ApplyStyleSheets<T>
//     where
//         T: StyleSheets<BevyRenderer>,
// {
//     type Origin = Self;
//     type VM = Self;
//
//     fn into_view_member(self) -> Self::VM {
//         self
//     }
// }
//
// impl<R, VM> IntoViewMemberWithOrigin<R> for Option<VM>
//     where
//         R: Renderer,
//         VM: IntoViewMemberWithOrigin<R>,
// {
//     type Origin = VM::Origin;
//     type VM = Option<VM::VM>;
//
//     fn into_view_member(self) -> Self::VM {
//         self.map(|n| n.into_view_member())
//     }
// }
//
// impl<LSS, RSS> IntoViewMemberWithOrigin<BevyRenderer>
// for Either<ApplyStyleSheets<LSS>, ApplyStyleSheets<RSS>>
//     where
//         LSS: StyleSheets<BevyRenderer>,
//         RSS: StyleSheets<BevyRenderer>,
// {
//     type Origin = ApplyStyleSheets<Either<LSS, RSS>>;
//     type VM = Self;
//
//     fn into_view_member(self) -> Self::VM {
//         self
//     }
// }
//
// impl<R, F, VM> IntoViewMemberWithOrigin<R> for Reactive<F, VM>
//     where
//         R: Renderer,
//         F: Fn() -> VM + MaybeSend + 'static,
//         VM: IntoViewMemberWithOrigin<R> + MaybeSend,
// {
//     type Origin = VM::Origin;
//     type VM = Reactive<Box<dyn Fn() -> VM::VM + MaybeSend>, VM::VM>;
//
//     fn into_view_member(self) -> Self::VM {
//         rx(Box::new(move || self.0().into_view_member()))
//     }
// }
//
// impl<R, VM> IntoViewMemberWithOrigin<R> for RebuildFnReceiver<R, VM>
//     where
//         R: Renderer,
//         VM: IntoViewMemberWithOrigin<R> + 'static,
// {
//     type Origin = VM::Origin;
//
//     type VM = RebuildFnReceiver<R, VM::VM>;
//
//     fn into_view_member(self) -> Self::VM {
//         self.map(|n| n.into_view_member())
//     }
// }
//
// impl<R, VM> IntoViewMemberWithOrigin<R> for futures_lite::stream::Boxed<VM>
//     where
//         R: Renderer,
//         VM: IntoViewMemberWithOrigin<R> + 'static,
// {
//     type Origin = VM::Origin;
//
//     type VM = futures_lite::stream::Boxed<VM::VM>;
//
//     fn into_view_member(self) -> Self::VM {
//         self.map(|n| n.into_view_member()).boxed()
//     }
// }
//
// impl<R, T> IntoViewMemberWithOrigin<R> for XFuture<T>
//     where
//         R: Renderer,
//         T: Future + MaybeSend + 'static,
//         T::Output: IntoViewMemberWithOrigin<R> + MaybeSend + 'static,
// {
//     type Origin = <T::Output as IntoViewMemberWithOrigin<R>>::Origin;
//
//     type VM = XFuture<futures_lite::future::Boxed<<T::Output as IntoViewMemberWithOrigin<R>>::VM>>;
//
//     fn into_view_member(self) -> Self::VM {
//         XFuture(async move { self.0.await.into_view_member() }.boxed())
//     }
// }

/*    impl<R, F, VM> ViewMemberOrigin<R> for Builder<R, F>
where
    F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> VM + MaybeSend + 'static,
    R: Renderer,
    VM: ViewMemberOrigin<R>,
{
    type Origin = VM::Origin;
    type VM = Builder<R, Box<dyn FnOnce(ViewMemberCtx<R>, BuildFlags) -> VM::VM + MaybeSend + 'static>>;

    fn into_view_member(self) -> Self::VM {
        style_builder(Box::new(move |ctx, flags| self.0(ctx, flags).into_view_member()))
    }
}*/
