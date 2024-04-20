use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Mut, World};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopBuilder};
use winit::window::WindowBuilder;

use rxy_core::{IntoView, Renderer, View, ViewCtx};

use crate::{LayoutContext, NativeRenderer, NodeBundle, Style, UiLayoutTree, Val};
use crate::running_app::XyRunningApp;
use crate::user_event::{EventLoopUserEvent, UserEventSender};
use crate::window::{XyWindowScene, XyWindowSurfaceRenderer};

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
   pub fn new(window_builder: WindowBuilder) -> Self {
      let mut world = World::new();
      let root_entity = world
         .spawn(NodeBundle {
            style: Style {
               width: Val::Percent(100.0),
               height: Val::Percent(100.0),
               row_gap: Val::Px(12.),
               column_gap: Val::Px(12.),
               ..Style::default()
            },
            ..NodeBundle::default()
         })
         .id();
      world.insert_resource(XyWindowScene {
         scene: Default::default(),
         root_entity,
      });

      world.insert_resource(UiLayoutTree::new());
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
         root_entity,
      } = self;
      let event_loop = EventLoopBuilder::<EventLoopUserEvent>::with_user_event()
         .build()
         .unwrap();
      world.insert_non_send_resource(UserEventSender::new(event_loop.create_proxy()));

      let window = window_builder.take().unwrap().build(&event_loop).unwrap();
      let physical_size = window.inner_size();
      NativeRenderer::spawn_task({
         let proxy = event_loop.create_proxy();
         async move {
            let (renderer, window) = XyWindowSurfaceRenderer::new(window).await;
            proxy.send_event(EventLoopUserEvent::WindowSurfaceReady {
               xy_window: window,
               surface_renderer: renderer,
            })
         }
      });
      {
         world.resource_scope(|world, mut layout_tree: Mut<UiLayoutTree>| {
            let layout_context = LayoutContext::new(
               1.0,
               glam::Vec2 {
                  x: physical_size.width as _,
                  y: physical_size.height as _,
               },
            );
            let style = world.get::<Style>(root_entity).unwrap();
            layout_tree.upsert_node(self.root_entity, style, &layout_context)
         });
      }
      let mut running_app = XyRunningApp::new(world, root_entity);
      let _ = event_loop.run(move |event, target| {
         target.set_control_flow(ControlFlow::Wait);
         match event {
            Event::UserEvent(user_event) => {
               running_app.handle_user_event(user_event);
            }
            Event::WindowEvent { event, .. } => match event {
               WindowEvent::RedrawRequested => {
                  running_app.redraw_requested();
               }
               WindowEvent::Resized(size) => {
                  running_app.resized(size);
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
