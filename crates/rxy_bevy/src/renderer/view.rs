use bevy_ui::prelude::Button;
use bevy_ui::{FocusPolicy, Interaction};

use rxy_core::{ElementAttrViewMember, IntoView, MemberOwner};

use crate::elements::{element_div, element_span};
use crate::{
    all_attrs, x_bundle, BevyElement, BevyElementAttrMember, BevyRenderer, Focusable, XBundle,
};

// todo: merge span
pub fn span(
    str: impl Into<String>,
) -> BevyElement<element_span, (BevyElementAttrMember<all_attrs::content>,)> {
    BevyElement::default().members(ElementAttrViewMember::<BevyRenderer, all_attrs::content>(
        str.into(),
    ))
}

pub fn div() -> BevyElement<element_div, ()> {
    BevyElement::default()
}

pub fn button(
) -> BevyElement<element_div, (XBundle<(FocusPolicy, Interaction, Button, Focusable)>,)> {
    BevyElement::default().members(x_bundle((
        FocusPolicy::default(),
        Interaction::default(),
        Button,
        Focusable::default(),
    )))
}

impl IntoView<BevyRenderer> for &'static str {
    type View = BevyElement<element_span, (BevyElementAttrMember<all_attrs::content>,)>;

    fn into_view(self) -> Self::View {
        span(self)
    }
}

impl IntoView<BevyRenderer> for String {
    type View = BevyElement<element_span, (BevyElementAttrMember<all_attrs::content>,)>;

    fn into_view(self) -> Self::View {
        span(self)
    }
}
