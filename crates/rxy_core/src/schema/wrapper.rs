use core::marker::PhantomData;

use crate::{FnSchema, IntoView, MaybeSend, Renderer, SchemaFn, SchemaParams, SchemaView};

#[derive(Clone)]
pub struct IntoViewSchemaFnWrapper<T, M>(pub T, PhantomData<M>);

impl<T, M> IntoViewSchemaFnWrapper<T, M> {
    pub fn new(t: T) -> Self {
        IntoViewSchemaFnWrapper::<T,M>(t, Default::default())
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
    SchemaView<R, FnSchema<IntoViewSchemaFnWrapper<F, R>, P>, (), ()>;

pub fn fn_schema_view<R, F, P>(f: F) -> FnSchemaView<R, F, P>
where
    R: Renderer,
    F: SchemaIntoViewFn<R, P>,
    P: SchemaParams<R>,
{
    SchemaView::new(FnSchema::new(IntoViewSchemaFnWrapper::<F,R>::new(f)))
}
