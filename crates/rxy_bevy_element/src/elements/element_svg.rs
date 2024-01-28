/* #![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use bevy::asset::AssetLoader;
use bevy::ecs::world::EntityMut;
use bevy::prelude::{AssetServer, Handle};
use bevy::sprite::Mesh2dHandle;
use bevy_svg::prelude::{Origin, Svg};

use crate::element_core::{DioxusAttributeDescription, SchemaProp, SchemaPropUntyped, SchemaType};
use crate::element_attrs::COMMON_PROPS_COUNT;
use crate::{common_props_define, impl_schema_type, element_attrs};

impl_schema_type!(svg, src);

pub struct src;

impl SchemaProp for src {
    const TAG_NAME: &'static str = stringify!(src);
    const INDEX: u8 = COMMON_PROPS_COUNT + 0;

    type Value = String;

    fn set_value(&self, entity_ref: &mut EntityMut, value: impl Into<Self::Value>) {
        let asset_server = entity_ref.world().resource::<AssetServer>();
        entity_ref.insert::<Handle<Svg>>(asset_server.load(&value.into()));
        if !entity_ref.contains::<Origin>() {
            entity_ref.insert(Origin::Center);
        }
        if !entity_ref.contains::<Mesh2dHandle>() {
            entity_ref.insert(Mesh2dHandle::default());
        }
    }
}
/* pub struct origin;

impl SchemaProp for origin {
    const TAG_NAME: &'static str = stringify!(origin);
    const INDEX: u8 = COMMON_PROPS_COUNT + 0;

    type Value = Origin;

    fn set_value(&self, entity_ref: &mut EntityMut, value: impl Into<Self::Value>) {
        entity_ref.insert(value.into());
    }
}
 */
 */
