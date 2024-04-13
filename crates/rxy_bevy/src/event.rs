use core::{
   fmt::Debug,
   hash::Hash,
   marker::PhantomData,
   ops::{Deref, DerefMut},
};
use std::iter::once;
use std::sync::Arc;

use bevy_app::PreUpdate;
use bevy_ecs::entity::EntityHashMap;
use bevy_ecs::{
   prelude::{Commands, Res, Resource, World},
   system::SystemId,
};
use bevy_input::{
   gamepad::GamepadButton, keyboard::KeyCode, mouse::MouseButton, ButtonInput, InputSystem,
};
use bevy_mod_picking::prelude::*;
use bevy_reflect::Reflect;
use bevy_utils::tracing::error;
use bevy_utils::{all_tuples, HashMap};

use rxy_core::{NodeTree, RendererNodeId, RendererWorld};

use crate::world_ext::EntityWorldMutExt;
use crate::{add_system, BevyRenderer, FocusedEntity};

fn add_focus_event<T>(
   world: &mut RendererWorld<BevyRenderer>,
   node_id: RendererNodeId<BevyRenderer>,
   input: T,
   input_way: FocusInputTriggerWay,
   system_id: SystemId,
) where
   T: Copy + Eq + Hash + Send + Sync + 'static,
   FocusInputEvents<T>: Resource,
{
   use bevy_ecs::schedule::IntoSystemConfigs;

   let is_add_system = world.contains_resource::<FocusInputEvents<T>>();

   let mut focus_input_events = world.get_resource_or_insert_with(FocusInputEvents::<T>::default);

   let events = focus_input_events.entry(node_id).or_default();
   events
      .entry((input, input_way))
      .or_default()
      .push(system_id);

   if !is_add_system {
      add_system(
         world,
         PreUpdate,
         FocusInputEvents::<T>::system_handle
            .after(InputSystem)
            .run_if(|events: Res<FocusInputEvents<T>>| !events.is_empty()),
      );
   }
}

pub trait EventIsMatch {
   type Data: Clone + Send + Sync + 'static;
   fn is_match(&self, other: &Self::Data) -> bool;
}

macro_rules! impl_event_is_match_empty_data {
    ($($ty:ty)*) => {
        $(
        impl EventIsMatch for $ty {
            type Data = ();

            fn is_match(&self, _other: &Self::Data) -> bool {
                true
            }
        }
        )*
    };
}
macro_rules! impl_event_is_match_pointer_button {
    ($($ty:ty)*) => {
        $(
        impl EventIsMatch for $ty {
            type Data = PointerButton;

            fn is_match(&self, other: &Self::Data) -> bool {
                &self.button == other
            }
        }
        )*
    };
}

impl_event_is_match_empty_data! {
    Pointer<Over> Pointer<Out> Pointer<Move>
}

impl_event_is_match_pointer_button! {
    Pointer<Down>
    Pointer<Up>
    Pointer<Click>
    Pointer<DragStart>
    Pointer<Drag>
    Pointer<DragEnd>
    Pointer<DragEnter>
    Pointer<DragOver>
    Pointer<DragLeave>
    Pointer<Drop>
}

fn remove_bubble_event<T>(
   world: &mut RendererWorld<BevyRenderer>,
   node_id: RendererNodeId<BevyRenderer>,
   system_id: SystemId,
) where
   T: EntityEvent + EventIsMatch,
{
   let system_ids = world
      .get_node_state_mut::<BubbleEventSystemIds<T>>(&node_id)
      .unwrap();
   system_ids.retain(|n| n.0 != system_id);
}

fn add_bubble_event<T>(
   world: &mut RendererWorld<BevyRenderer>,
   node_id: RendererNodeId<BevyRenderer>,
   system_id: SystemId,
   stop_propagation: bool,
   data: Option<T::Data>,
) where
   T: EntityEvent + EventIsMatch,
{
   let mut entity_world_mut = world.entity_mut(node_id);
   if entity_world_mut.contains::<On<T>>() {
      let system_ids = entity_world_mut.get_or_default::<BubbleEventSystemIds<T>>();
      system_ids.push((system_id, data));
   } else {
      entity_world_mut.world_scope(|world| {
         world.set_node_state(
            &node_id,
            BubbleEventSystemIds::<T>::new(smallvec::SmallVec::from_elem((system_id, data), 1)),
         );
      });
      entity_world_mut.insert(On::<T>::run(move |world: &mut World| {
         let mut listerner = world.resource_mut::<ListenerInput<T>>();
         if stop_propagation {
            listerner.stop_propagation();
         }

         let event_data: T = ListenerInput::deref(&*listerner).clone();
         world.node_state_scoped(
            &node_id,
            |world, system_ids: &mut BubbleEventSystemIds<T>| {
               for (system_id, data) in system_ids.iter() {
                  if let Some(data) = data {
                     if !event_data.is_match(data) {
                        return;
                     }
                  }
                  let err = world.run_system(*system_id);
                  if let Err(err) = err {
                     error!("run system error: {:?}", err);
                  }
               }
            },
         );
      }));
   }
}

pub trait FocusEventWorldExt {
   fn add_focus_event(
      &mut self,
      node_id: RendererNodeId<BevyRenderer>,
      input: FocusInputEvent,
      input_way: FocusInputTriggerWay,
      system_id: SystemId,
   );
   fn add_bubble_event(
      &mut self,
      node_id: RendererNodeId<BevyRenderer>,
      event: BubblePointerEvent,
      stop_propagation: bool,
      system_id: SystemId,
   );
   fn add_event(
      &mut self,
      node_id: RendererNodeId<BevyRenderer>,
      event: ElementEventId,
      system_id: SystemId,
   );
   fn remove_event(
      &mut self,
      node_id: RendererNodeId<BevyRenderer>,
      event: ElementEventId,
      system_id: SystemId,
   );
}

impl FocusEventWorldExt for World {
   fn add_focus_event(
      &mut self,
      node_id: RendererNodeId<BevyRenderer>,
      focus_event: FocusInputEvent,
      trigger_way: FocusInputTriggerWay,
      system_id: SystemId,
   ) {
      match focus_event {
         FocusInputEvent::Keyboard(input) => {
            add_focus_event(self, node_id, input, trigger_way, system_id)
         }
         FocusInputEvent::Mouse(input) => {
            add_focus_event(self, node_id, input, trigger_way, system_id)
         }
         FocusInputEvent::Gamepad(input) => {
            add_focus_event(self, node_id, input, trigger_way, system_id)
         }
      }
   }
   fn add_bubble_event(
      &mut self,
      node_id: RendererNodeId<BevyRenderer>,
      event: BubblePointerEvent,
      stop_propagation: bool,
      system_id: SystemId,
   ) {
      match event {
         BubblePointerEvent::Over => {
            add_bubble_event::<Pointer<Over>>(self, node_id, system_id, stop_propagation, None)
         }
         BubblePointerEvent::Out => {
            add_bubble_event::<Pointer<Out>>(self, node_id, system_id, stop_propagation, None)
         }
         BubblePointerEvent::Down(data) => {
            add_bubble_event::<Pointer<Down>>(self, node_id, system_id, stop_propagation, data)
         }
         BubblePointerEvent::Up(data) => {
            add_bubble_event::<Pointer<Up>>(self, node_id, system_id, stop_propagation, data)
         }
         BubblePointerEvent::Click(data) => {
            add_bubble_event::<Pointer<Click>>(self, node_id, system_id, stop_propagation, data)
         }
         BubblePointerEvent::Move => {
            add_bubble_event::<Pointer<Move>>(self, node_id, system_id, stop_propagation, None)
         }
         BubblePointerEvent::DragStart(data) => {
            add_bubble_event::<Pointer<DragStart>>(self, node_id, system_id, stop_propagation, data)
         }
         BubblePointerEvent::Drag(data) => {
            add_bubble_event::<Pointer<Drag>>(self, node_id, system_id, stop_propagation, data)
         }
         BubblePointerEvent::DragEnd(data) => {
            add_bubble_event::<Pointer<DragEnd>>(self, node_id, system_id, stop_propagation, data)
         }
         BubblePointerEvent::DragEnter(data) => {
            add_bubble_event::<Pointer<DragEnter>>(self, node_id, system_id, stop_propagation, data)
         }
         BubblePointerEvent::DragOver(data) => {
            add_bubble_event::<Pointer<DragOver>>(self, node_id, system_id, stop_propagation, data)
         }
         BubblePointerEvent::DragLeave(data) => {
            add_bubble_event::<Pointer<DragLeave>>(self, node_id, system_id, stop_propagation, data)
         }
         BubblePointerEvent::Drop(data) => {
            add_bubble_event::<Pointer<Drop>>(self, node_id, system_id, stop_propagation, data)
         }
      }
   }

   fn add_event(
      &mut self,
      node_id: RendererNodeId<BevyRenderer>,
      event: ElementEventId,
      system_id: SystemId,
   ) {
      match event {
         ElementEventId::NoBubble {
            focus_input_event,
            trigger_way,
         } => self.add_focus_event(node_id, focus_input_event, trigger_way, system_id),
         ElementEventId::Bubble {
            event,
            stop_propagation,
         } => self.add_bubble_event(node_id, event, stop_propagation, system_id),
      }
   }

   fn remove_event(
      &mut self,
      node_id: RendererNodeId<BevyRenderer>,
      event: ElementEventId,
      system_id: SystemId,
   ) {
      match event {
         ElementEventId::NoBubble {
            focus_input_event,
            trigger_way,
         } => match focus_input_event {
            FocusInputEvent::Keyboard(data) => {
               self
                  .resource_mut::<FocusInputEvents<KeyCode>>()
                  .remove_node_events(&node_id, &(data, trigger_way));
            }
            FocusInputEvent::Mouse(data) => {
               self
                  .resource_mut::<FocusInputEvents<MouseButton>>()
                  .remove_node_events(&node_id, &(data, trigger_way));
            }
            FocusInputEvent::Gamepad(data) => {
               self
                  .resource_mut::<FocusInputEvents<GamepadButton>>()
                  .remove_node_events(&node_id, &(data, trigger_way));
            }
         },
         ElementEventId::Bubble { event, .. } => match event {
            BubblePointerEvent::Over => {
               remove_bubble_event::<Pointer<Over>>(self, node_id, system_id)
            }
            BubblePointerEvent::Out => {
               remove_bubble_event::<Pointer<Over>>(self, node_id, system_id)
            }
            BubblePointerEvent::Down(..) => {
               remove_bubble_event::<Pointer<Down>>(self, node_id, system_id)
            }
            BubblePointerEvent::Up(..) => {
               remove_bubble_event::<Pointer<Up>>(self, node_id, system_id)
            }
            BubblePointerEvent::Click(..) => {
               remove_bubble_event::<Pointer<Click>>(self, node_id, system_id)
            }
            BubblePointerEvent::Move => {
               remove_bubble_event::<Pointer<Move>>(self, node_id, system_id)
            }
            BubblePointerEvent::DragStart(..) => {
               remove_bubble_event::<Pointer<DragStart>>(self, node_id, system_id)
            }
            BubblePointerEvent::Drag(..) => {
               remove_bubble_event::<Pointer<Drag>>(self, node_id, system_id)
            }
            BubblePointerEvent::DragEnd(..) => {
               remove_bubble_event::<Pointer<DragEnd>>(self, node_id, system_id)
            }
            BubblePointerEvent::DragEnter(..) => {
               remove_bubble_event::<Pointer<DragEnter>>(self, node_id, system_id)
            }
            BubblePointerEvent::DragOver(..) => {
               remove_bubble_event::<Pointer<DragOver>>(self, node_id, system_id)
            }
            BubblePointerEvent::DragLeave(..) => {
               remove_bubble_event::<Pointer<DragLeave>>(self, node_id, system_id)
            }
            BubblePointerEvent::Drop(..) => {
               remove_bubble_event::<Pointer<Drop>>(self, node_id, system_id)
            }
         },
      }
   }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BubbleEventSystemIds<T>(
   pub smallvec::SmallVec<[(SystemId, Option<T::Data>); 2]>,
   PhantomData<T>,
)
where
   T: EventIsMatch;

impl<T> Default for BubbleEventSystemIds<T>
where
   T: EventIsMatch,
{
   fn default() -> Self {
      Self(Default::default(), Default::default())
   }
}

impl<T> Deref for BubbleEventSystemIds<T>
where
   T: EventIsMatch,
{
   type Target = smallvec::SmallVec<[(SystemId, Option<T::Data>); 2]>;

   fn deref(&self) -> &Self::Target {
      &self.0
   }
}

impl<T> DerefMut for BubbleEventSystemIds<T>
where
   T: EventIsMatch,
{
   fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.0
   }
}

impl<T> BubbleEventSystemIds<T>
where
   T: EventIsMatch,
{
   pub fn new(vec: smallvec::SmallVec<[(SystemId, Option<T::Data>); 2]>) -> Self {
      Self(vec, Default::default())
   }
}

#[derive(Reflect, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ElementEventId {
   NoBubble {
      focus_input_event: FocusInputEvent,
      trigger_way: FocusInputTriggerWay,
   },
   Bubble {
      event: BubblePointerEvent,
      stop_propagation: bool,
   },
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Reflect)]
#[reflect(Debug, Hash, PartialEq)]
pub enum FocusInputEvent {
   Keyboard(KeyCode),
   Mouse(MouseButton),
   Gamepad(GamepadButton),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(no_field_bounds)]
pub enum BubblePointerEvent {
   Over,
   Out,
   Down(Option<PointerButton>),
   Up(Option<PointerButton>),
   Click(Option<PointerButton>),
   Move,
   DragStart(Option<PointerButton>),
   Drag(Option<PointerButton>),
   DragEnd(Option<PointerButton>),
   DragEnter(Option<PointerButton>),
   DragOver(Option<PointerButton>),
   DragLeave(Option<PointerButton>),
   Drop(Option<PointerButton>),
}

impl BubblePointerEvent {
   pub fn stop_propagation(self) -> ElementEventId {
      ElementEventId::Bubble {
         event: self,
         stop_propagation: true,
      }
   }
}

#[derive(Reflect, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FocusInputTriggerWay {
   JustPressed,
   JustReleased,
   Pressed,
}

type EntityFocusInputEvents<T> =
   HashMap<(T, FocusInputTriggerWay), smallvec::SmallVec<[SystemId; 1]>>;
type FocusInputEventsInner<T> = EntityHashMap<EntityFocusInputEvents<T>>;

#[derive(Resource)]
pub struct FocusInputEvents<T> {
   inner: FocusInputEventsInner<T>,
}

impl<T> Default for FocusInputEvents<T> {
   fn default() -> Self {
      Self {
         inner: Default::default(),
      }
   }
}

impl<T> FocusInputEvents<T>
where
   T: Copy + Eq + Hash + Send + Sync + 'static,
{
   pub fn remove_node_all_events(&mut self, node_id: &RendererNodeId<BevyRenderer>) {
      self.remove(node_id);
   }
   pub fn remove_node_events(
      &mut self,
      node_id: &RendererNodeId<BevyRenderer>,
      key: &(T, FocusInputTriggerWay),
   ) {
      self.get_mut(node_id).unwrap().remove(key);
   }

   pub fn system_handle(
      registers: Res<FocusInputEvents<T>>,
      event_reader: Res<ButtonInput<T>>,
      focus: Res<FocusedEntity>,
      mut commands: Commands,
   ) {
      let Some(focus) = focus.0 else {
         return;
      };
      let Some(systems) = registers.get(&focus) else {
         return;
      };
      for ((input, input_way), system_ids) in systems.iter() {
         if match input_way {
            FocusInputTriggerWay::JustPressed => event_reader.just_pressed(*input),
            FocusInputTriggerWay::JustReleased => event_reader.just_released(*input),
            FocusInputTriggerWay::Pressed => event_reader.pressed(*input),
         } {
            for system_id in system_ids {
               commands.run_system(*system_id);
            }
         }
      }
   }
}

impl<T> Deref for FocusInputEvents<T>
where
   T: Copy + Eq + Hash + Send + Sync + 'static,
{
   type Target = FocusInputEventsInner<T>;

   fn deref(&self) -> &Self::Target {
      &self.inner
   }
}

impl<T> DerefMut for FocusInputEvents<T>
where
   T: Copy + Eq + Hash + Send + Sync + 'static,
{
   fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.inner
   }
}

pub trait FocusInputEventIterator: 'static {
   fn iter_events(self) -> impl Iterator<Item = FocusInputEvent> + Send + Clone + 'static;
}

macro_rules! impl_focus_input_event_iterator_for_tuples {
    ($($ty:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($ty),*> FocusInputEventIterator for ($($ty,)*)
            where
                $($ty: FocusInputEventIterator,)*
        {
            fn iter_events(self) -> impl Iterator<Item = FocusInputEvent> + Send + Clone + 'static{
                let ($($ty,)*) = self;
                core::iter::empty()
                    $(
                        .chain($ty.iter_events())
                    )*
            }
        }
    }
}

all_tuples!(impl_focus_input_event_iterator_for_tuples, 0, 4, T);

impl FocusInputEventIterator for KeyCode {
   fn iter_events(self) -> impl Iterator<Item = FocusInputEvent> + Send + Clone + 'static {
      once(FocusInputEvent::Keyboard(self))
   }
}

impl FocusInputEventIterator for MouseButton {
   fn iter_events(self) -> impl Iterator<Item = FocusInputEvent> + Send + Clone + 'static {
      once(FocusInputEvent::Mouse(self))
   }
}

impl FocusInputEventIterator for GamepadButton {
   fn iter_events(self) -> impl Iterator<Item = FocusInputEvent> + Send + Clone + 'static {
      once(FocusInputEvent::Gamepad(self))
   }
}

pub trait ElementEventIds: Clone + Send + 'static {
   fn iter_event_ids(self) -> impl Iterator<Item = ElementEventId> + Send + 'static;
}

impl ElementEventIds for ElementEventId {
   fn iter_event_ids(self) -> impl Iterator<Item = ElementEventId> + Send + 'static {
      once(self)
   }
}

impl ElementEventIds for Vec<ElementEventId> {
   fn iter_event_ids(self) -> impl Iterator<Item = ElementEventId> + Send + 'static {
      self.into_iter()
   }
}

impl ElementEventIds for Arc<Vec<ElementEventId>> {
   fn iter_event_ids(self) -> impl Iterator<Item = ElementEventId> + Send + 'static {
      pub struct IteratorWrapper(Arc<Vec<ElementEventId>>, usize);

      impl Iterator for IteratorWrapper {
         type Item = ElementEventId;

         fn next(&mut self) -> Option<Self::Item> {
            let r = self.0.get(self.1).map(|n| *n);
            if r.is_some() {
               self.1 += 1;
            }
            r
         }
      }

      IteratorWrapper(self, 0)
   }
}

impl ElementEventIds for BubblePointerEvent {
   fn iter_event_ids(self) -> impl Iterator<Item = ElementEventId> + Send + 'static {
      once(ElementEventId::Bubble {
         event: self,
         stop_propagation: false,
      })
   }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntoIteratorWrapper<T>(pub T);

impl<T> ElementEventIds for IntoIteratorWrapper<T>
where
   T: IntoIterator<Item = ElementEventId> + Clone + Send + 'static,
   T::IntoIter: Send,
{
   fn iter_event_ids(self) -> impl Iterator<Item = ElementEventId> + Send + 'static {
      self.0.into_iter()
   }
}

macro_rules! impl_element_evet_id_iterator_for_tuples {
    ($($ty:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($ty),*> ElementEventIds for ($($ty,)*)
            where
                $($ty: ElementEventIds,)*
        {
            fn iter_event_ids(self) -> impl Iterator<Item = ElementEventId> + Send + 'static{
                let ($($ty,)*) = self;
                core::iter::empty()
                    $(
                        .chain($ty.iter_event_ids())
                    )*
            }
        }
    }
}

all_tuples!(impl_element_evet_id_iterator_for_tuples, 0, 4, T);

macro_rules! impl_element_pointer_events_alias {
    ($($name:ident = $event_type:ty;)*) => {
        $(
           pub type $name = $event_type;
           paste::paste!{
               pub type [<ListenerInput $name>] = ListenerInput<$name>;
           }
        )*
    };
}
impl_element_pointer_events_alias!(
    PointerOver = Pointer<Over>;
    PointerOut = Pointer<Out>;
    PointerDown = Pointer<Down>;
    PointerUp = Pointer<Up>;
    PointerClick = Pointer<Click>;
    PointerMove = Pointer<Move>;
    PointerDragStart = Pointer<DragStart>;
    PointerDrag = Pointer<Drag>;
    PointerDragEnd = Pointer<DragEnd>;
    PointerDragEnter = Pointer<DragEnter>;
    PointerDragOver = Pointer<DragOver>;
    PointerDragLeave = Pointer<DragLeave>;
    PointerDrop = Pointer<Drop>;
);

pub fn x_trigger_way(
   trigger_way: FocusInputTriggerWay,
   events: impl FocusInputEventIterator,
) -> impl ElementEventIds {
   IntoIteratorWrapper(events.iter_events().map(move |n| ElementEventId::NoBubble {
      focus_input_event: n,
      trigger_way,
   }))
}

pub fn x_just_pressed(events: impl FocusInputEventIterator) -> impl ElementEventIds {
   x_trigger_way(FocusInputTriggerWay::JustPressed, events)
}

pub fn x_just_released(events: impl FocusInputEventIterator) -> impl ElementEventIds {
   x_trigger_way(FocusInputTriggerWay::JustReleased, events)
}

pub fn x_pressed(events: impl FocusInputEventIterator) -> impl ElementEventIds {
   x_trigger_way(FocusInputTriggerWay::Pressed, events)
}

pub fn x_pointer_over() -> BubblePointerEvent {
   BubblePointerEvent::Over
}

pub fn x_pointer_out() -> BubblePointerEvent {
   BubblePointerEvent::Out
}

pub fn x_pointer_down() -> BubblePointerEvent {
   BubblePointerEvent::Down(None)
}

pub fn x_pointer_up() -> BubblePointerEvent {
   BubblePointerEvent::Up(None)
}

pub fn x_pointer_click() -> BubblePointerEvent {
   BubblePointerEvent::Click(None)
}

pub fn x_pointer_move() -> BubblePointerEvent {
   BubblePointerEvent::Move
}

pub fn x_pointer_drag_start() -> BubblePointerEvent {
   BubblePointerEvent::DragStart(None)
}

pub fn x_pointer_drag() -> BubblePointerEvent {
   BubblePointerEvent::Drag(None)
}

pub fn x_pointer_drag_end() -> BubblePointerEvent {
   BubblePointerEvent::DragEnd(None)
}

pub fn x_pointer_drag_enter() -> BubblePointerEvent {
   BubblePointerEvent::DragEnter(None)
}

pub fn x_pointer_drag_over() -> BubblePointerEvent {
   BubblePointerEvent::DragOver(None)
}

pub fn x_pointer_drag_leave() -> BubblePointerEvent {
   BubblePointerEvent::DragLeave(None)
}

pub fn x_pointer_drop() -> BubblePointerEvent {
   BubblePointerEvent::Drop(None)
}
