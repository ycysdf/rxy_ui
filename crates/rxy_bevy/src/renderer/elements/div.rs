#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use bevy_reflect::Reflect;
use bevy_ui::prelude::NodeBundle;

use rxy_core::{ElementAttrUntyped, ElementType, RendererNodeId, RendererWorld};

use crate::{BevyRenderer, BevyWorldExt};

use super::*;

#[derive(Reflect, Debug, Clone, Copy)]
pub struct element_div;

impl ElementType<BevyRenderer> for element_div {
    const TAG_NAME: &'static str = "div";
    const ATTRS: &'static [&'static [&'static dyn ElementAttrUntyped<BevyRenderer>]] = &[];

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
