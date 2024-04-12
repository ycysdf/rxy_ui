use core::marker::PhantomData;

use bevy_app::PreUpdate;
use bevy_ecs::prelude::*;

use crate::add_system;

#[derive(Resource)]
pub struct ResChangeObserve<T: Resource> {
   sender: async_broadcast::Sender<()>,
   receiver: async_broadcast::Receiver<()>,
   _marker: PhantomData<T>,
}

impl<T> Default for ResChangeObserve<T>
where
   T: Resource,
{
   fn default() -> Self {
      let (sender, receiver) = async_broadcast::broadcast(1024);
      Self {
         sender,
         receiver,
         _marker: Default::default(),
      }
   }
}

impl<T: Resource> ResChangeObserve<T> {
   pub fn new() -> Self {
      Self::default()
   }
}

pub trait ResChangeWorldExt {
   fn get_res_change_receiver<T>(&mut self) -> async_broadcast::Receiver<()>
   where
      T: Resource;
}

impl ResChangeWorldExt for World {
   fn get_res_change_receiver<T>(&mut self) -> async_broadcast::Receiver<()>
   where
      T: Resource,
   {
      let is_add_system = self.contains_resource::<ResChangeObserve<T>>();
      let res_change_observe = self.get_resource_or_insert_with(ResChangeObserve::<T>::new);

      let receiver = res_change_observe.receiver.clone();
      fn res_observe<T: Resource>(res_change: Res<ResChangeObserve<T>>) {
         let _ = res_change.sender.try_broadcast(());
      }

      if !is_add_system {
         add_system(
            self,
            PreUpdate,
            res_observe::<T>.run_if(|res_change: Res<ResChangeObserve<T>>, res: Res<T>| {
               res_change.receiver.receiver_count() > 0 && res.is_changed()
            }),
         );
      }
      receiver
   }
}
