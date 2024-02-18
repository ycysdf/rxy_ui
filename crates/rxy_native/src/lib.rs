mod renderer;

pub mod prelude {
    pub use crate::tt::XyApp;
}
pub mod tt {
    use hecs::World;
    use kurbo::{Affine, Size};
    use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
    use rxy_core::prelude::{IntoView, Renderer, View, ViewCtx};
    use rxy_core::utils::HashMap;
    use std::any::Any;
    use std::iter::Scan;
    use std::mem::size_of;
    use vello::util::{RenderContext, RenderSurface};
    use vello::{peniko::Color, AaSupport, RenderParams, RendererOptions, Scene};
    use winit::event::{Event, WindowEvent};
    use winit::event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy};
    use winit::window::{Window, WindowBuilder, WindowId};

    use crate::renderer::NativeRenderer;
    use bevy_ecs::commands::CommandQueue;

    pub enum EventLoopUserEvent {
        CommandQueue(CommandQueue),
        SurfaceReady(WindowSurfaceRenderer),
    }

    #[derive(Clone)]
    pub struct UserEventSender {
        event_proxy: EventLoopProxy<EventLoopUserEvent>,
    }

    impl UserEventSender {
        #[inline]
        pub fn send(&self, f: impl FnOnce(&mut XyWindow) + Send + 'static) {
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

    pub struct WindowSurfaceRenderer {
        surface: RenderSurface,
        renderer: vello::Renderer,
        render_cx: RenderContext,
    }

    impl WindowSurfaceRenderer {
        pub async fn new(window: &Window) -> Self {
            let mut render_cx = RenderContext::new().unwrap();
            let size = window.inner_size();
            let surface = render_cx
                .create_surface(window, size.width, size.height)
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
                },
            )
            .unwrap();
            WindowSurfaceRenderer {
                surface,
                renderer,
                render_cx,
            }
        }
        pub fn render_scene(&mut self, scene: &Scene, size: Option<(u32, u32)>) {
            // let scale_factor =window.scale_factor();
            // let mut size = window.inner_size().to_logical(scale_factor);

            let surface = &mut self.surface;
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
            self.renderer
                .render_to_surface(device, queue, scene, &surface_texture, &render_params)
                .expect("failed to render to surface");
            surface_texture.present();
            device.poll(wgpu::Maintain::Wait);
        }
    }

    pub struct XyWindow {
        pub world: hecs::World,
        pub user_event_sender: UserEventSender,
        pub window: Window,
        pub root_entity: hecs::Entity,
        pub surface_renderer: Option<WindowSurfaceRenderer>,
        pub scene: Scene,
    }

    pub struct XyApp {
        pub main_window: XyWindow,
        pub event_loop: EventLoop<EventLoopUserEvent>,
    }

    impl Default for XyApp {
        fn default() -> Self {
            Self::new(WindowBuilder::default())
        }
    }

    impl XyApp {
        pub fn new(main_window: WindowBuilder) -> Self {
            let event_loop = EventLoopBuilder::<EventLoopUserEvent>::with_user_event().build();
            let main_window = main_window.build(&event_loop).unwrap();
            let mut world = World::new();
            let user_event_sender = UserEventSender::new(event_loop.create_proxy());
            XyApp {
                event_loop,
                main_window: XyWindow {
                    window: main_window,
                    root_entity: world.spawn(()),
                    world,
                    user_event_sender,
                    surface_renderer: Default::default(),
                    scene: Default::default(),
                },
            }
        }

        pub fn add_view<V>(&mut self, view: impl IntoView<NativeRenderer, View = V>) -> V::Key
        where
            V: View<NativeRenderer>,
        {
            let root = self.main_window.root_entity;
            let view = view.into_view();
            view.build(
                ViewCtx {
                    world: &mut self.main_window,
                    parent: root,
                },
                None,
                false,
            )
        }

        pub fn run(mut self) {
            let proxy = self.event_loop.create_proxy();
            let surface_renderer = tokio::runtime::Builder::new_current_thread()
                .build()
                .unwrap()
                .block_on(async { WindowSurfaceRenderer::new(&self.main_window.window).await });
            // NativeRenderer::spawn_task(async move {
            //     proxy.send_event(EventLoopUserEvent::SurfaceReady(surface_renderer.await))
            // });

            // proxy.send_event(event)
            let _ = self.event_loop.run(move |event, _, control_flow| {
                control_flow.set_wait();
                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        // println!("The close button was pressed; stopping");
                        control_flow.set_exit();
                    }
                    Event::UserEvent(user_event) =>{
                        match user_event {
                            EventLoopUserEvent::CommandQueue(mut cmd_queue) => {
                                cmd_queue.apply(&mut self.main_window);
                            },
                            EventLoopUserEvent::SurfaceReady(mut surface_renderer) => {
                                surface_renderer.render_scene(&self.main_window.scene, None);
                                self.main_window.surface_renderer = Some(surface_renderer);
                            },
                        }
                    }
                    Event::RedrawRequested {
                        ..
                    } => {
                    }
                    Event::WindowEvent {
                        event: WindowEvent::ScaleFactorChanged { /*scale_factor,*/ .. },
                        ..
                    } => {
                        // window.request_redraw();
                    }
                    _ => (),
                }
            });
        }
    }
}
