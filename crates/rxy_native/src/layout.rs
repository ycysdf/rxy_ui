use bevy_ecs::change_detection::{Mut, Ref};
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{QueryState, With};
use bevy_ecs::world::World;
use bevy_hierarchy::Children;
use taffy::AvailableSpace;
use vello::skrifa::MetadataProvider;

use crate::ui_node::Node;
use crate::{
   GlobalTransform, LayoutContext, PositionedGlyph, Style, Text, TextLayoutInfo, Transform,
   UiLayoutTree, ViewVisibility,
};

pub struct LayoutState {
   pub text_query_state: QueryState<(
      Entity,
      &'static Text,
      &'static mut TextLayoutInfo,
      &'static Node,
      &'static GlobalTransform,
   )>,
   pub layout_query_state: QueryState<(
      Entity,
      &'static Node,
      &'static GlobalTransform,
      &'static ViewVisibility,
   )>,
   pub node_transform_query_state: QueryState<(
      // Entity,
      &'static mut Node,
      &'static mut Transform,
      &'static mut GlobalTransform,
   )>,
   pub style_query: QueryState<(Entity, Ref<'static, Style>), With<Node>>,
   pub children_query_state: QueryState<(Entity, Ref<'static, Children>), With<Node>>,
}

impl LayoutState {
   pub fn handle(&mut self, world: &mut World, root_entity: Entity, layout_context: LayoutContext) {
      self.text_query_state.iter_mut(world).for_each(
         |(entity, text, mut layout, node_transform, global_transform)| {
            let font = text.style.font.as_ref().unwrap();
            let font_ref = crate::draw_text::to_font_ref(&font).unwrap();
            let axes = font_ref.axes();
            let variations: &[(&str, f32)] = &[];
            let var_loc = axes.location(variations.iter().copied());
            let charmap = font_ref.charmap();
            let font_size = vello::skrifa::instance::Size::new(text.style.font_size);
            let metrics = font_ref.metrics(font_size, &var_loc);
            let line_height = metrics.ascent - metrics.descent + metrics.leading;
            let line_height = line_height * text.style.line_height;
            let glyph_metrics = font_ref.glyph_metrics(font_size, &var_loc);
            let mut width = 0.;
            let mut pen_x = 0f32;
            let mut pen_y = 0f32;
            layout.glyphs = text
               .text
               .chars()
               .map(|ch| {
                  if ch == '\n' {
                     pen_y += line_height;
                     pen_x = 0.0;
                     return None;
                  }
                  let gid = charmap.map(ch).unwrap_or_default();
                  let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
                  let x = pen_x;
                  pen_x += advance;
                  if pen_x > width {
                     width = pen_x;
                  }
                  Some(PositionedGlyph {
                     glyph_id: gid.to_u16(),
                     position: glam::Vec2 {
                        x,
                        y: pen_y + line_height,
                     },
                     size: glam::Vec2 {
                        x: advance,
                        y: line_height,
                     },
                  })
               })
               .collect();

            layout.logical_size = glam::Vec2 {
               x: width,
               y: pen_y + line_height,
            };
         },
      );
      world.resource_scope(|world, mut layout_tree: Mut<UiLayoutTree>| {
           for (entity, style) in self.style_query.iter(world) {
               layout_tree.upsert_node(entity, &style, &layout_context);
           }

           for (entity, children) in self.children_query_state.iter(world) {
               layout_tree.update_children(entity, &children);
               // if children.is_changed() {
               // }
           }

           layout_tree.compute_layout(
               root_entity,
               world,
               taffy::geometry::Size {
                   width: AvailableSpace::Definite(layout_context.physical_size.x as _),
                   height: AvailableSpace::Definite(layout_context.physical_size.y as _),
               },
           ).unwrap();
           update_uinode_geometry_recursive(
               world,
               root_entity,
               &layout_tree,
               &mut self.node_transform_query_state,
               &mut self.children_query_state,
               1.,
               glam::Vec2::ZERO,
               glam::Vec2::ZERO,
           );
          layout_tree.print_tree(root_entity);

           #[inline]
           /// Round `value` to the nearest whole integer, with ties (values with a fractional part equal to 0.5) rounded towards positive infinity.
           fn round_ties_up(value: f32) -> f32 {
               if value.fract() != -0.5 {
                   // The `round` function rounds ties away from zero. For positive numbers "away from zero" is towards positive infinity.
                   // So for all positive values, and negative values with a fractional part not equal to 0.5, `round` returns the correct result.
                   value.round()
               } else {
                   // In the remaining cases, where `value` is negative and its fractional part is equal to 0.5, we use `ceil` to round it up towards positive infinity.
                   value.ceil()
               }
           }

           fn round_layout_coords(value: glam::Vec2) -> glam::Vec2 {
               glam::Vec2 {
                   x: round_ties_up(value.x),
                   y: round_ties_up(value.y),
               }
           }

           fn update_uinode_geometry_recursive(
               world: &mut World,
               entity: Entity,
               layout_tree: &UiLayoutTree,
               node_transform_query: &mut QueryState<(&mut Node, &mut Transform,&mut GlobalTransform)>,
               children_query: &mut QueryState<(Entity, Ref<'static, Children>), With<Node>>,
               inverse_target_scale_factor: f32,
               _parent_size: glam::Vec2,
               mut absolute_location: glam::Vec2,
           ) {
               if let Ok((mut node, mut transform,mut global_transform)) = node_transform_query.get_mut(world,entity) {
                   let Ok(layout) = layout_tree.get_layout(entity) else {
                       return;
                   };
                   let layout_size =
                       inverse_target_scale_factor * glam::Vec2::new(layout.size.width, layout.size.height);
                   let layout_location =
                       inverse_target_scale_factor * glam::Vec2::new(layout.location.x, layout.location.y);

                   absolute_location += layout_location;
                   global_transform.0.translation = absolute_location;

                   let rounded_size = round_layout_coords(absolute_location + layout_size)
                       - round_layout_coords(absolute_location);

                   let rounded_location =
                       round_layout_coords(layout_location)/* + 0.5 * (rounded_size - parent_size)*/;
                   //
                   // // only trigger change detection when the new values are different
                   if node.calculated_size != rounded_size || node.unrounded_size != layout_size {
                       node.calculated_size = rounded_size;
                       node.unrounded_size = layout_size;
                   }

                   if transform.0.translation.trunc() != rounded_location {
                       transform.0.translation = rounded_location;
                   }
                   if let Ok(children) = children_query.get(world,entity).map(|n|n.1.iter().copied().collect::<Vec<_>>()) {
                       for child_uinode in children {
                           update_uinode_geometry_recursive(
                               world,
                               child_uinode,
                               layout_tree,
                               node_transform_query,
                               children_query,
                               inverse_target_scale_factor,
                               rounded_size,
                               absolute_location,
                           );
                       }
                   }
               }
           }
       });
   }
}
