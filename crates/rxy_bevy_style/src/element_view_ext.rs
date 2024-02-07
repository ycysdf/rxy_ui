use rxy_bevy::BevyRenderer;
use rxy_core::style::{ApplyStyleSheets, StyleSheets};
use rxy_core::{rx, style_builder, x_future, x_stream, BuildFlags, Builder, ElementView, InnerIvmToVm, IntoViewMember, MaybeSend, Reactive, ViewMemberCtx, ViewMemberOrigin, XFuture, XStream, ViewMember};
use std::future::Future;

pub trait ElementStyleExt: ElementView<BevyRenderer> {
    fn style<VM, SS>(
        self,
        style_sheets: impl IntoViewMember<BevyRenderer, Member = VM>,
    ) -> Self::AddMember<VM>
    where
        VM: ViewMember<BevyRenderer> + ViewMemberOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(style_sheets)
    }

    fn style_rx<IVM, VM, SS, F>(self, f: F) -> Self::AddMember<InnerIvmToVm<Reactive<F, IVM>, VM>>
    where
        F: Fn() -> IVM + MaybeSend + 'static,
        VM: ViewMember<BevyRenderer> + ViewMemberOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
        IVM: IntoViewMember<BevyRenderer, Member = VM> + MaybeSend + 'static,
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
        VM: ViewMember<BevyRenderer> + ViewMemberOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
        IVM: IntoViewMember<BevyRenderer, Member = VM>,
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
        VM: ViewMember<BevyRenderer> + ViewMemberOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
        TO::Output: IntoViewMember<BevyRenderer, Member = VM> + MaybeSend + 'static,
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
        VM: ViewMember<BevyRenderer> + ViewMemberOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
        IVM: IntoViewMember<BevyRenderer, Member = VM> + MaybeSend + 'static,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(x_stream(stream))
    }
}

impl<T> ElementStyleExt for T where T: ElementView<BevyRenderer> {}
