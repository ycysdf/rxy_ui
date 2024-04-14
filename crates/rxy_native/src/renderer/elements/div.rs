#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use kurbo::Vec2;
use vello::peniko::Color;

use rxy_core::{ElementType, ElementTypeUnTyped, RendererNodeId, RendererWorld};

use crate::{Display, Style, Val};
use crate::NodeBundle;
use crate::renderer::NativeRenderer;
use crate::renderer::node_tree::NodeTreeWorldExt;
use crate::ui_node::{BackgroundColor, BorderRadius, Node};

#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
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
      let bundle = NodeBundle {
         style: Style {
            width: Val::Px(200.),
            height: Val::Px(200.),
            display: Display::Flex,
            ..Style::default()
         },
         ..Default::default()
      };
      println!("bundle :{bundle:#?}");
      entity_world_mut.insert(bundle);
      entity_world_mut.insert(BackgroundColor(Color::AQUA));
      entity_world_mut.insert(BorderRadius {
         top_left: Val::Px(10.),
         top_right: Val::Px(10.),
         bottom_right: Val::Px(10.),
         bottom_left: Val::Px(10.),
      });

      entity_world_mut.id()
   }
}
