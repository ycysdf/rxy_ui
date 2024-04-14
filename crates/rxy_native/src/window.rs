use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Resource, World};
use std::future::Future;
use vello::peniko::Color;
use vello::util::{RenderContext, RenderSurface};
use vello::{AaSupport, RenderParams, RendererOptions, Scene};
use winit::window::Window;

pub struct XyWindow {
   pub window: Window,
   pub surface: RenderSurface<'static>,
}

pub struct XyWindowSurfaceRenderer {
   renderer: vello::Renderer,
   render_cx: RenderContext,
}

impl XyWindowSurfaceRenderer {
   pub fn new(window: Window) -> impl Future<Output = (Self, XyWindow)> + 'static {
      let mut render_cx = RenderContext::new().unwrap();
      let size = window.inner_size();
      async move {
         let window = window;
         let surface = render_cx
            .create_surface(
               unsafe { &*(&window as *const _) },
               size.width,
               size.height,
               Default::default(),
            )
            .await
            .unwrap();
         let dev_id = surface.dev_id;
         let device = &render_cx.devices[dev_id].device;
         let renderer = vello::Renderer::new(
            device,
            RendererOptions {
               surface_format: Some(surface.format),
               use_cpu: false,
               antialiasing_support: AaSupport {
                  area: true,
                  msaa8: false,
                  msaa16: false,
               },
               num_init_threads: None,
            },
         )
         .unwrap();
         (
            XyWindowSurfaceRenderer {
               renderer,
               render_cx,
            },
            XyWindow { window, surface },
         )
      }
   }

   pub fn resize_surface(&mut self, surface: &mut RenderSurface, (width, height): (u32, u32)) {
      self.render_cx.resize_surface(surface, width, height);
   }

   pub fn render_scene(
      &mut self,
      surface: &mut RenderSurface,
      scene: &Scene,
      size: Option<(u32, u32)>,
   ) {
      // let scale_factor =window.scale_factor();
      // let mut size = window.inner_size().to_logical(scale_factor);

      let (width, height) = if let Some((width, height)) = size {
         if surface.config.width != width || surface.config.height != height {
            self.render_cx.resize_surface(surface, width, height);
         }
         (width, height)
      } else {
         (surface.config.width, surface.config.height)
      };

      let surface_texture = surface
         .surface
         .get_current_texture()
         .expect("failed to acquire next swapchain texture");
      let dev_id = surface.dev_id;
      let device = &self.render_cx.devices[dev_id].device;
      let queue = &self.render_cx.devices[dev_id].queue;
      let render_params = RenderParams {
         base_color: Color::BLACK,
         width,
         height,
         antialiasing_method: vello::AaConfig::Area,
      };
      self
         .renderer
         .render_to_surface(device, queue, scene, &surface_texture, &render_params)
         .expect("failed to render to surface");
      surface_texture.present();
      device.poll(wgpu::Maintain::Wait);
   }
}

#[derive(Resource)]
pub struct XyWindowScene {
   pub scene: Scene,
   pub root_entity: Entity,
}

pub trait NativeWorldExt {
   fn window_scope(&mut self, f: impl FnOnce(&mut Self, &mut XyWindow));
   fn window_surface_renderer_scope(
      &mut self,
      f: impl FnOnce(&mut Self, &mut XyWindowSurfaceRenderer),
   );
   fn window_scene_scope(&mut self, f: impl FnOnce(&mut Self, &mut XyWindowScene));
}

impl NativeWorldExt for World {
   fn window_scope(&mut self, f: impl FnOnce(&mut Self, &mut XyWindow)) {
      let Some(mut window) = self.remove_non_send_resource::<XyWindow>() else {
         return;
      };
      f(self, &mut window);
      self.insert_non_send_resource(window);
   }

   fn window_surface_renderer_scope(
      &mut self,
      f: impl FnOnce(&mut Self, &mut XyWindowSurfaceRenderer),
   ) {
      let Some(mut surface) = self.remove_non_send_resource::<Box<XyWindowSurfaceRenderer>>()
      else {
         return;
      };
      f(self, &mut *surface);
      self.insert_non_send_resource(surface);
   }
   fn window_scene_scope(&mut self, f: impl FnOnce(&mut Self, &mut XyWindowScene)) {
      self.resource_scope(|world, mut scene| f(world, &mut *scene))
   }
}
