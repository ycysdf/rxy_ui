use std::marker::PhantomData;

use bevy_ecs::prelude::{IntoSystem, System};
use bevy_utils::default;

use rxy_bevy_macro::{bevy_force_dynamic_view, BevyIntoView};
use rxy_core::{IntoView, View, ViewCtx};

use crate::BevyRenderer;

#[derive(BevyIntoView)]
pub struct SystemOnce<S, M, IV>(pub S, pub PhantomData<(M, IV)>)
where
    M: Send + 'static,
    IV: IntoView<BevyRenderer> + Send + 'static,
    S: IntoSystem<(), IV, M> + Send + 'static;

#[bevy_force_dynamic_view]
pub fn system_once<S, M, IV>(system: S) -> SystemOnce<S, M, IV>
where
    M: Send + 'static,
    IV: IntoView<BevyRenderer> + Send + 'static,
    S: IntoSystem<(), IV, M> + Send + 'static,
{
    SystemOnce(system, default())
}

impl<S, M, IV> View<BevyRenderer> for SystemOnce<S, M, IV>
where
    M: Send + 'static,
    IV: IntoView<BevyRenderer> + Send + 'static,
    S: IntoSystem<(), IV, M> + Send + 'static,
{
    type Key = <IV::View as View<BevyRenderer>>::Key;

    fn build(
        self,
        ViewCtx { world, parent }: ViewCtx<BevyRenderer>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        let mut system: S::System = IntoSystem::into_system(self.0);
        system.initialize(world);
        let view: IV::View = system.run((), world).into_view();
        system.apply_deferred(world);
        view.build(ViewCtx { world, parent }, reserve_key, will_rebuild)
    }

    fn rebuild(self, ctx: ViewCtx<BevyRenderer>, key: Self::Key) {
        let mut system = IntoSystem::into_system(self.0);
        system.initialize(ctx.world);
        let view: IV::View = system.run((), ctx.world).into_view();
        system.apply_deferred(ctx.world);
        view.rebuild(ctx, key)
    }
}
