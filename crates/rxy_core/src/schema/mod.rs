use crate::{IntoElementView, IntoView, Renderer, View};
use core::marker::PhantomData;
pub use ctx::*;
pub use element::*;
#[cfg(feature = "async-channel")]
pub use event::*;
pub use param::*;
pub use prop::*;
pub use prop_state::*;
pub use prop_value_wrapper::*;
pub use props::*;
pub use r#fn::*;
pub use required_param::*;
pub use schema_with_element_view_bound::*;
pub use view::*;

mod context;
mod ctx;
mod element;
#[cfg(all(feature = "async-channel", feature = "std"))]
mod event;
mod r#fn;
mod param;
mod prop;
mod prop_state;
mod prop_value_wrapper;
mod props;
mod required_param;
mod schema_with_element_view_bound;
mod slot;
mod view;

pub trait Schema<R: Renderer>: Send + 'static {
    type View: View<R>;
    fn view(self, ctx: InnerSchemaCtx<R, Self>) -> Self::View;
}

impl<R, P, F> Schema<R> for FnElementSchema<F, P>
where
    R: Renderer,
    P: SchemaParams<R>,
    F: SchemaFn<P>,
    F::View: IntoElementView<R>,
{
    type View = <F::View as IntoElementView<R>>::View;

    fn view(self, ctx: InnerSchemaCtx<R, Self>) -> Self::View {
        let mut ctx = ctx.cast();
        self.0.call(P::from(&mut ctx)).into_element_view()
    }
}
impl<R, P, F> Schema<R> for FnSchema<F, P>
where
    R: Renderer,
    P: SchemaParams<R>,
    F: SchemaFn<P>,
    F::View: IntoView<R>,
{
    type View = <F::View as IntoView<R>>::View;

    fn view(self, ctx: InnerSchemaCtx<R, Self>) -> Self::View {
        let mut ctx = ctx.cast();
        self.0.call(P::from(&mut ctx)).into_view()
    }
}

#[derive(Clone)]
pub struct FnSchema<F, P, M = ()>(F, PhantomData<(P, M)>);

pub struct FnElementSchemaMarker;

pub type FnElementSchema<F, P> = FnSchema<F, P, FnElementSchemaMarker>;

impl<F, P, M> FnSchema<F, P, M> {
    pub fn new(f: F) -> Self
    where
        F: SchemaFn<P>,
    {
        FnSchema(f, Default::default())
    }

    pub fn map<MF>(self, f: impl FnOnce(F) -> MF) -> FnSchema<MF, P, M>
    where
        MF: SchemaFn<P>,
    {
        FnSchema(f(self.0), self.1)
    }
}

pub fn schema_view<R, P, F, M>(f: F, _m: M) -> SchemaView<R, FnSchema<F, P>, (), M>
where
    R: Renderer,
    F: SchemaFn<P>,
    P: SchemaParams<R>,
    F::View: IntoView<R>,
    M: Send + 'static,
{
    SchemaView::new(FnSchema::new(f))
}

pub fn element_schema_view<R, P, F, M>(
    f: F,
    _m: M,
) -> ElementSchemaView<R, FnElementSchema<F, P>, (), (), M>
where
    R: Renderer,
    F: SchemaFn<P>,
    P: SchemaParams<R>,
    F::View: IntoElementView<R>,
    M: Send + 'static,
{
    ElementSchemaView::new(FnElementSchema::new(f))
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstIndex<const I: usize, T = ()>(pub T);
