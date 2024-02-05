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


