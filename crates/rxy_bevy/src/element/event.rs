use core::{
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use bevy_a11y::Focus;
use bevy_app::PreUpdate;
use bevy_ecs::{
    prelude::{Commands, Entity, IntoSystem, Res, Resource, World},
    system::SystemId,
};
use bevy_input::{
    gamepad::{Gamepad, GamepadButton},
    keyboard::KeyCode,
    mouse::MouseButton,
    Input, InputSystem,
};
use bevy_mod_picking::prelude::*;
use bevy_utils::{EntityHashMap, HashMap};
use rxy_core::{
    prelude::{MemberOwner, Renderer, ViewMember, ViewMemberCtx},
    RendererNodeId,
};

use crate::{add_system, BevyRenderer, MemberOwnerBundleExt};

pub trait FocusEventWorldExt {
    fn add_focus_event<S>(
        &mut self,
        node_id: RendererNodeId<BevyRenderer>,
        input: FocusEventType,
        input_way: InputWay,
        system: S,
    ) where
        S: IntoSystem<(), (), M>;
}

#[inline]
pub fn add_focus_input_events_system<T>(world: &mut World)
where
    T: Copy + Eq + Hash + Send + Sync + 'static,
    FocusInputEvents<T>: Resource,
{
    add_system(
        world,
        PreUpdate,
        FocusInputEvents::<T>::system_handle
            .after(InputSystem)
            .run_if(|events: Res<FocusInputEvents<T>>| !events.is_empty()),
    )
}

impl FocusEventWorldExt for World {
    fn add_focus_event<M>(
        &mut self,
        node_id: RendererNodeId<BevyRenderer>,
        input: FocusEventType,
        input_way: InputWay,
        system: impl IntoSystem<(), (), M>,
    ) {
        use bevy_ecs::schedule::IntoSystemConfigs;

        let is_add_system = match input {
            FocusEventType::Keyboard(_) => self.contains_resource::<FocusInputEvents<KeyCode>>(),
            FocusEventType::Mouse(_) => self.contains_resource::<FocusInputEvents<MouseButton>>(),
            FocusEventType::Gamepad(_) => {
                self.contains_resource::<FocusInputEvents<GamepadButton>>()
            }
        };
        let system_id = self.register_system(system);

        let mut focus_input_events = self.get_resource_or_insert_with(FocusInputEvents::default);

        let events = focus_input_events.entry(node_id).or_default();
        events.entry((input, input_way)).or_default().push(system_id);

        if !is_add_system {
            match input {
                FocusEventType::Keyboard(_) => add_focus_input_events_system::<KeyCode>(self),
                FocusEventType::Mouse(_) => {
                    self.contains_resource::<FocusInputEvents<MouseButton>>()
                }
                FocusEventType::Gamepad(_) => {
                    self.contains_resource::<FocusInputEvents<GamepadButton>>()
                }
            }
        }
    }
}

pub trait IntoElementEventId {
    fn into(self) -> impl Iterator<Item = ElementEventId>;
}

pub enum ElementEventId {
    NoBubble {
        input_event: FocusEventType,
        input_way: InputWay,
    },
    Bubble(PointerKey),
}

impl ElementEventId {
    pub fn add(self, ctx: ViewMemberCtx<BevyRenderer>, system_id: SystemId) {}
}

pub enum FocusEventType {
    Keyboard(KeyCode),
    Mouse(MouseButton),
    Gamepad(GamepadButton),
}

pub enum PointerKey {
    Over,
    Out,
    Down(PointerButton),
    Up(PointerButton),
    Click(PointerButton),
    Move,
    DragStart(PointerButton),
    Drag(PointerButton),
    DragEnd(PointerButton),
    DragEnter(PointerButton),
    DragOver(PointerButton),
    DragLeave(PointerButton),
    Drop(PointerButton),
}

// pub trait InputEventSeter {
//     fn add(self,system_id: SystemId);
// }

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum InputWay {
    JustPressed,
    JustReleased,
    Pressed,
}

type EntityFocusInputEvents<T> = HashMap<(T, InputWay), smallvec::SmallVec<[SystemId; 1]>>;
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
                InputWay::JustPressed => event_reader.just_pressed(*input),
                InputWay::JustReleased => event_reader.just_released(*input),
                InputWay::Pressed => event_reader.pressed(*input),
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
pub struct FocusInputEventMemberState(SystemId);

pub struct FocusInputEventMember<T, S, M> {
    input: T,
    input_way: InputWay,
    system: S,
    _marker: PhantomData<M>,
}

impl<T, S, M> ViewMember<BevyRenderer> for FocusInputEventMember<T, S, M>
where
    T: Copy + Eq + Hash + Send + Sync + 'static,
    S: IntoSystem<(), (), M> + Send + 'static,
    M: Send + 'static,
{
    fn count() -> rxy_core::ViewMemberIndex {
        1
    }

    fn unbuild(mut ctx: ViewMemberCtx<BevyRenderer>) {
        let state = ctx.take_indexed_view_member_state::<FocusInputEventMemberState>().unwrap();
        let _ = ctx.world.remove_system(state.0);
    }

    fn build(self, mut ctx: ViewMemberCtx<BevyRenderer>, _will_rebuild: bool) {
        use bevy_ecs::schedule::IntoSystemConfigs;

        let is_add_system = ctx.world.contains_resource::<FocusInputEvents<T>>();
        let system_id = ctx.world.register_system(self.system);

        let mut focus_input_events =
            ctx.world.get_resource_or_insert_with(FocusInputEvents::default);

        let events = focus_input_events.entry(ctx.node_id).or_default();
        events.entry((self.input, self.input_way)).or_default().push(system_id);

        if !is_add_system {
            add_system(
                ctx.world,
                PreUpdate,
                FocusInputEvents::<T>::system_handle
                    .after(InputSystem)
                    .run_if(|events: Res<FocusInputEvents<T>>| !events.is_empty()),
            );
        }
        ctx.set_indexed_view_member_state(FocusInputEventMemberState(system_id));
    }

    fn rebuild(self, ctx: ViewMemberCtx<BevyRenderer>) {
        Self::unbuild(ViewMemberCtx {
            index: ctx.index,
            type_id: ctx.type_id,
            world: &mut *ctx.world,
            node_id: ctx.node_id,
        });
        self.build(ctx, true);
    }
}

impl<T> ElementKeyboardEvents for T where T: MemberOwner<BevyRenderer> + Sized {}

pub trait ElementKeyboardEvents: MemberOwner<BevyRenderer> + Sized {
    // fn on<EE: EntityEvent, Marker>(
    //     self,
    //     system: impl bevy_ecs::prelude::IntoSystem<(), (), Marker>,
    // ) -> Self::AddMember<XBundle<On<EE>>> {
    //     use bevy_mod_picking::prelude::*;
    //     self.bundle(On::<EE>::run(system))
    // }

    fn on_input_way<T, S, Marker>(
        self,
        input: T,
        input_way: InputWay,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<T, S, Marker>>
    where
        T: Copy + Eq + Hash + Send + Sync + 'static,
        S: bevy_ecs::prelude::IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.member(FocusInputEventMember {
            input,
            input_way,
            system,
            _marker: Default::default(),
        })
    }

    fn on_pressed<T, S, Marker>(
        self,
        input: T,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<T, S, Marker>>
    where
        T: Copy + Eq + Hash + Send + Sync + 'static,
        S: bevy_ecs::prelude::IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on_input_way(input, InputWay::Pressed, system)
    }

    fn on_return<S, Marker>(
        self,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<KeyCode, S, Marker>>
    where
        S: bevy_ecs::prelude::IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on_just_pressed(KeyCode::Return, system)
    }

    fn on_esc<S, Marker>(
        self,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<KeyCode, S, Marker>>
    where
        S: bevy_ecs::prelude::IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on_just_pressed(KeyCode::Escape, system)
    }

    fn on_just_pressed<T, S, Marker>(
        self,
        input: T,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<T, S, Marker>>
    where
        T: Copy + Eq + Hash + Send + Sync + 'static,
        S: bevy_ecs::prelude::IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on_input_way(input, InputWay::JustPressed, system)
    }

    fn on_just_released<T, S, Marker>(
        self,
        input: T,
        system: S,
    ) -> Self::AddMember<FocusInputEventMember<T, S, Marker>>
    where
        T: Copy + Eq + Hash + Send + Sync + 'static,
        S: bevy_ecs::prelude::IntoSystem<(), (), Marker> + Send + 'static,
        Marker: Send + 'static,
    {
        self.on_input_way(input, InputWay::JustReleased, system)
    }
}

macro_rules! impl_element_pointer_events_members {
    ($($name:ident = $event_type:ty;)*) => {
        $(
           pub type $name = $event_type;
           paste::paste!{
               pub type [<ListenerInput $name>] = ListenerInput<$name>;
           }
        )*

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
