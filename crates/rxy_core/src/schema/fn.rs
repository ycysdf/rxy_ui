use bevy_utils::all_tuples;
use crate::MaybeSend;

pub trait SchemaFn<P>: MaybeSend + 'static {
    type View;
    fn call(self, param: P) -> Self::View;
}

impl<V, F> SchemaFn<()> for F
where
    F: FnOnce() -> V + MaybeSend + 'static,
{
    type View = V;
    fn call(self, _p: ()) -> Self::View {
        self()
    }
}

macro_rules! impl_schema_fn {
    ($($P:ident),*) => {
        #[allow(non_snake_case)]
        impl<V,F,$($P),*> SchemaFn<($($P,)*)> for F
        where
            F: FnOnce($($P),*) -> V + MaybeSend + 'static,
        {
            type View = V;
            fn call(self, ($($P,)*): ($($P,)*)) -> Self::View {
                self($($P,)*)
            }
        }
    };
    () => {};
}

all_tuples!(impl_schema_fn, 1, 16, P);
