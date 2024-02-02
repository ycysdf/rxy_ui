use core::{
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};
use std::iter::once;

use bevy_a11y::Focus;
use bevy_app::PreUpdate;
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    prelude::{Commands, Entity, IntoSystem, Res, Resource, World},
    system::SystemId,
};
use bevy_input::{
    gamepad::GamepadButton,
    keyboard::KeyCode,
    mouse::MouseButton,
    Input, InputSystem,
};
use bevy_mod_picking::prelude::*;
use bevy_reflect::Reflect;
use bevy_utils::tracing::error;
use bevy_utils::{all_tuples, EntityHashMap, HashMap};
use rxy_core::{
    prelude::{MemberOwner, ViewMember, ViewMemberCtx},
    Renderer, RendererNodeId, RendererWorld,
};

use crate::{add_system, BevyRenderer};

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
    events.entry((input, input_way)).or_default().push(system_id);

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

fn add_bubble_event<T>(
    world: &mut RendererWorld<BevyRenderer>,
    node_id: RendererNodeId<BevyRenderer>,
    system_id: SystemId,
    stop_propagation: bool,
    mut filter: Option<impl FnMut(&T) -> bool + Send + Sync + 'static>,
) where
    T: EntityEvent,
{
    let mut entity_world_mut = world.entity_mut(node_id);
    if entity_world_mut.contains::<On<T>>() {
        let system_ids = BevyRenderer::get_or_insert_default_state_by_entity_mut::<
            BubbleEventSystemIds,
        >(&mut entity_world_mut);
        system_ids.push(system_id);
    } else {
        entity_world_mut.world_scope(|world| {
            BevyRenderer::set_node_state(
                world,
                &node_id,
                BubbleEventSystemIds(smallvec::SmallVec::from_elem(system_id, 1)),
            );
        });
        entity_world_mut.insert(On::<T>::run(move |world: &mut World| {
            let mut listerner = world.resource_mut::<ListenerInput<T>>();
            let data: &T = listerner.deref();
            if let Some(filter) = &mut filter {
                if !filter(data) {
                    return;
                }
            }
            if stop_propagation {
                listerner.stop_propagation();
            }

            BevyRenderer::node_state_scoped(
                world,
                &node_id,
                |world, system_ids: &mut BubbleEventSystemIds| {
                    for system_id in system_ids.iter() {
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
            BubblePointerEvent::Over => add_bubble_event::<Pointer<Over>>(
                self,
                node_id,
                system_id,
                stop_propagation,
                Some(always_true),
            ),
            BubblePointerEvent::Out => add_bubble_event::<Pointer<Over>>(
                self,
                node_id,
                system_id,
                stop_propagation,
                Some(always_true),
            ),
            BubblePointerEvent::Down(data) => add_bubble_event::<Pointer<Down>>(
                self,
                node_id,
                system_id,
                stop_propagation,
                data.map(|data| move |d: &Pointer<Down>| d.button == data),
            ),
            BubblePointerEvent::Up(data) => add_bubble_event::<Pointer<Up>>(
                self,
                node_id,
                system_id,
                stop_propagation,
                data.map(|data| move |d: &Pointer<Up>| d.button == data),
            ),
            BubblePointerEvent::Click(data) => add_bubble_event::<Pointer<Click>>(
                self,
                node_id,
                system_id,
                stop_propagation,
                data.map(|data| move |d: &Pointer<Click>| d.button == data),
            ),
            BubblePointerEvent::Move => add_bubble_event::<Pointer<Move>>(
                self,
                node_id,
                system_id,
                stop_propagation,
                Some(always_true),
            ),
            BubblePointerEvent::DragStart(data) => add_bubble_event::<Pointer<DragStart>>(
                self,
                node_id,
                system_id,
                stop_propagation,
                data.map(|data| move |d: &Pointer<DragStart>| d.button == data),
            ),
            BubblePointerEvent::Drag(data) => add_bubble_event::<Pointer<Drag>>(
                self,
                node_id,
                system_id,
                stop_propagation,
                data.map(|data| move |d: &Pointer<Drag>| d.button == data),
            ),
            BubblePointerEvent::DragEnd(data) => add_bubble_event::<Pointer<DragEnd>>(
                self,
                node_id,
                system_id,
                stop_propagation,
                data.map(|data| move |d: &Pointer<DragEnd>| d.button == data),
            ),
            BubblePointerEvent::DragEnter(data) => add_bubble_event::<Pointer<DragEnter>>(
                self,
                node_id,
                system_id,
                stop_propagation,
                data.map(|data| move |d: &Pointer<DragEnter>| d.button == data),
            ),
            BubblePointerEvent::DragOver(data) => add_bubble_event::<Pointer<DragOver>>(
                self,
                node_id,
                system_id,
                stop_propagation,
                data.map(|data| move |d: &Pointer<DragOver>| d.button == data),
            ),
            BubblePointerEvent::DragLeave(data) => add_bubble_event::<Pointer<DragLeave>>(
                self,
                node_id,
                system_id,
                stop_propagation,
                data.map(|data| move |d: &Pointer<DragLeave>| d.button == data),
            ),
            BubblePointerEvent::Drop(data) => add_bubble_event::<Pointer<Drop>>(
                self,
                node_id,
                system_id,
                stop_propagation,
                data.map(|data| move |d: &Pointer<Drop>| d.button == data),
            ),
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
                    self.resource_mut::<FocusInputEvents<KeyCode>>()
                        .remove_node_events(&node_id, &(data, trigger_way));
                }
                FocusInputEvent::Mouse(data) => {
                    self.resource_mut::<FocusInputEvents<MouseButton>>()
                        .remove_node_events(&node_id, &(data, trigger_way));
                }
                FocusInputEvent::Gamepad(data) => {
                    self.resource_mut::<FocusInputEvents<GamepadButton>>()
                        .remove_node_events(&node_id, &(data, trigger_way));
                }
            },
            ElementEventId::Bubble { .. } => {
                let system_ids =
                    BevyRenderer::get_node_state_mut::<BubbleEventSystemIds>(self, &node_id).unwrap();
                system_ids.retain(|n| *n == system_id);
            }
        }
    }
}

pub fn always_true<T>(_: &T) -> bool {
    true
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, DerefMut, Deref)]
pub struct BubbleEventSystemIds(smallvec::SmallVec<[SystemId; 2]>);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FocusInputTriggerWay {
    JustPressed,
    JustReleased,
    Pressed,
}

type EntityFocusInputEvents<T> =
    HashMap<(T, FocusInputTriggerWay), smallvec::SmallVec<[SystemId; 1]>>;
type FocusInputEventsInner<T> = EntityHashMap<Entity, EntityFocusInputEvents<T>>;

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
        event_reader: Res<Input<T>>,
        focus: Res<Focus>,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FocusInputEventMemberState<T>(SystemId, T);

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

pub trait ElementEventIdIterator: Clone + Send + 'static {
    fn iter_event_ids(&self) -> impl Iterator<Item = ElementEventId> + Send;
}

impl ElementEventIdIterator for ElementEventId {
    fn iter_event_ids(&self) -> impl Iterator<Item = ElementEventId> + Send {
        core::iter::once(*self)
    }
}

impl ElementEventIdIterator for BubblePointerEvent {
    fn iter_event_ids(&self) -> impl Iterator<Item = ElementEventId> + Send {
        core::iter::once(ElementEventId::Bubble { event: *self, stop_propagation: false })
    }
}

#[derive(Clone)]
pub struct IntoIteratorWrapper<T>(T);

impl<T> ElementEventIdIterator for IntoIteratorWrapper<T>
where
    T: IntoIterator<Item = ElementEventId> + Clone + Send + 'static,
    T::IntoIter: Send,
{
    fn iter_event_ids(&self) -> impl Iterator<Item = ElementEventId> + Send {
        self.clone().0.into_iter()
    }
}

macro_rules! impl_element_evet_id_iterator_for_tuples {
    ($($ty:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($ty),*> ElementEventIdIterator for ($($ty,)*)
            where
                $($ty: ElementEventIdIterator,)*
        {
            fn iter_event_ids(&self) -> impl Iterator<Item = ElementEventId> + Send {
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

pub fn x_trigger_way(
    trigger_way: FocusInputTriggerWay,
    events: impl FocusInputEventIterator,
) -> impl ElementEventIdIterator {
    IntoIteratorWrapper(events.iter_events().map(move |n| ElementEventId::NoBubble {
        focus_input_event: n,
        trigger_way,
    }))
}

pub fn x_just_pressed(events: impl FocusInputEventIterator) -> impl ElementEventIdIterator {
    x_trigger_way(FocusInputTriggerWay::JustPressed, events)
}
pub fn x_just_released(events: impl FocusInputEventIterator) -> impl ElementEventIdIterator {
    x_trigger_way(FocusInputTriggerWay::JustReleased, events)
}
pub fn x_pressed(events: impl FocusInputEventIterator) -> impl ElementEventIdIterator {
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

pub struct FocusInputEventMember<T, S, M> {
    element_event_ids: T,
    system: S,
    _marker: PhantomData<M>,
}

impl<T, S, M> ViewMember<BevyRenderer> for FocusInputEventMember<T, S, M>
where
    T: ElementEventIdIterator,
    S: IntoSystem<(), (), M> + Send + 'static,
    M: Send + 'static,
{
    fn count() -> rxy_core::ViewMemberIndex {
        1
    }

    fn unbuild(mut ctx: ViewMemberCtx<BevyRenderer>, _view_removed: bool) {
        let state = ctx.take_indexed_view_member_state::<FocusInputEventMemberState<T>>().unwrap();
        for event_id in state.1.iter_event_ids() {
            ctx.world.remove_event(ctx.node_id, event_id, state.0);
        }
        if let Err(err) = ctx.world.remove_system(state.0) {
            error!("remove_system error: {:?}", err);
        }
    }

    fn build(self, mut ctx: ViewMemberCtx<BevyRenderer>, _will_rebuild: bool) {
        let system_id = ctx.world.register_system(self.system);

        for event_id in self.element_event_ids.iter_event_ids() {
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
    ) -> Self::AddMember<FocusInputEventMember<T, S, Marker>>
    where
        T: ElementEventIdIterator,
        S: IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.member(FocusInputEventMember {
            element_event_ids,
            system,
            _marker: Default::default(),
        })
    }

    fn on_pressed<T, S, Marker>(
        self,
        events: impl FocusInputEventIterator,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
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
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
    where
        S: IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on_just_pressed(KeyCode::Return, system)
    }

    fn on_esc<S, Marker>(
        self,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
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
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
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
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
    where
        S: IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on(x_just_released(events), system)
    }

    fn on_pointer_over<S, Marker>(
        self,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
    where
        S: IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on(x_pointer_over(), system)
    }

    fn on_pointer_out<S, Marker>(
        self,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
    where
        S: IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on(x_pointer_out(), system)
    }

    fn on_pointer_down<S, Marker>(
        self,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
    where
        S: IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on(x_pointer_down(), system)
    }

    fn on_pointer_up<S, Marker>(
        self,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
    where
        S: IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on(x_pointer_up(), system)
    }

    fn on_pointer_click<S, Marker>(
        self,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
    where
        S: IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on(x_pointer_click(), system)
    }
    
    fn on_pointer_move<S, Marker>(
        self,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
    where
        S: IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on(x_pointer_move(), system)
    }
    fn on_pointer_drag_start<S, Marker>(
        self,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
    where
        S: IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on(x_pointer_drag_start(), system)
    }
    fn on_pointer_drag<S, Marker>(
        self,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
    where
        S: IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on(x_pointer_drag(), system)
    }
    fn on_pointer_drag_end<S, Marker>(
        self,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
    where
        S: IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on(x_pointer_drag_end(), system)
    }
    fn on_pointer_drag_enter<S, Marker>(
        self,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
    where
        S: IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on(x_pointer_drag_enter(), system)
    }
    fn on_pointer_drag_over<S, Marker>(
        self,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
    where
        S: IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on(x_pointer_drag_over(), system)
    }
    fn on_pointer_drag_leave<S, Marker>(
        self,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
    where
        S: IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on(x_pointer_drag_leave(), system)
    }
    fn on_pointer_drop<S, Marker>(
        self,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<impl ElementEventIdIterator, S, Marker>>
    where
        S: IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on(x_pointer_drop(), system)
    }
}

macro_rules! impl_element_pointer_events_members {
    ($($name:ident = $event_type:ty;)*) => {
        $(
           pub type $name = $event_type;
           paste::paste!{
               pub type [<ListenerInput $name>] = ListenerInput<$name>;
           }
        )*/* 

        impl<T> ElementPointerEvents for T
        where
            T: rxy_core::MemberOwner<$crate::BevyRenderer>+Sized {}

        pub trait ElementPointerEvents: rxy_core::MemberOwner<$crate::BevyRenderer>+Sized
        {
            $(
                paste::paste!{
                    fn [<on_ $name:snake>]<Marker>(
                        self,
                        system: impl bevy_ecs::prelude::IntoSystem<(), (), Marker>,
                    ) -> Self::AddMember<$crate::XBundle<On<$name>>> {
                        self.bundle(On::<$name>::run(system))
                    }
                }
            )*
        }
 */
    };
}
impl_element_pointer_events_members!(
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
