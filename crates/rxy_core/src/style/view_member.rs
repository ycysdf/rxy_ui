#![allow(clippy::blocks_in_conditions)]

use crate::style::{ApplyStyleSheets, StyleSheets};
use crate::{IntoViewMember, Renderer, ViewMember, ViewMemberCtx, ViewMemberIndex, ViewMemberOrigin};

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

impl<R, T> IntoViewMember<R> for ApplyStyleSheets<T>
where
    R: Renderer,
    T: StyleSheets<R>,
{
    type Member = Self;

    fn into_member(self) -> Self {
        self
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
    T: StyleSheets<R>,
{
    fn count() -> ViewMemberIndex {
        1
    }

    fn unbuild(mut ctx: ViewMemberCtx<R>, view_removed: bool) {
        todo!()
    }

    fn build(self, mut ctx: ViewMemberCtx<R>, _will_rebuild: bool) {
        todo!()
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        todo!()
    }
}
