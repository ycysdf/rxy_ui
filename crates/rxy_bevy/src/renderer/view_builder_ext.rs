use crate::{BevyRenderer, XBundle};
use bevy_ecs::bundle::Bundle;
use rxy_core::style::{ElementStyleMember, StyleSheets};
use rxy_core::{rx, MapToStyleSheetsMarker, MaybeSend, Reactive, XNest};

use rxy_core::{ElementView, MemberOwner};
macro_rules! impl_view_builder_ext {
    ($name:ident;$ty:ident) => {
        pub trait $name: $ty<BevyRenderer> + Sized {
            #[inline]
            fn bundle<T: Bundle>(self, bundle: T) -> Self::AddMember<XBundle<T>>
            where
                Self: Sized,
            {
                self.member(XBundle(bundle))
            }

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

        impl<T> $name for T where T: $ty<BevyRenderer> + Sized {}
    };
}
impl_view_builder_ext!(MemberOwnerViewBuilderExt;MemberOwner);
impl_view_builder_ext!(ElementViewViewBuilderExt;ElementView);
