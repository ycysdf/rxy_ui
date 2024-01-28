use bevy_reflect::Reflect;

// pub use element_input::*;
pub use element_text::*;
#[allow(unused_imports)]
pub use element_view::*;

use crate::text_styled_element::ReflectTextStyledElementType;
use crate::{define_elements, element_core::ElementTypeUnTyped, ElementAttr, ElementType};

// mod element_input;
mod element_text;
mod element_view;

#[inline(always)]
pub fn view_element_type() -> &'static dyn ElementTypeUnTyped {
    &view
}

define_elements!(
    #[derive(Reflect,Debug,Clone,Copy)]
    view {
        [attrs]
    }

    #[derive(Reflect, Debug, Clone, Copy)]
    #[reflect(TextStyledElementType)]
    text {
        [attrs]
        content
    }

    // #[derive(Reflect, Debug, Clone, Copy)]
    // #[reflect(TextStyledElementType)]
    // input {
    //     [attrs]
    //     text_value
    // }
);
