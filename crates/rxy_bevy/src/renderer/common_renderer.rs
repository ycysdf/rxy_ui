use crate::elements::{element_div, element_span};
use crate::{all_attrs, x_bundle, BevyElement, BevyRenderer, Focusable, XBundle};
use bevy_ui::prelude::Button;
use bevy_ui::{FocusPolicy, Interaction};
use rxy_core::common_renderer::CommonRenderer;
use rxy_core::{
    define_common_view_fns, MapToAttrMarker, ElementAttrMember, MemberOwner, ViewMember,
    ViewMemberOrigin, XNest,
};

define_common_view_fns!(BevyRenderer);

impl CommonRenderer for BevyRenderer {
    type DivView = BevyElement<element_div, ()>;
    type SpanView<
        T: ViewMember<Self>
            + ViewMemberOrigin<Self, Origin = ElementAttrViewMember<Self, Self::SpanContentEA>>,
    > = BevyElement<element_span, (T,)>;
    type ButtonView =
        BevyElement<element_div, (XBundle<(FocusPolicy, Interaction, Button, Focusable)>,)>;
    type SpanContentEA = all_attrs::content;

    fn crate_span<T>(
        str: impl XNest< MapInner<MapToAttrMarker<Self::SpanContentEA>> = T>,
    ) -> Self::SpanView<T>
    where
        T: ViewMember<Self>
            + ViewMemberOrigin<Self, Origin = ElementAttrViewMember<Self, Self::SpanContentEA>>,
        // T: ElementAttrMember<Self, EA = Self::SpanContentEA>,
    {
        BevyElement::default().members(str.map_inner::<MapToAttrMarker<Self::SpanContentEA>>())
    }

    fn crate_div() -> Self::DivView {
        BevyElement::default()
    }

    fn crate_button() -> Self::ButtonView {
        BevyElement::default().members(x_bundle((
            FocusPolicy::default(),
            Interaction::default(),
            Button,
            Focusable::default(),
        )))
    }
}
