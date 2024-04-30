use bitflags::bitflags;

use crate::style::attr_style_owner::AttrStyleOwner;
use crate::style::view_member::{StyleSheetIndex, StyleSheetLocation};
use crate::style::StyleItemValue;
use crate::style::{NodeAttrStyleItemId, NodeStyleItemId, NodeStyleSheetId};
use crate::{EitherExt, Renderer, RendererNodeId, RendererWorld};

use super::Result;
use alloc::vec::Vec;
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
    pub struct StyleInteraction: u8 {
        const Focus  = 0b00000001;
        const Hover  = 0b00000010;
        const Active = 0b00000110;
    }
}

impl StyleInteraction {
   pub fn is_match(self, interaction: StyleInteraction, strict: bool) -> bool {
      if strict {
         self == interaction
      } else {
         self.contains(interaction)
      }
   }

   pub fn priority_iter() -> impl Iterator<Item = Self> {
      [Self::Active, Self::Hover, Self::Focus].into_iter()
   }

   pub fn match_iter(self, strict: bool) -> impl Iterator<Item = Self> {
      if strict {
         Some(self).into_iter().either_left()
      } else {
         Self::priority_iter()
            .filter(move |n| self.contains(*n))
            .either_right()
      }
   }
}

#[derive(Clone, Default, Debug)]
pub struct StyleSheetDefinition {
   pub interaction: Option<StyleInteraction>,
   pub items: Vec<StyleItemValue>,
}

impl StyleSheetDefinition {
   pub fn iter_attr_style_item_ids(
      &self,
      style_sheet_location: StyleSheetLocation,
      style_sheet_index: StyleSheetIndex,
   ) -> impl Iterator<Item = NodeAttrStyleItemId> + '_ {
      self
         .items
         .iter()
         .enumerate()
         .map(move |(item_index, item)| NodeAttrStyleItemId {
            attr_id: item.attr_id,
            item_id: NodeStyleItemId {
               item_index: item_index as _,

               sheet_id: NodeStyleSheetId {
                  index: style_sheet_index,
                  location: style_sheet_location,
               },
            },
         })
   }
   pub fn add_to<R, T>(
      &self,
      attr_style_owner: &mut T,
      style_sheet_location: StyleSheetLocation,
      style_sheet_index: StyleSheetIndex,
      world: &RendererWorld<R>,
      node_id: RendererNodeId<R>,
   ) -> Result<R>
   where
      R: Renderer,
      T: AttrStyleOwner<R>,
   {
      attr_style_owner.add_attr_style_items(
         self
            .iter_attr_style_item_ids(style_sheet_location, style_sheet_index)
            .map(|n| T::from_definition_to_item_id(self, n).unwrap()),
         world,
         node_id,
      )?;
      Ok(())
   }
}
