use bevy_ecs::prelude::{Commands, World};
use bevy_ecs::world::Command;

use rxy_core::{IntoView, RendererNodeId, RendererWorld, View, ViewCtx};

use crate::{BevyRenderer, RxyRootEntity};

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

pub trait RxyViewSpawner<IV>
where
   IV: IntoView<BevyRenderer>,
{
   type Output;
   fn spawn_view_on_root(&mut self, into_view: IV) -> Self::Output {
      self.spawn_view(into_view, |world| world.resource::<RxyRootEntity>().0)
   }
   fn spawn_view(
      &mut self,
      into_view: IV,
      parent: impl FnOnce(&mut RendererWorld<BevyRenderer>) -> RendererNodeId<BevyRenderer>
         + Send
         + 'static,
   ) -> Self::Output;
}

impl<'w, 's, IV> RxyViewSpawner<IV> for Commands<'w, 's>
where
   IV: IntoView<BevyRenderer>,
{
   type Output = ();

   fn spawn_view(
      &mut self,
      into_view: IV,
      parent_f: impl FnOnce(&mut RendererWorld<BevyRenderer>) -> RendererNodeId<BevyRenderer>
         + Send
         + 'static,
   ) -> Self::Output
   where
      IV: IntoView<BevyRenderer>,
   {
      let view = into_view.into_view();
      self.add(move |world: &mut World| {
         let parent = parent_f(world);
         let _ = view.build(ViewCtx { world, parent }, None, false);
      });
   }
}

impl<IV> RxyViewSpawner<IV> for World
where
   IV: IntoView<BevyRenderer>,
{
   type Output = <IV::View as View<BevyRenderer>>::Key;

   fn spawn_view(
      &mut self,
      into_view: IV,
      parent_f: impl FnOnce(&mut RendererWorld<BevyRenderer>) -> RendererNodeId<BevyRenderer>
         + Send
         + 'static,
   ) -> Self::Output
   where
      IV: IntoView<BevyRenderer>,
   {
      let parent = parent_f(self);
      into_view.into_view().build(
         ViewCtx {
            world: self,
            parent,
         },
         None,
         false,
      )
   }
}
