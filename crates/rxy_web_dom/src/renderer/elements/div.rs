#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use rxy_core::{ElementAttrUntyped, ElementType, ElementTypeUnTyped, RendererNodeId, RendererWorld};

use crate::all_attrs::CommonAttrs;
use crate::renderer::WebRenderer;

impl CommonAttrs for element_div {}
pub const VIEW_ATTRS: &[&'static dyn ElementAttrUntyped<WebRenderer>] =
    <element_div as CommonAttrs>::ATTRS;

#[derive(Default, Debug, Clone, Copy)]
pub struct element_div;

impl ElementType<WebRenderer> for element_div {
    const TAG_NAME: &'static str = "div";
    const ATTRS: &'static [&'static [&'static dyn ElementAttrUntyped<WebRenderer>]] =
        &[VIEW_ATTRS];

    fn get() -> &'static dyn ElementTypeUnTyped<WebRenderer> {
        &element_div
    }

    fn spawn(
        world: &mut RendererWorld<WebRenderer>,
        parent: Option<&RendererNodeId<WebRenderer>>,
        reserve_node_id: Option<RendererNodeId<WebRenderer>>,
    ) -> RendererNodeId<WebRenderer> {
        let mut entity_world_mut = world.get_or_spawn_empty(parent, reserve_node_id);
        entity_world_mut.insert(NodeBundle::default());
        entity_world_mut.id()
    }
}
