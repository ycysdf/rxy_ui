use crate::AttrValue;
use crate::ElementEntityExtraData;
use crate::ElementUnitAttr;
use crate::SetAttrValueContext;
use bevy_ecs::prelude::*;
use bevy_ui::Style;
use std::ops::DerefMut;

pub trait ElementStyleEntityExt {
    fn try_set_style(&mut self, set_f: impl FnOnce(&mut Style));
    fn try_set<T: Component>(&mut self, set_f: impl FnOnce(&mut T));
    fn get_element_extra_data_mut(&mut self) -> Option<Mut<'_, ElementEntityExtraData>>;
}

impl ElementStyleEntityExt for EntityMut<'_> {
    #[inline]
    fn try_set_style(&mut self, set_f: impl FnOnce(&mut Style)) {
        if let Some(mut style) = self.get_mut::<Style>() {
            set_f(style.deref_mut());
        }
    }
    #[inline]
    fn try_set<T: Component>(&mut self, set_f: impl FnOnce(&mut T)) {
        if let Some(mut component) = self.get_mut::<T>() {
            set_f(component.deref_mut());
        }
    }
    fn get_element_extra_data_mut(&mut self) -> Option<Mut<'_, ElementEntityExtraData>> {
        self.get_mut::<ElementEntityExtraData>()
    }
}
pub trait ElementEntityWorldMutExt {
    fn as_entity_mut(&mut self) -> EntityMut<'_>;
}

impl ElementEntityWorldMutExt for EntityWorldMut<'_> {
    fn as_entity_mut(&mut self) -> EntityMut<'_> {
        self.into()
    }
}

pub trait WorldViewAttrExt {
    fn build_attr<A: ElementUnitAttr>(&mut self, entity: Entity, value: A::Value);
    fn rebuild_attr<A: ElementUnitAttr>(&mut self, entity: Entity, value: A::Value);
    fn unbuild_attr<A: ElementUnitAttr>(&mut self, entity: Entity);
}

impl WorldViewAttrExt for World {
    fn build_attr<A: ElementUnitAttr>(&mut self, entity: Entity, value: A::Value) {
        let mut entity_world_mut = self.entity_mut(entity);
        A::init(&mut entity_world_mut, value);
        entity_world_mut
            .as_entity_mut()
            .get_element_extra_data_mut()
            .unwrap() // todo: error handle
            .set_attr(A::INDEX, true);
    }
    fn rebuild_attr<A: ElementUnitAttr>(&mut self, entity: Entity, value: A::Value) {
        let type_registry = self.resource::<AppTypeRegistry>().clone();
        let mut entity_world_mut = self.entity_mut(entity);
        let mut context = SetAttrValueContext {
            entity_mut: &mut entity_world_mut.as_entity_mut(),
            type_registry: &type_registry,
        };
        A::set_value(&mut context, value);
        entity_world_mut
            .as_entity_mut()
            .get_element_extra_data_mut()
            .unwrap() // todo: error handle
            .set_attr(A::INDEX, true);
    }

    fn unbuild_attr<A: ElementUnitAttr>(&mut self, entity: Entity) {
        let type_registry = self.resource::<AppTypeRegistry>().clone();
        let mut entity_mut = self.entity_mut(entity);
        let mut context = SetAttrValueContext {
            entity_mut: &mut entity_mut.as_entity_mut(),
            type_registry: &type_registry,
        };
        A::set_value(&mut context, A::Value::default_value());
        entity_mut
            .as_entity_mut()
            .get_element_extra_data_mut()
            .unwrap() // todo: error handle
            .set_attr(A::INDEX, false);
    }
}

/*pub fn dyn_to_owning_ptr(dyn_reflect: Box<dyn Any>) -> OwningPtr<'static> {
    let mut dyn_reflect = ManuallyDrop::new(dyn_reflect);
    let dyn_reflect = &mut **dyn_reflect;
    let ptr = dyn_reflect as *const dyn Any as *const ();

    let ptr = NonNull::<u8>::new(ptr as *mut u8).unwrap();
    unsafe { OwningPtr::new(ptr) }
}

pub trait ReflectExtension {
    fn clone_real_value(
        &self,
        type_registry: &TypeRegistry,
        type_id: TypeId,
    ) -> Option<Box<dyn Reflect>>;
}

impl ReflectExtension for dyn Reflect {
    fn clone_real_value(
        &self,
        type_registry: &TypeRegistry,
        type_id: TypeId,
    ) -> Option<Box<dyn Reflect>> {
        let from_reflect = type_registry.get_type_data::<ReflectFromReflect>(type_id)?;
        from_reflect.from_reflect(self)
    }
}
*/

/*pub fn empty_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::None,
            ..default()
        },
        ..default()
    }
}

pub fn default_clone_component(
    world: &World,
    type_registry: AppTypeRegistry,
    entity: Entity,
    component_info: &ComponentInfo,
) -> Option<Box<dyn Any>> {
    let component_type_id = component_info.type_id()?;

    let component_id = component_info.id();

    let type_id = world
        .components()
        .get_info(component_id)
        .and_then(|n| n.type_id())?;
    let component_ptr = world.get_by_id(entity, component_id)?;

    let type_registry = type_registry.read();
    let from_ptr = type_registry.get_type_data::<ReflectFromPtr>(type_id);
    if from_ptr.is_none() {
        warn!(
            "component {:?} no found ReflectFromPtr type data!",
            component_info.name()
        );
    }
    let from_ptr = from_ptr?;
    let reflect_obj = unsafe { from_ptr.as_reflect(component_ptr) };

    let reflect_obj = reflect_obj.clone_real_value(&type_registry, component_type_id)?;
    Some(reflect_obj.into_any())
}

fn clone_entity<'a>(
    world: &'a mut World,
    entities_extra_data: &'a mut ElementEntitiesExtraData,
    template_world: &mut World,
    template_entities_extra_data: &mut ElementEntitiesExtraData,
    template_entity: Entity,
) -> EntityWorldMut<'a> {
    let template_world = &*template_world;
    if template_entities_extra_data
        .empty_node_entities
        .contains(&template_entity)
    {
        return world.spawn((
            empty_node(),
            template_world
                .get::<Name>(template_entity)
                .cloned()
                .unwrap_or(Name::new("[Empty Node]")),
        ));
    }

    let entity_extra_data = template_entities_extra_data.get(&template_entity).cloned();
    let Some(entity_extra_data) = entity_extra_data else {
        error!("No Found Entity Extra Data : {:?}", template_entity);
        return world.spawn((
            empty_node(),
            template_world
                .get::<Name>(template_entity)
                .cloned()
                .unwrap_or(Name::new("[Empty Node] [Error]")),
        ));
    };
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    let schema_type = get_element_type(entity_extra_data.element_name);
    let ignore_type_ids = vec![TypeId::of::<Parent>(), TypeId::of::<Children>()];

    let mut components = vec![];
    // let mut uninited_components = vec![];
    let mut component_ids = vec![];
    let template_entity_ref = template_world.entity(template_entity);
    let component_infos = template_entity_ref
        .archetype()
        .components()
        .filter_map(|n| {
            let Some(component_info) = template_world.components().get_info(n) else {
                warn!("component_info no found by id {:?}!", n);
                return None;
            };
            let Some(component_type_id) = ComponentInfo::type_id(component_info) else {
                warn!("component {:#?} type_id is null!", component_info.name());
                return None;
            };
            if ignore_type_ids.contains(&component_type_id) {
                return None;
            }
            Some((component_info, world.components().get_id(component_type_id)))
        })
        .collect::<Vec<_>>();
    let mut loaded_entity = world.spawn_empty();

    for (component_info, component_id) in component_infos {
        if schema_type.try_insert_no_reflect_components(
            &mut loaded_entity,
            template_world,
            template_entity,
            type_registry.clone(),
            component_info,
        ) {
            continue;
        }
        let component_type_id = component_info.type_id().unwrap();
        let type_registry = type_registry.read();

        let reflect_obj = {
            let Some(component_ptr) = template_entity_ref.get_by_id(component_info.id()) else {
                warn!("component {:#?} no found!", component_info.name());
                continue;
            };

            let Some(from_ptr) = type_registry.get_type_data::<ReflectFromPtr>(component_type_id)
            else {
                warn!(
                    "component {:#?} get ReflectFromPtr type data failed!",
                    component_info.name()
                );
                continue;
            };
            unsafe { from_ptr.as_reflect(component_ptr) }
        };
        match component_id {
            None => {
                let Some(reflect_component) =
                    type_registry.get_type_data::<ReflectComponent>(component_type_id)
                else {
                    warn!(
                        "component {:#?} no found ReflectComponent",
                        component_info.name()
                    );
                    continue;
                };

                reflect_component.insert(&mut loaded_entity, reflect_obj)
            }
            Some(component_id) => {
                component_ids.push(component_id);
                components.push(
                    reflect_obj
                        .clone_real_value(&type_registry, component_type_id)
                        .unwrap()
                        .into_any(),
                );
            }
        }
    }

    unsafe {
        loaded_entity.insert_by_ids(
            component_ids.as_slice(),
            components.into_iter().map(dyn_to_owning_ptr),
        )
    };
    entities_extra_data.insert(loaded_entity.id(), entity_extra_data);
    loaded_entity
}

pub fn clone_entity_nest<'a>(
    world: &'a mut World,
    entities_extra_data: &'a mut ElementEntitiesExtraData,
    template_world: &'a mut World,
    template_entities_extra_data: &'a mut ElementEntitiesExtraData,
    template_entity: Entity,
) -> Entity {
    let children = template_world
        .get::<Children>(template_entity)
        .map(|n| n.iter().copied().collect::<Vec<_>>());

    let new_entity = clone_entity(
        world,
        entities_extra_data,
        template_world,
        template_entities_extra_data,
        template_entity,
    )
    .id();

    if let Some(children) = children {
        let mut new_children_entities = Vec::with_capacity(children.len());
        for child_entity in children {
            let child_entity = clone_entity_nest(
                world,
                entities_extra_data,
                template_world,
                template_entities_extra_data,
                child_entity,
            );
            new_children_entities.push(child_entity);
        }

        world
            .entity_mut(new_entity)
            .push_children(&new_children_entities);
    }
    new_entity
}*/
