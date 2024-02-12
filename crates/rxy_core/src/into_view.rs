use crate::utils::all_tuples;

use rxy_macro::impl_into_view;

use crate::{Renderer, View};

pub trait IntoView<R>: 'static
where
    R: Renderer,
{
    type View: View<R>;
    fn into_view(self) -> Self::View;
}

macro_rules! impl_into_view_for_tuples {
    ($first:ident) => {
        impl_into_view_for_tuples!($first,);
    };
    ($first:ident,$($ty:ident),*$(,)?) => {
        impl<R,$first,$($ty),*> $crate::IntoView<R> for ($first,$($ty,)*)
		where
            R: $crate::Renderer,
			$first: $crate::IntoView<R>,
			$($ty: $crate::IntoView<R>),*
        {
			type View = ($first::View, $($ty::View,)*);

            fn into_view(self) -> Self::View {
				paste::paste! {
                    let ([<$first:lower>], $([<$ty:lower>],)*) = self;
                    (
                        [<$first:lower>].into_view(),
                        $([<$ty:lower>].into_view(),)*
                    )
                }
            }
        }
    }
}

impl_into_view!(());

all_tuples!(impl_into_view_for_tuples, 1, 12, T);

pub struct ToIntoView<T>(pub T);

impl<R, T> IntoView<R> for ToIntoView<T>
where
    R: Renderer,
    T: View<R>,
{
    type View = T;

    #[inline(always)]
    fn into_view(self) -> Self::View {
        self.0
    }
}

#[inline(always)]
pub fn into_view<R: Renderer, V: View<R>>(view: V) -> ToIntoView<V> {
    ToIntoView(view)
}

pub trait IntoCloneableView<R>: 'static
where
    R: Renderer,
{
    type View: View<R> + Clone;
    fn into_cloneable_view(self) -> Self::View;
}

impl<R, T> IntoCloneableView<R> for T
where
    R: Renderer,
    T: IntoView<R>,
    T::View: Clone,
{
    type View = T::View;

    #[inline(always)]
    fn into_cloneable_view(self) -> Self::View {
        self.into_view()
    }
}
