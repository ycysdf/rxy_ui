use crate::{ElementAttrMember, ElementAttrType, ElementSoloView, MapToAttrMarker, Renderer, XNest,
};
use alloc::borrow::Cow;

pub trait CommonRenderer: Renderer {
    type DivView: ElementSoloView<Self>;
    type SpanView<T: ElementAttrMember<Self, Self::SpanContentEA>>: ElementSoloView<Self>;
    type ButtonView: ElementSoloView<Self>;
    type SpanContentEA: ElementAttrType<Self, Value = Cow<'static, str>>;

    fn crate_span<T>(
        str: impl XNest<MapInner<MapToAttrMarker<Self::SpanContentEA>> = T>,
    ) -> Self::SpanView<T>
    where
        T: ElementAttrMember<Self, Self::SpanContentEA>;
    fn crate_div() -> Self::DivView;
    fn crate_button() -> Self::ButtonView;
}

#[macro_export]
macro_rules! define_common_view_fns {
    ($renderer:ident) => {
        #[inline(always)]
        pub fn span<T>(
            str: impl XNest<MapInner<MapToAttrMarker<<$renderer as CommonRenderer>::SpanContentEA>> = T,
            >,
        ) -> <$renderer as CommonRenderer>::SpanView<T>
        where
            T: rxy_core::ElementAttrMember<$renderer, <$renderer as CommonRenderer>::SpanContentEA>
        {
            <$renderer as CommonRenderer>::crate_span(str)
        }

        #[inline(always)]
        pub fn div() -> <$renderer as CommonRenderer>::DivView {
            <$renderer as CommonRenderer>::crate_div()
        }

        #[inline(always)]
        pub fn button() -> <$renderer as CommonRenderer>::ButtonView {
            <$renderer as CommonRenderer>::crate_button()
        }

        impl rxy_core::IntoView<$renderer> for std::borrow::Cow<'static, str> {
            type View = <$renderer as CommonRenderer>::SpanView<
                rxy_core::ElementAttr<$renderer, <$renderer as CommonRenderer>::SpanContentEA>,
            >;

            #[inline(always)]
            fn into_view(self) -> Self::View {
                <$renderer as CommonRenderer>::crate_span::<
                    rxy_core::ElementAttr<$renderer, <$renderer as CommonRenderer>::SpanContentEA>,
                >(rxy_core::ElementAttr::<
                    $renderer,
                    <$renderer as CommonRenderer>::SpanContentEA,
                >::new(self))
            }
        }

        impl rxy_core::IntoView<$renderer> for &'static str {
            type View = <$renderer as CommonRenderer>::SpanView<
                rxy_core::ElementAttr<$renderer, <$renderer as CommonRenderer>::SpanContentEA>,
            >;

            #[inline(always)]
            fn into_view(self) -> Self::View {
                let cow = std::borrow::Cow::<str>::Borrowed(self);
                rxy_core::IntoView::<$renderer>::into_view(cow)
            }
        }

        impl rxy_core::IntoView<$renderer> for String {
            type View = <$renderer as CommonRenderer>::SpanView<
                rxy_core::ElementAttr<$renderer, <$renderer as CommonRenderer>::SpanContentEA>,
            >;

            #[inline(always)]
            fn into_view(self) -> Self::View {
                let cow = std::borrow::Cow::<str>::Owned(self);
                rxy_core::IntoView::<$renderer>::into_view(cow)
            }
        }
    };
}
