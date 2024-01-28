use core::iter;

use bevy_ecs::prelude::{Entity, Query};
use bevy_ecs::query::ReadOnlyWorldQuery;
use bevy_ecs::world::{EntityRef, EntityWorldMut, World};
use rxy_bevy::RendererState;
use rxy_bevy_element::{
    AttrSetBits, ElementEntityExtraData, ElementEntityWorldMutExt,
};
use rxy_core::prelude::{Either, EitherExt};
use rxy_style::{
    IterExt, NodeInterStyleAttrInfo, NodeInterStyleState, NodeStyleAttrInfo, NodeStyleItemId,
    NodeStyleSheetId, PipeOp, StyleAttrId, StyleError, StyleSheetLocation,
};

use crate::node_style_state::NodeStyleSheetsState;
use crate::{
    EntityAttrSyncer, NodeStyleState, StyleEntityRefExt, StyleItemValue, StyleSheetDefinition,
    SyncerWrapper,
};
use crate::{EntityWorldRef, Result};
pub(crate) trait StateOwner<'a, 's>: Sized {
    fn get_style_sheets_state(&'s self, entity: Entity) -> Result<&'a NodeStyleSheetsState>;

    fn get_style_item_attr_id(
        &'s self,
        entity: Entity,
        style_item_id: impl Into<NodeStyleItemId>,
    ) -> Result<StyleAttrId> {
        let style_item_id: NodeStyleItemId = style_item_id.into();
        self.get_style_sheet_definition(entity, style_item_id)
            .and_then(|n| {
                n.items
                    .get(style_item_id.item_index as usize)
                    .ok_or(StyleError::NoFoundStyleItemId {
                        item_id: style_item_id,
                    })
                    .map(|n| n.attr_id)
            })
    }

    #[inline(always)]
    fn get_style_item_value(
        &'s self,
        entity: Entity,
        style_item_id: impl Into<NodeStyleItemId>,
    ) -> Result<&'a StyleItemValue> {
        let style_item_id: NodeStyleItemId = style_item_id.into();
        self.get_style_sheet_definition(entity, style_item_id)
            .and_then(|n| {
                n.items.get(style_item_id.item_index as usize).ok_or(
                    StyleError::NoFoundStyleItemId {
                        item_id: style_item_id,
                    },
                )
            })
    }

    fn get_style_sheet_definition(
        &'s self,
        entity: Entity,
        style_sheet_id: impl Into<NodeStyleSheetId>,
    ) -> Result<&'a StyleSheetDefinition> {
        let style_sheet_id: NodeStyleSheetId = style_sheet_id.into();
        let style_sheets_state = self.get_style_sheets_state(entity)?;
        match style_sheet_id.location {
            StyleSheetLocation::Inline => {
                style_sheets_state.get_inline_style_sheet(style_sheet_id.index)
            }
            StyleSheetLocation::Shared => {
                let style_sheet_id =
                    style_sheets_state.get_shared_style_sheet_id(style_sheet_id.index)?;

                let node_id = style_sheet_id.node_id;
                self.get_style_sheet_definition(node_id, style_sheet_id)
            }
        }
    }
}

pub(crate) trait StateOwnerWithNodeId<'a, 's>: StateOwner<'a, 's> {
    fn get_current_entity(&'s self) -> Entity;

    fn get_current_style_item_attr_id(
        &'s self,
        style_item_id: impl Into<NodeStyleItemId>,
    ) -> Result<StyleAttrId> {
        self.get_style_item_attr_id(self.get_current_entity(), style_item_id)
    }

    #[inline(always)]
    fn get_current_style_item_value(
        &'s self,
        style_item_id: impl Into<NodeStyleItemId>,
    ) -> Result<&'a StyleItemValue> {
        self.get_style_item_value(self.get_current_entity(), style_item_id)
    }

    fn get_current_style_sheet_definition(
        &'s self,
        style_sheet_id: impl Into<NodeStyleSheetId>,
    ) -> Result<&'a StyleSheetDefinition> {
        self.get_style_sheet_definition(self.get_current_entity(), style_sheet_id)
    }
}

impl<'a> StateOwner<'a, '_> for &'a World {
    fn get_style_sheets_state(&self, entity: Entity) -> Result<&'a NodeStyleSheetsState> {
        self.get_entity(entity)
            .ok_or(StyleError::NoFoundNode { node_id: entity })?
            .get_style_sheets_state()
    }
}
//
// impl<'a> StateOwner<'a> for &'a mut World {
//     fn get_style_sheets_state(&self, entity: Entity) -> Result<&'a NodeStyleSheetsState> {
//         self
//             .get_entity(entity)
//             .ok_or(StyleError::NoFoundNode { node_id: entity })?
//             .get_style_sheets_state()
//     }
// }

impl<'a> StateOwner<'a, '_> for EntityWorldRef<'a> {
    fn get_style_sheets_state(&self, entity: Entity) -> Result<&'a NodeStyleSheetsState> {
        self.world
            .get_entity(entity)
            .ok_or(StyleError::NoFoundNode { node_id: entity })?
            .get_style_sheets_state()
    }
}

impl<'a> StateOwnerWithNodeId<'a, '_> for EntityWorldRef<'a> {
    fn get_current_entity(&self) -> Entity {
        self.entity_ref.id()
    }
}

pub struct EntityStyleWorldQuery<'a, 'world, 'state, F: ReadOnlyWorldQuery> {
    pub query: Query<'world, 'state, &'a RendererState<NodeStyleSheetsState>, F>,
    pub current_entity: Entity,
}

// impl<'a, F: ReadOnlyWorldQuery> EntityStyleWorldQuery<'a, F> {
//     fn get_style_sheets_state(&self, entity: Entity) -> Result<&NodeStyleSheetsState> {
//         self.query
//             .get(entity)
//             .map(|n| &n.0)
//             .map_err(move |_| StyleError::NoFoundStyleSheetsState { node_id: entity })
//     }
//
//     fn get_style_sheet_definition(
//         &self,
//         entity: Entity,
//         style_sheet_id: impl Into<NodeStyleSheetId>,
//     ) -> Result<&StyleSheetDefinition> {
//         let style_sheet_id: NodeStyleSheetId = style_sheet_id.into();
//         let style_sheets_state = self.get_style_sheets_state(entity)?;
//         match style_sheet_id.location {
//             StyleSheetLocation::Inline => {
//                 style_sheets_state.get_inline_style_sheet(style_sheet_id.index)
//             }
//             StyleSheetLocation::Shared => {
//                 let style_sheet_id =
//                     style_sheets_state.get_shared_style_sheet_id(style_sheet_id.index)?;
//
//                 let node_id = style_sheet_id.node_id;
//                 self.get_style_sheet_definition(node_id, style_sheet_id)
//             }
//         }
//     }
//
//     #[inline(always)]
//     fn get_current_style_item_value(
//         &self,
//         style_item_id: impl Into<NodeStyleItemId>,
//     ) -> Result<&StyleItemValue> {
//         let style_item_id: NodeStyleItemId = style_item_id.into();
//         self.get_style_sheet_definition(self.current_entity, style_item_id)
//             .and_then(|n| {
//                 n.items.get(style_item_id.item_index as usize).ok_or(
//                     StyleError::NoFoundStyleItemId {
//                         item_id: style_item_id,
//                     },
//                 )
//             })
//     }
// }

impl<'a, 'world, 'state, F: ReadOnlyWorldQuery> StateOwner<'a, 'a>
    for EntityStyleWorldQuery<'world, 'state, 'a, F>
{
    fn get_style_sheets_state(&'a self, entity: Entity) -> Result<&'a NodeStyleSheetsState> {
        self.query
            .get(entity)
            .map(|n| &n.0)
            .map_err(move |_| StyleError::NoFoundStyleSheetsState { node_id: entity })
    }
}

impl<'a, 'world, 'state, F: ReadOnlyWorldQuery> StateOwnerWithNodeId<'a, 'a> for EntityStyleWorldQuery<'world, 'state,'a, F> {
    fn get_current_entity(&self) -> Entity {
        self.current_entity
    }
}

#[derive(Default)]
pub struct EntityStyleAttrInfoIterArgs<'a> {
    pub iter_normal_style_sheet: bool,
    pub iter_inter_style_sheet: bool,
    pub limit_attr_ids: Option<&'a [StyleAttrId]>,
    pub limit_attr_bits: Option<AttrSetBits>,
}

pub type EntityStyleAttrInfoIterItem<'a> = (
    StyleAttrId,
    Either<&'a NodeStyleAttrInfo, &'a NodeInterStyleAttrInfo>,
);

impl<'a> EntityStyleAttrInfoIterArgs<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn normal() -> Self {
        Self {
            iter_normal_style_sheet: true,
            ..Default::default()
        }
    }
    pub fn inter() -> Self {
        Self {
            iter_inter_style_sheet: true,
            ..Default::default()
        }
    }
    pub fn all_kind() -> Self {
        Self {
            iter_inter_style_sheet: true,
            iter_normal_style_sheet: true,
            ..Default::default()
        }
    }
    pub fn iter_and_sync_set<'w>(self, entity_world_mut: &'w mut EntityWorldMut<'w>) -> Result {
        let entity_ref = entity_world_mut.as_entity_mut();
        let wrapper = self.iter(unsafe { core::mem::transmute(entity_ref.as_readonly()) });

        wrapper.sync_attr_value_to_element(entity_world_mut)
    }

    pub fn iter(
        self,
        entity_ref: EntityRef<'a>,
    ) -> SyncerWrapper<impl Iterator<Item = EntityStyleAttrInfoIterItem<'a>>> {
        let r = iter::empty();

        fn op_limit_attr_bits<'a>(
            iter: impl Iterator<Item = EntityStyleAttrInfoIterItem<'a>>,
            limit_attr_bits: Option<AttrSetBits>,
        ) -> impl Iterator<Item = EntityStyleAttrInfoIterItem<'a>> {
            iter.option_op(limit_attr_bits, |n, limit_attr_bits| {
                n.filter(move |(attr_id, _)| {
                    !ElementEntityExtraData::static_is_set_attr(limit_attr_bits, *attr_id)
                })
            })
        }

        let attr_infos = || {
            let entity_style_state = entity_ref
                .get_ref::<RendererState<NodeStyleState>>()
                .map(|n| n.into_inner())
                .unwrap();
            match self.limit_attr_ids {
                Some(n) => n
                    .iter()
                    .filter_map(|id| {
                        let attr_info = entity_style_state.attr_infos.get(id);
                        attr_info.map(|n| (id, n))
                    })
                    .either_left(),
                None => entity_style_state.attr_infos.iter().either_right(),
            }
            .map(|n| (*n.0, n.1.either_left::<&'a NodeInterStyleAttrInfo>()))
        };

        let inter_attr_infos = || {
            let r = iter::empty();
            r.option_op(
                entity_ref
                    .get_ref::<RendererState<NodeInterStyleState>>()
                    .map(|n| n.into_inner()),
                |_, entity_inter_style_state| {
                    match self.limit_attr_ids {
                        Some(n) => n
                            .iter()
                            .filter_map(|id| {
                                let attr_info = entity_inter_style_state.attr_infos.get(id);
                                attr_info.map(|n| (id, n))
                            })
                            .either_left(),
                        None => entity_inter_style_state.attr_infos.iter().either_right(),
                    }
                    .map(|n| (*n.0, n.1.either_right::<&'a NodeStyleAttrInfo>()))
                },
            )
        };

        SyncerWrapper(
            r.chain_option(self.iter_normal_style_sheet.then(attr_infos))
                .chain_option(self.iter_inter_style_sheet.then(inter_attr_infos))
                .pipe(self.limit_attr_bits, op_limit_attr_bits),
        )
    }
}
