use bevy_reflect::{FromReflect, TypePath};

use crate::element_core::AttrValue;
use crate::smallbox::S1;
use crate::{
    AttrIndex, ElementEntityExtraData, ElementEntityWorldMutExt, SetAttrValueContext, SmallBox,
};
use bevy_ecs::reflect::AppTypeRegistry;
use bevy_ecs::world::EntityWorldMut;

pub trait HasIndex {
    const INDEX: AttrIndex;
}

pub trait ElementAttr: Send + Sync + 'static {
    type Value: AttrValue + Clone + Sized + FromReflect + TypePath;

    const NAME: &'static str;

    fn init(entity_world_mut: &mut EntityWorldMut, value: impl Into<Self::Value>) {
        let type_registry = entity_world_mut.world().resource::<AppTypeRegistry>().clone();
        let mut context = SetAttrValueContext {
            entity_mut: &mut entity_world_mut.as_entity_mut(),
            type_registry: &type_registry,
        };
        Self::set_value(&mut context, value)
    }

    fn set_value(context: &mut SetAttrValueContext, value: impl Into<Self::Value>);

    #[inline]
    fn set_dyn_value(context: &mut SetAttrValueContext, value: SmallBox<dyn AttrValue, S1>) {
        if let Ok(value) = value.downcast::<Self::Value>() {
            Self::set_value(context, value.into_inner());
        }
    }

    fn set_default_value(context: &mut SetAttrValueContext) {
        Self::set_value(context, Self::Value::default_value())
    }
}

pub trait ElementUnitAttrUntyped: Send + Sync {
    fn attr_name(&self) -> &'static str;

    fn index(&self) -> u8;

    fn default_value(&self) -> SmallBox<dyn AttrValue, S1>;

    fn set_dyn_value(&self, context: &mut SetAttrValueContext, value: SmallBox<dyn AttrValue, S1>);
    fn init(&self, entity_world_mut: &mut EntityWorldMut, value: SmallBox<dyn AttrValue, S1>);
    fn init_or_set(
        &self,
        entity_world_mut: &mut EntityWorldMut,
        value: Option<SmallBox<dyn AttrValue, S1>>,
    ) {
        let value = value.unwrap_or_else(|| self.default_value());
        let mut extra_data = entity_world_mut.get_mut::<ElementEntityExtraData>().unwrap();
        let is_init = extra_data.is_init_attr(self.index());
        if !is_init {
            extra_data.init_attr(self.index(), true);
        }
        if is_init {
            let type_registry = entity_world_mut.world().resource::<AppTypeRegistry>().clone();
            let mut context = SetAttrValueContext {
                entity_mut: &mut entity_world_mut.as_entity_mut(),
                type_registry: &type_registry,
            };
            self.set_dyn_value(&mut context, value);
        } else {
            self.init(entity_world_mut, value);
        }
    }
}

impl<T: ElementAttr + HasIndex> ElementUnitAttrUntyped for T {
    #[inline]
    fn attr_name(&self) -> &'static str {
        T::NAME
    }

    #[inline]
    fn index(&self) -> u8 {
        <T as HasIndex>::INDEX
    }

    #[inline]
    fn set_dyn_value(&self, context: &mut SetAttrValueContext, value: SmallBox<dyn AttrValue, S1>) {
        <T as ElementAttr>::set_dyn_value(context, value);
    }

    #[inline]
    fn init(&self, entity_world_mut: &mut EntityWorldMut, value: SmallBox<dyn AttrValue, S1>) {
        if let Ok(value) = value.downcast::<T::Value>() {
            T::init(entity_world_mut, value.into_inner())
        }
    }

    fn default_value(&self) -> SmallBox<dyn AttrValue, S1> {
        crate::smallbox!(T::Value::default_value())
    }
}

pub trait ElementUnitAttr: ElementAttr + HasIndex {}

impl<T: ElementAttr + HasIndex> ElementUnitAttr for T {}

pub enum SetAttrValueError {}
