use bevy_ecs::prelude::World;
use bevy_ecs::world::CommandQueue;
use winit::event_loop::EventLoopProxy;

use rxy_core::{DeferredNodeTreeScoped, RendererWorld};

use crate::window::{XyWindow, XyWindowSurfaceRenderer};
use crate::NativeRenderer;

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

impl DeferredNodeTreeScoped<NativeRenderer> for UserEventSender {
   fn scoped(&self, f: impl FnOnce(&mut RendererWorld<NativeRenderer>) + Send + 'static) {
      self.send(f)
   }
}
