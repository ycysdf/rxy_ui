use crate::{
    ConstIndex, Renderer, Required, InnerSchemaCtx, SchemaParam, SchemaParamDefault, SignalPropState,
};
use alloc::boxed::Box;
use core::any::TypeId;
use xy_reactive::prelude::ReadSignal;

// impl<R, T> SchemaPropParam<R> for ReadSignal<T>
// where
//     R: Renderer,
//     T: Send /* + Default */ + 'static,
// {
//     type Value = T;
// }

impl<R, T> SchemaParam<R> for ReadSignal<T>
where
    R: Renderer,
    T: Send + Sync + SchemaParamDefault<R> + 'static,
{
    fn from<const I: usize>(ctx: &mut InnerSchemaCtx<R>) -> Self {
        use xy_reactive::prelude::use_signal;
        let type_id = TypeId::of::<ConstIndex<I>>();

        let value: T = ctx
            .get_init_value::<T>(type_id)
            .unwrap_or_else(|| T::param_default(ctx));

        let (read_signal, write_signal) = use_signal(value);

        ctx.prop_state()
            .entry(type_id)
            .or_insert_with(|| Box::new(SignalPropState::new(write_signal)));

        read_signal
    }
}

impl<R, T> SchemaParam<R> for Required<ReadSignal<T>>
where
    R: Renderer,
    T: Send+Sync + 'static,
{
    fn from<const I: usize>(ctx: &mut InnerSchemaCtx<R>) -> Self {
        use xy_reactive::prelude::use_signal;

        let type_id = TypeId::of::<ConstIndex<I>>();

        let value: T = ctx.get_init_value::<T>(type_id).unwrap();

        let (read_signal, write_signal) = use_signal(value);

        ctx.prop_state()
            .entry(type_id)
            .or_insert_with(|| Box::new(SignalPropState::new(write_signal)));

        Required(read_signal)
    }
}
