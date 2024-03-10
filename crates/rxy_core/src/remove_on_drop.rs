use crate::{NodeTree, Renderer, RendererWorld, ViewKey};
use bevy_utils::synccell::SyncCell;
use bevy_utils::OnDrop;
use crate::renderer::DeferredNodeTreeScoped;

#[allow(dead_code)]
pub struct ViewRemoveOnDrop(SyncCell<OnDrop<Box<dyn FnOnce() + Send>>>);

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
