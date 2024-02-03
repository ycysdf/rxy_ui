use bevy_a11y::{AccessibilitySystem, Focus};
use bevy_app::prelude::*;
use bevy_core::Name;
use bevy_ecs::prelude::{
    Entity, FromWorld, IntoSystemConfigs, RemovedComponents, Res, Resource, World,
};
use bevy_ecs::system::ResMut;
use bevy_mod_picking::prelude::*;
use bevy_render::prelude::Visibility;
use bevy_ui::prelude::NodeBundle;
use bevy_ui::widget::TextFlags;
use bevy_ui::Style;

use rxy_bevy_element::attr_values::BevyAppAttrValueRegistryExt;
use rxy_bevy_element::BevyDioxusAppExt;

use crate::{handle_schedule_event, CommandChannelPlugin, Focusable, ScheduleSystemAdds};

#[derive(Resource)]
pub struct RxyContainerEntity {
    pub entity: Entity,
}

impl FromWorld for RxyContainerEntity {
    fn from_world(world: &mut World) -> Self {
        let world_mut = world.spawn((
            NodeBundle {
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            Name::new("[Rxy Ui Slots Container]"),
        ));
        Self {
            entity: world_mut.id(),
        }
    }
}

#[derive(Default)]
pub struct RxyPlugin {
    root_entity: Option<Entity>,
}

#[derive(Resource)]
pub struct RxyRootEntity(pub Entity);

impl Plugin for RxyPlugin {
    fn build(&self, app: &mut App) {
        let root_entity = RxyRootEntity(self.root_entity.unwrap_or_else(|| {
            app.world
                .spawn((
                    NodeBundle {
                        style: Style {
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Name::new("[Rxy Ui Root]"),
                ))
                .id()
        }));

        app.add_plugins((DefaultPickingPlugins, CommandChannelPlugin))
            .insert_resource(root_entity)
            .register_type::<TextFlags>()
            .register_type::<PickingInteraction>()
            .init_resource::<ScheduleSystemAdds>()
            .register_attr_values()
            .register_elements_type()
            .add_systems(
                First,
                handle_schedule_event
                    .run_if(|systems: Res<ScheduleSystemAdds>| !systems.systems.is_empty()),
            )
            .add_systems(
                PostUpdate,
                check_focus
                    .before(AccessibilitySystem::Update)
                    .run_if(|removed: RemovedComponents<Focusable>| !removed.is_empty()),
            );
    }
}
fn check_focus(mut focus: ResMut<Focus>, mut removed: RemovedComponents<Focusable>) {
    for entity in removed.read() {
        if focus.0 == Some(entity) {
            focus.0 = None;
        }
    }
}
