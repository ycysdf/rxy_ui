use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;

use crate::{FnSchema, IntoView, MaybeSend, Renderer, SchemaFn, SchemaParams, RendererSchemaView};

pub struct IntoViewSchemaFnWrapper<T, M>(pub T, PhantomData<M>);

impl<T, M> Debug for IntoViewSchemaFnWrapper<T, M>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IntoViewSchemaFnWrapper")
            .field(&self.0)
            .finish()
    }
}

impl<T, M> Clone for IntoViewSchemaFnWrapper<T, M>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        IntoViewSchemaFnWrapper(self.0.clone(), Default::default())
    }
}

impl<T, M> PartialEq for IntoViewSchemaFnWrapper<T, M>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T, M> Eq for IntoViewSchemaFnWrapper<T, M> where T: Eq {}

impl<T, M> IntoViewSchemaFnWrapper<T, M> {
    pub fn new(t: T) -> Self {
        IntoViewSchemaFnWrapper::<T, M>(t, Default::default())
    }
}

pub trait SchemaIntoViewFn<R, P = ()>: MaybeSend + 'static
where
    R: Renderer,
{
    type View: IntoView<R>;
    fn call(self, param: P) -> Self::View;
}

impl<R, P, T> SchemaFn<P> for IntoViewSchemaFnWrapper<T, R>
where
    R: Renderer,
    P: MaybeSend + 'static,
    T: SchemaIntoViewFn<R, P>,
{
    type View = T::View;

    fn call(self, param: P) -> Self::View {
        self.0.call(param)
    }
}

impl<R, P, T> SchemaIntoViewFn<R, P> for T
where
    R: Renderer,
    T: SchemaFn<P>,
    T::View: IntoView<R>,
{
    type View = T::View;

    fn call(self, param: P) -> Self::View {
        self.call(param)
    }
}

pub type FnSchemaView<R, F, P = ()> =
    RendererSchemaView<R, FnSchema<IntoViewSchemaFnWrapper<F, R>, P>, (), ()>;

pub fn fn_schema_view<R, F, P>(f: F) -> FnSchemaView<R, F, P>
where
    R: Renderer,
    F: SchemaIntoViewFn<R, P>,
    P: SchemaParams<R>,
{
    RendererSchemaView::new(FnSchema::new(IntoViewSchemaFnWrapper::<F, R>::new(f)))
}
