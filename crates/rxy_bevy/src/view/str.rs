use rxy_bevy_element::{all_attrs, elements};
use rxy_core::IntoView;
use crate::{BevyElement, BevyRenderer, span, ViewAttr};

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