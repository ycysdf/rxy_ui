#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use bevy_text::Text;
use bevy_ui::prelude::TextBundle;

use rxy_core::{ElementAttrUntyped, ElementType, RendererNodeId, RendererWorld};

use crate::{BevyRenderer, BevyWorldExt};
use crate::all_attrs::CommonAttrs;

use super::*;

// use crate::all_attrs;
// use crate::text_styled_element::TextStyledElementType;

#[derive(Reflect, Debug, Clone, Copy)]
// #[reflect(TextStyledElementType)]
pub struct element_span;

impl ElementType<BevyRenderer> for element_span {
    const TAG_NAME: &'static str = "span";

    const ATTRS: &'static [&'static [&'static dyn ElementAttrUntyped<BevyRenderer>]] = &[
        <element_div as CommonAttrs>::ATTRS,
    ];

    fn spawn(
        world: &mut RendererWorld<BevyRenderer>,
        parent: Option<&RendererNodeId<BevyRenderer>>,
        reserve_node_id: Option<RendererNodeId<BevyRenderer>>,
    ) -> RendererNodeId<BevyRenderer> {
        let mut entity_world_mut = world.get_or_spawn_empty(parent, reserve_node_id);
        entity_world_mut.insert(TextBundle::default());
        entity_world_mut.id()
    }
}
//
// impl TextStyledElementType for text {
//     fn set_font(&self, entity_ref: &mut EntityMut, value: <all_attrs::font as ElementAttr>::Value) {
//         let Some(mut t) = entity_ref.get_mut::<Text>() else {
//             return;
//         };
//         for section in t.sections.iter_mut() {
//             section.style.font = value.clone();
//         }
//     }
//
//     fn set_font_size(
//         &self,
//         entity_ref: &mut EntityMut,
//         value: <all_attrs::font_size as ElementAttr>::Value,
//     ) {
//         let Some(mut t) = entity_ref.get_mut::<Text>() else {
//             return;
//         };
//         for section in t.sections.iter_mut() {
//             section.style.font_size = value;
//         }
//     }
//
//     fn set_text_color(
//         &self,
//         entity_ref: &mut EntityMut,
//         value: <all_attrs::text_color as ElementAttr>::Value,
//     ) {
//         let Some(mut t) = entity_ref.get_mut::<Text>() else {
//             return;
//         };
//         for section in t.sections.iter_mut() {
//             section.style.color = value;
//         }
//     }
//
//     fn set_text_linebreak(
//         &self,
//         entity_ref: &mut EntityMut,
//         value: <all_attrs::text_linebreak as ElementAttr>::Value,
//     ) {
//         let Some(mut t) = entity_ref.get_mut::<Text>() else {
//             return;
//         };
//         t.linebreak_behavior = value;
//     }
//
//     fn set_text_align(
//         &self,
//         entity_ref: &mut EntityMut,
//         value: <all_attrs::text_align as ElementAttr>::Value,
//     ) {
//         let Some(mut t) = entity_ref.get_mut::<Text>() else {
//             return;
//         };
//         t.alignment = value;
//     }
// }

pub mod span_attrs {
    use bevy_text::TextStyle;

    use rxy_core::{AttrIndex, ElementAttr};

    use super::*;

    /*pub struct sections;

    impl ElementAttr for sections {
        type Value = TextSections;

        const NAME: &'static str = "sections";

        fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
            if let Some(mut t) = context.entity_mut.get_mut::<Text>() {
                t.sections = value.into().0;
                if !context.entity_mut.contains::<TextFlags>() {
                    context.entity_mut.insert(TextFlags::default());
                }
                if !context.entity_mut.contains::<TextLayoutInfo>() {
                    context.entity_mut.insert(TextLayoutInfo::default());
                }
            } else {
                context.entity_mut.insert((
                    Text::from_sections(value.into().0),
                    TextFlags::default(),
                    TextLayoutInfo::default(),
                ));
            }
        }
    }*/

    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    pub struct content;

    impl ElementAttr<BevyRenderer> for content {
        type Value = String;

        const NAME: &'static str = stringify!(content);

        fn update_value(
            world: &mut RendererWorld<BevyRenderer>,
            node_id: RendererNodeId<BevyRenderer>,
            value: impl Into<Self::Value>,
        ) {
            let Some(mut t) = world.get_mut::<Text>(node_id) else {
                return;
            };
            if t.sections.is_empty() {
                t.sections = vec![bevy_text::TextSection::new(
                    value.into(),
                    TextStyle::default(),
                )];
            } else if t.sections.len() == 1 {
                t.sections[0].value = value.into();
            } else {
                t.sections[0].value = value.into();
                unsafe {
                    t.sections.set_len(1);
                }
            }
        }
    }
}
