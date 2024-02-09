use core::marker::PhantomData;

use bevy_ecs::prelude::Entity;
use bevy_ecs::system::Resource;

use rxy_core::{
    prelude::{ViewMember, ViewMemberCtx},
    DeferredNodeTreeScoped, InnerIvmToVm, MaybeSend, NodeTree, Renderer, View, ViewCtx, ViewKey,
    ViewMemberIndex, ViewMemberOrigin,
};
use rxy_core::{IntoView, XNest};

use crate::{BevyRenderer, ResChangeWorldExt, TaskState};

pub struct XRes<T, F, V> {
    pub f: F,
    _marker: PhantomData<(T, V)>,
}

fn x_res_view_build<T, F, IV>(
    res: XRes<T, F, IV>,
    key: <IV::View as View<BevyRenderer>>::Key,
    state_node_id: &Entity,
    ctx: ViewCtx<BevyRenderer>,
) where
    T: Resource,
    F: Fn(&T) -> IV + Clone + Send + Sync + 'static,
    IV: IntoView<BevyRenderer> + Send,
{
    let world_scoped = ctx.world.deferred_world_scoped();

    let task = BevyRenderer::spawn_task({
        let res_change_receiver = ctx.world.get_res_change_receiver::<T>();
        let parent = ctx.parent;
        let f = res.f;
        let key = key.clone();
        async move {
            while let Ok(()) = res_change_receiver.recv().await {
                let f = f.clone();
                let key = key.clone();
                world_scoped.scoped(move |world| {
                    let resource = world.resource::<T>();
                    let view = f(resource).into_view();
                    view.rebuild(ViewCtx { world, parent }, key);
                })
            }
        }
    });
    ctx.world
        .set_node_state(state_node_id, TaskState::new(task));
}

impl<T, F, IV> View<BevyRenderer> for XRes<T, F, IV>
where
    T: Resource,
    F: Fn(&T) -> IV + Clone + Send + Sync + 'static,
    IV: IntoView<BevyRenderer> + Send,
{
    type Key = <IV::View as View<BevyRenderer>>::Key;

    fn build(
        self,
        ctx: ViewCtx<BevyRenderer>,
        reserve_key: Option<Self::Key>,
        _will_rebuild: bool,
    ) -> Self::Key {
        let view = (self.f)(ctx.world.resource::<T>()).into_view();
        let key = view.build(
            ViewCtx {
                world: &mut *ctx.world,
                parent: ctx.parent,
            },
            reserve_key,
            true,
        );
        let Some(state_node_id) = key.state_node_id() else {
            return key;
        };
        x_res_view_build(self, key.clone(), &state_node_id, ctx);
        key
    }

    fn rebuild(self, ctx: ViewCtx<BevyRenderer>, key: Self::Key) {
        let view = (self.f)(ctx.world.resource::<T>()).into_view();
        view.rebuild(
            ViewCtx {
                world: &mut *ctx.world,
                parent: ctx.parent,
            },
            key.clone(),
        );

        let Some(state_node_id) = key.state_node_id() else {
            return;
        };
        drop(ctx.world.take_node_state::<TaskState>(&state_node_id));

        x_res_view_build(self, key, &state_node_id, ctx);
    }
}

impl<T, F, IV> IntoView<BevyRenderer> for XRes<T, F, IV>
where
    T: Resource,
    F: Fn(&T) -> IV + Clone + Send + Sync + 'static,
    IV: IntoView<BevyRenderer> + Send,
{
    type View = XRes<T, F, IV>;
    fn into_view(self) -> Self::View {
        self
    }
}

pub fn x_res<T, U, F>(f: F) -> XRes<T, F, U>
where
    F: Fn(&T) -> U + Clone + Send + Sync + 'static,
    T: Resource,
    U: Send + 'static,
{
    XRes {
        f,
        _marker: Default::default(),
    }
}

fn x_res_view_member_build<T, F, VM>(res: XRes<T, F, VM>, mut ctx: ViewMemberCtx<BevyRenderer>)
where
    T: Resource,
    F: Fn(&T) -> VM + Clone + Send + Sync + 'static,
    VM: ViewMember<BevyRenderer>,
{
    let world_scoped = ctx.world.deferred_world_scoped();

    let task = BevyRenderer::spawn_task({
        let ctx = ViewMemberCtx::<BevyRenderer> {
            index: ctx.index,
            world: &mut *ctx.world,
            node_id: ctx.node_id,
        };
        let res_change_receiver = ctx.world.get_res_change_receiver::<T>();
        let f = res.f;
        async move {
            while let Ok(()) = res_change_receiver.recv().await {
                let f = f.clone();
                world_scoped.scoped(move |world| {
                    let resource = world.resource::<T>();
                    let vm = f(resource);
                    vm.rebuild(ViewMemberCtx {
                        index: ctx.index,
                        world,
                        node_id: ctx.node_id,
                    });
                })
            }
        }
    });
    ctx.set_indexed_view_member_state(TaskState::new(task));
}

impl<T, F, VM, IVM, M> ViewMemberOrigin<BevyRenderer> for InnerIvmToVm<XRes<T, F, IVM>, M>
where
    T: Resource,
    F: Fn(&T) -> IVM + Clone + Send + Sync + 'static,
    VM: ViewMember<BevyRenderer> + ViewMemberOrigin<BevyRenderer>,
    IVM: XNest<BevyRenderer, MapInner<M> = VM> + Send + 'static,
    M: MaybeSend + 'static,
{
    type Origin = VM::Origin;
}

impl<T, F, VM, IVM, M> ViewMember<BevyRenderer> for InnerIvmToVm<XRes<T, F, IVM>, M>
where
    T: Resource,
    F: Fn(&T) -> IVM + Clone + Send + Sync + 'static,
    VM: ViewMember<BevyRenderer>,
    IVM: XNest<BevyRenderer, MapInner<M> = VM> + Send + 'static,
    M: MaybeSend + 'static,
{
    fn count() -> ViewMemberIndex {
        VM::count()
    }

    fn unbuild(ctx: ViewMemberCtx<BevyRenderer>, view_removed: bool) {
        VM::unbuild(ctx, view_removed)
    }

    fn build(self, ctx: ViewMemberCtx<BevyRenderer>, will_rebuild: bool) {
        let f = move |resource: &T| (self.0.f)(resource).into_member();
        ViewMember::build(x_res(f), ctx, will_rebuild);
    }

    fn rebuild(self, mut ctx: ViewMemberCtx<BevyRenderer>) {
        let f = move |resource: &T| (self.0.f)(resource).into_member();
        ViewMember::rebuild(x_res(f), ctx);
    }
}

impl<T, F, VM> ViewMemberOrigin<BevyRenderer> for XRes<T, F, VM>
where
    T: Resource,
    F: Fn(&T) -> VM + Clone + Send + Sync + 'static,
    VM: ViewMember<BevyRenderer> + ViewMemberOrigin<BevyRenderer>,
{
    type Origin = VM::Origin;
}

impl<T, F, VM> ViewMember<BevyRenderer> for XRes<T, F, VM>
where
    T: Resource,
    F: Fn(&T) -> VM + Clone + Send + Sync + 'static,
    VM: ViewMember<BevyRenderer>,
{
    fn count() -> ViewMemberIndex {
        VM::count()
    }

    fn unbuild(ctx: ViewMemberCtx<BevyRenderer>, view_removed: bool) {
        VM::unbuild(ctx, view_removed)
    }

    fn build(self, ctx: ViewMemberCtx<BevyRenderer>, will_rebuild: bool) {
        let vm = (self.f)(ctx.world.resource::<T>());
        vm.build(
            ViewMemberCtx {
                index: ctx.index,
                world: &mut *ctx.world,
                node_id: ctx.node_id,
            },
            will_rebuild,
        );
        x_res_view_member_build(self, ctx);
    }

    fn rebuild(self, mut ctx: ViewMemberCtx<BevyRenderer>) {
        let vm = (self.f)(ctx.world.resource::<T>());
        vm.rebuild(ViewMemberCtx {
            index: ctx.index,
            world: &mut *ctx.world,
            node_id: ctx.node_id,
        });
        drop(ctx.take_indexed_view_member_state::<TaskState>());
        x_res_view_member_build(self, ctx);
    }
}
