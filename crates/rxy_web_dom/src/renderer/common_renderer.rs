use crate::attrs::ElementAttrNodeValue;
use crate::elements::{ElementTypeButton, ElementTypeDiv, ElementTypeSpan};
use crate::renderer::WebRenderer;
use crate::WebElement;
use rxy_core::common_renderer::CommonRenderer;
use rxy_core::{
    define_common_view_fns, ElementAttrMember, MapToAttrMarker, MemberOwner, XNest,
};

define_common_view_fns!(WebRenderer);

impl CommonRenderer for WebRenderer {
    type DivView = WebElement<ElementTypeDiv, ()>;
    type SpanView<T: ElementAttrMember<Self, Self::SpanContentEA>> =
        WebElement<ElementTypeSpan, (T,)>;
    type ButtonView = WebElement<ElementTypeButton, ()>;
    type SpanContentEA = ElementAttrNodeValue;

    fn crate_span<T>(
        str: impl XNest<MapInner<MapToAttrMarker<Self::SpanContentEA>> = T>,
    ) -> Self::SpanView<T>
    where
        T: ElementAttrMember<Self, Self::SpanContentEA>,
    {
        WebElement::default().members(str.map_inner::<MapToAttrMarker<Self::SpanContentEA>>())
    }

    fn crate_div() -> Self::DivView {
        WebElement::default()
    }

    fn crate_button() -> Self::ButtonView {
        WebElement::default()
    }
}
