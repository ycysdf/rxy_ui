#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use bevy_reflect::Reflect;
use bevy_ui::node_bundles::ImageBundle;

use rxy_core::{ElementType, ElementTypeUnTyped, RendererNodeId, RendererWorld};

use crate::{BevyRenderer, BevyWorldExt};

#[derive(Reflect, Default, Debug, Clone, Copy)]
pub struct element_img;

impl ElementType<BevyRenderer> for element_img {
   const TAG_NAME: &'static str = "img";

   fn get() -> &'static dyn ElementTypeUnTyped<BevyRenderer> {
      &element_img
   }

   fn spawn(
      world: &mut RendererWorld<BevyRenderer>,
      parent: Option<&RendererNodeId<BevyRenderer>>,
      reserve_node_id: Option<RendererNodeId<BevyRenderer>>,
   ) -> RendererNodeId<BevyRenderer> {
      let mut entity_world_mut = world.get_or_spawn_empty(parent, reserve_node_id);
      entity_world_mut.insert(ImageBundle::default());
      entity_world_mut.id()
   }
}

pub mod element_img_attrs {
   use bevy_asset::Handle;
   use bevy_render::texture::Image;
   use bevy_ui::UiImage;

   use rxy_core::{ElementAttrType, RendererNodeId, RendererWorld};

   use crate::BevyRenderer;

   #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
   pub struct src;

   impl ElementAttrType<BevyRenderer> for src {
      type Value = Handle<Image>;

      const NAME: &'static str = stringify!(src);

      fn update_value(
         world: &mut RendererWorld<BevyRenderer>,
         node_id: RendererNodeId<BevyRenderer>,
         value: impl Into<Self::Value>,
      ) {
         if let Some(mut ui_image) = world.get_mut::<UiImage>(node_id) {
            ui_image.texture = value.into();
         }
      }
   }

   #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
   pub struct flip_x;

   impl ElementAttrType<BevyRenderer> for flip_x {
      type Value = bool;

      const NAME: &'static str = stringify!(flip_x);

      fn update_value(
         world: &mut RendererWorld<BevyRenderer>,
         node_id: RendererNodeId<BevyRenderer>,
         value: impl Into<Self::Value>,
      ) {
         if let Some(mut ui_image) = world.get_mut::<UiImage>(node_id) {
            ui_image.flip_x = value.into();
         }
      }
   }

   #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
   pub struct flip_y;

   impl ElementAttrType<BevyRenderer> for flip_y {
      type Value = bool;

      const NAME: &'static str = stringify!(flip_y);

      fn update_value(
         world: &mut RendererWorld<BevyRenderer>,
         node_id: RendererNodeId<BevyRenderer>,
         value: impl Into<Self::Value>,
      ) {
         if let Some(mut ui_image) = world.get_mut::<UiImage>(node_id) {
            ui_image.flip_y = value.into();
         }
      }
   }
}
