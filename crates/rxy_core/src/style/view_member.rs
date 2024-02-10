#![allow(clippy::blocks_in_conditions)]

use crate::style::{ApplyStyleSheets, StyleError, StyleSheets, StyledNodeTree};
use crate::{Renderer, ViewMember, ViewMemberCtx, ViewMemberIndex, ViewMemberOrigin};

pub type StyleItemIndex = u8;
pub type StyleSheetIndex = u8;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum StyleSheetLocation {
    Shared,
    Inline,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ApplyStyleSheetsMemberState {
    pub inline_sheet_index: StyleSheetIndex,
    pub inline_sheet_count: StyleSheetIndex,
    pub shared_sheet_index: StyleSheetIndex,
    pub shared_sheet_count: StyleSheetIndex,
}

impl ApplyStyleSheetsMemberState {
    pub fn get_and_increment_and_by_location(
        &mut self,
        location: StyleSheetLocation,
    ) -> StyleSheetIndex {
        match location {
            StyleSheetLocation::Inline => {
                let r = self.inline_sheet_index;
                self.inline_sheet_index += 1;
                r
            }
            StyleSheetLocation::Shared => {
                let r = self.shared_sheet_index;
                self.shared_sheet_index += 1;
                r
            }
        }
    }
}

impl<R, T> ViewMemberOrigin<R> for ApplyStyleSheets<T>
where
    R: Renderer,
    T: StyleSheets<R>,
{
    type Origin = Self;
}

impl<R, T> ViewMember<R> for ApplyStyleSheets<T>
where
    R: Renderer,
    R::NodeTree: StyledNodeTree<R>,
    T: StyleSheets<R>,
{
    fn count() -> ViewMemberIndex {
        1
    }

    fn unbuild(mut ctx: ViewMemberCtx<R>, view_removed: bool) {
        if view_removed {
            return;
        }
        let member_state = ctx
            .indexed_view_member_state_mut::<ApplyStyleSheetsMemberState>()
            .cloned()
            .unwrap();

        ctx.world
            .unbuild_style_sheet(ctx.node_id.clone(), member_state)
            .unwrap();
    }

    fn build(self, mut ctx: ViewMemberCtx<R>, _will_rebuild: bool) {
        let member_state = ctx
            .indexed_view_member_state_mut::<ApplyStyleSheetsMemberState>()
            .cloned();
        let is_first = member_state.is_none();
        let new_member_state = ctx
            .world
            .build_style_sheets(ctx.node_id.clone(), self.0, member_state)
            .unwrap();
        if is_first {
            ctx.set_indexed_view_member_state(new_member_state);
        }
    }

    fn rebuild(self, mut ctx: ViewMemberCtx<R>) {
        let member_state = ctx
            .indexed_view_member_state_mut::<ApplyStyleSheetsMemberState>()
            .cloned()
            .unwrap();

        ctx.world
            .rebuild_style_sheet(ctx.node_id, self.0, member_state)
            .unwrap();
    }
}
