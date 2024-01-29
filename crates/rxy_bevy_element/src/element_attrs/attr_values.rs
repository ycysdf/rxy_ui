use bevy_app::App;
use bevy_asset::Handle;
use bevy_reflect::prelude::*;
use bevy_render::color::Color;
use bevy_render::texture::Image;
use bevy_render::view::Visibility;
use bevy_sprite::TextureAtlas;
use bevy_text::{BreakLineOn, TextAlignment, TextSection, TextStyle};
use bevy_transform::components::Transform;
use bevy_ui::{AlignContent, AlignItems, AlignSelf, BackgroundColor, BorderColor, Direction, Display, FlexDirection, FlexWrap, JustifyContent, JustifyItems, JustifySelf, OverflowAxis, PositionType, Val, ZIndex};
use bevy_utils::tracing::warn;
use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::element_attrs::MyFromStr;
use crate::element_core::AttrValue;
use crate::smallbox::S1;
use crate::{from_str, impl_default_attr_value, impl_default_attr_values, smallbox, SmallBox};



pub fn parse_color(class: &str) -> Option<Color> {
    // Split the class into components
    let components: Vec<&str> = class.split('/').collect();
    let color_and_variant: Vec<&str> = components[0].split('-').collect();

    // If there's an alpha channel specified, get it
    let alpha = if components.len() > 1 {
        match components[1].parse::<u16>() {
            // convert from 100 to 255k
            Ok(a) => (a * 255 / 100) as u8,
            Err(_) => return None, // Invalid alpha
        }
    } else {
        255 // Default alpha
    };

    // Handle special colors
    if color_and_variant.len() == 1 {
        return match color_and_variant[0] {
            "transparent" => Some(Color::rgba_u8(0, 0, 0, 0)),
            "white" => Some(Color::rgba_u8(255, 255, 255, alpha)),
            "black" => Some(Color::rgba_u8(0, 0, 0, alpha)),
            _ => None
            // _ => COLORS.get(color_and_variant[0]).map(|c| {
            //     let (_, variant) = c.iter().next().unwrap();
            //     Color::rgba_u8(variant[0], variant[1], variant[2], alpha)
            // }),
        };
    }

    None
    // Handle regular colors
    // let color = color_and_variant[0];
    // let variant = color_and_variant[1];
    //
    // COLORS.get(color).and_then(|variants| {
    //     variants
    //         .get(variant)
    //         .map(|&[r, g, b, _]| Color::rgba_u8(r, g, b, alpha))
    // })
}
pub fn parse_size_val(text: &str) -> Val {
    match text {
        "full" => Val::Percent(100.),
        "auto" => Val::Auto,
        class => {
            if class.ends_with('%') {
                Val::Percent(
                    class
                        .strip_suffix('%')
                        .unwrap()
                        .parse::<f32>()
                        .unwrap_or(0.0)
                        / 100.0,
                )
            } else if class.ends_with("px") {
                Val::Px(class.parse::<f32>().unwrap_or(0.0))
            } else {
                class.parse::<f32>().map(Val::Px).unwrap_or_else(|_| {
                    warn!("Invalid size value: {}", class);
                    Val::Auto
                })
            }
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug, Reflect, Serialize, Deserialize)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub struct OptionalOverflow {
    pub x: Option<OverflowAxis>,
    pub y: Option<OverflowAxis>,
}

impl MyFromStr for OverflowAxis {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "visible" => Some(OverflowAxis::Visible),
            "clip" => Some(OverflowAxis::Clip),
            _ => None,
        }
    }
}
#[allow(unused_macros)]
macro_rules! downcast_chain {
    ($value:ident,$type:ty,$($candidate_type:ty),*) => {
        {
            let r = <dyn Reflect>::downcast::<$type>($value).map(|n| *n);
            $(
                let r = r.or_else(|value| {
                    <dyn Reflect>::downcast::<$candidate_type>(value)
                                .map(|n| *n)
                                .map(Into::into)
                });
                )*
            r.ok()
        }
    };
    ($type:ty) => {
        downcast_chain!($type,);
    }
}
#[allow(unused_macros)]
macro_rules! impl_from_attr_value {
    ($type:ty,$($candidate_type:ty),*) => {
        impl From<DomAttributeValue> for Option<$type> {
            fn from(value: DomAttributeValue) -> Self {
                match value {
                    DomAttributeValue::Text(value) => from_str(&value),
                    DomAttributeValue::Any(value) => {
                        downcast_chain!(value,$type,$($candidate_type)*)
                    }
                    _ => None,
                }
            }
        }
    };
    ($type:ty) => {
        impl_from_attr_value!($type,);
    }
}

#[derive(Debug, PartialEq, Default, Clone, Copy, Reflect)]
#[reflect(Default, PartialEq)]
pub struct OptionalTransform {
    pub translation: Option<Vec3>,
    pub rotation: Option<Quat>,
    pub scale: Option<Vec3>,
}

impl OptionalTransform {
    pub fn is_some(&self) -> [bool; 3] {
        [
            self.translation.is_some(),
            self.rotation.is_some(),
            self.scale.is_some(),
        ]
    }
}

#[derive(Copy, Clone, Default, PartialEq, Debug, Reflect)]
#[reflect(PartialEq)]
pub struct UiOptionalRect {
    pub left: Option<Val>,
    pub right: Option<Val>,
    pub top: Option<Val>,
    pub bottom: Option<Val>,
}

impl UiOptionalRect {
    pub fn all(val: Val) -> Self {
        Self {
            left: Some(val),
            right: Some(val),
            top: Some(val),
            bottom: Some(val),
        }
    }
    pub fn values(&self) -> [&Option<Val>; 4] {
        [&self.left, &self.right, &self.top, &self.bottom]
    }
    pub fn zero() -> Self {
        Self {
            left: Some(Val::Px(0.)),
            right: Some(Val::Px(0.)),
            top: Some(Val::Px(0.)),
            bottom: Some(Val::Px(0.)),
        }
    }

    pub const fn new(left: Val, right: Val, top: Val, bottom: Val) -> Self {
        Self {
            left: Some(left),
            right: Some(right),
            top: Some(top),
            bottom: Some(bottom),
        }
    }

    pub const fn px(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left: Some(Val::Px(left)),
            right: Some(Val::Px(right)),
            top: Some(Val::Px(top)),
            bottom: Some(Val::Px(bottom)),
        }
    }

    pub const fn percent(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left: Some(Val::Percent(left)),
            right: Some(Val::Percent(right)),
            top: Some(Val::Percent(top)),
            bottom: Some(Val::Percent(bottom)),
        }
    }

    pub fn horizontal(value: Val) -> Self {
        Self {
            left: Some(value),
            right: Some(value),
            ..Default::default()
        }
    }

    pub fn vertical(value: Val) -> Self {
        Self {
            top: Some(value),
            bottom: Some(value),
            ..Default::default()
        }
    }

    pub fn axes(horizontal: Val, vertical: Val) -> Self {
        Self {
            left: Some(horizontal),
            right: Some(horizontal),
            top: Some(vertical),
            bottom: Some(vertical),
        }
    }

    pub fn left(value: Val) -> Self {
        Self {
            left: Some(value),
            ..Default::default()
        }
    }

    pub fn right(value: Val) -> Self {
        Self {
            right: Some(value),
            ..Default::default()
        }
    }

    pub fn top(value: Val) -> Self {
        Self {
            top: Some(value),
            ..Default::default()
        }
    }

    pub fn bottom(value: Val) -> Self {
        Self {
            bottom: Some(value),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Reflect, Clone)]
pub struct TextSections(pub Vec<TextSection>);

impl PartialEq for TextSections {
    fn eq(&self, other: &Self) -> bool {
        self.reflect_partial_eq(other).unwrap_or(false)
    }
}

impl From<String> for TextSections {
    fn from(value: String) -> Self {
        Self(vec![TextSection::new(value, TextStyle::default())])
    }
}

impl<'a> From<&'a str> for TextSections {
    fn from(value: &'a str) -> Self {
        Self(vec![TextSection::new(value, TextStyle::default())])
    }
}

#[derive(Reflect, Debug, Clone, PartialEq)]
#[reflect(FromReflect)]
pub enum UiTexture {
    Color(Color),
    Image {
        image: Handle<Image>,
        flip_x: bool,
        flip_y: bool,
        color: Color,
    },
    Atlas {
        atlas: Handle<TextureAtlas>,
        index: usize,
        flip_x: bool,
        flip_y: bool,
        color: Color,
    },
}

impl From<Color> for UiTexture {
    fn from(value: Color) -> Self {
        UiTexture::Color(value)
    }
}

impl MyFromStr for UiTexture {
    fn from_str(s: &str) -> Option<Self> {
        parse_color(s).map(UiTexture::Color)
    }
}

impl MyFromStr for BorderColor {
    fn from_str(s: &str) -> Option<Self> {
        from_str::<Color>(s).map(BorderColor)
    }
}

impl MyFromStr for BackgroundColor {
    fn from_str(s: &str) -> Option<Self> {
        from_str::<Color>(s).map(BackgroundColor)
    }
}

impl MyFromStr for Color {
    fn from_str(s: &str) -> Option<Self> {
        parse_color(s)
    }
}

impl MyFromStr for UiOptionalRect {
    fn from_str(s: &str) -> Option<Self> {
        let mut split = s.split_whitespace().map(parse_size_val).collect::<Vec<_>>();
        match split.len() {
            1 => Some(UiOptionalRect::all(split.pop().unwrap())),
            2 => {
                let first = split.pop().unwrap();
                let second = split.pop().unwrap();
                Some(UiOptionalRect::axes(second, first))
            }
            3 => {
                let first = split.pop().unwrap();
                let second = split.pop().unwrap();
                let three = split.pop().unwrap();
                Some(UiOptionalRect::new(second, second, first, three))
            }
            4 => {
                let first = split.pop().unwrap();
                let second = split.pop().unwrap();
                let three = split.pop().unwrap();
                let four = split.pop().unwrap();
                Some(UiOptionalRect::new(four, second, first, three))
            }
            _ => None,
        }
    }
}

impl MyFromStr for Display {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "flex" => Some(Display::Flex),
            "grid" => Some(Display::Grid),
            "none" => Some(Display::None),
            _ => None,
        }
    }
}

impl MyFromStr for PositionType {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "relative" => Some(PositionType::Relative),
            "absolute" => Some(PositionType::Absolute),
            _ => None,
        }
    }
}

impl MyFromStr for OptionalOverflow {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "visible" | "visible visible" => Some(OptionalOverflow {
                x: Some(OverflowAxis::Visible),
                y: Some(OverflowAxis::Visible),
            }),
            "visible clip" => Some(OptionalOverflow {
                x: Some(OverflowAxis::Clip),
                y: Some(OverflowAxis::Clip),
            }),
            "clip" | "clip clip" => Some(OptionalOverflow {
                x: Some(OverflowAxis::Clip),
                y: Some(OverflowAxis::Clip),
            }),
            "clip visible" => Some(OptionalOverflow {
                x: Some(OverflowAxis::Clip),
                y: Some(OverflowAxis::Clip),
            }),
            _ => None,
        }
    }
}

impl MyFromStr for Direction {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "ltr" => Some(Direction::LeftToRight),
            "rtl" => Some(Direction::RightToLeft),
            "inherit" => Some(Direction::Inherit),
            _ => None,
        }
    }
}

impl MyFromStr for AlignItems {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "default" => Some(AlignItems::Default),
            "start" => Some(AlignItems::Start),
            "end" => Some(AlignItems::End),
            "flex-start" => Some(AlignItems::FlexStart),
            "flex-end" => Some(AlignItems::FlexEnd),
            "center" => Some(AlignItems::Center),
            "baseline" => Some(AlignItems::Baseline),
            "stretch" => Some(AlignItems::Stretch),
            _ => None,
        }
    }
}

impl MyFromStr for JustifyItems {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "default" => Some(JustifyItems::Default),
            "start" => Some(JustifyItems::Start),
            "end" => Some(JustifyItems::End),
            "center" => Some(JustifyItems::Center),
            "baseline" => Some(JustifyItems::Baseline),
            "stretch" => Some(JustifyItems::Stretch),
            _ => None,
        }
    }
}

impl MyFromStr for AlignSelf {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "auto" => Some(AlignSelf::Auto),
            "start" => Some(AlignSelf::Start),
            "end" => Some(AlignSelf::End),
            "flex-start" => Some(AlignSelf::FlexStart),
            "flex-end" => Some(AlignSelf::FlexEnd),
            "center" => Some(AlignSelf::Center),
            "baseline" => Some(AlignSelf::Baseline),
            "stretch" => Some(AlignSelf::Stretch),
            _ => None,
        }
    }
}

impl MyFromStr for JustifySelf {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "auto" => Some(JustifySelf::Auto),
            "start" => Some(JustifySelf::Start),
            "end" => Some(JustifySelf::End),
            "center" => Some(JustifySelf::Center),
            "baseline" => Some(JustifySelf::Baseline),
            "stretch" => Some(JustifySelf::Stretch),
            _ => None,
        }
    }
}

impl MyFromStr for AlignContent {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "default" => Some(AlignContent::Default),
            "start" => Some(AlignContent::Start),
            "end" => Some(AlignContent::End),
            "flex-start" => Some(AlignContent::FlexStart),
            "flex-end" => Some(AlignContent::FlexEnd),
            "center" => Some(AlignContent::Center),
            "stretch" => Some(AlignContent::Stretch),
            "space-evenly" => Some(AlignContent::SpaceEvenly),
            "space-between" => Some(AlignContent::SpaceBetween),
            "space-around" => Some(AlignContent::SpaceAround),
            _ => None,
        }
    }
}

impl MyFromStr for JustifyContent {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "default" => Some(JustifyContent::Default),
            "start" => Some(JustifyContent::Start),
            "end" => Some(JustifyContent::End),
            "flex-start" => Some(JustifyContent::FlexStart),
            "flex-end" => Some(JustifyContent::FlexEnd),
            "center" => Some(JustifyContent::Center),
            "space-evenly" => Some(JustifyContent::SpaceEvenly),
            "space-between" => Some(JustifyContent::SpaceBetween),
            "space-around" => Some(JustifyContent::SpaceAround),
            _ => None,
        }
    }
}

impl MyFromStr for FlexDirection {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "row" => Some(FlexDirection::Row),
            "column" => Some(FlexDirection::Column),
            "row-reverse" => Some(FlexDirection::RowReverse),
            "column-reverse" => Some(FlexDirection::ColumnReverse),
            _ => None,
        }
    }
}

impl MyFromStr for FlexWrap {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "no-wrap" => Some(FlexWrap::NoWrap),
            "wrap" => Some(FlexWrap::Wrap),
            "wrap-reverse" => Some(FlexWrap::WrapReverse),
            _ => None,
        }
    }
}

impl MyFromStr for Visibility {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "visible" => Some(Visibility::Visible),
            "hidden" => Some(Visibility::Hidden),
            "inherit" => Some(Visibility::Inherited),
            _ => None,
        }
    }
}

impl MyFromStr for BreakLineOn {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "word-boundary" => Some(BreakLineOn::WordBoundary),
            "any-character" => Some(BreakLineOn::AnyCharacter),
            "no-wrap" => Some(BreakLineOn::NoWrap),
            _ => None,
        }
    }
}

impl MyFromStr for TextAlignment {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "left" => Some(TextAlignment::Left),
            "center" => Some(TextAlignment::Center),
            "right" => Some(TextAlignment::Right),
            _ => None,
        }
    }
}

impl AttrValue for UiOptionalRect {
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(*self)
    }
    fn default_value() -> Self {
        <Self as Default>::default()
    }

    fn merge_value(&mut self, value: Self) {
        self.left = self.left.or(value.left);
        self.right = self.right.or(value.right);
        self.top = self.top.or(value.top);
        self.bottom = self.bottom.or(value.bottom);
    }

    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl AttrValue for OptionalOverflow {
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(*self)
    }
    fn default_value() -> Self {
        <Self as Default>::default()
    }

    fn merge_value(&mut self, value: Self) {
        self.x = self.x.or(value.x);
        self.y = self.y.or(value.y);
    }

    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl AttrValue for OptionalTransform {
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(*self)
    }

    fn default_value() -> Self {
        <Self as Default>::default()
    }

    fn merge_value(&mut self, value: Self) {
        self.translation = self.translation.or(value.translation);
        self.rotation = self.rotation.or(value.rotation);
        self.scale = self.scale.or(value.scale);
    }

    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl AttrValue for ZIndex {
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(*self)
    }

    fn default_value() -> Self {
        <Self as Default>::default()
    }

    fn eq(&self, other: &Self) -> bool {
        match self {
            ZIndex::Local(i) => match other {
                ZIndex::Local(o_i) => i == o_i,
                ZIndex::Global(_) => false,
            },
            ZIndex::Global(i) => match other {
                ZIndex::Local(_) => false,
                ZIndex::Global(o_i) => i == o_i,
            },
        }
    }
}

impl AttrValue for BorderColor {
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(*self)
    }

    fn default_value() -> Self {
        BorderColor(Color::rgba_u8(0, 0, 0, 0))
    }

    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl AttrValue for BackgroundColor {
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(*self)
    }

    fn default_value() -> Self {
        BackgroundColor(Color::rgba_u8(0, 0, 0, 0))
    }

    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl_default_attr_values! {
    UiTexture: UiTexture::Color(Color::rgba_u8(0, 0, 0, 0)),
    Color: Color::rgba_u8(0, 0, 0, 0),
    BreakLineOn: BreakLineOn::WordBoundary,
    TextAlignment: TextAlignment::Left,
    Display,
    PositionType,
    Direction,
    AlignItems,
    JustifyItems,
    AlignSelf,
    JustifySelf,
    AlignContent,
    JustifyContent,
    FlexDirection,
    FlexWrap,
    Visibility,
    TextSections,
    Transform,
    Quat,
    Vec3,
    OverflowAxis
}

pub trait BevyAppAttrValueRegistryExt {
    fn register_attr_values(&mut self) -> &mut Self;
}

impl BevyAppAttrValueRegistryExt for App {
    fn register_attr_values(&mut self) -> &mut Self {
        self.register_type::<TextSections>()
            .register_type::<UiTexture>()
    }
}
