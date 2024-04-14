use std::sync::Arc;

use bevy_ecs::change_detection::Ref;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Mut, Query, QueryState, With, World};
use bevy_ecs::system::SystemParam;
use bevy_hierarchy::Children;
use kurbo::{Affine, Point, Stroke, Vec2};
use taffy::{AvailableSpace, print_tree};
use vello::glyph::Glyph;
use vello::peniko::{Blob, Brush, Color, Fill, Font};
use vello::skrifa::{FontRef, MetadataProvider};
use winit::dpi::PhysicalSize;

use rxy_core::EitherExt;

use crate::{GlobalTransform, LayoutContext, Style, Transform, UiLayoutTree, ViewVisibility};
use crate::ui_node::{BackgroundColor, BorderColor, BorderRadius, Node, Outline, VelloFragment};
use crate::user_event::EventLoopUserEvent;
use crate::window::NativeWorldExt;

pub struct XyRunningApp {
   pub root_entity: Entity,
   pub draw_query_state: QueryState<(
      Entity,
      // &'static mut VelloFragment,
      Option<&'static BackgroundColor>,
      Option<&'static BorderColor>,
      Option<&'static Outline>,
      Option<&'static BorderRadius>,
      &'static Node,
      &'static GlobalTransform,
      &'static ViewVisibility,
      // Option<&CalculatedClip>,
   )>,
   pub layout_query_state: QueryState<(
      Entity,
      &'static Node,
      &'static GlobalTransform,
      &'static ViewVisibility,
      // Option<&CalculatedClip>,
   )>,
   pub node_transform_query_state: QueryState<(
      // Entity,
      &'static mut Node,
      &'static mut Transform,
      &'static mut GlobalTransform,
      // Option<&CalculatedClip>,
   )>,
   pub children_query_state: QueryState<(Entity, Ref<'static, Children>), With<Node>>,
   pub style_query: QueryState<(Entity, Ref<'static,Style>), With<Node>>,
   pub font: Font,
   pub world: World,
}
const ROBOTO_FONT: &[u8] =
   include_bytes!("C:/Users/Ycy/Projects/vello/examples/assets/roboto/Roboto-Regular.ttf");

impl XyRunningApp {
   pub fn new(mut world: World, root_entity: Entity) -> Self {
      let font = Font::new(Blob::new(Arc::new(ROBOTO_FONT)), 0);

      Self {
         font,
         root_entity,
         draw_query_state: world.query_filtered(),
         layout_query_state: world.query_filtered(),
         node_transform_query_state: world.query_filtered(),
         children_query_state: world.query_filtered(),
         style_query: world.query_filtered(),
         world,
      }
   }

   pub(crate) fn handle_user_event(&mut self, user_event: EventLoopUserEvent) {
      match user_event {
         EventLoopUserEvent::CommandQueue(mut cmd_queue) => {
            cmd_queue.apply(&mut self.world);
         }
         EventLoopUserEvent::WindowSurfaceReady {
            xy_window,
            surface_renderer: renderer,
         } => {
            xy_window.window.request_redraw();
            self.world.insert_non_send_resource(xy_window);
            self.world.insert_non_send_resource(Box::new(renderer));
         }
      }
   }
   pub fn resized(&mut self, size: PhysicalSize<u32>) {
      self.world.window_scope(|world, xy_window| {
         world.window_surface_renderer_scope(|world, surface_renderer| {
            surface_renderer.resize_surface(&mut xy_window.surface, size.into());
            xy_window.window.request_redraw();
         })
      });
   }
   pub fn redraw_requested(&mut self) {
      let root_entity = self.root_entity;
      // let Some(children) = self
      //    .world
      //    .get::<Children>(root_entity)
      //    .map(|n| n.iter().copied().collect::<Vec<_>>())
      // else {
      //    return;
      // };
      self.world.window_scope(|world, xy_window| {
         world.window_surface_renderer_scope(|world, surface_renderer| {
            {
               world.resource_scope(|world, mut layout_tree: Mut<UiLayoutTree>| {
                  let physical_size = xy_window.window.inner_size();

                  let layout_context = LayoutContext::new(1.0,glam::Vec2 {
                     x: physical_size.width as _,
                     y: physical_size.height as _,
                  });

                  for (entity, style) in self.style_query.iter(world) {
                     layout_tree.upsert_node(entity, &style, &layout_context);
                  }

                  for (entity, children) in self.children_query_state.iter(world) {
                     layout_tree.update_children(entity, &children);
                     // if children.is_changed() {
                     // }
                  }

                  layout_tree.print_tree(root_entity);
                  println!("physical_size {:?}",physical_size);
                  layout_tree.compute_layout(
                     root_entity,
                     taffy::geometry::Size {
                        width: AvailableSpace::Definite(physical_size.width as _),
                        height: AvailableSpace::Definite(physical_size.height as _),
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
               // self
               //    .layout_query_state
               //    .par_iter_mut(world)
               //    .for_each(|(entity, node, transform, view_visibility)| {
               //
               //    });
            }

            world.window_scene_scope(|world, window_scene| {
               self.draw_query_state.iter_mut(world).for_each(
                  |(
                     entity,
                     // mut vello_node,
                     bg_color,
                     border_color,
                     outline,
                     border_radius,
                     node,
                     global_transform,
                     view_visibility,
                  )| {
                     println!("draw_query_state {:#?}",node);
                     let calculated_size = node.calculated_size;
                     let calculated_size = Vec2::new(calculated_size.x as _,calculated_size.y as _);
                     let shape = if let Some(border_radius) = border_radius {
                        let radii =
                           border_radius.resolve( calculated_size,Vec2::default(), 1.);
                        kurbo::RoundedRect::from_rect(
                           kurbo::Rect::from_points(
                              Point::ZERO,
                              calculated_size.to_point(),
                           ),
                           radii,
                        )
                        .either_left()
                     } else {
                        kurbo::Rect::from_points(
                           Point::ZERO,
                           calculated_size.to_point(),
                        )
                        .either_right()
                     };
                     if let Some(border_color) = border_color {
                        window_scene.scene.stroke(
                           &Stroke::new(2.),
                           global_transform.into(),
                           &Brush::Solid(border_color.0),
                           None,
                           &shape,
                        )
                     }
                     if let Some(bg_color) = bg_color {
                        println!("fill {:?} {:?}",global_transform,shape);
                        window_scene.scene.fill(
                           Fill::NonZero,
                           global_transform.into(),
                           &Brush::Solid(bg_color.0),
                           None,
                           &shape,
                        );
                     }
                  },
               );
               fn to_font_ref(font: &Font) -> Option<FontRef<'_>> {
                  use vello::skrifa::raw::FileRef;
                  let file_ref = FileRef::new(font.data.as_ref()).ok()?;
                  match file_ref {
                     FileRef::Font(font) => Some(font),
                     FileRef::Collection(collection) => collection.get(font.index).ok(),
                  }
               }
               let text = "Hello World Font";
               let font_ref = to_font_ref(&self.font).unwrap();
               let brush = Brush::Solid(Color::WHITE);
               let style = Fill::NonZero;
               let axes = font_ref.axes();
               let variations: &[(&str, f32)] = &[];
               let var_loc = axes.location(variations.iter().copied());
               let charmap = font_ref.charmap();
               let size = 40.0;
               let font_size = vello::skrifa::instance::Size::new(size);
               let metrics = font_ref.metrics(font_size, &var_loc);
               let line_height = metrics.ascent - metrics.descent + metrics.leading;
               let glyph_metrics = font_ref.glyph_metrics(font_size, &var_loc);
               let mut pen_x = 0f32;
               let mut pen_y = 0f32;
               window_scene
                  .scene
                  .draw_glyphs(&self.font)
                  .font_size(size)
                  .transform(Affine::translate((300.0, 300.0)))
                  // .glyph_transform(glyph_transform)
                  .normalized_coords(var_loc.coords())
                  .brush(&brush)
                  .hint(false)
                  .draw(
                     style,
                     text.chars().filter_map(|ch| {
                        if ch == '\n' {
                           pen_y += line_height;
                           pen_x = 0.0;
                           return None;
                        }
                        let gid = charmap.map(ch).unwrap_or_default();
                        let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
                        let x = pen_x;
                        pen_x += advance;
                        Some(Glyph {
                           id: gid.to_u16() as u32,
                           x,
                           y: pen_y,
                        })
                     }),
                  );

               window_scene.scene.fill(
                  Fill::EvenOdd,
                  Affine::default(),
                  &Brush::Solid(Color::rgb8(44, 55, 122)),
                  None,
                  &kurbo::Circle::new(Point::new(10., 100.), 40.0),
               );

               surface_renderer.render_scene(&mut xy_window.surface, &window_scene.scene, None);
               window_scene.scene.reset();
            })
         })
      });
   }
}
