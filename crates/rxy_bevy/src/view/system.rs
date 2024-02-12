use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{In, IntoSystem, Resource, Schedules, World};
use bevy_ecs::schedule::{ScheduleLabel, SystemConfigs};
use bevy_utils::default;

use rxy_bevy_macro::{bevy_force_dynamic_view, BevyIntoView};
use rxy_core::{IntoView, View, ViewCtx};

use crate::BevyRenderer;

#[derive(BevyIntoView)]
pub struct SystemView<L, S, M, IV>(pub L, pub S, pub PhantomData<(M, IV)>)
where
    M: Send + 'static,
    IV: IntoView<BevyRenderer> + Send + 'static,
    L: ScheduleLabel,
    S: IntoSystem<(), IV, M> + Send + 'static;

impl<L, S, M, IV> SystemView<L, S, M, IV>
where
    M: Send + 'static,
    IV: IntoView<BevyRenderer> + Send + 'static,
    L: ScheduleLabel,
    S: IntoSystem<(), IV, M> + Send + 'static,
{
    pub fn configure<F>(self, f: F) -> ConfiguredSystemView<L, S, F, M, IV>
    where
        F: FnOnce(SystemConfigs) -> SystemConfigs + Send + 'static,
    {
        ConfiguredSystemView(self.0, self.1, f, self.2)
    }
}

/// .
///
/// # Safety
/// make sure that this view is always alive
/// .
#[bevy_force_dynamic_view]
pub unsafe fn system<L, S, M, IV>(label: L, system: S) -> SystemView<L, S, M, IV>
where
    M: Send + 'static,
    IV: IntoView<BevyRenderer> + Send + 'static,
    L: ScheduleLabel,
    S: IntoSystem<(), IV, M> + Send + 'static,
{
    SystemView::<L, S, M, IV>(label, system, default())
}

/// .
///
/// # Safety
/// make sure that this view is always alive
/// .
#[bevy_force_dynamic_view]
pub unsafe fn system_with_config<L, S, M, IV, F>(
    label: L,
    config_f: F,
    system: S,
) -> ConfiguredSystemView<L, S, F, M, IV>
where
    M: Send + 'static,
    IV: IntoView<BevyRenderer> + Send + 'static,
    L: ScheduleLabel,
    S: IntoSystem<(), IV, M> + Send + 'static,
    F: FnOnce(SystemConfigs) -> SystemConfigs + Send + 'static,
{
    SystemView::<L, S, M, IV>(label, system, default()).configure(config_f)
}

#[derive(Resource, Default)]
pub struct ScheduleSystemAdds {
    pub systems: Vec<(
        bevy_utils::intern::Interned<dyn ScheduleLabel>,
        SystemConfigs,
    )>,
}

pub fn handle_schedule_event(world: &mut World) {
    let systems: Vec<_> = world
        .resource_mut::<ScheduleSystemAdds>()
        .systems
        .drain(..)
        .collect();

    for (label, system) in systems {
        world.schedule_scope(label, move |_world, scheduler| {
            scheduler.add_systems(system);
        });
    }
}

pub fn add_system(world: &mut World, label: impl ScheduleLabel, system: SystemConfigs) {
    if world.resource::<Schedules>().contains(label.intern()) {
        world.schedule_scope(label.intern(), move |_world, scheduler| {
            scheduler.add_systems(system);
        })
    } else {
        world
            .resource_mut::<ScheduleSystemAdds>()
            .systems
            .push((label.intern(), system));
    }
}

impl<L, S, M, IV> View<BevyRenderer> for SystemView<L, S, M, IV>
where
    M: Send + 'static,
    IV: IntoView<BevyRenderer> + Send + 'static,
    L: ScheduleLabel,
    S: IntoSystem<(), IV, M> + Send + 'static,
{
    type Key = <IV::View as View<BevyRenderer>>::Key;

    fn build(
        self,
        ViewCtx { world, parent }: ViewCtx<BevyRenderer>,
        reserve_key: Option<Self::Key>,
        _will_rebuild: bool,
    ) -> Self::Key {
        let (system, key) = build_system(self.1, world, parent, reserve_key);
        add_system(world, self.0, system);
        key
    }

    fn rebuild(self, _ctx: ViewCtx<BevyRenderer>, _key: Self::Key) {
        todo!()
    }
}

#[derive(BevyIntoView)]
pub struct ConfiguredSystemView<L, S, F, M, IV>(pub L, pub S, pub F, pub PhantomData<(M, IV)>)
where
    M: Send + 'static,
    IV: IntoView<BevyRenderer> + Send + 'static,
    L: ScheduleLabel,
    S: IntoSystem<(), IV, M> + Send + 'static,
    F: FnOnce(SystemConfigs) -> SystemConfigs + Send + 'static;

impl<L, S, F, M, IV> View<BevyRenderer> for ConfiguredSystemView<L, S, F, M, IV>
where
    M: Send + 'static,
    IV: IntoView<BevyRenderer> + Send + 'static,
    L: ScheduleLabel,
    S: IntoSystem<(), IV, M> + Send + 'static,
    F: FnOnce(SystemConfigs) -> SystemConfigs + Send + 'static,
{
    type Key = <IV::View as View<BevyRenderer>>::Key;

    fn build(
        self,
        ViewCtx { world, parent }: ViewCtx<BevyRenderer>,
        reserve_key: Option<Self::Key>,
        _will_rebuild: bool,
    ) -> Self::Key {
        let (system, key) = build_system(self.1, world, parent, reserve_key);

        add_system(world, self.0, self.2(system));
        key
    }

    fn rebuild(self, _ctx: ViewCtx<BevyRenderer>, _key: Self::Key) {
        todo!()
    }
}

fn build_system<M, IV, S>(
    system: S,
    world: &mut World,
    parent: Entity,
    reserve_key: Option<<<IV as IntoView<BevyRenderer>>::View as View<BevyRenderer>>::Key>,
) -> (SystemConfigs, <IV::View as View<BevyRenderer>>::Key)
where
    M: Send + 'static,
    IV: IntoView<BevyRenderer> + Send + 'static,
    S: IntoSystem<(), IV, M> + Send + 'static,
{
    use bevy_ecs::system::System;
    let mut system: S::System = IntoSystem::into_system(system);
    system.initialize(world);
    let view: IV::View = system.run((), world).into_view();
    system.apply_deferred(world);
    let key = view.build(ViewCtx { world, parent }, reserve_key, true);
    use bevy_ecs::schedule::IntoSystemConfigs;
    let system1 = system.pipe({
        let key = key.clone();
        move |In(r): In<IV>, world: &mut World| {
            let view = r.into_view();
            view.rebuild(ViewCtx { world, parent }, key.clone());
        }
    });
    (system1.into_configs(), key)
}
