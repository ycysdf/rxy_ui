#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;

use rxy_core::{ElementType, ElementTypeUnTyped, RendererNodeId, RendererWorld};

use crate::{ImageBundle, NativeRenderer};
use crate::world_ext::BevyWorldExt;

#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default))]
pub struct element_img;

impl ElementType<NativeRenderer> for element_img {
   const TAG_NAME: &'static str = "img";

   fn get() -> &'static dyn ElementTypeUnTyped<NativeRenderer> {
      &element_img
   }

   fn spawn(
      world: &mut RendererWorld<NativeRenderer>,
      parent: Option<&RendererNodeId<NativeRenderer>>,
      reserve_node_id: Option<RendererNodeId<NativeRenderer>>,
   ) -> RendererNodeId<NativeRenderer> {
      let mut entity_world_mut = world.get_or_spawn_empty(parent, reserve_node_id);
      // entity_world_mut.insert(ImageBundle::default());
      entity_world_mut.id()
   }
}

pub mod element_img_attrs {
   use std::sync::Arc;
   use vello::peniko::{Blob, Format, Image};
   use rxy_core::{ElementAttrType, RendererNodeId, RendererWorld};

   use crate::NativeRenderer;

   #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
   pub struct src;

   // impl ElementAttrType<NativeRenderer> for src {
   //    type Value = Handle<Image>;
   //
   //    const NAME: &'static str = stringify!(src);
   //
   //    fn update_value(
   //       world: &mut RendererWorld<NativeRenderer>,
   //       node_id: RendererNodeId<NativeRenderer>,
   //       value: impl Into<Self::Value>,
   //    ) {
   //
   //       if let Some(mut ui_image) = world.get_mut::<UiImage>(node_id) {
   //          ui_image.texture = value.into();
   //       }
   //    }
   // }
   //
   // fn decode_image(data: &[u8]) -> anyhow::Result<Image> {
   //    let image = image::io::Reader::new(std::io::Cursor::new(data))
   //        .with_guessed_format()?
   //        .decode()?;
   //    let width = image.width();
   //    let height = image.height();
   //    let data = Arc::new(image.into_rgba8().into_vec());
   //    let blob = Blob::new(data);
   //    Ok(Image::new(blob, Format::Rgba8, width, height))
   // }

   // #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
   // pub struct flip_x;
   //
   // impl ElementAttrType<NativeRenderer> for flip_x {
   //    type Value = bool;
   //
   //    const NAME: &'static str = stringify!(flip_x);
   //
   //    fn update_value(
   //       world: &mut RendererWorld<NativeRenderer>,
   //       node_id: RendererNodeId<NativeRenderer>,
   //       value: impl Into<Self::Value>,
   //    ) {
   //       if let Some(mut ui_image) = world.get_mut::<UiImage>(node_id) {
   //          ui_image.flip_x = value.into();
   //       }
   //    }
   // }
   //
   // #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
   // pub struct flip_y;
   //
   // impl ElementAttrType<NativeRenderer> for flip_y {
   //    type Value = bool;
   //
   //    const NAME: &'static str = stringify!(flip_y);
   //
   //    fn update_value(
   //       world: &mut RendererWorld<NativeRenderer>,
   //       node_id: RendererNodeId<NativeRenderer>,
   //       value: impl Into<Self::Value>,
   //    ) {
   //       if let Some(mut ui_image) = world.get_mut::<UiImage>(node_id) {
   //          ui_image.flip_y = value.into();
   //       }
   //    }
   // }
}
