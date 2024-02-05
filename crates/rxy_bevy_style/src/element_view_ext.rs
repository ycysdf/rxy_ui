use crate::into_view_member::ViewMemberWithOriginWrapper;
use crate::style_sheets::StyleSheets;
use crate::{into_view_member::ViewMemberWithOrigin, ApplyStyleSheets};
use futures_lite::StreamExt;
use rxy_bevy::BevyRenderer;
use rxy_core::{
    rx, style_builder, x_future, x_stream, BuildFlags, Builder, ElementView, IntoViewMemberWrapper,
    MaybeSend, Reactive, ViewMember, ViewMemberCtx, XFuture, XStream,
};
use std::future::Future;

pub trait ElementStyleExt: ElementView<BevyRenderer> {
    fn style<VM, SS>(
        self,
        style_sheets: impl ViewMemberWithOrigin<BevyRenderer, VM, Origin = ApplyStyleSheets<SS>>,
    ) -> Self::AddMember<VM>
    where
        VM: ViewMember<BevyRenderer>,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(IntoViewMemberWrapper(style_sheets.into_view_member()))
    }

    fn style_rx<VMO, VM, SS, F>(
        self,
        f: F,
    ) -> Self::AddMember<ViewMemberWithOriginWrapper<Reactive<F, VMO>, VM>>
    where
        F: Fn() -> VMO + MaybeSend + 'static,
        VM: ViewMember<BevyRenderer>,
        VMO: ViewMemberWithOrigin<BevyRenderer, VM, Origin = ApplyStyleSheets<SS>> + MaybeSend,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(IntoViewMemberWrapper(ViewMemberWithOriginWrapper::new(rx(
            f,
        ))))
    }

    fn style_builder<VM, VMO, SS>(
        self,
        f: impl FnOnce(ViewMemberCtx<BevyRenderer>, BuildFlags) -> VMO + MaybeSend + 'static,
    ) -> Self::AddMember<
        Builder<
            BevyRenderer,
            impl FnOnce(ViewMemberCtx<BevyRenderer>, BuildFlags) -> VM + MaybeSend + 'static,
        >,
    >
    where
        VM: ViewMember<BevyRenderer>,
        VMO: ViewMemberWithOrigin<BevyRenderer, VM, Origin = ApplyStyleSheets<SS>>,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(style_builder(|ctx, flags| f(ctx, flags).into_view_member()))
    }

    fn style_future<TO, VM, VMO, SS>(
        self,
        future: TO,
    ) -> Self::AddMember<ViewMemberWithOriginWrapper<XFuture<TO>, VM>>
    where
        TO: Future + MaybeSend + 'static,
        VM: ViewMember<BevyRenderer>,
        TO::Output: ViewMemberWithOrigin<BevyRenderer, VM, Origin = ApplyStyleSheets<SS>>
            + MaybeSend
            + 'static,
        SS: StyleSheets<BevyRenderer> + MaybeSend + 'static,
    {
        self.member(IntoViewMemberWrapper(ViewMemberWithOriginWrapper::new(
            x_future(future),
        )))
    }

    fn style_stream<VM, VMO, SS>(
        self,
        stream: impl futures_lite::stream::Stream<Item = VMO> + Unpin + MaybeSend + 'static,
    ) -> Self::AddMember<
        XStream<impl futures_lite::stream::Stream<Item = VM> + Unpin + MaybeSend + 'static>,
    >
    where
        VM: ViewMember<BevyRenderer>,
        VMO: ViewMemberWithOrigin<BevyRenderer, VM, Origin = ApplyStyleSheets<SS>>,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(x_stream(stream.map(|n| n.into_view_member())))
    }
}

impl<T> ElementStyleExt for T where T: ElementView<BevyRenderer> {}
