use crate::{BevyRenderer, BevyWrapper, ViewAttr};
use bevy_transform::components::Transform;
use bevy_ui::Val;
use rxy_bevy_element::all_attrs::*;
use rxy_core::MemberOwner;

pub trait CompositeAttrs: MemberOwner<BevyRenderer> + Sized {
    #[inline(always)]
    fn border<T: Into<BevyWrapper<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(
        ViewAttr<border_left>,
        ViewAttr<border_right>,
        ViewAttr<border_top>,
        ViewAttr<border_bottom>,
    )> {
        let value = value.into().0;
        self.member((
            ViewAttr::<border_left>(value),
            ViewAttr::<border_right>(value),
            ViewAttr::<border_top>(value),
            ViewAttr::<border_bottom>(value),
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
    fn margin_horizontal<T: Into<BevyWrapper<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(ViewAttr<margin_left>, ViewAttr<margin_right>)> {
        let value = value.into().0;
        self.member((
            ViewAttr::<margin_left>(value),
            ViewAttr::<margin_right>(value),
        ))
    }

    #[inline(always)]
    fn margin_vertical<T: Into<BevyWrapper<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(ViewAttr<margin_top>, ViewAttr<margin_bottom>)> {
        let value = value.into().0;
        self.member((
            ViewAttr::<margin_top>(value),
            ViewAttr::<margin_bottom>(value),
        ))
    }

    #[inline(always)]
    fn margin<T: Into<BevyWrapper<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(
        ViewAttr<margin_left>,
        ViewAttr<margin_right>,
        ViewAttr<margin_top>,
        ViewAttr<margin_bottom>,
    )> {
        let value = value.into().0;
        self.member((
            ViewAttr::<margin_left>(value),
            ViewAttr::<margin_right>(value),
            ViewAttr::<margin_top>(value),
            ViewAttr::<margin_bottom>(value),
        ))
    }

    #[inline(always)]
    fn padding_horizontal<T: Into<BevyWrapper<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(ViewAttr<padding_left>, ViewAttr<padding_right>)> {
        let value = value.into().0;
        self.member((
            ViewAttr::<padding_left>(value),
            ViewAttr::<padding_right>(value),
        ))
    }

    #[inline(always)]
    fn padding_vertical<T: Into<BevyWrapper<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(ViewAttr<padding_top>, ViewAttr<padding_bottom>)> {
        let value = value.into().0;
        self.member((
            ViewAttr::<padding_top>(value),
            ViewAttr::<padding_bottom>(value),
        ))
    }

    #[inline(always)]
    fn padding<T: Into<BevyWrapper<Val>>>(
        self,
        value: T,
    ) -> Self::AddMember<(
        ViewAttr<padding_left>,
        ViewAttr<padding_right>,
        ViewAttr<padding_top>,
        ViewAttr<padding_bottom>,
    )> {
        let value = value.into().0;
        self.member((
            ViewAttr::<padding_left>(value),
            ViewAttr::<padding_right>(value),
            ViewAttr::<padding_top>(value),
            ViewAttr::<padding_bottom>(value),
        ))
    }
}

impl<T> CompositeAttrs for T where T: MemberOwner<BevyRenderer> {}
