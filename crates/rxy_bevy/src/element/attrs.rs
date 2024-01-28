use rxy_bevy_element::elements;

#[macro_export]
macro_rules! common_attrs_fn_define {
    ($name:ident;$($attr:ident)*) => {
        paste::paste!{
            pub trait [<$name ViewBuilder>]: rxy_core::MemberOwner<$crate::BevyRenderer> + Sized {

                $(
                    fn $attr<T: $crate::IntoViewAttrMember<$crate::all_attrs::$attr>>(self, value: T) -> Self::AddMember<T::Attr>
                        where (Self::VM, T::Attr): rxy_core::ViewMember<$crate::BevyRenderer>
                    {
                        self.member(value.into_attr())
                    }
                )*
            }

            impl<T: rxy_core::MemberOwner<$crate::BevyRenderer>> [<$name ViewBuilder>] for T {}
        }
    };
}
#[macro_export]
macro_rules! element_attrs_fn_define {
    ($name:ident;$element:ty;$($attr:ident)*) => {
        paste::paste!{
            pub trait [<$name ViewBuilder>]: rxy_core::MemberOwner<$crate::BevyRenderer> + Sized {

                $(
                    fn $attr<T: $crate::IntoViewAttrMember<$crate::all_attrs::$attr>>(self, value: T) -> Self::AddMember<T::Attr>
                        where (Self::VM, T::Attr): rxy_core::ViewMember<$crate::BevyRenderer>
                    {
                        self.member(value.into_attr())
                    }
                )*
            }

            impl<T: rxy_core::MemberOwner<$crate::BevyRenderer,E =$element>> [<$name ViewBuilder>] for T {}
        }
    };
}

common_attrs_fn_define!(CommonAttrs;
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
// common_attrs_fn_define!(CommonCompositeAttrs;
//     margin
//     margin_horizontal
//     margin_vertical
//     padding
//     padding_horizontal
//     padding_vertical
//     border
//     transform
// );

element_attrs_fn_define!(TextAttrs;elements::text;
    content
);

// element_attrs_fn_define!(InputAttrs;elements::input;
//     text_value
// );
