use core::ops::Deref;
use std::any::TypeId;

use bevy_ecs::component::Component;
use bevy_ecs::{
    entity::Entity,
    world::{EntityMut, EntityRef, EntityWorldMut, World},
};
use bevy_hierarchy::DespawnRecursiveExt;
use rxy_bevy::{BevyRenderer, RendererState};
use rxy_style::{NodeInterStyleAttrInfos, NodeStyleAttrInfos, StyleSheetId, StyleSheetLocation};

use crate::attr_iter::StateOwnerWithNodeId;
use crate::node_style_state::NodeStyleSheetsState;
use crate::plugin::TypedEntities;
use crate::{Result, SharedStyleState, StyleError, StyleSheetDefinition};

#[derive(Copy, Clone)]
pub struct EntityWorldRef<'a> {
    pub world: &'a World,
    pub entity_ref: EntityRef<'a>,
}

impl<'a> EntityWorldRef<'a> {
    pub fn new(world: &'a World, entity: Entity) -> Self {
        Self {
            world,
            entity_ref: world.entity(entity),
        }
    }

    pub fn get_ref<T: Component>(&self) -> Option<&'a T> {
        self.entity_ref.get::<T>()
    }
}

impl<'a> Deref for EntityWorldRef<'a> {
    type Target = EntityRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.entity_ref
    }
}

impl<'a> From<EntityWorldMut<'a>> for EntityWorldRef<'a> {
    fn from(entity_world_mut: EntityWorldMut<'a>) -> Self {
        let entity = entity_world_mut.id();
        let world = &*entity_world_mut.into_world_mut();
        EntityWorldRef::new(world, entity)
    }
}
impl<'a> From<&'a mut EntityWorldMut<'_>> for EntityWorldRef<'a> {
    fn from(entity_world_mut: &'a mut EntityWorldMut<'_>) -> Self {
        let entity = entity_world_mut.id();
        let world = entity_world_mut.world();
        EntityWorldRef::new(world, entity)
    }
}

pub trait StyleEntityRefExt<'a> {
    fn get_style_sheets_state(&self) -> Result<&'a NodeStyleSheetsState>;
    fn get_shared_style_state(&self) -> Result<&'a SharedStyleState>;
    fn get_style_state(&self) -> Result<&'a NodeStyleAttrInfos>;
    fn get_inter_style_state(&self) -> Result<&'a NodeInterStyleAttrInfos>;
}
pub trait StyleEntityMutExt {
    fn get_style_sheets_state(&mut self) -> Result<&mut NodeStyleSheetsState>;
    fn get_shared_style_state(&mut self) -> Result<&mut SharedStyleState>;
    fn get_style_state(&mut self) -> Result<&mut NodeStyleAttrInfos>;
    fn get_inter_style_state(&mut self) -> Result<&mut NodeInterStyleAttrInfos>;
}

impl<'a> StyleEntityRefExt<'a> for EntityRef<'a> {
    fn get_style_sheets_state(&self) -> Result<&'a NodeStyleSheetsState> {
        self.get_ref::<RendererState<NodeStyleSheetsState>>()
            .map(|n| &n.into_inner().0)
            .ok_or(StyleError::NoFoundStyleSheetsState { node_id: self.id() })
    }
    fn get_shared_style_state(&self) -> Result<&'a SharedStyleState> {
        self.get_ref::<RendererState<SharedStyleState>>()
            .map(|n| &n.into_inner().0)
            .ok_or(StyleError::NoFoundSharedStyleSheet { node_id: self.id() })
    }

    fn get_style_state(&self) -> Result<&'a NodeStyleAttrInfos> {
        self.get_ref::<RendererState<NodeStyleAttrInfos>>()
            .map(|n| &n.into_inner().0)
            .ok_or(StyleError::NoFoundStyleState { node_id: self.id() })
    }

    fn get_inter_style_state(&self) -> Result<&'a NodeInterStyleAttrInfos> {
        self.get_ref::<RendererState<NodeInterStyleAttrInfos>>()
            .map(|n| &n.into_inner().0)
            .ok_or(StyleError::NoFoundInterStyleState { node_id: self.id() })
    }
}
impl<'a> StyleEntityRefExt<'a> for EntityWorldRef<'a> {
    fn get_style_sheets_state(&self) -> Result<&'a NodeStyleSheetsState> {
        self.get_ref::<RendererState<NodeStyleSheetsState>>()
            .map(|n| &n.0)
            .ok_or(StyleError::NoFoundStyleSheetsState { node_id: self.id() })
    }
    fn get_shared_style_state(&self) -> Result<&'a SharedStyleState> {
        self.get_ref::<RendererState<SharedStyleState>>()
            .map(|n| &n.0)
            .ok_or(StyleError::NoFoundSharedStyleSheet { node_id: self.id() })
    }

    fn get_style_state(&self) -> Result<&'a NodeStyleAttrInfos> {
        self.get_ref::<RendererState<NodeStyleAttrInfos>>()
            .map(|n| &n.0)
            .ok_or(StyleError::NoFoundStyleState { node_id: self.id() })
    }

    fn get_inter_style_state(&self) -> Result<&'a NodeInterStyleAttrInfos> {
        self.get_ref::<RendererState<NodeInterStyleAttrInfos>>()
            .map(|n| &n.0)
            .ok_or(StyleError::NoFoundInterStyleState { node_id: self.id() })
    }
}

macro_rules! impl_style_ext_for_entity_mut {
    ($ty:ty) => {
        impl StyleEntityMutExt for $ty {
            fn get_style_sheets_state(&mut self) -> Result<&mut NodeStyleSheetsState> {
                let node_id = self.id();
                self.get_mut::<RendererState<NodeStyleSheetsState>>()
                    .map(|n| &mut n.into_inner().0)
                    .ok_or(StyleError::NoFoundStyleSheetsState { node_id })
            }
            fn get_shared_style_state(&mut self) -> Result<&mut SharedStyleState> {
                let node_id = self.id();
                self.get_mut::<RendererState<SharedStyleState>>()
                    .map(|n| &mut n.into_inner().0)
                    .ok_or(StyleError::NoFoundSharedStyleSheet { node_id })
            }
            fn get_style_state(&mut self) -> Result<&mut NodeStyleAttrInfos> {
                let node_id = self.id();
                self.get_mut::<RendererState<NodeStyleAttrInfos>>()
                    .map(|n| &mut n.into_inner().0)
                    .ok_or(StyleError::NoFoundStyleState { node_id })
            }

            fn get_inter_style_state(&mut self) -> Result<&mut NodeInterStyleAttrInfos> {
                let node_id = self.id();
                self.get_mut::<RendererState<NodeInterStyleAttrInfos>>()
                    .map(|n| &mut n.into_inner().0)
                    .ok_or(StyleError::NoFoundInterStyleState { node_id })
            }
        }
    };
}
impl_style_ext_for_entity_mut!(EntityMut<'_>);
impl_style_ext_for_entity_mut!(EntityWorldMut<'_>);

pub trait EntityWorldMutExt<'a> {
    fn scoped_style_sheets_state<U>(
        &mut self,
        f: impl FnOnce(&mut EntityWorldMut<'a>, &mut NodeStyleSheetsState) -> U,
    ) -> Result<U>;
    fn scoped_shared_style_state<U>(
        &mut self,
        f: impl FnOnce(&mut EntityWorldMut<'a>, &mut SharedStyleState) -> U,
    ) -> Result<U>;
    fn scoped_style_state<U>(
        &mut self,
        f: impl FnOnce(&mut EntityWorldMut<'a>, &mut NodeStyleAttrInfos) -> U,
    ) -> Result<U>;
    fn scoped_inter_style_state<U>(
        &mut self,
        f: impl FnOnce(&mut EntityWorldMut<'a>, &mut NodeInterStyleAttrInfos) -> U,
    ) -> Result<U>;
    fn scoped_inter_style_state_or_default<U>(
        &mut self,
        f: impl FnOnce(&mut EntityWorldMut<'a>, &mut NodeInterStyleAttrInfos) -> U,
    ) -> Result<U>;
}

impl<'a> EntityWorldMutExt<'a> for EntityWorldMut<'a> {
    fn scoped_style_sheets_state<U>(
        &mut self,
        f: impl FnOnce(&mut EntityWorldMut<'a>, &mut NodeStyleSheetsState) -> U,
    ) -> Result<U> {
        let mut state = core::mem::take(self.get_style_sheets_state()?);
        let r = f(self, &mut state);
        self.insert::<RendererState<NodeStyleSheetsState>>(RendererState(state));
        Ok(r)
    }
    fn scoped_shared_style_state<U>(
        &mut self,
        f: impl FnOnce(&mut EntityWorldMut<'a>, &mut SharedStyleState) -> U,
    ) -> Result<U> {
        let mut state = core::mem::take(self.get_shared_style_state()?);
        let r = f(self, &mut state);
        self.insert::<RendererState<SharedStyleState>>(RendererState(state));
        Ok(r)
    }

    fn scoped_style_state<U>(
        &mut self,
        f: impl FnOnce(&mut EntityWorldMut<'a>, &mut NodeStyleAttrInfos) -> U,
    ) -> Result<U> {
        let mut state = core::mem::take(self.get_style_state()?);
        let r = f(self, &mut state);
        self.insert::<RendererState<NodeStyleAttrInfos>>(RendererState(state));
        Ok(r)
    }
    fn scoped_inter_style_state<U>(
        &mut self,
        f: impl FnOnce(&mut EntityWorldMut<'a>, &mut NodeInterStyleAttrInfos) -> U,
    ) -> Result<U> {
        let mut state = core::mem::take(self.get_inter_style_state()?);
        let r = f(self, &mut state);
        self.insert::<RendererState<NodeInterStyleAttrInfos>>(RendererState(state));
        Ok(r)
    }
    fn scoped_inter_style_state_or_default<U>(
        &mut self,
        f: impl FnOnce(&mut EntityWorldMut<'a>, &mut NodeInterStyleAttrInfos) -> U,
    ) -> Result<U> {
        let mut state = core::mem::take(BevyRenderer::get_or_insert_default_state_by_entity_mut::<
            NodeInterStyleAttrInfos,
        >(self));
        let r = f(self, &mut state);
        self.insert::<RendererState<NodeInterStyleAttrInfos>>(RendererState(state));
        Ok(r)
    }
}

pub trait StyleWorldExt {
    fn get_style_sheet_definition_ref(
        &self,
        style_sheet_id: StyleSheetId<BevyRenderer>,
    ) -> Result<&StyleSheetDefinition>;
    /* fn get_typed_entity_or_spawn(&mut self, type_id: TypeId, reserve_key: Option<Entity>)
    -> Entity; */
    fn get_typed_entity(&mut self, type_id: TypeId) -> Option<Entity>;

    fn scoped_style_sheet_definition<'a, U>(
        &'a mut self,
        style_sheet_id: StyleSheetId<BevyRenderer>,
        f: impl FnOnce(&mut EntityWorldMut<'a>, &mut StyleSheetDefinition) -> U,
    ) -> Result<U>;
    fn insert_typed_entity(&mut self, type_id: TypeId, entity: Entity);
}

impl StyleWorldExt for World {
    fn scoped_style_sheet_definition<'a, U>(
        &'a mut self,
        style_sheet_id: StyleSheetId<BevyRenderer>,
        f: impl FnOnce(&mut EntityWorldMut<'a>, &mut StyleSheetDefinition) -> U,
    ) -> Result<U> {
        assert_eq!(style_sheet_id.location, StyleSheetLocation::Inline);
        let mut entity_world_mut = self.entity_mut(style_sheet_id.node_id);
        entity_world_mut.scoped_style_sheets_state(|entity_world_mut, style_sheets_state| {
            match style_sheet_id.location {
                StyleSheetLocation::Inline => {
                    let style_sheet_definition =
                        style_sheets_state.get_inline_style_sheet_mut(style_sheet_id.index)?;
                    Ok(f(entity_world_mut, style_sheet_definition))
                }
                StyleSheetLocation::Shared => {
                    unreachable!()
                    // let style_sheet_id = style_sheets_state
                    //     .get_shared_style_sheet_id(style_sheet_id.index)?
                    //     .clone();
                    // entity_world_mut.world_scope(|world| {
                    //     let mut entity_world_mut = world.entity_mut(style_sheet_id.node_id);
                    //     entity_world_mut.scoped_style_sheets_state(
                    //         |entity_world_mut, style_sheets_state| {
                    //             let style_sheet_definition = style_sheets_state
                    //                 .get_inline_style_sheet_mut(style_sheet_id.index)?;
                    //             let r = f(entity_world_mut, style_sheet_definition);
                    //             Ok::<U, StyleError>(r)
                    //         },
                    //     )
                    // })?
                }
            }
        })?
    }

    fn get_style_sheet_definition_ref(
        &self,
        style_sheet_id: StyleSheetId<BevyRenderer>,
    ) -> Result<&StyleSheetDefinition> {
        let entity_world_ref = EntityWorldRef::new(self, style_sheet_id.node_id);
        entity_world_ref.get_current_style_sheet_definition(style_sheet_id)
    }
    /*
       fn get_typed_entity_or_spawn(
           &mut self,
           type_id: TypeId,
           reserve_key: Option<Entity>,
       ) -> Entity {
           let typed_entities = self.resource::<TypedEntities>();
           if typed_entities.contains_key(&type_id) {
               typed_entities.get(&type_id).unwrap().clone()
           } else {
               let shared_style_container = self.resource::<RxySharedStyleContainer>().0;
               let entity = match reserve_key {
                   None => self
                       .spawn(bevy_core::Name::new("[shared_style]"))
                       .set_parent(shared_style_container)
                       .id(),
                   Some(reserve_key) => self
                       .get_or_spawn(reserve_key)
                       .unwrap()
                       .insert(bevy_core::Name::new("[shared_style]"))
                       .set_parent(shared_style_container)
                       .id(),
               };
               self.resource_mut::<TypedEntities>().insert(type_id, entity);
               entity
           }
       }
    */
    fn get_typed_entity(&mut self, type_id: TypeId) -> Option<Entity> {
        self.resource_mut::<TypedEntities>().get(&type_id).cloned()
    }

    fn insert_typed_entity(&mut self, type_id: TypeId, entity: Entity) {
        let prev = self.resource_mut::<TypedEntities>().insert(type_id, entity);
        if let Some(prev) = prev {
            self.entity_mut(prev).despawn_recursive()
        }
    }
}
