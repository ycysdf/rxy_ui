#![allow(non_camel_case_types)]

use crate::renderer::WebRenderer;
use rxy_core::{AttrIndex, ElementAttrType, HasIndex, RendererNodeId, RendererWorld, count_macro, paste, ElementAttr};
use std::borrow::Cow;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};
use wasm_bindgen::intern;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct WebRendererElementAttr<const INDEX: AttrIndex>;

macro_rules! define_element_attr {
    (@custom $ty:ident) => {};
    (@attribute $ty:ident) => {
        paste! {
            impl ElementAttrType<WebRenderer> for [<ElementAttr $ty:camel>] {
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
        paste! {
            impl ElementAttrType<WebRenderer> for [<ElementAttr $ty:camel>] {
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
        name = $name:ident;
        index_start = $index_start:expr;
        $(
        $m:tt = [
            $($ty:tt),*
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
        $($m:tt $ty:tt)*
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
                        paste! {
                            let [<$ty:snake>] = &WebRendererElementAttr::<_int_>;
                            [<$ty:snake>]
                        }
                    },
                )*
                ];
            }
        }

        count_macro::count! {
            paste! {
                pub trait [<$name ViewBuilder>]: rxy_core::MemberOwner<$crate::WebRenderer> + Sized {
                    $(
                        fn [<$ty:snake>]<T>(self, value: impl rxy_core::XNest<MapInner<rxy_core::MapToAttrMarker<WebRendererElementAttr<_int_a_>>> = T>) -> Self::AddMember<T>
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
        "column-gap",
        "row-gap",
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
}

impl ElementAttrType<WebRenderer> for ElementAttrNodeValue{
    type Value = Cow<'static, str>;

    const NAME: &'static str = stringify!(node-value);

    fn update_value(
        _world: &mut RendererWorld<WebRenderer>,
        node_id: RendererNodeId<WebRenderer>,
        value: impl Into<Self::Value>,
    ) {
        node_id.set_node_value(Some(&*value.into()));
    }
}
