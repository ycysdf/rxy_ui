use std::hash::Hash;
use std::marker::PhantomData;

use bevy_ecs::prelude::{Component, IntoSystem};
use bevy_ecs::system::SystemId;
use bevy_input::keyboard::KeyCode;
use bevy_reflect::Reflect;
use bevy_utils::tracing::error;

use rxy_core::{MemberOwner, ViewMember, ViewMemberCtx, ViewMemberOrigin};

use crate::event::*;
use crate::BevyRenderer;

#[derive(Component, Reflect, Debug, Copy, Clone)]
pub struct EventSystem;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FocusInputEventMemberState<T>(pub SystemId, pub T);

pub struct EventViewMember<T, S, M> {
   pub element_event_ids: T,
   pub system: S,
   pub _marker: PhantomData<M>,
}

impl<T, S, M> ViewMemberOrigin<BevyRenderer> for EventViewMember<T, S, M>
where
   T: ElementEventIds,
   S: IntoSystem<(), (), M> + Send + 'static,
   M: Send + 'static,
{
   type Origin = Self;
}

impl<T, S, M> ViewMember<BevyRenderer> for EventViewMember<T, S, M>
where
   T: ElementEventIds,
   S: IntoSystem<(), (), M> + Send + 'static,
   M: Send + 'static,
{
   fn count() -> rxy_core::ViewMemberIndex {
      1
   }

   fn unbuild(mut ctx: ViewMemberCtx<BevyRenderer>, _view_removed: bool) {
      let state = ctx
         .take_indexed_view_member_state::<FocusInputEventMemberState<T>>()
         .unwrap();
      for event_id in state.1.iter_event_ids() {
         ctx.world.remove_event(ctx.node_id, event_id, state.0);
      }
      if let Err(err) = ctx.world.remove_system(state.0) {
         error!("remove_system error: {:?}", err);
      }
   }

   fn build(self, mut ctx: ViewMemberCtx<BevyRenderer>, _will_rebuild: bool) {
      let system_id = ctx.world.register_system(self.system);

      ctx.world.entity_mut(system_id.entity()).insert(EventSystem);

      for event_id in self.element_event_ids.clone().iter_event_ids() {
         ctx.world.add_event(ctx.node_id, event_id, system_id);
      }

      ctx.set_indexed_view_member_state(FocusInputEventMemberState(
         system_id,
         self.element_event_ids,
      ));
   }

   fn rebuild(self, ctx: ViewMemberCtx<BevyRenderer>) {
      Self::unbuild(
         ViewMemberCtx {
            index: ctx.index,
            world: &mut *ctx.world,
            node_id: ctx.node_id,
         },
         false,
      );
      self.build(ctx, true);
   }
}

impl<T> ElementKeyboardEvents for T where T: MemberOwner<BevyRenderer> + Sized {}

pub trait ElementKeyboardEvents: MemberOwner<BevyRenderer> + Sized {
   fn on<T, S, Marker>(
      self,
      element_event_ids: T,
      system: S,
   ) -> Self::AddMember<EventViewMember<T, S, Marker>>
   where
      T: ElementEventIds,
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.member(EventViewMember {
         element_event_ids,
         system,
         _marker: Default::default(),
      })
   }

   fn on_pressed<T, S, Marker>(
      self,
      events: impl FocusInputEventIterator,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      T: Copy + Eq + Hash + Send + Sync + 'static,
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on(x_pressed(events), system)
   }

   fn on_enter<S, Marker>(
      self,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on_just_pressed(KeyCode::Enter, system)
   }

   fn on_esc<S, Marker>(
      self,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on_just_pressed(KeyCode::Escape, system)
   }

   fn on_just_pressed<S, Marker>(
      self,
      events: impl FocusInputEventIterator,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on(x_just_pressed(events), system)
   }

   fn on_just_released<S, Marker>(
      self,
      events: impl FocusInputEventIterator,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on(x_just_released(events), system)
   }

   fn on_pointer_over<S, Marker>(
      self,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on(x_pointer_over(), system)
   }

   fn on_pointer_out<S, Marker>(
      self,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on(x_pointer_out(), system)
   }

   fn on_pointer_down<S, Marker>(
      self,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on(x_pointer_down(), system)
   }

   fn on_pointer_up<S, Marker>(
      self,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on(x_pointer_up(), system)
   }

   fn on_pointer_click<S, Marker>(
      self,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on(x_pointer_click(), system)
   }

   fn on_pointer_move<S, Marker>(
      self,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on(x_pointer_move(), system)
   }
   fn on_pointer_drag_start<S, Marker>(
      self,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on(x_pointer_drag_start(), system)
   }
   fn on_pointer_drag<S, Marker>(
      self,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on(x_pointer_drag(), system)
   }
   fn on_pointer_drag_end<S, Marker>(
      self,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on(x_pointer_drag_end(), system)
   }
   fn on_pointer_drag_enter<S, Marker>(
      self,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on(x_pointer_drag_enter(), system)
   }
   fn on_pointer_drag_over<S, Marker>(
      self,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on(x_pointer_drag_over(), system)
   }
   fn on_pointer_drag_leave<S, Marker>(
      self,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on(x_pointer_drag_leave(), system)
   }
   fn on_pointer_drop<S, Marker>(
      self,
      system: S,
   ) -> Self::AddMember<EventViewMember<impl ElementEventIds, S, Marker>>
   where
      S: IntoSystem<(), (), Marker> + Send + 'static,
      Marker: Send + 'static,
   {
      self.on(x_pointer_drop(), system)
   }
}
