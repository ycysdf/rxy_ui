use alloc::vec::Vec;
use core::any::Any;

use crate::{BoxedPropValue, MaybeSend, ReBuildFn, Renderer};

// option
pub trait PropState<R>: MaybeSend
where
   R: Renderer,
{
   fn apply(&mut self, new_value: BoxedPropValue, world: &mut R::NodeTree);
   #[cfg(feature = "send_sync")]
   fn as_any_mut(&mut self) -> &mut (dyn Any + MaybeSend);
   #[cfg(not(feature = "send_sync"))]
   fn as_any_mut(&mut self) -> &mut (dyn Any);
}

#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct ReceiverPropState<R, T>
where
   R: Renderer,
   T: Clone + PartialEq + MaybeSend + 'static,
{
   pub re_build_fns: Vec<ReBuildFn<R, T>>,
   pub value: Option<T>,
   // #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
   // eq_fn: Option<fn(&dyn Any, &dyn Any) -> bool>,
   // #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
   // pub clone_fn: Option<fn(&dyn Any) -> BoxedPropValue>,
}

impl<R, T> ReceiverPropState<R, T>
where
   R: Renderer,
   T: Clone + PartialEq + MaybeSend + 'static,
{
   pub fn new() -> Self {
      Self {
         value: None,
         re_build_fns: Default::default(),
      }
   }
}

impl<R, T> Default for ReceiverPropState<R, T>
where
   R: Renderer,
   T: Clone + PartialEq + MaybeSend + 'static,
{
   fn default() -> Self {
      Self::new()
   }
}

impl<R, T> PropState<R> for ReceiverPropState<R, T>
where
   R: Renderer,
   T: Clone + PartialEq + MaybeSend + 'static,
{
   fn apply(&mut self, new_value: BoxedPropValue, world: &mut R::NodeTree) {
      let Ok(new_value) = new_value.downcast::<T>().map(|n| *n) else {
         return;
      };
      if self.value.as_ref().is_some_and(|n| &new_value == n) {
         return;
      }
      for f in self.re_build_fns.iter_mut() {
         f.call(world, new_value.clone());
      }
      self.value = Some(new_value);
   }

   #[cfg(feature = "send_sync")]
   fn as_any_mut(&mut self) -> &mut (dyn Any + MaybeSend) {
      self
   }

   #[cfg(not(feature = "send_sync"))]
   fn as_any_mut(&mut self) -> &mut (dyn Any) {
      self
   }
}
