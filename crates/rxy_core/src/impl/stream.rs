use core::any::TypeId;

use bevy_utils::futures::now_or_never;
use futures_lite::{Stream, StreamExt};

use crate::{
    build_info::{build_info_is_contained, build_times_increment},
    into_view, mutable_view_rebuild, BuildState, IntoView, MemberReBuilder, MutableView, Renderer,
    RendererNodeId, ToIntoView, View, ViewCtx, ViewKey, ViewMember, ViewMemberCtx, ViewMemberIndex,
    ViewReBuilder,
};

pub struct LazyViewMemberState {
    pub already_build: bool,
}

fn stream_vm_rebuild<R, S, VM>(mut stream: S, mut ctx: ViewMemberCtx<R>, mut is_already_build: bool)
where
    R: Renderer,
    S: Stream<Item = VM> + Unpin + Send + 'static,
    VM: ViewMember<R>,
{
    if let Some(option) = now_or_never(stream.next()) {
        let Some(vm) = option else {
            return;
        };
        let ctx = ViewMemberCtx {
            index: ctx.index,
            type_id: TypeId::of::<VM>(),
            world: &mut *ctx.world,
            node_id: ctx.node_id.clone(),
        };
        if is_already_build {
            vm.rebuild(ctx)
        } else {
            vm.build(ctx, true);
            is_already_build = true;
        }
    }
    if !is_already_build {
        ctx.set_view_member_state(LazyViewMemberState {
            already_build: false,
        });
    }
    let index = ctx.index;
    let re_builder = R::get_member_re_builder(ctx, is_already_build);

    R::spawn_and_detach(async move {
        while let Some(vm) = stream.next().await {
            re_builder.rebuild(vm, index);
        }
    })
}

impl<R, VM> ViewMember<R> for futures_lite::stream::Boxed<VM>
where
    R: Renderer,
    VM: ViewMember<R>,
{
    fn count() -> ViewMemberIndex {
        VM::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
        VM::unbuild(ctx, view_removed)
    }

    fn build(self, ctx: ViewMemberCtx<R>, _will_rebuild: bool) {
        stream_vm_rebuild(self, ctx, false);
    }

    fn rebuild(self, mut ctx: ViewMemberCtx<R>) {
        let already_build = {
            ctx.view_member_state_mut::<LazyViewMemberState>()
                .map(|n| n.already_build)
                .unwrap_or_else(|| true)
        };
        stream_vm_rebuild(self, ctx, already_build);
    }
}

pub fn stream_view_rebuild<R, S>(
    mut stream: S,
    key: StreamViewKey<R, S>,
    mut build_state: BuildState<StreamViewKey<R, S>>,
    ctx: ViewCtx<R>,
) where
    S: Stream + Unpin + Send + 'static,
    R: Renderer,
    S::Item: IntoView<R>,
{
    let re_builder = R::get_view_re_builder(ctx);

    R::spawn_and_detach({
        let key = key.clone();
        async move {
            while let Some(view) = stream.next().await {
                re_builder.rebuild(view.into_view(), build_state);
                build_state = BuildState::AlreadyBuild(key.clone());
            }
        }
    });
}

pub type StreamViewKey<R, S> = <<<S as Stream>::Item as IntoView<R>>::View as View<R>>::Key;

impl<R, S> View<R> for XStream<S>
where
    R: Renderer,
    S: Stream + Send + Unpin + 'static,
    S::Item: IntoView<R>,
{
    type Key = StreamViewKey<R, S>;

    fn build(
        mut self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        let key = now_or_never(self.0.next()).and_then(|option| {
            option.map(|view| {
                let key = view.into_view().build(
                    ViewCtx {
                        world: &mut *ctx.world,
                        parent: ctx.parent.clone(),
                    },
                    reserve_key,
                    true,
                );
                if let Some(state_node_id) = key.state_node_id() {
                    build_times_increment::<R>(&mut *ctx.world, state_node_id);
                }
                key
            })
        });
        let (build_state, key) = match key {
            None => {
                let key = Self::Key::reserve_key(&mut *ctx.world, will_rebuild);
                (BuildState::NoBuildWithReserveKey(key.clone()), key)
            }
            Some(key) => (BuildState::AlreadyBuild(key.clone()), key),
        };
        stream_view_rebuild(self.0, key.clone(), build_state, ctx);

        key
    }

    fn rebuild(mut self, ctx: ViewCtx<R>, o_key: Self::Key) {
        let Some(state_node_id) = o_key.state_node_id() else {
            return;
        };

        let is_build = build_info_is_contained::<R>(ctx.world, &state_node_id);

        let r_key = now_or_never(self.0.next()).and_then(|option| {
            option.map(|view| {
                let view = view.into_view();
                if is_build {
                    view.rebuild(
                        ViewCtx {
                            world: &mut *ctx.world,
                            parent: ctx.parent.clone(),
                        },
                        o_key.clone(),
                    );
                    o_key.clone()
                } else {
                    view.build(
                        ViewCtx {
                            world: &mut *ctx.world,
                            parent: ctx.parent.clone(),
                        },
                        Some(o_key.clone()),
                        true,
                    )
                }
            })
        });
        let (build_state, key) = match r_key {
            None => {
                if is_build {
                    (BuildState::AlreadyBuild(o_key.clone()), o_key)
                } else {
                    (BuildState::NoBuildWithReserveKey(o_key.clone()), o_key)
                }
            }
            Some(key) => (BuildState::AlreadyBuild(key), o_key),
        };

        stream_view_rebuild(self.0, key.clone(), build_state, ctx);
    }
}

impl<R, IV> IntoView<R> for futures_lite::stream::Boxed<IV>
where
    R: Renderer,
    IV: IntoView<R>,
{
    type View = XStream<futures_lite::stream::Boxed<IV>>;

    fn into_view(self) -> Self::View {
        XStream(self)
    }
}

impl<S, F, R, IV> IntoView<R> for futures_lite::stream::Map<S, F>
where
    IV: IntoView<R>,
    R: Renderer,
    S: Stream + Unpin + Send + 'static,
    F: FnMut(S::Item) -> IV + Send + 'static,
{
    type View = XStream<Self>;

    fn into_view(self) -> Self::View {
        XStream(self)
    }
}

pub struct XStream<T>(pub T);

impl<R, S> IntoView<R> for XStream<S>
where
    R: Renderer,
    S: Stream + Unpin + Send + 'static,
    S::Item: IntoView<R>,
{
    type View = Self;

    fn into_view(self) -> Self::View {
        self
    }
}

#[inline(always)]
pub fn x_stream<S>(stream: S) -> XStream<S>
where
    S: Stream + Unpin + Send + 'static,
{
    XStream(stream)
}

impl<R, S> ViewMember<R> for XStream<S>
where
    R: Renderer,
    S: Stream + Unpin + Send + 'static,
    S::Item: ViewMember<R>,
{
    fn count() -> ViewMemberIndex {
        S::Item::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
        S::Item::unbuild(ctx, view_removed)
    }

    fn build(self, ctx: ViewMemberCtx<R>, _will_rebuild: bool) {
        stream_vm_rebuild(self.0, ctx, false);
    }

    fn rebuild(self, mut ctx: ViewMemberCtx<R>) {
        let already_build = {
            ctx.view_member_state_mut::<LazyViewMemberState>()
                .map(|n| n.already_build)
                .unwrap_or_else(|| true)
        };
        stream_vm_rebuild(self.0, ctx, already_build);
    }
}

// fn stream_mutable_view_rebuild<R: Renderer, V: MutableView<R>>(
//     mut stream: futures_lite::stream::Boxed<V>,
//     ctx: ViewCtx<R>,
//     state_node_id: R::NodeId,
// ) {
//     if let Some(option) = now_or_never(stream.next()) {
//         let Some(v) = option else {
//             return;
//         };
//         mutable_view_rebuild(
//             v,
//             ViewCtx {
//                 world: &mut *ctx.world,
//                 parent: ctx.parent.clone(),
//             },
//             state_node_id.clone(),
//         );
//     }
//
//     let mut re_builder = R::get_view_re_builder(ctx);
//
//     R::spawn_and_detach({
//         async move {
//             while let Some(v) = stream.next().await {
//                 re_builder.mutable_rebuild(v, &state_node_id);
//             }
//         }
//     });
// }
// impl<R, V> MutableView<R> for futures_lite::stream::Boxed<V>
//     where
//         R: Renderer,
//         V: MutableView<R>,
// {
//     type Key = ();
//
//     fn build(
//         self,
//         ctx: ViewCtx<R>,
//         _will_rebuild: bool,
//         state_node_id: RendererNodeId<R>,
//     ) -> Self::Key {
//         stream_mutable_view_rebuild(self, ctx, state_node_id);
//     }
//
//     fn rebuild(
//         self,
//         ctx: ViewCtx<R>,
//         key: Self::Key,
//         state_node_id: RendererNodeId<R>,
//     ) -> Option<Self::Key> {
//         stream_mutable_view_rebuild(self, ctx, state_node_id);
//         Some(key)
//     }
// }
