use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_reflect::prelude::*;

use crate::elements::get_element_type;
use crate::{all_attrs, ElementAttr, ElementStyleEntityExt, SetAttrValueContext};

impl<'w> SetAttrValueContext<'w> {
    pub fn get_text_styled_element_type(&mut self) -> Option<&'static dyn TextStyledElementType> {
        let schema_name = self
            .entity_mut
            .get_element_extra_data_mut()
            .map(|n| n.element_name)?;
        let schema_type = get_element_type(schema_name);
        let type_registry = self.type_registry.read();
        type_registry
            .get_type_data::<ReflectTextStyledElementType>(schema_type.type_id())
            .and_then(|n| n.get(schema_type.as_reflect()))
    }
}

#[reflect_trait]
pub trait TextStyledElementType {
    fn set_font(&self, entity_ref: &mut EntityMut, value: <all_attrs::font as ElementAttr>::Value);
    fn set_font_size(
        &self,
        entity_ref: &mut EntityMut,
        value: <all_attrs::font_size as ElementAttr>::Value,
    );
    fn set_text_color(
        &self,
        entity_ref: &mut EntityMut,
        value: <all_attrs::text_color as ElementAttr>::Value,
    );
    fn set_text_linebreak(
        &self,
        entity_ref: &mut EntityMut,
        value: <all_attrs::text_linebreak as ElementAttr>::Value,
    );
    fn set_text_align(
        &self,
        entity_ref: &mut EntityMut,
        value: <all_attrs::text_align as ElementAttr>::Value,
    );
}

pub fn context_children_scope(
    context: &mut SetAttrValueContext,
    mut f: impl FnMut(Entity, &mut SetAttrValueContext),
) {
    let Some(children) = context.entity_mut.get_mut::<Children>() else {
        return;
    };
    let children: Vec<Entity> = children.into_iter().copied().collect();
    for entity in children {
        f(entity, context);
    }
}

pub fn set_text_value(
    context: &mut SetAttrValueContext,
    mut f: impl FnMut(&'static dyn TextStyledElementType, &mut EntityMut),
) {
    if let Some(text_element_type) = context.get_text_styled_element_type() {
        f(text_element_type, context.entity_mut);
    } /* else {
          context_children_scope(context, move |entity, context| {
              let Some(text_element_type) = context.get_text_styled_element_type_by_entity(entity)
              else {
                  return;
              };
              context.entity_mut_scope(entity, |entity_ref| {
                  f(text_element_type, entity_ref);
              });
          });
      }*/
}
