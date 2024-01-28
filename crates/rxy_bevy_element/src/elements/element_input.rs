// #![allow(non_upper_case_globals)]
// #![allow(non_camel_case_types)]

// use std::any::TypeId;

// use bevy_cosmic_edit::*;
// use bevy_ecs::component::ComponentInfo;
// use bevy_ecs::prelude::*;
// use bevy_render::color::Color;
// use bevy_text::TextAlignment;
// use bevy_ui::node_bundles::NodeBundle;
// use bevy_ui::{BorderColor, Style, UiRect, Val};
// use bevy_utils::default;

// use crate::text_styled_element::TextStyledElementType;
// use crate::{all_attrs, ElementType, SetAttrValueContext};

// use super::*;

// pub fn bevy_color_to_cosmic(color: Color) -> CosmicColor {
//     CosmicColor::rgba(
//         (color.r() * 255.) as u8,
//         (color.g() * 255.) as u8,
//         (color.b() * 255.) as u8,
//         (color.a() * 255.) as u8,
//     )
// }

// impl ElementType for input {
//     fn update_entity(entity_mut: &mut EntityWorldMut) {
//         let attrs = AttrsOwned::new(Attrs::new().color(bevy_color_to_cosmic(Color::BLACK)));
//         let _placeholder_attrs = AttrsOwned::new(
//             Attrs::new().color(bevy_color_to_cosmic(Color::hex("#e6e6e6").unwrap())),
//         );
//         entity_mut.insert(NodeBundle::default());

//         entity_mut.insert((
//             Style {
//                 border: UiRect::all(Val::Px(1.)),
//                 width: Val::Px(100.),
//                 height: Val::Px(22.),
//                 ..default()
//             },
//             BorderColor(Color::DARK_GRAY),
//             CosmicEditBundle {
//                 text_position: CosmicTextPosition::Left { padding: 4 },
//                 attrs: CosmicAttrs(attrs.clone()),
//                 metrics: CosmicMetrics {
//                     font_size: 18.,
//                     line_height: 18. * 1.,
//                     scale_factor: 1.,
//                 },
//                 max_lines: CosmicMaxLines(1),
//                 text_setter: CosmicText::OneStyle(String::from("")),
//                 mode: CosmicMode::InfiniteLine,
//                 ..default()
//             },
//         ));
//     }

//     fn try_insert_no_reflect_components(
//         _entity_mut: &mut EntityWorldMut,
//         template_world: &World,
//         template_entity: Entity,
//         _type_registry: AppTypeRegistry,
//         _component_info: &ComponentInfo,
//     ) -> bool {
//         let type_id = ComponentInfo::type_id(_component_info).unwrap();

//         match type_id {
//             n if n == TypeId::of::<CosmicAttrs>() => {
//                 _entity_mut.insert(
//                     template_world
//                         .get::<CosmicAttrs>(template_entity)
//                         .cloned()
//                         .unwrap(),
//                 );
//             }
//             n if n == TypeId::of::<CosmicText>() => {
//                 _entity_mut.insert(
//                     template_world
//                         .get::<CosmicText>(template_entity)
//                         .cloned()
//                         .unwrap(),
//                 );
//             }
//             n if n == TypeId::of::<PlaceholderText>() => {
//                 _entity_mut.insert(
//                     template_world
//                         .get::<PlaceholderText>(template_entity)
//                         .cloned()
//                         .unwrap(),
//                 );
//             }
//             n if n == TypeId::of::<PlaceholderAttrs>() => {
//                 _entity_mut.insert(
//                     template_world
//                         .get::<PlaceholderAttrs>(template_entity)
//                         .cloned()
//                         .unwrap(),
//                 );
//             }
//             _ => return false,
//         }
//         true
//     }
// }

// impl TextStyledElementType for input {
//     fn set_font(&self, entity_ref: &mut EntityMut, _v: <all_attrs::font as ElementAttr>::Value) {
//         let Some(_attrs) = entity_ref.get_mut::<CosmicAttrs>() else {
//             return;
//         };
//         // todo: CosmicText font set
//         //        attrs.0.family_owned
//     }

//     fn set_font_size(
//         &self,
//         entity_ref: &mut EntityMut,
//         v: <all_attrs::font_size as ElementAttr>::Value,
//     ) {
//         let Some(mut metrics) = entity_ref.get_mut::<CosmicMetrics>() else {
//             return;
//         };
//         metrics.font_size = v;
//         metrics.line_height = v;
//     }

//     fn set_text_color(
//         &self,
//         entity_ref: &mut EntityMut,
//         v: <all_attrs::text_color as ElementAttr>::Value,
//     ) {
//         let Some(mut attrs) = entity_ref.get_mut::<CosmicAttrs>() else {
//             return;
//         };
//         attrs.0.color_opt = Some(bevy_color_to_cosmic(v));
//     }

//     fn set_text_linebreak(
//         &self,
//         _entity_ref: &mut EntityMut,
//         _v: <all_attrs::text_linebreak as ElementAttr>::Value,
//     ) {
//         // todo: CosmicText text_linebreak
//     }

//     fn set_text_align(
//         &self,
//         entity_ref: &mut EntityMut,
//         v: <all_attrs::text_align as ElementAttr>::Value,
//     ) {
//         let Some(mut pos) = entity_ref.get_mut::<CosmicTextPosition>() else {
//             return;
//         };
//         match v {
//             TextAlignment::Left => {
//                 if !matches!(*pos, CosmicTextPosition::Left { .. }) {
//                     *pos = CosmicTextPosition::Left { padding: 0 }
//                 }
//             }
//             TextAlignment::Center => {
//                 *pos = CosmicTextPosition::Center;
//             }
//             TextAlignment::Right => {
//                 //                if !matches!(*pos, CosmicTextPosition::Right { .. }) {
//                 //                    *pos = CosmicTextPosition::Right { padding: 0 }
//                 //                }
//             }
//         }
//     }
// }

// pub mod input_attrs {
//     use bevy_utils::tracing::warn;

//     use super::*;

//     #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
//     pub struct text_value;

//     impl ElementAttr for text_value {
//         type Value = String;

//         const NAME: &'static str = stringify!(value);

//         fn set_value(context: &mut SetAttrValueContext, p_value: impl Into<Self::Value>) {
//             if let Some(mut t) = context.entity_mut.get_mut::<CosmicText>() {
//                 *t = CosmicText::OneStyle(p_value.into());
//             } else {
//                 warn!("no found CosmicText component!");
//             }
//         }
//     }
// }
