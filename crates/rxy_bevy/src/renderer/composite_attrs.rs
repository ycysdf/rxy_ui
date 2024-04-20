use rxy_core::{impl_composite_attrs, impl_composite_attrs_use};
use crate::BevyRenderer;

impl_composite_attrs_use!();
impl_composite_attrs!(BevyRenderer;MemberOwnerCompositeAttrs;MemberOwner);
impl_composite_attrs!(BevyRenderer;ElementViewCompositeAttrs;ElementView);
