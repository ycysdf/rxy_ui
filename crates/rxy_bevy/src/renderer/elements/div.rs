#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use bevy_reflect::Reflect;
use bevy_ui::prelude::NodeBundle;

use rxy_core::{ElementType, ElementTypeUnTyped, RendererNodeId, RendererWorld};

use crate::{BevyRenderer, BevyWorldExt};


#[derive(Reflect, Default, Debug, Clone, Copy)]
pub struct element_div;

impl ElementType<BevyRenderer> for element_div {
    const TAG_NAME: &'static str = "div";

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
