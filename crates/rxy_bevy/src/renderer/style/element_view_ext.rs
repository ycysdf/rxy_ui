use crate::BevyRenderer;
use rxy_core::style::{ElementStyleMember, StyleSheets};
use rxy_core::{rx, ElementView, MapToStyleSheetsMarker, MaybeSend, Reactive, XNest};

pub trait ElementViewStyleExt: ElementView<BevyRenderer> {
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
    fn rx_style<F, X, VM, SS>(
        self,
        f: F,
    ) -> Self::AddMember<Reactive<impl Fn() -> VM + MaybeSend + 'static, VM>>
    where
        F: Fn() -> X + MaybeSend + 'static,
        X: XNest<MapInner<MapToStyleSheetsMarker<SS>> = VM>,
        VM: ElementStyleMember<BevyRenderer, SS>,
        SS: StyleSheets<BevyRenderer>,
    {
        self.member(rx(move || f().map_inner::<MapToStyleSheetsMarker<SS>>()))
    }
}

impl<T> ElementViewStyleExt for T where T: ElementView<BevyRenderer> {}
