use crate::{BevyRenderer, BevyWrapper, ElementAttrAgent, IntoViewAttrMember, ViewAttr};
use bevy_transform::components::Transform;
use bevy_ui::Val;
use rxy_bevy_element::all_attrs::*;
use rxy_core::MemberOwner;

pub trait CompositeAttrs: MemberOwner<BevyRenderer> + Sized {
    #[inline(always)]
    fn border_x<T: IntoViewAttrMember<ElementAttrAgent<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(T::OtherAttr<border_left>, T::OtherAttr<border_right>)>
    where
        T: Clone,
    {
        self.member((
            value.clone().into_other_attr::<border_left>(),
            value.into_other_attr::<border_right>(),
        ))
    }

    #[inline(always)]
    fn border_y<T: IntoViewAttrMember<ElementAttrAgent<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(T::OtherAttr<border_top>, T::OtherAttr<border_bottom>)>
    where
        T: Clone,
    {
        self.member((
            value.clone().into_other_attr::<border_top>(),
            value.into_other_attr::<border_bottom>(),
        ))
    }

    #[inline(always)]
    fn border<T: IntoViewAttrMember<ElementAttrAgent<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(
        T::OtherAttr<border_left>,
        T::OtherAttr<border_right>,
        T::OtherAttr<border_top>,
        T::OtherAttr<border_bottom>,
    )>
    where
        T: Clone,
    {
        self.member((
            value.clone().into_other_attr::<border_left>(),
            value.clone().into_other_attr::<border_right>(),
            value.clone().into_other_attr::<border_top>(),
            value.into_other_attr::<border_bottom>(),
        ))
    }

    #[inline(always)]
    fn transform<T: Into<BevyWrapper<Transform>>>(
        self,
        value: T,
    ) -> Self::AddMember<(ViewAttr<translation>, ViewAttr<rotation>, ViewAttr<scale>)> {
        let value = value.into().0;
        self.member((
            ViewAttr::<translation>(value.translation),
            ViewAttr::<rotation>(value.rotation),
            ViewAttr::<scale>(value.scale),
        ))
    }

    #[inline(always)]
    fn margin_horizontal<T: IntoViewAttrMember<ElementAttrAgent<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(T::OtherAttr<margin_left>, T::OtherAttr<margin_right>)>
    where
        T: Clone,
    {
        self.member((
            value.clone().into_other_attr::<margin_left>(),
            value.into_other_attr::<margin_right>(),
        ))
    }

    #[inline(always)]
    fn margin_vertical<T: IntoViewAttrMember<ElementAttrAgent<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(T::OtherAttr<margin_top>, T::OtherAttr<margin_bottom>)>
    where
        T: Clone,
    {
        self.member((
            value.clone().into_other_attr::<margin_top>(),
            value.into_other_attr::<margin_bottom>(),
        ))
    }

    #[inline(always)]
    fn margin<T: IntoViewAttrMember<ElementAttrAgent<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(
        T::OtherAttr<margin_left>,
        T::OtherAttr<margin_right>,
        T::OtherAttr<margin_top>,
        T::OtherAttr<margin_bottom>,
    )>
    where
        T: Clone,
    {
        self.member((
            value.clone().into_other_attr::<margin_left>(),
            value.clone().into_other_attr::<margin_right>(),
            value.clone().into_other_attr::<margin_top>(),
            value.into_other_attr::<margin_bottom>(),
        ))
    }

    #[inline(always)]
    fn padding_horizontal<T: IntoViewAttrMember<ElementAttrAgent<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(T::OtherAttr<padding_left>, T::OtherAttr<padding_right>)>
    where
        T: Clone,
    {
        self.member((
            value.clone().into_other_attr::<padding_left>(),
            value.into_other_attr::<padding_right>(),
        ))
    }

    #[inline(always)]
    fn padding_vertical<T: IntoViewAttrMember<ElementAttrAgent<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(T::OtherAttr<padding_top>, T::OtherAttr<padding_bottom>)>
    where
        T: Clone,
    {
        self.member((
            value.clone().into_other_attr::<padding_top>(),
            value.into_other_attr::<padding_bottom>(),
        ))
    }

    #[inline(always)]
    fn padding<T: IntoViewAttrMember<ElementAttrAgent<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(
        T::OtherAttr<padding_left>,
        T::OtherAttr<padding_right>,
        T::OtherAttr<padding_top>,
        T::OtherAttr<padding_bottom>,
    )>
    where
        T: Clone,
    {
        self.member((
            value.clone().into_other_attr::<padding_left>(),
            value.clone().into_other_attr::<padding_right>(),
            value.clone().into_other_attr::<padding_top>(),
            value.into_other_attr::<padding_bottom>(),
        ))
    }
}

impl<T> CompositeAttrs for T where T: MemberOwner<BevyRenderer> {}
