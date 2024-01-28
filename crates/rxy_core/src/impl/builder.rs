use alloc::boxed::Box;
use core::marker::PhantomData;
use std::any::TypeId;

use crate::{
    ElementView, IntoElementView, IntoView, MemberOwner, MutableView, Renderer, RendererNodeId,
    SoloView, View, ViewCtx, ViewMember, ViewMemberCtx,
};

#[derive(Clone)]
pub struct Builder<R, F>(pub F, PhantomData<R>);

pub fn view_builder<R, T, F>(f: F) -> Builder<R, F>
where
    R: Renderer,
    F: FnOnce(ViewCtx<R>, BuildFlags) -> T + Send + 'static,
    T: IntoView<R>,
{
    Builder(f, Default::default())
}

pub fn member_builder<R, T, F>(f: F) -> Builder<R, F>
where
    R: Renderer,
    F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> T+ Send + 'static,
    T: ViewMember<R>,
{
    Builder(f, Default::default())
}

pub fn style_builder<R, VM, F>(f: F) -> Builder<R, F>
where
    R: Renderer,
    F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> VM+ Send + 'static,
    VM: ViewMember<R>,
{
    Builder(f, Default::default())
}

#[derive(Debug, Clone, Copy)]
pub struct BuildFlags {
    pub will_rebuild: bool,
    pub is_rebuild: bool,
}

impl<F, R, MV> MutableView<R> for Builder<R, F>
where
    MV: MutableView<R>,
    F: FnOnce(ViewCtx<R>, BuildFlags) -> MV + Send + 'static,
    R: Renderer,
{
    type Key = MV::Key;

    fn build(
        self,
        ctx: ViewCtx<R>,
        will_rebuild: bool,
        state_node_id: RendererNodeId<R>,
    ) -> Self::Key {
        self.0(
            ViewCtx {
                world: &mut *ctx.world,
                parent: ctx.parent.clone(),
            },
            BuildFlags {
                will_rebuild,
                is_rebuild: true,
            },
        )
        .build(ctx, will_rebuild, state_node_id)
    }

    fn rebuild(
        self,
        ctx: ViewCtx<R>,
        key: Self::Key,
        state_node_id: RendererNodeId<R>,
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
        .rebuild(ctx, key, state_node_id)
    }
}

impl<R, F, U> ViewMember<R> for Builder<R, F>
where
    F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> U + Send + 'static,
    R: Renderer,
    U: ViewMember<R>,
{
    fn count() -> u8 {
        1
    }

    fn unbuild(_ctx: ViewMemberCtx<R>) {}

    fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
        self.0(
            ViewMemberCtx {
                index: ctx.index,
                type_id: ctx.type_id,
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
                type_id: TypeId::of::<U>(),
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
                type_id: ctx.type_id,
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
            type_id: TypeId::of::<U>(),
            world: &mut *ctx.world,
            node_id: ctx.node_id,
        })
    }
}

impl<R, F, IV> View<R> for Builder<R, F>
where
    IV: IntoView<R>,
    F: FnOnce(ViewCtx<R>, BuildFlags) -> IV + Send + 'static,
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
impl<R, F, IV> SoloView<R> for Builder<R, F>
where
    IV: IntoView<R>,
    IV::View: SoloView<R>,
    F: FnOnce(ViewCtx<R>, BuildFlags) -> IV + Send + 'static,
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
    F: FnOnce(ViewCtx<R>, BuildFlags) -> IV + Send + 'static,
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

impl<R, F, IV> IntoView<R> for Builder<R, F>
where
    IV: IntoView<R>,
    F: FnOnce(ViewCtx<R>, BuildFlags) -> IV + Send + 'static,
    R: Renderer,
{
    type View = Self;

    fn into_view(self) -> Self::View {
        self
    }
}

pub struct BoxedBuilder<R, T>(Box<dyn FnOnce(ViewCtx<R>, BuildFlags) -> T + Send + 'static>)
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

impl<R, V> MutableView<R> for BoxedBuilder<R, V>
where
    V: MutableView<R>,
    R: Renderer,
{
    type Key = V::Key;

    fn build(
        self,
        ctx: ViewCtx<R>,
        will_rebuild: bool,
        state_node_id: RendererNodeId<R>,
    ) -> Self::Key {
        MutableView::build(
            Builder(self.0, Default::default()),
            ctx,
            will_rebuild,
            state_node_id,
        )
    }

    fn rebuild(
        self,
        ctx: ViewCtx<R>,
        key: Self::Key,
        state_node_id: RendererNodeId<R>,
    ) -> Option<Self::Key> {
        MutableView::rebuild(Builder(self.0, Default::default()), ctx, key, state_node_id)
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
