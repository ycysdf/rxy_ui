use crate::renderer::DeferredNodeTreeScoped;
use crate::utils::OnDrop;
use crate::utils::SyncCell;
use crate::{NodeTree, Renderer, RendererWorld, ViewKey};
use alloc::boxed::Box;

#[allow(dead_code)]
#[cfg(feature = "send_sync")]
pub struct ViewRemoveOnDrop(SyncCell<OnDrop<Box<dyn FnOnce() + crate::MaybeSend>>>);

#[allow(dead_code)]
#[cfg(not(feature = "send_sync"))]
pub struct ViewRemoveOnDrop(SyncCell<OnDrop<Box<dyn FnOnce()>>>);

pub trait RemoveOnDropWorldExt<R>
where
   R: Renderer,
{
   fn remove_on_drop<K>(&mut self, view_key: K) -> ViewRemoveOnDrop
   where
      K: ViewKey<R>;
}

impl<R> RemoveOnDropWorldExt<R> for RendererWorld<R>
where
   R: Renderer,
{
   fn remove_on_drop<K>(&mut self, view_key: K) -> ViewRemoveOnDrop
   where
      K: ViewKey<R>,
   {
      let deferred_world = self.deferred_world_scoped();
      ViewRemoveOnDrop(SyncCell::new(OnDrop::new(Box::new(move || {
         deferred_world.scoped(|world| view_key.remove(world))
      }))))
   }
}
