use crate::{
    ElementAttr, ElementAttrMember, ElementSoloView,
    IntoViewMember, Renderer,
};
use alloc::borrow::Cow;

pub trait CommonRenderer: Renderer {
    type DivView: ElementSoloView<Self>;
    type SpanView<T: ElementAttrMember<Self, EA = Self::SpanContentEA>>: ElementSoloView<Self>;
    type ButtonView: ElementSoloView<Self>;
    type SpanContentEA: ElementAttr<Self, Value = Cow<'static, str>>;

    fn crate_span<T>(str: impl IntoViewMember<Self, T>) -> Self::SpanView<T>
    where
        T: ElementAttrMember<Self, EA = Self::SpanContentEA>;
    fn crate_div() -> Self::DivView;
    fn crate_button() -> Self::ButtonView;
}

#[macro_export]
macro_rules! define_common_view_fns {
    ($renderer:ident) => {
        #[inline(always)]
        pub fn span<T>(str: impl IntoViewMember<$renderer, T>) -> <$renderer as CommonRenderer>::SpanView<T>
        where
            T: ElementAttrMember<$renderer, EA = <$renderer as CommonRenderer>::SpanContentEA>,
        {
            <$renderer as CommonRenderer>::crate_span(str)
        }

        #[inline(always)]
        pub fn div() -> <$renderer as CommonRenderer>::DivView
        {
            <$renderer as CommonRenderer>::crate_div()
        }

        #[inline(always)]
        pub fn button() -> <$renderer as CommonRenderer>::ButtonView
        {
            <$renderer as CommonRenderer>::crate_button()
        }

        use rxy_core::IntoView;
        use std::borrow::Cow;
        use rxy_core::ElementAttrViewMember;

        impl IntoView<$renderer> for Cow<'static, str>
        {
            type View = <$renderer as CommonRenderer>::SpanView<ElementAttrViewMember<$renderer, <$renderer as CommonRenderer>::SpanContentEA>>;

            #[inline(always)]
            fn into_view(self) -> Self::View {
                <$renderer as CommonRenderer>::crate_span::<ElementAttrViewMember<$renderer, <$renderer as CommonRenderer>::SpanContentEA>>(ElementAttrViewMember::<
                    $renderer,
                    <$renderer as CommonRenderer>::SpanContentEA,
                >::new(self))
            }
        }

        impl IntoView<$renderer> for &'static str
        {
            type View = <$renderer as CommonRenderer>::SpanView<ElementAttrViewMember<$renderer, <$renderer as CommonRenderer>::SpanContentEA>>;

            #[inline(always)]
            fn into_view(self) -> Self::View {
                let cow = Cow::<str>::Borrowed(self);
                IntoView::<$renderer>::into_view(cow)
            }
        }

        impl IntoView<$renderer> for String
        {
            type View = <$renderer as CommonRenderer>::SpanView<ElementAttrViewMember<$renderer, <$renderer as CommonRenderer>::SpanContentEA>>;

            #[inline(always)]
            fn into_view(self) -> Self::View {
                let cow = Cow::<str>::Owned(self);
                IntoView::<$renderer>::into_view(cow)
            }
        }

    };
}
