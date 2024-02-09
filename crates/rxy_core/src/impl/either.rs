use crate::either::{Either, EitherExt};
use crate::mutable_view::{MutableView, MutableViewKey};
use crate::{
    to_mutable, virtual_container, IntoView, XNest, Mapper, Renderer, RendererNodeId,
    RendererWorld, ToMutableWrapper, View, ViewCtx, ViewMember, ViewMemberCtx, ViewMemberIndex,
    ViewMemberOrigin, VirtualContainer,
};
use futures_lite::future::Or;

impl<R, LV, RV> MutableView<R> for Either<LV, RV>
where
    R: Renderer,
    LV: MutableView<R>,
    RV: MutableView<R>,
{
    type Key = Either<LV::Key, RV::Key>;

    fn no_placeholder_when_no_rebuild() -> bool {
        LV::no_placeholder_when_no_rebuild() && RV::no_placeholder_when_no_rebuild()
    }

    fn build(self, ctx: ViewCtx<R>, placeholder_node_id: Option<RendererNodeId<R>>) -> Self::Key {
        match self {
            Either::Left(r) => r.build(ctx, placeholder_node_id).either_left(),
            Either::Right(r) => r.build(ctx, placeholder_node_id).either_right(),
        }
    }

    fn rebuild(
        self,
        ctx: ViewCtx<R>,
        key: Self::Key,
        placeholder_node_id: RendererNodeId<R>,
    ) -> Option<Self::Key> {
        fn change<R: Renderer, K: MutableViewKey<R>, V: MutableView<R>>(
            key: K,
            view: V,
            ctx: ViewCtx<R>,
            state_node_id: RendererNodeId<R>,
        ) -> V::Key {
            key.remove(&mut *ctx.world);
            let new_key = view.build(
                ViewCtx {
                    world: &mut *ctx.world,
                    parent: ctx.parent.clone(),
                },
                Some(state_node_id.clone()),
            );
            new_key.insert_before(&mut *ctx.world, Some(&ctx.parent), Some(&state_node_id));
            // let is_hidden = R::get_is_hidden(&mut *ctx.world, &state_node_id);
            // new_key.set_visibility(&mut *ctx.world, is_hidden, &state_node_id);
            new_key
        }
        match (key, self) {
            (Either::Left(key), Either::Left(view)) => view
                .rebuild(ctx, key, placeholder_node_id)
                .map(Either::Left),
            (Either::Right(key), Either::Right(view)) => view
                .rebuild(ctx, key, placeholder_node_id)
                .map(Either::Right),
            (Either::Left(key), Either::Right(view)) => {
                Some(change(key, view, ctx, placeholder_node_id).either_right())
            }
            (Either::Right(key), Either::Left(view)) => {
                Some(change(key, view, ctx, placeholder_node_id).either_left())
            }
        }
    }
}

impl<LK, RK, R> MutableViewKey<R> for Either<LK, RK>
where
    LK: MutableViewKey<R>,
    RK: MutableViewKey<R>,
    R: Renderer,
{
    fn remove(self, world: &mut RendererWorld<R>) {
        match self {
            Either::Left(l) => l.remove(world),
            Either::Right(r) => r.remove(world),
        }
    }

    fn insert_before(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
    ) {
        match self {
            Either::Left(l) => {
                l.insert_before(world, parent, before_node_id);
            }
            Either::Right(r) => {
                r.insert_before(world, parent, before_node_id);
            }
        }
    }

    fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool) {
        match self {
            Either::Left(l) => {
                l.set_visibility(world, hidden);
            }
            Either::Right(r) => {
                r.set_visibility(world, hidden);
            }
        }
    }

    fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
        match self {
            Either::Left(l) => l.first_node_id(world),
            Either::Right(r) => r.first_node_id(world),
        }
    }

    fn state_node_id(&self) -> Option<RendererNodeId<R>> {
        match self {
            Either::Left(l) => l.state_node_id(),
            Either::Right(r) => r.state_node_id(),
        }
    }
}

impl<R, LV, RV> IntoView<R> for Either<LV, RV>
where
    R: Renderer,
    LV: IntoView<R>,
    RV: IntoView<R>,
{
    type View = VirtualContainer<R, Either<ToMutableWrapper<LV::View>, ToMutableWrapper<RV::View>>>;

    fn into_view(self) -> Self::View {
        virtual_container(
            match self {
                Either::Left(n) => Either::Left(to_mutable(n.into_view())),
                Either::Right(n) => Either::Right(to_mutable(n.into_view())),
            },
            "[Either Placeholder]",
        )
    }
}

// todo:
// impl<R, LVMO, RVMO> XNest<R> for Either<LVMO, RVMO>
// where
//     R: Renderer,
//     LVMO: XNest<R>,
//     RVMO: XNest<R>,
// {
//     type InnerMember = Either<LVMO::InnerMember, RVMO::InnerMember>;
//     type MapMember<M: Mapper<Self>> = Either<LVMO::MapMember<M>, RVMO::MapMember<M>>;
//
//     fn map_inner<U: Mapper<Self>>(self) -> Self::MapMember<U> {
//         match self {
//             Either::Left(n) => Either::Left(U::map(n)),
//             Either::Right(n) => Either::Right(U::map(n)),
//         }
//     }
// }

// impl<R, LVM, RVM> ViewMemberOrigin<R> for Either<LVM, RVM>
// where
//     R: Renderer,
//     LVM: ViewMemberOrigin<R>,
//     RVM: ViewMember<R> + ViewMemberOrigin<R, Origin = LVM::Origin>,
// {
//     type Origin = LVM::Origin;
// }

impl<R, LVM, RVM> ViewMember<R> for Either<LVM, RVM>
where
    R: Renderer,
    LVM: ViewMember<R>,
    RVM: ViewMember<R>,
{
    fn count() -> ViewMemberIndex {
        LVM::count() + LVM::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
        LVM::unbuild(
            ViewMemberCtx {
                index: ctx.index,
                world: &mut *ctx.world,
                node_id: ctx.node_id.clone(),
            },
            view_removed,
        );
        RVM::unbuild(
            ViewMemberCtx {
                index: ctx.index + LVM::count(),
                world: ctx.world,
                node_id: ctx.node_id,
            },
            view_removed,
        );
    }

    fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
        match self {
            Either::Left(l) => l.build(ctx, will_rebuild),
            Either::Right(r) => r.build(ctx, will_rebuild),
        }
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        match self {
            Either::Left(l) => {
                RVM::unbuild(
                    ViewMemberCtx {
                        index: ctx.index + LVM::count(),
                        world: &mut *ctx.world,
                        node_id: ctx.node_id.clone(),
                    },
                    false,
                );
                l.rebuild(ctx);
            }
            Either::Right(r) => {
                LVM::unbuild(
                    ViewMemberCtx {
                        index: ctx.index,
                        world: &mut *ctx.world,
                        node_id: ctx.node_id.clone(),
                    },
                    false,
                );
                r.rebuild(ctx)
            }
        }
    }
}

// impl<L: CloneTo, R: CloneTo> CloneTo for Either<L, R> {
//     type To = Either<L::To, R::To>;
//
//     fn clone_to(&self) -> Self::To {
//         match self {
//             Either::Left(l) => Either::Left(l.clone_to()),
//             Either::Right(r) => Either::Right(r.clone_to()),
//         }
//     }
// }
/*
impl<LK, RK, R> ViewKey<R> for Either<LK, RK>
    where
        LK: ViewKey<R>,
        RK: ViewKey<R>,
        R: Renderer,
{
    fn remove(self, world: &mut RendererWorld<R>) {
        match self {
            Either::Left(l) => l.remove(world),
            Either::Right(r) => r.remove(world),
        }
    }

    fn insert_before(&self, world: &mut RendererWorld<R>, parent: Option<&RendererNodeId<R>>, before_node_id: Option<&RendererNodeId<R>>) {
        match self {
            Either::Left(l) => {
                l.insert_before(world, parent, before_node_id);
            }
            Either::Right(r) => {
                r.insert_before(world, parent, before_node_id);
            }
        }
    }

    fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool) {
        match self {
            Either::Left(l) => {
                l.set_visibility(world, hidden);
            }
            Either::Right(r) => {
                r.set_visibility(world, hidden);
            }
        }
    }

    fn state_node_id(&self) -> Option<RendererNodeId<R>> {
        match self {
            Either::Left(l) => l.state_node_id(),
            Either::Right(r) => r.state_node_id(),
        }
    }

    fn reserve_key(world: &mut RendererWorld<R>, will_rebuild: bool) -> Self {
        Either::Left(LK::reserve_key(world, will_rebuild))
    }

    fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
        match self {
            Either::Left(l) => l.first_node_id(world),
            Either::Right(r) => r.first_node_id(world),
        }
    }
}*/
