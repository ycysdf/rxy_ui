use crate::{
   MutableView, MutableViewKey, Renderer, RendererNodeId, RendererWorld, View, ViewCtx, ViewKey,
};

pub fn to_mutable<T>(t: T) -> ToMutableWrapper<T> {
   ToMutableWrapper(t)
}

#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[derive(Hash, Clone, Debug, PartialEq)]
pub struct ToMutableWrapper<T>(pub T);

impl<VK: ViewKey<R>, R: Renderer> MutableViewKey<R> for ToMutableWrapper<VK> {
   fn remove(self, world: &mut RendererWorld<R>) {
      self.0.remove(world)
   }

   fn insert_before(
      &self,
      world: &mut RendererWorld<R>,
      parent: Option<&RendererNodeId<R>>,
      before_node_id: Option<&RendererNodeId<R>>,
   ) {
      self.0.insert_before(world, parent, before_node_id)
   }

   fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool) {
      self.0.set_visibility(world, hidden)
   }

   fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
      self.0.first_node_id(world)
   }

   fn state_node_id(&self) -> Option<RendererNodeId<R>> {
      self.0.state_node_id()
   }
}

impl<R, V> MutableView<R> for ToMutableWrapper<V>
where
   R: Renderer,
   V: View<R>,
{
   type Key = ToMutableWrapper<V::Key>;

   fn no_placeholder_when_no_rebuild() -> bool {
      true
   }

   fn build(self, ctx: ViewCtx<R>, placeholder_node_id: Option<RendererNodeId<R>>) -> Self::Key {
      ToMutableWrapper(self.0.build(ctx, None, placeholder_node_id.is_some()))
   }

   fn rebuild(
      self,
      ctx: ViewCtx<R>,
      key: Self::Key,
      _placeholder_node_id: RendererNodeId<R>,
   ) -> Option<Self::Key> {
      self.0.rebuild(ctx, key.0);
      None
   }
}
