#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use bevy_reflect::Reflect;
use bevy_ui::prelude::NodeBundle;

use rxy_core::{ElementAttrUntyped, ElementType, ElementTypeUnTyped, RendererNodeId, RendererWorld};

use crate::all_attrs::CommonAttrs;
use crate::{BevyRenderer, BevyWorldExt};

impl CommonAttrs for element_div {}
pub const VIEW_ATTRS: &[&'static dyn ElementAttrUntyped<BevyRenderer>] =
    <element_div as CommonAttrs>::ATTRS;

#[derive(Reflect, Default, Debug, Clone, Copy)]
pub struct element_div;

impl ElementType<BevyRenderer> for element_div {
    const TAG_NAME: &'static str = "div";
    const ATTRS: &'static [&'static [&'static dyn ElementAttrUntyped<BevyRenderer>]] =
        &[VIEW_ATTRS];

    fn get() -> &'static dyn ElementTypeUnTyped<BevyRenderer> {
        &element_div
    }

    fn spawn(
        world: &mut RendererWorld<BevyRenderer>,
        parent: Option<&RendererNodeId<BevyRenderer>>,
        reserve_node_id: Option<RendererNodeId<BevyRenderer>>,
    ) -> RendererNodeId<BevyRenderer> {
        let mut entity_world_mut = world.get_or_spawn_empty(parent, reserve_node_id);
        entity_world_mut.insert(NodeBundle::default());
        entity_world_mut.id()
    }
}
