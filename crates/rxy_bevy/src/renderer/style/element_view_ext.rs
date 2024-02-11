use super::rxy_bevy_crate::BevyRenderer;
use rxy_core::style::{ApplyStyleSheets, ElementStyleMember, StyleSheets};
use rxy_core::{
    rx, style_builder, x_future, x_stream, BuildFlags, ElementView, InnerIvmToVm,
    MapToStyleSheetsMarker, MaybeSend, Reactive, ViewMember, ViewMemberCtx, ViewMemberOrigin,
    XBuilder, XFuture, XNest, XStream,
};
use std::future::Future;

pub trait ElementStyleExt: ElementView<BevyRenderer> {
    #[inline]
    fn style<VM, SS>(
        self,
        style_sheets: impl XNest<MapInner<MapToStyleSheetsMarker<SS>> = VM>,
    ) -> Self::AddMember<VM>
    where
        VM: ElementStyleMember<BevyRenderer, SS>,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(style_sheets.map_inner::<MapToStyleSheetsMarker<SS>>())
    }

    #[inline]
    fn rx_style<F, X, VM, SS>(self, f: F) -> Self::AddMember<Reactive<impl Fn() -> VM + MaybeSend + 'static, VM>>
    where
        F: Fn() -> X + MaybeSend + 'static,
        X: XNest<MapInner<MapToStyleSheetsMarker<SS>> = VM>,
        VM: ElementStyleMember<BevyRenderer, SS>,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(rx(move || {
            f().map_inner::<MapToStyleSheetsMarker<SS>>()
        }))
    }
}

impl<T> ElementStyleExt for T where T: ElementView<BevyRenderer> {}