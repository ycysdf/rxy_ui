use crate::{Context, Renderer, InnerSchemaCtx, SchemaParam, ViewCtx, MaybeSync, MaybeSend};

impl<R, T> SchemaParam<R> for Context<T>
where
    R: Renderer,
    T: MaybeSend + MaybeSync + Clone + 'static,
{
    fn from<const I: usize>(ctx: &mut InnerSchemaCtx<R>) -> Self {
        Context(
            ViewCtx::<R> {
                world: &mut *ctx.world,
                parent: ctx.parent.clone(),
            }
            .context::<T>(),
        )
    }
}
