#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use bevy_ecs::prelude::*;

use rxy_core::{ElementAttrType, ElementType, ElementTypeUnTyped, RendererNodeId, RendererWorld};

use crate::renderer::node_tree::NodeTreeWorldExt;
use crate::renderer::NativeRenderer;
use crate::TextBundle;

#[derive(Debug, Default, Clone, Copy)]
// #[reflect(TextStyledElementType)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct element_span;

impl ElementType<NativeRenderer> for element_span {
   const TAG_NAME: &'static str = "span";

   fn get() -> &'static dyn ElementTypeUnTyped<NativeRenderer> {
      &element_span
   }

   fn spawn(
      world: &mut RendererWorld<NativeRenderer>,
      parent: Option<&RendererNodeId<NativeRenderer>>,
      reserve_node_id: Option<RendererNodeId<NativeRenderer>>,
   ) -> RendererNodeId<NativeRenderer> {
      let mut entity_world_mut = world.get_or_spawn_empty(parent, reserve_node_id);
      entity_world_mut.insert(TextBundle::default());
      entity_world_mut.id()
   }
}

/*impl TextStyledElementType for element_span {
    fn set_font(
        &self,
        entity_ref: &mut EntityWorldMut<'_>,
        value: <all_attrs::font as ElementAttrType<NativeRenderer>>::Value,
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
        value: <all_attrs::font_size as ElementAttrType<NativeRenderer>>::Value,
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
        value: <all_attrs::text_color as ElementAttrType<NativeRenderer>>::Value,
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
        value: <all_attrs::text_linebreak as ElementAttrType<NativeRenderer>>::Value,
    ) {
        let Some(mut t) = entity_ref.get_mut::<Text>() else {
            return;
        };
        t.linebreak_behavior = value;
    }

    fn set_text_align(
        &self,
        entity_ref: &mut EntityWorldMut<'_>,
        value: <all_attrs::text_align as ElementAttrType<NativeRenderer>>::Value,
    ) {
        let Some(mut t) = entity_ref.get_mut::<Text>() else {
            return;
        };
        t.alignment = value;
    }
}*/

pub mod attrs {
   use std::borrow::Cow;

   use rxy_core::{AttrIndex, ElementAttrType, HasIndex};

   use crate::renderer::NativeRenderer;

   use super::*;

   #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
   pub struct content;

   impl HasIndex for content {
      const INDEX: AttrIndex = 1;
   }

   impl ElementAttrType<NativeRenderer> for content {
      type Value = Cow<'static, str>;

      const NAME: &'static str = stringify!(content);

      fn update_value(
         world: &mut RendererWorld<NativeRenderer>,
         node_id: RendererNodeId<NativeRenderer>,
         value: impl Into<Self::Value>,
      ) {
      }
   }
}
