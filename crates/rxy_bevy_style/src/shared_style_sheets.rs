use bevy_ecs::entity::Entity;
use bevy_utils::HashSet;

#[derive(Default, Clone)]
pub struct SharedStyleState {
    pub subscribers: HashSet<Entity>,
}

impl SharedStyleState {
    pub fn add_subscriber(&mut self, entity: Entity) {
        self.subscribers.insert(entity);
    }

    pub fn remove_subscriber(&mut self, entity: Entity) {
        self.subscribers.remove(&entity);
    }
}
