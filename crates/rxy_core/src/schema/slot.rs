use crate::{CloneableSchemaSlot, ConstIndex, Renderer, InnerSchemaCtx, SchemaParam, SchemaSlot};
use core::any::TypeId;

impl<R> SchemaParam<R> for SchemaSlot<R>
where
    R: Renderer,
{
    fn from<const I: usize>(ctx: &mut InnerSchemaCtx<R>) -> Self {
        let prop_type_id = TypeId::of::<ConstIndex<I>>();
        SchemaSlot::new(ctx.slots.remove(&prop_type_id))
    }
}
impl<R> SchemaParam<R> for CloneableSchemaSlot<R>
where
    R: Renderer,
{
    fn from<const I: usize>(ctx: &mut InnerSchemaCtx<R>) -> Self {
        let prop_type_id = TypeId::of::<ConstIndex<I>>();
        CloneableSchemaSlot::new(ctx.cloneable_slots.remove(&prop_type_id))
    }
}
