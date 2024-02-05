use crate::{all_attrs, BevyRenderer};
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use rxy_core::ElementAttr;

pub trait TextStyledElementEntityWorldMutExt {
    fn scoped_text_styled_element_type(
        &mut self,
        f: impl FnMut(&'static dyn TextStyledElementType, &mut EntityMut),
    );
}

impl TextStyledElementEntityWorldMutExt for EntityWorldMut<'_> {
    fn scoped_text_styled_element_type(
        &mut self,
        mut f: impl FnMut(&'static dyn TextStyledElementType, &mut EntityMut),
    ) {
        todo!()
        // let schema_name = self
        //     .get_element_extra_data_mut()
        //     .map(|n| n.element_name)?;
        // let schema_type = get_element_type(schema_name);
        // let type_registry = self.type_registry.read();
        // type_registry
        //     .get_type_data::<ReflectTextStyledElementType>(schema_type.type_id())
        //     .and_then(|n| n.get(schema_type.as_reflect()))
    }
}

#[reflect_trait]
pub trait TextStyledElementType {
    fn set_font(
        &self,
        entity_ref: &mut EntityMut,
        value: <all_attrs::font as ElementAttr<BevyRenderer>>::Value,
    );
    fn set_font_size(
        &self,
        entity_ref: &mut EntityMut,
        value: <all_attrs::font_size as ElementAttr<BevyRenderer>>::Value,
    );
    fn set_text_color(
        &self,
        entity_ref: &mut EntityMut,
        value: <all_attrs::text_color as ElementAttr<BevyRenderer>>::Value,
    );
    fn set_text_linebreak(
        &self,
        entity_ref: &mut EntityMut,
        value: <all_attrs::text_linebreak as ElementAttr<BevyRenderer>>::Value,
    );
    fn set_text_align(
        &self,
        entity_ref: &mut EntityMut,
        value: <all_attrs::text_align as ElementAttr<BevyRenderer>>::Value,
    );
}
//
// pub fn context_children_scope(
//     context: &mut SetAttrValueContext,
//     mut f: impl FnMut(Entity, &mut SetAttrValueContext),
// ) {
//     let Some(children) = context.entity_mut.get_mut::<Children>() else {
//         return;
//     };
//     let children: Vec<Entity> = children.into_iter().copied().collect();
//     for entity in children {
//         f(entity, context);
//     }
// }
