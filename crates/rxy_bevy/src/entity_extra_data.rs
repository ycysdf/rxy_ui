use bevy_ecs::prelude::Component;

use rxy_core::{AttrIndex, ElementTypeUnTyped};
use crate::BevyRenderer;

pub type AttrSetBits = u64;
pub type AttrInitBits = u64;

#[derive(Component, Clone)]
pub struct ElementEntityExtraData {
    pub element_type: &'static dyn ElementTypeUnTyped<BevyRenderer>,
    pub attr_is_set: AttrSetBits,
    pub attr_is_init: AttrSetBits,
}

impl ElementEntityExtraData {
    pub fn new(element_type: &'static dyn ElementTypeUnTyped<BevyRenderer>) -> Self {
        Self {
            element_type,
            attr_is_set: 0,
            attr_is_init: 0,
        }
    }

    pub fn set_attr(&mut self, attr_index: AttrIndex, is_set: bool) {
        if attr_index == 0 {
            return;
        }
        if is_set {
            self.attr_is_set |= 1 << attr_index;
        } else {
            self.attr_is_set &= !(1 << attr_index);
        }
    }
    pub fn init_attr(&mut self, attr_index: AttrIndex, is_init: bool) {
        if attr_index == 0 {
            return;
        }
        if is_init {
            self.attr_is_init |= 1 << attr_index;
        } else {
            self.attr_is_init &= !(1 << attr_index);
        }
    }

    pub fn is_set_attr(&self, attr_index: AttrIndex) -> bool {
        Self::static_is_set_attr(self.attr_is_set, attr_index)
    }

    pub fn is_init_attr(&self, attr_index: AttrIndex) -> bool {
        (self.attr_is_init >> attr_index) & 1 == 1
    }

    pub fn static_is_set_attr(attr_is_set: AttrSetBits, attr_index: AttrIndex) -> bool {
        (attr_is_set >> attr_index) & 1 == 1
    }
}
