use core::{
    marker::PhantomData,
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};

use bevy_app::PreUpdate;
use bevy_ecs::prelude::*;
use std::sync::Arc;

use crate::add_system;

pub struct ResChangeReceiver {
    inner: async_channel::Receiver<()>,
    observer_count: Arc<AtomicUsize>,
}

impl Deref for ResChangeReceiver {
    type Target = async_channel::Receiver<()>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Drop for ResChangeReceiver {
    fn drop(&mut self) {
        self.observer_count.fetch_sub(1, Ordering::Relaxed);
    }
}

#[derive(Resource)]
pub struct ResChangeObserve<T: Resource> {
    sender: async_channel::Sender<()>,
    receiver: async_channel::Receiver<()>,
    observer_count: Arc<AtomicUsize>,
    _marker: PhantomData<T>,
}

impl<T: Resource> ResChangeObserve<T> {
    pub fn new() -> Self {
        let (sender, receiver) = async_channel::unbounded();
        Self {
            sender,
            receiver,
            observer_count: Default::default(),
            _marker: Default::default(),
        }
    }
}

pub trait ResChangeWorldExt {
    fn get_res_change_receiver<T>(&mut self) -> ResChangeReceiver
    where
        T: Resource;
}

impl ResChangeWorldExt for World {
    fn get_res_change_receiver<T>(&mut self) -> ResChangeReceiver
    where
        T: Resource,
    {
        let is_add_system = self.contains_resource::<ResChangeObserve<T>>();
        let res_change_observe = self.get_resource_or_insert_with(ResChangeObserve::<T>::new);

        res_change_observe
            .observer_count
            .fetch_add(1, Ordering::Relaxed);
        fn res_observe<T: Resource>(res_change: Res<ResChangeObserve<T>>) {
            for _ in 0..res_change.sender.receiver_count() {
                if res_change.sender.try_send(()).is_err() {
                    break;
                }
            }
        }
        let r = ResChangeReceiver {
            inner: res_change_observe.receiver.clone(),
            observer_count: res_change_observe.observer_count.clone(),
        };

        if !is_add_system {
            add_system(
                self,
                PreUpdate,
                res_observe::<T>.run_if(|res_change: Res<ResChangeObserve<T>>, res: Res<T>| {
                    res_change.observer_count.load(Ordering::Relaxed) > 0 && res.is_changed()
                }),
            );
        }
        r
    }
}
