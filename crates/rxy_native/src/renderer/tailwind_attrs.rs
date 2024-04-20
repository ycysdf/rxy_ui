use crate::Visibility;
use crate::{AlignItems, Display, FlexDirection, FlexWrap, JustifyContent, PositionType, Val};
use rxy_core::{impl_tailwind_attrs, impl_tailwind_attrs_use};

use crate::NativeRenderer;

impl_tailwind_attrs_use!();
impl_tailwind_attrs!(NativeRenderer;MemberOwnerTailwindAttrs;MemberOwner);
impl_tailwind_attrs!(NativeRenderer;ElementViewTailwindAttrs;ElementView);
