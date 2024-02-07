use rxy_bevy::BevyRenderer;
use rxy_core::style::{ApplyStyleSheets, StyleSheets};
use rxy_core::{
    rx, style_builder, x_future, x_stream, BuildFlags, Builder, ElementView, InnerIvmToVm,
    IntoViewMember, MaybeSend, Reactive, ViewMember, ViewMemberCtx, ViewMemberOrigin, XFuture,
    XStream,
};
use std::future::Future;
use std::marker::PhantomData;

pub trait IntoStyleViewMember<VM> {}

impl<T, VM, SS> IntoStyleViewMember<VM> for T
where
    T: IntoViewMember<BevyRenderer, VM>,
    VM: ViewMemberOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
    SS: StyleSheets<BevyRenderer>,
{
}

pub trait ElementStyleExt: ElementView<BevyRenderer> {
    // fn style<VM>(self, style_sheets: impl IntoStyleViewMember<VM>) -> Self::AddMember<VM>
    // where
    //     VM: ViewMemberOrigin<BevyRenderer>,
    // {
    //     self.member(style_sheets)
    // }
    fn style<VM, SS>(
        self,
        style_sheets: impl IntoViewMember<BevyRenderer, VM>,
    ) -> Self::AddMember<VM>
    where
        VM: ViewMemberOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(style_sheets)
    }

    fn style_rx<IVM, VM, SS, F>(self, f: F) -> Self::AddMember<InnerIvmToVm<Reactive<F, IVM>, VM>>
    where
        F: Fn() -> IVM + MaybeSend + 'static,
        VM: ViewMemberOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
        IVM: IntoViewMember<BevyRenderer, VM> + MaybeSend + 'static,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(rx(f))
    }

    fn style_builder<F, VM, IVM, SS>(
        self,
        f: F,
    ) -> Self::AddMember<InnerIvmToVm<Builder<BevyRenderer, F>, VM>>
    where
        F: FnOnce(ViewMemberCtx<BevyRenderer>, BuildFlags) -> IVM + MaybeSend + 'static,
        VM: ViewMemberOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
        IVM: IntoViewMember<BevyRenderer, VM>,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(style_builder(f))
    }

    fn style_future<TO, VM, IVM, SS>(
        self,
        future: TO,
    ) -> Self::AddMember<InnerIvmToVm<XFuture<TO>, VM>>
    where
        TO: Future + MaybeSend + 'static,
        VM: ViewMemberOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
        TO::Output: IntoViewMember<BevyRenderer, VM> + MaybeSend + 'static,
        SS: StyleSheets<BevyRenderer> + MaybeSend + 'static,
    {
        self.member(x_future(future))
    }

    fn style_stream<S, VM, IVM, SS>(
        self,
        stream: S,
    ) -> Self::AddMember<InnerIvmToVm<XStream<S>, VM>>
    where
        S: futures_lite::stream::Stream<Item = IVM> + Unpin + MaybeSend + 'static,
        VM: ViewMemberOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
        IVM: IntoViewMember<BevyRenderer, VM> + MaybeSend + 'static,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(x_stream(stream))
    }
}

impl<T> ElementStyleExt for T where T: ElementView<BevyRenderer> {}
