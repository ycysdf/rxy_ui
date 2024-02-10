use bevy_render::prelude::Visibility;
use bevy_text::{BreakLineOn, TextAlignment};
use bevy_ui::{AlignItems, Display, FlexDirection, FlexWrap, JustifyContent, PositionType, Val};
use rxy_bevy::all_attrs::{
    align_items, column_gap, display, flex_direction, flex_grow, flex_shrink, flex_wrap, height,
    justify_content, margin_bottom, margin_left, margin_right, margin_top, max_height, max_width,
    min_height, min_width, overflow_x, overflow_y, padding_bottom, padding_left, padding_right,
    padding_top, position_type, row_gap, text_align, text_linebreak, visibility, width, z_index,
};
use rxy_bevy::BevyRenderer;
use rxy_core::{
    ElementAttr, ElementAttrMember, ElementAttrType, MapToAttrMarker, MemberOwner, Renderer,
    ViewMember, ViewMemberOrigin, XNest,
};

pub trait TailwindAttrs: MemberOwner<BevyRenderer> + Sized {
    fn visible(self) -> Self::AddMember<ElementAttr<BevyRenderer, visibility>> {
        self.member(ElementAttr::new(Visibility::Visible.into()))
    }
    fn invisible(self) -> Self::AddMember<ElementAttr<BevyRenderer, visibility>> {
        self.member(ElementAttr::new(Visibility::Hidden.into()))
    }
    fn flex(self) -> Self::AddMember<ElementAttr<BevyRenderer, display>> {
        self.member(ElementAttr::new(Display::Flex.into()))
    }
    fn flex_col(
        self,
    ) -> Self::AddMember<(
        ElementAttr<BevyRenderer, display>,
        ElementAttr<BevyRenderer, flex_direction>,
    )> {
        self.member((
            ElementAttr::new(Display::Flex.into()),
            ElementAttr::new(FlexDirection::Column.into()),
        ))
    }
    fn flex_row(
        self,
    ) -> Self::AddMember<(
        ElementAttr<BevyRenderer, display>,
        ElementAttr<BevyRenderer, flex_direction>,
    )> {
        self.member((
            ElementAttr::new(Display::Flex.into()),
            ElementAttr::new(FlexDirection::Row.into()),
        ))
    }
    fn grid(self) -> Self::AddMember<ElementAttr<BevyRenderer, display>> {
        self.member(ElementAttr::new(Display::Grid.into()))
    }
    fn shrink(self) -> Self::AddMember<ElementAttr<BevyRenderer, flex_shrink>> {
        self.member(ElementAttr::new(1.0.into()))
    }
    fn shrink_0(self) -> Self::AddMember<ElementAttr<BevyRenderer, flex_shrink>> {
        self.member(ElementAttr::new(0.0.into()))
    }
    fn grow(self) -> Self::AddMember<ElementAttr<BevyRenderer, flex_grow>> {
        self.member(ElementAttr::new(1.0.into()))
    }
    fn grow_0(self) -> Self::AddMember<ElementAttr<BevyRenderer, flex_grow>> {
        self.member(ElementAttr::new(0.0.into()))
    }

    fn justify_start(self) -> Self::AddMember<ElementAttr<BevyRenderer, justify_content>> {
        self.member(ElementAttr::new(JustifyContent::Start.into()))
    }
    fn justify_end(self) -> Self::AddMember<ElementAttr<BevyRenderer, justify_content>> {
        self.member(ElementAttr::new(JustifyContent::End.into()))
    }
    fn justify_center(self) -> Self::AddMember<ElementAttr<BevyRenderer, justify_content>> {
        self.member(ElementAttr::new(JustifyContent::Center.into()))
    }
    fn justify_between(self) -> Self::AddMember<ElementAttr<BevyRenderer, justify_content>> {
        self.member(ElementAttr::new(JustifyContent::SpaceBetween.into()))
    }
    fn justify_around(self) -> Self::AddMember<ElementAttr<BevyRenderer, justify_content>> {
        self.member(ElementAttr::new(JustifyContent::SpaceAround.into()))
    }
    fn justify_evenly(self) -> Self::AddMember<ElementAttr<BevyRenderer, justify_content>> {
        self.member(ElementAttr::new(JustifyContent::SpaceEvenly.into()))
    }
    fn items_start(self) -> Self::AddMember<ElementAttr<BevyRenderer, align_items>> {
        self.member(ElementAttr::new(AlignItems::FlexStart.into()))
    }
    fn items_end(self) -> Self::AddMember<ElementAttr<BevyRenderer, align_items>> {
        self.member(ElementAttr::new(AlignItems::FlexEnd.into()))
    }
    fn items_center(self) -> Self::AddMember<ElementAttr<BevyRenderer, align_items>> {
        self.member(ElementAttr::new(AlignItems::Center.into()))
    }
    fn items_baseline(self) -> Self::AddMember<ElementAttr<BevyRenderer, align_items>> {
        self.member(ElementAttr::new(AlignItems::Baseline.into()))
    }
    fn items_stretch(self) -> Self::AddMember<ElementAttr<BevyRenderer, align_items>> {
        self.member(ElementAttr::new(AlignItems::Stretch.into()))
    }

    fn gap<T>(
        self,
        value: T,
    ) -> Self::AddMember<(
        T::MapInner<MapToAttrMarker<column_gap>>,
        T::MapInner<MapToAttrMarker<row_gap>>,
    )>
    where
        T: XNest + Clone,
        T::MapInner<MapToAttrMarker<column_gap>>: ElementAttrMember<BevyRenderer, column_gap>,
        T::MapInner<MapToAttrMarker<row_gap>>: ElementAttrMember<BevyRenderer, row_gap>,
    {
        self.member((value.clone().map_inner(), value.map_inner()))
    }

    fn gap_x<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<column_gap>>>
    where
        T: XNest,
        T::MapInner<MapToAttrMarker<column_gap>>: ElementAttrMember<BevyRenderer, column_gap>,
    {
        self.member(value.map_inner())
    }

    fn gap_y<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<row_gap>>>
    where
        T: XNest,
        T::MapInner<MapToAttrMarker<row_gap>>: ElementAttrMember<BevyRenderer, row_gap>,
    {
        self.member(value.map_inner())
    }

    fn relative(self) -> Self::AddMember<ElementAttr<BevyRenderer, position_type>> {
        self.member(ElementAttr::new(PositionType::Relative.into()))
    }
    fn absolute(self) -> Self::AddMember<ElementAttr<BevyRenderer, position_type>> {
        self.member(ElementAttr::new(PositionType::Absolute.into()))
    }
    fn hidden(self) -> Self::AddMember<ElementAttr<BevyRenderer, display>> {
        self.member(ElementAttr::new(Display::None.into()))
    }

    fn flex_wrap(self) -> Self::AddMember<ElementAttr<BevyRenderer, flex_wrap>> {
        self.member(ElementAttr::new(FlexWrap::Wrap.into()))
    }

    fn flex_wrap_reverse(self) -> Self::AddMember<ElementAttr<BevyRenderer, flex_wrap>> {
        self.member(ElementAttr::new(FlexWrap::WrapReverse.into()))
    }
    fn flex_nowrap(self) -> Self::AddMember<ElementAttr<BevyRenderer, flex_wrap>> {
        self.member(ElementAttr::new(FlexWrap::NoWrap.into()))
    }

    fn w<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<width>>>
    where
        T: XNest,
        T::MapInner<MapToAttrMarker<width>>: ElementAttrMember<BevyRenderer, width>,
    {
        self.member(value.map_inner())
    }

    fn h<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<height>>>
    where
        T: XNest,
        T::MapInner<MapToAttrMarker<height>>: ElementAttrMember<BevyRenderer, height>,
    {
        self.member(value.map_inner())
    }

    fn min_w<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<min_width>>>
    where
        T: XNest,
        T::MapInner<MapToAttrMarker<min_width>>: ElementAttrMember<BevyRenderer, min_width>,
    {
        self.member(value.map_inner())
    }

    fn max_w<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<max_width>>>
    where
        T: XNest,
        T::MapInner<MapToAttrMarker<max_width>>: ElementAttrMember<BevyRenderer, max_width>,
    {
        self.member(value.map_inner())
    }

    fn min_h<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<min_height>>>
    where
        T: XNest,
        T::MapInner<MapToAttrMarker<min_height>>: ElementAttrMember<BevyRenderer, min_height>,
    {
        self.member(value.map_inner())
    }

    fn max_h<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<max_height>>>
    where
        T: XNest,
        T::MapInner<MapToAttrMarker<max_height>>: ElementAttrMember<BevyRenderer, max_height>,
    {
        self.member(value.map_inner())
    }

    fn w_screen(self) -> Self::AddMember<ElementAttr<BevyRenderer, width>> {
        self.member(ElementAttr::new(Val::Vw(100.).into()))
    }
    fn h_screen(self) -> Self::AddMember<ElementAttr<BevyRenderer, height>> {
        self.member(ElementAttr::new(Val::Vh(100.).into()))
    }

    fn size_screen(
        self,
    ) -> Self::AddMember<(
        ElementAttr<BevyRenderer, width>,
        ElementAttr<BevyRenderer, height>,
    )> {
        self.member((
            ElementAttr::new(Val::Vw(100.).into()),
            ElementAttr::new(Val::Vh(100.).into()),
        ))
    }

    fn h_full(self) -> Self::AddMember<ElementAttr<BevyRenderer, height>> {
        self.member(ElementAttr::new(Val::Percent(100.).into()))
    }

    fn w_full(self) -> Self::AddMember<ElementAttr<BevyRenderer, width>> {
        self.member(ElementAttr::new(Val::Percent(100.).into()))
    }

    fn text_nowrap(self) -> Self::AddMember<ElementAttr<BevyRenderer, text_linebreak>> {
        self.member(ElementAttr::new(BreakLineOn::NoWrap.into()))
    }
    fn text_left(self) -> Self::AddMember<ElementAttr<BevyRenderer, text_align>> {
        self.member(ElementAttr::new(TextAlignment::Left.into()))
    }
    fn text_center(self) -> Self::AddMember<ElementAttr<BevyRenderer, text_align>> {
        self.member(ElementAttr::new(TextAlignment::Center.into()))
    }
    fn text_right(self) -> Self::AddMember<ElementAttr<BevyRenderer, text_align>> {
        self.member(ElementAttr::new(TextAlignment::Right.into()))
    }

    fn size<T>(
        self,
        value: T,
    ) -> Self::AddMember<(
        T::MapInner<MapToAttrMarker<width>>,
        T::MapInner<MapToAttrMarker<height>>,
    )>
    where
        T: XNest + Clone,
        T::MapInner<MapToAttrMarker<width>>: ElementAttrMember<BevyRenderer, width>,
        T::MapInner<MapToAttrMarker<height>>: ElementAttrMember<BevyRenderer, height>,
    {
        self.member((value.clone().map_inner(), value.map_inner()))
    }

    fn center(
        self,
    ) -> Self::AddMember<(
        ElementAttr<BevyRenderer, align_items>,
        ElementAttr<BevyRenderer, justify_content>,
    )> {
        self.member((
            ElementAttr::new(AlignItems::Center.into()),
            ElementAttr::new(JustifyContent::Center.into()),
        ))
    }

    fn overflow<T>(
        self,
        value: T,
    ) -> Self::AddMember<(
        T::MapInner<MapToAttrMarker<overflow_x>>,
        T::MapInner<MapToAttrMarker<overflow_y>>,
    )>
    where
        T: XNest + Clone,
        T::MapInner<MapToAttrMarker<overflow_x>>: ElementAttrMember<BevyRenderer, overflow_x>,
        T::MapInner<MapToAttrMarker<overflow_y>>: ElementAttrMember<BevyRenderer, overflow_y>,
    {
        self.member((value.clone().map_inner(), value.map_inner()))
    }

    fn pt<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<padding_top>>>
    where
        T: XNest,
        T::MapInner<MapToAttrMarker<padding_top>>: ElementAttrMember<BevyRenderer, padding_top>,
    {
        self.member(value.map_inner())
    }

    fn pb<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<padding_bottom>>>
    where
        T: XNest,
        T::MapInner<MapToAttrMarker<padding_bottom>>:
            ElementAttrMember<BevyRenderer, padding_bottom>,
    {
        self.member(value.map_inner())
    }

    fn pl<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<padding_left>>>
    where
        T: XNest,
        T::MapInner<MapToAttrMarker<padding_left>>: ElementAttrMember<BevyRenderer, padding_left>,
    {
        self.member(value.map_inner())
    }

    fn pr<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<padding_right>>>
    where
        T: XNest,
        T::MapInner<MapToAttrMarker<padding_right>>: ElementAttrMember<BevyRenderer, padding_right>,
    {
        self.member(value.map_inner())
    }

    fn px<T>(
        self,
        value: T,
    ) -> Self::AddMember<(
        T::MapInner<MapToAttrMarker<padding_left>>,
        T::MapInner<MapToAttrMarker<padding_right>>,
    )>
    where
        T: XNest + Clone,
        T::MapInner<MapToAttrMarker<padding_left>>: ElementAttrMember<BevyRenderer, padding_left>,
        T::MapInner<MapToAttrMarker<padding_right>>: ElementAttrMember<BevyRenderer, padding_right>,
    {
        self.member((value.clone().map_inner(), value.map_inner()))
    }

    fn py<T>(
        self,
        value: T,
    ) -> Self::AddMember<(
        T::MapInner<MapToAttrMarker<padding_top>>,
        T::MapInner<MapToAttrMarker<padding_bottom>>,
    )>
    where
        T: XNest + Clone,
        T::MapInner<MapToAttrMarker<padding_top>>: ElementAttrMember<BevyRenderer, padding_top>,
        T::MapInner<MapToAttrMarker<padding_bottom>>:
            ElementAttrMember<BevyRenderer, padding_bottom>,
    {
        self.member((value.clone().map_inner(), value.map_inner()))
    }

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
        T::MapInner<MapToAttrMarker<padding_left>>: ElementAttrMember<BevyRenderer, padding_left>,
        T::MapInner<MapToAttrMarker<padding_right>>: ElementAttrMember<BevyRenderer, padding_right>,
        T::MapInner<MapToAttrMarker<padding_top>>: ElementAttrMember<BevyRenderer, padding_top>,
        T::MapInner<MapToAttrMarker<padding_bottom>>:
            ElementAttrMember<BevyRenderer, padding_bottom>,
    {
        self.member((
            value.clone().map_inner(),
            value.clone().map_inner(),
            value.clone().map_inner(),
            value.map_inner(),
        ))
    }

    fn mt<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<margin_top>>>
    where
        T: XNest,
        T::MapInner<MapToAttrMarker<margin_top>>: ElementAttrMember<BevyRenderer, margin_top>,
    {
        self.member(value.map_inner())
    }

    fn mb<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<margin_bottom>>>
    where
        T: XNest,
        T::MapInner<MapToAttrMarker<margin_bottom>>: ElementAttrMember<BevyRenderer, margin_bottom>,
    {
        self.member(value.map_inner())
    }

    fn ml<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<margin_left>>>
    where
        T: XNest,
        T::MapInner<MapToAttrMarker<margin_left>>: ElementAttrMember<BevyRenderer, margin_left>,
    {
        self.member(value.map_inner())
    }

    fn mr<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<margin_right>>>
    where
        T: XNest,
        T::MapInner<MapToAttrMarker<margin_right>>: ElementAttrMember<BevyRenderer, margin_right>,
    {
        self.member(value.map_inner())
    }

    fn mx<T>(
        self,
        value: T,
    ) -> Self::AddMember<(
        T::MapInner<MapToAttrMarker<margin_left>>,
        T::MapInner<MapToAttrMarker<margin_right>>,
    )>
    where
        T: XNest + Clone,
        T::MapInner<MapToAttrMarker<margin_left>>: ElementAttrMember<BevyRenderer, margin_left>,
        T::MapInner<MapToAttrMarker<margin_right>>: ElementAttrMember<BevyRenderer, margin_right>,
    {
        self.member((value.clone().map_inner(), value.map_inner()))
    }

    fn my<T>(
        self,
        value: T,
    ) -> Self::AddMember<(
        T::MapInner<MapToAttrMarker<margin_top>>,
        T::MapInner<MapToAttrMarker<margin_bottom>>,
    )>
    where
        T: XNest + Clone,
        T::MapInner<MapToAttrMarker<margin_top>>: ElementAttrMember<BevyRenderer, margin_top>,
        T::MapInner<MapToAttrMarker<margin_bottom>>: ElementAttrMember<BevyRenderer, margin_bottom>,
    {
        self.member((value.clone().map_inner(), value.map_inner()))
    }

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
        T::MapInner<MapToAttrMarker<margin_left>>: ElementAttrMember<BevyRenderer, margin_left>,
        T::MapInner<MapToAttrMarker<margin_right>>: ElementAttrMember<BevyRenderer, margin_right>,
        T::MapInner<MapToAttrMarker<margin_top>>: ElementAttrMember<BevyRenderer, margin_top>,
        T::MapInner<MapToAttrMarker<margin_bottom>>: ElementAttrMember<BevyRenderer, margin_bottom>,
    {
        self.member((
            value.clone().map_inner(),
            value.clone().map_inner(),
            value.clone().map_inner(),
            value.map_inner(),
        ))
    }

    fn z<T>(self, value: T) -> Self::AddMember<T::MapInner<MapToAttrMarker<z_index>>>
    where
        T: XNest,
        T::MapInner<MapToAttrMarker<z_index>>: ElementAttrMember<BevyRenderer, z_index>,
    {
        self.member(value.map_inner())
    }
}
impl<T> TailwindAttrs for T where T: MemberOwner<BevyRenderer> {}
