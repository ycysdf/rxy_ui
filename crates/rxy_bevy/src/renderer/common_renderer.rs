use bevy_ui::{FocusPolicy, Interaction};
use bevy_ui::prelude::Button;

use rxy_core::{define_common_view_fns, ElementAttrMember, ElementView, MapToAttrMarker, XNest};
use rxy_core::common_renderer::CommonRenderer;

use crate::{BevyRenderer, Focusable, x_bundle};
#[cfg(feature = "dynamic_element")]
use crate::DynamicBevyElement;
use crate::elements::{element_div, element_img, element_span, element_span_attrs};

define_common_view_fns!(BevyRenderer);

#[cfg(not(feature = "dynamic_element"))]
impl CommonRenderer for BevyRenderer {
    type DivView = BevyElement<element_div, ()>;
    type TextView<T: ElementAttrMember<Self, Self::TextContentEA>> =
        BevyElement<element_span, (T,)>;
    type ButtonView =
        BevyElement<element_div, (XBundle<(FocusPolicy, Interaction, Button, Focusable)>,)>;
    type ImgView = BevyElement<element_img, ()>;
    type TextContentEA = element_span_attrs::content;

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

    fn crate_img() -> Self::ImgView {
        BevyElement::default()
    }
}

#[cfg(feature = "dynamic_element")]
impl CommonRenderer for BevyRenderer {
    type DivView = DynamicBevyElement<element_div>;
    type TextView<T: ElementAttrMember<Self, Self::TextContentEA>> =
        DynamicBevyElement<element_span>;
    type ButtonView = DynamicBevyElement<element_div>;
    type ImgView = DynamicBevyElement<element_img>;
    type TextContentEA = element_span_attrs::content;

    fn crate_text<T>(
        str: impl XNest<MapInner<MapToAttrMarker<Self::TextContentEA>> = T>,
    ) -> Self::TextView<T>
    where
        T: ElementAttrMember<Self, Self::TextContentEA>,
    {
        DynamicBevyElement::default()
            .members(str.map_inner::<MapToAttrMarker<Self::TextContentEA>>())
    }

    fn crate_div() -> Self::DivView {
        DynamicBevyElement::default()
    }

    fn crate_button() -> Self::ButtonView {
        DynamicBevyElement::default().members(x_bundle((
            FocusPolicy::default(),
            Interaction::default(),
            Button,
            Focusable::default(),
        )))
    }

    fn crate_img() -> Self::ImgView {
        DynamicBevyElement::default()
    }
}
