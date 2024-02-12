use crate::elements::{element_button, element_div, element_text};
use crate::renderer::WebRenderer;
use crate::WebElement;
use rxy_core::common_renderer::CommonRenderer;
use rxy_core::{define_common_view_fns, ElementAttrMember, MapToAttrMarker, MemberOwner, XNest};
use crate::attrs::node_value;

define_common_view_fns!(WebRenderer);

impl CommonRenderer for WebRenderer {
    type DivView = WebElement<element_div, ()>;
    type TextView<T: ElementAttrMember<Self, Self::TextContentEA>> = WebElement<element_text, (T,)>;
    type ButtonView = WebElement<element_button, ()>;
    type TextContentEA = node_value;

    fn crate_text<T>(
        str: impl XNest<MapInner<MapToAttrMarker<Self::TextContentEA>> = T>,
    ) -> Self::TextView<T>
    where
        T: ElementAttrMember<Self, Self::TextContentEA>,
    {
        WebElement::default().members(str.map_inner::<MapToAttrMarker<Self::TextContentEA>>())
    }

    fn crate_div() -> Self::DivView {
        WebElement::default()
    }

    fn crate_button() -> Self::ButtonView {
        WebElement::default()
    }
}
