use super::Result;
use crate::style::{
    NodeAttrStyleItemId, NodeInterStyleAttrInfos, NodeInterStyleItemId, NodeStyleAttrInfos,
    NodeStyleItemId, NodeStyleSheetId, StyleError,
};
use crate::style::style_sheet_definition::StyleSheetDefinition;
use crate::style::view_member::StyleItemIndex;
use crate::{AttrIndex, Either, EitherExt, Renderer, RendererNodeId, RendererWorld};
use bevy_asset::AssetContainer;
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

pub trait AttrStyleOwner<R>
where
    R: Renderer,
{
    type ItemId;
    fn from_definition_to_item_id(
        style_sheet_definition: &StyleSheetDefinition,
        item_id: NodeAttrStyleItemId,
    ) -> Result<R, Self::ItemId>;

    fn add_attr_style_item(
        &mut self,
        attr_style_item_id: Self::ItemId,
        world: &RendererWorld<R>,
        node_id: RendererNodeId<R>,
    ) -> Result<R>;

    fn add_attr_style_items(
        &mut self,
        items: impl Iterator<Item = Self::ItemId> + Sized,
        world: &RendererWorld<R>,
        node_id: RendererNodeId<R>,
    ) -> Result<R> {
        for item in items {
            self.add_attr_style_item(item, world, node_id.clone())?;
        }
        Ok(())
    }

    // return: require_reset
    fn remove_attr_style_item(&mut self, attr_style_item_id: Self::ItemId) -> Result<R, bool>;

    fn check_style_sheet_type(&self, style_sheet_definition: &StyleSheetDefinition) -> Result<R>;

    fn remove_attr_style_of_definition(
        &mut self,
        style_sheet_definition: &StyleSheetDefinition,
        style_sheet_id: NodeStyleSheetId,
        mut require_reset_f: impl FnMut(AttrIndex),
    ) -> Result<R>
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

impl<R> AttrStyleOwner<R> for NodeInterStyleAttrInfos
where
    R: Renderer,
{
    type ItemId = NodeInterStyleItemId;
    fn from_definition_to_item_id(
        style_sheet_definition: &StyleSheetDefinition,
        item_id: NodeAttrStyleItemId,
    ) -> Result<R, Self::ItemId> {
        let sheet_interaction = style_sheet_definition
            .interaction
            .ok_or(StyleError::StyleSheetTypeIncorrect)?;
        Ok(NodeInterStyleItemId {
            style_interaction: sheet_interaction,
            style_item_id: item_id,
        })
    }

    fn add_attr_style_item(
        &mut self,
        attr_style_item_id: Self::ItemId,
        world: &RendererWorld<R>,
        node_id: RendererNodeId<R>,
    ) -> Result<R> {
        self.entry(attr_style_item_id.style_interaction)
            .or_default()
            .add_attr_style_item(attr_style_item_id.into(), world, node_id)
    }

    fn remove_attr_style_item(&mut self, attr_style_item_id: Self::ItemId) -> Result<R, bool> {
        self.get_mut(&attr_style_item_id.style_interaction)
            .ok_or(StyleError::NoFoundInterAttrInfos {
                item_id: attr_style_item_id,
            })?
            .remove_attr_style_item(attr_style_item_id.into())
    }

    fn check_style_sheet_type(&self, style_sheet_definition: &StyleSheetDefinition) -> Result<R> {
        if style_sheet_definition.interaction.is_none() {
            return Err(StyleError::StyleSheetTypeIncorrect);
        }
        Ok(())
    }
}

impl<R> AttrStyleOwner<R> for NodeStyleAttrInfos
where
    R: Renderer,
{
    type ItemId = NodeAttrStyleItemId;

    fn from_definition_to_item_id(
        _style_sheet_definition: &StyleSheetDefinition,
        item_id: NodeAttrStyleItemId,
    ) -> Result<R, Self::ItemId> {
        Ok(item_id)
    }

    fn add_attr_style_item(
        &mut self,
        attr_style_item_id: NodeAttrStyleItemId,
        _world: &RendererWorld<R>,
        _node_id: RendererNodeId<R>,
    ) -> Result<R, ()> {
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

    fn remove_attr_style_item(
        &mut self,
        attr_style_item_id: NodeAttrStyleItemId,
    ) -> Result<R, bool> {
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

    fn check_style_sheet_type(
        &self,
        style_sheet_definition: &StyleSheetDefinition,
    ) -> Result<R, ()> {
        if style_sheet_definition.interaction.is_some() {
            return Err(StyleError::StyleSheetTypeIncorrect);
        }
        Ok(())
    }
}
