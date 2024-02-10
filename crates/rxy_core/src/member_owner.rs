use crate::utils::all_tuples;
use crate::{MaybeSend, Renderer, ViewMember};

// todo:
pub trait MemberOwner<R>
where
    R: Renderer,
{
    type E: MaybeSend + 'static;
    type VM: ViewMember<R>;
    type AddMember<VM: ViewMember<R>>: MemberOwner<R>;
    type SetMembers<VM: ViewMember<R> + MemberOwner<R>>: MemberOwner<R>;
    fn member<VM>(
        self,
        member: VM,
    ) -> Self::AddMember<VM>
    where
        (Self::VM, VM): ViewMember<R>,
        VM: ViewMember<R>;
    fn members<VM: ViewMember<R>>(
        self,
        members: VM,
    ) -> Self::SetMembers<(VM,)>
    where
        VM: ViewMember<R>;
}

macro_rules! impl_member_owner_for_tuple {
    ($($t:ident),*) => {
        #[allow(non_snake_case)]
        impl<R, $($t),*> MemberOwner<R> for ($($t,)*)
        where
            R: Renderer,
            $($t: ViewMember<R>),*
        {
            type E = ();
            type VM = Self;
            type AddMember<T: ViewMember<R>> = (Self, T);
            type SetMembers<T: ViewMember<R> + MemberOwner<R>> = T;

            fn member<T>(self, member: T) -> Self::AddMember<T>
            where
                (Self::VM, T): ViewMember<R>,
                T: ViewMember<R>,
            {
                // let ($($t,)*) = self;
                // ($($t,)* member,)
                (self,member)
            }

            fn members<T>(self, members: T) -> Self::SetMembers<(T,)>
            where
                T: ViewMember<R>
            {
                (members,)
            }
        }
    };
    (END;$($t:ident),*) => {
        #[allow(non_snake_case)]
        impl<R, $($t),*> MemberOwner<R> for ($($t,)*)
        where
            R: Renderer,
            $($t: ViewMember<R>),*
        {
            type E = ();
            type VM = ();
            type AddMember<T: ViewMember<R>> = ();
            type SetMembers<T: ViewMember<R> + MemberOwner<R>> = T;

            fn member<T>(self, _member: T) -> Self::AddMember<T>
            where
                (Self::VM, T): ViewMember<R>,
                T: ViewMember<R>,
            {
                unimplemented!()
            }

            fn members<T: ViewMember<R>>(self, members: T) -> Self::SetMembers<(T,)>
            where
                T: ViewMember<R>
            {
                (members,)
            }
        }
    };
}

#[allow(non_snake_case)]
impl<R> MemberOwner<R> for ()
where
    R: Renderer,
{
    type E = ();
    type VM = Self;
    type AddMember<T: ViewMember<R>> = (T,);
    type SetMembers<T: ViewMember<R> + MemberOwner<R>> = T;

    fn member<T>(
        self,
        member: T,
    ) -> Self::AddMember<T>
    where
        (Self::VM, T): ViewMember<R>,
        T: ViewMember<R>,
    {
        (member,)
    }

    fn members<T>(
        self,
        members: T,
    ) -> Self::SetMembers<(T,)>
    where
        T: ViewMember<R>,
    {
        (members,)
    }
}

all_tuples!(impl_member_owner_for_tuple, 1, 4, M);
