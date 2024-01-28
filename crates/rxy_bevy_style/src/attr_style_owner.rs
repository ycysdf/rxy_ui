use crate::{EntityWorldRef, NodeStyleState, StyleSheetDefinition};
use crate::{Result, StyleError};
use rxy_core::prelude::{Either, EitherExt};
use rxy_style::{
    NodeAttrStyleItemId, NodeInterStyleItemId, NodeInterStyleState, NodeStyleItemId,
    NodeStyleSheetId, StyleAttrId, StyleItemIndex,
};

/*pub enum NodeStyleStateMutVariant<'a> {
    Normal(&'a mut NodeStyleState),
    Interaction(&'a mut NodeInterStyleState),
}*/

pub enum NodeStyleStateRefVariant<'a> {
    Normal(&'a mut NodeStyleState),
    Interaction(&'a mut NodeInterStyleState),
}

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
    fn remove_attr_style_item(&mut self, attr_style_item_id: NodeAttrStyleItemId) -> Result<bool>;

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
            let attr_style_item_id = NodeAttrStyleItemId {
                attr_id,
                item_id: NodeStyleItemId {
                    item_index,
                    sheet_id: style_sheet_id,
                },
            };
            if self.remove_attr_style_item(attr_style_item_id)? {
                require_reset_f(attr_id);
            }
        }
        Ok(())
    }


}

/*impl<'a> AttrStyleOwner for NodeStyleStateMutVariant<'a> {
    fn add_attr_style_item(
        &mut self,
        attr_style_item_id: NodeAttrStyleItemId,
        entity_world_ref: EntityWorldRef,
    ) -> Result {
        match self {
            NodeStyleStateMutVariant::Normal(n) => {
                n.add_attr_style_item(attr_style_item_id, entity_world_ref)
            }
            NodeStyleStateMutVariant::Interaction(n) => {
                n.add_attr_style_item(attr_style_item_id, entity_world_ref)
            }
        }
    }

    fn remove_attr_style_item(&mut self, attr_style_item_id: NodeAttrStyleItemId) -> Result<bool> {
        match self {
            NodeStyleStateMutVariant::Normal(n) => n.remove_attr_style_item(attr_style_item_id),
            NodeStyleStateMutVariant::Interaction(n) => {
                n.remove_attr_style_item(attr_style_item_id)
            }
        }
    }

    fn check_style_sheet_type(&self, style_sheet_definition: &StyleSheetDefinition) -> Result {
        match self {
            NodeStyleStateMutVariant::Normal(_) => {
                if style_sheet_definition.interaction.is_some() {
                    return Err(StyleError::StyleSheetTypeIncorrect);
                }
            }
            NodeStyleStateMutVariant::Interaction(_) => {
                if style_sheet_definition.interaction.is_none() {
                    return Err(StyleError::StyleSheetTypeIncorrect);
                }
            }
        }
        Ok(())
    }
}*/

impl AttrStyleOwner for NodeInterStyleState {
    type ItemId = NodeInterStyleItemId;
    fn remove_attr_style_item(&mut self, attr_style_item_id: NodeAttrStyleItemId) -> Result<bool> {
        match self.attr_infos.remove(&attr_style_item_id.attr_id) {
            None => Err(StyleError::NoFoundAttrId {
                attr_id: attr_style_item_id.attr_id,
            }),
            Some(value) => match value.0 {
                Either::Left(n) => {
                    assert_eq!(n.item_id, attr_style_item_id.item_id);
                    Ok(true)
                }
                Either::Right(mut vec) => {
                    let (result, vec) = {
                        let Some(index) = vec
                            .iter()
                            .position(|n| n.item_id == attr_style_item_id.item_id)
                        else {
                            return Err(StyleError::NoFoundStyleItemId {
                                item_id: attr_style_item_id.item_id,
                            });
                        };
                        vec.swap_remove(index);
                        (Ok(true), vec)
                    };
                    if !vec.is_empty() {
                        self.attr_infos
                            .insert(attr_style_item_id.attr_id, vec.either_right().into());
                    }
                    result
                }
            },
        }
    }

    fn add_attr_style_item(
        &mut self,
        attr_style_item_id: NodeInterStyleItemId,
        _entity_world_ref: EntityWorldRef,
    ) -> Result {
        let value = match self.attr_infos.remove(&attr_style_item_id.attr_id) {
            None => attr_style_item_id.either_left().into(),
            Some(items) => {
                let mut vec = items.0.map_left(|item| vec![item]).into_inner();
                vec.push(attr_style_item_id);
                vec.either_right().into()
            }
        };
        self.attr_infos.insert(attr_style_item_id.attr_id, value);
        Ok(())
    }

    fn check_style_sheet_type(&self, style_sheet_definition: &StyleSheetDefinition) -> Result {
        if style_sheet_definition.interaction.is_none() {
            return Err(StyleError::StyleSheetTypeIncorrect);
        }
        Ok(())
    }

    fn from_definition_to_item_id(
        style_sheet_definition: &StyleSheetDefinition,
        item_id: NodeAttrStyleItemId,
    ) -> Result<Self::ItemId> {
        let sheet_interaction = style_sheet_definition
            .interaction
            .ok_or(StyleError::StyleSheetTypeIncorrect)?;
        Ok(NodeInterStyleItemId {
            sheet_interaction,
            style_item_id: item_id,
        })
    }
}
