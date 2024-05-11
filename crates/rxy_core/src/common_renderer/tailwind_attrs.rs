#[macro_export]
macro_rules! impl_tailwind_attrs_use {
   () => {
      use rxy_core::{
         ElementAttr, ElementAttrMember, ElementView, MapToAttrMarker, MemberOwner, XNest,
      };

      use crate::all_attrs::{
         align_items, column_gap, display, flex_direction, flex_grow, flex_shrink, flex_wrap,
         height, justify_content, margin_bottom, margin_left, margin_right, margin_top, max_height,
         max_width, min_height, min_width, overflow_x, overflow_y, padding_bottom, padding_left,
         padding_right, padding_top, position_type, row_gap, visibility, width,
      };
   };
}

#[macro_export]
macro_rules! impl_tailwind_attrs {
   ($renderer:ident;$name:ident;$ty:ident) => {
      pub trait $name: $ty<$renderer> + Sized {
         impl_tailwind_attrs!(@TAFFY;$renderer;$name;$ty);
      }
      impl<T> $name for T where T: $ty<$renderer> {}
   };
   ($renderer:ident;$name:ident;$ty:ident;include_text_and_z_index) => {
      pub trait $name: $ty<$renderer> + Sized {
         impl_tailwind_attrs!(@TAFFY;$renderer;$name;$ty);
         impl_tailwind_attrs!(@TEXT;$renderer;$name;$ty);
         #[inline]
         fn z<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<crate::all_attrs::z_index>>>
         where
            T: XNest,
            T::MapInner<MapToAttrMarker<crate::all_attrs::z_index>>: ElementAttrMember<$renderer, crate::all_attrs::z_index>,
         {
            self.member(value.map_inner())
         }
      }
      impl<T> $name for T where T: $ty<$renderer> {}
   };
   ($renderer:ident;$name:ident;$ty:ident;include_text) => {
      pub trait $name: $ty<$renderer> + Sized {
         impl_tailwind_attrs!(@TAFFY;$renderer;$name;$ty)
         impl_tailwind_attrs!(@TEXT;$renderer;$name;$ty)
      }
      impl<T> $name for T where T: $ty<$renderer> {}
   };
   (@TEXT;$renderer:ident;$name:ident;$ty:ident) => {
         #[inline]
         fn text_nowrap(self) -> Self::AddMember<StaticElementAttr<$renderer, crate::all_attrs::text_linebreak>> {
            self.member(StaticElementAttr::new(BreakLineOn::NoWrap.into()))
         }
         #[inline]
         fn text_left(self) -> Self::AddMember<StaticElementAttr<$renderer, crate::all_attrs::text_align>> {
            self.member(StaticElementAttr::new(JustifyText::Left.into()))
         }
         #[inline]
         fn text_center(self) -> Self::AddMember<StaticElementAttr<$renderer, crate::all_attrs::text_align>> {
            self.member(StaticElementAttr::new(JustifyText::Center.into()))
         }
         #[inline]
         fn text_right(self) -> Self::AddMember<StaticElementAttr<$renderer, crate::all_attrs::text_align>> {
            self.member(StaticElementAttr::new(JustifyText::Right.into()))
         }
   };
   (@TAFFY;$renderer:ident;$name:ident;$ty:ident) => {
      #[inline]
      fn visible(self) -> Self::AddMember<StaticElementAttr<$renderer, visibility>> {
         self.member(StaticElementAttr::new(Visibility::Visible.into()))
      }
      #[inline]
      fn invisible(self) -> Self::AddMember<StaticElementAttr<$renderer, visibility>> {
         self.member(StaticElementAttr::new(Visibility::Hidden.into()))
      }
      #[inline]
      fn flex(self) -> Self::AddMember<StaticElementAttr<$renderer, display>> {
         self.member(StaticElementAttr::new(Display::Flex.into()))
      }
      #[inline]
      fn flex_col(
         self,
      ) -> Self::AddMember<(
         StaticElementAttr<$renderer, display>,
         StaticElementAttr<$renderer, flex_direction>,
      )> {
         self.member((
            StaticElementAttr::new(Display::Flex.into()),
            StaticElementAttr::new(FlexDirection::Column.into()),
         ))
      }
      #[inline]
      fn flex_row(
         self,
      ) -> Self::AddMember<(
         StaticElementAttr<$renderer, display>,
         StaticElementAttr<$renderer, flex_direction>,
      )> {
         self.member((
            StaticElementAttr::new(Display::Flex.into()),
            StaticElementAttr::new(FlexDirection::Row.into()),
         ))
      }
      #[cfg(feature="grid")]
      #[inline]
      fn grid(self) -> Self::AddMember<StaticElementAttr<$renderer, display>> {
         self.member(StaticElementAttr::new(Display::Grid.into()))
      }
      #[inline]
      fn shrink(self) -> Self::AddMember<StaticElementAttr<$renderer, flex_shrink>> {
         self.member(StaticElementAttr::new(1.0.into()))
      }
      #[inline]
      fn shrink_0(self) -> Self::AddMember<StaticElementAttr<$renderer, flex_shrink>> {
         self.member(StaticElementAttr::new(0.0.into()))
      }
      #[inline]
      fn grow(self) -> Self::AddMember<StaticElementAttr<$renderer, flex_grow>> {
         self.member(StaticElementAttr::new(1.0.into()))
      }
      #[inline]
      fn grow_0(self) -> Self::AddMember<StaticElementAttr<$renderer, flex_grow>> {
         self.member(StaticElementAttr::new(0.0.into()))
      }

      #[inline]
      fn justify_start(self) -> Self::AddMember<StaticElementAttr<$renderer, justify_content>> {
         self.member(StaticElementAttr::new(JustifyContent::Start.into()))
      }
      #[inline]
      fn justify_end(self) -> Self::AddMember<StaticElementAttr<$renderer, justify_content>> {
         self.member(StaticElementAttr::new(JustifyContent::End.into()))
      }
      #[inline]
      fn justify_center(self) -> Self::AddMember<StaticElementAttr<$renderer, justify_content>> {
         self.member(StaticElementAttr::new(JustifyContent::Center.into()))
      }
      #[inline]
      fn justify_between(self) -> Self::AddMember<StaticElementAttr<$renderer, justify_content>> {
         self.member(StaticElementAttr::new(JustifyContent::SpaceBetween.into()))
      }
      #[inline]
      fn justify_around(self) -> Self::AddMember<StaticElementAttr<$renderer, justify_content>> {
         self.member(StaticElementAttr::new(JustifyContent::SpaceAround.into()))
      }
      #[inline]
      fn justify_evenly(self) -> Self::AddMember<StaticElementAttr<$renderer, justify_content>> {
         self.member(StaticElementAttr::new(JustifyContent::SpaceEvenly.into()))
      }
      #[inline]
      fn items_start(self) -> Self::AddMember<StaticElementAttr<$renderer, align_items>> {
         self.member(StaticElementAttr::new(AlignItems::FlexStart.into()))
      }
      #[inline]
      fn items_end(self) -> Self::AddMember<StaticElementAttr<$renderer, align_items>> {
         self.member(StaticElementAttr::new(AlignItems::FlexEnd.into()))
      }
      #[inline]
      fn items_center(self) -> Self::AddMember<StaticElementAttr<$renderer, align_items>> {
         self.member(StaticElementAttr::new(AlignItems::Center.into()))
      }
      #[inline]
      fn items_baseline(self) -> Self::AddMember<StaticElementAttr<$renderer, align_items>> {
         self.member(StaticElementAttr::new(AlignItems::Baseline.into()))
      }
      #[inline]
      fn items_stretch(self) -> Self::AddMember<StaticElementAttr<$renderer, align_items>> {
         self.member(StaticElementAttr::new(AlignItems::Stretch.into()))
      }

      #[inline]
      fn gap<T>(
         self,
         value: T,
      ) -> Self::AddMember<(
         T::MapInner<MapToAttrMarker<column_gap>>,
         T::MapInner<MapToAttrMarker<row_gap>>,
      )>
      where
         T: XNest + Clone,
         T::MapInner<MapToAttrMarker<column_gap>>: ElementAttrMember<$renderer, column_gap>,
         T::MapInner<MapToAttrMarker<row_gap>>: ElementAttrMember<$renderer, row_gap>,
      {
         self.member((value.clone().map_inner(), value.map_inner()))
      }

      #[inline]
      fn gap_x<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<column_gap>>>
      where
         T: XNest,
         T::MapInner<MapToAttrMarker<column_gap>>: ElementAttrMember<$renderer, column_gap>,
      {
         self.member(value.map_inner())
      }

      #[inline]
      fn gap_y<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<row_gap>>>
      where
         T: XNest,
         T::MapInner<MapToAttrMarker<row_gap>>: ElementAttrMember<$renderer, row_gap>,
      {
         self.member(value.map_inner())
      }

      #[inline]
      fn relative(self) -> Self::AddMember<StaticElementAttr<$renderer, position_type>> {
         self.member(StaticElementAttr::new(PositionType::Relative.into()))
      }
      #[inline]
      fn absolute(self) -> Self::AddMember<StaticElementAttr<$renderer, position_type>> {
         self.member(StaticElementAttr::new(PositionType::Absolute.into()))
      }
      #[inline]
      fn hidden(self) -> Self::AddMember<StaticElementAttr<$renderer, display>> {
         self.member(StaticElementAttr::new(Display::None.into()))
      }

      #[inline]
      fn flex_wrap_wrap(self) -> Self::AddMember<StaticElementAttr<$renderer, flex_wrap>> {
         self.member(StaticElementAttr::new(FlexWrap::Wrap.into()))
      }

      #[inline]
      fn flex_wrap_reverse(self) -> Self::AddMember<StaticElementAttr<$renderer, flex_wrap>> {
         self.member(StaticElementAttr::new(FlexWrap::WrapReverse.into()))
      }
      #[inline]
      fn flex_nowrap(self) -> Self::AddMember<StaticElementAttr<$renderer, flex_wrap>> {
         self.member(StaticElementAttr::new(FlexWrap::NoWrap.into()))
      }

      #[inline]
      fn w<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<width>>>
      where
         T: XNest,
         T::MapInner<MapToAttrMarker<width>>: ElementAttrMember<$renderer, width>,
      {
         self.member(value.map_inner())
      }

      #[inline]
      fn h<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<height>>>
      where
         T: XNest,
         T::MapInner<MapToAttrMarker<height>>: ElementAttrMember<$renderer, height>,
      {
         self.member(value.map_inner())
      }

      #[inline]
      fn min_w<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<min_width>>>
      where
         T: XNest,
         T::MapInner<MapToAttrMarker<min_width>>: ElementAttrMember<$renderer, min_width>,
      {
         self.member(value.map_inner())
      }

      #[inline]
      fn max_w<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<max_width>>>
      where
         T: XNest,
         T::MapInner<MapToAttrMarker<max_width>>: ElementAttrMember<$renderer, max_width>,
      {
         self.member(value.map_inner())
      }

      #[inline]
      fn min_h<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<min_height>>>
      where
         T: XNest,
         T::MapInner<MapToAttrMarker<min_height>>: ElementAttrMember<$renderer, min_height>,
      {
         self.member(value.map_inner())
      }

      #[inline]
      fn max_h<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<max_height>>>
      where
         T: XNest,
         T::MapInner<MapToAttrMarker<max_height>>: ElementAttrMember<$renderer, max_height>,
      {
         self.member(value.map_inner())
      }

      #[inline]
      fn w_screen(self) -> Self::AddMember<StaticElementAttr<$renderer, width>> {
         self.member(StaticElementAttr::new(Val::Vw(100.).into()))
      }
      #[inline]
      fn h_screen(self) -> Self::AddMember<StaticElementAttr<$renderer, height>> {
         self.member(StaticElementAttr::new(Val::Vh(100.).into()))
      }

      #[inline]
      fn size_screen(
         self,
      ) -> Self::AddMember<(
         StaticElementAttr<$renderer, width>,
         StaticElementAttr<$renderer, height>,
      )> {
         self.member((
            StaticElementAttr::new(Val::Vw(100.).into()),
            StaticElementAttr::new(Val::Vh(100.).into()),
         ))
      }

      #[inline]
      fn h_full(self) -> Self::AddMember<StaticElementAttr<$renderer, height>> {
         self.member(StaticElementAttr::new(Val::Percent(100.).into()))
      }

      #[inline]
      fn w_full(self) -> Self::AddMember<StaticElementAttr<$renderer, width>> {
         self.member(StaticElementAttr::new(Val::Percent(100.).into()))
      }

      #[inline]
      fn size_full(
         self,
      ) -> Self::AddMember<(
         StaticElementAttr<$renderer, width>,
         StaticElementAttr<$renderer, height>,
      )> {
         self.member((
            StaticElementAttr::new(Val::Percent(100.).into()),
            StaticElementAttr::new(Val::Percent(100.).into()),
         ))
      }


      #[inline]
      fn size<T>(
         self,
         value: T,
      ) -> Self::AddMember<(
         T::MapInner<MapToAttrMarker<width>>,
         T::MapInner<MapToAttrMarker<height>>,
      )>
      where
         T: XNest + Clone,
         T::MapInner<MapToAttrMarker<width>>: ElementAttrMember<$renderer, width>,
         T::MapInner<MapToAttrMarker<height>>: ElementAttrMember<$renderer, height>,
      {
         self.member((value.clone().map_inner(), value.map_inner()))
      }

      #[inline]
      fn center(
         self,
      ) -> Self::AddMember<(
         StaticElementAttr<$renderer, align_items>,
         StaticElementAttr<$renderer, justify_content>,
      )> {
         self.member((
            StaticElementAttr::new(AlignItems::Center.into()),
            StaticElementAttr::new(JustifyContent::Center.into()),
         ))
      }

      #[inline]
      fn overflow<T>(
         self,
         value: T,
      ) -> Self::AddMember<(
         T::MapInner<MapToAttrMarker<overflow_x>>,
         T::MapInner<MapToAttrMarker<overflow_y>>,
      )>
      where
         T: XNest + Clone,
         T::MapInner<MapToAttrMarker<overflow_x>>: ElementAttrMember<$renderer, overflow_x>,
         T::MapInner<MapToAttrMarker<overflow_y>>: ElementAttrMember<$renderer, overflow_y>,
      {
         self.member((value.clone().map_inner(), value.map_inner()))
      }

      #[inline]
      fn pt<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<padding_top>>>
      where
         T: XNest,
         T::MapInner<MapToAttrMarker<padding_top>>: ElementAttrMember<$renderer, padding_top>,
      {
         self.member(value.map_inner())
      }

      #[inline]
      fn pb<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<padding_bottom>>>
      where
         T: XNest,
         T::MapInner<MapToAttrMarker<padding_bottom>>:
            ElementAttrMember<$renderer, padding_bottom>,
      {
         self.member(value.map_inner())
      }

      #[inline]
      fn pl<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<padding_left>>>
      where
         T: XNest,
         T::MapInner<MapToAttrMarker<padding_left>>:
            ElementAttrMember<$renderer, padding_left>,
      {
         self.member(value.map_inner())
      }

      #[inline]
      fn pr<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<padding_right>>>
      where
         T: XNest,
         T::MapInner<MapToAttrMarker<padding_right>>:
            ElementAttrMember<$renderer, padding_right>,
      {
         self.member(value.map_inner())
      }

      #[inline]
      fn px<T>(
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
      fn py<T>(
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
      fn p<T>(
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

      #[inline]
      fn mt<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<margin_top>>>
      where
         T: XNest,
         T::MapInner<MapToAttrMarker<margin_top>>: ElementAttrMember<$renderer, margin_top>,
      {
         self.member(value.map_inner())
      }

      #[inline]
      fn mb<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<margin_bottom>>>
      where
         T: XNest,
         T::MapInner<MapToAttrMarker<margin_bottom>>:
            ElementAttrMember<$renderer, margin_bottom>,
      {
         self.member(value.map_inner())
      }

      #[inline]
      fn ml<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<margin_left>>>
      where
         T: XNest,
         T::MapInner<MapToAttrMarker<margin_left>>: ElementAttrMember<$renderer, margin_left>,
      {
         self.member(value.map_inner())
      }

      #[inline]
      fn mr<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<margin_right>>>
      where
         T: XNest,
         T::MapInner<MapToAttrMarker<margin_right>>:
            ElementAttrMember<$renderer, margin_right>,
      {
         self.member(value.map_inner())
      }

      #[inline]
      fn mx<T>(
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
      fn my<T>(
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
      fn m<T>(
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
   };
}
