use bevy_ecs::bundle::Bundle;

use rxy_core::{ElementView, MemberOwner};

use crate::{BevyRenderer, XBundle};

macro_rules! impl_view_builder_ext {
   ($name:ident;$ty:ident) => {
      pub trait $name: $ty<BevyRenderer> + Sized {
         #[inline]
         fn bundle<T: Bundle>(self, bundle: T) -> Self::AddMember<XBundle<T>>
         where
            Self: Sized,
         {
            self.member(XBundle(bundle))
         }
      }

      impl<T> $name for T where T: $ty<BevyRenderer> + Sized {}
   };
}
impl_view_builder_ext!(MemberOwnerViewBuilderExt;MemberOwner);
impl_view_builder_ext!(ElementViewViewBuilderExt;ElementView);
