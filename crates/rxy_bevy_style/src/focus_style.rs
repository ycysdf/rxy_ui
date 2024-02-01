use crate::attr_iter::{EntityStyleWorldQuery, StateOwnerWithNodeId};
use crate::interaction_style::{IterExt, SetAttrValuesCommand};
use crate::node_style_state::NodeStyleSheetsState;
use crate::Previous;
use bevy_a11y::Focus;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Query, Res};
use bevy_ecs::system::{Commands, ResMut};
use rxy_bevy::RendererState;
use rxy_bevy_element::ElementEntityExtraData;
use rxy_style::{NodeInterStyleAttrInfos, NodeStyleAttrInfos, StyleInteraction};

pub fn update_focus_style(
    mut commands: Commands,
    style_sheets_query: Query<&RendererState<NodeStyleSheetsState>>,
    styled_query: Query<(
        &ElementEntityExtraData,
        &RendererState<NodeInterStyleAttrInfos>,
        &RendererState<NodeStyleAttrInfos>,
    )>,
    focus: Res<Focus>,
    mut previous_focus: ResMut<Previous<Focus>>,
) {
    let mut set_attrs_cmd = SetAttrValuesCommand::default();

    let focus_entity = focus.0;
    let previous_focus_entity = previous_focus.0 .0;
    *previous_focus = Previous(Focus(focus_entity));

    do_f(
        previous_focus_entity,
        focus_entity,
        &styled_query,
        &mut set_attrs_cmd,
        style_sheets_query,
    );
    commands.add(set_attrs_cmd);
}

fn do_f<'a, 'world, 'state>(
    previous_focus_entity: Option<Entity>,
    focus_entity: Option<Entity>,
    styled_query: &Query<(
        &ElementEntityExtraData,
        &RendererState<NodeInterStyleAttrInfos>,
        &RendererState<NodeStyleAttrInfos>,
    )>,
    set_attrs_cmd: &mut SetAttrValuesCommand,
    style_sheets_query: Query<'world, 'state, &'a RendererState<NodeStyleSheetsState>>,
) -> Query<'world, 'state, &'a RendererState<NodeStyleSheetsState>> {
    match (previous_focus_entity, focus_entity) {
        (None, Some(focus_entity)) => {
            let Ok((entity_extra_data, RendererState(inter_attr_infos), _)) =
                styled_query.get(focus_entity)
            else {
                return style_sheets_query;
            };
            let Some(focus_attr_infos) = inter_attr_infos.get(&StyleInteraction::Focus) else {
                return style_sheets_query;
            };

            let entity_style_world_query = EntityStyleWorldQuery {
                query: style_sheets_query,
                current_entity: focus_entity,
            };

            for (attr_index, attr_info) in focus_attr_infos
                .iter()
                .map(|n| (*n.0, ()))
                .filter_attr_already_set(entity_extra_data.attr_is_set)
                .filter_map(|(attr_index, _)| {
                    focus_attr_infos.get(&attr_index).map(|attr_info| (attr_index, attr_info))
                })
            {
                let value = entity_style_world_query
                    .get_current_style_item_value(attr_info.top_item_id())
                    .unwrap();
                set_attrs_cmd.add(focus_entity, attr_index, Some(value));
            }
            entity_style_world_query.query
        }
        (Some(previous_focus), None) => {
            let Ok((
                entity_extra_data,
                RendererState(inter_attr_infos),
                RendererState(attr_infos),
            )) = styled_query.get(previous_focus)
            else {
                return style_sheets_query;
            };
            let Some(focus_attr_infos) =
                inter_attr_infos.get(&StyleInteraction::Focus).map(|n| n.keys())
            else {
                return style_sheets_query;
            };

            let entity_style_world_query = EntityStyleWorldQuery {
                query: style_sheets_query,
                current_entity: previous_focus,
            };

            for (attr_index, _) in focus_attr_infos
                .map(|n| (*n, ()))
                .filter_attr_already_set(entity_extra_data.attr_is_set)
            {
                let value = attr_infos.get(&attr_index).map(|attr_info| {
                    entity_style_world_query
                        .get_current_style_item_value(attr_info.top_item_id())
                        .unwrap()
                });
                set_attrs_cmd.add(previous_focus, attr_index, value);
            }
            entity_style_world_query.query
        }
        (Some(previous_focus_entity), Some(focus_entity)) => {
            let style_sheets_query = do_f(
                Some(previous_focus_entity),
                None,
                styled_query,
                set_attrs_cmd,
                style_sheets_query,
            );
            do_f(
                None,
                Some(focus_entity),
                styled_query,
                set_attrs_cmd,
                style_sheets_query,
            )
        }
        _ => style_sheets_query,
    }
}
