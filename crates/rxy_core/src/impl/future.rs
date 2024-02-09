use core::future::{Future, IntoFuture};
use core::marker::PhantomData;

use futures_lite::{FutureExt, StreamExt};

use crate::build_info::{node_build_status, node_build_times_increment};
use crate::renderer::DeferredNodeTreeScoped;
use crate::{
    InnerIvmToVm, IntoView, XNest, Mapper, MaybeSend, NodeTree, Renderer, TaskState, View,
    ViewCtx, ViewKey, ViewMember, ViewMemberCtx, ViewMemberIndex, ViewMemberOrigin,
};

pub struct XFuture<T>(pub T);

#[inline(always)]
pub fn x_future<T>(f: impl IntoFuture<IntoFuture = T>) -> XFuture<T>
where
    T: Future,
{
    XFuture(f.into_future())
}

pub type FutureViewKey<R, T> = <<<T as Future>::Output as IntoView<R>>::View as View<R>>::Key;

impl<R, T> View<R> for XFuture<T>
where
    R: Renderer,
    T: Future + MaybeSend + 'static,
    T::Output: IntoView<R> + MaybeSend + 'static,
{
    type Key = FutureViewKey<R, T>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        _reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        let key = FutureViewKey::<R, T>::reserve_key(ctx.world, will_rebuild);

        future_view_rebuild(self.0, ctx, will_rebuild, key.clone());
        key
    }

    fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key) {
        let Some(state_node_id) = key.state_node_id() else {
            return;
        };
        drop(ctx.world.take_node_state::<XFutureState<R>>(&state_node_id));
        future_view_rebuild(self.0, ctx, true, key);
    }
}

pub struct XFutureState<R>(pub TaskState<R>)
where
    R: Renderer;

impl<R> XFutureState<R>
where
    R: Renderer,
{
    pub fn new(task: R::Task<()>) -> Self {
        Self(TaskState::new(task))
    }
}

fn future_view_rebuild<R, T>(
    future: T,
    ctx: ViewCtx<R>,
    will_rebuild: bool,
    key: FutureViewKey<R, T>,
) where
    R: Renderer,
    T: Future + MaybeSend + 'static,
    T::Output: IntoView<R> + MaybeSend + 'static,
{
    let Some(state_node_id) = key.state_node_id() else {
        return;
    };
    let world_scoped = ctx.world.deferred_world_scoped();

    let task = R::spawn_task(async move {
        let view = future.await;
        world_scoped.scoped(move |world| {
            let Some(state_node_id) = key.state_node_id() else {
                return;
            };
            let view = view.into_view();
            if node_build_status::<R>(world, &state_node_id).is_no_build() {
                view.build(
                    ViewCtx {
                        world,
                        parent: ctx.parent.clone(),
                    },
                    Some(key),
                    will_rebuild,
                );
            } else {
                view.rebuild(
                    ViewCtx {
                        world,
                        parent: ctx.parent.clone(),
                    },
                    key,
                );
            }
            if will_rebuild {
                node_build_times_increment::<R>(world, state_node_id);
            }
        });
    });
    ctx.world.ensure_spawn(state_node_id.clone());
    ctx.world
        .set_node_state(&state_node_id, XFutureState::<R>::new(task));
}

impl<R, T> IntoView<R> for XFuture<T>
where
    R: Renderer,
    T: Future + MaybeSend + 'static,
    T::Output: IntoView<R> + MaybeSend + 'static,
{
    type View = Self;

    fn into_view(self) -> Self::View {
        self
    }
}

pub fn future_view_member_rebuild<R, T>(future: T, mut ctx: ViewMemberCtx<R>, will_rebuild: bool)
where
    R: Renderer,
    T: Future + MaybeSend + 'static,
    T::Output: ViewMember<R> + MaybeSend + 'static,
{
    drop(ctx.take_indexed_view_member_state::<TaskState<R>>());
    let world_scoped = ctx.world.deferred_world_scoped();

    let node_id = ctx.node_id.clone();
    let task = R::spawn_task(async move {
        let view_member = future.await;
        world_scoped.scoped(move |world| {
            let mut ctx = ViewMemberCtx::<R> {
                index: ctx.index,
                world,
                node_id,
            };

            if ctx.build_status().is_no_build() {
                view_member.build(
                    ViewMemberCtx {
                        index: ctx.index,
                        world: &mut *ctx.world,
                        node_id: ctx.node_id.clone(),
                    },
                    will_rebuild,
                );
            } else {
                view_member.rebuild(ViewMemberCtx {
                    index: ctx.index,
                    world: &mut *ctx.world,
                    node_id: ctx.node_id.clone(),
                });
            }
            if will_rebuild {
                ctx.build_times_increment();
            }
        });
    });
    ctx.set_indexed_view_member_state(TaskState::<R>::new(task));
}

// impl<R, TO, VM> ViewMemberOrigin<R> for InnerIvmToVm<XFuture<TO>, VM>
// where
//     R: Renderer,
//     TO: Future + MaybeSend + 'static,
//     VM: ViewMemberOrigin<R>,
//     TO::Output: XNest<R, MapMember = VM> + MaybeSend + 'static,
// {
//     type Origin = VM::Origin;
// }

// todo: ivm
// impl<R, TO, VM, M> ViewMember<R> for InnerIvmToVm<XFuture<TO>, M>
// where
//     R: Renderer,
//     TO: Future + MaybeSend + 'static,
//     VM: ViewMember<R>,
//     TO::Output: XNest<R, MapMember<M> = VM> + MaybeSend + 'static,
//     M: Mapper<R>,
// {
//     fn count() -> ViewMemberIndex {
//         VM::count()
//     }
//
//     fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
//         VM::unbuild(ctx, view_removed)
//     }
//
//     fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
//         let reactive = x_future(async move { self.0 .0.await.into_member() });
//         reactive.build(ctx, will_rebuild)
//     }
//
//     fn rebuild(self, ctx: ViewMemberCtx<R>) {
//         let reactive = x_future(async move { self.0 .0.await.into_member() });
//         reactive.rebuild(ctx)
//     }
// }

// todo: ivm
// impl<R, TO> XNest<R> for XFuture<TO>
// where
//     R: Renderer,
//     TO: Future + MaybeSend + 'static,
//     TO::Output: XNest<R> + MaybeSend + 'static,
// {
//     type InnerMember = <TO::Output as XNest<R>>::InnerMember;
//     type MapMember<M: Mapper<Self>> =
//         InnerIvmToVm<Self, <TO::Output as XNest<R>>::MapMember<M>>;
//
//     fn map_inner<U: Mapper<Self>>(self) -> Self::MapMember<U> {
//         InnerIvmToVm::new(self)
//     }
// }

// impl<R, T> ViewMemberOrigin<R> for XFuture<T>
// where
//     R: Renderer,
//     T: Future + MaybeSend + 'static,
//     T::Output: ViewMemberOrigin<R> + MaybeSend + 'static,
// {
//     type Origin = <T::Output as ViewMemberOrigin<R>>::Origin;
// }

impl<R, T> ViewMember<R> for XFuture<T>
where
    R: Renderer,
    T: Future + MaybeSend + 'static,
    T::Output: ViewMember<R> + MaybeSend + 'static,
{
    fn count() -> ViewMemberIndex {
        T::Output::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
        T::Output::unbuild(ctx, view_removed)
    }

    fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
        future_view_member_rebuild(self.0, ctx, will_rebuild)
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        self.build(ctx, true);
    }
}

// todo: ivm
// impl<R, T> XNest<R> for futures_lite::future::Boxed<T>
// where
//     R: Renderer,
//     T: XNest<R> + MaybeSend + 'static,
// {
//     type InnerMember = T::InnerMember;
//     type MapMember<M: Mapper<Self>> = futures_lite::future::Boxed<T::MapMember<M>>;
//
//     fn map_inner<U: Mapper<Self>>(self) -> Self::MapMember<U> {
//         async move { U::map(self.await) }.boxed()
//     }
// }

// impl<R, T> ViewMemberOrigin<R> for futures_lite::future::Boxed<T>
// where
//     R: Renderer,
//     T: ViewMember<R> + ViewMemberOrigin<R>,
// {
//     type Origin = T::Origin;
// }

impl<R, T> ViewMember<R> for futures_lite::future::Boxed<T>
where
    R: Renderer,
    T: ViewMember<R> + MaybeSend + 'static,
{
    fn count() -> ViewMemberIndex {
        T::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
        T::unbuild(ctx, view_removed)
    }

    fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
        future_view_member_rebuild(self, ctx, will_rebuild)
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        self.build(ctx, true);
    }
}

impl<R, T> IntoView<R> for futures_lite::future::Boxed<T>
where
    R: Renderer,
    T: IntoView<R> + MaybeSend + 'static,
{
    type View = XFuture<Self>;

    fn into_view(self) -> Self::View {
        XFuture(self)
    }
}
