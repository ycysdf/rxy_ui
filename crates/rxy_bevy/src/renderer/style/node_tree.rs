use super::rxy_bevy_crate::BevyRenderer;
use super::{StyleError, StyleWorldExt};
use crate::renderer::style::node_style_state::NodeStyleSheetsState;
use crate::renderer::style::{
    EntityStyleAttrInfoIterArgs, Previous, StyleEntityMutExt, StyleEntityWorldMutExt,
};
use crate::{
    ElementEntityExtraData, ElementEntityWorldMutExt, EntityWorldMutExt,
    RendererState,
};
use bevy_ecs::prelude::{EntityWorldMut, World};
use bevy_ui::Interaction;
use rxy_core::style::{
    AppliedStyleSheet, ApplyStyleSheetsMemberState, AttrStyleOwner, NodeInterStyleAttrInfos,
    NodeStyleAttrInfos, NodeStyleSheetId, StyleSheetCtx, StyleSheetDefinition, StyleSheetLocation,
    StyleSheets, StyledNodeTree,
};
use rxy_core::{AttrIndex, RendererNodeId};
use crate::attrs::get_attr_by_index;

pub fn scoped_style_sheet_definition<U>(
    applied_style_sheet: &AppliedStyleSheet<BevyRenderer>,
    entity_world_mut: &mut EntityWorldMut,
    f: impl FnOnce(&mut EntityWorldMut, Option<&StyleSheetDefinition>) -> U,
) -> super::Result<U> {
    let entity = entity_world_mut.id();
    match applied_style_sheet {
        AppliedStyleSheet::None => Ok(f(entity_world_mut, None)),
        AppliedStyleSheet::Inline(style_sheet_definition) => {
            Ok(f(entity_world_mut, Some(style_sheet_definition)))
        }
        AppliedStyleSheet::Shared(style_sheet_id) => entity_world_mut.world_scope(|world| {
            world.scoped_style_sheet_definition(
                style_sheet_id.clone(),
                |entity_world_mut, style_sheet_definition| {
                    entity_world_mut.world_scope(|world| {
                        let mut entity_world_mut = world.entity_mut(entity);
                        f(&mut entity_world_mut, Some(&*style_sheet_definition))
                    })
                },
            )
        }),
    }
}

impl StyledNodeTree<BevyRenderer> for World {
    fn unbuild_style_sheet(
        &mut self,
        node_id: RendererNodeId<BevyRenderer>,
        member_state: ApplyStyleSheetsMemberState,
    ) -> Result<(), StyleError> {
        let mut entity_world_mut: EntityWorldMut<'_> = self.entity_mut(node_id);
        let attr_is_set = entity_world_mut
            .get_mut::<ElementEntityExtraData>()
            .ok_or(StyleError::NoFoundElementEntityExtraData {
                node_id: node_id.clone(),
            })?
            .attr_is_set;

        let mut reset_keys = vec![];

        fn remove_attr_style_of_definition(
            entity_world_mut: &mut EntityWorldMut,
            style_sheet_definition: &StyleSheetDefinition,
            style_sheet_id: NodeStyleSheetId,
            style_state: &mut NodeStyleAttrInfos,
            require_reset_f: impl FnMut(AttrIndex),
        ) -> Result<(), StyleError> {
            if style_sheet_definition.interaction.is_some() {
                entity_world_mut
                    .get_inter_style_state()?
                    .remove_attr_style_of_definition(
                        style_sheet_definition,
                        style_sheet_id,
                        require_reset_f,
                    )?;
            } else {
                style_state.remove_attr_style_of_definition(
                    style_sheet_definition,
                    style_sheet_id,
                    require_reset_f,
                )?;
            }

            Ok::<(), StyleError>(())
        }

        // entity_mut
        entity_world_mut.scoped_style_sheets_state(|entity_world_mut, style_sheets_state| {
            entity_world_mut.scoped_style_state(|entity_world_mut, style_state| {
                for (style_sheet_index, style_sheet_definition) in
                    style_sheets_state.take_inline_style_sheets_from_member(member_state)
                {
                    remove_attr_style_of_definition(
                        entity_world_mut,
                        &style_sheet_definition,
                        NodeStyleSheetId {
                            index: style_sheet_index,
                            location: StyleSheetLocation::Inline,
                        },
                        style_state,
                        |key: AttrIndex| {
                            if !ElementEntityExtraData::static_is_set_attr(attr_is_set, key) {
                                reset_keys.push(key);
                            }
                        },
                    )?;
                }

                for (style_sheet_index, style_sheet_id) in
                    style_sheets_state.take_shared_style_sheets_from_member(member_state)
                {
                    {
                        entity_world_mut
                            .world_scope(|world: &mut World| {
                                let mut entity_world_mut = world.entity_mut(style_sheet_id.node_id);
                                let shared_style_state =
                                    entity_world_mut.get_shared_style_state()?;
                                shared_style_state.remove_subscriber(node_id);
                                Ok::<(), StyleError>(())
                            })
                            .unwrap();
                    }

                    assert_eq!(style_sheet_id.location, StyleSheetLocation::Inline);
                    entity_world_mut.world_scope(|world: &mut World| {
                        let node_id = style_sheet_id.node_id;
                        let mut entity_world_mut = world.entity_mut(node_id);
                        entity_world_mut.scoped_style_sheets_state(
                            |entity_world_mut, style_sheets_state| {
                                let style_sheet_definition = style_sheets_state
                                    .get_inline_style_sheet(style_sheet_id.index)?;
                                remove_attr_style_of_definition(
                                    entity_world_mut,
                                    style_sheet_definition,
                                    NodeStyleSheetId {
                                        index: style_sheet_index,
                                        location: StyleSheetLocation::Shared,
                                    },
                                    style_state,
                                    |key: AttrIndex| {
                                        if !ElementEntityExtraData::static_is_set_attr(
                                            attr_is_set,
                                            key,
                                        ) {
                                            reset_keys.push(key);
                                        }
                                    },
                                )
                            },
                        )
                    })??;
                }
                Ok::<(), StyleError>(())
            })??;
            Ok::<(), StyleError>(())
        })??;

        entity_world_mut.world_scope(|world: &mut World| {
            for attr_index in reset_keys.iter().cloned() {
                get_attr_by_index(attr_index).set_value(world, node_id, None);
            }
        });

        EntityStyleAttrInfoIterArgs {
            iter_normal_style_sheet: true,
            iter_inter_style_sheet: true,
            limit_attr_ids: Some(reset_keys.as_slice()),
            ..Default::default()
        }
        .iter_and_sync_set(entity_world_mut)?;
        Ok(())
    }

    fn build_style_sheets<T>(
        &mut self,
        node_id: RendererNodeId<BevyRenderer>,
        style_sheets: T,
        member_state: Option<ApplyStyleSheetsMemberState>,
    ) -> Result<ApplyStyleSheetsMemberState, StyleError>
    where
        T: StyleSheets<BevyRenderer>,
    {
        let mut entity_world_mut = self.entity_mut(node_id.clone());
        entity_world_mut.insert_if_not_exist(RendererState(NodeStyleAttrInfos::default()));
        entity_world_mut.insert_if_not_exist(Interaction::default());
        entity_world_mut.insert_if_not_exist(Previous(Interaction::default()));

        let style_sheets_state = entity_world_mut.get_or_default::<NodeStyleSheetsState>();

        let inline_style_sheet_count = style_sheets_state.inline_style_sheet.len() as _;
        let shared_style_sheet_count = style_sheets_state.shared_style_sheet_ids.len() as _;

        let (applied_style_sheets, mut member_state, is_first_build) =
            if let Some(member_state) = member_state {
                let style_sheet_ctx = StyleSheetCtx {
                    inline_style_sheet_index: member_state.inline_sheet_index,
                    shared_style_sheet_index: member_state.shared_sheet_index,
                    world: self,
                    node_id: node_id.clone(),
                };
                let (style_sheets, info) = style_sheets.style_sheets(style_sheet_ctx);

                assert_eq!(
                    member_state,
                    ApplyStyleSheetsMemberState {
                        inline_sheet_index: member_state.inline_sheet_index,
                        inline_sheet_count: info.inline_style_sheet_count,
                        shared_sheet_index: member_state.shared_sheet_index,
                        shared_sheet_count: info.shared_style_sheet_count,
                    }
                );
                (style_sheets, member_state, false)
            } else {
                let style_sheet_ctx = StyleSheetCtx {
                    inline_style_sheet_index: inline_style_sheet_count,
                    shared_style_sheet_index: shared_style_sheet_count,
                    world: self,
                    node_id: node_id.clone(),
                };
                let (style_sheets, info) = style_sheets.style_sheets(style_sheet_ctx);

                let member_state = ApplyStyleSheetsMemberState {
                    inline_sheet_index: inline_style_sheet_count,
                    inline_sheet_count: info.inline_style_sheet_count,
                    shared_sheet_index: shared_style_sheet_count,
                    shared_sheet_count: info.shared_style_sheet_count,
                };
                (style_sheets, member_state, true)
            };

        {
            let mut entity_world_mut = self.entity_mut(node_id.clone());
            let mut recalculate_interaction_style_value = false;

            entity_world_mut.scoped_style_sheets_state(
                |entity_world_mut, style_sheets_state| {
                    entity_world_mut.scoped_style_state(|entity_world_mut, node_style_state| {
                        for applied_style_sheet in applied_style_sheets {
                            let Some(style_sheet_location) =
                                applied_style_sheet.style_sheet_location()
                            else {
                                continue;
                            };
                            let style_sheet_index = if is_first_build {
                                style_sheets_state.get_style_sheet_len(style_sheet_location)
                            } else {
                                member_state.get_and_increment_and_by_location(style_sheet_location)
                            };
                            if !scoped_style_sheet_definition(
                                &applied_style_sheet,
                                entity_world_mut,
                                |entity_world_mut, style_sheet_definition| {
                                    let Some(style_sheet_definition) = style_sheet_definition
                                    else {
                                        return Ok(false);
                                    };
                                    if style_sheet_definition.items.is_empty() {
                                        return Ok(false);
                                    }

                                    if style_sheet_definition.interaction.is_some() {
                                        recalculate_interaction_style_value = true;
                                        entity_world_mut.scoped_inter_style_state_or_default(
                                            move |entity_world_mut, attr_style_owner| {
                                                let node_id = entity_world_mut.id();
                                                style_sheet_definition.add_to(
                                                    attr_style_owner,
                                                    style_sheet_location,
                                                    style_sheet_index,
                                                    entity_world_mut.world(),
                                                    node_id,
                                                )
                                            },
                                        )??;
                                    } else {
                                        let node_id = entity_world_mut.id();
                                        style_sheet_definition.add_to(
                                            node_style_state,
                                            style_sheet_location,
                                            style_sheet_index,
                                            entity_world_mut.world(),
                                            node_id,
                                        )?;
                                    }

                                    Ok(true)
                                },
                            )?? {
                                continue;
                            }

                            if is_first_build {
                                style_sheets_state.push_applied_style_sheet(applied_style_sheet);
                            } else {
                                style_sheets_state.set_applied_style_sheet(
                                    style_sheet_index,
                                    applied_style_sheet,
                                );
                            }
                        }
                        Ok::<(), StyleError>(())
                    })?
                },
            )??;

            if recalculate_interaction_style_value
                && !entity_world_mut.contains::<RendererState<NodeInterStyleAttrInfos>>()
            {
                entity_world_mut.insert(RendererState(NodeInterStyleAttrInfos::default()));
            }
            EntityStyleAttrInfoIterArgs {
                iter_inter_style_sheet: recalculate_interaction_style_value,
                iter_normal_style_sheet: true,
                ..Default::default()
            }
            .iter_and_sync_set(entity_world_mut)?;
        }

        Ok(member_state)
    }

    fn rebuild_style_sheet<T>(
        &mut self,
        node_id: RendererNodeId<BevyRenderer>,
        style_sheets: T,
        mut member_state: ApplyStyleSheetsMemberState,
    ) -> Result<(), StyleError>
    where
        T: StyleSheets<BevyRenderer>,
    {
        let style_sheet_ctx = StyleSheetCtx {
            inline_style_sheet_index: member_state.inline_sheet_index,
            shared_style_sheet_index: member_state.shared_sheet_index,
            world: self,
            node_id: node_id.clone(),
        };
        let (style_sheets, info) = style_sheets.style_sheets(style_sheet_ctx);

        assert_eq!(
            member_state,
            ApplyStyleSheetsMemberState {
                inline_sheet_index: member_state.inline_sheet_index,
                inline_sheet_count: info.inline_style_sheet_count,
                shared_sheet_index: member_state.shared_sheet_index,
                shared_sheet_count: info.shared_style_sheet_count,
            }
        );
        let mut entity_world_mut = self.entity_mut(node_id.clone());

        let mut entity_mut = entity_world_mut.as_entity_mut();
        let mut iter_args = EntityStyleAttrInfoIterArgs::normal();
        let mut attr_ids = vec![];
        let style_sheets_state = entity_mut.get_style_sheets_state()?;
        for style_sheet in style_sheets {
            match style_sheet {
                AppliedStyleSheet::None => {}
                AppliedStyleSheet::Inline(style_sheet) => {
                    for (item_index, item_value) in style_sheet.items.into_iter().enumerate() {
                        attr_ids.push(item_value.attr_id);
                        if style_sheet.interaction.is_some() {
                            iter_args.iter_inter_style_sheet = true;
                        }
                        style_sheets_state.inline_style_sheet
                            [member_state.inline_sheet_index as usize]
                            .as_mut()
                            .unwrap()
                            .items[item_index]
                            .value = item_value.value;
                    }

                    member_state.inline_sheet_index += 1;
                }
                AppliedStyleSheet::Shared(_style_sheet) => {
                    member_state.shared_sheet_index += 1;
                    // todo:
                    // if style_sheets_state.shared_style_sheet_ids
                    //     [member_state.shared_sheet_index as usize]
                    //     != Some(style_sheet)
                    // {
                    //     // todo: remove old style sheet and add new style sheet
                    //     if let Some(_old_style_sheet_id) = style_sheets_state
                    //         .shared_style_sheet_ids
                    //         [member_state.shared_sheet_index as usize]
                    //         .clone()
                    //     {}
                    // }
                }
            }
        }
        iter_args.limit_attr_ids = Some(attr_ids.as_slice());

        iter_args.iter_and_sync_set(entity_world_mut)?;
        Ok(())
    }
}
