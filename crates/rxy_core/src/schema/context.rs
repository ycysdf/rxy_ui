use crate::{Context, Renderer, InnerSchemaCtx, SchemaParam, ViewCtx};

impl<R, T> SchemaParam<R> for Context<T>
where
    R: Renderer,
    T: Send + Sync + Clone + 'static,
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
