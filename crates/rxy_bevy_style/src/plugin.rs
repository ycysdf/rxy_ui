use crate::focus_style::update_focus_style;
use crate::interaction_style::update_interaction_styles;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{DetectChanges, IntoSystemConfigs, Res, Resource, World};
use bevy_ecs::world::FromWorld;
use bevy_utils::HashMap;
use core::any::TypeId;
use core::fmt::Debug;
use derive_more::{Deref, DerefMut};
use rxy_bevy::FocusedEntity;

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
