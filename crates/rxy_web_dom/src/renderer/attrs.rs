#![allow(non_camel_case_types)]

use crate::renderer::WebRenderer;
use rxy_core::{AttrIndex, ElementAttr, HasIndex, RendererNodeId, RendererWorld,count_macro};
use std::borrow::Cow;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct WebRendererElementAttr<const INDEX: AttrIndex>;

macro_rules! common_attrs_fn_define {
    ($name:ident;$index_start:expr;$($ty:ident)*) => {
        rxy_core::paste!{
        count_macro::count! {
            $(
            pub type [<ElementAttr $ty:camel>] = WebRendererElementAttr<_int_a_>;

            impl rxy_core::HasIndex for [<ElementAttr $ty:camel>] {
               const INDEX: rxy_core::AttrIndex = $index_start+_int_b_;
            }

            impl ElementAttr<WebRenderer> for [<ElementAttr $ty:camel>] {
                type Value = Cow<'static, str>;

                const NAME: &'static str = stringify!($ty);

                fn update_value(
                    _world: &mut RendererWorld<WebRenderer>,
                    node_id: RendererNodeId<WebRenderer>,
                    value: impl Into<Self::Value>,
                ) {
                    if let Some(element) = node_id.dyn_ref::<HtmlElement>() {
                        element.style().set_property(Self::NAME, &value.into().to_string()).unwrap();
                    }
                }
            }
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
        rxy_core::paste!{
            pub trait [<$name ViewBuilder>]: rxy_core::MemberOwner<$crate::WebRenderer> + Sized {
                $(
                    fn $ty<T>(self, value: impl rxy_core::XNest<$crate::WebRenderer, MapMember<rxy_core::MapToAttrMarker<WebRendererElementAttr<_int_a_>>> = T>) -> Self::AddMember<T>
                    where
                        T: rxy_core::ViewMember<$crate::WebRenderer>
                        + rxy_core::ViewMemberOrigin<$crate::WebRenderer, Origin = rxy_core::ElementAttrViewMember<$crate::WebRenderer, WebRendererElementAttr<_int_b_>>>,
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

common_attrs_fn_define!{CommonAttrs;0;
    // class
    // name
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
}

//
// #[macro_export]
// macro_rules! common_attrs_fn_define {
//     ($name:ident;$index_start:expr;$($attr:ident)*) => {
//         count_macro::count! {
//             $(
//             impl rxy_core::HasIndex for $crate::all_attrs::$attr{
//                const INDEX: rxy_core::AttrIndex = $index_start+_int_;
//             }
//             )*
//         }
//
//     };
// }
// #[macro_export]
// macro_rules! element_attrs_fn_define {
//     ($name:ident;$element:ty;$index_start:expr;$($attr:ident)*) => {
//         count_macro::count! {
//             $(
//             impl rxy_core::HasIndex for $crate::all_attrs::$attr{
//                const INDEX: rxy_core::AttrIndex = $index_start+_int_;
//             }
//             )*
//         }
//
//         #[allow(non_upper_case_globals)]
//         #[allow(non_camel_case_types)]
//         pub trait $name {
//             const ATTRS: &'static [&'static dyn rxy_core::ElementAttrUntyped<$crate::WebRenderer>] = &[
//                 $(&$crate::all_attrs::$attr,)*
//             ];
//         }
//
//         paste::paste!{
//             pub trait [<$name ViewBuilder>]: rxy_core::MemberOwner<$crate::WebRenderer> + Sized
//             {
//                 $(
//                     fn $attr<T>(self, value: impl rxy_core::XNest<$crate::WebRenderer, T>) -> Self::AddMember<T>
//                     where
//                         T: rxy_core::ElementAttrMember<$crate::WebRenderer, EA = $crate::all_attrs::$attr>,
//                         (Self::VM, T): rxy_core::ViewMember<$crate::WebRenderer>
//                     {
//                         self.member(value)
//                     }
//                 )*
//             }
//
//             impl<T: rxy_core::MemberOwner<$crate::WebRenderer,E=$element>> [<$name ViewBuilder>] for T {}
//         }
//     };
// }
//
//
// const COMMON_ATTRS_COUNT: AttrIndex = <outline_color as HasIndex>::INDEX + 1;
//
// element_attrs_fn_define!(TextAttrs;element_span;COMMON_ATTRS_COUNT - 1;
//     content
// );
//
// // element_attrs_fn_define!(InputAttrs;elements::input;
// //     text_value
// // );
//
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct name;
//
// impl ElementAttr<WebRenderer> for name {
//     type Value = Cow<'static, str>;
//
//     const NAME: &'static str = stringify!(name);
//
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         if let Some(element) = node_id.dyn_ref::<Element>() {
//             element.set_attribute("name", &value.into()).unwrap();
//         }
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct z_index;
//
// impl ElementAttr<WebRenderer> for z_index {
//     type Value = ZIndex;
//
//     const NAME: &'static str = stringify!(z_index);
//
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         if let Some(element) = node_id.dyn_ref::<HtmlElement>() {
//             element.style().set_property("z-index", &value.into().to_string()).unwrap();
//         }
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct bg_color;
//
// impl ElementAttr<WebRenderer> for bg_color {
//     type Value = Color;
//
//     const NAME: &'static str = stringify!(bg_color);
//
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         if let Some(mut bc) = world.get_mut::<BackgroundColor>(node_id) {
//             *bc = BackgroundColor(value.into());
//         }
//         // let entity = world.id();
//         // context.commands.add(move |world| {
//         //     let entity_mut = world.entity_mut(entity);
//         //     match value.into() {
//         //         UiTexture::Color(color) => {
//         //             entity_mut.insert(BackgroundColor(color));
//         //             entity_mut.remove::<(
//         //                 UiImage,
//         //                 UiImageSize,
//         //                 Handle<TextureAtlas>,
//         //                 UiTextureAtlasImage,
//         //             )>();
//         //         }
//         //         UiTexture::Image {
//         //             color,
//         //             flip_x,
//         //             flip_y,
//         //             image: image_handle,
//         //         } => {
//         //             entity_mut.insert((
//         //                 BackgroundColor(color),
//         //                 UiImage {
//         //                     texture: image_handle,
//         //                     flip_y,
//         //                     flip_x,
//         //                 },
//         //                 UiImageSize::default(),
//         //             ));
//         //             context
//         //                 .entity_mut
//         //                 .remove::<(Handle<TextureAtlas>, UiTextureAtlasImage)>();
//         //         }
//         //         UiTexture::Atlas {
//         //             flip_y,
//         //             flip_x,
//         //             color,
//         //             index,
//         //             atlas,
//         //         } => {
//         //             entity_mut.insert((
//         //                 BackgroundColor(color),
//         //                 atlas,
//         //                 UiTextureAtlasImage {
//         //                     index,
//         //                     flip_x,
//         //                     flip_y,
//         //                 },
//         //             ));
//         //             entity_mut.remove::<(UiImage, UiImageSize)>();
//         //         }
//         //     }
//         // })
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct border_left;
//
// impl ElementAttr<WebRenderer> for border_left {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(border_left);
//
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             let value = value.into();
//             style.border.left = value;
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct border_right;
//
// impl ElementAttr<WebRenderer> for border_right {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(border_right);
//
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             let value = value.into();
//             style.border.right = value;
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct border_top;
//
// impl ElementAttr<WebRenderer> for border_top {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(border_top);
//
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             let value = value.into();
//             style.border.top = value;
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct border_bottom;
//
// impl ElementAttr<WebRenderer> for border_bottom {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(border_bottom);
//
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             let value = value.into();
//             style.border.bottom = value;
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct border_color;
//
// impl ElementAttr<WebRenderer> for border_color {
//     type Value = Color;
//
//     const NAME: &'static str = stringify!(border_color);
//
//     fn first_set_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         let mut entity_world_mut = world.entity_mut(node_id);
//         entity_world_mut.insert(BorderColor(value.into()));
//     }
//
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         if let Some(mut br) = world.get_mut::<BorderColor>(node_id) {
//             *br = BorderColor(value.into());
//         }
//         // world.insert(value.into());
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct display;
//
// impl ElementAttr<WebRenderer> for display {
//     type Value = bevy_ui::Display;
//
//     const NAME: &'static str = stringify!(display);
//
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.display = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct position_type;
//
// impl ElementAttr<WebRenderer> for position_type {
//     type Value = PositionType;
//
//     const NAME: &'static str = stringify!(position_type);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.position_type = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct overflow_x;
//
// impl ElementAttr<WebRenderer> for overflow_x {
//     type Value = OverflowAxis;
//
//     const NAME: &'static str = stringify!(overflow_x);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             let value = value.into();
//             style.overflow.x = value;
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct overflow_y;
//
// impl ElementAttr<WebRenderer> for overflow_y {
//     type Value = OverflowAxis;
//
//     const NAME: &'static str = stringify!(overflow_y);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             let value = value.into();
//             style.overflow.y = value;
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct direction;
//
// impl ElementAttr<WebRenderer> for direction {
//     type Value = Direction;
//
//     const NAME: &'static str = stringify!(direction);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.direction = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct left;
//
// impl ElementAttr<WebRenderer> for left {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(left);
//
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.left = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct right;
//
// impl ElementAttr<WebRenderer> for right {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(right);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.right = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct top;
//
// impl ElementAttr<WebRenderer> for top {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(top);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.top = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct bottom;
//
// impl ElementAttr<WebRenderer> for bottom {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(bottom);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.bottom = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct width;
//
// impl ElementAttr<WebRenderer> for width {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(width);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.width = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct height;
//
// impl ElementAttr<WebRenderer> for height {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(height);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.height = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct min_width;
//
// impl ElementAttr<WebRenderer> for min_width {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(min_width);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.min_width = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct min_height;
//
// impl ElementAttr<WebRenderer> for min_height {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(min_height);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.min_height = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct max_width;
//
// impl ElementAttr<WebRenderer> for max_width {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(max_width);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.max_width = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct max_height;
//
// impl ElementAttr<WebRenderer> for max_height {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(max_height);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.max_height = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct margin_left;
//
// impl ElementAttr<WebRenderer> for margin_left {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(margin_left);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             let value = value.into();
//             style.margin.left = value;
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct margin_right;
//
// impl ElementAttr<WebRenderer> for margin_right {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(margin_right);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             let value = value.into();
//             style.margin.right = value;
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct margin_top;
//
// impl ElementAttr<WebRenderer> for margin_top {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(margin_top);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             let value = value.into();
//             style.margin.top = value;
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct margin_bottom;
//
// impl ElementAttr<WebRenderer> for margin_bottom {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(margin_bottom);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             let value = value.into();
//             style.margin.bottom = value;
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct padding_left;
//
// impl ElementAttr<WebRenderer> for padding_left {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(padding_left);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             let value = value.into();
//             style.padding.left = value;
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct padding_right;
//
// impl ElementAttr<WebRenderer> for padding_right {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(padding_right);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             let value = value.into();
//             style.padding.right = value;
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct padding_top;
//
// impl ElementAttr<WebRenderer> for padding_top {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(padding_top);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             let value = value.into();
//             style.padding.top = value;
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct padding_bottom;
//
// impl ElementAttr<WebRenderer> for padding_bottom {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(padding_bottom);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             let value = value.into();
//             style.padding.bottom = value;
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct aspect_ratio;
//
// impl ElementAttr<WebRenderer> for aspect_ratio {
//     type Value = Option<f32>;
//
//     const NAME: &'static str = stringify!(aspect_ratio);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.aspect_ratio = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct align_items;
//
// impl ElementAttr<WebRenderer> for align_items {
//     type Value = AlignItems;
//
//     const NAME: &'static str = stringify!(align_items);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.align_items = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct justify_items;
//
// impl ElementAttr<WebRenderer> for justify_items {
//     type Value = JustifyItems;
//
//     const NAME: &'static str = stringify!(justify_items);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.justify_items = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct align_self;
//
// impl ElementAttr<WebRenderer> for align_self {
//     type Value = AlignSelf;
//
//     const NAME: &'static str = stringify!(align_self);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.align_self = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct justify_self;
//
// impl ElementAttr<WebRenderer> for justify_self {
//     type Value = JustifySelf;
//
//     const NAME: &'static str = stringify!(justify_self);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.justify_self = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct align_content;
//
// impl ElementAttr<WebRenderer> for align_content {
//     type Value = AlignContent;
//
//     const NAME: &'static str = stringify!(align_content);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.align_content = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct justify_content;
//
// impl ElementAttr<WebRenderer> for justify_content {
//     type Value = JustifyContent;
//
//     const NAME: &'static str = stringify!(justify_content);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.justify_content = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct flex_direction;
//
// impl ElementAttr<WebRenderer> for flex_direction {
//     type Value = FlexDirection;
//
//     const NAME: &'static str = stringify!(flex_direction);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.flex_direction = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct flex_wrap;
//
// impl ElementAttr<WebRenderer> for flex_wrap {
//     type Value = FlexWrap;
//
//     const NAME: &'static str = stringify!(flex_wrap);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.flex_wrap = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct flex_grow;
//
// impl ElementAttr<WebRenderer> for flex_grow {
//     type Value = f32;
//
//     const NAME: &'static str = stringify!(flex_grow);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.flex_grow = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct flex_shrink;
//
// impl ElementAttr<WebRenderer> for flex_shrink {
//     type Value = f32;
//
//     const NAME: &'static str = stringify!(flex_shrink);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.flex_shrink = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct flex_basis;
//
// impl ElementAttr<WebRenderer> for flex_basis {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(flex_basis);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.flex_basis = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct column_gap;
//
// impl ElementAttr<WebRenderer> for column_gap {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(column_gap);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.column_gap = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct row_gap;
//
// impl ElementAttr<WebRenderer> for row_gap {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(row_gap);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         world.entity_mut(node_id).try_set_style(|style| {
//             style.row_gap = value.into();
//         });
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct visibility;
//
// impl ElementAttr<WebRenderer> for visibility {
//     type Value = Visibility;
//
//     const NAME: &'static str = stringify!(visibility);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         if let Some(mut v) = world.get_mut::<Visibility>(node_id) {
//             *v = value.into();
//         }
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct translation;
//
// impl ElementAttr<WebRenderer> for translation {
//     type Value = Vec3;
//
//     const NAME: &'static str = stringify!(translation);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         let value = value.into();
//         if let Some(mut tf) = world.get_mut::<Transform>(node_id) {
//             tf.translation = value;
//         } else {
//             warn!("no found Transform component!");
//         }
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct rotation;
//
// impl ElementAttr<WebRenderer> for rotation {
//     type Value = Quat;
//
//     const NAME: &'static str = stringify!(rotation);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         let value = value.into();
//         if let Some(mut tf) = world.get_mut::<Transform>(node_id) {
//             tf.rotation = value;
//         } else {
//             warn!("no found Transform component!");
//         }
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct scale;
//
// impl ElementAttr<WebRenderer> for scale {
//     type Value = Vec3;
//
//     const NAME: &'static str = stringify!(scale);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         let value = value.into();
//         if let Some(mut tf) = world.get_mut::<Transform>(node_id) {
//             tf.scale = value;
//         } else {
//             warn!("no found Transform component!");
//         }
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct text_color;
//
// impl ElementAttr<WebRenderer> for text_color {
//     type Value = Color;
//
//     const NAME: &'static str = stringify!(text_color);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         let value = value.into();
//         world.entity_mut(node_id).scoped_text_styled_element_type(
//             |text_schema_type, entity_ref| {
//                 text_schema_type.set_text_color(entity_ref, value);
//             },
//         );
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct font_size;
//
// impl ElementAttr<WebRenderer> for font_size {
//     type Value = f32;
//
//     const NAME: &'static str = stringify!(font_size);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         let value = value.into();
//         world.entity_mut(node_id).scoped_text_styled_element_type(
//             |text_schema_type, entity_ref| {
//                 text_schema_type.set_font_size(entity_ref, value);
//             },
//         );
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct text_linebreak;
//
// impl ElementAttr<WebRenderer> for text_linebreak {
//     type Value = BreakLineOn;
//
//     const NAME: &'static str = stringify!(text_linebreak);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         let value = value.into();
//         world.entity_mut(node_id).scoped_text_styled_element_type(
//             |text_schema_type, entity_ref| {
//                 text_schema_type.set_text_linebreak(entity_ref, value);
//             },
//         );
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct text_align;
//
// impl ElementAttr<WebRenderer> for text_align {
//     type Value = TextAlignment;
//
//     const NAME: &'static str = stringify!(text_align);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         let value = value.into();
//         world.entity_mut(node_id).scoped_text_styled_element_type(
//             |text_schema_type, entity_ref| {
//                 text_schema_type.set_text_align(entity_ref, value);
//             },
//         );
//     }
// }
//
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct font;
//
// impl ElementAttr<WebRenderer> for font {
//     type Value = Handle<Font>;
//
//     const NAME: &'static str = stringify!(font);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         let value = value.into();
//         world.entity_mut(node_id).scoped_text_styled_element_type(
//             |text_schema_type, entity_ref| {
//                 text_schema_type.set_font(entity_ref, value.clone());
//             },
//         );
//     }
// }
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct outline_width;
//
// impl ElementAttr<WebRenderer> for outline_width {
//     type Value = Val;
//
//     const NAME: &'static str = stringify!(outline_width);
//
//     fn first_set_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         let mut entity_world_mut = world.entity_mut(node_id);
//         let value = value.into();
//         if entity_world_mut.contains::<Outline>() {
//             let mut outline = entity_world_mut.get_mut::<Outline>().unwrap();
//             outline.width = value;
//         } else {
//             entity_world_mut.insert(Outline {
//                 width: value,
//                 color: Color::BLACK,
//                 offset: Val::Px(0.),
//             });
//         }
//     }
//
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         let value = value.into();
//         if let Some(mut outline) = world.get_mut::<Outline>(node_id) {
//             outline.width = value;
//         }
//     }
// }
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct outline_offset;
//
// impl ElementAttr<WebRenderer> for outline_offset {
//     type Value = Val;
//     fn first_set_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         let mut entity_world_mut = world.entity_mut(node_id);
//         let value = value.into();
//         if entity_world_mut.contains::<Outline>() {
//             let mut outline = entity_world_mut.get_mut::<Outline>().unwrap();
//             outline.offset = value;
//         } else {
//             entity_world_mut.insert(Outline {
//                 offset: value,
//                 color: Color::BLACK,
//                 width: Val::Px(1.),
//             });
//         }
//     }
//
//     const NAME: &'static str = stringify!(outline_offset);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         let value = value.into();
//         if let Some(mut outline) = world.get_mut::<Outline>(node_id) {
//             outline.offset = value;
//         }
//     }
// }
// #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
// pub struct outline_color;
//
// impl ElementAttr<WebRenderer> for outline_color {
//     type Value = Color;
//
//     fn first_set_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         let mut entity_world_mut = world.entity_mut(node_id);
//         let value = value.into();
//         if entity_world_mut.contains::<Outline>() {
//             let mut outline = entity_world_mut.get_mut::<Outline>().unwrap();
//             outline.color = value;
//         } else {
//             entity_world_mut.insert(Outline {
//                 offset: Val::Px(0.),
//                 color: value,
//                 width: Val::Px(0.),
//             });
//         }
//     }
//
//     const NAME: &'static str = stringify!(outline_color);
//     fn update_value(
//         world: &mut RendererWorld<WebRenderer>,
//         node_id: RendererNodeId<WebRenderer>,
//         value: impl Into<Self::Value>,
//     ) {
//         let value = value.into();
//         if let Some(mut outline) = world.get_mut::<Outline>(node_id) {
//             outline.color = value;
//         }
//     }
// }
