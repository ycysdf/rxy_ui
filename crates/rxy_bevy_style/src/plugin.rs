use bevy_app::{App, Plugin, Update};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Changed, Commands, EntityWorldMut, Query, Resource, World};
use bevy_ecs::query::With;
use bevy_ecs::world::FromWorld;
use bevy_ui::Interaction;
use bevy_utils::{EntityHashMap, HashMap};
use core::any::TypeId;
use core::fmt::Debug;
use derive_more::{Deref, DerefMut};
use rxy_bevy::{BevyRenderer, RendererState};
use rxy_bevy_element::{
    view_element_type, AttrIndex, AttrValue, ElementEntityExtraData, SmallBox, S1,
};
use rxy_style::{
    NodeInterStyleState, StyleAttrId, StyleInteraction, StyleSheetId, StyleSheetLocation,
};
use std::ops::AddAssign;

use crate::attr_iter::{EntityStyleWorldQuery, StateOwnerWithNodeId};
use crate::node_style_state::NodeStyleSheetsState;
use crate::{NodeStyleState, Result, StyleSheetDefinition, StyleWorldExt};

#[derive(Resource, Default, Deref, DerefMut)]
pub struct TypedEntities(HashMap<TypeId, Entity>);

#[derive(Resource)]
pub struct RxySharedStyleContainer(pub Entity);

impl FromWorld for RxySharedStyleContainer {
    fn from_world(world: &mut World) -> Self {
        Self(
            world
                .spawn((bevy_core::Name::new("[Rxy Shared Style Container]"),))
                .id(),
        )
    }
}

#[derive(Default)]
pub struct RxyStyleSheetPlugin {}

impl Plugin for RxyStyleSheetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RxySharedStyleContainer>()
            .init_resource::<TypedEntities>()
            .add_systems(Update, update_interaction_styles);
    }
}

#[derive(Component)]
pub struct Previous<T>(pub T);

pub fn interaction_to_style_kind(interaction: Interaction) -> Option<StyleInteraction> {
    match interaction {
        Interaction::Hovered => Some(StyleInteraction::Hover),
        Interaction::Pressed => Some(StyleInteraction::Active),
        _ => None,
    }
}

pub fn update_interaction_styles(
    mut commands: Commands,
    style_query: Query<&RendererState<NodeStyleSheetsState>>,
    mut entities: Query<
        (
            Entity,
            &ElementEntityExtraData,
            &RendererState<NodeInterStyleState>,
            &RendererState<NodeStyleState>,
            &Interaction,
            &mut Previous<Interaction>,
        ),
        (
            Changed<Interaction>,
            With<RendererState<NodeStyleSheetsState>>,
        ),
    >,
) {
    if entities.is_empty() {
        return;
    }

    let mut attr_changed_values: EntityHashMap<Entity, Vec<(AttrIndex, Option<StyleAttrValue>)>> =
        Default::default();
    let mut style_query = Some(style_query);
    for (
        entity,
        entity_extra_data,
        RendererState(entity_inter_style_state),
        RendererState(entity_style_state),
        interaction,
        mut previous_interaction,
    ) in entities.iter_mut()
    {
        if entity_inter_style_state.attr_infos.is_empty() {
            continue;
        }
        let prev_interaction = previous_interaction.0.clone();
        let interaction = interaction.clone();
        *previous_interaction = Previous(interaction);
        let filtered_attr_infos = entity_inter_style_state
            .attr_infos
            .iter()
            .filter(|(styled_unit_key, _)| !entity_extra_data.is_set_attr(**styled_unit_key as _));

        let entity_style_world_query = EntityStyleWorldQuery {
            query: style_query.take().unwrap(),
            current_entity: entity,
        };

        let interaction_style_kind = interaction_to_style_kind(interaction);
        for (attr_index, attr_info) in filtered_attr_infos {
            let item_value = match (prev_interaction, interaction) {
                (_, Interaction::None) => {
                    if let Some(attr_info) = entity_style_state.attr_infos.get(attr_index) {
                        let item_id = attr_info.eval_current_item_id();
                        Some(
                            entity_style_world_query
                                .get_current_style_item_value(item_id)
                                .unwrap(),
                        )
                    } else {
                        None
                    }
                }
                (Interaction::None, _interaction) => {
                    let Some(item_id) =
                        attr_info.eval_current_item_id(interaction_style_kind.unwrap(), false)
                    else {
                        continue;
                    };

                    Some(
                        entity_style_world_query
                            .get_current_style_item_value(item_id)
                            .unwrap(),
                    )
                }
                (Interaction::Hovered, Interaction::Pressed) => {
                    let Some(item_id) =
                        attr_info.eval_current_item_id(interaction_style_kind.unwrap(), true)
                    else {
                        continue;
                    };

                    Some(
                        entity_style_world_query
                            .get_current_style_item_value(item_id)
                            .unwrap(),
                    )
                }
                (Interaction::Pressed, Interaction::Hovered) => {
                    if let Some(item_id) =
                        attr_info.eval_current_item_id(interaction_style_kind.unwrap(), false)
                    {
                        Some(
                            entity_style_world_query
                                .get_current_style_item_value(item_id)
                                .unwrap(),
                        )
                    } else if let Some(attr_info) = entity_style_state.attr_infos.get(attr_index) {
                        let item_id = attr_info.eval_current_item_id();
                        Some(
                            entity_style_world_query
                                .get_current_style_item_value(item_id)
                                .unwrap(),
                        )
                    } else {
                        None
                    }
                }
                _ => {
                    continue;
                }
            };

            attr_changed_values
                .entry(entity)
                .or_default()
                .push((*attr_index, item_value.map(|n| n.value.clone())));
        }
        style_query = Some(entity_style_world_query.query);
    }

    commands.add(move |world: &mut World| {
        for (entity, changed) in attr_changed_values.into_iter() {
            let Some(mut entity_world_mut) = world.get_entity_mut(entity) else {
                continue;
            };
            let attr_is_set = entity_world_mut
                .get_mut::<ElementEntityExtraData>()
                .unwrap()
                .attr_is_set;
            for (attr_index, value) in changed.into_iter().filter(|(attr_index, _)| {
                !ElementEntityExtraData::static_is_set_attr(attr_is_set, *attr_index)
            }) {
                view_element_type()
                    .attr_by_index(attr_index as _)
                    .init_or_set(&mut entity_world_mut, value);
            }
        }
    });
}

pub type StyleAttrValue = SmallBox<dyn AttrValue, S1>;

#[derive(Debug, Clone)]
pub struct StyleItemValue {
    pub attr_id: StyleAttrId,
    pub value: StyleAttrValue,
}

pub struct StyleSheetsInfo {
    pub inline_style_sheet_count: u8,
    pub shared_style_sheet_count: u8,
}

impl AddAssign for StyleSheetsInfo {
    fn add_assign(&mut self, rhs: Self) {
        self.inline_style_sheet_count += rhs.inline_style_sheet_count;
        self.shared_style_sheet_count += rhs.shared_style_sheet_count;
    }
}

#[derive(Debug, Clone)]
pub enum AppliedStyleSheet {
    None,
    Inline(StyleSheetDefinition),
    Shared(StyleSheetId<BevyRenderer>),
}

impl AppliedStyleSheet {
    pub fn style_sheet_location(&self) -> Option<StyleSheetLocation> {
        match self {
            AppliedStyleSheet::None => None,
            AppliedStyleSheet::Inline(_) => Some(StyleSheetLocation::Inline),
            AppliedStyleSheet::Shared(_) => Some(StyleSheetLocation::Shared),
        }
    } /*

      pub fn get_style_sheet_definition<'a>(
          &'a self,
          mut query: impl StateOwnerWithNodeId<'a,'_>,
      ) -> Result<Option<&'a StyleSheetDefinition>> {
          Ok(match self {
              AppliedStyleSheet::None => None,
              AppliedStyleSheet::Inline(style_sheet) => Some(style_sheet),
              AppliedStyleSheet::Shared(style_sheet_id) => {
                  Some(query.get_current_style_sheet_definition(style_sheet_id.clone())?)
              }
          })
      }*/

    pub fn scoped_style_sheet_definition<'a, U>(
        &'a self,
        entity_world_mut: &'a mut EntityWorldMut,
        f: impl FnOnce(&mut EntityWorldMut, Option<&StyleSheetDefinition>) -> U,
    ) -> Result<U> {
        let entity = entity_world_mut.id();
        match self {
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
}
