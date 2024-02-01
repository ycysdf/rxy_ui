use bevy_hierarchy::BuildWorldChildren;

pub use attrs::*;
use bevy_ui::{widget::Button, FocusPolicy, Interaction};
use rxy_bevy_element::{all_attrs, elements, ElementType};
use rxy_bevy_macro::BevyIntoView;
use rxy_core::{
    view_children, Element, ElementView, ElementViewChildren, IntoView, MemberOwner,
    RendererElementType, RendererNodeId, RendererWorld, SoloView, View, ViewCtx, ViewMember,
};

use crate::{x_bundle, BevyRenderer, BevyWrapper, Focusable, ViewAttr, XBundle};

mod attrs;
pub mod event;

// todo: merge span
pub fn span(
    str: impl Into<String>,
) -> BevyElement<elements::text, (ViewAttr<all_attrs::content>,)> {
    BevyElement::default().members(ViewAttr::<all_attrs::content>(str.into()))
}

pub fn div() -> BevyElement<elements::view, ()> {
    BevyElement::default()
}

pub fn button(
) -> BevyElement<elements::view, (XBundle<(FocusPolicy, Interaction, Button, Focusable)>,)> {
    BevyElement::default().members(x_bundle((
        FocusPolicy::default(),
        Interaction::default(),
        Button,
        Focusable::default(),
    )))
}

// pub fn input() -> BevyElement<elements::input, ()> {
//     BevyElement::default()
// }

impl<T> RendererElementType<BevyRenderer> for BevyWrapper<T>
where
    T: ElementType,
{
    const NAME: &'static str = T::TAG_NAME;

    fn spawn(
        world: &mut RendererWorld<BevyRenderer>,
        parent: Option<RendererNodeId<BevyRenderer>>,
        reserve_node_id: Option<RendererNodeId<BevyRenderer>>,
    ) -> RendererNodeId<BevyRenderer> {
        let mut entity_world_mut = match reserve_node_id {
            None => world.spawn_empty(),
            Some(reserve_node_id) => world.get_or_spawn(reserve_node_id).unwrap(),
        };
        if let Some(parent) = parent {
            entity_world_mut.set_parent(parent);
        }
        entity_world_mut.insert(bevy_core::Name::new(T::TAG_NAME));
        T::update_entity(&mut entity_world_mut);
        entity_world_mut.id()
    }
}

#[derive(Clone, BevyIntoView)]
pub struct BevyElement<E, VM>(pub Element<BevyRenderer, BevyWrapper<E>, VM>)
where
    E: ElementType,
    VM: ViewMember<BevyRenderer>;

impl<E, VM> BevyElement<E, VM>
where
    E: ElementType,
    VM: ViewMember<BevyRenderer>,
{
    #[cfg(not(feature = "view_erasure"))]
    pub fn children<CV>(self, children: CV) -> BevyElementChildren<E, VM, CV::View>
    where
        CV: IntoView<BevyRenderer>,
    {
        view_children(self, children)
    }

    #[cfg(feature = "view_erasure")]
    pub fn children<CV>(
        self,
        children: CV,
    ) -> BevyElementChildren<E, VM, rxy_core::BoxedErasureView<BevyRenderer>>
    where
        CV: IntoView<BevyRenderer>,
    {
        use rxy_core::IntoViewErasureExt;
        view_children(self, unsafe { children.into_erasure_view() })
    }
}

pub type BevyElementChildren<E, VM, CV> = ElementViewChildren<BevyElement<E, VM>, CV, BevyRenderer>;

impl<E> Default for BevyElement<E, ()>
where
    E: ElementType,
{
    fn default() -> Self {
        BevyElement::<E, ()>(Element::<BevyRenderer, BevyWrapper<E>, ()>::default())
    }
}
/*
impl<E, VM: ViewMember<BevyRenderer>>
    Into<Element<BevyRenderer, BevyWrapper<E>, Box<dyn DynamicViewMember<BevyRenderer>>>>
    for BevyElement<E, VM>
where
    E: ElementType,
{
    fn into(
        self,
    ) -> Element<BevyRenderer, BevyWrapper<E>, Box<dyn DynamicViewMember<BevyRenderer>>> {
        Element {
            members: self.0.members.into_dynamic(),
            _marker: self.0._marker,
        }
    }
}
*/
impl<E, VM> View<BevyRenderer> for BevyElement<E, VM>
where
    E: ElementType,
    VM: ViewMember<BevyRenderer>,
{
    type Key = RendererNodeId<BevyRenderer>;

    #[inline]
    fn build(
        self,
        ctx: ViewCtx<BevyRenderer>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        self.0.build(ctx, reserve_key, will_rebuild)
    }

    #[inline]
    fn rebuild(self, ctx: ViewCtx<BevyRenderer>, key: Self::Key) {
        self.0.rebuild(ctx, key)
    }
}

impl<E, VM> SoloView<BevyRenderer> for BevyElement<E, VM>
where
    E: ElementType,
    VM: ViewMember<BevyRenderer>,
{
    fn node_id(key: &Self::Key) -> &RendererNodeId<BevyRenderer> {
        key
    }
}

impl<E, VM> ElementView<BevyRenderer> for BevyElement<E, VM>
where
    E: ElementType,
    VM: ViewMember<BevyRenderer>,
{
    fn element_node_id(key: &Self::Key) -> &RendererNodeId<BevyRenderer> {
        key
    }
}

impl<E, VM> MemberOwner<BevyRenderer> for BevyElement<E, VM>
where
    E: ElementType,
    VM: ViewMember<BevyRenderer>,
{
    type E = BevyWrapper<E>;
    type VM = VM;
    type AddMember<T: ViewMember<BevyRenderer>> = BevyElement<E, (VM, T)>;
    type SetMembers<T: ViewMember<BevyRenderer> + MemberOwner<BevyRenderer>> = BevyElement<E, T>;

    fn member<T>(self, member: T) -> Self::AddMember<T>
    where
        (VM, T): ViewMember<BevyRenderer>,
        T: ViewMember<BevyRenderer>,
    {
        BevyElement(self.0.member(member))
    }

    fn members<T>(self, members: T) -> Self::SetMembers<(T,)>
    where
        T: ViewMember<BevyRenderer>,
    {
        BevyElement(self.0.members(members))
    }
}
