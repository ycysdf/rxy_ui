use crate::{ElementView, InnerSchemaCtx, MaybeSend, Renderer, Schema};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElementSchemaBoundWrapper<T>(pub T);

pub trait SchemaWithElementViewBound<R: Renderer>: MaybeSend + 'static {
   type View: ElementView<R>;
   fn view(self, ctx: InnerSchemaCtx<R, Self>) -> Self::View;
}

impl<R, T> Schema<R> for ElementSchemaBoundWrapper<T>
where
   R: Renderer,
   T: SchemaWithElementViewBound<R>,
{
   type View = T::View;

   #[inline]
   fn view(self, ctx: InnerSchemaCtx<R, Self>) -> Self::View {
      self.0.view(ctx.cast())
   }
}

impl<R, T> SchemaWithElementViewBound<R> for T
where
   R: Renderer,
   T: Schema<R>,
   T::View: ElementView<R>,
{
   type View = T::View;

   #[inline]
   fn view(self, ctx: InnerSchemaCtx<R, Self>) -> Self::View {
      self.view(ctx)
   }
}
