#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use kurbo::Vec2;
use vello::peniko::Color;

use rxy_core::{ElementType, ElementTypeUnTyped, RendererNodeId, RendererWorld};
use crate::{Display, Style, UiRect, Val};
use crate::NodeBundle;
use crate::renderer::NativeRenderer;
use crate::renderer::node_tree::NodeTreeWorldExt;
use crate::ui_node::{BackgroundColor, BorderColor, BorderRadius, Node, Outline};

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
            border: UiRect{
               left: Val::Px(30.),
               right: Val::Px(30.),
               top: Val::Px(30.),
               bottom: Val::Px(30.),
            },
            // padding: UiRect{
            //    left: Val::Px(10.),
            //    right: Val::Px(10.),
            //    top: Val::Px(10.),
            //    bottom: Val::Px(10.),
            // },
            // margin: UiRect{
            //    left: Val::Px(30.),
            //    right: Val::Px(30.),
            //    top: Val::Px(30.),
            //    bottom: Val::Px(30.),
            // },
            ..Style::default()
         },
         ..Default::default()
      };
      println!("bundle :{bundle:#?}");
      entity_world_mut.insert(bundle);
      entity_world_mut.insert(BackgroundColor(Color::AQUA));
      entity_world_mut.insert(BorderColor(Color::RED));
      entity_world_mut.insert(Outline{
         width: Val::Px(4.),
         offset: Val::Px(4.),
         color: Color::PINK,
      });
      entity_world_mut.insert(BorderRadius {
         top_left: Val::Px(40.),
         top_right: Val::Px(40.),
         bottom_right: Val::Px(40.),
         bottom_left: Val::Px(40.),
      });

      entity_world_mut.id()
   }
}
