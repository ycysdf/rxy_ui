use std::borrow::Cow;
use std::ops::Deref;

use async_channel::Receiver;
use bevy_ecs::prelude::Mut;
use bevy_ecs::system::Resource;

use hooked_collection::{HookVec, HookedVec, VecOperation};
use rxy_core::{DataOrPlaceholderNodeId, MaybeSend, RendererWorld, VecDataSource};

use crate::BevyRenderer;

pub trait GetHookedVec {
   type HookVec: HookVec;
   fn get_hooked_vec(&self) -> &HookedVec<<Self::HookVec as HookVec>::Item, Self::HookVec>;
}

impl<T, O> GetHookedVec for T
where
   T: Deref<Target = HookedVec<O::Item, O>>,
   O: HookVec,
{
   type HookVec = O;

   fn get_hooked_vec(&self) -> &HookedVec<O::Item, O> {
      self.deref()
   }
}

pub fn use_hooked_vec_resource_source<T>(
   receiver: Receiver<VecOperation<<T::HookVec as HookVec>::Item>>,
) -> ResourceHookedVecSource<T>
where
   T: Resource + GetHookedVec,
   <T::HookVec as HookVec>::Item: MaybeSend + Clone + 'static,
   T::HookVec: MaybeSend + 'static,
{
   ResourceHookedVecSource {
      receiver,
      _marker: std::marker::PhantomData,
   }
}

pub struct ResourceHookedVecSource<T>
where
   T: Resource + GetHookedVec,
{
   receiver: Receiver<VecOperation<<T::HookVec as HookVec>::Item>>,
   _marker: std::marker::PhantomData<T>,
}

impl<T> VecDataSource<BevyRenderer> for ResourceHookedVecSource<T>
where
   T: Resource + GetHookedVec,
   <T::HookVec as HookVec>::Item: MaybeSend + Clone + 'static,
   T::HookVec: MaybeSend + 'static,
{
   type Item = <T::HookVec as HookVec>::Item;
   type InitState = ();
   type State = ();
   type Op = VecOperation<Self::Item>;

   fn map_and_init_state<U>(
      self,
      world: &mut RendererWorld<BevyRenderer>,
      mut map_f: impl FnMut(&Self::Item, &mut RendererWorld<BevyRenderer>, usize) -> U,
   ) -> (Vec<U>, Option<(Self::InitState, Receiver<Self::Op>)>) {
      let vec = world.resource_scope(|world, hooked_vec_getter: Mut<T>| {
         hooked_vec_getter
            .get_hooked_vec()
            .iter()
            .enumerate()
            .map(|(i, item)| map_f(item, world, i))
            .collect::<Vec<_>>()
      });
      (vec, Some(((), self.receiver)))
   }

   fn ready_state(_state: &mut Self::InitState) -> Self::State {}

   fn apply_ops(
      _state: Self::State,
      ops: Vec<Self::Op>,
      world: &mut RendererWorld<BevyRenderer>,
      _state_node_id: DataOrPlaceholderNodeId<BevyRenderer>,
      mut f: impl FnMut(VecOperation<Cow<Self::Item>>, &[Self::Item], &mut RendererWorld<BevyRenderer>),
   ) {
      world.resource_scope(|world, hooked_vec_getter: Mut<T>| {
         let hooked_vec = hooked_vec_getter.get_hooked_vec();
         for op in ops {
            f(
               op.as_ref().map(|n| Cow::Borrowed(n)),
               hooked_vec.as_slice(),
               world,
            );
         }
      });
   }
}
