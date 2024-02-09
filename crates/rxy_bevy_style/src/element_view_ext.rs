use rxy_bevy::BevyRenderer;
use rxy_core::style::{ApplyStyleSheets, StyleSheets};
use rxy_core::{
    rx, style_builder, x_future, x_stream, BuildFlags, XBuilder, ElementView, InnerIvmToVm,
    MapToStyleSheetsMarker, MaybeSend, Reactive, ViewMember, ViewMemberCtx, ViewMemberOrigin,
    XFuture, XNest, XStream,
};
use std::future::Future;

pub trait ElementStyleExt: ElementView<BevyRenderer> {
    fn style<VM, SS>(
        self,
        style_sheets: impl XNest<BevyRenderer, MapInner<MapToStyleSheetsMarker<SS>> = VM>,
    ) -> Self::AddMember<VM>
    where
        VM: ViewMember<BevyRenderer>
            + ViewMemberOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(style_sheets.map_inner::<MapToStyleSheetsMarker<SS>>())
    }
    //
    // fn style_rx<IVM, VM, SS, F>(
    //     self,
    //     f: F,
    // ) -> Self::AddMember<InnerIvmToVm<Reactive<F, IVM>, ApplyStyleSheets<SS>>>
    // where
    //     F: Fn() -> IVM + MaybeSend + 'static,
    //     VM: ViewMember<BevyRenderer>
    //         + ViewMemberOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
    //     IVM: XNest<BevyRenderer, MapMember<MapToStyleSheetsMarker<SS>> = VM> + MaybeSend + 'static,
    //     SS: StyleSheets<BevyRenderer>,
    // {
    //     self.member(InnerIvmToVm::new(rx(f)))
    //     // self.style(rx(f))
    // }
    //
    // fn style_builder<F, VM, IVM, SS>(
    //     self,
    //     f: F,
    // ) -> Self::AddMember<InnerIvmToVm<Builder<BevyRenderer, F>, VM>>
    // where
    //     F: FnOnce(ViewMemberCtx<BevyRenderer>, BuildFlags) -> IVM + MaybeSend + 'static,
    //     VM: ViewMember<BevyRenderer> + ViewMemberOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
    //     IVM: XNest<BevyRenderer, MapMember= VM>,
    //     SS: StyleSheets<BevyRenderer>,
    // {
    //     self.member(style_builder(f))
    // }
    //
    // fn style_future<TO, VM, IVM, SS>(
    //     self,
    //     future: TO,
    // ) -> Self::AddMember<InnerIvmToVm<XFuture<TO>, VM>>
    // where
    //     TO: Future + MaybeSend + 'static,
    //     VM: ViewMember<BevyRenderer> + ViewMemberOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
    //     TO::Output: XNest<BevyRenderer, MapMember= VM> + MaybeSend + 'static,
    //     SS: StyleSheets<BevyRenderer> + MaybeSend + 'static,
    // {
    //     self.member(x_future(future))
    // }
    //
    // fn style_stream<S, VM, IVM, SS>(
    //     self,
    //     stream: S,
    // ) -> Self::AddMember<InnerIvmToVm<XStream<S>, VM>>
    // where
    //     S: futures_lite::stream::Stream<Item = IVM> + Unpin + MaybeSend + 'static,
    //     VM: ViewMember<BevyRenderer> + ViewMemberOrigin<BevyRenderer, Origin = ApplyStyleSheets<SS>>,
    //     IVM: XNest<BevyRenderer, MapMember= VM> + MaybeSend + 'static,
    //     SS: StyleSheets<BevyRenderer>,
    // {
    //     self.member(x_stream(stream))
    // }
}

impl<T> ElementStyleExt for T where T: ElementView<BevyRenderer> {}
