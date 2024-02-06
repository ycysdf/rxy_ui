// use bevy_render::prelude::Color;
// use bevy_render::view::Visibility;
// use bevy_text::{BreakLineOn, TextAlignment};
// use bevy_ui::Display;
// use bevy_ui::{
//     AlignContent, AlignItems, AlignSelf, BackgroundColor, BorderColor, FlexDirection, FlexWrap,
//     JustifyContent, JustifyItems, JustifySelf, OverflowAxis, PositionType,
// };
// use bevy_ui::{Direction, Val};
//
// use crate::into_attr_value::{OptionalOverflow, UiOptionalRect};
//
// pub fn parse_color(class: &str) -> Option<Color> {
//     // Split the class into components
//     let components: Vec<&str> = class.split('/').collect();
//     let color_and_variant: Vec<&str> = components[0].split('-').collect();
//
//     // If there's an alpha channel specified, get it
//     let alpha = if components.len() > 1 {
//         match components[1].parse::<u16>() {
//             // convert from 100 to 255k
//             Ok(a) => (a * 255 / 100) as u8,
//             Err(_) => return None, // Invalid alpha
//         }
//     } else {
//         255 // Default alpha
//     };
//
//     // Handle special colors
//     if color_and_variant.len() == 1 {
//         return match color_and_variant[0] {
//             "transparent" => Some(Color::rgba_u8(0, 0, 0, 0)),
//             "white" => Some(Color::rgba_u8(255, 255, 255, alpha)),
//             "black" => Some(Color::rgba_u8(0, 0, 0, alpha)),
//             _ => None, // _ => COLORS.get(color_and_variant[0]).map(|c| {
//                        //     let (_, variant) = c.iter().next().unwrap();
//                        //     Color::rgba_u8(variant[0], variant[1], variant[2], alpha)
//                        // }),
//         };
//     }
//
//     None
//     // Handle regular colors
//     // let color = color_and_variant[0];
//     // let variant = color_and_variant[1];
//     //
//     // COLORS.get(color).and_then(|variants| {
//     //     variants
//     //         .get(variant)
//     //         .map(|&[r, g, b, _]| Color::rgba_u8(r, g, b, alpha))
//     // })
// }
//
// pub fn parse_size_val(text: &str) -> Val {
//     match text {
//         "full" => Val::Percent(100.),
//         "auto" => Val::Auto,
//         class => {
//             if class.ends_with('%') {
//                 Val::Percent(
//                     class
//                         .strip_suffix('%')
//                         .unwrap()
//                         .parse::<f32>()
//                         .unwrap_or(0.0)
//                         / 100.0,
//                 )
//             } else if class.ends_with("px") {
//                 Val::Px(class.parse::<f32>().unwrap_or(0.0))
//             } else {
//                 class.parse::<f32>().map(Val::Px).unwrap_or_else(|_| {
//                     // todo: tracing
//                     // warn!("Invalid size value: {}", class);
//                     Val::Auto
//                 })
//             }
//         }
//     }
// }
//
// pub trait MyFromStr: Sized {
//     fn from_str(s: &str) -> Option<Self>;
// }
//
// pub fn from_str<T: MyFromStr>(s: &str) -> Option<T> {
//     T::from_str(s)
// }
//
// impl MyFromStr for BorderColor {
//     fn from_str(s: &str) -> Option<Self> {
//         from_str::<Color>(s).map(BorderColor)
//     }
// }
//
// impl MyFromStr for BackgroundColor {
//     fn from_str(s: &str) -> Option<Self> {
//         from_str::<Color>(s).map(BackgroundColor)
//     }
// }
//
// impl MyFromStr for Color {
//     fn from_str(s: &str) -> Option<Self> {
//         parse_color(s)
//     }
// }
//
// impl MyFromStr for UiOptionalRect {
//     fn from_str(s: &str) -> Option<Self> {
//         let mut split = s.split_whitespace().map(parse_size_val).collect::<Vec<_>>();
//         match split.len() {
//             1 => Some(UiOptionalRect::all(split.pop().unwrap())),
//             2 => {
//                 let first = split.pop().unwrap();
//                 let second = split.pop().unwrap();
//                 Some(UiOptionalRect::axes(second, first))
//             }
//             3 => {
//                 let first = split.pop().unwrap();
//                 let second = split.pop().unwrap();
//                 let three = split.pop().unwrap();
//                 Some(UiOptionalRect::new(second, second, first, three))
//             }
//             4 => {
//                 let first = split.pop().unwrap();
//                 let second = split.pop().unwrap();
//                 let three = split.pop().unwrap();
//                 let four = split.pop().unwrap();
//                 Some(UiOptionalRect::new(four, second, first, three))
//             }
//             _ => None,
//         }
//     }
// }
//
// impl MyFromStr for Display {
//     fn from_str(s: &str) -> Option<Self> {
//         match s {
//             "flex" => Some(Display::Flex),
//             "grid" => Some(Display::Grid),
//             "none" => Some(Display::None),
//             _ => None,
//         }
//     }
// }
//
// impl MyFromStr for PositionType {
//     fn from_str(s: &str) -> Option<Self> {
//         match s {
//             "relative" => Some(PositionType::Relative),
//             "absolute" => Some(PositionType::Absolute),
//             _ => None,
//         }
//     }
// }
//
// impl MyFromStr for OptionalOverflow {
//     fn from_str(s: &str) -> Option<Self> {
//         match s {
//             "visible" | "visible visible" => Some(OptionalOverflow {
//                 x: Some(OverflowAxis::Visible),
//                 y: Some(OverflowAxis::Visible),
//             }),
//             "visible clip" => Some(OptionalOverflow {
//                 x: Some(OverflowAxis::Clip),
//                 y: Some(OverflowAxis::Clip),
//             }),
//             "clip" | "clip clip" => Some(OptionalOverflow {
//                 x: Some(OverflowAxis::Clip),
//                 y: Some(OverflowAxis::Clip),
//             }),
//             "clip visible" => Some(OptionalOverflow {
//                 x: Some(OverflowAxis::Clip),
//                 y: Some(OverflowAxis::Clip),
//             }),
//             _ => None,
//         }
//     }
// }
//
// impl MyFromStr for Direction {
//     fn from_str(s: &str) -> Option<Self> {
//         match s {
//             "ltr" => Some(Direction::LeftToRight),
//             "rtl" => Some(Direction::RightToLeft),
//             "inherit" => Some(Direction::Inherit),
//             _ => None,
//         }
//     }
// }
//
// impl MyFromStr for AlignItems {
//     fn from_str(s: &str) -> Option<Self> {
//         match s {
//             "default" => Some(AlignItems::Default),
//             "start" => Some(AlignItems::Start),
//             "end" => Some(AlignItems::End),
//             "flex-start" => Some(AlignItems::FlexStart),
//             "flex-end" => Some(AlignItems::FlexEnd),
//             "center" => Some(AlignItems::Center),
//             "baseline" => Some(AlignItems::Baseline),
//             "stretch" => Some(AlignItems::Stretch),
//             _ => None,
//         }
//     }
// }
//
// impl MyFromStr for JustifyItems {
//     fn from_str(s: &str) -> Option<Self> {
//         match s {
//             "default" => Some(JustifyItems::Default),
//             "start" => Some(JustifyItems::Start),
//             "end" => Some(JustifyItems::End),
//             "center" => Some(JustifyItems::Center),
//             "baseline" => Some(JustifyItems::Baseline),
//             "stretch" => Some(JustifyItems::Stretch),
//             _ => None,
//         }
//     }
// }
//
// impl MyFromStr for AlignSelf {
//     fn from_str(s: &str) -> Option<Self> {
//         match s {
//             "auto" => Some(AlignSelf::Auto),
//             "start" => Some(AlignSelf::Start),
//             "end" => Some(AlignSelf::End),
//             "flex-start" => Some(AlignSelf::FlexStart),
//             "flex-end" => Some(AlignSelf::FlexEnd),
//             "center" => Some(AlignSelf::Center),
//             "baseline" => Some(AlignSelf::Baseline),
//             "stretch" => Some(AlignSelf::Stretch),
//             _ => None,
//         }
//     }
// }
//
// impl MyFromStr for JustifySelf {
//     fn from_str(s: &str) -> Option<Self> {
//         match s {
//             "auto" => Some(JustifySelf::Auto),
//             "start" => Some(JustifySelf::Start),
//             "end" => Some(JustifySelf::End),
//             "center" => Some(JustifySelf::Center),
//             "baseline" => Some(JustifySelf::Baseline),
//             "stretch" => Some(JustifySelf::Stretch),
//             _ => None,
//         }
//     }
// }
//
// impl MyFromStr for AlignContent {
//     fn from_str(s: &str) -> Option<Self> {
//         match s {
//             "default" => Some(AlignContent::Default),
//             "start" => Some(AlignContent::Start),
//             "end" => Some(AlignContent::End),
//             "flex-start" => Some(AlignContent::FlexStart),
//             "flex-end" => Some(AlignContent::FlexEnd),
//             "center" => Some(AlignContent::Center),
//             "stretch" => Some(AlignContent::Stretch),
//             "space-evenly" => Some(AlignContent::SpaceEvenly),
//             "space-between" => Some(AlignContent::SpaceBetween),
//             "space-around" => Some(AlignContent::SpaceAround),
//             _ => None,
//         }
//     }
// }
//
// impl MyFromStr for JustifyContent {
//     fn from_str(s: &str) -> Option<Self> {
//         match s {
//             "default" => Some(JustifyContent::Default),
//             "start" => Some(JustifyContent::Start),
//             "end" => Some(JustifyContent::End),
//             "flex-start" => Some(JustifyContent::FlexStart),
//             "flex-end" => Some(JustifyContent::FlexEnd),
//             "center" => Some(JustifyContent::Center),
//             "space-evenly" => Some(JustifyContent::SpaceEvenly),
//             "space-between" => Some(JustifyContent::SpaceBetween),
//             "space-around" => Some(JustifyContent::SpaceAround),
//             _ => None,
//         }
//     }
// }
//
// impl MyFromStr for FlexDirection {
//     fn from_str(s: &str) -> Option<Self> {
//         match s {
//             "row" => Some(FlexDirection::Row),
//             "column" => Some(FlexDirection::Column),
//             "row-reverse" => Some(FlexDirection::RowReverse),
//             "column-reverse" => Some(FlexDirection::ColumnReverse),
//             _ => None,
//         }
//     }
// }
//
// impl MyFromStr for FlexWrap {
//     fn from_str(s: &str) -> Option<Self> {
//         match s {
//             "no-wrap" => Some(FlexWrap::NoWrap),
//             "wrap" => Some(FlexWrap::Wrap),
//             "wrap-reverse" => Some(FlexWrap::WrapReverse),
//             _ => None,
//         }
//     }
// }
//
// impl MyFromStr for Visibility {
//     fn from_str(s: &str) -> Option<Self> {
//         match s {
//             "visible" => Some(Visibility::Visible),
//             "hidden" => Some(Visibility::Hidden),
//             "inherit" => Some(Visibility::Inherited),
//             _ => None,
//         }
//     }
// }
//
// impl MyFromStr for BreakLineOn {
//     fn from_str(s: &str) -> Option<Self> {
//         match s {
//             "word-boundary" => Some(BreakLineOn::WordBoundary),
//             "any-character" => Some(BreakLineOn::AnyCharacter),
//             "no-wrap" => Some(BreakLineOn::NoWrap),
//             _ => None,
//         }
//     }
// }
//
// impl MyFromStr for TextAlignment {
//     fn from_str(s: &str) -> Option<Self> {
//         match s {
//             "left" => Some(TextAlignment::Left),
//             "center" => Some(TextAlignment::Center),
//             "right" => Some(TextAlignment::Right),
//             _ => None,
//         }
//     }
// }
//
// impl MyFromStr for OverflowAxis {
//     fn from_str(s: &str) -> Option<Self> {
//         match s {
//             "visible" => Some(OverflowAxis::Visible),
//             "clip" => Some(OverflowAxis::Clip),
//             _ => None,
//         }
//     }
// }
