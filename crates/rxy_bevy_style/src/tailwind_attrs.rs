use bevy_render::prelude::Visibility;
use bevy_text::{BreakLineOn, TextAlignment};
use bevy_ui::{
    AlignItems, Display, FlexDirection, FlexWrap, JustifyContent, OverflowAxis,
    PositionType, Val, ZIndex,
};
use rxy_bevy::{BevyRenderer, BevyWrapper, CommonAttrsViewBuilder, CompositeAttrs, ViewAttr};
use rxy_bevy_element::all_attrs::*;
use rxy_core::MemberOwner;

pub trait TailwindAttrs: MemberOwner<BevyRenderer> + Sized {
    fn visible(self) -> Self::AddMember<ViewAttr<visibility>> {
        self.member(ViewAttr::<visibility>(Visibility::Visible))
    }
    fn invisible(self) -> Self::AddMember<ViewAttr<visibility>> {
        self.member(ViewAttr::<visibility>(Visibility::Hidden))
    }
    fn flex(self) -> Self::AddMember<ViewAttr<display>> {
        self.member(ViewAttr::<display>(Display::Flex))
    }
    fn flex_col(self) -> Self::AddMember<(ViewAttr<display>, ViewAttr<flex_direction>)> {
        self.member((
            ViewAttr::<display>(Display::Flex),
            ViewAttr::<flex_direction>(FlexDirection::Column),
        ))
    }
    fn flex_row(self) -> Self::AddMember<(ViewAttr<display>, ViewAttr<flex_direction>)> {
        self.member((
            ViewAttr::<display>(Display::Flex),
            ViewAttr::<flex_direction>(FlexDirection::Row),
        ))
    }
    fn grid(self) -> Self::AddMember<ViewAttr<display>> {
        self.member(ViewAttr::<display>(Display::Grid))
    }
    fn shrink(self) -> Self::AddMember<ViewAttr<flex_shrink>> {
        self.member(ViewAttr::<flex_shrink>(1.0))
    }
    fn shrink_0(self) -> Self::AddMember<ViewAttr<flex_shrink>> {
        self.member(ViewAttr::<flex_shrink>(0.0))
    }
    fn grow(self) -> Self::AddMember<ViewAttr<flex_grow>> {
        self.member(ViewAttr::<flex_grow>(1.0))
    }
    fn grow_0(self) -> Self::AddMember<ViewAttr<flex_grow>> {
        self.member(ViewAttr::<flex_grow>(0.0))
    }

    fn justify_start(self) -> Self::AddMember<ViewAttr<justify_content>> {
        self.member(ViewAttr::<justify_content>(JustifyContent::Start))
    }
    fn justify_end(self) -> Self::AddMember<ViewAttr<justify_content>> {
        self.member(ViewAttr::<justify_content>(JustifyContent::End))
    }
    fn justify_center(self) -> Self::AddMember<ViewAttr<justify_content>> {
        self.member(ViewAttr::<justify_content>(JustifyContent::Center))
    }
    fn justify_between(self) -> Self::AddMember<ViewAttr<justify_content>> {
        self.member(ViewAttr::<justify_content>(JustifyContent::SpaceBetween))
    }
    fn justify_around(self) -> Self::AddMember<ViewAttr<justify_content>> {
        self.member(ViewAttr::<justify_content>(JustifyContent::SpaceAround))
    }
    fn justify_evenly(self) -> Self::AddMember<ViewAttr<justify_content>> {
        self.member(ViewAttr::<justify_content>(JustifyContent::SpaceEvenly))
    }
    fn items_start(self) -> Self::AddMember<ViewAttr<align_items>> {
        self.member(ViewAttr::<align_items>(AlignItems::FlexStart))
    }
    fn items_end(self) -> Self::AddMember<ViewAttr<align_items>> {
        self.member(ViewAttr::<align_items>(AlignItems::FlexEnd))
    }
    fn items_center(self) -> Self::AddMember<ViewAttr<align_items>> {
        self.member(ViewAttr::<align_items>(AlignItems::Center))
    }
    fn items_baseline(self) -> Self::AddMember<ViewAttr<align_items>> {
        self.member(ViewAttr::<align_items>(AlignItems::Baseline))
    }
    fn items_stretch(self) -> Self::AddMember<ViewAttr<align_items>> {
        self.member(ViewAttr::<align_items>(AlignItems::Stretch))
    }
    fn gap<T: Into<BevyWrapper<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(ViewAttr<column_gap>, ViewAttr<row_gap>)> {
        let value = value.into().0;
        self.member((ViewAttr::<column_gap>(value), ViewAttr::<row_gap>(value)))
    }
    fn gap_x<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<ViewAttr<row_gap>> {
        let value = value.into().0;
        self.member(ViewAttr::<row_gap>(value))
    }
    fn gap_y<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<ViewAttr<column_gap>> {
        let value = value.into().0;
        self.member(ViewAttr::<column_gap>(value))
    }
    fn relative(self) -> Self::AddMember<ViewAttr<position_type>> {
        self.member(ViewAttr::<position_type>(PositionType::Relative))
    }
    fn absolute(self) -> Self::AddMember<ViewAttr<position_type>> {
        self.member(ViewAttr::<position_type>(PositionType::Absolute))
    }
    fn hidden(self) -> Self::AddMember<ViewAttr<display>> {
        self.member(ViewAttr::<display>(Display::None))
    }

    fn flex_wrap(self) -> Self::AddMember<ViewAttr<flex_wrap>> {
        self.member(ViewAttr::<flex_wrap>(FlexWrap::Wrap))
    }

    fn flex_wrap_reverse(self) -> Self::AddMember<ViewAttr<flex_wrap>> {
        self.member(ViewAttr::<flex_wrap>(FlexWrap::WrapReverse))
    }
    fn flex_nowrap(self) -> Self::AddMember<ViewAttr<flex_wrap>> {
        self.member(ViewAttr::<flex_wrap>(FlexWrap::NoWrap))
    }
    fn w<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<ViewAttr<width>> {
        let value = value.into().0;
        self.width(value)
    }
    fn h<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<ViewAttr<height>> {
        let value = value.into().0;
        self.height(value)
    }

    fn min_w<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<ViewAttr<min_width>> {
        let value = value.into().0;
        self.min_width(value)
    }

    fn max_w<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<ViewAttr<max_width>> {
        let value = value.into().0;
        self.max_width(value)
    }

    fn min_h<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<ViewAttr<min_height>> {
        let value = value.into().0;
        self.min_height(value)
    }

    fn max_h<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<ViewAttr<max_height>> {
        let value = value.into().0;
        self.max_height(value)
    }

    fn w_screen(self) -> Self::AddMember<ViewAttr<width>> {
        self.member(ViewAttr::<width>(Val::Vw(100.)))
    }
    fn h_screen(self) -> Self::AddMember<ViewAttr<height>> {
        self.member(ViewAttr::<height>(Val::Vh(100.)))
    }

    fn size_screen(self) -> Self::AddMember<(ViewAttr<width>, ViewAttr<height>)> {
        self.member((
            ViewAttr::<width>(Val::Vw(100.)),
            ViewAttr::<height>(Val::Vh(100.)),
        ))
    }

    fn h_full(self) -> Self::AddMember<ViewAttr<height>> {
        self.member(ViewAttr::<height>(Val::Percent(100.)))
    }

    fn w_full(self) -> Self::AddMember<ViewAttr<width>> {
        self.member(ViewAttr::<width>(Val::Percent(100.)))
    }

    fn text_nowrap(self) -> Self::AddMember<ViewAttr<text_linebreak>> {
        self.member(ViewAttr::<text_linebreak>(BreakLineOn::NoWrap))
    }
    fn text_left(self) -> Self::AddMember<ViewAttr<text_align>> {
        self.member(ViewAttr::<text_align>(TextAlignment::Left))
    }
    fn text_center(self) -> Self::AddMember<ViewAttr<text_align>> {
        self.member(ViewAttr::<text_align>(TextAlignment::Center))
    }
    fn text_right(self) -> Self::AddMember<ViewAttr<text_align>> {
        self.member(ViewAttr::<text_align>(TextAlignment::Right))
    }

    fn size<T: Into<BevyWrapper<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(ViewAttr<width>, ViewAttr<height>)> {
        let value = value.into().0;
        self.member((ViewAttr::<width>(value), ViewAttr::<height>(value)))
    }

    fn center(self) -> Self::AddMember<(ViewAttr<align_items>, ViewAttr<justify_content>)> {
        self.member((
            ViewAttr::<align_items>(AlignItems::Center),
            ViewAttr::<justify_content>(JustifyContent::Center),
        ))
    }

    fn overflow<T: Into<BevyWrapper<OverflowAxis>>>(
        self,
        value: T,
    ) -> Self::AddMember<(ViewAttr<overflow_x>, ViewAttr<overflow_y>)> {
        let value = value.into().0;
        self.member((ViewAttr::<overflow_x>(value), ViewAttr::<overflow_y>(value)))
    }

    fn pt<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<ViewAttr<padding_top>> {
        let value = value.into().0;
        self.member(ViewAttr::<padding_top>(value))
    }

    fn pb<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<ViewAttr<padding_bottom>> {
        let value = value.into().0;
        self.member(ViewAttr::<padding_bottom>(value))
    }

    fn pl<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<ViewAttr<padding_left>> {
        let value = value.into().0;
        self.member(ViewAttr::<padding_left>(value))
    }

    fn pr<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<ViewAttr<padding_right>> {
        let value = value.into().0;
        self.member(ViewAttr::<padding_right>(value))
    }

    fn px<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<(ViewAttr<padding_left>, ViewAttr<padding_right>)> {
        let value = value.into().0;
        self.padding_horizontal(value)
    }

    fn py<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<(ViewAttr<padding_top>, ViewAttr<padding_bottom>)> {
        let value = value.into().0;
        self.padding_vertical(value)
    }

    fn p<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<(ViewAttr<padding_left>, ViewAttr<padding_right>, ViewAttr<padding_top>, ViewAttr<padding_bottom>)> {
        let value = value.into().0;
        self.padding(value)
    }

    fn mt<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<ViewAttr<margin_top>> {
        let value = value.into().0;
        self.member(ViewAttr::<margin_top>(value))
    }

    fn mb<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<ViewAttr<margin_bottom>> {
        let value = value.into().0;
        self.member(ViewAttr::<margin_bottom>(value))
    }

    fn ml<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<ViewAttr<margin_left>> {
        let value = value.into().0;
        self.member(ViewAttr::<margin_left>(value))
    }

    fn mr<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<ViewAttr<margin_right>> {
        let value = value.into().0;
        self.member(ViewAttr::<margin_right>(value))
    }

    fn mx<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<(ViewAttr<margin_left>, ViewAttr<margin_right>)> {
        let value = value.into().0;
        self.margin_horizontal(value)
    }

    fn my<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<(ViewAttr<margin_top>, ViewAttr<margin_bottom>)> {
        let value = value.into().0;
        self.margin_vertical(value)
    }

    fn m<T: Into<BevyWrapper<Val>>>(self, value: T) -> Self::AddMember<(ViewAttr<margin_left>, ViewAttr<margin_right>, ViewAttr<margin_top>, ViewAttr<margin_bottom>)> {
        let value = value.into().0;
        self.margin(value)
    }

    fn z<T: Into<BevyWrapper<ZIndex>>>(self, value: T) -> Self::AddMember<ViewAttr<z_index>> {
        let value = value.into().0;
        self.z_index(value)
    }
}

impl<T> TailwindAttrs for T where T: MemberOwner<BevyRenderer> {}
