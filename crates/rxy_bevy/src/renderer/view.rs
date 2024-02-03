use bevy_ui::{FocusPolicy, Interaction};
use bevy_ui::prelude::Button;
use rxy_bevy_element::{all_attrs, elements};
use rxy_core::{IntoView, MemberOwner};
use crate::{BevyRenderer, Focusable, ViewAttr, x_bundle, XBundle};
use crate::renderer::element::{BevyElement};

pub fn span(
    str: impl Into<String>,
) -> BevyElement<elements::text, (ViewAttr<all_attrs::content>,)> {
    BevyElement::default().members(ViewAttr::<all_attrs::content>(str.into()))
}

pub fn div() -> BevyElement<elements::view, ()> {
    BevyElement::default()
}

pub fn button(
) -> BevyElement<elements::view, (XBundle<(FocusPolicy, Interaction, Button, Focusable)>,)> {
    BevyElement::default().members(x_bundle((
        FocusPolicy::default(),
        Interaction::default(),
        Button,
        Focusable::default(),
    )))
}

impl IntoView<BevyRenderer> for &'static str {
    type View = BevyElement<elements::text, (ViewAttr<all_attrs::content>,)>;

    fn into_view(self) -> Self::View {
        span(self)
    }
}

impl IntoView<BevyRenderer> for String {
    type View = BevyElement<elements::text, (ViewAttr<all_attrs::content>,)>;

    fn into_view(self) -> Self::View {
        span(self)
    }
}