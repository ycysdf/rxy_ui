use crate::focus_style::update_focus_style;
use crate::interaction_style::update_interaction_styles;
use crate::{Result, StyleSheetDefinition, StyleWorldExt};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{DetectChanges, EntityWorldMut, IntoSystemConfigs, Res, Resource, World};
use bevy_ecs::world::FromWorld;
use bevy_utils::HashMap;
use core::any::TypeId;
use core::fmt::Debug;
use derive_more::{Deref, DerefMut};
use rxy_bevy::{BevyRenderer, FocusedEntity};
use rxy_bevy_element::{AttrValue, SmallBox, S1};
use rxy_style::{StyleAttrId, StyleSheetId, StyleSheetLocation};
use std::ops::AddAssign;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct TypedEntities(HashMap<TypeId, Entity>);

#[derive(Resource)]
pub struct RxySharedStyleContainer(pub Entity);

impl FromWorld for RxySharedStyleContainer {
    fn from_world(world: &mut World) -> Self {
        Self(world.spawn((bevy_core::Name::new("[Rxy Shared Style Container]"),)).id())
    }
}

#[derive(Default)]
pub struct RxyStyleSheetPlugin {}

impl Plugin for RxyStyleSheetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RxySharedStyleContainer>()
            .init_resource::<TypedEntities>()
            .init_resource::<Previous<FocusedEntity>>()
            .add_systems(
                Update,
                (
                    update_interaction_styles.after(update_focus_style),
                    update_focus_style.run_if(|res: Res<FocusedEntity>| res.is_changed()),
                ),
            );
    }
}

#[derive(Default, Component, Resource, Clone, Debug, Deref, DerefMut)]
pub struct Previous<T>(pub T);

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
