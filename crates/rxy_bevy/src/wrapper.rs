use crate::BevyRenderer;
use rxy_core::{FnSchema, IntoView, SchemaFn, SchemaParams, SchemaView};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct IntoViewSchemaFnWrapper<T, M = BevyRenderer>(pub T, PhantomData<M>);

impl<T, M> IntoViewSchemaFnWrapper<T, M> {
    pub fn new(t: T) -> Self {
        IntoViewSchemaFnWrapper(t, Default::default())
    }
}

pub trait SchemaIntoViewFn<P = ()>: Send + 'static {
    type View: IntoView<BevyRenderer>;
    fn call(self, param: P) -> Self::View;
}

impl<P, T> SchemaFn<P> for IntoViewSchemaFnWrapper<T>
where
    P: Send + 'static,
    T: SchemaIntoViewFn<P>,
{
    type View = T::View;

    fn call(self, param: P) -> Self::View {
        self.0.call(param)
    }
}

impl<P, T> SchemaIntoViewFn<P> for T
where
    T: SchemaFn<P>,
    T::View: IntoView<BevyRenderer>,
{
    type View = T::View;

    fn call(self, param: P) -> Self::View {
        self.call(param)
    }
}

pub type FnSchemaView<F, P = ()> =
    SchemaView<BevyRenderer, FnSchema<IntoViewSchemaFnWrapper<F>, P>, (), ()>;

pub fn pl_schema_view<F, P>(f: F) -> FnSchemaView<F, P>
where
    F: SchemaIntoViewFn<P>,
    P: SchemaParams<BevyRenderer>,
{
    SchemaView::new(FnSchema::new(IntoViewSchemaFnWrapper::new(f)))
}
