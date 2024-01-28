#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::any::TypeId;

use bevy_ecs::component::ComponentInfo;
use bevy_ecs::prelude::*;
use bevy_text::{Text, TextLayoutInfo};
use bevy_ui::prelude::TextBundle;

use crate::all_attrs;
use crate::text_styled_element::TextStyledElementType;

use super::*;

impl ElementType for text {
    fn update_entity<'w>(entity_mut: &'w mut EntityWorldMut) {
        entity_mut.insert(TextBundle::default());
    }

    fn try_insert_no_reflect_components(
        _entity_mut: &mut EntityWorldMut,
        _template_world: &World,
        _template_entity: Entity,
        _type_registry: AppTypeRegistry,
        _component_info: &ComponentInfo,
    ) -> bool {
        let type_id = ComponentInfo::type_id(_component_info).unwrap();

        match type_id {
            n if n == TypeId::of::<TextLayoutInfo>() => {
                _entity_mut.insert(TextLayoutInfo::default());
            }
            _ => return false,
        }
        true
    }
}

impl TextStyledElementType for text {
    fn set_font(
        &self,
        entity_ref: &mut EntityMut,
        value: <all_attrs::font as ElementAttr>::Value,
    ) {
        let Some(mut t) = entity_ref.get_mut::<Text>() else {
            return;
        };
        for section in t.sections.iter_mut() {
            section.style.font = value.clone();
        }
    }

    fn set_font_size(
        &self,
        entity_ref: &mut EntityMut,
        value: <all_attrs::font_size as ElementAttr>::Value,
    ) {
        let Some(mut t) = entity_ref.get_mut::<Text>() else {
            return;
        };
        for section in t.sections.iter_mut() {
            section.style.font_size = value;
        }
    }

    fn set_text_color(
        &self,
        entity_ref: &mut EntityMut,
        value: <all_attrs::text_color as ElementAttr>::Value,
    ) {
        let Some(mut t) = entity_ref.get_mut::<Text>() else {
            return;
        };
        for section in t.sections.iter_mut() {
            section.style.color = value;
        }
    }

    fn set_text_linebreak(
        &self,
        entity_ref: &mut EntityMut,
        value: <all_attrs::text_linebreak as ElementAttr>::Value,
    ) {
        let Some(mut t) = entity_ref.get_mut::<Text>() else {
            return;
        };
        t.linebreak_behavior = value;
    }

    fn set_text_align(
        &self,
        entity_ref: &mut EntityMut,
        value: <all_attrs::text_align as ElementAttr>::Value,
    ) {
        let Some(mut t) = entity_ref.get_mut::<Text>() else {
            return;
        };
        t.alignment = value;
    }
}

pub mod text_attrs {
    use bevy_text::TextStyle;

    use crate::SetAttrValueContext;

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

    impl ElementAttr for content {
        type Value = String;

        const NAME: &'static str = stringify!(content);

        fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
            let Some(mut t) = context.entity_mut.get_mut::<Text>() else {
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
