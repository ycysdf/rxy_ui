use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;

use crate::element_core::ElementUnitAttrUntyped;
use crate::AttrIndex;

pub trait ElementTypeUnTyped: Reflect {
    fn tag_name(&self) -> &'static str;

    fn attrs(&self) -> &'static [&'static [&'static dyn ElementUnitAttrUntyped]];

    fn attr(&self, attr_name: &str) -> Option<&'static dyn ElementUnitAttrUntyped>;
    fn attr_by_index(&self, index: AttrIndex) -> &'static dyn ElementUnitAttrUntyped {
        let mut index = index as usize;
        for attrs in self.attrs() {
            if index < attrs.len() {
                return attrs[index];
            }
            index -= attrs.len();
        }
        unreachable!();
    }
    fn spawn(&self, entity_mut: &mut EntityWorldMut);
}

impl<T: ElementTypeBase + ElementType> ElementTypeUnTyped for T {
    #[inline]
    fn tag_name(&self) -> &'static str {
        T::TAG_NAME
    }

    #[inline]
    fn attrs(&self) -> &'static [&'static [&'static dyn ElementUnitAttrUntyped]] {
        T::ATTRS
    }

    #[inline]
    fn attr(&self, attr_name: &str) -> Option<&'static dyn ElementUnitAttrUntyped> {
        T::attr(attr_name)
    }

    #[inline]
    fn spawn<'w>(&self, entity_mut: &mut EntityWorldMut) {
        T::update_entity(entity_mut)
    }
}

pub trait ElementTypeBase: Reflect + FromReflect + TypePath {
    const TAG_NAME: &'static str;
    const ATTRS: &'static [&'static [&'static dyn ElementUnitAttrUntyped]];

    fn attr(attr_name: &str) -> Option<&'static dyn ElementUnitAttrUntyped> {
        use bevy_utils::HashMap;
        static ATTRS: std::sync::OnceLock<HashMap<&'static str, &'static dyn ElementUnitAttrUntyped>> =
            std::sync::OnceLock::new();
        let map = ATTRS.get_or_init(|| {
            let mut map: HashMap<&'static str, &'static dyn ElementUnitAttrUntyped> = HashMap::new();
            for attrs in Self::ATTRS {
                for attr in *attrs {
                    map.insert(attr.attr_name(), *attr);
                }
            }
            map
        });
        map.get(attr_name).copied()
    }
}

pub trait ElementType: ElementTypeBase {
    fn update_entity(entity_mut: &mut EntityWorldMut);
}
