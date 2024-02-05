use crate::all_attrs::{
    border_bottom, border_left, border_right, border_top, margin_bottom, margin_left, margin_right,
    margin_top, padding_bottom, padding_left, padding_right, padding_top, rotation, scale,
    translation,
};
use crate::into_attr_value::BevyAttrValue;
use crate::{BevyRenderer, BevyWrapper};
use bevy_transform::components::Transform;
use bevy_ui::Val;
use rxy_core::{
    ElementAttr, ElementAttrMember, IntoViewMember, IntoViewMemberWrapper, MemberOwner,
};

pub trait CompositeAttrs: MemberOwner<BevyRenderer> + Sized {
    fn border_x<T, EA>(
        self,
        value: impl IntoViewMember<BevyRenderer, T>,
    ) -> Self::AddMember<(T::Attr<border_left>, T::Attr<border_right>)>
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

    fn border_y<T, EA>(
        self,
        value: impl IntoViewMember<BevyRenderer, T>,
    ) -> Self::AddMember<(T::Attr<border_top>, T::Attr<border_bottom>)>
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

    fn border<T, EA>(
        self,
        value: impl IntoViewMember<BevyRenderer, T>,
    ) -> Self::AddMember<(
        T::Attr<border_left>,
        T::Attr<border_right>,
        T::Attr<border_top>,
        T::Attr<border_bottom>,
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

    fn margin_horizontal<T, EA>(
        self,
        value: T,
    ) -> Self::AddMember<(T::Attr<margin_left>, T::Attr<margin_right>)>
    where
        T: ElementAttrMember<BevyRenderer, EA = EA> + Clone,
        EA: ElementAttr<BevyRenderer, Value = BevyAttrValue<Val>>,
    {
        self.member(IntoViewMemberWrapper((
            value.clone().into_other_attr::<margin_left>(),
            value.into_other_attr::<margin_right>(),
        )))
    }

    fn margin_vertical<T, EA>(
        self,
        value: T,
    ) -> Self::AddMember<(T::Attr<margin_top>, T::Attr<margin_bottom>)>
    where
        T: ElementAttrMember<BevyRenderer, EA = EA> + Clone,
        EA: ElementAttr<BevyRenderer, Value = BevyAttrValue<Val>>,
    {
        self.member(IntoViewMemberWrapper((
            value.clone().into_other_attr::<margin_top>(),
            value.into_other_attr::<margin_bottom>(),
        )))
    }

    fn margin<T, EA>(
        self,
        value: T,
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
        self.member(IntoViewMemberWrapper((
            value.clone().into_other_attr::<margin_left>(),
            value.clone().into_other_attr::<margin_right>(),
            value.clone().into_other_attr::<margin_top>(),
            value.into_other_attr::<margin_bottom>(),
        )))
    }

    fn padding_horizontal<T, EA>(
        self,
        value: T,
    ) -> Self::AddMember<(T::Attr<padding_left>, T::Attr<padding_right>)>
    where
        T: ElementAttrMember<BevyRenderer, EA = EA> + Clone,
        EA: ElementAttr<BevyRenderer, Value = BevyAttrValue<Val>>,
    {
        self.member(IntoViewMemberWrapper((
            value.clone().into_other_attr::<padding_left>(),
            value.into_other_attr::<padding_right>(),
        )))
    }

    fn padding_vertical<T, EA>(
        self,
        value: T,
    ) -> Self::AddMember<(T::Attr<padding_top>, T::Attr<padding_bottom>)>
    where
        T: ElementAttrMember<BevyRenderer, EA = EA> + Clone,
        EA: ElementAttr<BevyRenderer, Value = BevyAttrValue<Val>>,
    {
        self.member(IntoViewMemberWrapper((
            value.clone().into_other_attr::<padding_top>(),
            value.into_other_attr::<padding_bottom>(),
        )))
    }

    fn padding<T, EA>(
        self,
        value: T,
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
        self.member(IntoViewMemberWrapper((
            value.clone().into_other_attr::<padding_left>(),
            value.clone().into_other_attr::<padding_right>(),
            value.clone().into_other_attr::<padding_top>(),
            value.into_other_attr::<padding_bottom>(),
        )))
    }
}

impl<T> CompositeAttrs for T where T: MemberOwner<BevyRenderer> {}
