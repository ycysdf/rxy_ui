use std::time::Instant;

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::World;
use bevy_ecs::system::SystemParam;
use tracing::{info_span, instrument};
use winit::dpi::PhysicalSize;

use crate::draw::DrawState;
use crate::draw_text::SceneExt;
use crate::layout::LayoutState;
use crate::LayoutContext;
use crate::user_event::EventLoopUserEvent;
use crate::window::NativeWorldExt;

pub struct XyRunningApp {
   pub root_entity: Entity,
   pub draw_state: DrawState,
   pub layout_state: LayoutState,
   pub world: World,
}
impl XyRunningApp {
   pub fn new(mut world: World, root_entity: Entity) -> Self {
      Self {
         root_entity,
         draw_state: DrawState {
            shape_query_state: world.query_filtered(),
            bg_query_state: world.query_filtered(),
            border_query_state: world.query_filtered(),
            outline_query_state: world.query_filtered(),
            text_query_state: world.query_filtered(),
            shape_map: Default::default(),
         },
         layout_state: LayoutState {
            text_query_state: world.query_filtered(),
            layout_query_state: world.query_filtered(),
            node_transform_query_state: world.query_filtered(),
            children_query_state: world.query_filtered(),
            style_query: world.query_filtered(),
         },
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

   #[instrument(name = "redraw",skip(self))]
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
         // xy_window.window.request_redraw();
         world.window_surface_renderer_scope(|world, surface_renderer| {
            let physical_size = xy_window.window.inner_size();
            info_span!("compute_layout").in_scope(|| {
               let layout_context = LayoutContext::new(
                  1.0,
                  glam::Vec2 {
                     x: physical_size.width as _,
                     y: physical_size.height as _,
                  },
               );
               self.layout_state.handle(world, root_entity, layout_context);
            });

            world.window_scene_scope(|world, window_scene| {
               info_span!("draw_scene").in_scope(|| {
                  self.draw_state.draw_scene(world, &mut window_scene.scene);
               });
               info_span!("render_scene").in_scope(|| {
                  surface_renderer.render_scene(&mut xy_window.surface, &window_scene.scene, None);
               });

               window_scene.scene.reset();
            });
         })
      });
   }
}
