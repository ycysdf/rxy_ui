use crate::{
    Either, IntoView, IntoViewMember, Renderer, View, ViewMember, ViewMemberCtx, ViewMemberIndex,
};

impl<R, T, E> IntoView<R> for Result<T, E>
where
    R: Renderer,
    T: IntoView<R>,
    E: IntoView<R>,
{
    type View = <Either<T, E> as IntoView<R>>::View;

    fn into_view(self) -> Self::View {
        let either: Either<T, E> = self.into();
        IntoView::into_view(either)
    }
}

impl<R, T, E> IntoViewMember<R, Self> for Result<T, E>
where
    R: Renderer,
    T: ViewMember<R>,
    E: ViewMember<R>,
{
    fn into_member(self) -> Self {
        self
    }
}

impl<R, T, E> ViewMember<R> for Result<T, E>
where
    R: Renderer,
    T: ViewMember<R>,
    E: ViewMember<R>,
{
    fn count() -> ViewMemberIndex {
        T::count() + E::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
        T::unbuild(
            ViewMemberCtx {
                index: ctx.index,
                world: &mut *ctx.world,
                node_id: ctx.node_id.clone(),
            },
            view_removed,
        );
        E::unbuild(
            ViewMemberCtx {
                index: ctx.index + T::count(),
                world: ctx.world,
                node_id: ctx.node_id,
            },
            view_removed,
        );
    }

    fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
        let either: Either<T, E> = self.into();
        either.build(ctx, will_rebuild);
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        let either: Either<T, E> = self.into();
        either.rebuild(ctx);
    }
}
