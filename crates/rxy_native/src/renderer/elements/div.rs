#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use kurbo::Vec2;
use vello::peniko::Color;
use crate::renderer::NativeRenderer;
use rxy_core::{ElementType, ElementTypeUnTyped, RendererNodeId, RendererWorld};
use crate::geometry::Val;
use crate::node_bundles::NodeBundle;
use crate::renderer::node_tree::NodeTreeWorldExt;
use crate::ui_node::{BackgroundColor, BorderRadius, Node};

#[derive(Default, Debug, Clone, Copy)]
pub struct element_div;

impl ElementType<NativeRenderer> for element_div {
    const TAG_NAME: &'static str = "div";

    fn get() -> &'static dyn ElementTypeUnTyped<NativeRenderer> {
        &element_div
    }

    fn spawn(
        world: &mut RendererWorld<NativeRenderer>,
        parent: Option<&RendererNodeId<NativeRenderer>>,
        reserve_node_id: Option<RendererNodeId<NativeRenderer>>,
    ) -> RendererNodeId<NativeRenderer> {
        let mut entity_world_mut = world.get_or_spawn_empty(parent, reserve_node_id);
        entity_world_mut.insert(NodeBundle {
            node: Node {
                calculated_size: Vec2::new(80., 80.),
                stack_index: 0,
            },
            ..Default::default()
        });
        // entity_world_mut.insert(BackgroundColor(Color::AQUA));
        // entity_world_mut.insert(BorderRadius {
        //     top_left: Val::Px(10.),
        //     top_right: Val::Px(10.),
        //     bottom_right: Val::Px(10.),
        //     bottom_left: Val::Px(10.),
        // });

        entity_world_mut.id()
    }
}
