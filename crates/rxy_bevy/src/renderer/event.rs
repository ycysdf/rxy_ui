use std::hash::Hash;

use bevy_ecs::prelude::IntoSystem;
use bevy_input::keyboard::KeyCode;

use rxy_core::{ElementView, MemberOwner};

use crate::event::*;
use crate::{BevyRenderer, EventViewMember};

macro_rules! define_event_view_builder {
   ($name:ident;$ty:ident) => {
      impl<T> $name for T where T: $ty<BevyRenderer> + Sized {}

      pub trait $name: $ty<BevyRenderer> + Sized {
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

         fn on_return<S, Marker>(
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
   };
}
define_event_view_builder!(MemberOwnerEventViewBuilder;MemberOwner);
define_event_view_builder!(ElementViewEventViewBuilder;ElementView);
