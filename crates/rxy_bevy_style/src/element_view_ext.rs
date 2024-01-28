use crate::{into_view_member::IntoViewMemberWithOrigin, ApplyStyleSheets};
use crate::style_sheets::StyleSheets;
use futures_lite::StreamExt;
use rxy_bevy::BevyRenderer;
use rxy_core::{
    rx, style_builder, x_future, x_stream, BuildFlags, Builder, ElementView, Reactive,
    ViewMemberCtx, XFuture, XStream,
};
use std::future::{Future, IntoFuture};

pub trait ElementStyleExt: ElementView<BevyRenderer> {
    fn style<VM, SS>(self, style_sheets: VM) -> Self::AddMember<VM::VM>
    where
        VM: IntoViewMemberWithOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(style_sheets.into_view_member())
    }

    fn style_rx<VM, SS>(
        self,
        f: impl Fn() -> VM + Send + 'static,
    ) -> Self::AddMember<Reactive<impl Fn() -> VM::VM + Send + 'static, VM::VM>>
    where
        VM: IntoViewMemberWithOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(rx(move || f().into_view_member()))
    }

    fn style_builder<VM, SS>(
        self,
        f: impl FnOnce(ViewMemberCtx<BevyRenderer>, BuildFlags) -> VM + Send + 'static,
    ) -> Self::AddMember<
        Builder<
            BevyRenderer,
            impl FnOnce(ViewMemberCtx<BevyRenderer>, BuildFlags) -> VM::VM + Send + 'static,
        >,
    >
    where
        VM: IntoViewMemberWithOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(style_builder(|ctx, flags| f(ctx, flags).into_view_member()))
    }

    fn style_future<F, VM, SS>(
        self,
        future: F,
    ) -> Self::AddMember<XFuture<impl Future<Output = VM::VM> + Send + 'static>>
    where
        F: IntoFuture<Output = VM>,
        F::IntoFuture: Send + 'static,
        VM: IntoViewMemberWithOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>> + Send + 'static,
        SS: StyleSheets<BevyRenderer> + Send + 'static,
    {
        let future = future.into_future();
        self.member(x_future(async move { future.await.into_view_member() }))
    }

    fn style_stream<VM, SS>(
        self,
        stream: impl futures_lite::stream::Stream<Item = VM> + Unpin + Send + 'static,
    ) -> Self::AddMember<
        XStream<impl futures_lite::stream::Stream<Item = VM::VM> + Unpin + Send + 'static>,
    >
    where
        VM: IntoViewMemberWithOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(x_stream(stream.map(|n| n.into_view_member())))
    }
}

impl<T> ElementStyleExt for T where T: ElementView<BevyRenderer> {}
