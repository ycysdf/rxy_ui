use bevy_utils::synccell::SyncCell;
use bevy_utils::{all_tuples, HashMap};

use crate::build_info::BuildStatus;
use crate::{Renderer, ViewMemberCtx, ViewMemberIndex};

pub trait ViewMember<R>: Send + 'static
where
    R: Renderer,
{
    fn count() -> ViewMemberIndex;
    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool);
    fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool);
    fn rebuild(self, ctx: ViewMemberCtx<R>);
}

impl<R> ViewMember<R> for ()
where
    R: Renderer,
{
    fn count() -> ViewMemberIndex {
        0
    }

    fn unbuild(_ctx: ViewMemberCtx<R>, _view_removed: bool) {}

    fn build(self, _ctx: ViewMemberCtx<R>, _will_rebuild: bool) {}
    fn rebuild(self, _ctx: ViewMemberCtx<R>) {}
}

impl<R, T, F> ViewMember<R> for F
where
    F: Fn() -> T + Send + 'static,
    R: Renderer,
    T: ViewMember<R>,
{
    fn count() -> ViewMemberIndex {
        T::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
        T::unbuild(ctx, view_removed)
    }

    fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
        self().build(ctx, will_rebuild)
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        self().rebuild(ctx)
    }
}

macro_rules! impl_view_member_for_tuples {
    ($first:ident) => {
        impl_view_member_for_tuples!($first,);
    };
    ($first:ident,$($ty:ident),*$(,)?) => {
        #[allow(unused_assignments)]
        impl<R,$first,$($ty),*> $crate::ViewMember<R> for ($first,$($ty,)*)
		where
            R: $crate::Renderer,
			$first: $crate::ViewMember<R>,
			$($ty: $crate::ViewMember<R>),*
        {
            fn count() -> crate::ViewMemberIndex {
                $first::count() $(+ $ty::count())*
            }

            fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
                let mut index = ctx.index;
                $first::unbuild(ViewMemberCtx{
                    index,
                    world: &mut *ctx.world,
                    node_id: ctx.node_id.clone(),
                }, view_removed);
                index += $first::count();
                $(
                $ty::unbuild(ViewMemberCtx{
                    index,
                    world: &mut *ctx.world,
                    node_id: ctx.node_id.clone(),
                }, view_removed);
                index += $ty::count();
                )*
            }
            fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
                let mut index = ctx.index;
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
                    [<$first:lower>].build(ViewMemberCtx{
                        index,
                        world: &mut *ctx.world,
                        node_id: ctx.node_id.clone(),
                    },will_rebuild);
                    index += $first::count();
                    $(

                    [<$ty:lower>].build(ViewMemberCtx{
                        index,
                        world: &mut *ctx.world,
                        node_id: ctx.node_id.clone(),
                    },will_rebuild);
                    index += $ty::count();

                    )*
				}
            }

            fn rebuild(self, ctx: ViewMemberCtx<R>) {
                let mut index = ctx.index;
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
                    [<$first:lower>].rebuild(ViewMemberCtx{
                        index,
                        world: &mut *ctx.world,
                        node_id: ctx.node_id.clone(),
                    });
                    index += $first::count();
                    $(
                    [<$ty:lower>].rebuild(ViewMemberCtx{
                        index,
                        world: &mut *ctx.world,
                        node_id: ctx.node_id.clone(),
                    });
                    index += $ty::count();
                    )*
				}
            }
        }
    }
}

all_tuples!(impl_view_member_for_tuples, 1, 12, M);

pub struct MemberHashMapState<S: Send + 'static>(pub SyncCell<HashMap<ViewMemberIndex, S>>);

impl<'a, R: Renderer> ViewMemberCtx<'a, R> {
    pub fn indexed_view_member_state_mut<S: Send + 'static>(&mut self) -> Option<&mut S> {
        R::get_state_mut::<MemberHashMapState<S>>(self.world, &self.node_id)
            .and_then(|s| s.0.get().get_mut(&self.index))
    }
    pub fn take_indexed_view_member_state<S: Send + 'static>(&mut self) -> Option<S> {
        R::get_state_mut::<MemberHashMapState<S>>(self.world, &self.node_id)
            .and_then(|s| s.0.get().remove(&self.index))
    }
    pub fn set_indexed_view_member_state<S: Send + 'static>(&mut self, state: S) {
        if let Some(map) =
            R::get_state_mut::<MemberHashMapState<S>>(&mut *self.world, &self.node_id)
        {
            map.0.get().insert(self.index, state);
        } else {
            let mut map = HashMap::default();
            map.insert(self.index, state);
            R::set_state(
                &mut *self.world,
                &self.node_id,
                MemberHashMapState(SyncCell::new(map)),
            );
        }
    }
}

pub trait ViewMemberExt<R>
where
    R: Renderer,
{
    fn build_or_rebuild_by(self, ctx: ViewMemberCtx<R>, build_status: BuildStatus);
    #[inline]
    fn build_or_rebuild(self, mut ctx: ViewMemberCtx<R>)
    where
        Self: Sized,
    {
        let build_status = ctx.build_times_increment();
        self.build_or_rebuild_by(ctx, build_status);
    }
}

impl<R, T> ViewMemberExt<R> for T
where
    R: Renderer,
    T: ViewMember<R>,
{
    #[inline]
    fn build_or_rebuild_by(self, ctx: ViewMemberCtx<R>, build_status: BuildStatus) {
        if build_status.is_no_build() {
            self.build(ctx, true);
        } else {
            self.rebuild(ctx);
        }
    }
}
