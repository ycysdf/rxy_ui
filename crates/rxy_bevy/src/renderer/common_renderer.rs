use crate::elements::{element_div, element_span};
use crate::{all_attrs, x_bundle, BevyElement, BevyRenderer, Focusable, XBundle};
use bevy_ui::prelude::Button;
use bevy_ui::{FocusPolicy, Interaction};
use rxy_core::common_renderer::CommonRenderer;
use rxy_core::{define_common_view_fns, ElementAttrMember, IntoViewMember, MemberOwner};


define_common_view_fns!(BevyRenderer);

impl CommonRenderer for BevyRenderer {
    type DivView = BevyElement<element_div, ()>;
    type SpanView<T: ElementAttrMember<Self, EA = Self::SpanContentEA>> =
        BevyElement<element_span, (T,)>;
    type ButtonView =
        BevyElement<element_div, (XBundle<(FocusPolicy, Interaction, Button, Focusable)>,)>;
    type SpanContentEA = all_attrs::content;

    fn crate_span<T>(str: impl IntoViewMember<Self, T>) -> Self::SpanView<T>
    where
        T: ElementAttrMember<Self, EA = Self::SpanContentEA>,
    {
        BevyElement::default().members(str)
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

