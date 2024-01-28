use crate::{Either, IntoView, Renderer, View, ViewMember, ViewMemberCtx};

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
impl<R, T, E> ViewMember<R> for Result<T, E>
where
    R: Renderer,
    T: ViewMember<R>,
    E: ViewMember<R>,
{
    fn count() -> u8 {
        T::count() + E::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>) {
        T::unbuild(ViewMemberCtx {
            index: ctx.index,
            type_id: std::any::TypeId::of::<T>(),
            world: &mut *ctx.world,
            node_id: ctx.node_id.clone(),
        });
        E::unbuild(ViewMemberCtx {
            index: ctx.index + T::count(),
            type_id: std::any::TypeId::of::<E>(),
            world: ctx.world,
            node_id: ctx.node_id,
        });
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
