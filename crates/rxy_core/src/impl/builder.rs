use core::marker::PhantomData;

use crate::{
   ElementView, IntoElementView, IntoView, MaybeSend, MutableView, Renderer, RendererNodeId,
   SoloView, View, ViewCtx, ViewMember, ViewMemberCtx, ViewMemberIndex, ViewMemberOrigin,
};

#[derive(Clone)]
pub struct XBuilder<R, F>(pub F, PhantomData<R>);

impl<R, F> XBuilder<R, F> {
   pub fn new(f: F) -> Self {
      XBuilder(f, PhantomData)
   }
}

pub fn view_builder<R, T, F>(f: F) -> XBuilder<R, F>
where
   R: Renderer,
   F: FnOnce(ViewCtx<R>, BuildFlags) -> T + MaybeSend + 'static,
   T: IntoView<R>,
{
   XBuilder(f, Default::default())
}

pub fn member_builder<R, T, F>(f: F) -> XBuilder<R, F>
where
   R: Renderer,
   F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> T + MaybeSend + 'static,
{
   XBuilder(f, Default::default())
}

pub fn style_builder<R, VM, F>(f: F) -> XBuilder<R, F>
where
   R: Renderer,
   F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> VM + MaybeSend + 'static,
{
   XBuilder(f, Default::default())
}

#[derive(Debug, Clone, Copy)]
pub struct BuildFlags {
   pub will_rebuild: bool,
   pub is_rebuild: bool,
}

impl<F, R, MV> MutableView<R> for XBuilder<R, F>
where
   MV: MutableView<R>,
   F: FnOnce(ViewCtx<R>, BuildFlags) -> MV + MaybeSend + 'static,
   R: Renderer,
{
   type Key = MV::Key;

   fn no_placeholder_when_no_rebuild() -> bool {
      MV::no_placeholder_when_no_rebuild()
   }

   fn build(self, ctx: ViewCtx<R>, placeholder_node_id: Option<RendererNodeId<R>>) -> Self::Key {
      self.0(
         ViewCtx {
            world: &mut *ctx.world,
            parent: ctx.parent.clone(),
         },
         BuildFlags {
            will_rebuild: placeholder_node_id.is_some(),
            is_rebuild: true,
         },
      )
      .build(ctx, placeholder_node_id)
   }

   fn rebuild(
      self,
      ctx: ViewCtx<R>,
      key: Self::Key,
      placeholder_node_id: RendererNodeId<R>,
   ) -> Option<Self::Key> {
      self.0(
         ViewCtx {
            world: &mut *ctx.world,
            parent: ctx.parent.clone(),
         },
         BuildFlags {
            will_rebuild: true,
            is_rebuild: false,
         },
      )
      .rebuild(ctx, key, placeholder_node_id)
   }
}

impl<R, F, VM> ViewMemberOrigin<R> for XBuilder<R, F>
where
   F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> VM + MaybeSend + 'static,
   R: Renderer,
   VM: ViewMemberOrigin<R>,
{
   type Origin = VM::Origin;
}

impl<R, F, VM> ViewMember<R> for XBuilder<R, F>
where
   F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> VM + MaybeSend + 'static,
   R: Renderer,
   VM: ViewMember<R>,
{
   fn count() -> ViewMemberIndex {
      VM::count()
   }

   fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
      VM::unbuild(ctx, view_removed)
   }

   fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
      self.0(
         ViewMemberCtx {
            index: ctx.index,
            world: &mut *ctx.world,
            node_id: ctx.node_id.clone(),
         },
         BuildFlags {
            will_rebuild,
            is_rebuild: false,
         },
      )
      .build(
         ViewMemberCtx {
            index: ctx.index,
            world: &mut *ctx.world,
            node_id: ctx.node_id,
         },
         will_rebuild,
      )
   }

   fn rebuild(self, ctx: ViewMemberCtx<R>) {
      self.0(
         ViewMemberCtx {
            index: ctx.index,
            world: &mut *ctx.world,
            node_id: ctx.node_id.clone(),
         },
         BuildFlags {
            will_rebuild: true,
            is_rebuild: true,
         },
      )
      .rebuild(ViewMemberCtx {
         index: ctx.index,
         world: &mut *ctx.world,
         node_id: ctx.node_id,
      })
   }
}

impl<R, F, IV> View<R> for XBuilder<R, F>
where
   IV: IntoView<R>,
   F: FnOnce(ViewCtx<R>, BuildFlags) -> IV + MaybeSend + 'static,
   R: Renderer,
{
   type Key = <IV::View as View<R>>::Key;

   fn build(
      self,
      ctx: ViewCtx<R>,
      reserve_key: Option<Self::Key>,
      will_rebuild: bool,
   ) -> Self::Key {
      self.0(
         ViewCtx {
            world: &mut *ctx.world,
            parent: ctx.parent.clone(),
         },
         BuildFlags {
            will_rebuild,
            is_rebuild: false,
         },
      )
      .into_view()
      .build(ctx, reserve_key, will_rebuild)
   }

   fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key) {
      self.0(
         ViewCtx {
            world: &mut *ctx.world,
            parent: ctx.parent.clone(),
         },
         BuildFlags {
            will_rebuild: true,
            is_rebuild: true,
         },
      )
      .into_view()
      .rebuild(ctx, key)
   }
}

impl<R, F, IV> SoloView<R> for XBuilder<R, F>
where
   IV: IntoView<R>,
   IV::View: SoloView<R>,
   F: FnOnce(ViewCtx<R>, BuildFlags) -> IV + MaybeSend + 'static,
   R: Renderer,
{
   fn node_id(key: &Self::Key) -> &RendererNodeId<R> {
      <IV::View as SoloView<R>>::node_id(key)
   }
}
/*
impl<R, F, IV> MemberOwner<R> for Builder<R, F>
where
    IV: IntoView<R>,
    IV::View: SoloView<R>,
    F: FnOnce(ViewCtx<R>, BuildFlags) -> IV + MaybeSend + 'static,
    R: Renderer,
{
    type E = ();
    type VM = ();
    type AddMember<T: ViewMember<R>> = ();
    type SetMembers<T: ViewMember<R> + MemberOwner<R>> = ();

    fn member<T>(self, member: T) -> Self::AddMember<T> where (Self::VM, T): ViewMember<R>, T: ViewMember<R> {
    }

    fn members<T: ViewMember<R>>(self, members: T) -> Self::SetMembers<(T, )> where T: ViewMember<R> {
    }
}*/

impl<R, F, IV> IntoView<R> for XBuilder<R, F>
where
   IV: IntoView<R>,
   F: FnOnce(ViewCtx<R>, BuildFlags) -> IV + MaybeSend + 'static,
   R: Renderer,
{
   type View = Self;

   fn into_view(self) -> Self::View {
      self
   }
}
/*
#[cfg(feature = "send_sync")]
pub struct BoxedBuilder<R, T>(Box<dyn FnOnce(ViewCtx<R>, BuildFlags) -> T + MaybeSend + 'static>)
where
    R: Renderer,
    T: ?Sized;

#[cfg(not(feature = "send_sync"))]
pub struct BoxedBuilder<R, T>(Box<dyn FnOnce(ViewCtx<R>, BuildFlags) -> T + 'static>)
where
    R: Renderer,
    T: ?Sized;

impl<R, IV> View<R> for BoxedBuilder<R, IV>
where
    IV: IntoView<R>,
    R: Renderer,
{
    type Key = <IV::View as View<R>>::Key;

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        View::build(
            Builder(self.0, Default::default()),
            ctx,
            reserve_key,
            will_rebuild,
        )
    }

    fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key) {
        View::rebuild(Builder(self.0, Default::default()), ctx, key);
    }
}

impl<R, MV> MutableView<R> for BoxedBuilder<R, MV>
where
    MV: MutableView<R>,
    R: Renderer,
{
    type Key = MV::Key;

    fn no_placeholder_when_no_rebuild() -> bool {
        MV::no_placeholder_when_no_rebuild()
    }

    fn build(self, ctx: ViewCtx<R>, placeholder_node_id: Option<RendererNodeId<R>>) -> Self::Key {
        MutableView::build(
            Builder(self.0, Default::default()),
            ctx,
            placeholder_node_id,
        )
    }

    fn rebuild(
        self,
        ctx: ViewCtx<R>,
        key: Self::Key,
        placeholder_node_id: RendererNodeId<R>,
    ) -> Option<Self::Key> {
        MutableView::rebuild(
            Builder(self.0, Default::default()),
            ctx,
            key,
            placeholder_node_id,
        )
    }
}

impl<R, V> IntoView<R> for BoxedBuilder<R, V>
where
    V: IntoView<R>,
    R: Renderer,
{
    type View = Self;

    fn into_view(self) -> Self::View {
        self
    }
}
*/

pub trait OnBuildExt<R>: ElementView<R>
where
   R: Renderer,
{
   #[inline]
   fn on_build<F, T>(self, f: F) -> Self::AddMember<XBuilder<R, F>>
   where
      F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> T + MaybeSend + 'static,
      T: ViewMember<R>,
   {
      self.member(member_builder(f))
   }
}

impl<R, EV> OnBuildExt<R> for EV
where
   R: Renderer,
   EV: ElementView<R>,
{
}
