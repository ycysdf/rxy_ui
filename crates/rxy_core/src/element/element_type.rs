use crate::element::ElementAttrUntyped;
use crate::{MaybeReflect, MaybeSend, MaybeSync, Renderer, RendererNodeId, RendererWorld};

pub trait ElementTypeUnTyped<R>: MaybeReflect + MaybeSend + MaybeSync
where
   R: Renderer,
{
   fn tag_name(&self) -> &'static str;

   fn spawn(
      &self,
      world: &mut RendererWorld<R>,
      parent: Option<&RendererNodeId<R>>,
      reserve_node_id: Option<RendererNodeId<R>>,
   ) -> RendererNodeId<R>;
}

impl<R, T: ElementType<R>> ElementTypeUnTyped<R> for T
where
   R: Renderer,
{
   #[inline]
   fn tag_name(&self) -> &'static str {
      T::TAG_NAME
   }

   #[inline]
   fn spawn(
      &self,
      world: &mut RendererWorld<R>,
      parent: Option<&RendererNodeId<R>>,
      reserve_node_id: Option<RendererNodeId<R>>,
   ) -> RendererNodeId<R> {
      T::spawn(world, parent, reserve_node_id)
   }
}

pub trait ElementType<R>: MaybeReflect + MaybeSend + MaybeSync + 'static
where
   R: Renderer,
{
   const TAG_NAME: &'static str;

   fn get() -> &'static dyn ElementTypeUnTyped<R>;

   fn spawn(
      world: &mut RendererWorld<R>,
      parent: Option<&RendererNodeId<R>>,
      reserve_node_id: Option<RendererNodeId<R>>,
   ) -> RendererNodeId<R>;
}

pub trait ElementTypeAttrs<R>: ElementType<R>
where
   R: Renderer,
{
   const ATTRS: &'static [&'static dyn ElementAttrUntyped<R>];
}
