use rxy_style::{
    NodeAttrStyleItemId, NodeStyleItemId, NodeStyleSheetId, StyleInteraction, StyleSheetIndex,
    StyleSheetLocation,
};

use crate::{AttrStyleOwner, EntityWorldRef, Result, StyleItemValue};

// impl StyleSheetDefinitionOwner for EntityWorldMut<'_> {
//     fn get_inline_style_sheet(
//         &self,
//         style_sheet_index: StyleSheetIndex,
//     ) -> Result<&StyleSheetDefinition> {
//         self.get::<RendererState<NodeStyleAttrInfos>>()
//             .map(|n| &n.0)
//             .ok_or(StyleError::NoFoundEntityStyleState)
//             .and_then(|n| n.get_inline_style_sheet(style_sheet_index))
//     }

//     fn get_shared_style_sheet<'a>(
//         &'a self,
//         style_sheet_index: StyleSheetIndex,
//         shared_style_sheets: &'a SharedStyleSheets,
//     ) -> Result<&'a StyleSheetDefinition> {
//         self.get::<RendererState<NodeStyleAttrInfos>>()
//             .map(|n| &n.0)
//             .ok_or(StyleError::NoFoundEntityStyleState)
//             .and_then(|n| n.get_shared_style_sheet(style_sheet_index, shared_style_sheets))
//     }
// }

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
        self.items
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
    pub fn add_to<T: AttrStyleOwner>(
        &self,
        attr_style_owner: &mut T,
        style_sheet_location: StyleSheetLocation,
        style_sheet_index: StyleSheetIndex,
        entity_world_ref: EntityWorldRef,
    ) -> Result<()> {
        attr_style_owner.add_attr_style_items(
            self.iter_attr_style_item_ids(style_sheet_location, style_sheet_index)
                .map(|n| T::from_definition_to_item_id(self, n).unwrap()),
            entity_world_ref,
        )?;
        Ok(())
    }
}
