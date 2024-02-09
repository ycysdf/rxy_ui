use alloc::boxed::Box;
use core::cell::UnsafeCell;
use core::marker::PhantomData;
use xy_reactive::effect::ErasureEffect;

use xy_reactive::prelude::{create_render_effect, use_memo, Memo, ReadSignal, RwSignal, SignalGet};
use xy_reactive::render_effect::RenderEffect;

use crate::{
    DeferredNodeTreeScoped, InnerIvmToVm, IntoView, XNest,
    MaybeSend, MaybeSync, MemberOwner, NodeTree, Renderer, RendererNodeId, RendererWorld, View,
    ViewCtx, ViewKey, ViewMember, ViewMemberCtx, ViewMemberIndex, ViewMemberOrigin,
};

struct FnOnceCell<'a, I, T> {
    func: Option<Box<dyn FnOnce(I) -> T + 'a>>,
    _marker: PhantomData<I>,
}

impl<'a, I, T> FnOnceCell<'a, I, T> {
    fn new(func: impl FnOnce(I) -> T + 'a) -> Self {
        FnOnceCell {
            func: Some(Box::new(func)),
            _marker: PhantomData,
        }
    }

    fn call(&mut self, input: I) -> T {
        self.func.take().expect("Function called after moving")(input)
    }
}

#[derive(Clone)]
pub struct Reactive<F, T, M = ()>(pub F, pub PhantomData<(T, M)>)
where
    F: Fn() -> T + MaybeSend + 'static;

pub fn rx<F, T>(f: F) -> Reactive<F, T>
where
    F: Fn() -> T + MaybeSend + 'static,
{
    Reactive(f, Default::default())
}

pub fn create_effect_with_init<T: Clone + 'static, I: 'static>(
    input: impl Fn() -> I + MaybeSend + 'static,
    init: impl FnOnce(I) -> T,
    f: impl Fn(I, T) + 'static,
) -> RenderEffect<T> {
    let init_wrapper: FnOnceCell<'static, I, T> =
        unsafe { core::mem::transmute(FnOnceCell::new(init)) };
    let init_wrapper = UnsafeCell::new(init_wrapper);

    unsafe {
        create_render_effect(move |r: Option<T>| {
            if let Some(r) = r {
                f(input(), r.clone());
                r
            } else {
                let init = &mut *init_wrapper.get();
                init.call(input())
            }
        })
    }
}

impl<R, F, VM> ViewMemberOrigin<R> for Reactive<F, VM>
where
    R: Renderer,
    F: Fn() -> VM + MaybeSend + 'static,
    VM: ViewMemberOrigin<R>,
{
    type Origin = VM::Origin;
}

impl<R, F, VM> ViewMember<R> for Reactive<F, VM>
where
    R: Renderer,
    F: Fn() -> VM + MaybeSend + 'static,
    VM: ViewMember<R> + MaybeSend,
{
    fn count() -> ViewMemberIndex {
        VM::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
        VM::unbuild(ctx, view_removed);
    }

    fn build(self, mut ctx: ViewMemberCtx<R>, _will_rebuild: bool) {
        let index = ctx.index;
        let node_id = ctx.node_id.clone();
        let deferred_world_scoped = ctx.world.deferred_world_scoped();
        let _effect = create_effect_with_init(
            self.0,
            |vm: VM| {
                vm.build(
                    ViewMemberCtx {
                        index,
                        world: &mut *ctx.world,
                        node_id: ctx.node_id.clone(),
                    },
                    true,
                )
            },
            move |member: VM, _| {
                let node_id = node_id.clone();
                deferred_world_scoped.scoped(move |world| {
                    if !world.exist_node_id(&node_id) {
                        return;
                    }
                    let ctx = ViewMemberCtx {
                        index,
                        world,
                        node_id,
                    };
                    member.rebuild(ctx);
                });
            },
        );

        ctx.set_indexed_view_member_state(ReactiveDisposerState(_effect.erase()))
    }

    fn rebuild(self, mut ctx: ViewMemberCtx<R>) {
        drop(ctx.take_indexed_view_member_state::<ReactiveDisposerState>());

        let deferred_world_scoped = ctx.world.deferred_world_scoped();
        let index = ctx.index;
        let node_id = ctx.node_id.clone();
        let _effect = create_render_effect(move |_| {
            let vm = self.0();
            let node_id = node_id.clone();
            deferred_world_scoped.scoped(move |world| {
                if !world.exist_node_id(&node_id) {
                    return;
                }
                let ctx: ViewMemberCtx<'_, R> = ViewMemberCtx {
                    index,
                    world,
                    node_id,
                };
                vm.rebuild(ctx);
            });
        });

        ctx.set_indexed_view_member_state(ReactiveDisposerState(_effect.erase()))
    }
}

macro_rules! impl_view_member_for_signal_get {
    ($ident:ident) => {
        impl<R, VM> ViewMemberOrigin<R> for $ident<VM>
        where
            R: Renderer,
            VM: ViewMemberOrigin<R> + MaybeSync + Clone,
        {
            type Origin = VM::Origin;
        }

        impl<R, VM> ViewMember<R> for $ident<VM>
        where
            R: Renderer,
            VM: ViewMember<R> + MaybeSync + Clone,
        {
            fn count() -> ViewMemberIndex {
                VM::count()
            }

            fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
                VM::unbuild(ctx, view_removed);
            }

            fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
                rx(move || self.get()).build(ctx, will_rebuild);
            }

            fn rebuild(self, ctx: ViewMemberCtx<R>) {
                rx(move || self.get()).rebuild(ctx);
            }
        }
    };
}

impl_view_member_for_signal_get!(Memo);
impl_view_member_for_signal_get!(ReadSignal);
impl_view_member_for_signal_get!(RwSignal);

#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(type_path = false)
)]
#[derive(Clone, Debug)]
pub struct ReactiveViewKey<R, K>
where
    R: Renderer,
    K: ViewKey<R>,
{
    key: K,
    disposer_state_node_id: RendererNodeId<R>,
}

#[cfg(feature = "bevy_reflect")]
impl<R, K> bevy_reflect::TypePath for ReactiveViewKey<R, K>
where
    R: Renderer,
    K: ViewKey<R>,
{
    fn type_path() -> &'static str {
        "rxy_core::ReactiveViewKey<R,K>"
    }

    fn short_type_path() -> &'static str {
        "ReactiveViewKey<R,K>"
    }
}

pub struct ReactiveDisposerState(pub ErasureEffect);

impl<R, K> ViewKey<R> for ReactiveViewKey<R, K>
where
    R: Renderer,
    K: ViewKey<R>,
{
    fn remove(self, world: &mut RendererWorld<R>) {
        let state = world
            .take_node_state::<ReactiveDisposerState>(&self.disposer_state_node_id)
            .unwrap();
        drop(state);
        world.remove_node(&self.disposer_state_node_id);
        self.key.remove(world);
    }

    fn insert_before(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
    ) {
        self.key.insert_before(world, parent, before_node_id);
    }

    fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool) {
        self.key.set_visibility(world, hidden);
    }

    fn state_node_id(&self) -> Option<RendererNodeId<R>> {
        self.key.state_node_id()
    }

    fn reserve_key(world: &mut RendererWorld<R>, will_rebuild: bool) -> Self {
        let key = K::reserve_key(world, will_rebuild);
        Self {
            key,
            disposer_state_node_id: world.spawn_data_node(),
        }
    }

    fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
        self.key.first_node_id(world)
    }
}

impl<R, F, IV> View<R> for Reactive<F, IV>
where
    R: Renderer,
    F: Fn() -> IV + MaybeSend + 'static,
    IV: IntoView<R> + MaybeSend,
{
    type Key = ReactiveViewKey<R, <IV::View as View<R>>::Key>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        _will_rebuild: bool,
    ) -> Self::Key {
        let (reserve_key, reserve_disposer) = reserve_key
            .map(|n| (n.key, n.disposer_state_node_id))
            .unzip();
        let world_scoped = ctx.world.deferred_world_scoped();
        let parent = ctx.parent.clone();
        let _effect = create_effect_with_init(
            self.0,
            |f: IV| {
                f.into_view().build(
                    ViewCtx {
                        world: &mut *ctx.world,
                        parent: ctx.parent,
                    },
                    reserve_key,
                    true,
                )
            },
            move |f: IV, key| {
                let view = f.into_view();
                let parent = parent.clone();
                world_scoped.scoped(move |world| {
                    view.rebuild(ViewCtx { world, parent }, key);
                });
            },
        );
        let view_key = _effect.with_value_mut(|n| n.clone()).unwrap();
        // todo: no save disposer_state_node_id ? , no handle empty view
        let disposer_state_node_id = reserve_disposer.unwrap_or_else(|| {
            view_key
                .state_node_id()
                .unwrap_or_else(|| ctx.world.spawn_data_node())
        });
        ctx.world.set_node_state::<ReactiveDisposerState>(
            &disposer_state_node_id,
            ReactiveDisposerState(_effect.erase()),
        );

        ReactiveViewKey {
            key: view_key,
            disposer_state_node_id,
        }
    }

    fn rebuild(
        self,
        ctx: ViewCtx<R>,
        ReactiveViewKey {
            key,
            disposer_state_node_id,
        }: Self::Key,
    ) {
        drop(
            ctx.world
                .take_node_state::<ReactiveDisposerState>(&disposer_state_node_id),
        );

        let world_scoped = ctx.world.deferred_world_scoped();
        let parent = ctx.parent.clone();

        let _effect = create_effect_with_init(
            self.0,
            |f: IV| {
                f.into_view().rebuild(
                    ViewCtx {
                        world: &mut *ctx.world,
                        parent: ctx.parent.clone(),
                    },
                    key.clone(),
                );
                key
            },
            move |f: IV, key| {
                let view = f.into_view();
                let parent = parent.clone();
                world_scoped.scoped(move |world| {
                    view.rebuild(ViewCtx { world, parent }, key);
                });
            },
        );
        ctx.world.set_node_state::<ReactiveDisposerState>(
            &disposer_state_node_id,
            ReactiveDisposerState(_effect.erase()),
        );
    }
}

impl<R, F, IV> IntoView<R> for Reactive<F, IV>
where
    R: Renderer,
    F: Fn() -> IV + MaybeSend + 'static,
    IV: IntoView<R> + MaybeSend,
{
    type View = Reactive<F, IV>;

    fn into_view(self) -> Self::View {
        self
    }
}

// tod: ivm
pub trait MemberOwnerRxExt<R>: MemberOwner<R>
where
    R: Renderer,
{
//     #[inline(always)]
//     fn rx_member<T, VM>(
//         self,
//         f: impl Fn() -> T + MaybeSend + 'static,
//     ) -> Self::AddMember<Reactive<impl Fn() -> VM + MaybeSend + 'static, VM>>
//     where
//         Self: Sized,
//         VM: ViewMember<R>,
//         T: XNest<R, MapMember<VmMapper<R>> = VM>,
//     {
//         //  todo:
//         todo!()
//         // self.member(XNestWrapper(rx(move || f().map_inner::<FlatMapper<R>>())))
//         // self.member(rx(move || f()))
//     }
}

impl<R, T> MemberOwnerRxExt<R> for T
where
    R: Renderer,
    T: MemberOwner<R>,
{
}
