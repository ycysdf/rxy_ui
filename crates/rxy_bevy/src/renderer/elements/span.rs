#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use bevy_text::Text;
use bevy_ui::prelude::TextBundle;

use rxy_core::{
    ElementAttrType, ElementType, ElementTypeUnTyped, RendererNodeId,
    RendererWorld,
};

use crate::{
    all_attrs, BevyRenderer, BevyWorldExt, ReflectTextStyledElementType, TextStyledElementType,
};

#[derive(Reflect, Debug, Default, Clone, Copy)]
#[reflect(TextStyledElementType)]
pub struct element_span;

impl ElementType<BevyRenderer> for element_span {
    const TAG_NAME: &'static str = "span";

    fn get() -> &'static dyn ElementTypeUnTyped<BevyRenderer> {
        &element_span
    }

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

impl TextStyledElementType for element_span {
    fn set_font(
        &self,
        entity_ref: &mut EntityWorldMut<'_>,
        value: <all_attrs::font as ElementAttrType<BevyRenderer>>::Value,
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
        entity_ref: &mut EntityWorldMut<'_>,
        value: <all_attrs::font_size as ElementAttrType<BevyRenderer>>::Value,
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
        entity_ref: &mut EntityWorldMut<'_>,
        value: <all_attrs::text_color as ElementAttrType<BevyRenderer>>::Value,
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
        entity_ref: &mut EntityWorldMut<'_>,
        value: <all_attrs::text_linebreak as ElementAttrType<BevyRenderer>>::Value,
    ) {
        let Some(mut t) = entity_ref.get_mut::<Text>() else {
            return;
        };
        t.linebreak_behavior = value;
    }

    fn set_text_align(
        &self,
        entity_ref: &mut EntityWorldMut<'_>,
        value: <all_attrs::text_align as ElementAttrType<BevyRenderer>>::Value,
    ) {
        let Some(mut t) = entity_ref.get_mut::<Text>() else {
            return;
        };
        t.alignment = value;
    }
}

pub mod attrs {
    use bevy_text::TextStyle;
    use std::borrow::Cow;

    use rxy_core::ElementAttrType;

    use super::*;

    /*pub struct sections;

    impl ElementAttrType for sections {
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

    impl ElementAttrType<BevyRenderer> for content
    {
        type Value = Cow<'static, str>;

        const NAME: &'static str = stringify!(content);

        fn update_value(
            world: &mut RendererWorld<BevyRenderer>,
            node_id: RendererNodeId<BevyRenderer>,
            value: impl Into<Self::Value>,
        ) {
            let value = value.into().to_string();
            let Some(mut t) = world.get_mut::<Text>(node_id) else {
                return;
            };
            if t.sections.is_empty() {
                t.sections = vec![bevy_text::TextSection::new(value, TextStyle::default())];
            } else if t.sections.len() == 1 {
                t.sections[0].value = value;
            } else {
                t.sections[0].value = value;
                unsafe {
                    t.sections.set_len(1);
                }
            }
        }
    }
}
