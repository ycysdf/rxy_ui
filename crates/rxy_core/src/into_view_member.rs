use crate::utils::all_tuples;
use crate::{Renderer, ViewMember};

pub trait IntoViewMember<R, VM>
where
    R: Renderer,
    VM: ViewMember<R>,
{
    fn into_member(self) -> VM;
}

impl<R> IntoViewMember<R, Self> for ()
where
    R: Renderer,
{
    fn into_member(self) -> Self {
        ()
    }
}


macro_rules! impl_into_view_member_for_tuples {
    ($(($ty:ident,$vm:ident)),*$(,)?) => {
        #[allow(non_snake_case)]
        impl<R,$($ty),*,$($vm),*> $crate::IntoViewMember<R,($($vm,)*)> for ($($ty,)*)
		where
            R: $crate::Renderer,
			$($vm: $crate::ViewMember<R>,)*
			$($ty: $crate::IntoViewMember<R,$vm>,)*
        {
            fn into_member(self) -> ($($vm,)*) {
                let ($($ty,)*) = self;
                ($($ty.into_member(),)*)
            }
        }
    }
}

all_tuples!(impl_into_view_member_for_tuples, 1, 4, T, M);

#[derive(Clone, Debug, PartialEq,Eq)]
pub struct IntoViewMemberWrapper<T>(pub T);

impl<R, VM> IntoViewMember<R, VM> for IntoViewMemberWrapper<VM>
where
    R: Renderer,
    VM: ViewMember<R>,
{
    fn into_member(self) -> VM {
        self.0
    }
}
