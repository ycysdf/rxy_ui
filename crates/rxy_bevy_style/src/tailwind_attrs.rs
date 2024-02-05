use bevy_render::prelude::Visibility;
use bevy_text::{BreakLineOn, TextAlignment};
use bevy_ui::{AlignItems, Display, FlexDirection, FlexWrap, JustifyContent, OverflowAxis, PositionType, Val};
use rxy_bevy::all_attrs::{
    align_items, column_gap, display, flex_direction, flex_grow, flex_shrink, flex_wrap, height,
    justify_content, margin_bottom, margin_left, margin_right, margin_top, max_height, max_width,
    min_height, min_width, overflow_x, overflow_y, padding_bottom, padding_left, padding_right,
    padding_top, position_type, row_gap, text_align, text_linebreak, visibility, width, z_index,
    CommonAttrsViewBuilder,
};
use rxy_bevy::into_attr_value::BevyAttrValue;
use rxy_bevy::BevyRenderer;
use rxy_core::{
    ElementAttr, ElementAttrMember, ElementAttrViewMember, IntoViewMember, IntoViewMemberWrapper,
    MemberOwner,
};

pub trait TailwindAttrs: MemberOwner<BevyRenderer> + Sized {
    fn visible(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, visibility>> {
        self.member(ElementAttrViewMember(Visibility::Visible.into()))
    }
    fn invisible(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, visibility>> {
        self.member(ElementAttrViewMember(Visibility::Hidden.into()))
    }
    fn flex(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, display>> {
        self.member(ElementAttrViewMember(Display::Flex.into()))
    }
    fn flex_col(
        self,
    ) -> Self::AddMember<(
        ElementAttrViewMember<BevyRenderer, display>,
        ElementAttrViewMember<BevyRenderer, flex_direction>,
    )> {
        self.member((
            ElementAttrViewMember(Display::Flex.into()),
            ElementAttrViewMember(FlexDirection::Column.into()),
        ))
    }
    fn flex_row(
        self,
    ) -> Self::AddMember<(
        ElementAttrViewMember<BevyRenderer, display>,
        ElementAttrViewMember<BevyRenderer, flex_direction>,
    )> {
        self.member((
            ElementAttrViewMember(Display::Flex.into()),
            ElementAttrViewMember(FlexDirection::Row.into()),
        ))
    }
    fn grid(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, display>> {
        self.member(ElementAttrViewMember(Display::Grid.into()))
    }
    fn shrink(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, flex_shrink>> {
        self.member(ElementAttrViewMember(1.0.into()))
    }
    fn shrink_0(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, flex_shrink>> {
        self.member(ElementAttrViewMember(0.0.into()))
    }
    fn grow(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, flex_grow>> {
        self.member(ElementAttrViewMember(1.0.into()))
    }
    fn grow_0(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, flex_grow>> {
        self.member(ElementAttrViewMember(0.0.into()))
    }

    fn justify_start(
        self,
    ) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, justify_content>> {
        self.member(ElementAttrViewMember(JustifyContent::Start.into()))
    }
    fn justify_end(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, justify_content>> {
        self.member(ElementAttrViewMember(JustifyContent::End.into()))
    }
    fn justify_center(
        self,
    ) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, justify_content>> {
        self.member(ElementAttrViewMember(JustifyContent::Center.into()))
    }
    fn justify_between(
        self,
    ) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, justify_content>> {
        self.member(ElementAttrViewMember(JustifyContent::SpaceBetween.into()))
    }
    fn justify_around(
        self,
    ) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, justify_content>> {
        self.member(ElementAttrViewMember(JustifyContent::SpaceAround.into()))
    }
    fn justify_evenly(
        self,
    ) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, justify_content>> {
        self.member(ElementAttrViewMember(JustifyContent::SpaceEvenly.into()))
    }
    fn items_start(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, align_items>> {
        self.member(ElementAttrViewMember(AlignItems::FlexStart.into()))
    }
    fn items_end(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, align_items>> {
        self.member(ElementAttrViewMember(AlignItems::FlexEnd.into()))
    }
    fn items_center(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, align_items>> {
        self.member(ElementAttrViewMember(AlignItems::Center.into()))
    }
    fn items_baseline(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, align_items>> {
        self.member(ElementAttrViewMember(AlignItems::Baseline.into()))
    }
    fn items_stretch(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, align_items>> {
        self.member(ElementAttrViewMember(AlignItems::Stretch.into()))
    }

    fn gap<T, EA>(
        self,
        value: impl IntoViewMember<BevyRenderer, T>,
    ) -> Self::AddMember<(T::Attr<column_gap>, T::Attr<row_gap>)>
    where
        T: ElementAttrMember<BevyRenderer, EA = EA> + Clone,
        EA: ElementAttr<BevyRenderer, Value = BevyAttrValue<Val>>,
    {
        let member = value.into_member();
        self.member(IntoViewMemberWrapper((
            member.clone().into_other_attr(),
            member.into_other_attr(),
        )))
    }

    fn gap_x<T, EA>(
        self,
        value: impl IntoViewMember<BevyRenderer, T>,
    ) -> Self::AddMember<T::Attr<column_gap>>
    where
        T: ElementAttrMember<BevyRenderer, EA = EA>,
        EA: ElementAttr<BevyRenderer, Value = BevyAttrValue<Val>>,
    {
        self.member(IntoViewMemberWrapper(value.into_member().into_other_attr()))
    }

    fn gap_y<T, EA>(
        self,
        value: impl IntoViewMember<BevyRenderer, T>,
    ) -> Self::AddMember<T::Attr<row_gap>>
    where
        T: ElementAttrMember<BevyRenderer, EA = EA>,
        EA: ElementAttr<BevyRenderer, Value = BevyAttrValue<Val>>,
    {
        self.member(IntoViewMemberWrapper(value.into_member().into_other_attr()))
    }
    fn relative(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, position_type>> {
        self.member(ElementAttrViewMember(PositionType::Relative.into()))
    }
    fn absolute(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, position_type>> {
        self.member(ElementAttrViewMember(PositionType::Absolute.into()))
    }
    fn hidden(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, display>> {
        self.member(ElementAttrViewMember(Display::None.into()))
    }

    fn flex_wrap(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, flex_wrap>> {
        self.member(ElementAttrViewMember(FlexWrap::Wrap.into()))
    }

    fn flex_wrap_reverse(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, flex_wrap>> {
        self.member(ElementAttrViewMember(FlexWrap::WrapReverse.into()))
    }
    fn flex_nowrap(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, flex_wrap>> {
        self.member(ElementAttrViewMember(FlexWrap::NoWrap.into()))
    }

    fn w<T, EA>(self, value: impl IntoViewMember<BevyRenderer, T>) -> Self::AddMember<T>
    where
        T: ElementAttrMember<BevyRenderer, EA = width>,
    {
        self.width(value)
    }

    fn h<T, EA>(self, value: impl IntoViewMember<BevyRenderer, T>) -> Self::AddMember<T>
    where
        T: ElementAttrMember<BevyRenderer, EA = height>,
    {
        self.height(value)
    }

    fn min_w<T, EA>(self, value: impl IntoViewMember<BevyRenderer, T>) -> Self::AddMember<T>
    where
        T: ElementAttrMember<BevyRenderer, EA = min_width>,
    {
        self.min_width(value)
    }

    fn max_w<T, EA>(self, value: impl IntoViewMember<BevyRenderer, T>) -> Self::AddMember<T>
    where
        T: ElementAttrMember<BevyRenderer, EA = max_width>,
    {
        self.max_width(value)
    }

    fn min_h<T, EA>(self, value: impl IntoViewMember<BevyRenderer, T>) -> Self::AddMember<T>
    where
        T: ElementAttrMember<BevyRenderer, EA = min_height>,
    {
        self.min_height(value)
    }
    fn max_h<T, EA>(self, value: impl IntoViewMember<BevyRenderer, T>) -> Self::AddMember<T>
    where
        T: ElementAttrMember<BevyRenderer, EA = max_height>,
    {
        self.max_height(value)
    }
    fn w_screen(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, width>> {
        self.member(ElementAttrViewMember(Val::Vw(100.).into()))
    }
    fn h_screen(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, height>> {
        self.member(ElementAttrViewMember(Val::Vh(100.).into()))
    }

    fn size_screen(
        self,
    ) -> Self::AddMember<(
        ElementAttrViewMember<BevyRenderer, width>,
        ElementAttrViewMember<BevyRenderer, height>,
    )> {
        self.member((
            ElementAttrViewMember(Val::Vw(100.).into()),
            ElementAttrViewMember(Val::Vh(100.).into()),
        ))
    }

    fn h_full(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, height>> {
        self.member(ElementAttrViewMember(Val::Percent(100.).into()))
    }

    fn w_full(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, width>> {
        self.member(ElementAttrViewMember(Val::Percent(100.).into()))
    }

    fn text_nowrap(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, text_linebreak>> {
        self.member(ElementAttrViewMember(BreakLineOn::NoWrap.into()))
    }
    fn text_left(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, text_align>> {
        self.member(ElementAttrViewMember(TextAlignment::Left.into()))
    }
    fn text_center(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, text_align>> {
        self.member(ElementAttrViewMember(TextAlignment::Center.into()))
    }
    fn text_right(self) -> Self::AddMember<ElementAttrViewMember<BevyRenderer, text_align>> {
        self.member(ElementAttrViewMember(TextAlignment::Right.into()))
    }

    fn size<T, EA>(
        self,
        value: impl IntoViewMember<BevyRenderer, T>,
    ) -> Self::AddMember<(T::Attr<width>, T::Attr<height>)>
    where
        T: ElementAttrMember<BevyRenderer, EA = EA> + Clone,
        EA: ElementAttr<BevyRenderer, Value = BevyAttrValue<Val>>,
    {
        let member = value.into_member();
        self.member(IntoViewMemberWrapper((
            member.clone().into_other_attr(),
            member.into_other_attr(),
        )))
    }

    fn center(
        self,
    ) -> Self::AddMember<(
        ElementAttrViewMember<BevyRenderer, align_items>,
        ElementAttrViewMember<BevyRenderer, justify_content>,
    )> {
        self.member((
            ElementAttrViewMember(AlignItems::Center.into()),
            ElementAttrViewMember(JustifyContent::Center.into()),
        ))
    }

    fn overflow<T, EA>(
        self,
        value: impl IntoViewMember<BevyRenderer, T>,
    ) -> Self::AddMember<(T::Attr<overflow_x>, T::Attr<overflow_y>)>
    where
        T: ElementAttrMember<BevyRenderer, EA = EA> + Clone,
        EA: ElementAttr<BevyRenderer, Value = BevyAttrValue<OverflowAxis>>,
    {
        let member = value.into_member();
        self.member(IntoViewMemberWrapper((
            member.clone().into_other_attr(),
            member.into_other_attr(),
        )))
    }

    fn pt<T, EA>(self, value: impl IntoViewMember<BevyRenderer, T>) -> Self::AddMember<T>
    where
        T: ElementAttrMember<BevyRenderer, EA = padding_top>,
    {
        self.padding_top(value)
    }

    fn pb<T, EA>(self, value: impl IntoViewMember<BevyRenderer, T>) -> Self::AddMember<T>
    where
        T: ElementAttrMember<BevyRenderer, EA = padding_bottom>,
    {
        self.padding_bottom(value)
    }

    fn pl<T, EA>(self, value: impl IntoViewMember<BevyRenderer, T>) -> Self::AddMember<T>
    where
        T: ElementAttrMember<BevyRenderer, EA = padding_left>,
    {
        self.padding_left(value)
    }

    fn pr<T, EA>(self, value: impl IntoViewMember<BevyRenderer, T>) -> Self::AddMember<T>
    where
        T: ElementAttrMember<BevyRenderer, EA = padding_right>,
    {
        self.padding_right(value)
    }

    fn px<T, EA>(
        self,
        value: impl IntoViewMember<BevyRenderer, T>,
    ) -> Self::AddMember<(T::Attr<padding_left>, T::Attr<padding_right>)>
    where
        T: ElementAttrMember<BevyRenderer, EA = EA> + Clone,
        EA: ElementAttr<BevyRenderer, Value = BevyAttrValue<Val>>,
    {
        let member = value.into_member();
        self.member(IntoViewMemberWrapper((
            member.clone().into_other_attr(),
            member.into_other_attr(),
        )))
    }
    fn py<T, EA>(
        self,
        value: impl IntoViewMember<BevyRenderer, T>,
    ) -> Self::AddMember<(T::Attr<padding_top>, T::Attr<padding_bottom>)>
    where
        T: ElementAttrMember<BevyRenderer, EA = EA> + Clone,
        EA: ElementAttr<BevyRenderer, Value = BevyAttrValue<Val>>,
    {
        let member = value.into_member();
        self.member(IntoViewMemberWrapper((
            member.clone().into_other_attr(),
            member.into_other_attr(),
        )))
    }

    fn p<T, EA>(
        self,
        value: impl IntoViewMember<BevyRenderer, T>,
    ) -> Self::AddMember<(
        T::Attr<padding_left>,
        T::Attr<padding_right>,
        T::Attr<padding_top>,
        T::Attr<padding_bottom>,
    )>
    where
        T: ElementAttrMember<BevyRenderer, EA = EA> + Clone,
        EA: ElementAttr<BevyRenderer, Value = BevyAttrValue<Val>>,
    {
        let member = value.into_member();
        self.member(IntoViewMemberWrapper((
            member.clone().into_other_attr(),
            member.clone().into_other_attr(),
            member.clone().into_other_attr(),
            member.into_other_attr(),
        )))
    }

    fn mt<T, EA>(self, value: impl IntoViewMember<BevyRenderer, T>) -> Self::AddMember<T>
    where
        T: ElementAttrMember<BevyRenderer, EA = margin_top>,
    {
        self.margin_top(value)
    }

    fn mb<T, EA>(self, value: impl IntoViewMember<BevyRenderer, T>) -> Self::AddMember<T>
    where
        T: ElementAttrMember<BevyRenderer, EA = margin_bottom>,
    {
        self.margin_bottom(value)
    }

    fn ml<T, EA>(self, value: impl IntoViewMember<BevyRenderer, T>) -> Self::AddMember<T>
    where
        T: ElementAttrMember<BevyRenderer, EA = margin_left>,
    {
        self.margin_left(value)
    }

    fn mr<T, EA>(self, value: impl IntoViewMember<BevyRenderer, T>) -> Self::AddMember<T>
    where
        T: ElementAttrMember<BevyRenderer, EA = margin_right>,
    {
        self.margin_right(value)
    }

    fn mx<T, EA>(
        self,
        value: impl IntoViewMember<BevyRenderer, T>,
    ) -> Self::AddMember<(T::Attr<margin_left>, T::Attr<margin_right>)>
    where
        T: ElementAttrMember<BevyRenderer, EA = EA> + Clone,
        EA: ElementAttr<BevyRenderer, Value = BevyAttrValue<Val>>,
    {
        let member = value.into_member();
        self.member(IntoViewMemberWrapper((
            member.clone().into_other_attr(),
            member.into_other_attr(),
        )))
    }

    fn my<T, EA>(
        self,
        value: impl IntoViewMember<BevyRenderer, T>,
    ) -> Self::AddMember<(T::Attr<margin_top>, T::Attr<margin_bottom>)>
    where
        T: ElementAttrMember<BevyRenderer, EA = EA> + Clone,
        EA: ElementAttr<BevyRenderer, Value = BevyAttrValue<Val>>,
    {
        let member = value.into_member();
        self.member(IntoViewMemberWrapper((
            member.clone().into_other_attr(),
            member.into_other_attr(),
        )))
    }

    fn m<T, EA>(
        self,
        value: impl IntoViewMember<BevyRenderer, T>,
    ) -> Self::AddMember<(
        T::Attr<margin_left>,
        T::Attr<margin_right>,
        T::Attr<margin_top>,
        T::Attr<margin_bottom>,
    )>
    where
        T: ElementAttrMember<BevyRenderer, EA = EA> + Clone,
        EA: ElementAttr<BevyRenderer, Value = BevyAttrValue<Val>>,
    {
        let member = value.into_member();
        self.member(IntoViewMemberWrapper((
            member.clone().into_other_attr(),
            member.clone().into_other_attr(),
            member.clone().into_other_attr(),
            member.into_other_attr(),
        )))
    }

    fn z<T, EA>(self, value: impl IntoViewMember<BevyRenderer, T>) -> Self::AddMember<T>
    where
        T: ElementAttrMember<BevyRenderer, EA = z_index>,
    {
        self.z_index(value)
    }
}
impl<T> TailwindAttrs for T where T: MemberOwner<BevyRenderer> {}
