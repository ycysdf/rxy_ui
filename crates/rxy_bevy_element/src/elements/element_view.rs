#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use bevy_ecs::world::EntityWorldMut;
use bevy_ui::prelude::NodeBundle;

use crate::ElementType;

use super::*;

impl ElementType for view {
    fn update_entity(entity_mut: &mut EntityWorldMut) {
        entity_mut.insert(NodeBundle::default());
    }
}
