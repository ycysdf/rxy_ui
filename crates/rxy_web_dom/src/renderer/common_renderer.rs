use crate::attrs::ElementAttrNodeValue;
use crate::elements::{ElementTypeButton, ElementTypeDiv, ElementTypeSpan, NodeTypeText};
use crate::renderer::WebRenderer;
use crate::WebElement;
use rxy_core::common_renderer::CommonRenderer;
use rxy_core::{define_common_view_fns, ElementAttrMember, MapToAttrMarker, MemberOwner, XNest};

define_common_view_fns!(WebRenderer);

impl CommonRenderer for WebRenderer {
    type DivView = WebElement<ElementTypeDiv, ()>;
    type TextView<T: ElementAttrMember<Self, Self::TextContentEA>> = WebElement<NodeTypeText, (T,)>;
    type ButtonView = WebElement<ElementTypeButton, ()>;
    type TextContentEA = ElementAttrNodeValue;

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
