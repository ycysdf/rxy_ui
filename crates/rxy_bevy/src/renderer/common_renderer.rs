use crate::elements::{element_div, element_span};
use crate::{all_attrs, x_bundle, BevyElement, BevyRenderer, Focusable, XBundle};
use bevy_ui::prelude::Button;
use bevy_ui::{FocusPolicy, Interaction};
use rxy_core::common_renderer::CommonRenderer;
use rxy_core::{
    define_common_view_fns, ElementAttrMember, MapToAttrMarker, MemberOwner, ViewMember,
    ViewMemberOrigin, XNest,
};

define_common_view_fns!(BevyRenderer);

impl CommonRenderer for BevyRenderer {
    type DivView = BevyElement<element_div, ()>;
    type TextView<T: ElementAttrMember<Self, Self::TextContentEA>> =
        BevyElement<element_span, (T,)>;
    type ButtonView =
        BevyElement<element_div, (XBundle<(FocusPolicy, Interaction, Button, Focusable)>,)>;
    type TextContentEA = all_attrs::content;

    fn crate_text<T>(
        str: impl XNest<MapInner<MapToAttrMarker<Self::TextContentEA>> = T>,
    ) -> Self::TextView<T>
    where
        T: ElementAttrMember<Self, Self::TextContentEA>,
    {
        BevyElement::default().members(str.map_inner::<MapToAttrMarker<Self::TextContentEA>>())
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
