use alloc::sync::Arc;
use core::any::{Any, TypeId};
use core::fmt::Debug;
use core::marker::PhantomData;

use bevy_utils::smallvec::SmallVec;

use crate::element::ElementType;
use crate::{
   view_children, ElementView, ElementViewChildren, IntoView, MemberOwner, NodeTree, Renderer,
   RendererNodeId, RendererWorld, SoloView, View, ViewCtx, ViewKey, ViewMember, ViewMemberCtx,
   ViewMemberIndex,
};

#[derive(Clone)]
pub struct ElementTypeTypeInfo<R>
where
   R: Renderer,
{
   pub spawn: fn(
      &mut RendererWorld<R>,
      Option<&RendererNodeId<R>>,
      Option<RendererNodeId<R>>,
   ) -> RendererNodeId<R>,
}

impl<R> ElementTypeTypeInfo<R>
where
   R: Renderer,
{
   pub fn new<E>() -> Self
   where
      E: ElementType<R>,
   {
      Self {
         spawn: |world, parent, reserve_node_id| world.spawn_node::<E>(parent, reserve_node_id),
      }
   }
}

#[cfg(feature = "bevy_reflect")]
impl<R, E> bevy_reflect::FromType<E> for ElementTypeTypeInfo<R>
where
   R: Renderer,
   E: ElementType<R>,
{
   fn from_type() -> Self {
      ElementTypeTypeInfo::new::<E>()
   }
}

#[derive(Clone)]
pub struct DynamicViewMemberTypeInfo<R>
where
   R: Renderer,
{
   count: ViewMemberIndex,
   build: fn(Box<dyn Any>, ViewMemberCtx<R>, will_rebuild: bool),
   unbuild: fn(ViewMemberCtx<R>, will_rebuild: bool),
   rebuild: fn(Box<dyn Any>, ViewMemberCtx<R>),
}

impl<R> DynamicViewMemberTypeInfo<R>
where
   R: Renderer,
{
   pub fn new<T>() -> Self
   where
      T: ViewMember<R>,
   {
      Self {
         count: T::count(),
         build: |vm, ctx, will_rebuild| {
            let vm: T = *vm.downcast::<T>().unwrap();
            vm.build(ctx, will_rebuild)
         },
         unbuild: |ctx, will_rebuild| T::unbuild(ctx, will_rebuild),
         rebuild: |vm, ctx| {
            let vm: T = *vm.downcast::<T>().unwrap();
            vm.rebuild(ctx)
         },
      }
   }
}

pub type DynamicElementViewMember<R> =
   SmallVec<[(Box<dyn Any + Send>, DynamicViewMemberTypeInfo<R>); 4]>;

pub struct DynamicElement<R, E>
where
   R: Renderer,
   E: ElementType<R>,
{
   pub element_type_type_id: TypeId,
   pub members: DynamicElementViewMember<R>,
   pub _marker: PhantomData<E>,
}

impl<R, E> Default for DynamicElement<R, E>
where
   R: Renderer,
   E: ElementType<R>,
{
   fn default() -> Self {
      Self {
         element_type_type_id: TypeId::of::<E>(),
         members: Default::default(),
         _marker: Default::default(),
      }
   }
}

impl<R, E> DynamicElement<R, E>
where
   R: Renderer,
   E: ElementType<R>,
{
   pub fn new() -> Self {
      Self::default()
   }

   #[inline]
   #[cfg(not(feature = "view_children_erasure"))]
   pub fn children<CV>(self, children: CV) -> ElementViewChildren<R, DynamicElement<R, E>, CV::View>
   where
      CV: IntoView<R>,
   {
      view_children(self, children)
   }

   #[inline]
   #[cfg(feature = "view_children_erasure")]
   pub fn children<CV>(
      self,
      children: CV,
   ) -> ElementViewChildren<R, DynamicElement<R, E>, crate::BoxedErasureView<R>>
   where
      CV: IntoView<R>,
   {
      self.erasure_children(children)
   }

   #[inline]
   pub fn erasure_children<CV2>(
      self,
      children: CV2,
   ) -> ElementViewChildren<R, DynamicElement<R, E>, crate::BoxedErasureView<R>>
   where
      CV2: IntoView<R>,
   {
      use crate::IntoViewErasureExt;
      view_children(self, unsafe { children.into_erasure_view() })
   }
}

impl<E, R> SoloView<R> for DynamicElement<R, E>
where
   R: Renderer,
   E: ElementType<R>,
{
   fn node_id(key: &Self::Key) -> &RendererNodeId<R> {
      &key.0
   }
}

impl<R, E> ElementView<R> for DynamicElement<R, E>
where
   R: Renderer,
   E: ElementType<R>,
{
   fn element_node_id(key: &Self::Key) -> &RendererNodeId<R> {
      &key.0
   }

   type E = E;
   type AddMember<T: ViewMember<R>> = Self;
   type SetMembers<T: ViewMember<R> + MemberOwner<R>> = Self;

   fn member_count(&self) -> ViewMemberIndex {
      self.members.len() as _
   }

   fn member<T>(mut self, member: T) -> Self::AddMember<T>
   where
      ((), T): ViewMember<R>,
      T: ViewMember<R>,
   {
      self
         .members
         .push((Box::new(member), DynamicViewMemberTypeInfo::new::<T>()));
      self
   }

   fn members<T>(mut self, members: T) -> Self::SetMembers<(T,)>
   where
      T: ViewMember<R>,
   {
      self.members.clear();
      self
         .members
         .push((Box::new(members), DynamicViewMemberTypeInfo::new::<T>()));
      self
   }
}

#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[derive(Clone, Debug)]
pub struct DynamicElementViewKey<R>(
   pub RendererNodeId<R>,
   #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
   pub  Arc<SmallVec<[(fn(ViewMemberCtx<R>, will_rebuild: bool), ViewMemberIndex); 4]>>,
)
where
   R: Renderer;

impl<R> ViewKey<R> for DynamicElementViewKey<R>
where
   R: Renderer,
{
   #[inline]
   fn remove(self, world: &mut RendererWorld<R>) {
      for (unbuild_fn, index) in self.1.iter() {
         unbuild_fn(
            ViewMemberCtx {
               index: *index,
               world,
               node_id: self.0.clone(),
            },
            true,
         );
      }
      self.0.remove(world);
   }

   #[inline]
   fn insert_before(
      &self,
      world: &mut RendererWorld<R>,
      parent: Option<&RendererNodeId<R>>,
      before_node_id: Option<&RendererNodeId<R>>,
   ) {
      self.0.insert_before(world, parent, before_node_id);
   }

   fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool) {
      self.0.set_visibility(world, hidden);
   }

   fn state_node_id(&self) -> Option<RendererNodeId<R>> {
      Some(self.0.clone())
   }

   fn reserve_key(
      world: &mut RendererWorld<R>,
      will_rebuild: bool,
      parent: RendererNodeId<R>,
      spawn: bool,
   ) -> Self {
      Self(
         <RendererNodeId<R> as ViewKey<R>>::reserve_key(world, will_rebuild, parent, spawn),
         Default::default(),
      )
   }

   fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
      self.0.first_node_id(world)
   }
}

impl<R, E> View<R> for DynamicElement<R, E>
where
   R: Renderer,
   E: ElementType<R>,
{
   type Key = DynamicElementViewKey<R>;

   fn build(
      self,
      ctx: ViewCtx<R>,
      reserve_key: Option<Self::Key>,
      will_rebuild: bool,
   ) -> Self::Key {
      let element_type_type_info = ctx
         .world
         .get_type_state::<ElementTypeTypeInfo<R>>(self.element_type_type_id)
         .unwrap();
      let spawned_node_id = {
         let parent = ctx.parent.clone();
         (element_type_type_info.spawn)(ctx.world, Some(&parent), reserve_key.map(|n| n.0))
      };
      let mut unbuild_fns = SmallVec::with_capacity(self.members.len());
      let mut index = 0;
      for (vm, type_info) in self.members {
         unbuild_fns.push((type_info.unbuild, index));
         (type_info.build)(
            vm,
            ViewMemberCtx {
               index,
               world: &mut *ctx.world,
               node_id: spawned_node_id.clone(),
            },
            will_rebuild,
         );
         index += type_info.count;
      }
      DynamicElementViewKey(spawned_node_id, Arc::new(unbuild_fns))
   }

   fn rebuild(self, ctx: ViewCtx<R>, state_key: Self::Key) {
      let mut index = 0;
      for (vm, type_info) in self.members {
         (type_info.rebuild)(
            vm,
            ViewMemberCtx {
               index,
               world: ctx.world,
               node_id: state_key.0.clone(),
            },
         );
         index += type_info.count;
      }
   }
}

impl<R, E> IntoView<R> for DynamicElement<R, E>
where
   R: Renderer,
   E: ElementType<R>,
{
   type View = DynamicElement<R, E>;

   fn into_view(self) -> Self::View {
      self
   }
}
