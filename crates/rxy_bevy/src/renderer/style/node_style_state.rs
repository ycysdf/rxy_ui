use super::{Result, StyleError};
use bevy_ecs::prelude::Entity;
use super::rxy_bevy_crate::BevyRenderer;
use rxy_core::style::{
    AppliedStyleSheet, ApplyStyleSheetsMemberState, NodeStyleSheetId, StyleSheetDefinition,
    StyleSheetId, StyleSheetIndex, StyleSheetLocation, StyleSheetsInfo,
};

#[derive(Default, Clone, Debug)]
pub struct NodeStyleSheetsState {
    pub inline_style_sheet: Vec<Option<StyleSheetDefinition>>,
    pub shared_style_sheet_ids: Vec<Option<StyleSheetId<BevyRenderer>>>,
}

impl FromIterator<AppliedStyleSheet<BevyRenderer>> for NodeStyleSheetsState {
    fn from_iter<T: IntoIterator<Item = AppliedStyleSheet<BevyRenderer>>>(iter: T) -> Self {
        let mut r = NodeStyleSheetsState::default();
        for item in iter.into_iter() {
            match item {
                AppliedStyleSheet::None => {}
                AppliedStyleSheet::Inline(style_sheet) => {
                    r.inline_style_sheet.push(Some(style_sheet));
                }
                AppliedStyleSheet::Shared(style_sheet_id) => {
                    r.shared_style_sheet_ids.push(Some(style_sheet_id));
                }
            }
        }
        r
    }
}

impl NodeStyleSheetsState {
    pub fn apply_as_shared(
        &self,
        entity: Entity,
        index: StyleSheetIndex,
    ) -> impl Iterator<Item = AppliedStyleSheet<BevyRenderer>> + Send + 'static {
        let shared_style_sheet_ids = self.shared_style_sheet_ids.clone();
        let inline_style_sheet_len = self.inline_style_sheet.len();
        (0..inline_style_sheet_len)
            .map(move |i| {
                AppliedStyleSheet::Shared(StyleSheetId {
                    node_style_sheet_id: NodeStyleSheetId {
                        index: index + i as StyleSheetIndex,
                        location: StyleSheetLocation::Inline,
                    },
                    node_id: entity,
                })
            })
            .chain(
                shared_style_sheet_ids
                    .into_iter()
                    .flatten()
                    .map(|n| AppliedStyleSheet::Shared(n.clone())),
            )
    }

    pub fn style_sheets_info(&self) -> StyleSheetsInfo {
        StyleSheetsInfo {
            inline_style_sheet_count: self.inline_style_sheet.len() as _,
            shared_style_sheet_count: self.shared_style_sheet_ids.len() as _,
        }
    }
}

impl NodeStyleSheetsState {
    pub fn get_inline_style_sheet(
        &self,
        style_sheet_index: StyleSheetIndex,
    ) -> Result<&StyleSheetDefinition> {
        self.inline_style_sheet
            .get(style_sheet_index as usize)
            .ok_or(StyleError::NoFoundStyleSheetOnNode(NodeStyleSheetId {
                location: StyleSheetLocation::Inline,
                index: style_sheet_index,
            }))?
            .as_ref()
            .ok_or(StyleError::RemovedStyleSheet(NodeStyleSheetId {
                location: StyleSheetLocation::Inline,
                index: style_sheet_index,
            }))
    }
    pub fn get_inline_style_sheet_mut(
        &mut self,
        style_sheet_index: StyleSheetIndex,
    ) -> Result<&mut StyleSheetDefinition> {
        self.inline_style_sheet
            .get_mut(style_sheet_index as usize)
            .ok_or(StyleError::NoFoundStyleSheetOnNode(NodeStyleSheetId {
                location: StyleSheetLocation::Inline,
                index: style_sheet_index,
            }))?
            .as_mut()
            .ok_or(StyleError::RemovedStyleSheet(NodeStyleSheetId {
                location: StyleSheetLocation::Inline,
                index: style_sheet_index,
            }))
    }

    pub fn get_shared_style_sheet_id(
        &self,
        style_sheet_index: StyleSheetIndex,
    ) -> Result<StyleSheetId<BevyRenderer>> {
        let style_sheet_id = self
            .shared_style_sheet_ids
            .get(style_sheet_index as usize)
            .ok_or(StyleError::NoFoundStyleSheetOnNode(NodeStyleSheetId {
                location: StyleSheetLocation::Shared,
                index: style_sheet_index,
            }))?
            .as_ref()
            .ok_or(StyleError::RemovedStyleSheet(NodeStyleSheetId {
                location: StyleSheetLocation::Shared,
                index: style_sheet_index,
            }))?;
        Ok(style_sheet_id.clone())
    }

    pub fn get_style_sheet_len(&self, location: StyleSheetLocation) -> StyleSheetIndex {
        (match location {
            StyleSheetLocation::Inline => self.inline_style_sheet.len(),
            StyleSheetLocation::Shared => self.shared_style_sheet_ids.len(),
        }) as _
    }
    pub fn push_applied_style_sheet(
        &mut self,
        applied_style_sheet: AppliedStyleSheet<BevyRenderer>,
    ) {
        match applied_style_sheet {
            AppliedStyleSheet::None => {}
            AppliedStyleSheet::Inline(style_sheet) => {
                self.inline_style_sheet.push(Some(style_sheet));
            }
            AppliedStyleSheet::Shared(style_sheet_id) => {
                self.shared_style_sheet_ids.push(Some(style_sheet_id));
            }
        }
    }
    pub fn set_applied_style_sheet(
        &mut self,
        style_sheet_index: StyleSheetIndex,
        applied_style_sheet: AppliedStyleSheet<BevyRenderer>,
    ) {
        match applied_style_sheet {
            AppliedStyleSheet::None => {
                self.inline_style_sheet[style_sheet_index as usize] = None;
            }
            AppliedStyleSheet::Inline(style_sheet_definition) => {
                self.inline_style_sheet[style_sheet_index as usize] = Some(style_sheet_definition);
            }
            AppliedStyleSheet::Shared(style_sheet_id) => {
                self.shared_style_sheet_ids[style_sheet_index as usize] = Some(style_sheet_id);
            }
        }
    }

    pub fn take_inline_style_sheets_from_member(
        &mut self,
        member_state: ApplyStyleSheetsMemberState,
    ) -> impl Iterator<Item = (StyleSheetIndex, StyleSheetDefinition)> + '_ {
        self.inline_style_sheet
            .iter_mut()
            .enumerate()
            .skip(member_state.inline_sheet_index as _)
            .take(member_state.inline_sheet_count as _)
            .filter_map(|n| n.1.take().map(|s| (n.0 as _, s)))
    }

    pub fn take_shared_style_sheets_from_member(
        &mut self,
        member_state: ApplyStyleSheetsMemberState,
    ) -> impl Iterator<Item = (StyleSheetIndex, StyleSheetId<BevyRenderer>)> + '_ {
        self.shared_style_sheet_ids
            .iter_mut()
            .enumerate()
            .skip(member_state.shared_sheet_index as _)
            .take(member_state.shared_sheet_count as _)
            .filter_map(|n| n.1.take().map(|s| (n.0 as _, s)))
    }
}
