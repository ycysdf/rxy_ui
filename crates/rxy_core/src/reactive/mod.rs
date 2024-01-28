pub use map::*;
pub use schema_prop::*;
mod map;
mod schema_param;
mod schema_prop;
mod state;

use alloc::boxed::Box;
use core::cell::UnsafeCell;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use xy_reactive::effect::ErasureEffect;

use xy_reactive::prelude::create_render_effect;
use xy_reactive::render_effect::RenderEffect;

use crate::{
    BuildState, DeferredWorldScoped, IntoView, MemberOwner, Renderer, RendererNodeId,
    RendererWorld, View, ViewCtx, ViewKey, ViewMember, ViewMemberCtx, ViewReBuilder,
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

pub struct Reactive<F, T, M = ()>(pub F, pub PhantomData<(T, M)>)
where
    F: Fn() -> T + Send + 'static;

pub fn rx<F, T>(f: F) -> Reactive<F, T>
where
    F: Fn() -> T + Send + 'static,
{
    Reactive(f, Default::default())
}

pub fn create_effect_with_init<T: Clone + 'static, I: 'static>(
    input: impl Fn() -> I + Send + 'static,
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

impl<R, F, VM> ViewMember<R> for Reactive<F, VM>
where
    R: Renderer,
    F: Fn() -> VM + Send + 'static,
    VM: ViewMember<R> + Send,
{
    fn count() -> u8 {
        VM::count()
    }

    fn unbuild(mut ctx: ViewMemberCtx<R>) {
        let _ = ctx.take_view_member_state::<ReactiveDisposerState>();
        VM::unbuild(ctx);
    }

    fn build(self, mut ctx: ViewMemberCtx<R>, _will_rebuild: bool) {
        // todo: when vm_type_id is same
        let type_id = core::any::TypeId::of::<VM>();
        let index = ctx.index;
        let node_id = ctx.node_id.clone();
        let deferred_world_scoped = R::deferred_world_scoped(ctx.world);
        let _effect = create_effect_with_init(
            self.0,
            |vm: VM| {
                vm.build(
                    ViewMemberCtx {
                        index,
                        type_id,
                        world: &mut *ctx.world,
                        node_id: ctx.node_id.clone(),
                    },
                    true,
                )
            },
            move |member: VM, _| {
                let node_id = node_id.clone();
                deferred_world_scoped.deferred_world(move |world| {
                    if !R::exist_node_id(world, &node_id) {
                        return;
                    }
                    let ctx = ViewMemberCtx {
                        index,
                        type_id,
                        world,
                        node_id,
                    };
                    member.rebuild(ctx);
                });
            },
        );

        ctx.set_view_member_state(ReactiveDisposerState(_effect.erase()))
    }

    fn rebuild(self, mut ctx: ViewMemberCtx<R>) {
        drop(ctx.take_view_member_state::<ReactiveDisposerState>());

        let deferred_world_scoped = R::deferred_world_scoped(ctx.world);
        let type_id = core::any::TypeId::of::<VM>();
        let index = ctx.index;
        let node_id = ctx.node_id.clone();
        let _effect = create_render_effect(move |_| {
            let vm = self.0();
            let node_id = node_id.clone();
            deferred_world_scoped.deferred_world(move |world| {
                if !R::exist_node_id(world, &node_id) {
                    return;
                }
                let ctx: ViewMemberCtx<'_, R> = ViewMemberCtx {
                    index,
                    type_id,
                    world,
                    node_id,
                };
                vm.rebuild(ctx);
            });
        });

        ctx.set_view_member_state(ReactiveDisposerState(_effect.erase()))
    }
}

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

impl<R, K> Hash for ReactiveViewKey<R, K>
where
    R: Renderer,
    K: ViewKey<R>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state)
    }
}

pub struct ReactiveDisposerState(pub ErasureEffect);

impl<R, K> ViewKey<R> for ReactiveViewKey<R, K>
where
    R: Renderer,
    K: ViewKey<R>,
{
    fn remove(self, world: &mut RendererWorld<R>) {
        let state =
            R::take_state::<ReactiveDisposerState>(world, &self.disposer_state_node_id).unwrap();
        drop(state);
        R::remove_node(world, &self.disposer_state_node_id);
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
            disposer_state_node_id: R::spawn_data_node(world),
        }
    }

    fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
        self.key.first_node_id(world)
    }
}

impl<R, F, IV> View<R> for Reactive<F, IV>
where
    R: Renderer,
    F: Fn() -> IV + Send + 'static,
    IV: IntoView<R> + Send,
{
    type Key = ReactiveViewKey<R, <IV::View as View<R>>::Key>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        _will_rebuild: bool,
    ) -> Self::Key {
        let view_re_builder = R::get_view_re_builder(ViewCtx {
            world: &mut *ctx.world,
            parent: ctx.parent.clone(),
        });
        let f = self.0;
        let (reserve_key, reserve_disposer) = reserve_key
            .map(|n| (n.key, n.disposer_state_node_id))
            .unzip();
        let _effect = create_effect_with_init(
            f,
            |f: IV| {
                f.into_view().build(
                    ViewCtx {
                        world: &mut *ctx.world,
                        parent: ctx.parent.clone(),
                    },
                    reserve_key,
                    true,
                )
            },
            move |f: IV, key| {
                let view = f.into_view();
                view_re_builder.rebuild(view, BuildState::AlreadyBuild(key))
            },
        );
        let view_key = _effect.with_value_mut(|n| n.clone()).unwrap();
        let disposer_state_node_id = reserve_disposer.unwrap_or_else(|| {
            view_key
                .state_node_id()
                .unwrap_or_else(|| R::spawn_data_node(ctx.world))
        });
        R::set_state::<ReactiveDisposerState>(
            ctx.world,
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
        drop(R::take_state::<ReactiveDisposerState>(
            ctx.world,
            &disposer_state_node_id,
        ));

        let view_re_builder = R::get_view_re_builder(ViewCtx {
            world: &mut *ctx.world,
            parent: ctx.parent.clone(),
        });
        let f = self.0;

        let _effect = create_effect_with_init(
            f,
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
                view_re_builder.rebuild(view, BuildState::AlreadyBuild(key))
            },
        );
        R::set_state::<ReactiveDisposerState>(
            ctx.world,
            &disposer_state_node_id,
            ReactiveDisposerState(_effect.erase()),
        );
    }
}

impl<R, F, IV> IntoView<R> for Reactive<F, IV>
where
    R: Renderer,
    F: Fn() -> IV + Send + 'static,
    IV: IntoView<R> + Send,
{
    type View = Reactive<F, IV>;

    fn into_view(self) -> Self::View {
        self
    }
}

pub trait MemberOwnerRxExt<R>: MemberOwner<R>
where
    R: Renderer,
{
    #[inline(always)]
    fn rx_member<T: ViewMember<R>, F>(self, f: F) -> Self::AddMember<Reactive<F, T>>
    where
        Self: Sized,
        F: Fn() -> T + Send + 'static,
    {
        self.member(rx(f))
    }
}

impl<R, T> MemberOwnerRxExt<R> for T
where
    R: Renderer,
    T: MemberOwner<R>,
{
}
