use crate::element::attr_value::AttrValue;
use crate::smallbox::{SmallBox, S1};
use crate::{
   MaybeFromReflect, MaybeSend, MaybeSync, MaybeTypePath, NodeTree, Renderer, RendererNodeId,
   RendererWorld,
};

#[cfg(feature = "attr_index_u16")]
pub type AttrIndex = u8;
#[cfg(not(feature = "attr_index_u16"))]
pub type AttrIndex = u8;

pub trait HasIndex {
   const INDEX: AttrIndex;
}

pub trait ElementAttrType<R>: HasIndex + MaybeSend + MaybeSync + 'static
where
   R: Renderer,
{
   type Value: AttrValue + Clone + Sized + MaybeFromReflect + MaybeTypePath;

   const NAME: &'static str;

   fn first_set_value(
      world: &mut RendererWorld<R>,
      node_id: RendererNodeId<R>,
      value: impl Into<Self::Value>,
   ) {
      Self::update_value(world, node_id, value)
   }

   fn update_value(
      world: &mut RendererWorld<R>,
      node_id: RendererNodeId<R>,
      value: impl Into<Self::Value>,
   );

   #[inline]
   fn set_value(
      world: &mut RendererWorld<R>,
      node_id: RendererNodeId<R>,
      value: Option<impl Into<Self::Value>>,
   ) {
      let value = value
         .map(|n| n.into())
         .unwrap_or_else(|| Self::Value::default_value());
      if world.prepare_set_attr_and_get_is_init(&node_id, Self::INDEX) {
         Self::update_value(world, node_id, value);
      } else {
         Self::first_set_value(world, node_id, value);
      }
   }

   #[inline]
   fn set_dyn_value(
      world: &mut RendererWorld<R>,
      node_id: RendererNodeId<R>,
      value: Option<SmallBox<dyn AttrValue, S1>>,
   ) {
      Self::set_value(
         world,
         node_id,
         value.map(|n| n.downcast::<Self::Value>().unwrap().into_inner()),
      );
   }

   fn set_default_value(world: &mut RendererWorld<R>, node_id: RendererNodeId<R>) {
      Self::update_value(world, node_id, Self::Value::default_value())
   }
}

pub trait ElementAttrUntyped<R>: MaybeSend + MaybeSync + 'static
where
   R: Renderer,
{
   fn attr_name(&self) -> &'static str;

   fn index(&self) -> u8;

   fn default_value(&self) -> SmallBox<dyn AttrValue, S1>;

   fn set_value(
      &self,
      world: &mut RendererWorld<R>,
      node_id: RendererNodeId<R>,
      value: Option<SmallBox<dyn AttrValue, S1>>,
   );
}

impl<R, T> ElementAttrUntyped<R> for T
where
   R: Renderer,
   T: ElementAttrType<R>,
{
   #[inline]
   fn attr_name(&self) -> &'static str {
      T::NAME
   }

   #[inline]
   fn index(&self) -> u8 {
      T::INDEX
   }

   fn default_value(&self) -> SmallBox<dyn AttrValue, S1> {
      crate::smallbox!(T::Value::default_value())
   }

   #[inline]
   fn set_value(
      &self,
      world: &mut RendererWorld<R>,
      node_id: RendererNodeId<R>,
      value: Option<SmallBox<dyn AttrValue, S1>>,
   ) {
      T::set_dyn_value(world, node_id, value);
   }
}
