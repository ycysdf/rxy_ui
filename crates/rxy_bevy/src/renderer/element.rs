// use crate::{BevyRenderer, BevyWrapper};
// use bevy_hierarchy::BuildWorldChildren;
// use rxy_bevy_element::ElementType;
// use rxy_bevy_macro::BevyIntoView;
// use rxy_core::{
//     view_children, Element, ElementView, ElementViewChildren, ElementViewKey, IntoView,
//     IntoViewMember, MemberOwner, RendererElementType, RendererNodeId, RendererWorld, SoloView,
//     View, ViewCtx, ViewMember,
// };
//
// // todo: merge span
//
// impl<T> RendererElementType<BevyRenderer> for BevyWrapper<T>
// where
//     T: ElementType,
// {
//     const NAME: &'static str = T::TAG_NAME;
//
//     fn spawn(
//         world: &mut RendererWorld<BevyRenderer>,
//         parent: Option<RendererNodeId<BevyRenderer>>,
//         reserve_node_id: Option<RendererNodeId<BevyRenderer>>,
//     ) -> RendererNodeId<BevyRenderer> {
//         let mut entity_world_mut = match reserve_node_id {
//             None => world.spawn_empty(),
//             Some(reserve_node_id) => world.get_or_spawn(reserve_node_id).unwrap(),
//         };
//         if let Some(parent) = parent {
//             entity_world_mut.set_parent(parent);
//         }
//         entity_world_mut.insert(bevy_core::Name::new(T::TAG_NAME));
//         T::update_entity(&mut entity_world_mut);
//         entity_world_mut.id()
//     }
// }
//
// #[derive(Clone, BevyIntoView)]
// pub struct BevyElement<E, VM>(pub Element<BevyRenderer, BevyWrapper<E>, VM>)
// where
//     E: ElementType,
//     VM: ViewMember<BevyRenderer>;
//
//
// impl<E> Default for BevyElement<E, ()>
// where
//     E: ElementType,
// {
//     fn default() -> Self {
//         BevyElement::<E, ()>(Element::<BevyRenderer, BevyWrapper<E>, ()>::default())
//     }
// }
//
// impl<E, VM> View<BevyRenderer> for BevyElement<E, VM>
// where
//     E: ElementType,
//     VM: ViewMember<BevyRenderer>,
// {
//     type Key = ElementViewKey<BevyRenderer, VM>;
//
//     #[inline]
//     fn build(
//         self,
//         ctx: ViewCtx<BevyRenderer>,
//         reserve_key: Option<Self::Key>,
//         will_rebuild: bool,
//     ) -> Self::Key {
//         self.0.build(ctx, reserve_key, will_rebuild)
//     }
//
//     #[inline]
//     fn rebuild(self, ctx: ViewCtx<BevyRenderer>, key: Self::Key) {
//         self.0.rebuild(ctx, key)
//     }
// }
//
// impl<E, VM> SoloView<BevyRenderer> for BevyElement<E, VM>
// where
//     E: ElementType,
//     VM: ViewMember<BevyRenderer>,
// {
//     fn node_id(key: &Self::Key) -> &RendererNodeId<BevyRenderer> {
//         &key.0
//     }
// }
//
// impl<E, VM> ElementView<BevyRenderer> for BevyElement<E, VM>
// where
//     E: ElementType,
//     VM: ViewMember<BevyRenderer>,
// {
//     fn element_node_id(key: &Self::Key) -> &RendererNodeId<BevyRenderer> {
//         &key.0
//     }
// }
//
// impl<E, VM> MemberOwner<BevyRenderer> for BevyElement<E, VM>
// where
//     E: ElementType,
//     VM: ViewMember<BevyRenderer>,
// {
//     type E = BevyWrapper<E>;
//     type VM = VM;
//     type AddMember<T: ViewMember<BevyRenderer>> = BevyElement<E, (VM, T)>;
//     type SetMembers<T: ViewMember<BevyRenderer> + MemberOwner<BevyRenderer>> = BevyElement<E, T>;
//
//     fn member<T>(self, member: impl IntoViewMember<BevyRenderer, VM>) -> Self::AddMember<T>
//     where
//         (VM, T): ViewMember<BevyRenderer>,
//         T: ViewMember<BevyRenderer>,
//     {
//         BevyElement(self.0.member(member))
//     }
//
//     fn members<T>(self, members: impl IntoViewMember<BevyRenderer, VM>) -> Self::SetMembers<(T,)>
//     where
//         T: ViewMember<BevyRenderer>,
//     {
//         BevyElement(self.0.members(members))
//     }
// }
