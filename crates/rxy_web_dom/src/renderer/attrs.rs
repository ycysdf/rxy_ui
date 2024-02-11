#![allow(non_camel_case_types)]

use crate::renderer::WebRenderer;
use rxy_core::{AttrIndex, ElementAttrType, HasIndex, RendererNodeId, RendererWorld, count_macro, paste, ElementAttr};
use std::borrow::Cow;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct WebRendererElementAttr<const INDEX: AttrIndex>;

macro_rules! define_element_attr {
    (@custom $ty:ident) => {};
    (@attribute $ty:ident) => {
        impl ElementAttrType<WebRenderer> for paste!([<ElementAttr $ty:camel>]) {
            type Value = Cow<'static, str>;

            const NAME: &'static str = stringify!($ty);

            fn update_value(
                _world: &mut RendererWorld<WebRenderer>,
                node_id: RendererNodeId<WebRenderer>,
                value: impl Into<Self::Value>,
            ) {
                if let Some(element) = node_id.dyn_ref::<HtmlElement>() {
                    element.set_attribute(Self::NAME, &*value.into()).unwrap();
                }
            }
        }
    };
    (@style_prop $ty:ident) => {
        impl ElementAttrType<WebRenderer> for paste!([<ElementAttr $ty:camel>]) {
            type Value = Cow<'static, str>;

            const NAME: &'static str = stringify!($ty);

            fn update_value(
                _world: &mut RendererWorld<WebRenderer>,
                node_id: RendererNodeId<WebRenderer>,
                value: impl Into<Self::Value>,
            ) {
                if let Some(element) = node_id.dyn_ref::<HtmlElement>() {
                    element.style().set_property(Self::NAME, &*value.into()).unwrap();
                }
            }
        }
    };
}

macro_rules! define_element_attr_fns {
    (
        name = $name:ident;
        index_start = $index_start:expr;
        $(
        $m:tt = [
            $($ty:ident),*
        ]
        )*
    ) => {
        define_element_attr_fns!(
            $name;
            $index_start;
            $(
                $($m $ty)*
            )*
        );
    };
    (
        $name:ident;
        $index_start:expr;
        $($m:tt $ty:ident)*
    ) => {
        count_macro::count! {
            paste!{
            $(
                pub type [<ElementAttr $ty:camel>] = WebRendererElementAttr<_int_a_>;

                impl rxy_core::HasIndex for [<ElementAttr $ty:camel>] {
                   const INDEX: rxy_core::AttrIndex = $index_start + _int_b_;
                }
                define_element_attr!(@$m $ty);
            )*
            }
        }

        #[allow(non_upper_case_globals)]
        #[allow(non_camel_case_types)]
        pub trait $name {
            count_macro::count! {
                const ATTRS: &'static [&'static dyn rxy_core::ElementAttrUntyped<$crate::WebRenderer>] = &[
                $(
                    {
                        let $ty = &WebRendererElementAttr::<_int_>;
                        $ty
                    },
                )*
                ];
            }
        }

        count_macro::count! {
            paste! {
                pub trait [<$name ViewBuilder>]: rxy_core::MemberOwner<$crate::WebRenderer> + Sized {
                    $(
                        fn $ty<T>(self, value: impl rxy_core::XNest<MapInner<rxy_core::MapToAttrMarker<WebRendererElementAttr<_int_a_>>> = T>) -> Self::AddMember<T>
                        where
                            T: rxy_core::ElementAttrMember<$crate::WebRenderer, WebRendererElementAttr<_int_b_>>,
                            (Self::VM, T): rxy_core::ViewMember<$crate::WebRenderer>
                        {
                            self.member(value.map_inner::<rxy_core::MapToAttrMarker<WebRendererElementAttr<_int_c_>>>())
                        }
                    )*
                }

                impl<T: rxy_core::MemberOwner<$crate::WebRenderer>> [<$name ViewBuilder>] for T {}
            }
        }
    };
}

define_element_attr_fns! {
    name = CommonAttrs;
    index_start = 0;
    attribute = [
        id,
        name,
        class
    ]
    custom = [
        node_value
    ]
    style_prop = [
        z_index,
        bg_color,
        border_left,
        border_right,
        border_top,
        border_bottom,
        border_color,
        display,
        position_type,
        overflow_x,
        overflow_y,
        direction,
        left,
        right,
        top,
        bottom,
        width,
        height,
        min_width,
        min_height,
        max_width,
        max_height,
        margin_left,
        margin_right,
        margin_top,
        margin_bottom,
        padding_left,
        padding_right,
        padding_top,
        padding_bottom,
        aspect_ratio,
        align_items,
        justify_items,
        align_self,
        justify_self,
        align_content,
        justify_content,
        flex_direction,
        flex_wrap,
        flex_grow,
        flex_shrink,
        flex_basis,
        column_gap,
        row_gap,
        visibility,
        translation,
        rotation,
        scale,
        text_color,
        font_size,
        text_linebreak,
        text_align,
        font,
        outline_width,
        outline_offset,
        outline_color
    ]
}

impl ElementAttrType<WebRenderer> for ElementAttrNodeValue{
    type Value = Cow<'static, str>;

    const NAME: &'static str = stringify!(node_value);

    fn update_value(
        _world: &mut RendererWorld<WebRenderer>,
        node_id: RendererNodeId<WebRenderer>,
        value: impl Into<Self::Value>,
    ) {
        node_id.set_node_value(Some(&*value.into()));
    }
}