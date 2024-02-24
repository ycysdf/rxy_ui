use bevy_ecs::entity::EntityHashMap;
use bevy_ecs::prelude::Resource;
use taffy::Taffy;

#[derive(Resource, Default)]
pub struct UiSurface {
    entity_to_taffy: EntityHashMap<taffy::node::Node>,
    taffy: Taffy,
}
