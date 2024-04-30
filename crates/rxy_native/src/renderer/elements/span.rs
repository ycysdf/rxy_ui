#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::sync::Arc;

use bevy_ecs::prelude::*;
use vello::peniko::{Blob, Brush, Color, Font};

use rxy_core::{ElementAttrType, ElementType, ElementTypeUnTyped, RendererNodeId, RendererWorld};

use crate::{Text, TextBundle};
use crate::draw_text::TextStyle;
use crate::renderer::NativeRenderer;
use crate::world_ext::BevyWorldExt;


const ROBOTO_FONT: &[u8] =
   include_bytes!("C:/Users/Ycy/Projects/vello/examples/assets/roboto/Roboto-Regular.ttf");

#[derive(Debug, Default, Clone, Copy)]
// #[reflect(TextStyledElementType)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
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
      let font = Font::new(Blob::new(Arc::new(ROBOTO_FONT)), 0);

      let mut entity_world_mut = world.get_or_spawn_empty(parent, reserve_node_id);
      entity_world_mut.insert(TextBundle {
         text: Text {
            text: Default::default(),
            style: TextStyle {
               font_size: 28.,
               color: Color::WHITE.into(),
               font: Some(font),
               ..Default::default()
            },
         },
         ..TextBundle::default()
      });
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

pub mod element_span_attrs {
   use std::borrow::Cow;

   use rxy_core::{AttrIndex, ElementAttrType, HasIndex};

   use crate::renderer::NativeRenderer;
   use crate::Text;

   use super::*;

   #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
   pub struct content;

   impl ElementAttrType<NativeRenderer> for content {
      type Value = Cow<'static, str>;

      const NAME: &'static str = stringify!(content);

      fn update_value(
         world: &mut RendererWorld<NativeRenderer>,
         node_id: RendererNodeId<NativeRenderer>,
         value: impl Into<Self::Value>,
      ) {
         if let Some(mut text) = world.get_mut::<Text>(node_id) {
            text.text = value.into();
         }
      }
   }
}
