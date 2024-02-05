#![allow(non_camel_case_types)]

use std::borrow::Cow;

use crate::into_attr_value::BevyAttrValue;
use crate::renderer::text_styled_element::TextStyledElementEntityWorldMutExt;
use crate::{AttrSetBits, BevyRenderer, ElementStyleEntityExt};
use bevy_asset::Handle;
use bevy_render::color::Color;
use bevy_render::view::Visibility;
use bevy_text::{BreakLineOn, Font, TextAlignment};
use bevy_transform::components::Transform;
use bevy_ui::{
    AlignContent, AlignItems, AlignSelf, BackgroundColor, BorderColor, Direction, FlexDirection,
    FlexWrap, JustifyContent, JustifyItems, JustifySelf, Outline, OverflowAxis, PositionType, Val,
    ZIndex,
};
use bevy_utils::tracing::warn;
use glam::{Quat, Vec3};
use crate::elements::element_span;
use rxy_core::{AttrIndex, ElementAttr, HasIndex, RendererNodeId, RendererWorld};

#[macro_export]
macro_rules! common_attrs_fn_define {
    ($name:ident;$index_start:expr;$($attr:ident)*) => {
        count_macro::count! {
            $(
            impl rxy_core::HasIndex for $crate::all_attrs::$attr{
               const INDEX: rxy_core::AttrIndex = $index_start+_int_;
            }
            )*
        }

        #[allow(non_upper_case_globals)]
        #[allow(non_camel_case_types)]
        pub trait $name {
            const ATTRS: &'static [&'static dyn rxy_core::ElementAttrUntyped<$crate::BevyRenderer>] = &[
                $(&$crate::all_attrs::$attr,)*
            ];
        }

        paste::paste!{
            pub trait [<$name ViewBuilder>]: rxy_core::MemberOwner<$crate::BevyRenderer> + Sized {

                $(
                    fn $attr<T>(self, value: impl rxy_core::IntoViewMember<$crate::BevyRenderer, T>) -> Self::AddMember<T>
                    where
                        T: rxy_core::ElementAttrMember<$crate::BevyRenderer, EA = $crate::all_attrs::$attr>,
                        (Self::VM, T): rxy_core::ViewMember<$crate::BevyRenderer>
                    {
                        self.member(value)
                    }
                )*
            }

            impl<T: rxy_core::MemberOwner<$crate::BevyRenderer>> [<$name ViewBuilder>] for T {}
        }
    };
}
#[macro_export]
macro_rules! element_attrs_fn_define {
    ($name:ident;$element:ty;$index_start:expr;$($attr:ident)*) => {
        count_macro::count! {
            $(
            impl rxy_core::HasIndex for $crate::all_attrs::$attr{
               const INDEX: rxy_core::AttrIndex = $index_start+_int_;
            }
            )*
        }

        #[allow(non_upper_case_globals)]
        #[allow(non_camel_case_types)]
        pub trait $name {
            const ATTRS: &'static [&'static dyn rxy_core::ElementAttrUntyped<$crate::BevyRenderer>] = &[
                $(&$crate::all_attrs::$attr,)*
            ];
        }

        paste::paste!{
            pub trait [<$name ViewBuilder>]: rxy_core::MemberOwner<$crate::BevyRenderer> + Sized
            {

                $(
                    fn $attr<T>(self, value: impl rxy_core::IntoViewMember<$crate::BevyRenderer, T>) -> Self::AddMember<T>
                    where
                        T: rxy_core::ElementAttrMember<$crate::BevyRenderer, EA = $crate::all_attrs::$attr>,
                        (Self::VM, T): rxy_core::ViewMember<$crate::BevyRenderer>
                    {
                        self.member(value)
                    }
                )*
            }

            impl<T: rxy_core::MemberOwner<$crate::BevyRenderer,E=$element>> [<$name ViewBuilder>] for T {}
        }
    };
}

common_attrs_fn_define!(CommonAttrs;0;
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

const COMMON_ATTRS_COUNT: AttrIndex = <outline_color as HasIndex>::INDEX + 1;

element_attrs_fn_define!(TextAttrs;element_span;COMMON_ATTRS_COUNT - 1;
    content
);

// element_attrs_fn_define!(InputAttrs;elements::input;
//     text_value
// );

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct class;

impl ElementAttr<BevyRenderer> for class {
    type Value = Cow<'static, str>;

    const NAME: &'static str = stringify!(class);

    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        _value: impl Into<Self::Value>,
    ) {
        // todo:
        // let value = value.into().0;
        // handle_classes(context, value.as_ref());
        // if !context.entity_extra_data().interaction_classes.is_empty()
        //     && !world.contains::<Interaction>()
        // {
        //     world.insert(Interaction::default());
        // }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct name;

impl ElementAttr<BevyRenderer> for name {
    type Value = Cow<'static, str>;

    const NAME: &'static str = stringify!(name);

    fn first_set_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let mut entity_world_mut = world.entity_mut(node_id);
        entity_world_mut.insert(bevy_core::Name::new(value.into()));
    }

    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        if let Some(mut n) = world.get_mut::<bevy_core::Name>(node_id) {
            *n = bevy_core::Name::new(value.into());
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct z_index;

impl ElementAttr<BevyRenderer> for z_index {
    type Value = BevyAttrValue<ZIndex>;

    const NAME: &'static str = stringify!(z_index);

    fn first_set_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let mut entity_world_mut = world.entity_mut(node_id);
        entity_world_mut.insert(value.into().0);
    }

    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        if let Some(mut z) = world.get_mut::<ZIndex>(node_id) {
            *z = value.into().0;
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct bg_color;

impl ElementAttr<BevyRenderer> for bg_color {
    type Value = BevyAttrValue<Color>;

    const NAME: &'static str = stringify!(bg_color);

    fn first_set_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let mut entity_world_mut = world.entity_mut(node_id);
        entity_world_mut.insert(BackgroundColor(value.into().0));
    }

    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        if let Some(mut bc) = world.get_mut::<BackgroundColor>(node_id) {
            *bc = BackgroundColor(value.into().0);
        }
        // let entity = world.id();
        // context.commands.add(move |world| {
        //     let entity_mut = world.entity_mut(entity);
        //     match value.into() {
        //         UiTexture::Color(color) => {
        //             entity_mut.insert(BackgroundColor(color));
        //             entity_mut.remove::<(
        //                 UiImage,
        //                 UiImageSize,
        //                 Handle<TextureAtlas>,
        //                 UiTextureAtlasImage,
        //             )>();
        //         }
        //         UiTexture::Image {
        //             color,
        //             flip_x,
        //             flip_y,
        //             image: image_handle,
        //         } => {
        //             entity_mut.insert((
        //                 BackgroundColor(color),
        //                 UiImage {
        //                     texture: image_handle,
        //                     flip_y,
        //                     flip_x,
        //                 },
        //                 UiImageSize::default(),
        //             ));
        //             context
        //                 .entity_mut
        //                 .remove::<(Handle<TextureAtlas>, UiTextureAtlasImage)>();
        //         }
        //         UiTexture::Atlas {
        //             flip_y,
        //             flip_x,
        //             color,
        //             index,
        //             atlas,
        //         } => {
        //             entity_mut.insert((
        //                 BackgroundColor(color),
        //                 atlas,
        //                 UiTextureAtlasImage {
        //                     index,
        //                     flip_x,
        //                     flip_y,
        //                 },
        //             ));
        //             entity_mut.remove::<(UiImage, UiImageSize)>();
        //         }
        //     }
        // })
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct border_left;

impl ElementAttr<BevyRenderer> for border_left {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(border_left);

    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            let value = value.into().0;
            style.border.left = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct border_right;

impl ElementAttr<BevyRenderer> for border_right {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(border_right);

    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            let value = value.into().0;
            style.border.right = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct border_top;

impl ElementAttr<BevyRenderer> for border_top {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(border_top);

    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            let value = value.into().0;
            style.border.top = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct border_bottom;

impl ElementAttr<BevyRenderer> for border_bottom {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(border_bottom);

    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            let value = value.into().0;
            style.border.bottom = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct border_color;

impl ElementAttr<BevyRenderer> for border_color {
    type Value = BevyAttrValue<Color>;

    const NAME: &'static str = stringify!(border_color);

    fn first_set_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let mut entity_world_mut = world.entity_mut(node_id);
        entity_world_mut.insert(BorderColor(value.into().0));
    }

    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        if let Some(mut br) = world.get_mut::<BorderColor>(node_id) {
            *br = BorderColor(value.into().0);
        }
        // world.insert(value.into());
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct display;

impl ElementAttr<BevyRenderer> for display {
    type Value = BevyAttrValue<bevy_ui::Display>;

    const NAME: &'static str = stringify!(display);

    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.display = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct position_type;

impl ElementAttr<BevyRenderer> for position_type {
    type Value = BevyAttrValue<PositionType>;

    const NAME: &'static str = stringify!(position_type);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.position_type = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct overflow_x;

impl ElementAttr<BevyRenderer> for overflow_x {
    type Value = BevyAttrValue<OverflowAxis>;

    const NAME: &'static str = stringify!(overflow_x);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            let value = value.into().0;
            style.overflow.x = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct overflow_y;

impl ElementAttr<BevyRenderer> for overflow_y {
    type Value = BevyAttrValue<OverflowAxis>;

    const NAME: &'static str = stringify!(overflow_y);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            let value = value.into().0;
            style.overflow.y = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct direction;

impl ElementAttr<BevyRenderer> for direction {
    type Value = BevyAttrValue<Direction>;

    const NAME: &'static str = stringify!(direction);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.direction = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct left;

impl ElementAttr<BevyRenderer> for left {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(left);

    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.left = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct right;

impl ElementAttr<BevyRenderer> for right {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(right);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.right = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct top;

impl ElementAttr<BevyRenderer> for top {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(top);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.top = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct bottom;

impl ElementAttr<BevyRenderer> for bottom {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(bottom);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.bottom = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct width;

impl ElementAttr<BevyRenderer> for width {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(width);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.width = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct height;

impl ElementAttr<BevyRenderer> for height {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(height);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.height = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct min_width;

impl ElementAttr<BevyRenderer> for min_width {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(min_width);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.min_width = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct min_height;

impl ElementAttr<BevyRenderer> for min_height {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(min_height);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.min_height = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct max_width;

impl ElementAttr<BevyRenderer> for max_width {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(max_width);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.max_width = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct max_height;

impl ElementAttr<BevyRenderer> for max_height {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(max_height);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.max_height = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct margin_left;

impl ElementAttr<BevyRenderer> for margin_left {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(margin_left);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            let value = value.into().0;
            style.margin.left = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct margin_right;

impl ElementAttr<BevyRenderer> for margin_right {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(margin_right);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            let value = value.into().0;
            style.margin.right = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct margin_top;

impl ElementAttr<BevyRenderer> for margin_top {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(margin_top);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            let value = value.into().0;
            style.margin.top = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct margin_bottom;

impl ElementAttr<BevyRenderer> for margin_bottom {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(margin_bottom);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            let value = value.into().0;
            style.margin.bottom = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct padding_left;

impl ElementAttr<BevyRenderer> for padding_left {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(padding_left);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            let value = value.into().0;
            style.padding.left = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct padding_right;

impl ElementAttr<BevyRenderer> for padding_right {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(padding_right);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            let value = value.into().0;
            style.padding.right = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct padding_top;

impl ElementAttr<BevyRenderer> for padding_top {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(padding_top);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            let value = value.into().0;
            style.padding.top = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct padding_bottom;

impl ElementAttr<BevyRenderer> for padding_bottom {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(padding_bottom);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            let value = value.into().0;
            style.padding.bottom = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct aspect_ratio;

impl ElementAttr<BevyRenderer> for aspect_ratio {
    type Value = Option<f32>;

    const NAME: &'static str = stringify!(aspect_ratio);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.aspect_ratio = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct align_items;

impl ElementAttr<BevyRenderer> for align_items {
    type Value = BevyAttrValue<AlignItems>;

    const NAME: &'static str = stringify!(align_items);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.align_items = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct justify_items;

impl ElementAttr<BevyRenderer> for justify_items {
    type Value = BevyAttrValue<JustifyItems>;

    const NAME: &'static str = stringify!(justify_items);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.justify_items = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct align_self;

impl ElementAttr<BevyRenderer> for align_self {
    type Value = BevyAttrValue<AlignSelf>;

    const NAME: &'static str = stringify!(align_self);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.align_self = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct justify_self;

impl ElementAttr<BevyRenderer> for justify_self {
    type Value = BevyAttrValue<JustifySelf>;

    const NAME: &'static str = stringify!(justify_self);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.justify_self = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct align_content;

impl ElementAttr<BevyRenderer> for align_content {
    type Value = BevyAttrValue<AlignContent>;

    const NAME: &'static str = stringify!(align_content);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.align_content = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct justify_content;

impl ElementAttr<BevyRenderer> for justify_content {
    type Value = BevyAttrValue<JustifyContent>;

    const NAME: &'static str = stringify!(justify_content);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.justify_content = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct flex_direction;

impl ElementAttr<BevyRenderer> for flex_direction {
    type Value = BevyAttrValue<FlexDirection>;

    const NAME: &'static str = stringify!(flex_direction);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.flex_direction = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct flex_wrap;

impl ElementAttr<BevyRenderer> for flex_wrap {
    type Value = BevyAttrValue<FlexWrap>;

    const NAME: &'static str = stringify!(flex_wrap);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.flex_wrap = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct flex_grow;

impl ElementAttr<BevyRenderer> for flex_grow {
    type Value = f32;

    const NAME: &'static str = stringify!(flex_grow);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.flex_grow = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct flex_shrink;

impl ElementAttr<BevyRenderer> for flex_shrink {
    type Value = f32;

    const NAME: &'static str = stringify!(flex_shrink);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.flex_shrink = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct flex_basis;

impl ElementAttr<BevyRenderer> for flex_basis {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(flex_basis);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.flex_basis = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct column_gap;

impl ElementAttr<BevyRenderer> for column_gap {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(column_gap);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.column_gap = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct row_gap;

impl ElementAttr<BevyRenderer> for row_gap {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(row_gap);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        world.entity_mut(node_id).try_set_style(|style| {
            style.row_gap = value.into().0;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct visibility;

impl ElementAttr<BevyRenderer> for visibility {
    type Value = BevyAttrValue<Visibility>;

    const NAME: &'static str = stringify!(visibility);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        if let Some(mut v) = world.get_mut::<Visibility>(node_id) {
            *v = value.into().0;
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct translation;

impl ElementAttr<BevyRenderer> for translation {
    type Value = BevyAttrValue<Vec3>;

    const NAME: &'static str = stringify!(translation);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let value = value.into().0;
        if let Some(mut tf) = world.get_mut::<Transform>(node_id) {
            tf.translation = value;
        } else {
            warn!("no found Transform component!");
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct rotation;

impl ElementAttr<BevyRenderer> for rotation {
    type Value = BevyAttrValue<Quat>;

    const NAME: &'static str = stringify!(rotation);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let value = value.into().0;
        if let Some(mut tf) = world.get_mut::<Transform>(node_id) {
            tf.rotation = value;
        } else {
            warn!("no found Transform component!");
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct scale;

impl ElementAttr<BevyRenderer> for scale {
    type Value = BevyAttrValue<Vec3>;

    const NAME: &'static str = stringify!(scale);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let value = value.into().0;
        if let Some(mut tf) = world.get_mut::<Transform>(node_id) {
            tf.scale = value;
        } else {
            warn!("no found Transform component!");
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct text_color;

impl ElementAttr<BevyRenderer> for text_color {
    type Value = BevyAttrValue<Color>;

    const NAME: &'static str = stringify!(text_color);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let value = value.into();
        world.entity_mut(node_id).scoped_text_styled_element_type(
            |text_schema_type, entity_ref| {
                text_schema_type.set_text_color(entity_ref, value);
            },
        );
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct font_size;

impl ElementAttr<BevyRenderer> for font_size {
    type Value = f32;

    const NAME: &'static str = stringify!(font_size);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let value = value.into();
        world.entity_mut(node_id).scoped_text_styled_element_type(
            |text_schema_type, entity_ref| {
                text_schema_type.set_font_size(entity_ref, value);
            },
        );
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct text_linebreak;

impl ElementAttr<BevyRenderer> for text_linebreak {
    type Value = BevyAttrValue<BreakLineOn>;

    const NAME: &'static str = stringify!(text_linebreak);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let value = value.into();
        world.entity_mut(node_id).scoped_text_styled_element_type(
            |text_schema_type, entity_ref| {
                text_schema_type.set_text_linebreak(entity_ref, value);
            },
        );
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct text_align;

impl ElementAttr<BevyRenderer> for text_align {
    type Value = BevyAttrValue<TextAlignment>;

    const NAME: &'static str = stringify!(text_align);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let value = value.into();
        world.entity_mut(node_id).scoped_text_styled_element_type(
            |text_schema_type, entity_ref| {
                text_schema_type.set_text_align(entity_ref, value);
            },
        );
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct font;

impl ElementAttr<BevyRenderer> for font {
    type Value = BevyAttrValue<Handle<Font>>;

    const NAME: &'static str = stringify!(font);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let value = value.into();
        world.entity_mut(node_id).scoped_text_styled_element_type(
            |text_schema_type, entity_ref| {
                text_schema_type.set_font(entity_ref, value.clone());
            },
        );
    }
}
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct outline_width;

impl ElementAttr<BevyRenderer> for outline_width {
    type Value = BevyAttrValue<Val>;

    const NAME: &'static str = stringify!(outline_width);

    fn first_set_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let mut entity_world_mut = world.entity_mut(node_id);
        let value = value.into().0;
        if entity_world_mut.contains::<Outline>() {
            let mut outline = entity_world_mut.get_mut::<Outline>().unwrap();
            outline.width = value;
        } else {
            entity_world_mut.insert(Outline {
                width: value,
                color: Color::BLACK,
                offset: Val::Px(0.),
            });
        }
    }

    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let value = value.into().0;
        if let Some(mut outline) = world.get_mut::<Outline>(node_id) {
            outline.width = value;
        }
    }
}
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct outline_offset;

impl ElementAttr<BevyRenderer> for outline_offset {
    type Value = BevyAttrValue<Val>;
    fn first_set_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let mut entity_world_mut = world.entity_mut(node_id);
        let value = value.into().0;
        if entity_world_mut.contains::<Outline>() {
            let mut outline = entity_world_mut.get_mut::<Outline>().unwrap();
            outline.offset = value;
        } else {
            entity_world_mut.insert(Outline {
                offset: value,
                color: Color::BLACK,
                width: Val::Px(1.),
            });
        }
    }

    const NAME: &'static str = stringify!(outline_offset);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let value = value.into().0;
        if let Some(mut outline) = world.get_mut::<Outline>(node_id) {
            outline.offset = value;
        }
    }
}
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct outline_color;

impl ElementAttr<BevyRenderer> for outline_color {
    type Value = BevyAttrValue<Color>;

    fn first_set_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let mut entity_world_mut = world.entity_mut(node_id);
        let value = value.into().0;
        if entity_world_mut.contains::<Outline>() {
            let mut outline = entity_world_mut.get_mut::<Outline>().unwrap();
            outline.color = value;
        } else {
            entity_world_mut.insert(Outline {
                offset: Val::Px(0.),
                color: value,
                width: Val::Px(0.),
            });
        }
    }

    const NAME: &'static str = stringify!(outline_color);
    fn update_value(
        world: &mut RendererWorld<BevyRenderer>,
        node_id: RendererNodeId<BevyRenderer>,
        value: impl Into<Self::Value>,
    ) {
        let value = value.into().0;
        if let Some(mut outline) = world.get_mut::<Outline>(node_id) {
            outline.color = value;
        }
    }
}
