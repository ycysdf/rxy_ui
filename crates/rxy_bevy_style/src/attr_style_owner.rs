use crate::{EntityWorldRef, StyleSheetDefinition};
use crate::{Result, StyleError};
use rxy_core::prelude::{Either, EitherExt};
use rxy_style::{
    NodeAttrStyleItemId, NodeInterStyleAttrInfos, NodeInterStyleItemId, NodeStyleAttrInfos,
    NodeStyleItemId, NodeStyleSheetId, StyleAttrId, StyleItemIndex,
};
use std::collections::BinaryHeap;

/*pub enum NodeStyleAttrInfosMutVariant<'a> {
    Normal(&'a mut NodeStyleAttrInfos),
    Interaction(&'a mut NodeInterStyleState),
}
pub enum NodeStyleAttrInfosRefVariant<'a> {
    Normal(&'a mut NodeStyleAttrInfos),
    Interaction(&'a mut NodeInterStyleAttrInfos),
}

*/

pub trait AttrStyleOwner {
    type ItemId;
    fn from_definition_to_item_id(
        style_sheet_definition: &StyleSheetDefinition,
        item_id: NodeAttrStyleItemId,
    ) -> Result<Self::ItemId>;

    fn add_attr_style_item(
        &mut self,
        attr_style_item_id: Self::ItemId,
        entity_world_ref: EntityWorldRef,
    ) -> Result;

    fn add_attr_style_items(
        &mut self,
        items: impl Iterator<Item = Self::ItemId> + Sized,
        entity_world_ref: EntityWorldRef,
    ) -> Result {
        for item in items {
            self.add_attr_style_item(item, entity_world_ref)?;
        }
        Ok(())
    }

    // return: require_reset
    fn remove_attr_style_item(&mut self, attr_style_item_id: Self::ItemId) -> Result<bool>;

    fn check_style_sheet_type(&self, style_sheet_definition: &StyleSheetDefinition) -> Result;

    fn remove_attr_style_of_definition(
        &mut self,
        style_sheet_definition: &StyleSheetDefinition,
        style_sheet_id: NodeStyleSheetId,
        mut require_reset_f: impl FnMut(StyleAttrId),
    ) -> Result
    where
        Self: Sized,
    {
        self.check_style_sheet_type(style_sheet_definition)?;
        for (item_index, attr_id) in style_sheet_definition
            .items
            .iter()
            .enumerate()
            .map(|n| (n.0 as StyleItemIndex, n.1.attr_id))
        {
            let attr_style_item_id = Self::from_definition_to_item_id(
                style_sheet_definition,
                NodeAttrStyleItemId {
                    attr_id,
                    item_id: NodeStyleItemId {
                        item_index,
                        sheet_id: style_sheet_id,
                    },
                },
            )?;
            if self.remove_attr_style_item(attr_style_item_id)? {
                require_reset_f(attr_id);
            }
        }
        Ok(())
    }
}

impl AttrStyleOwner for NodeInterStyleAttrInfos {
    type ItemId = NodeInterStyleItemId;
    fn from_definition_to_item_id(
        style_sheet_definition: &StyleSheetDefinition,
        item_id: NodeAttrStyleItemId,
    ) -> Result<Self::ItemId> {
        let sheet_interaction =
            style_sheet_definition.interaction.ok_or(StyleError::StyleSheetTypeIncorrect)?;
        Ok(NodeInterStyleItemId {
            style_interaction: sheet_interaction,
            style_item_id: item_id,
        })
    }

    fn add_attr_style_item(
        &mut self,
        attr_style_item_id: Self::ItemId,
        _entity_world_ref: EntityWorldRef,
    ) -> Result {
        self.entry(attr_style_item_id.style_interaction)
            .or_default()
            .add_attr_style_item(attr_style_item_id.into(), _entity_world_ref)
    }

    fn remove_attr_style_item(&mut self, attr_style_item_id: Self::ItemId) -> Result<bool> {
        self.get_mut(&attr_style_item_id.style_interaction)
            .ok_or(StyleError::NoFoundInterAttrInfos {
                item_id: attr_style_item_id,
            })?
            .remove_attr_style_item(attr_style_item_id.into())
    }

    fn check_style_sheet_type(&self, style_sheet_definition: &StyleSheetDefinition) -> Result {
        if style_sheet_definition.interaction.is_none() {
            return Err(StyleError::StyleSheetTypeIncorrect);
        }
        Ok(())
    }
}

impl AttrStyleOwner for NodeStyleAttrInfos {
    type ItemId = NodeAttrStyleItemId;

    fn from_definition_to_item_id(
        _style_sheet_definition: &StyleSheetDefinition,
        item_id: NodeAttrStyleItemId,
    ) -> Result<Self::ItemId> {
        Ok(item_id)
    }

    fn add_attr_style_item(
        &mut self,
        attr_style_item_id: NodeAttrStyleItemId,
        _entity_world_ref: EntityWorldRef,
    ) -> Result<()> {
        let value = match self.remove(&attr_style_item_id.attr_id) {
            None => attr_style_item_id.item_id.either_left().into(),
            Some(attr_info) => {
                let mut heap = attr_info
                    .0
                    .map_left(|item| {
                        let mut heap = BinaryHeap::new();
                        heap.push(item);
                        heap
                    })
                    .into_inner();
                heap.push(attr_style_item_id.item_id);
                heap.either_right().into()
            }
        };

        self.insert(attr_style_item_id.attr_id, value);
        Ok(())
    }

    fn remove_attr_style_item(&mut self, attr_style_item_id: NodeAttrStyleItemId) -> Result<bool> {
        match self.remove(&attr_style_item_id.attr_id) {
            None => Err(StyleError::NoFoundAttrId {
                attr_id: attr_style_item_id.attr_id,
            }),
            Some(value) => match value.0 {
                Either::Left(n) => {
                    assert_eq!(n, attr_style_item_id.item_id);
                    Ok(true)
                }
                Either::Right(mut heap) => {
                    let (result, heap) = if heap.peek() == Some(&attr_style_item_id.item_id) {
                        heap.pop();
                        (Ok(true), heap)
                    } else {
                        let prev_len = heap.len();
                        let heap = heap
                            .into_iter()
                            .filter(|n| n == &attr_style_item_id.item_id)
                            .collect::<BinaryHeap<NodeStyleItemId>>();
                        if heap.len() == prev_len {
                            return Err(StyleError::NoFoundStyleItemId {
                                item_id: attr_style_item_id.item_id,
                            });
                        }
                        (Ok(false), heap)
                    };
                    if !heap.is_empty() {
                        self.insert(attr_style_item_id.attr_id, heap.either_right().into());
                    }
                    result
                }
            },
        }
    }

    fn check_style_sheet_type(&self, style_sheet_definition: &StyleSheetDefinition) -> Result<()> {
        if style_sheet_definition.interaction.is_some() {
            return Err(StyleError::StyleSheetTypeIncorrect);
        }
        Ok(())
    }
}
