use crate::renderer::{NativeElement, NativeRenderer};
use rxy_core::common_renderer::CommonRenderer;
use rxy_core::{define_common_view_fns, ElementAttrMember, ElementView, MapToAttrMarker, MemberOwner, XNest};
use crate::all_attrs;
use crate::renderer::elements::{element_div, element_span};

define_common_view_fns!(NativeRenderer);

impl CommonRenderer for NativeRenderer {
    type DivView = NativeElement<element_div, ()>;
    type TextView<T: ElementAttrMember<Self, Self::TextContentEA>> =
        NativeElement<element_span, (T,)>;
    type ButtonView = NativeElement<element_div, ()>;
    type ImgView = NativeElement<element_div, ()>;
    type TextContentEA = all_attrs::content;

    fn crate_text<T>(
        str: impl XNest<MapInner<MapToAttrMarker<Self::TextContentEA>> = T>,
    ) -> Self::TextView<T>
    where
        T: ElementAttrMember<Self, Self::TextContentEA>,
    {
        NativeElement::default().members(str.map_inner::<MapToAttrMarker<Self::TextContentEA>>())
    }

    fn crate_div() -> Self::DivView {
        NativeElement::default()
    }

    fn crate_button() -> Self::ButtonView {
        NativeElement::default()
    }

    fn crate_img() -> Self::ImgView {
        todo!()
    }
}
