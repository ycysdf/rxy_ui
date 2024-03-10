use crate::{ElementAttrMember, ElementAttrType, ElementView, MapToAttrMarker, Renderer, XNest,
};
use alloc::borrow::Cow;

pub trait CommonRenderer: Renderer {
    type DivView: ElementView<Self>;
    type TextView<T: ElementAttrMember<Self, Self::TextContentEA>>: ElementView<Self>;
    type ButtonView: ElementView<Self>;
    type ImgView: ElementView<Self>;
    type TextContentEA: ElementAttrType<Self, Value = Cow<'static, str>>;

    fn crate_text<T>(
        str: impl XNest<MapInner<MapToAttrMarker<Self::TextContentEA>> = T>,
    ) -> Self::TextView<T>
    where
        T: ElementAttrMember<Self, Self::TextContentEA>;
    fn crate_div() -> Self::DivView;
    fn crate_button() -> Self::ButtonView;
    fn crate_img() -> Self::ImgView;
}

#[macro_export]
macro_rules! define_common_view_fns {
    ($renderer:ident) => {
        #[inline]
        pub fn span<T>(
            str: impl XNest<MapInner<MapToAttrMarker<<$renderer as CommonRenderer>::TextContentEA>> = T,
            >,
        ) -> <$renderer as CommonRenderer>::TextView<T>
        where
            T: rxy_core::ElementAttrMember<$renderer, <$renderer as CommonRenderer>::TextContentEA>
        {
            <$renderer as CommonRenderer>::crate_text(str)
        }

        #[inline]
        pub fn div() -> <$renderer as CommonRenderer>::DivView {
            <$renderer as CommonRenderer>::crate_div()
        }

        #[inline]
        pub fn img() -> <$renderer as CommonRenderer>::ImgView {
            <$renderer as CommonRenderer>::crate_img()
        }

        #[inline]
        pub fn button() -> <$renderer as CommonRenderer>::ButtonView {
            <$renderer as CommonRenderer>::crate_button()
        }

        impl rxy_core::IntoView<$renderer> for std::borrow::Cow<'static, str> {
            type View = <$renderer as CommonRenderer>::TextView<
                rxy_core::ElementAttr<$renderer, <$renderer as CommonRenderer>::TextContentEA>,
            >;

            #[inline]
            fn into_view(self) -> Self::View {
                <$renderer as CommonRenderer>::crate_text::<
                    rxy_core::ElementAttr<$renderer, <$renderer as CommonRenderer>::TextContentEA>,
                >(rxy_core::ElementAttr::<
                    $renderer,
                    <$renderer as CommonRenderer>::TextContentEA,
                >::new(self))
            }
        }

        impl rxy_core::IntoView<$renderer> for &'static str {
            type View = <$renderer as CommonRenderer>::TextView<
                rxy_core::ElementAttr<$renderer, <$renderer as CommonRenderer>::TextContentEA>,
            >;

            #[inline]
            fn into_view(self) -> Self::View {
                let cow = std::borrow::Cow::<str>::Borrowed(self);
                rxy_core::IntoView::<$renderer>::into_view(cow)
            }
        }

        impl rxy_core::IntoView<$renderer> for String {
            type View = <$renderer as CommonRenderer>::TextView<
                rxy_core::ElementAttr<$renderer, <$renderer as CommonRenderer>::TextContentEA>,
            >;

            #[inline]
            fn into_view(self) -> Self::View {
                let cow = std::borrow::Cow::<str>::Owned(self);
                rxy_core::IntoView::<$renderer>::into_view(cow)
            }
        }
    };
}
