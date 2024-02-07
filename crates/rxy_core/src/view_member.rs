use crate::utils::all_tuples;
use crate::{MaybeSend, Renderer, ViewMemberCtx, ViewMemberIndex};
pub trait ViewMemberOrigin<R>: ViewMember<R>
where
    R: Renderer,
{
    type Origin: ViewMember<R>;
}

pub trait ViewMember<R>: MaybeSend + 'static
where
    R: Renderer,
{
    fn count() -> ViewMemberIndex;
    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool);
    fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool);
    fn rebuild(self, ctx: ViewMemberCtx<R>);
}

impl<R> ViewMemberOrigin<R> for ()
where
    R: Renderer,
{
    type Origin = ();
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

/*impl<R, T, F> ViewMember<R> for F
where
    F: Fn() -> T + MaybeSend + 'static,
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
}*/

macro_rules! impl_view_member_for_tuples {
    ($first:ident) => {
        impl_view_member_for_tuples!($first,);
    };
    ($first:ident,$($ty:ident),*$(,)?) => {
        impl<R,$first,$($ty),*> $crate::ViewMemberOrigin<R> for ($first,$($ty,)*)
		where
            R: $crate::Renderer,
			$first: $crate::ViewMemberOrigin<R>,
			$($ty: $crate::ViewMemberOrigin<R,Origin=$first::Origin>),*
        {
            type Origin = $first::Origin;
        }

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

all_tuples!(impl_view_member_for_tuples, 1, 4, M);
