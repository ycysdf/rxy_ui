use std::any::TypeId;
use std::ops::{Deref, DerefMut};

use crate::{CloneableSchemaSlot, ConstIndex, MaybeReflect, RebuildFnReceiver, Renderer, InnerSchemaCtx, SchemaParam, SchemaSlot, Static};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Required<T>(pub T);

impl<T> Required<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for Required<T> {
    fn from(value: T) -> Self {
        Required(value)
    }
}

impl<T> Deref for Required<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Required<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<R, T> SchemaParam<R> for Required<Static<T>>
where
    R: Renderer,
    T: Send + 'static,
{
    fn from<const I: usize>(ctx: &mut InnerSchemaCtx<R>) -> Self {
        let prop_type_id = TypeId::of::<ConstIndex<I>>();
        let value: T = ctx.get_init_value::<T>(prop_type_id)
            .unwrap();
        Required(Static(value))
    }
}

impl<R, T> SchemaParam<R> for Required<RebuildFnReceiver<R, T>>
where
    R: Renderer,
    T: MaybeReflect + Clone + PartialEq + Send + 'static,
{
    fn from<const I: usize>(ctx: &mut InnerSchemaCtx<R>) -> Self {
        Required(<RebuildFnReceiver<R, T> as SchemaParam<R>>::from::<I>(ctx))
    }
}

impl<R> SchemaParam<R> for Required<SchemaSlot<R>>
where
    R: Renderer,
{
    fn from<const I: usize>(ctx: &mut InnerSchemaCtx<R>) -> Self {
        Required(<SchemaSlot<R> as SchemaParam<R>>::from::<I>(ctx))
    }
}

impl<R> SchemaParam<R> for Required<CloneableSchemaSlot<R>>
where
    R: Renderer,
{
    fn from<const I: usize>(ctx: &mut InnerSchemaCtx<R>) -> Self {
        Required(<CloneableSchemaSlot<R> as SchemaParam<R>>::from::<I>(
            ctx,
        ))
    }
}
