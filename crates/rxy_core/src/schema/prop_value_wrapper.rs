use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::any::Any;
use core::sync::atomic::*;

use crate::utils::all_tuples;
use crate::{IntoSchemaProp, MaybeSend, Renderer, SchemaPropValue};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntoSchemaPropValueWrapper<T>(pub T);

pub trait IntoSchemaPropValue<T> {
   fn into(self) -> T;
}

impl<R, T, IT> IntoSchemaProp<R, T> for IT
where
   R: Renderer,
   IT: IntoSchemaPropValue<IntoSchemaPropValueWrapper<T>>,
   T: MaybeSend + 'static,
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
    bool,
    usize,
    isize,
    char,
    String,
    &'static str,
    Box<dyn Any>,
    AtomicI8,
    AtomicU8,
    AtomicI16,
    AtomicI32,
    AtomicI64,
    AtomicU16,
    AtomicU32,
    AtomicU64,
    AtomicBool,
    AtomicIsize,
    AtomicUsize,
}

impl<T: Clone> IntoSchemaPropValue<IntoSchemaPropValueWrapper<Self>> for Cow<'static, T> {
   fn into(self) -> IntoSchemaPropValueWrapper<Self> {
      IntoSchemaPropValueWrapper(self)
   }
}

impl<T> IntoSchemaPropValue<IntoSchemaPropValueWrapper<Self>> for Vec<T> {
   fn into(self) -> IntoSchemaPropValueWrapper<Self> {
      IntoSchemaPropValueWrapper(self)
   }
}

impl<T> IntoSchemaPropValue<IntoSchemaPropValueWrapper<Self>> for AtomicPtr<T> {
   fn into(self) -> IntoSchemaPropValueWrapper<Self> {
      IntoSchemaPropValueWrapper(self)
   }
}

impl<T, const SIZE: usize> IntoSchemaPropValue<IntoSchemaPropValueWrapper<Self>> for [T; SIZE] {
   fn into(self) -> IntoSchemaPropValueWrapper<Self> {
      IntoSchemaPropValueWrapper(self)
   }
}

#[macro_export]
macro_rules! impl_schema_prop_value_wrapper_into_for_tuple {
    ($($ty:ident),*) => {
        impl<$($ty),*> IntoSchemaPropValue<IntoSchemaPropValueWrapper<Self>> for ($($ty,)*)
        {
            fn into(self) -> IntoSchemaPropValueWrapper<Self> {
                IntoSchemaPropValueWrapper(self)
            }
        }
    };
}

all_tuples!(impl_schema_prop_value_wrapper_into_for_tuple, 0, 4, T);
