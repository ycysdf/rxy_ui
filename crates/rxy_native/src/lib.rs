pub use renderer::*;
mod renderer;
pub mod all_attrs {
    // pub use crate::attrs::*;
    // pub use crate::elements::input_attrs::*;
    pub use crate::elements::attrs::*;
}
pub mod prelude {
    pub use crate::tt::XyApp;

    pub use crate::renderer::common_renderer::*;
    pub use crate::renderer::*;
    // pub use crate::attrs::element_view_builder::*;
}
pub mod tt {
    use kurbo::{Affine, Point, Size, Vec2};
    use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
    use rxy_core::prelude::{IntoView, Renderer, View, ViewCtx};
    use rxy_core::utils::{HashMap, SyncCell};
    use std::any::Any;
    use std::iter::Scan;
    use std::mem::size_of;
    use std::ops::{Deref, DerefMut};
    use vello::util::{RenderContext, RenderSurface};
    use vello::{peniko::Color, AaSupport, RenderParams, RendererOptions, Scene};
    use winit::event::{Event, WindowEvent};
    use winit::event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy};
    use winit::window::{Window, WindowBuilder, WindowId};

    use crate::renderer::NativeRenderer;
    use crate::ui_node::{
        BackgroundColor, BorderColor, BorderRadius, Node, Outline, VelloFragment,
    };
    use bevy_ecs::commands::CommandQueue;
    use bevy_ecs::prelude::{Entity, Resource, World};
    use bevy_ecs::query::{AnyOf, Changed, QueryState};
    use vello::peniko::{Brush, Fill};

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

        pub fn resize(&mut self, (width, height): (u32, u32)) {
            self.render_cx
                .resize_surface(&mut self.surface, width, height);
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

    pub struct XyWindow(Window);

    impl Deref for XyWindow {
        type Target = Window;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for XyWindow {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    #[derive(Resource)]
    pub struct XyWindowScene {
        pub scene: Scene,
        pub root_entity: Entity,
    }

    pub struct XyApp {
        pub world: World,
        pub event_loop: EventLoop<EventLoopUserEvent>,
        pub root_entity: Entity,
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
            let root_entity = world.spawn(()).id();
            world.insert_non_send_resource(XyWindow(main_window));
            world.insert_resource(XyWindowScene {
                scene: Default::default(),
                root_entity,
            });
            world.insert_non_send_resource(UserEventSender::new(event_loop.create_proxy()));
            XyApp {
                event_loop,
                world,
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
            // let proxy = self.event_loop.create_proxy();
            let window = self.world.non_send_resource::<XyWindow>();
            let mut surface_renderer = tokio::runtime::Builder::new_current_thread()
                .build()
                .unwrap()
                .block_on(async { WindowSurfaceRenderer::new(window.deref()).await });
            // let renderer = WindowSurfaceRenderer::new(&main_window);
            // world.insert_non_send_resource(surface_renderer);
            // NativeRenderer::spawn_task(async move {
            //     proxy.send_event(EventLoopUserEvent::SurfaceReady(surface_renderer.await))
            // });

            let _ = self.event_loop.run(move |event, _, control_flow| {
                control_flow.set_wait();
                match event {
                    Event::WindowEvent { event, .. } => {
                        let mut window = &self.world.non_send_resource::<XyWindow>().0;
                        match event {
                            WindowEvent::Resized(size) => {
                                surface_renderer.resize(size.into());
                                window.request_redraw();
                            }
                            WindowEvent::ScaleFactorChanged { .. } => {}
                            WindowEvent::CloseRequested => {
                                control_flow.set_exit();
                            }
                            _ => {}
                        }
                    }
                    Event::UserEvent(user_event) => {
                        match user_event {
                            EventLoopUserEvent::CommandQueue(mut cmd_queue) => {
                                cmd_queue.apply(&mut self.world);
                            }
                            EventLoopUserEvent::SurfaceReady(mut surface_renderer) => {
                                // surface_renderer.render_scene(&self.main_window.scene, None);
                                // self.main_window.surface_renderer = Some(surface_renderer);
                            }
                        }
                    }
                    Event::RedrawRequested { .. } => {
                        println!("RedrawRequested");
                        let XyWindowScene {
                            scene: mut root_scene,
                            root_entity,
                        } = self.world.remove_resource::<XyWindowScene>().unwrap();
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
                            > = self.world.query_filtered();
                            println!("ui_nodes : {:?}", ui_nodes.iter(&self.world).len());
                            // par_
                            ui_nodes.iter_mut(&mut self.world).for_each(
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
                                            let radii = border_radius.resolve(
                                                node.calculated_size,
                                                Vec2::default(),
                                                1.,
                                            );
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

                                    root_scene.append(&vello_node.0, None);
                                },
                            );

                            // let mut vello_nodes: QueryState<&VelloFragment, ()> =
                            //     self.world.query_filtered();
                            //
                            // for node in vello_nodes.iter(&self.world) {
                            //     root_scene.append(&node.0, None);
                            // }
                        }

                        root_scene.fill(
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
                        surface_renderer.render_scene(&root_scene, None);
                        root_scene.reset();
                        self.world.insert_resource(XyWindowScene {
                            scene: root_scene,
                            root_entity,
                        });
                    }
                    _ => (),
                }
            });
        }
    }
}
