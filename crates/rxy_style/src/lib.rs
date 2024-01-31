use ahash::HashMap;
use bitflags::bitflags;
use core::any::TypeId;
use core::iter::Chain;
use core::ops::Deref;
use derive_more::IntoIterator;
#[allow(unused_imports)]
#[allow(dead_code)]
use derive_more::{Deref, DerefMut, From};
use rxy_core::{
    Either, EitherExt, MemberOwner, Renderer, RendererNodeId, RendererWorld, ViewMember,
};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::Hash;
use thiserror::Error;

pub mod prelude {
    pub use super::{x, x_active, x_focus, x_hover};
}

pub type StyleAttrId = u8;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum StyleSheetLocation {
    Shared,
    Inline,
}

pub type StyleItemIndex = u8;
pub type StyleSheetIndex = u8;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct NodeStyleItemId {
    pub item_index: StyleItemIndex,
    pub sheet_id: NodeStyleSheetId,
}

impl PartialOrd for NodeStyleItemId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeStyleItemId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sheet_id
            .location
            .cmp(&other.sheet_id.location)
            .then_with(|| self.sheet_id.index.cmp(&other.sheet_id.index))
            .then_with(|| self.item_index.cmp(&other.item_index))
    }
}

impl From<NodeStyleItemId> for NodeStyleSheetId {
    fn from(val: NodeStyleItemId) -> Self {
        val.sheet_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct NodeInterStyleItemId {
    pub style_interaction: StyleInteraction,
    pub style_item_id: NodeAttrStyleItemId,
}

impl From<NodeInterStyleItemId> for NodeStyleItemId {
    fn from(val: NodeInterStyleItemId) -> Self {
        val.style_item_id.item_id
    }
}

impl From<NodeInterStyleItemId> for NodeAttrStyleItemId {
    fn from(val: NodeInterStyleItemId) -> Self {
        val.style_item_id
    }
}

impl Deref for NodeInterStyleItemId {
    type Target = NodeAttrStyleItemId;

    fn deref(&self) -> &Self::Target {
        &self.style_item_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct NodeAttrStyleItemId {
    pub attr_id: StyleAttrId,
    pub item_id: NodeStyleItemId,
}

impl From<NodeAttrStyleItemId> for NodeStyleSheetId {
    fn from(val: NodeAttrStyleItemId) -> Self {
        val.item_id.into()
    }
}

impl Deref for NodeAttrStyleItemId {
    type Target = NodeStyleItemId;

    fn deref(&self) -> &Self::Target {
        &self.item_id
    }
}

pub type SharedStyleSheetId = TypeId;

pub struct StyleSheetOwner<T>(pub Option<StyleInteraction>, pub T);

pub fn x() -> StyleSheetOwner<()> {
    StyleSheetOwner(None, ())
}

pub fn x_hover() -> StyleSheetOwner<()> {
    StyleSheetOwner(Some(StyleInteraction::Hover), ())
}

pub fn x_active() -> StyleSheetOwner<()> {
    StyleSheetOwner(Some(StyleInteraction::Active), ())
}

pub fn x_focus() -> StyleSheetOwner<()> {
    StyleSheetOwner(Some(StyleInteraction::Focus), ())
}

impl<R, T> MemberOwner<R> for StyleSheetOwner<T>
where
    R: Renderer,
    T: MemberOwner<R>,
{
    type E = T::E;
    type VM = T::VM;
    type AddMember<VM: ViewMember<R>> = StyleSheetOwner<T::AddMember<VM>>;
    type SetMembers<VM: ViewMember<R> + MemberOwner<R>> = StyleSheetOwner<T::SetMembers<VM>>;

    fn member<VM>(self, member: VM) -> Self::AddMember<VM>
    where
        (Self::VM, VM): ViewMember<R>,
        VM: ViewMember<R>,
    {
        StyleSheetOwner(self.0, self.1.member(member))
    }

    fn members<VM: ViewMember<R>>(self, members: VM) -> Self::SetMembers<(VM,)>
    where
        VM: ViewMember<R>,
    {
        StyleSheetOwner(self.0, self.1.members(members))
    }
}

#[derive(Clone, Debug, Hash)]
pub struct StyleSheetId<R>
where
    R: Renderer,
{
    pub node_style_sheet_id: NodeStyleSheetId,
    pub node_id: RendererNodeId<R>,
}

impl<R> AsRef<NodeStyleSheetId> for StyleSheetId<R>
where
    R: Renderer,
{
    fn as_ref(&self) -> &NodeStyleSheetId {
        &self.node_style_sheet_id
    }
}

impl<R> From<StyleSheetId<R>> for NodeStyleSheetId
where
    R: Renderer,
{
    fn from(val: StyleSheetId<R>) -> Self {
        val.node_style_sheet_id
    }
}

impl<R> Deref for StyleSheetId<R>
where
    R: Renderer,
{
    type Target = NodeStyleSheetId;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

pub struct StyleSheetCtx<'a, R>
where
    R: Renderer,
{
    pub inline_style_sheet_index: StyleSheetIndex,
    pub shared_style_sheet_index: StyleSheetIndex,
    pub world: &'a mut RendererWorld<R>,
    pub node_id: RendererNodeId<R>,
}

impl<'a, R> StyleSheetCtx<'a, R>
where
    R: Renderer,
{
    pub fn add_style_sheet(&mut self) {}
}

#[derive(Deref, DerefMut, From, Clone, Debug)]
pub struct NodeStyleAttrInfo(pub Either<NodeStyleItemId, BinaryHeap<NodeStyleItemId>>);

impl NodeStyleAttrInfo {
    #[inline(always)]
    pub fn top_item_id(&self) -> NodeStyleItemId {
        *self.as_ref().map_right(|n| n.peek().unwrap()).into_inner()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct NodeStyleSheetId {
    pub index: StyleSheetIndex,
    pub location: StyleSheetLocation,
}

pub type Result<R, T = ()> = core::result::Result<T, StyleError<R>>;

#[derive(Error, Clone, Debug)]
pub enum StyleError<R>
where
    R: Renderer,
{
    #[error("no found inter attr infos: {item_id:?}")]
    NoFoundInterAttrInfos { item_id: NodeInterStyleItemId },
    #[error("no found style item: {attr_id:?}")]
    NoFoundAttrId { attr_id: StyleAttrId },
    #[error("no found style item: {item_id:?}")]
    NoFoundStyleItemId { item_id: NodeStyleItemId },
    #[error("style sheets is none")]
    StyleSheetIsNone,

    #[error("no found style sheet: {node_id:?}")]
    NoFoundStyleState { node_id: RendererNodeId<R> },

    #[error("no found interaction style state: {node_id:?}")]
    NoFoundInterStyleState { node_id: RendererNodeId<R> },

    #[error("no found style sheet: {0:?}")]
    NoFoundStyleSheetOnNode(NodeStyleSheetId),

    #[error("no found style sheet: {node_id:?}")]
    NoFoundSharedStyleSheet { node_id: RendererNodeId<R> },

    #[error("removed style sheet: {0:?}")]
    RemovedStyleSheet(NodeStyleSheetId),

    #[error("style sheet type incorrect")]
    StyleSheetTypeIncorrect,

    #[error("no found style sheet: {node_id:?}")]
    NoFoundStyleSheetsState { node_id: RendererNodeId<R> },

    #[error("shared style sheet not exists")]
    SharedEntityNotExists,

    #[error("no found style sheet: {node_id:?}")]
    NoFoundElementEntityExtraData { node_id: RendererNodeId<R> },

    #[error("no found node: {node_id:?}")]
    NoFoundNode { node_id: RendererNodeId<R> },
}

#[derive(Default, Deref, DerefMut, Debug, IntoIterator, From)]
pub struct NodeStyleAttrInfos(pub HashMap<StyleAttrId, NodeStyleAttrInfo>);

#[derive(Default, Deref, DerefMut, Debug, IntoIterator, From)]
pub struct NodeInterStyleAttrInfos(pub HashMap<StyleInteraction, NodeStyleAttrInfos>);

impl NodeInterStyleAttrInfos {
    pub fn remove_attr_info(
        &mut self,
        attr_id: &StyleAttrId,
    ) -> Option<(StyleInteraction, NodeStyleAttrInfo)> {
        self.iter_mut().find_map(|(interaction, n)| n.remove(attr_id).map(|n| (*interaction, n)))
    }
    pub fn get_attr_info(
        &self,
        interaction: StyleInteraction,
        attr_id: StyleAttrId,
    ) -> Option<&NodeStyleAttrInfo> {
        self.get(&interaction).and_then(|n| n.get(&attr_id))
    }
    pub fn match_attr(
        &self,
        attr_id: StyleAttrId,
        interaction: StyleInteraction,
        strict: bool,
    ) -> Option<&NodeStyleAttrInfo> {
        match interaction {
            StyleInteraction::Active => self
                .get(&StyleInteraction::Active)
                .and_then(|n| n.get(&attr_id))
                .condition(strict, |n| {
                    n.or_else(|| self.get(&StyleInteraction::Hover).and_then(|n| n.get(&attr_id)))
                }),
            interaction => self.get(&interaction).and_then(|n| n.get(&attr_id)),
        }
    }

    /// There are repeated AttrId
    pub fn iter_match_attr(
        &self,
        interaction: Option<StyleInteraction>,
        strict: bool,
    ) -> impl Iterator<Item = (StyleAttrId, &NodeStyleAttrInfo, StyleInteraction)> + '_ {
        let Some(interaction) = interaction else {
            return core::iter::empty().either_left();
        };
        if strict {
            self.get(&interaction)
                .into_iter()
                .map(|n| &n.0)
                .flatten()
                .map(move |(attr_id, attr_info)| (*attr_id, attr_info, interaction))
                .either_left()
        } else {
            StyleInteraction::priority_iter()
                .filter(move |n| interaction.is_match(*n, false))
                .flat_map(|interaction| {
                    self.get(&interaction)
                        .into_iter()
                        .map(|n| &n.0)
                        .flatten()
                        .map(move |(attr_id, attr_info)| (*attr_id, attr_info, interaction))
                })
                .either_right()
        }
        .either_right()
    }

    /// There are repeated AttrId
    #[inline(always)]
    pub fn iter_match_attr_ids(
        &self,
        interaction: Option<StyleInteraction>,
        strict: bool,
    ) -> impl Iterator<Item = (StyleAttrId, StyleInteraction)> + '_ {
        self.iter_match_attr(interaction, strict)
            .map(|(attr_id, _, interaction)| (attr_id, interaction))
    }
}

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
            Self::priority_iter().filter(move |n| self.contains(*n)).either_right()
        }
    }
}

// todo: extract to lib
pub trait PipeOp: Sized {
    #[inline(always)]
    fn pipe<S, U>(self, state: S, f: fn(Self, S) -> U) -> U {
        f(self, state)
    }
    #[inline(always)]
    fn condition(self, condition: bool, f: impl FnOnce(Self) -> Self) -> Self {
        if condition {
            f(self)
        } else {
            self
        }
    }
    #[inline(always)]
    fn condition_map<U>(self, condition: bool, f: impl FnOnce(Self) -> U) -> Either<Self, U> {
        if condition {
            f(self).either_right()
        } else {
            self.either_left()
        }
    }
    #[inline(always)]
    fn option_map<T, U>(self, option: Option<T>, f: impl FnOnce(Self, T) -> U) -> Either<Self, U> {
        match option {
            Some(n) => f(self, n).either_right(),
            None => self.either_left(),
        }
    }
    #[inline(always)]
    fn option_map_else<T, U, U2>(
        self,
        option: Option<T>,
        f: impl FnOnce(Self, T) -> U,
        else_f: impl FnOnce(Self) -> U2,
    ) -> Either<U, U2> {
        match option {
            Some(n) => f(self, n).either_left(),
            None => else_f(self).either_right(),
        }
    }
}

impl<T> PipeOp for T where T: Sized {}

pub trait IterExt: Iterator + Sized {
    #[inline(always)]
    fn chain_option<I>(self, option: Option<I>) -> Either<Self, Chain<Self, I>>
    where
        I: Iterator<Item = Self::Item>,
    {
        self.option_map(option, |n, i| n.chain(i))
    }
}

impl<T> IterExt for T where T: Iterator + Sized {}
