use crate::r#static::Static;
use crate::{rebuild_fn, rebuild_fn_channel, ConstIndex, InnerSchemaCtx, MaybeReflect, ReBuildFn, RebuildFnReceiver, ReceiverPropState, Renderer, MaybeSend};
use alloc::boxed::Box;
use bevy_utils::all_tuples;
use core::any::TypeId;

pub trait SchemaParam<R>: MaybeSend + 'static
where
    R: Renderer,
{
    fn from<const I: usize>(ctx: &mut InnerSchemaCtx<R>) -> Self;
}

// pub trait SchemaPropParam<R>: SchemaParam<R>
// where
//     R: Renderer,
// {
//     type Value: MaybeSend + 'static;
// }

pub trait SchemaParams<R>: MaybeSend + 'static
where
    R: Renderer,
{
    fn from(ctx: &mut InnerSchemaCtx<R>) -> Self;
}

impl<R> SchemaParams<R> for ()
where
    R: Renderer,
{
    fn from(_: &mut InnerSchemaCtx<R>) -> Self {}
}

pub trait SchemaParamDefault<R>
where
    R: Renderer,
{
    fn param_default(ctx: &mut InnerSchemaCtx<R>) -> Self;
}

impl<R, T> SchemaParamDefault<R> for T
where
    R: Renderer,
    T: Default,
{
    fn param_default(_ctx: &mut InnerSchemaCtx<R>) -> Self {
        T::default()
    }
}

impl<R, T> SchemaParam<R> for Static<T>
where
    R: Renderer,
    T: SchemaParamDefault<R> + MaybeSend + 'static,
{
    fn from<const I: usize>(ctx: &mut InnerSchemaCtx<R>) -> Self {
        let prop_type_id = TypeId::of::<ConstIndex<I>>();
        let value: T = ctx
            .get_init_value::<T>(prop_type_id)
            .unwrap_or_else(|| T::param_default(ctx));
        Static(value)
    }
}

// impl<R, T> SchemaPropParam<R> for RebuildFnReceiver<R, T>
// where
//     R: Renderer,
//     T: MaybeReflect + Clone + PartialEq + MaybeSend + 'static,
// {
//     type Value = T;
// }

impl<R, T> SchemaParam<R> for RebuildFnReceiver<R, T>
where
    R: Renderer,
    T: MaybeReflect + Clone + PartialEq + MaybeSend + 'static,
{
    fn from<const I: usize>(ctx: &mut InnerSchemaCtx<R>) -> Self {
        let prop_type_id = TypeId::of::<ConstIndex<I>>();
        let (mut rebuild_f, sender) = rebuild_fn_channel::<R, T>();
        let value: Option<T> = ctx.get_init_value::<T>(prop_type_id);
        let x1 = &mut **ctx
            .prop_state()
            .entry(prop_type_id)
            .or_insert_with(|| Box::new(ReceiverPropState::<R, T>::new()));
        let x1 = x1
            .as_any_mut()
            .downcast_mut::<ReceiverPropState<R, T>>()
            .unwrap();
        x1.re_build_fns.push(ReBuildFn::new(move |world, x: T| {
            rebuild_f.call(world, x);
        }));
        if let Some(value) = &value {
            x1.value = Some(value.clone());
        }
        rebuild_fn(
            value,
            Box::new(move |f| {
                let _ = sender.send(f);
            }),
        )
    }
}

macro_rules! impl_schema_param {
    ($($P:ident),*) => {
        impl<R,$($P),*> SchemaParams<R> for ($($P,)*)
        where
            R: Renderer,
            $($P: SchemaParam<R>),*
        {
            count_macro::count! {
            fn from(ctx: &mut InnerSchemaCtx<R>) -> Self {
                (
                    $(
                        $P::from::<_int_>(ctx),
                    )*
                )
            }
            }
        }
    };
    () => {};
}

all_tuples!(impl_schema_param, 1, 16, P);
