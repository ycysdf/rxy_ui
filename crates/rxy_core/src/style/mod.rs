mod attr_style_owner;
mod style_sheet_definition;
mod style_sheet_items;
mod view_member;

use crate::utils::all_tuples;
use crate::{
    AttrIndex, AttrValue, Either, EitherExt, MemberOwner, Renderer, RendererNodeId, RendererWorld,
    SmallBox, ViewMember, ViewMemberOrigin, XNest, XValueWrapper, S1,
};
pub use attr_style_owner::*;
use bevy_utils::HashMap;
use derive_more::{Deref, DerefMut, From, IntoIterator};
use futures_lite::StreamExt;
use std::any::TypeId;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::iter::{once, Chain};
use std::ops::{AddAssign, Deref};
pub use style_sheet_definition::*;
pub use style_sheet_items::*;
pub use view_member::*;

pub type Result<R, T = ()> = core::result::Result<T, StyleError<R>>;
pub type StyleAttrValue = SmallBox<dyn AttrValue, S1>;

pub mod prelude {
    pub use super::{x, x_active, x_focus, x_hover};
}

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
    pub attr_id: AttrIndex,
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

#[derive(Clone, Debug)]
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

// #[derive(Error, Clone, Debug)]
#[derive(Clone, Debug)]
pub enum StyleError<R>
where
    R: Renderer,
{
    // #[error("no found inter attr infos: {item_id:?}")]
    NoFoundInterAttrInfos { item_id: NodeInterStyleItemId },
    // #[error("no found style item: {attr_id:?}")]
    NoFoundAttrId { attr_id: AttrIndex },
    // #[error("no found style item: {item_id:?}")]
    NoFoundStyleItemId { item_id: NodeStyleItemId },
    // #[error("style sheets is none")]
    StyleSheetIsNone,

    // #[error("no found style sheet: {node_id:?}")]
    NoFoundStyleState { node_id: RendererNodeId<R> },

    // #[error("no found interaction style state: {node_id:?}")]
    NoFoundInterStyleState { node_id: RendererNodeId<R> },

    // #[error("no found style sheet: {0:?}")]
    NoFoundStyleSheetOnNode(NodeStyleSheetId),

    // #[error("no found style sheet: {node_id:?}")]
    NoFoundSharedStyleSheet { node_id: RendererNodeId<R> },

    // #[error("removed style sheet: {0:?}")]
    RemovedStyleSheet(NodeStyleSheetId),

    // #[error("style sheet type incorrect")]
    StyleSheetTypeIncorrect,

    // #[error("no found style sheet: {node_id:?}")]
    NoFoundStyleSheetsState { node_id: RendererNodeId<R> },

    // #[error("shared style sheet not exists")]
    SharedEntityNotExists,

    // #[error("no found style sheet: {node_id:?}")]
    NoFoundElementEntityExtraData { node_id: RendererNodeId<R> },

    // #[error("no found node: {node_id:?}")]
    NoFoundNode { node_id: RendererNodeId<R> },
}

#[derive(Default, Deref, DerefMut, Debug, IntoIterator, From)]
pub struct NodeStyleAttrInfos(pub HashMap<AttrIndex, NodeStyleAttrInfo>);

#[derive(Default, Deref, DerefMut, Debug, IntoIterator, From)]
pub struct NodeInterStyleAttrInfos(pub HashMap<StyleInteraction, NodeStyleAttrInfos>);

impl NodeInterStyleAttrInfos {
    pub fn remove_attr_info(
        &mut self,
        attr_id: &AttrIndex,
    ) -> Option<(StyleInteraction, NodeStyleAttrInfo)> {
        self.iter_mut()
            .find_map(|(interaction, n)| n.remove(attr_id).map(|n| (*interaction, n)))
    }
    pub fn get_attr_info(
        &self,
        interaction: StyleInteraction,
        attr_id: AttrIndex,
    ) -> Option<&NodeStyleAttrInfo> {
        self.get(&interaction).and_then(|n| n.get(&attr_id))
    }
    pub fn match_attr(
        &self,
        attr_id: AttrIndex,
        interaction: StyleInteraction,
        strict: bool,
    ) -> Option<&NodeStyleAttrInfo> {
        match interaction {
            StyleInteraction::Active => self
                .get(&StyleInteraction::Active)
                .and_then(|n| n.get(&attr_id))
                .condition(strict, |n| {
                    n.or_else(|| {
                        self.get(&StyleInteraction::Hover)
                            .and_then(|n| n.get(&attr_id))
                    })
                }),
            interaction => self.get(&interaction).and_then(|n| n.get(&attr_id)),
        }
    }

    /// There are repeated AttrId
    pub fn iter_match_attr(
        &self,
        interaction: Option<StyleInteraction>,
        strict: bool,
    ) -> impl Iterator<Item = (AttrIndex, &NodeStyleAttrInfo, StyleInteraction)> + '_ {
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
    ) -> impl Iterator<Item = (AttrIndex, StyleInteraction)> + '_ {
        self.iter_match_attr(interaction, strict)
            .map(|(attr_id, _, interaction)| (attr_id, interaction))
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

pub struct ApplyStyleSheets<T>(pub T);

// pub trait IntoStyleViewMember<R> {
//     type Member: ViewMemberOrigin<R, Origin = ApplyStyleSheets<Self::StyleSheets>>;
//     type StyleSheets: StyleSheets<R>;
//     fn into_style_view_member(self) -> Self::Member;
// }
//
// impl<R, VM, T, SS> IntoStyleViewMember<R> for T
// where
//     R: Renderer,
//     T: XNest<R, Member = VM>,
//     VM: ViewMember<R> + ViewMemberOrigin<R, Origin = ApplyStyleSheets<SS>>,
//     SS: StyleSheets<R>,
// {
//     type Member = VM;
//     type StyleSheets = SS;
//
//     fn into_style_view_member(self) -> Self::Member {
//         self.into_member()
//     }
// }

// impl<R, T> IntoStyleViewMember<R> for ApplyStyleSheets<T>
// where
//     R: Renderer,
//     T: StyleSheets<R>,
// {
//     type Member = ApplyStyleSheets<T>;
//     type StyleSheets = T;
//
//     fn into_style_view_member(self) -> Self::Member {
//         self
//     }
// }

#[derive(Debug, Clone)]
pub struct StyleItemValue {
    pub attr_id: AttrIndex,
    pub value: StyleAttrValue,
}

pub struct StyleSheetsInfo {
    pub inline_style_sheet_count: u8,
    pub shared_style_sheet_count: u8,
}

impl AddAssign for StyleSheetsInfo {
    fn add_assign(&mut self, rhs: Self) {
        self.inline_style_sheet_count += rhs.inline_style_sheet_count;
        self.shared_style_sheet_count += rhs.shared_style_sheet_count;
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

#[derive(Debug, Clone)]
pub enum AppliedStyleSheet<R>
where
    R: Renderer,
{
    None,
    Inline(StyleSheetDefinition),
    Shared(StyleSheetId<R>),
}

impl<R> AppliedStyleSheet<R>
where
    R: Renderer,
{
    pub fn style_sheet_location(&self) -> Option<StyleSheetLocation> {
        match self {
            AppliedStyleSheet::None => None,
            AppliedStyleSheet::Inline(_) => Some(StyleSheetLocation::Inline),
            AppliedStyleSheet::Shared(_) => Some(StyleSheetLocation::Shared),
        }
    } /*

      pub fn get_style_sheet_definition<'a>(
          &'a self,
          mut query: impl StateOwnerWithNodeId<'a,'_>,
      ) -> Result<Option<&'a StyleSheetDefinition>> {
          Ok(match self {
              AppliedStyleSheet::None => None,
              AppliedStyleSheet::Inline(style_sheet) => Some(style_sheet),
              AppliedStyleSheet::Shared(style_sheet_id) => {
                  Some(query.get_current_style_sheet_definition(style_sheet_id.clone())?)
              }
          })
      }*/

    pub fn scoped_style_sheet_definition<'a, U>(
        &'a self,
        entity_world_mut: &'a mut RendererWorld<R>,
        node_id: RendererNodeId<R>,
        f: impl FnOnce(&'a mut RendererWorld<R>, RendererNodeId<R>, Option<&StyleSheetDefinition>) -> U,
    ) -> Result<R, U> {
        todo!()
        // let entity = entity_world_mut.id();
        // match self {
        //     AppliedStyleSheet::None => Ok(f(entity_world_mut, None)),
        //     AppliedStyleSheet::Inline(style_sheet_definition) => {
        //         Ok(f(entity_world_mut, Some(style_sheet_definition)))
        //     }
        //     AppliedStyleSheet::Shared(style_sheet_id) => entity_world_mut.world_scope(|world| {
        //         world.scoped_style_sheet_definition(
        //             style_sheet_id.clone(),
        //             |entity_world_mut, style_sheet_definition| {
        //                 entity_world_mut.world_scope(|world| {
        //                     let mut entity_world_mut = world.entity_mut(entity);
        //                     f(&mut entity_world_mut, Some(&*style_sheet_definition))
        //                 })
        //             },
        //         )
        //     }),
        // }
    }
}

pub trait StyleSheets<R>: Send + 'static
where
    R: Renderer,
{
    fn style_sheets(
        self,
        ctx: StyleSheetCtx<R>,
    ) -> (
        impl Iterator<Item = AppliedStyleSheet<R>> + Send + 'static,
        StyleSheetsInfo,
    );
}

impl<T> Into<XValueWrapper<Self>> for StyleSheetOwner<T>
{
    fn into(self) -> XValueWrapper<Self> {
        XValueWrapper(self)
    }
}

impl<R, T> StyleSheets<R> for StyleSheetOwner<T>
where
    R: Renderer,
    T: StyleSheetItems<R>,
{
    fn style_sheets(
        self,
        ctx: StyleSheetCtx<R>,
    ) -> (
        impl Iterator<Item = AppliedStyleSheet<R>> + Send + 'static,
        StyleSheetsInfo,
    ) {
        (
            once(AppliedStyleSheet::Inline(StyleSheetDefinition {
                interaction: self.0,
                items: T::iter(self.1, ctx).collect(),
            })),
            StyleSheetsInfo {
                inline_style_sheet_count: 1,
                shared_style_sheet_count: 0,
            },
        )
    }
}

// impl<T> StyleSheets<BevyRenderer> for BevyWrapper<T>
// where
//     T: TypedStyleLabel,
// {
//     fn style_sheets(
//         self,
//         ctx: StyleSheetCtx<BevyRenderer>,
//     ) -> (
//         impl Iterator<Item = AppliedStyleSheet> + Send + 'static,
//         StyleSheetsInfo,
//     ) {
//         todo!()
//     }
// }

//
// impl<R> StyleSheets<R> for TypeId
// where
//     R: Renderer,
// {
//     fn style_sheets(
//         self,
//         ctx: StyleSheetCtx<R>,
//     ) -> (
//         impl Iterator<Item = AppliedStyleSheet<R>> + Send + 'static,
//         StyleSheetsInfo,
//     ) {
//         typed_shared_style_sheets(self, ctx)
//     }
// }

// impl<R, LSS, RSS> StyleSheets<R> for Either<LSS, RSS>
// where
//     R: Renderer,
//     LSS: StyleSheets<R>,
//     RSS: StyleSheets<R>,
// {
//     fn style_sheets(
//         self,
//         ctx: StyleSheetCtx<R>,
//     ) -> (
//         impl Iterator<Item = AppliedStyleSheet> + Send + 'static,
//         StyleSheetsInfo,
//     ) {
//         match self {
//             Either::Left(l) => {
//                 let x = l.style_sheets(ctx);
//                 (x.0.either_left(), x.1)
//             }
//             Either::Right(r) => {
//                 let x = r.style_sheets(ctx);
//                 (x.0.either_right(), x.1)
//             }
//         }
//     }
// }

macro_rules! impl_style_sheets_for_tuple {
    ($($t:ident),*) => {

        #[allow(non_snake_case)]
        impl<R, $($t),*> StyleSheets<R> for ($($t,)*)
        where
            R: Renderer,
            $($t: StyleSheets<R>),*
        {
            #[inline]
            fn style_sheets(
                self,
                ctx: StyleSheetCtx<R>,
            ) -> (impl Iterator<Item = AppliedStyleSheet<R>> + Send + 'static,StyleSheetsInfo) {
                let ($($t,)*) = self;
                let r = core::iter::empty();
                let mut _r_info  = StyleSheetsInfo{
                    inline_style_sheet_count: ctx.inline_style_sheet_index,
                    shared_style_sheet_count: ctx.shared_style_sheet_index,
                };
                $(
                    let (style_sheets,info) = $t.style_sheets(StyleSheetCtx {
                        inline_style_sheet_index: _r_info.inline_style_sheet_count,
                        shared_style_sheet_index: _r_info.shared_style_sheet_count,
                        // world: &mut *ctx.world,
                        world: unsafe {&mut *(ctx.world as *mut _)},
                        node_id: ctx.node_id.clone(),
                    });
                    _r_info += info;
                    let r = r.chain(style_sheets);
                )*
                (r,_r_info)
            }
        }
    };
}
all_tuples!(impl_style_sheets_for_tuple, 0, 6, T);
