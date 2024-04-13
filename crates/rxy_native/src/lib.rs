pub use renderer::*;

mod renderer;
pub mod all_attrs {
   // pub use crate::attrs::*;
   // pub use crate::elements::input_attrs::*;
   pub use crate::elements::attrs::*;
}
pub mod prelude {
   pub use crate::renderer::common_renderer::*;
   pub use crate::renderer::*;
   pub use crate::tt::XyApp;

   // pub use crate::attrs::element_view_builder::*;
}
pub mod tt {
   use std::future::Future;

   use bevy_ecs::prelude::{Entity, Resource, World};
   use bevy_ecs::query::QueryState;
   use bevy_ecs::world::CommandQueue;
   use kurbo::{Affine, Point, Vec2};
   use vello::peniko::{Brush, Fill};
   use vello::util::{RenderContext, RenderSurface};
   use vello::{peniko::Color, AaSupport, RenderParams, RendererOptions, Scene};
   use winit::dpi::PhysicalSize;
   use winit::event::{Event, WindowEvent};
   use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};
   use winit::window::{Window, WindowBuilder};

   use rxy_core::prelude::{IntoView, View, ViewCtx};
   use rxy_core::Renderer;

   use crate::renderer::NativeRenderer;
   use crate::ui_node::{BackgroundColor, BorderColor, BorderRadius, Node, Outline, VelloFragment};

   pub enum EventLoopUserEvent {
      CommandQueue(CommandQueue),
      WindowSurfaceReady {
         xy_window: XyWindow,
         surface_renderer: XyWindowSurfaceRenderer,
      },
   }

   #[derive(Clone)]
   pub struct UserEventSender {
      event_proxy: EventLoopProxy<EventLoopUserEvent>,
   }

   impl UserEventSender {
      #[inline]
      pub fn send(&self, f: impl FnOnce(&mut World) + Send + 'static) {
         let mut command_queue = CommandQueue::default();
         command_queue.push(f);
         self.send_queue(command_queue)
      }

      #[inline]
      pub fn send_queue(&self, command_queue: CommandQueue) {
         let _ = self
            .event_proxy
            .send_event(EventLoopUserEvent::CommandQueue(command_queue));
      }
   }

   impl UserEventSender {
      pub fn new(event_proxy: EventLoopProxy<EventLoopUserEvent>) -> Self {
         UserEventSender { event_proxy }
      }
   }

   pub struct XyWindow {
      window: Window,
      surface: RenderSurface<'static>,
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
         self.resource_scope(|world, mut scene| {
            f(world, &mut *scene)
         })
      }
   }

   pub struct XyApp {
      pub world: World,
      pub root_entity: Entity,
      pub window_builder: Option<WindowBuilder>,
   }

   impl Default for XyApp {
      fn default() -> Self {
         Self::new(WindowBuilder::default())
      }
   }

   impl XyApp {
      pub fn resized(world: &mut World, size: PhysicalSize<u32>) {
         world.window_scope(|world, xy_window| {
            world.window_surface_renderer_scope(|world, surface_renderer| {
               surface_renderer.resize_surface(&mut xy_window.surface, size.into());
               xy_window.window.request_redraw();
            })
         });
      }
      pub fn redraw_requested(world: &mut World) {
         fn inner(
            world: &mut World,
            xy_window: &mut XyWindow,
            surface_renderer: &mut XyWindowSurfaceRenderer,
            window_scene: &mut XyWindowScene,
         ) {
            {
               let mut ui_nodes: QueryState<
                  (
                     Entity,
                     &mut VelloFragment,
                     Option<&BackgroundColor>,
                     Option<&BorderColor>,
                     Option<&Outline>,
                     Option<&BorderRadius>,
                     &Node,
                     // &GlobalTransform
                     // &ViewVisibility,
                     // Option<&CalculatedClip>,
                  ),
                  (),
               > = world.query_filtered();
               println!("ui_nodes : {:?}", ui_nodes.iter(&world).len());
               // par_
               ui_nodes.iter_mut(world).for_each(
                  |(
                     entity,
                     mut vello_node,
                     bg_color,
                     border_color,
                     outline,
                     border_radius,
                     node,
                  )| {
                     if let Some(bg_color) = bg_color {
                        if let Some(border_radius) = border_radius {
                           let radii =
                              border_radius.resolve(node.calculated_size, Vec2::default(), 1.);
                           let rounded_rect = kurbo::RoundedRect::from_rect(
                              kurbo::Rect::from_points(
                                 Point::new(0., 0.),
                                 node.calculated_size.to_point(),
                              ),
                              radii,
                           );
                           vello_node.fill(
                              Fill::NonZero,
                              Affine::default(),
                              &Brush::Solid(bg_color.0),
                              None,
                              &rounded_rect,
                           );
                        } else {
                           vello_node.fill(
                              Fill::NonZero,
                              Affine::default(),
                              &Brush::Solid(bg_color.0),
                              None,
                              &kurbo::Rect::from_points(
                                 Point::new(0., 0.),
                                 node.calculated_size.to_point(),
                              ),
                           );
                        }
                     }

                     window_scene.scene.append(&vello_node.0, None);
                  },
               );

               // let mut vello_nodes: QueryState<&VelloFragment, ()> =
               //     world.query_filtered();
               //
               // for node in vello_nodes.iter(&world) {
               //     root_scene.append(&node.0, None);
               // }
            }

            window_scene.scene.fill(
               Fill::EvenOdd,
               Affine::default(),
               &Brush::Solid(Color::rgb8(44, 55, 122)),
               None,
               &kurbo::Circle::new(Point::new(10., 10.), 10.0),
            );
            // scene.fill(
            //     Fill::EvenOdd,
            //     Affine::default(),
            //     &Brush::Solid(Color::rgb8(255, 0, 255)),
            //     None,
            //     &kurbo::Circle::new(Point::new(100., 100.), 150.0),
            // );
            // scene.fill(
            //     Fill::EvenOdd,
            //     Affine::default(),
            //     &Brush::Solid(Color::rgb8(255, 0, 0)),
            //     None,
            //     &kurbo::Circle::new(Point::new(100., 100.), 30.0),
            // );
            surface_renderer.render_scene(&mut xy_window.surface, &window_scene.scene, None);
            window_scene.scene.reset();
         };
         world.window_scope(|world, window| {
            println!("WWW");
            world.window_surface_renderer_scope(|world, surface_renderer| {
               println!("WWW2");
               world.window_scene_scope(|world, scene| {
                  inner(world, window, surface_renderer, scene);
               })
            })
         });
      }
      pub fn new(window_builder: WindowBuilder) -> Self {
         let mut world = World::new();
         let root_entity = world.spawn(()).id();
         world.insert_resource(XyWindowScene {
            scene: Default::default(),
            root_entity,
         });
         XyApp {
            world,
            window_builder: Some(window_builder),
            root_entity,
         }
      }

      pub fn add_view<V>(&mut self, view: impl IntoView<NativeRenderer, View = V>) -> V::Key
      where
         V: View<NativeRenderer>,
      {
         let parent = self.root_entity;
         let view = view.into_view();
         view.build(
            ViewCtx {
               world: &mut self.world,
               parent,
            },
            None,
            false,
         )
      }

      pub fn run(mut self) {
         let XyApp {
            mut world,
            mut window_builder,
            root_entity: _,
         } = self;
         let event_loop = EventLoopBuilder::<EventLoopUserEvent>::with_user_event()
             .build()
             .unwrap();
         world.insert_non_send_resource(UserEventSender::new(event_loop.create_proxy()));

         NativeRenderer::spawn_task({
            let window = window_builder.take().unwrap().build(&event_loop).unwrap();
            let proxy = event_loop.create_proxy();
            async move {
               let (renderer, window) = XyWindowSurfaceRenderer::new(window).await;
               proxy.send_event(EventLoopUserEvent::WindowSurfaceReady { xy_window: window, surface_renderer: renderer })
            }
         });
         let _ = event_loop.run(move |event, target| {
            target.set_control_flow(ControlFlow::Wait);
            match event {
               Event::UserEvent(user_event) => match user_event {
                  EventLoopUserEvent::CommandQueue(mut cmd_queue) => {
                     cmd_queue.apply(&mut world);
                  }
                  EventLoopUserEvent::WindowSurfaceReady { xy_window, surface_renderer: renderer } => {
                     xy_window.window.request_redraw();
                     world.insert_non_send_resource(xy_window);
                     world.insert_non_send_resource(Box::new(renderer));
                  }
               },
               Event::WindowEvent { event, .. } => match event {
                  WindowEvent::RedrawRequested => {
                     Self::redraw_requested(&mut world);
                  }
                  WindowEvent::Resized(size) => {
                     Self::resized(&mut world, size);
                  }
                  WindowEvent::ScaleFactorChanged { .. } => {}
                  WindowEvent::CloseRequested => {
                     target.exit();
                  }
                  _ => {}
               },
               _ => (),
            }
         });
      }
   }
}
