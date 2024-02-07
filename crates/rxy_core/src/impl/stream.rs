use core::any::TypeId;
use core::pin::pin;

use crate::renderer::DeferredNodeTreeScoped;
use crate::utils::now_or_never;
use futures_lite::{Stream, StreamExt};

use crate::{
    build_info::{node_build_status, node_build_times_increment},
    into_view, mutable_view_rebuild, Either, InnerIvmToVm, IntoView, IntoViewMember, MaybeSend,
    MutableView, NodeTree, Renderer, RendererNodeId, TaskState, ToIntoView, View, ViewCtx, ViewKey,
    ViewMember, ViewMemberBuildExt, ViewMemberCtx, ViewMemberIndex, ViewMemberOrigin,
};

fn stream_vm_rebuild<R, S, VM>(
    x_stream: XStream<S>,
    mut ctx: ViewMemberCtx<R>,
    maybe_already_build: bool,
) where
    R: Renderer,
    S: Stream<Item = VM> + MaybeSend + 'static,
    VM: ViewMember<R>,
{
    drop(ctx.take_indexed_view_member_state::<TaskState<R>>());
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
    let world_scoped = ctx.world.deferred_world_scoped();

    ctx.set_indexed_view_member_state(TaskState::<R>::new(R::spawn_task(async move {
        let mut stream = pin!(stream);
        while let Some(vm) = stream.next().await {
            let node_id = node_id.clone();
            world_scoped.scoped(move |world| {
                if world.exist_node_id(&node_id) {
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

impl<R, IVM, VM> IntoViewMember<R, futures_lite::stream::Boxed<VM>>
    for futures_lite::stream::Boxed<IVM>
where
    R: Renderer,
    VM: ViewMember<R>,
    IVM: IntoViewMember<R, VM> + 'static,
{
    // type Origin = VM::Origin;
    fn into_member(self) -> futures_lite::stream::Boxed<VM> {
        self.map(|n| n.into_member()).boxed()
    }
}

impl<R, VM> ViewMemberOrigin<R> for futures_lite::stream::Boxed<VM>
where
    R: Renderer,
    VM: ViewMemberOrigin<R>,
{
    type Origin = VM::Origin;
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
    S: Stream + MaybeSend + 'static,
    R: Renderer,
    S::Item: IntoView<R>,
{
    let Some(state_node_id) = key.state_node_id() else {
        return key;
    };

    let world_scoped = ctx.world.deferred_world_scoped();

    ctx.world.ensure_spawn(state_node_id.clone());
    let task = R::spawn_task({
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
                    if !world.exist_node_id(&parent) {
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
    ctx.world
        .set_node_state(&state_node_id, XStreamState::<R>::new(task));

    key
}

pub type StreamViewKey<R, S> = <<<S as Stream>::Item as IntoView<R>>::View as View<R>>::Key;

pub struct XStreamState<R>(TaskState<R>)
where
    R: Renderer;

impl<R> XStreamState<R>
where
    R: Renderer,
{
    pub fn new(task: R::Task<()>) -> Self {
        Self(TaskState::new(task))
    }
}

impl<R, S> View<R> for XStream<S>
where
    R: Renderer,
    S: Stream + MaybeSend + 'static,
    S::Item: IntoView<R> + MaybeSend,
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
        drop(ctx.world.take_node_state::<XStreamState<R>>(&state_node_id));

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
    IV: IntoView<R> + MaybeSend,
{
    type View = XStream<futures_lite::stream::Boxed<IV>>;

    fn into_view(self) -> Self::View {
        x_stream_immediate(self)
    }
}

impl<S, F, R, IV> IntoView<R> for futures_lite::stream::Map<S, F>
where
    IV: IntoView<R> + MaybeSend,
    R: Renderer,
    S: Stream + MaybeSend + 'static,
    F: FnMut(S::Item) -> IV + MaybeSend + 'static,
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
    pub fn map<F, U>(self, mut f: F) -> XStream<futures_lite::stream::Map<T, F>>
    where
        F: FnMut(T::Item) -> U + MaybeSend + 'static,
    {
        let value = self.value.map(&mut f);
        XStream {
            stream: self.stream.map(f),
            value,
            already_end: self.already_end,
        }
    }
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
    S: Stream + MaybeSend + 'static,
    S::Item: IntoView<R> + MaybeSend,
{
    type View = Self;

    fn into_view(self) -> Self::View {
        self
    }
}

#[inline(always)]
pub fn x_stream<S>(stream: S) -> XStream<S>
where
    S: Stream + MaybeSend + 'static,
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
    S: Stream + Unpin + MaybeSend + 'static,
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

impl<R, S, IVM, VM> ViewMemberOrigin<R> for InnerIvmToVm<XStream<S>, VM>
where
    R: Renderer,
    S: Stream<Item = IVM> + MaybeSend + 'static,
    IVM: IntoViewMember<R, VM> + MaybeSend + 'static,
    VM: ViewMemberOrigin<R>,
{
    type Origin = VM::Origin;
}

impl<R, S, IVM, VM> ViewMember<R> for InnerIvmToVm<XStream<S>, VM>
where
    R: Renderer,
    S: Stream<Item = IVM> + MaybeSend + 'static,
    IVM: IntoViewMember<R, VM> + MaybeSend + 'static,
    VM: ViewMember<R>,
{
    fn count() -> ViewMemberIndex {
        VM::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
        VM::unbuild(ctx, view_removed)
    }

    fn build(self, ctx: ViewMemberCtx<R>, _will_rebuild: bool) {
        stream_vm_rebuild(self.0.map(|n| n.into_member()), ctx, true);
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        stream_vm_rebuild(self.0.map(|n| n.into_member()), ctx, false);
    }
}

impl<R, S, IVM, VM> IntoViewMember<R, InnerIvmToVm<Self, VM>> for XStream<S>
where
    R: Renderer,
    S: Stream<Item = IVM> + MaybeSend + 'static,
    IVM: IntoViewMember<R, VM> + MaybeSend + 'static,
    VM: ViewMember<R>,
{
    fn into_member(self) -> InnerIvmToVm<Self, VM> {
        InnerIvmToVm::new(self)
    }
}
impl<R, S> ViewMemberOrigin<R> for XStream<S>
where
    R: Renderer,
    S: Stream + MaybeSend + 'static,
    S::Item: ViewMemberOrigin<R>,
{
    type Origin = <S::Item as ViewMemberOrigin<R>>::Origin;
}

impl<R, S> ViewMember<R> for XStream<S>
where
    R: Renderer,
    S: Stream + MaybeSend + 'static,
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
