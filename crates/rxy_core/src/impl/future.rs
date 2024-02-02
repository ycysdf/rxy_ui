use crate::build_info::{node_build_status, node_build_times_increment};
use crate::renderer::DeferredWorldScoped;
use crate::{
    BuildState, IntoView, Renderer, View, ViewCtx, ViewKey, ViewMember, ViewMemberCtx,
    ViewMemberIndex,
};
use bevy_utils::futures::now_or_never;
use core::any::TypeId;
use core::future::{Future, IntoFuture};
use futures_lite::StreamExt;

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
    T: Future + Send + 'static,
    T::Output: IntoView<R> + Send + 'static,
{
    type Key = FutureViewKey<R, T>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        _reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        let key = FutureViewKey::<R, T>::reserve_key(&mut *ctx.world, will_rebuild);

        future_view_build(self.0, ctx, will_rebuild, key.clone());
        key
    }

    fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key) {
        let Some(state_node_id) = key.state_node_id() else {
            return;
        };
        if !node_build_status::<R>(ctx.world, &state_node_id).is_no_build() {
            let world_scoped = R::deferred_world_scoped(ctx.world);
            R::spawn_and_detach(async move {
                let view = self.0.await;
                world_scoped.scoped(|world| {
                    view.into_view().rebuild(
                        ViewCtx {
                            world,
                            parent: ctx.parent,
                        },
                        key,
                    );
                });
            });
        } else {
            future_view_build(self.0, ctx, true, key.clone());
        }
    }
}

fn future_view_build<R, T>(
    future: T,
    ctx: ViewCtx<R>,
    will_rebuild: bool,
    reserve_key: FutureViewKey<R, T>,
) where
    R: Renderer,
    T: Future + Send + 'static,
    T::Output: IntoView<R> + Send + 'static,
{
    let world_scoped = R::deferred_world_scoped(ctx.world);

    R::spawn_and_detach(async move {
        let view = future.await;
        world_scoped.scoped(move |world| {
            let Some(state_node_id) = reserve_key.state_node_id() else {
                return;
            };
            if !node_build_status::<R>(world, &state_node_id).is_no_build() {
                return;
            }
            // todo: check view is removed
            let key = view.into_view().build(
                ViewCtx {
                    world,
                    parent: ctx.parent.clone(),
                },
                Some(reserve_key),
                true,
            );
            if will_rebuild {
                if let Some(state_node_id) = key.state_node_id() {
                    node_build_times_increment::<R>(world, state_node_id);
                }
            }
        });
    });
}

impl<R, T> IntoView<R> for XFuture<T>
where
    R: Renderer,
    T: Future + Send + 'static,
    T::Output: IntoView<R> + Send + 'static,
{
    type View = Self;

    fn into_view(self) -> Self::View {
        self
    }
}

pub fn future_view_member_build<R, T>(future: T, ctx: ViewMemberCtx<R>, will_rebuild: bool)
where
    R: Renderer,
    T: Future + Send + 'static,
    T::Output: ViewMember<R> + Send + 'static,
{
    let world_scoped = R::deferred_world_scoped(ctx.world);

    R::spawn_and_detach(async move {
        let view_member = future.await;
        world_scoped.scoped(move |world| {
            let mut ctx = ViewMemberCtx::<R> {
                index: ctx.index,
                world,
                node_id: ctx.node_id,
            };

            if !ctx.build_status().is_no_build() {
                return;
            }
            view_member.build(
                ViewMemberCtx {
                    index: ctx.index,
                    world: &mut *ctx.world,
                    node_id: ctx.node_id.clone(),
                },
                will_rebuild,
            );
            if will_rebuild {
                ctx.build_times_increment();
            }
        });
    });
}

impl<R, T> ViewMember<R> for XFuture<T>
where
    R: Renderer,
    T: Future + Send + 'static,
    T::Output: ViewMember<R> + Send + 'static,
{
    fn count() -> ViewMemberIndex {
        T::Output::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
        T::Output::unbuild(ctx, view_removed)
    }

    fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
        future_view_member_build(self.0, ctx, will_rebuild)
    }

    fn rebuild(self, mut ctx: ViewMemberCtx<R>) {
        if !ctx.build_status().is_no_build() {
            let world_scoped = R::deferred_world_scoped(ctx.world);
            R::spawn_and_detach(async move {
                let view_member = self.0.await;
                world_scoped.scoped(move |world| {
                    view_member.rebuild(ViewMemberCtx {
                        index: ctx.index,
                        world,
                        node_id: ctx.node_id,
                    });
                });
            });
        } else {
            future_view_member_build(self.0, ctx, true)
        }
    }
}

impl<R, T> IntoView<R> for futures_lite::future::Boxed<T>
where
    R: Renderer,
    T: IntoView<R> + Send + 'static,
{
    type View = XFuture<Self>;

    fn into_view(self) -> Self::View {
        XFuture(self)
    }
}
