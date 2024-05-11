use crate::{element::ElementAttrType, SmallBox, S1};
use core::marker::PhantomData;

pub trait ElementAttrMember<R, EA>:
   ViewMember<R> + ViewMemberOrigin<R, Origin = ElementAttr<R, EA>>
where
   R: Renderer,
   EA: ElementAttrType<R>,
{
}

impl<T, R, EA> ElementAttrMember<R, EA> for T
where
   T: ViewMember<R> + ViewMemberOrigin<R, Origin = ElementAttr<R, EA>>,
   R: Renderer,
   EA: ElementAttrType<R>,
{
}

#[derive(Clone, Debug)]
pub struct ElementAttr<R, EA, M = ()>(pub EA::Value, PhantomData<M>)
where
   R: Renderer,
   EA: ElementAttrType<R>;

use crate::{
   AttrValue, NodeTree, Renderer, ViewMember, ViewMemberCtx, ViewMemberIndex, ViewMemberOrigin,
};

use super::attr_value;

impl<R, EA> PartialEq for ElementAttr<R, EA>
where
   R: Renderer,
   EA: ElementAttrType<R>,
{
   fn eq(&self, other: &Self) -> bool {
      self.0.eq(&other.0)
   }
}

impl<R, EA> ElementAttr<R, EA>
where
   R: Renderer,
   EA: ElementAttrType<R>,
{
   #[inline]
   pub fn new(value: EA::Value) -> Self {
      Self(value, Default::default())
   }
}

impl<R, EA> ViewMemberOrigin<R> for ElementAttr<R, EA>
where
   R: Renderer,
   EA: ElementAttrType<R>,
{
   type Origin = Self;
}

pub struct ElementAttrValue(pub SmallBox<dyn AttrValue, S1>);

impl<R, EA> ViewMember<R> for ElementAttr<R, EA>
where
   R: Renderer,
   EA: ElementAttrType<R>,
{
   fn count() -> ViewMemberIndex {
      1
   }

   fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
      if view_removed {
         return;
      }
      ctx.world.unset_attr::<EA>(ctx.node_id);
   }

   fn build(self, mut ctx: ViewMemberCtx<R>, will_rebuild: bool) {
      if will_rebuild {
         ctx.set_indexed_view_member_state(ElementAttrValue(self.0.clone_att_value()))
      }
      ctx.world.set_attr::<EA>(ctx.node_id, self.0);
   }

   fn rebuild(self, mut ctx: ViewMemberCtx<R>) {
      if let Some(attr_value) = ctx.indexed_view_member_state_mut::<ElementAttrValue>() {
         let attr_value = attr_value.0.as_any().downcast_ref::<EA::Value>().unwrap();
         if attr_value.eq(&self.0) {
            return;
         } else {
            ctx.set_indexed_view_member_state(ElementAttrValue(self.0.clone_att_value()));
         }
      }
      ctx.world.set_attr::<EA>(ctx.node_id, self.0);
   }
}

#[derive(Clone, Debug)]
pub struct StaticElementAttr<R, EA>(pub EA::Value)
where
   R: Renderer,
   EA: ElementAttrType<R>;

impl<R, EA> PartialEq for StaticElementAttr<R, EA>
where
   R: Renderer,
   EA: ElementAttrType<R>,
{
   fn eq(&self, other: &Self) -> bool {
      self.0.eq(&other.0)
   }
}

impl<R, EA> StaticElementAttr<R, EA>
where
   R: Renderer,
   EA: ElementAttrType<R>,
{
   #[inline]
   pub fn new(value: EA::Value) -> Self {
      Self(value)
   }
}

impl<R, EA> ViewMemberOrigin<R> for StaticElementAttr<R, EA>
where
   R: Renderer,
   EA: ElementAttrType<R>,
{
   type Origin = Self;
}

impl<R, EA> ViewMember<R> for StaticElementAttr<R, EA>
where
   R: Renderer,
   EA: ElementAttrType<R>,
{
   fn count() -> ViewMemberIndex {
      1
   }

   fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
      if view_removed {
         return;
      }
      ctx.world.unset_attr::<EA>(ctx.node_id);
   }

   #[inline]
   fn build(self, mut ctx: ViewMemberCtx<R>, will_rebuild: bool) {
      ElementAttr::<R, EA>::new(self.0).build(ctx, false);
   }

   #[inline]
   fn rebuild(self, mut ctx: ViewMemberCtx<R>) {}
}
