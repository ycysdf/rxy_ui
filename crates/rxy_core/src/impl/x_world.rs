use core::marker::PhantomData;

use crate::{
   IntoElementView, IntoView, MaybeSend, MutableView, Renderer, RendererNodeId, RendererWorld,
   SoloView, View, ViewCtx, ViewMember, ViewMemberCtx, ViewMemberIndex, ViewMemberOrigin,
};

#[derive(Clone)]
pub struct XWorld<R, F>(pub F, PhantomData<R>);

impl<R, F> XWorld<R, F> {
   pub fn new(f: F) -> Self {
      XWorld(f, PhantomData)
   }
}

pub fn x_world<R, T, F>(f: F) -> XWorld<R, F>
where
   R: Renderer,
   F: FnOnce(&mut RendererWorld<R>) -> T + MaybeSend + 'static,
{
   XWorld(f, Default::default())
}

impl<F, R, MV> MutableView<R> for XWorld<R, F>
where
   MV: MutableView<R>,
   F: FnOnce(&mut RendererWorld<R>) -> MV + MaybeSend + 'static,
   R: Renderer,
{
   type Key = MV::Key;

   fn no_placeholder_when_no_rebuild() -> bool {
      MV::no_placeholder_when_no_rebuild()
   }

   fn build(self, ctx: ViewCtx<R>, placeholder_node_id: Option<RendererNodeId<R>>) -> Self::Key {
      self.0(&mut *ctx.world).build(ctx, placeholder_node_id)
   }

   fn rebuild(
      self,
      ctx: ViewCtx<R>,
      key: Self::Key,
      placeholder_node_id: RendererNodeId<R>,
   ) -> Option<Self::Key> {
      self.0(&mut *ctx.world).rebuild(ctx, key, placeholder_node_id)
   }
}

impl<R, F, VM> ViewMemberOrigin<R> for XWorld<R, F>
where
   F: FnOnce(&mut RendererWorld<R>) -> VM + MaybeSend + 'static,
   R: Renderer,
   VM: ViewMemberOrigin<R>,
{
   type Origin = VM::Origin;
}

impl<R, F, VM> ViewMember<R> for XWorld<R, F>
where
   F: FnOnce(&mut RendererWorld<R>) -> VM + MaybeSend + 'static,
   R: Renderer,
   VM: ViewMember<R>,
{
   fn count() -> ViewMemberIndex {
      1
   }

   fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
      VM::unbuild(ctx, view_removed)
   }

   fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
      self.0(&mut *ctx.world).build(
         ViewMemberCtx {
            index: ctx.index,
            world: &mut *ctx.world,
            node_id: ctx.node_id,
         },
         will_rebuild,
      )
   }

   fn rebuild(self, ctx: ViewMemberCtx<R>) {
      self.0(&mut *ctx.world).rebuild(ViewMemberCtx {
         index: ctx.index,
         world: &mut *ctx.world,
         node_id: ctx.node_id,
      })
   }
}

impl<R, F, IV> View<R> for XWorld<R, F>
where
   IV: IntoView<R>,
   F: FnOnce(&mut RendererWorld<R>) -> IV + MaybeSend + 'static,
   R: Renderer,
{
   type Key = <IV::View as View<R>>::Key;

   fn build(
      self,
      ctx: ViewCtx<R>,
      reserve_key: Option<Self::Key>,
      will_rebuild: bool,
   ) -> Self::Key {
      self.0(&mut *ctx.world)
         .into_view()
         .build(ctx, reserve_key, will_rebuild)
   }

   fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key) {
      self.0(&mut *ctx.world).into_view().rebuild(ctx, key)
   }
}

impl<R, F, IV> SoloView<R> for XWorld<R, F>
where
   IV: IntoView<R>,
   IV::View: SoloView<R>,
   F: FnOnce(&mut RendererWorld<R>) -> IV + MaybeSend + 'static,
   R: Renderer,
{
   fn node_id(key: &Self::Key) -> &RendererNodeId<R> {
      <IV::View as SoloView<R>>::node_id(key)
   }
}

impl<R, F, IV> IntoView<R> for XWorld<R, F>
where
   IV: IntoView<R>,
   F: FnOnce(&mut RendererWorld<R>) -> IV + MaybeSend + 'static,
   R: Renderer,
{
   type View = Self;

   fn into_view(self) -> Self::View {
      self
   }
}
