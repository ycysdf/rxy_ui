use bevy_render::view::Visibility;
use bevy_text::{BreakLineOn, JustifyText};
use bevy_ui::{AlignItems, Display, FlexDirection, FlexWrap, JustifyContent, PositionType, Val};
use rxy_core::{impl_tailwind_attrs, impl_tailwind_attrs_use, StaticElementAttr};

use crate::BevyRenderer;

impl_tailwind_attrs_use!();
impl_tailwind_attrs!(BevyRenderer;MemberOwnerTailwindAttrs;MemberOwner;include_text_and_z_index);
impl_tailwind_attrs!(BevyRenderer;ElementViewTailwindAttrs;ElementView;include_text_and_z_index);
