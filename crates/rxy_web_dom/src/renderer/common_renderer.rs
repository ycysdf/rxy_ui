use crate::renderer::WebRenderer;
use rxy_core::common_renderer::CommonRenderer;
use rxy_core::{ElementAttrMember, IntoViewMember};

impl CommonRenderer for WebRenderer {
    type DivView = ();
    type SpanView<T: ElementAttrMember<Self, EA=Self::SpanContentEA>> = ();
    type ButtonView = ();
    type SpanContentEA = ();

    fn crate_span<T>(str: impl IntoViewMember<Self, T>) -> Self::SpanView<T> where T: ElementAttrMember<Self, EA=Self::SpanContentEA> {
        todo!()
    }

    fn crate_div() -> Self::DivView {
        todo!()
    }

    fn crate_button() -> Self::ButtonView {
        todo!()
    }
}
