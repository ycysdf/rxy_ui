use crate::{IntoSchemaProp, Renderer, SchemaPropValue};
use alloc::borrow::Cow;
use alloc::string::String;

pub struct IntoSchemaPropValueWrapper<T>(pub T);

pub trait IntoSchemaPropValue<T> {
    fn into(self) -> T;
}

impl<R, T, IT> IntoSchemaProp<R, T> for IT
where
    R: Renderer,
    IT: IntoSchemaPropValue<IntoSchemaPropValueWrapper<T>>,
    T: Send + 'static,
{
    type Prop = SchemaPropValue<T>;

    fn into_schema_prop<const I: usize>(self) -> Self::Prop {
        SchemaPropValue::new(self.into().0)
    }
}

#[macro_export]
macro_rules! impl_into_schema_prop_value_wrapper_into {
    ($($from:ty => $to:ty,)*) => {
        $(
            impl IntoSchemaPropValue<IntoSchemaPropValueWrapper<$to>> for $from {
                fn into(self) -> IntoSchemaPropValueWrapper<$to> {
                    IntoSchemaPropValueWrapper(self.into())
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_schema_prop_value_wrapper_into {
    ($($ty:ty,)*) => {
        $(
            impl IntoSchemaPropValue<$crate::IntoSchemaPropValueWrapper<Self>> for $ty {
                fn into(self) -> $crate::IntoSchemaPropValueWrapper<Self> {
                    $crate::IntoSchemaPropValueWrapper(self)
                }
            }
        )*
    };
}

impl_schema_prop_value_wrapper_into! {
    u8,
    u16,
    u32,
    u64,
    i8,
    i16,
    i32,
    i64,
    f32,
    f64,
    (),
    bool,
    usize,
    isize,
    String,
    &'static str,
}

impl<T: Clone> IntoSchemaPropValue<IntoSchemaPropValueWrapper<Self>> for Cow<'static, T> {
    fn into(self) -> IntoSchemaPropValueWrapper<Self> {
        IntoSchemaPropValueWrapper(self)
    }
}
