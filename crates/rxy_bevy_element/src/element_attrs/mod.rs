#![allow(non_camel_case_types)]


use crate::{
    attrs_trait_define,
};

pub mod attr_values;

pub(crate) mod attrs;
// pub(crate) mod composite_attrs;

pub trait MyFromStr: Sized {
    fn from_str(s: &str) -> Option<Self>;
}

pub fn from_str<T: MyFromStr>(s: &str) -> Option<T> {
    T::from_str(s)
}

attrs_trait_define!(CommonAttrs;0;
    class
    name
    z_index
    bg_color
    border_left
    border_right
    border_top
    border_bottom
    border_color
    display
    position_type
    overflow_x
    overflow_y
    direction
    left
    right
    top
    bottom
    width
    height
    min_width
    min_height
    max_width
    max_height
    margin_left
    margin_right
    margin_top
    margin_bottom
    padding_left
    padding_right
    padding_top
    padding_bottom
    aspect_ratio
    align_items
    justify_items
    align_self
    justify_self
    align_content
    justify_content
    flex_direction
    flex_wrap
    flex_grow
    flex_shrink
    flex_basis
    column_gap
    row_gap
    visibility
    translation
    rotation
    scale
    text_color
    font_size
    text_linebreak
    text_align
    font
    outline_width
    outline_offset
    outline_color
);
// composite_attrs_trait_define!(CommonCompositeAttrs;
//     margin
//     margin_horizontal
//     margin_vertical
//     padding
//     padding_horizontal
//     padding_vertical
//     border
//     transform
// );
