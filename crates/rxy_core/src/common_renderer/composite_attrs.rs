#[macro_export]
macro_rules! impl_composite_attrs_use {
   ()=>{
      use rxy_core::{ElementAttrMember, ElementView, MapToAttrMarker, MemberOwner, XNest};
      use crate::all_attrs::{
         border_bottom, border_left, border_right, border_top, margin_bottom, margin_left, margin_right,
         margin_top, padding_bottom, padding_left, padding_right, padding_top,
      };
   }
}
#[macro_export]
macro_rules! impl_composite_attrs {
   ($renderer:ident;$name:ident;$ty:ident) => {
      pub trait $name: $ty<$renderer> + Sized {
         #[inline]
         fn border_x<T>(
            self,
            value: T,
         ) -> Self::AddMember<(
            T::MapInner<MapToAttrMarker<border_left>>,
            T::MapInner<MapToAttrMarker<border_right>>,
         )>
         where
            T: XNest + Clone,
            T::MapInner<MapToAttrMarker<border_left>>: ElementAttrMember<$renderer, border_left>,
            T::MapInner<MapToAttrMarker<border_right>>:
               ElementAttrMember<$renderer, border_right>,
         {
            self.member((value.clone().map_inner(), value.map_inner()))
         }

         #[inline]
         fn border_y<T>(
            self,
            value: T,
         ) -> Self::AddMember<(
            T::MapInner<MapToAttrMarker<border_top>>,
            T::MapInner<MapToAttrMarker<border_bottom>>,
         )>
         where
            T: XNest + Clone,
            T::MapInner<MapToAttrMarker<border_top>>: ElementAttrMember<$renderer, border_top>,
            T::MapInner<MapToAttrMarker<border_bottom>>:
               ElementAttrMember<$renderer, border_bottom>,
         {
            self.member((value.clone().map_inner(), value.map_inner()))
         }

         #[inline]
         fn border<T>(
            self,
            value: T,
         ) -> Self::AddMember<(
            T::MapInner<MapToAttrMarker<border_left>>,
            T::MapInner<MapToAttrMarker<border_right>>,
            T::MapInner<MapToAttrMarker<border_top>>,
            T::MapInner<MapToAttrMarker<border_bottom>>,
         )>
         where
            T: XNest + Clone,
            T::MapInner<MapToAttrMarker<border_left>>: ElementAttrMember<$renderer, border_left>,
            T::MapInner<MapToAttrMarker<border_right>>:
               ElementAttrMember<$renderer, border_right>,
            T::MapInner<MapToAttrMarker<border_top>>: ElementAttrMember<$renderer, border_top>,
            T::MapInner<MapToAttrMarker<border_bottom>>:
               ElementAttrMember<$renderer, border_bottom>,
         {
            self.member((
               value.clone().map_inner(),
               value.clone().map_inner(),
               value.clone().map_inner(),
               value.map_inner(),
            ))
         }

         #[inline]
         fn margin_horizontal<T>(
            self,
            value: T,
         ) -> Self::AddMember<(
            T::MapInner<MapToAttrMarker<margin_left>>,
            T::MapInner<MapToAttrMarker<margin_right>>,
         )>
         where
            T: XNest + Clone,
            T::MapInner<MapToAttrMarker<margin_left>>: ElementAttrMember<$renderer, margin_left>,
            T::MapInner<MapToAttrMarker<margin_right>>:
               ElementAttrMember<$renderer, margin_right>,
         {
            self.member((value.clone().map_inner(), value.map_inner()))
         }

         #[inline]
         fn margin_vertical<T>(
            self,
            value: T,
         ) -> Self::AddMember<(
            T::MapInner<MapToAttrMarker<margin_top>>,
            T::MapInner<MapToAttrMarker<margin_bottom>>,
         )>
         where
            T: XNest + Clone,
            T::MapInner<MapToAttrMarker<margin_top>>: ElementAttrMember<$renderer, margin_top>,
            T::MapInner<MapToAttrMarker<margin_bottom>>:
               ElementAttrMember<$renderer, margin_bottom>,
         {
            self.member((value.clone().map_inner(), value.map_inner()))
         }

         #[inline]
         fn margin<T>(
            self,
            value: T,
         ) -> Self::AddMember<(
            T::MapInner<MapToAttrMarker<margin_left>>,
            T::MapInner<MapToAttrMarker<margin_right>>,
            T::MapInner<MapToAttrMarker<margin_top>>,
            T::MapInner<MapToAttrMarker<margin_bottom>>,
         )>
         where
            T: XNest + Clone,
            T::MapInner<MapToAttrMarker<margin_left>>: ElementAttrMember<$renderer, margin_left>,
            T::MapInner<MapToAttrMarker<margin_right>>:
               ElementAttrMember<$renderer, margin_right>,
            T::MapInner<MapToAttrMarker<margin_top>>: ElementAttrMember<$renderer, margin_top>,
            T::MapInner<MapToAttrMarker<margin_bottom>>:
               ElementAttrMember<$renderer, margin_bottom>,
         {
            self.member((
               value.clone().map_inner(),
               value.clone().map_inner(),
               value.clone().map_inner(),
               value.map_inner(),
            ))
         }

         #[inline]
         fn padding_horizontal<T>(
            self,
            value: T,
         ) -> Self::AddMember<(
            T::MapInner<MapToAttrMarker<padding_left>>,
            T::MapInner<MapToAttrMarker<padding_right>>,
         )>
         where
            T: XNest + Clone,
            T::MapInner<MapToAttrMarker<padding_left>>:
               ElementAttrMember<$renderer, padding_left>,
            T::MapInner<MapToAttrMarker<padding_right>>:
               ElementAttrMember<$renderer, padding_right>,
         {
            self.member((value.clone().map_inner(), value.map_inner()))
         }

         #[inline]
         fn padding_vertical<T>(
            self,
            value: T,
         ) -> Self::AddMember<(
            T::MapInner<MapToAttrMarker<padding_top>>,
            T::MapInner<MapToAttrMarker<padding_bottom>>,
         )>
         where
            T: XNest + Clone,
            T::MapInner<MapToAttrMarker<padding_top>>: ElementAttrMember<$renderer, padding_top>,
            T::MapInner<MapToAttrMarker<padding_bottom>>:
               ElementAttrMember<$renderer, padding_bottom>,
         {
            self.member((value.clone().map_inner(), value.map_inner()))
         }

         #[inline]
         fn padding<T>(
            self,
            value: T,
         ) -> Self::AddMember<(
            T::MapInner<MapToAttrMarker<padding_left>>,
            T::MapInner<MapToAttrMarker<padding_right>>,
            T::MapInner<MapToAttrMarker<padding_top>>,
            T::MapInner<MapToAttrMarker<padding_bottom>>,
         )>
         where
            T: XNest + Clone,
            T::MapInner<MapToAttrMarker<padding_left>>:
               ElementAttrMember<$renderer, padding_left>,
            T::MapInner<MapToAttrMarker<padding_right>>:
               ElementAttrMember<$renderer, padding_right>,
            T::MapInner<MapToAttrMarker<padding_top>>: ElementAttrMember<$renderer, padding_top>,
            T::MapInner<MapToAttrMarker<padding_bottom>>:
               ElementAttrMember<$renderer, padding_bottom>,
         {
            self.member((
               value.clone().map_inner(),
               value.clone().map_inner(),
               value.clone().map_inner(),
               value.map_inner(),
            ))
         }
      }

      impl<T> $name for T where T: $ty<$renderer> {}
   };
}
