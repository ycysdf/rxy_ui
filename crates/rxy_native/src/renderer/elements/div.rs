#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use kurbo::Vec2;
use vello::peniko::Color;

use rxy_core::{ElementType, ElementTypeUnTyped, RendererNodeId, RendererWorld};
use crate::{Display, Style, UiRect, Val};
use crate::NodeBundle;
use crate::renderer::NativeRenderer;
use crate::ui_node::{BackgroundColor, BorderColor, BorderRadius, Node, Outline};
use crate::world_ext::BevyWorldExt;

#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
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
      entity_world_mut.insert(NodeBundle{
         ..Default::default()
      });
      entity_world_mut.id()
   }
}
