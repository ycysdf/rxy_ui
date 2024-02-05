#![allow(non_camel_case_types)]

use std::borrow::Cow;

use crate::ecs_fns::ElementStyleEntityExt;
use crate::element_core::ElementAttr;
use crate::SetAttrValueContext;
use bevy_asset::Handle;
use bevy_ecs::world::EntityWorldMut;
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
// use crate::tailwind::handle_classes;
use crate::text_styled_element::set_text_value;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct class;

impl ElementAttr for class {
    type Value = Cow<'static, str>;

    const NAME: &'static str = stringify!(class);

    fn set_value(_context: &mut SetAttrValueContext, _value: impl Into<Self::Value>) {
        // todo:
        // let value = value.into();
        // handle_classes(context, value.as_ref());
        // if !context.entity_extra_data().interaction_classes.is_empty()
        //     && !context.entity_mut.contains::<Interaction>()
        // {
        //     context.entity_mut.insert(Interaction::default());
        // }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct name;

impl ElementAttr for name {
    type Value = Cow<'static, str>;

    const NAME: &'static str = stringify!(name);

    fn init(entity_world_mut: &mut EntityWorldMut, value: impl Into<Self::Value>) {
        entity_world_mut.insert(bevy_core::Name::new(value.into()));
    }

    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        if let Some(mut n) = context.entity_mut.get_mut::<bevy_core::Name>() {
            *n = bevy_core::Name::new(value.into());
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct z_index;

impl ElementAttr for z_index {
    type Value = ZIndex;

    const NAME: &'static str = stringify!(z_index);

    fn init(entity_world_mut: &mut EntityWorldMut, value: impl Into<Self::Value>) {
        entity_world_mut.insert(value.into());
    }

    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        if let Some(mut z) = context.entity_mut.get_mut::<ZIndex>() {
            *z = value.into();
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct bg_color;

impl ElementAttr for bg_color {
    type Value = Color;

    const NAME: &'static str = stringify!(bg_color);

    fn init(entity_world_mut: &mut EntityWorldMut, value: impl Into<Self::Value>) {
        entity_world_mut.insert(value.into());
    }

    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        if let Some(mut bc) = context.entity_mut.get_mut::<BackgroundColor>() {
            *bc = BackgroundColor(value.into());
        }
        // let entity = context.entity_mut.id();
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

impl ElementAttr for border_left {
    type Value = Val;

    const NAME: &'static str = stringify!(border_left);

    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            let value = value.into();
            style.border.left = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct border_right;

impl ElementAttr for border_right {
    type Value = Val;

    const NAME: &'static str = stringify!(border_right);

    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            let value = value.into();
            style.border.right = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct border_top;

impl ElementAttr for border_top {
    type Value = Val;

    const NAME: &'static str = stringify!(border_top);

    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            let value = value.into();
            style.border.top = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct border_bottom;

impl ElementAttr for border_bottom {
    type Value = Val;

    const NAME: &'static str = stringify!(border_bottom);

    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            let value = value.into();
            style.border.bottom = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct border_color;

impl ElementAttr for border_color {
    type Value = Color;

    const NAME: &'static str = stringify!(border_color);

    fn init(entity_world_mut: &mut EntityWorldMut, value: impl Into<Self::Value>) {
        entity_world_mut.insert(value.into());
    }

    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        if let Some(mut br) = context.entity_mut.get_mut::<BorderColor>() {
            *br = BorderColor(value.into());
        }
        // context.entity_mut.insert(value.into());
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct display;

impl ElementAttr for display {
    type Value = bevy_ui::Display;

    const NAME: &'static str = stringify!(display);

    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.display = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct position_type;

impl ElementAttr for position_type {
    type Value = PositionType;

    const NAME: &'static str = stringify!(position_type);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.position_type = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct overflow_x;

impl ElementAttr for overflow_x {
    type Value = OverflowAxis;

    const NAME: &'static str = stringify!(overflow_x);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            let value = value.into();
            style.overflow.x = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct overflow_y;

impl ElementAttr for overflow_y {
    type Value = OverflowAxis;

    const NAME: &'static str = stringify!(overflow_y);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            let value = value.into();
            style.overflow.y = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct direction;

impl ElementAttr for direction {
    type Value = Direction;

    const NAME: &'static str = stringify!(direction);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.direction = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct left;

impl ElementAttr for left {
    type Value = Val;

    const NAME: &'static str = stringify!(left);

    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.left = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct right;

impl ElementAttr for right {
    type Value = Val;

    const NAME: &'static str = stringify!(right);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.right = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct top;

impl ElementAttr for top {
    type Value = Val;

    const NAME: &'static str = stringify!(top);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.top = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct bottom;

impl ElementAttr for bottom {
    type Value = Val;

    const NAME: &'static str = stringify!(bottom);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.bottom = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct width;

impl ElementAttr for width {
    type Value = Val;

    const NAME: &'static str = stringify!(width);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.width = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct height;

impl ElementAttr for height {
    type Value = Val;

    const NAME: &'static str = stringify!(height);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.height = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct min_width;

impl ElementAttr for min_width {
    type Value = Val;

    const NAME: &'static str = stringify!(min_width);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.min_width = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct min_height;

impl ElementAttr for min_height {
    type Value = Val;

    const NAME: &'static str = stringify!(min_height);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.min_height = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct max_width;

impl ElementAttr for max_width {
    type Value = Val;

    const NAME: &'static str = stringify!(max_width);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.max_width = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct max_height;

impl ElementAttr for max_height {
    type Value = Val;

    const NAME: &'static str = stringify!(max_height);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.max_height = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct margin_left;

impl ElementAttr for margin_left {
    type Value = Val;

    const NAME: &'static str = stringify!(margin_left);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            let value = value.into();
            style.margin.left = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct margin_right;

impl ElementAttr for margin_right {
    type Value = Val;

    const NAME: &'static str = stringify!(margin_right);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            let value = value.into();
            style.margin.right = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct margin_top;

impl ElementAttr for margin_top {
    type Value = Val;

    const NAME: &'static str = stringify!(margin_top);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            let value = value.into();
            style.margin.top = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct margin_bottom;

impl ElementAttr for margin_bottom {
    type Value = Val;

    const NAME: &'static str = stringify!(margin_bottom);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            let value = value.into();
            style.margin.bottom = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct padding_left;

impl ElementAttr for padding_left {
    type Value = Val;

    const NAME: &'static str = stringify!(padding_left);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            let value = value.into();
            style.padding.left = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct padding_right;

impl ElementAttr for padding_right {
    type Value = Val;

    const NAME: &'static str = stringify!(padding_right);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            let value = value.into();
            style.padding.right = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct padding_top;

impl ElementAttr for padding_top {
    type Value = Val;

    const NAME: &'static str = stringify!(padding_top);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            let value = value.into();
            style.padding.top = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct padding_bottom;

impl ElementAttr for padding_bottom {
    type Value = Val;

    const NAME: &'static str = stringify!(padding_bottom);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            let value = value.into();
            style.padding.bottom = value;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct aspect_ratio;

impl ElementAttr for aspect_ratio {
    type Value = Option<f32>;

    const NAME: &'static str = stringify!(aspect_ratio);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.aspect_ratio = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct align_items;

impl ElementAttr for align_items {
    type Value = AlignItems;

    const NAME: &'static str = stringify!(align_items);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.align_items = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct justify_items;

impl ElementAttr for justify_items {
    type Value = JustifyItems;

    const NAME: &'static str = stringify!(justify_items);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.justify_items = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct align_self;

impl ElementAttr for align_self {
    type Value = AlignSelf;

    const NAME: &'static str = stringify!(align_self);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.align_self = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct justify_self;

impl ElementAttr for justify_self {
    type Value = JustifySelf;

    const NAME: &'static str = stringify!(justify_self);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.justify_self = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct align_content;

impl ElementAttr for align_content {
    type Value = AlignContent;

    const NAME: &'static str = stringify!(align_content);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.align_content = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct justify_content;

impl ElementAttr for justify_content {
    type Value = JustifyContent;

    const NAME: &'static str = stringify!(justify_content);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.justify_content = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct flex_direction;

impl ElementAttr for flex_direction {
    type Value = FlexDirection;

    const NAME: &'static str = stringify!(flex_direction);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.flex_direction = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct flex_wrap;

impl ElementAttr for flex_wrap {
    type Value = FlexWrap;

    const NAME: &'static str = stringify!(flex_wrap);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.flex_wrap = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct flex_grow;

impl ElementAttr for flex_grow {
    type Value = f32;

    const NAME: &'static str = stringify!(flex_grow);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.flex_grow = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct flex_shrink;

impl ElementAttr for flex_shrink {
    type Value = f32;

    const NAME: &'static str = stringify!(flex_shrink);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.flex_shrink = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct flex_basis;

impl ElementAttr for flex_basis {
    type Value = Val;

    const NAME: &'static str = stringify!(flex_basis);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.flex_basis = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct column_gap;

impl ElementAttr for column_gap {
    type Value = Val;

    const NAME: &'static str = stringify!(column_gap);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.column_gap = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct row_gap;

impl ElementAttr for row_gap {
    type Value = Val;

    const NAME: &'static str = stringify!(row_gap);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_mut.try_set_style(|style| {
            style.row_gap = value.into();
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct visibility;

impl ElementAttr for visibility {
    type Value = Visibility;

    const NAME: &'static str = stringify!(visibility);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        if let Some(mut v) = context.entity_mut.get_mut::<Visibility>() {
            *v = value.into();
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct translation;

impl ElementAttr for translation {
    type Value = Vec3;

    const NAME: &'static str = stringify!(translation);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        if let Some(mut tf) = context.entity_mut.get_mut::<Transform>() {
            tf.translation = value;
        } else {
            warn!("no found Transform component!");
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct rotation;

impl ElementAttr for rotation {
    type Value = Quat;

    const NAME: &'static str = stringify!(rotation);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        if let Some(mut tf) = context.entity_mut.get_mut::<Transform>() {
            tf.rotation = value;
        } else {
            warn!("no found Transform component!");
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct scale;

impl ElementAttr for scale {
    type Value = Vec3;

    const NAME: &'static str = stringify!(scale);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        if let Some(mut tf) = context.entity_mut.get_mut::<Transform>() {
            tf.scale = value;
        } else {
            warn!("no found Transform component!");
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct text_color;

impl ElementAttr for text_color {
    type Value = Color;

    const NAME: &'static str = stringify!(text_color);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        set_text_value(context, |text_schema_type, entity_ref| {
            text_schema_type.set_text_color(entity_ref, value);
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct font_size;

impl ElementAttr for font_size {
    type Value = f32;

    const NAME: &'static str = stringify!(font_size);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        set_text_value(context, |text_schema_type, entity_ref| {
            text_schema_type.set_font_size(entity_ref, value);
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct text_linebreak;

impl ElementAttr for text_linebreak {
    type Value = BreakLineOn;

    const NAME: &'static str = stringify!(text_linebreak);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        set_text_value(context, |text_schema_type, entity_ref| {
            text_schema_type.set_text_linebreak(entity_ref, value);
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct text_align;

impl ElementAttr for text_align {
    type Value = TextAlignment;

    const NAME: &'static str = stringify!(text_align);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        set_text_value(context, |text_schema_type, entity_ref| {
            text_schema_type.set_text_align(entity_ref, value);
        });
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct font;

impl ElementAttr for font {
    type Value = Handle<Font>;

    const NAME: &'static str = stringify!(font);
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        set_text_value(context, |text_schema_type, entity_ref| {
            text_schema_type.set_font(entity_ref, value.clone());
        });
    }
}
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct outline_width;

impl ElementAttr for outline_width {
    type Value = Val;

    const NAME: &'static str = stringify!(outline_width);

    fn init(entity_world_mut: &mut EntityWorldMut, value: impl Into<Self::Value>) {
        let value = value.into();
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

    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        if let Some(mut outline) = context.entity_mut.get_mut::<Outline>() {
            outline.width = value;
        }
    }
}
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct outline_offset;

impl ElementAttr for outline_offset {
    type Value = Val;
    fn init(entity_world_mut: &mut EntityWorldMut, value: impl Into<Self::Value>) {
        let value = value.into();
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
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        if let Some(mut outline) = context.entity_mut.get_mut::<Outline>() {
            outline.offset = value;
        }
    }
}
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct outline_color;

impl ElementAttr for outline_color {
    type Value = Color;

    fn init(entity_world_mut: &mut EntityWorldMut, value: impl Into<Self::Value>) {
        let value = value.into();
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
    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        if let Some(mut outline) = context.entity_mut.get_mut::<Outline>() {
            outline.color = value;
        }
    }
}
