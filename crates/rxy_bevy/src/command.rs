use crate::{BevyRenderer, RxyRootEntity};
use bevy_ecs::prelude::{Commands, World};
use bevy_ecs::system::Command;
use rxy_core::{IntoView, View, ViewCtx};

pub struct RxyUiSpawnCommand<V>
where
    V: View<BevyRenderer>,
{
    view: V,
}

impl<V> Command for RxyUiSpawnCommand<V>
where
    V: View<BevyRenderer>,
{
    fn apply(self, world: &mut World) {
        let root = world.resource::<RxyRootEntity>().0;
        let _ = self.view.build(
            ViewCtx {
                world,
                parent: root,
            },
            None,
            false,
        );
    }
}

pub trait RxyUiCommandExt {
    fn spawn_rxy_ui<IV>(&mut self, f: impl FnOnce() -> IV + Send + 'static)
    where
        IV: IntoView<BevyRenderer>;
}

impl<'w, 's> RxyUiCommandExt for Commands<'w, 's> {
    fn spawn_rxy_ui<IV>(&mut self, f: impl FnOnce() -> IV + Send + 'static)
    where
        IV: IntoView<BevyRenderer>,
    {
        self.add(move |world: &mut World| {
            let root = world.resource::<RxyRootEntity>().0;
            let _ = f().into_view().build(
                ViewCtx {
                    world,
                    parent: root,
                },
                None,
                false,
            );
        });
    }
}
