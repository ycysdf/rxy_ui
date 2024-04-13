#![allow(non_camel_case_types)]

use super::elements::*;
use crate::renderer::WebRenderer;
use rxy_core::{
   attrs_fn_define, count_macro, impl_index_for_tys, paste, AttrIndex, ElementAttr,
   ElementAttrType, HasIndex, RendererNodeId, RendererWorld, XNest,
};
use std::borrow::Cow;
use wasm_bindgen::intern;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct WebRendererElementAttr<const INDEX: AttrIndex>;

macro_rules! define_element_attr {
    (@common_attribute $ty:ident) => {};
    (@attribute $ty:ident) => {
        paste::paste! {
            pub struct [<$ty:snake>];
            impl ElementAttrType<WebRenderer> for [<$ty:snake>] {
                type Value = Cow<'static, str>;

                const NAME: &'static str = stringify!($ty);

                fn update_value(
                    _world: &mut RendererWorld<WebRenderer>,
                    node_id: RendererNodeId<WebRenderer>,
                    value: impl Into<Self::Value>,
                ) {
                    if let Some(element) = node_id.dyn_ref::<HtmlElement>() {
                        element.set_attribute(intern(Self::NAME), &*value.into()).unwrap();
                    }
                }
            }
        }
    };
    (@style_prop $ty:tt) => {
        paste::paste! {
            pub struct [<$ty:snake>];
            impl ElementAttrType<WebRenderer> for [<$ty:snake>] {
                type Value = Cow<'static, str>;

                const NAME: &'static str = $ty;

                fn update_value(
                    _world: &mut RendererWorld<WebRenderer>,
                    node_id: RendererNodeId<WebRenderer>,
                    value: impl Into<Self::Value>,
                ) {
                    let value =&*value.into();
                    node_id.unchecked_ref::<HtmlElement>().style().set_property(intern(Self::NAME), value).unwrap();
                }
            }
        }
    };
}

macro_rules! define_element_attr_fns {
    (
        $(
            $(element = $element:ident;)?
            -
            $(
            $m:ident = [
                $($ty:tt),*
            ]
            )*
            ----
        )*
    ) => {
        define_element_attr_fns!(
            @INNER;
            $(
                $(element = $element;)?
                -
                $(
                    $($m:$ty)*
                )*
                ----
            )*
        );
    };
    (
        @INNER;
        $(
            $(element = $element:ident;)?
            -
            $($m:ident:$ty:tt)*
            ----
        )*
    ) => {
        $(
            $(define_element_attr!(@$m $ty);)*
        )*
        paste::paste! {
            impl_index_for_tys! {
                $(
                    $([<$ty:snake>])*
                )*
            }
        }

        // #[allow(non_upper_case_globals)]
        // #[allow(non_camel_case_types)]
        // pub trait $name {
        //     count_macro::count! {
        //         const ATTRS: &'static [&'static dyn rxy_core::ElementAttrUntyped<$crate::WebRenderer>] = &[
        //         $(
        //             {
        //                 paste! {
        //                     let [<$ty:snake>] = &WebRendererElementAttr::<_int_>;
        //                     [<$ty:snake>]
        //                 }
        //             },
        //         )*
        //         ];
        //     }
        // }



        paste!{
        $(
            attrs_fn_define! {
                renderer = WebRenderer;
                $(element = [<element_ $element>];)?
                attrs = [
                    $({
                        name = $ty,
                        ty = [<$ty:snake>]
                    })*
                ]
            }
        )*
        }

    };
}
define_element_attr_fns! {
    -
    attribute = [
        id,
        name,
        class
    ]
    style_prop = [
        // "style",
        "z-index",
        "background-color",
        "border-left",
        "border-right",
        "border-top",
        "border-bottom",
        "border-color",
        "display",
        "position-type",
        "overflow-x",
        "overflow-y",
        "direction",
        "left",
        "right",
        "top",
        "bottom",
        "width",
        "height",
        "min-width",
        "min-height",
        "max-width",
        "max-height",
        "margin-left",
        "margin-right",
        "margin-top",
        "margin-bottom",
        "padding-left",
        "padding-right",
        "padding-top",
        "padding-bottom",
        "aspect-ratio",
        "align-items",
        "justify-items",
        "align-self",
        "justify-self",
        "align-content",
        "justify-content",
        "flex-direction",
        "flex-wrap",
        "flex-grow",
        "flex-shrink",
        "flex-basis",
        "gap",
        "visibility",
        "translation",
        "rotation",
        "scale",
        "text-color",
        "font-size",
        "text-linebreak",
        "text-align",
        "font",
        "outline-width",
        "outline-offset",
        "outline-color",
        "border",
        "margin",
        "padding"
    ]
    ----
    element = img;
    -
    attribute = [
        src
    ]
    ----
    element = input;
    -
    // common_attribute = [
    //     value,
    //     placeholder
    // ]
    ----
    element = textarea;
    -
    // common_attribute = [
    //     value,
    //     placeholder
    // ]
    ----
    element = select;
    -
    attribute = [
    ]
    ----
    element = a;
    -
    attribute = [
        href,
        target
    ]
    ----
    element = button;
    -
    attribute = [
        disabled
    ]
    ----
    element = text;
    -
    common_attribute = [
        node_value
    ]
    ----
}

pub struct node_value;

impl ElementAttrType<WebRenderer> for node_value {
   type Value = Cow<'static, str>;

   const NAME: &'static str = stringify!(node - value);

   fn update_value(
      _world: &mut RendererWorld<WebRenderer>,
      node_id: RendererNodeId<WebRenderer>,
      value: impl Into<Self::Value>,
   ) {
      node_id.set_node_value(Some(&*value.into()));
   }
}

// define_element_attr!(@attribute value);
// define_element_attr!(@attribute placeholder);
