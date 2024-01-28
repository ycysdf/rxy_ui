use crate::style_sheets::StyleSheets;
use crate::ApplyStyleSheets;
use futures_lite::{FutureExt, StreamExt};
use rxy_bevy::BevyRenderer;
use rxy_core::{
    rx, Either, Reactive, RebuildFnReceiver, Renderer, ViewMember, XFuture,
};
use std::future::Future;

pub trait IntoViewMemberWithOrigin<R>
where
    R: Renderer,
{
    type Origin;
    type VM: ViewMember<R>;

    fn into_view_member(self) -> Self::VM;
}

impl<T> IntoViewMemberWithOrigin<BevyRenderer> for T
where
    T: StyleSheets<BevyRenderer>,
{
    type Origin = ApplyStyleSheets<T>;
    type VM = ApplyStyleSheets<Self>;

    fn into_view_member(self) -> Self::VM {
        ApplyStyleSheets(self)
    }
}

impl<T> IntoViewMemberWithOrigin<BevyRenderer> for ApplyStyleSheets<T>
where
    T: StyleSheets<BevyRenderer>,
{
    type Origin = Self;
    type VM = Self;

    fn into_view_member(self) -> Self::VM {
        self
    }
}

impl<R, VM> IntoViewMemberWithOrigin<R> for Option<VM>
where
    R: Renderer,
    VM: IntoViewMemberWithOrigin<R>,
{
    type Origin = VM::Origin;
    type VM = Option<VM::VM>;

    fn into_view_member(self) -> Self::VM {
        self.map(|n| n.into_view_member())
    }
}

impl<LSS, RSS> IntoViewMemberWithOrigin<BevyRenderer>
    for Either<ApplyStyleSheets<LSS>, ApplyStyleSheets<RSS>>
where
    LSS: StyleSheets<BevyRenderer>,
    RSS: StyleSheets<BevyRenderer>,
{
    type Origin = ApplyStyleSheets<Either<LSS, RSS>>;
    type VM = Self;

    fn into_view_member(self) -> Self::VM {
        self
    }
}

impl<R, F, VM> IntoViewMemberWithOrigin<R> for Reactive<F, VM>
where
    R: Renderer,
    F: Fn() -> VM + Send + 'static,
    VM: IntoViewMemberWithOrigin<R> + Send,
{
    type Origin = VM::Origin;
    type VM = Reactive<Box<dyn Fn() -> VM::VM + Send>, VM::VM>;

    fn into_view_member(self) -> Self::VM {
        rx(Box::new(move || self.0().into_view_member()))
    }
}

impl<R, VM> IntoViewMemberWithOrigin<R> for RebuildFnReceiver<R, VM>
where
    R: Renderer,
    VM: IntoViewMemberWithOrigin<R> + 'static,
{
    type Origin = VM::Origin;

    type VM = RebuildFnReceiver<R, VM::VM>;

    fn into_view_member(self) -> Self::VM {
        self.map(|n| n.into_view_member())
    }
}

impl<R, VM> IntoViewMemberWithOrigin<R> for futures_lite::stream::Boxed<VM>
where
    R: Renderer,
    VM: IntoViewMemberWithOrigin<R> + 'static,
{
    type Origin = VM::Origin;

    type VM = futures_lite::stream::Boxed<VM::VM>;

    fn into_view_member(self) -> Self::VM {
        self.map(|n| n.into_view_member()).boxed()
    }
}

impl<R, T> IntoViewMemberWithOrigin<R> for XFuture<T>
where
    R: Renderer,
    T: Future + Send + 'static,
    T::Output: IntoViewMemberWithOrigin<R> + Send + 'static,
{
    type Origin = <T::Output as IntoViewMemberWithOrigin<R>>::Origin;

    type VM = XFuture<futures_lite::future::Boxed<<T::Output as IntoViewMemberWithOrigin<R>>::VM>>;

    fn into_view_member(self) -> Self::VM {
        XFuture(async move { self.0.await.into_view_member() }.boxed())
    }
}

/*    impl<R, F, VM> ViewMemberOrigin<R> for Builder<R, F>
where
    F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> VM + Send + 'static,
    R: Renderer,
    VM: ViewMemberOrigin<R>,
{
    type Origin = VM::Origin;
    type VM = Builder<R, Box<dyn FnOnce(ViewMemberCtx<R>, BuildFlags) -> VM::VM + Send + 'static>>;

    fn into_view_member(self) -> Self::VM {
        style_builder(Box::new(move |ctx, flags| self.0(ctx, flags).into_view_member()))
    }
}*/
