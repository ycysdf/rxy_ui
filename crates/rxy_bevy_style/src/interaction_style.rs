use crate::attr_iter::{EntityStyleWorldQuery, StateOwnerWithNodeId};
use crate::node_style_state::NodeStyleSheetsState;
use crate::plugin::StyleAttrValue;
use crate::{Previous, StyleItemValue};
use bevy_a11y::Focus;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Changed, Commands, Query, With, World};
use bevy_ecs::system::{Command, ResMut};
use bevy_ui::Interaction;
use bevy_utils::EntityHashMap;
use derive_more::{Deref, DerefMut};
use rxy_bevy::RendererState;
use rxy_bevy_element::{view_element_type, AttrIndex, AttrSetBits, ElementEntityExtraData};
use rxy_style::{NodeInterStyleAttrInfos, NodeStyleAttrInfos, StyleInteraction};

#[derive(Default, DerefMut, Deref, Debug)]
pub struct SetAttrValuesCommand(EntityHashMap<Entity, Vec<(AttrIndex, Option<StyleAttrValue>)>>);

impl SetAttrValuesCommand {
    pub fn add(
        &mut self,
        entity: Entity,
        attr_index: AttrIndex,
        style_item_value: Option<&StyleItemValue>,
    ) {
        self.0
            .entry(entity)
            .or_default()
            .push((attr_index, style_item_value.map(|n| n.value.clone())));
    }
}

impl Command for SetAttrValuesCommand {
    fn apply(self, world: &mut World) {
        for (entity, changed) in self.0.into_iter() {
            let Some(mut entity_world_mut) = world.get_entity_mut(entity) else {
                continue;
            };
            let attr_is_set =
                entity_world_mut.get_mut::<ElementEntityExtraData>().unwrap().attr_is_set;
            for (attr_index, value) in changed.into_iter().filter_attr_already_set(attr_is_set) {
                view_element_type()
                    .attr_by_index(attr_index as _)
                    .init_or_set(&mut entity_world_mut, value);
            }
        }
    }
}

pub fn interaction_to_style_interaction(interaction: Interaction) -> Option<StyleInteraction> {
    match interaction {
        Interaction::Hovered => Some(StyleInteraction::Hover),
        Interaction::Pressed => Some(StyleInteraction::Active),
        _ => None,
    }
}

pub trait IterExt<M> {
    fn filter_attr_already_set(
        self,
        attr_is_set: AttrSetBits,
    ) -> impl Iterator<Item = (AttrIndex, M)>;
}

impl<T, M> IterExt<M> for T
where
    T: Iterator<Item = (AttrIndex, M)>,
{
    fn filter_attr_already_set(
        self,
        attr_is_set: AttrSetBits,
    ) -> impl Iterator<Item = (AttrIndex, M)> {
        self.filter(move |(attr_index, _)| {
            !ElementEntityExtraData::static_is_set_attr(attr_is_set, *attr_index)
        })
    }
}

pub fn update_interaction_styles(
    mut commands: Commands,
    style_sheets_query: Query<&RendererState<NodeStyleSheetsState>>,
    mut inter_styled_query: Query<
        (
            Entity,
            &ElementEntityExtraData,
            &RendererState<NodeInterStyleAttrInfos>,
            &RendererState<NodeStyleAttrInfos>,
            &Interaction,
            &mut Previous<Interaction>,
        ),
        (
            Changed<Interaction>,
            With<RendererState<NodeStyleSheetsState>>,
        ),
    >,
    mut focus: ResMut<Focus>,
) {
    if inter_styled_query.is_empty() {
        return;
    }
    let mut set_attrs_cmd = SetAttrValuesCommand::default();

    let mut style_sheets_query = Some(style_sheets_query);
    let mut attr_bits: AttrSetBits;
    for (
        entity,
        entity_extra_data,
        RendererState(entity_inter_style_state),
        RendererState(entity_style_state),
        interaction,
        mut previous_interaction,
    ) in inter_styled_query.iter_mut()
    {
        if entity_inter_style_state.is_empty() {
            continue;
        }
        let prev_interaction = previous_interaction.0.clone();
        let interaction = interaction.clone();
        *previous_interaction = Previous(interaction);

        let entity_style_world_query = EntityStyleWorldQuery {
            query: style_sheets_query.take().unwrap(),
            current_entity: entity,
        };
        attr_bits = 0;

        let style_interaction = interaction_to_style_interaction(interaction);
        let pre_style_interaction = interaction_to_style_interaction(prev_interaction);

        let is_focused = focus.0 == Some(entity);
        match (prev_interaction, interaction) {
            (_, Interaction::None) => {
                for (attr_index, _matched_interaction) in entity_inter_style_state
                    .iter_match_attr_ids(
                        // Update all attrs for the first time (prev interaction is not correct at this point)
                        if prev_interaction == Interaction::None {
                            Some(StyleInteraction::all())
                        } else {
                            pre_style_interaction
                        },
                        false,
                    )
                    .filter_attr_already_set(entity_extra_data.attr_is_set | attr_bits)
                {
                    // todo: extract to type
                    if (attr_bits >> attr_index) & 1 == 1 {
                        continue;
                    }
                    attr_bits |= 1 << attr_index;
                    let value = if is_focused {
                        entity_inter_style_state
                            .get_attr_info(StyleInteraction::Focus, attr_index)
                            .or_else(|| entity_style_state.get(&attr_index))
                    } else {
                        entity_style_state.get(&attr_index)
                    }
                    .map(|attr_info| {
                        entity_style_world_query
                            .get_current_style_item_value(attr_info.top_item_id())
                            .unwrap()
                    });
                    set_attrs_cmd.add(entity, attr_index, value);
                }
            }
            (Interaction::None, _)
            | (Interaction::Hovered, Interaction::Pressed)
            | (Interaction::Pressed, Interaction::Hovered) => {
                // todo: code decoupling
                if interaction == Interaction::Pressed && focus.0 != Some(entity) {
                    *focus = Focus(Some(entity));
                }

                for (attr_index, matched_interaction) in entity_inter_style_state
                    .iter_match_attr_ids(
                        style_interaction,
                        prev_interaction == Interaction::Pressed
                            && interaction == Interaction::Hovered,
                    )
                    .filter_attr_already_set(entity_extra_data.attr_is_set | attr_bits)
                {
                    if (attr_bits >> attr_index) & 1 == 1 {
                        continue;
                    }
                    attr_bits |= 1 << attr_index;
                    let value = entity_inter_style_state
                        .get_attr_info(matched_interaction, attr_index)
                        .map(|attr_info| {
                            entity_style_world_query
                                .get_current_style_item_value(attr_info.top_item_id())
                                .unwrap()
                        });
                    set_attrs_cmd.add(entity, attr_index, value);
                }
            }
            _ => {
                continue;
            }
        }

        style_sheets_query = Some(entity_style_world_query.query);
    }

    commands.add(set_attrs_cmd);
}
