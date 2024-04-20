use rxy_core::{impl_composite_attrs, impl_composite_attrs_use};
use crate::NativeRenderer;

impl_composite_attrs_use!();
impl_composite_attrs!(NativeRenderer;MemberOwnerCompositeAttrs;MemberOwner);
impl_composite_attrs!(NativeRenderer;ElementViewCompositeAttrs;ElementView);
