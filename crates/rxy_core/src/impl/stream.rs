use core::any::TypeId;
use std::pin::pin;

use crate::renderer::DeferredWorldScoped;
use bevy_utils::futures::now_or_never;
use futures_lite::{Stream, StreamExt};

use crate::{
    build_info::{node_build_status, node_build_times_increment},
    into_view, mutable_view_rebuild, Either, IntoView, MutableView,
    Renderer, RendererNodeId, TaskState, ToIntoView, View, ViewCtx, ViewKey, ViewMember,
    ViewMemberCtx, ViewMemberExt, ViewMemberIndex,
};

fn stream_vm_rebuild<R, S, VM>(
    x_stream: XStream<S>,
    mut ctx: ViewMemberCtx<R>,
    maybe_already_build: bool,
) where
    R: Renderer,
    S: Stream<Item = VM> + Send + 'static,
    VM: ViewMember<R>,
{
    let default_value = x_stream.value;
    let stream = x_stream.stream;
    if let Some(vm) = default_value {
        let mut ctx = ViewMemberCtx {
            index: ctx.index,
            world: &mut *ctx.world,
            node_id: ctx.node_id.clone(),
        };
        if !maybe_already_build {
            ctx.build_times_increment();
            vm.build(ctx, true);
        } else {
            vm.build_or_rebuild(ctx);
        }
    }
    if x_stream.already_end {
        return;
    }
    let index = ctx.index;
    let node_id = ctx.node_id.clone();
    let world_scoped = R::deferred_world_scoped(ctx.world);

    ctx.set_indexed_view_member_state(TaskState::<R>::new(R::spawn(async move {
        let mut stream = pin!(stream);
        while let Some(vm) = stream.next().await {
            let node_id = node_id.clone();
            world_scoped.scoped(move |world| {
                if R::exist_node_id(world, &node_id) {
                    return;
                }
                vm.build_or_rebuild(ViewMemberCtx {
                    index,
                    world,
                    node_id,
                });
            });
        }
    })));
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
        stream_vm_rebuild(x_stream_immediate(self), ctx, true);
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        stream_vm_rebuild(x_stream_immediate(self), ctx, false);
    }
}

pub fn stream_view_rebuild<R, S>(
    stream: S,
    key: StreamViewKey<R, S>,
    ctx: ViewCtx<R>,
) -> StreamViewKey<R, S>
where
    S: Stream + Send + 'static,
    R: Renderer,
    S::Item: IntoView<R>,
{
    let Some(state_node_id) = key.state_node_id() else {
        return key;
    };

    let world_scoped = R::deferred_world_scoped(ctx.world);

    R::ensure_spawn(ctx.world, state_node_id.clone());
    let task = R::spawn({
        let state_node_id = state_node_id.clone();
        let parent = ctx.parent;
        let key = key.clone();
        async move {
            let mut stream = pin!(stream);
            while let Some(view) = stream.next().await {
                let view = view.into_view();
                let key = key.clone();
                let parent = parent.clone();
                let state_node_id = state_node_id.clone();
                world_scoped.scoped(move |world| {
                    if !R::exist_node_id(world, &parent) {
                        return;
                    }

                    let is_build = !node_build_status::<R>(world, &state_node_id).is_no_build();
                    if !is_build {
                        view.build(ViewCtx { world, parent }, Some(key), true);
                    } else {
                        view.rebuild(ViewCtx { world, parent }, key);
                    }
                    node_build_times_increment::<R>(world, state_node_id.clone());
                })
            }
        }
    });
    R::set_node_state(
        ctx.world,
        &state_node_id,
        XStreamState(TaskState::<R>::new(task)),
    );

    key
}

pub type StreamViewKey<R, S> = <<<S as Stream>::Item as IntoView<R>>::View as View<R>>::Key;

#[derive(Clone, Debug)]
pub struct XStreamState<T>(T);

impl<R, S> View<R> for XStream<S>
where
    R: Renderer,
    S: Stream + Send + 'static,
    S::Item: IntoView<R> + Send,
{
    type Key = StreamViewKey<R, S>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        let stream = self.stream;
        let default_value = self.value;
        let key = if let Some(view) = default_value {
            let view = view.into_view();
            let key = view.build(
                ViewCtx {
                    world: &mut *ctx.world,
                    parent: ctx.parent.clone(),
                },
                reserve_key,
                true,
            );
            if let Some(state_node_id) = key.state_node_id() {
                node_build_times_increment::<R>(ctx.world, state_node_id);
            }
            Some(key)
        } else {
            None
        };
        let key = key.unwrap_or_else(|| Self::Key::reserve_key(&mut *ctx.world, will_rebuild));

        if self.already_end {
            return key;
        }
        stream_view_rebuild(stream, key, ctx)
    }

    fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key) {
        let Some(state_node_id) = key.state_node_id() else {
            return;
        };
        drop(R::take_node_state::<XStreamState<R>>(ctx.world, &state_node_id));

        let stream = self.stream;
        let default_value = self.value;
        if let Some(view) = default_value {
            let view = view.into_view();
            view.rebuild(
                ViewCtx {
                    world: &mut *ctx.world,
                    parent: ctx.parent.clone(),
                },
                key.clone(),
            );
            if let Some(state_node_id) = key.state_node_id() {
                node_build_times_increment::<R>(ctx.world, state_node_id);
            }
        }
        if self.already_end {
            return;
        }
        stream_view_rebuild(stream, key, ctx);
    }
}

impl<R, IV> IntoView<R> for futures_lite::stream::Boxed<IV>
where
    R: Renderer,
    IV: IntoView<R> + Send,
{
    type View = XStream<futures_lite::stream::Boxed<IV>>;

    fn into_view(self) -> Self::View {
        x_stream_immediate(self)
    }
}

impl<S, F, R, IV> IntoView<R> for futures_lite::stream::Map<S, F>
where
    IV: IntoView<R> + Send,
    R: Renderer,
    S: Stream + Send + 'static,
    F: FnMut(S::Item) -> IV + Send + 'static,
{
    type View = XStream<Self>;

    fn into_view(self) -> Self::View {
        x_stream(self)
    }
}

pub struct XStream<T>
where
    T: Stream,
{
    pub stream: T,
    value: Option<T::Item>,
    already_end: bool,
}

impl<T> XStream<T>
where
    T: Stream,
{
    pub fn with_value(self, value: T::Item) -> Self {
        Self {
            stream: self.stream,
            value: Some(value),
            already_end: self.already_end,
        }
    }
}

impl<R, S> IntoView<R> for XStream<S>
where
    R: Renderer,
    S: Stream + Send + 'static,
    S::Item: IntoView<R> + Send,
{
    type View = Self;

    fn into_view(self) -> Self::View {
        self
    }
}

#[inline(always)]
pub fn x_stream<S>(stream: S) -> XStream<S>
where
    S: Stream + Send + 'static,
{
    XStream {
        stream: stream,
        value: None,
        already_end: false,
    }
}
#[inline(always)]
pub fn x_stream_immediate<S>(mut stream: S) -> XStream<S>
where
    S: Stream + Unpin + Send + 'static,
{
    if let Some(value) = now_or_never(stream.next()) {
        if value.is_some() {
            XStream {
                stream,
                value,
                already_end: false,
            }
        } else {
            XStream {
                stream,
                value,
                already_end: true,
            }
        }
    } else {
        XStream {
            stream,
            value: None,
            already_end: false,
        }
    }
}

impl<R, S> ViewMember<R> for XStream<S>
where
    R: Renderer,
    S: Stream + Send + 'static,
    S::Item: ViewMember<R>,
{
    fn count() -> ViewMemberIndex {
        S::Item::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
        S::Item::unbuild(ctx, view_removed)
    }

    fn build(self, ctx: ViewMemberCtx<R>, _will_rebuild: bool) {
        stream_vm_rebuild(self, ctx, true);
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        stream_vm_rebuild(self, ctx, false);
    }
}
