// use crate::shared_context::HydrateSharedContext;
use crate::{
   // shared_context::{SharedContext, SsrSharedContext},
   signal_traits::{DefinedAt, SignalUpdateUntracked, SignalWithUntracked, Trigger},
   source::{
      AnySource, AnySubscriber, ReactiveNode, Source, Subscriber, ToAnySource, ToAnySubscriber,
   },
   unwrap_signal,
};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use slotmap::{new_key_type, SlotMap};
use std::{
   any::{Any, TypeId},
   cell::RefCell,
   fmt::Debug,
   marker::PhantomData,
   mem,
   panic::Location,
   sync::{Arc, Weak},
};

new_key_type! { struct NodeId; }

lazy_static! {
   static ref MAP: RwLock<SlotMap<NodeId, Box<dyn Any + Send + Sync>>> = Default::default();
}
thread_local! {
    static OWNER: RefCell<Option<Owner>> = Default::default();
}

#[derive(Debug, Clone)]
pub struct Root<T>(pub Owner, pub T);

impl<T> Root<T> {
   pub fn global(fun: impl FnOnce() -> T) -> T {
      let Root(owner, value) = Root::new(fun);
      mem::forget(owner);
      value
   }

   // #[cfg(feature = "web")]
   // pub fn global_hydrate(fun: impl FnOnce() -> T) -> T {
   //     let Root(owner, value) = Root::new_with_shared_context(
   //         fun,
   //         Some(Arc::new(HydrateSharedContext::new())),
   //     );
   //     mem::forget(owner);
   //     value
   // }

   // pub fn global_ssr(fun: impl FnOnce() -> T) -> Root<T> {
   //     Root::new_with_shared_context(
   //         fun,
   //         Some(Arc::new(SsrSharedContext::new())),
   //     )
   // }

   pub fn new(fun: impl FnOnce() -> T) -> Self {
      Self::new_with_shared_context(fun /*, None*/)
   }

   pub fn new_with_shared_context(
      fun: impl FnOnce() -> T,
      // shared_context: Option<Arc<dyn SharedContext + Send + Sync>>,
   ) -> Self {
      let owner = Owner {
         // shared_context,
         ..Default::default()
      };
      let prev = OWNER.with(|o| std::mem::replace(&mut *o.borrow_mut(), Some(owner.clone())));

      let value = fun();

      OWNER.with(|o| *o.borrow_mut() = prev);

      Self(owner, value)
   }

   pub fn into_value(self) -> T {
      self.1
   }
}

#[derive(Debug, Clone, Default)]
pub struct Owner {
   pub(crate) inner: Arc<RwLock<OwnerInner>>,
   // pub(crate) shared_context: Option<Arc<dyn SharedContext + Send + Sync>>,
}

impl Owner {
   pub fn new() -> Self {
      // let (parent, shared_context) = {
      let parent = {
         OWNER.with(|o| {
            o.borrow().as_ref().map(|o| {
               // (, o.shared_context.clone())
               Arc::downgrade(&o.inner)
            })
         })
         // .unzip()
      };
      Self {
         inner: Arc::new(RwLock::new(OwnerInner {
            parent,
            nodes: Default::default(),
            contexts: Default::default(),
            cleanups: Default::default(),
         })),
         // shared_context: shared_context.flatten(),
      }
   }

   pub fn with<T>(&self, fun: impl FnOnce() -> T) -> T {
      let prev = { OWNER.with(|o| mem::replace(&mut *o.borrow_mut(), Some(self.clone()))) };
      let val = fun();
      OWNER.with(|o| {
         *o.borrow_mut() = prev;
      });
      val
   }

   pub fn with_cleanup<T>(&self, fun: impl FnOnce() -> T) -> T {
      let (cleanups, nodes) = {
         let mut lock = self.inner.write();
         (mem::take(&mut lock.cleanups), mem::take(&mut lock.nodes))
      };
      for cleanup in cleanups {
         cleanup();
      }
      for node in nodes {
         _ = MAP.write().remove(node);
      }

      self.with(fun)
   }

   // #[inline(always)]
   // pub fn shared_context() -> Option<Arc<dyn SharedContext + Send + Sync>> {
   //     #[cfg(feature = "hydration")]
   //     {
   //         OWNER.with(|o| {
   //             o.borrow().as_ref().and_then(|o| o.shared_context.clone())
   //         })
   //     }
   //     #[cfg(not(feature = "hydration"))]
   //     {
   //         None
   //     }
   // }

   pub fn on_cleanup(fun: impl FnOnce() + Send + Sync + 'static) {
      if let Some(owner) = Owner::current() {
         owner.inner.write().cleanups.push(Box::new(fun));
      }
   }

   fn register(&self, node: NodeId) {
      self.inner.write().nodes.push(node);
   }

   pub fn current() -> Option<Owner> {
      OWNER.with(|o| o.borrow().clone())
   }
}

#[derive(Default)]
pub(crate) struct OwnerInner {
   pub parent: Option<Weak<RwLock<OwnerInner>>>,
   nodes: Vec<NodeId>,
   pub contexts: FxHashMap<TypeId, Box<dyn Any + Send + Sync>>,
   pub cleanups: Vec<Box<dyn FnOnce() + Send + Sync>>,
}

impl Debug for OwnerInner {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("OwnerInner")
         .field("parent", &self.parent)
         .field("nodes", &self.nodes)
         .field("contexts", &self.contexts)
         .field("cleanups", &self.cleanups.len())
         .finish()
   }
}

impl Drop for OwnerInner {
   fn drop(&mut self) {
      for cleanup in mem::take(&mut self.cleanups) {
         cleanup();
      }
      for node in mem::take(&mut self.nodes) {
         _ = MAP.write().remove(node);
      }
   }
}

#[derive(Debug)]
pub struct Stored<T> {
   node: NodeId,
   ty: PhantomData<T>,
}

impl<T> Copy for Stored<T> {}

impl<T> Clone for Stored<T> {
   fn clone(&self) -> Self {
      *self
   }
}

impl<T> Stored<T>
where
   T: Send + Sync + 'static,
{
   #[track_caller]
   pub fn new(value: T) -> Self {
      let node = {
         MAP.write()
            .insert(Box::new(value) as Box<dyn Any + Send + Sync>)
      };
      OWNER.with(|o| {
         if let Some(owner) = &*o.borrow() {
            owner.register(node);
         }
      });

      Self {
         node,
         ty: PhantomData,
      }
   }

   pub fn with_value<U>(&self, fun: impl FnOnce(&T) -> U) -> Option<U> {
      let m = MAP.read();
      let m = m.get(self.node);

      m.and_then(|n| n.downcast_ref::<T>()).map(fun)
   }

   pub fn get(&self) -> Option<T>
   where
      T: Clone,
   {
      self.with_value(T::clone)
   }

   pub fn exists(&self) -> bool
   where
      T: Clone,
   {
      MAP.read().contains_key(self.node)
   }

   pub fn dispose(&self) {
      MAP.write().remove(self.node);
   }
}

#[doc(hidden)]
pub trait StoredData {
   type Data;

   fn get_value(&self) -> Option<Self::Data>;

   fn dispose(&self);
}

impl<T> ReactiveNode for T
where
   T: StoredData,
   T::Data: ReactiveNode,
{
   fn mark_dirty(&self) {
      if let Some(inner) = self.get_value() {
         inner.mark_dirty();
      }
   }

   fn mark_check(&self) {
      if let Some(inner) = self.get_value() {
         inner.mark_check();
      }
   }

   fn mark_subscribers_check(&self) {
      if let Some(inner) = self.get_value() {
         inner.mark_subscribers_check();
      }
   }

   fn update_if_necessary(&self) -> bool {
      if let Some(inner) = self.get_value() {
         inner.update_if_necessary()
      } else {
         false
      }
   }
}

impl<T> Source for T
where
   T: StoredData,
   T::Data: Source,
{
   fn add_subscriber(&self, subscriber: AnySubscriber) {
      if let Some(inner) = self.get_value() {
         inner.add_subscriber(subscriber);
      }
   }

   fn remove_subscriber(&self, subscriber: &AnySubscriber) {
      if let Some(inner) = self.get_value() {
         inner.remove_subscriber(subscriber);
      }
   }

   fn clear_subscribers(&self) {
      if let Some(inner) = self.get_value() {
         inner.clear_subscribers();
      }
   }
}

impl<T> Subscriber for T
where
   T: StoredData,
   T::Data: Subscriber,
{
   fn add_source(&self, source: AnySource) {
      if let Some(inner) = self.get_value() {
         inner.add_source(source);
      }
   }

   fn clear_sources(&self, subscriber: &AnySubscriber) {
      if let Some(inner) = self.get_value() {
         inner.clear_sources(subscriber);
      }
   }
}

impl<T> DefinedAt for T
where
   T: StoredData,
   T::Data: DefinedAt,
{
   fn defined_at(&self) -> Option<&'static Location<'static>> {
      self.get_value().and_then(|n| n.defined_at())
   }
}

impl<T> ToAnySource for T
where
   T: StoredData,
   T::Data: ToAnySource + DefinedAt,
{
   #[track_caller]
   fn to_any_source(&self) -> AnySource {
      self
         .get_value()
         .map(|inner| inner.to_any_source())
         .unwrap_or_else(unwrap_signal!(self))
   }
}

impl<T> ToAnySubscriber for T
where
   T: StoredData,
   T::Data: ToAnySubscriber + DefinedAt,
{
   #[track_caller]
   fn to_any_subscriber(&self) -> AnySubscriber {
      self
         .get_value()
         .map(|inner| inner.to_any_subscriber())
         .unwrap_or_else(unwrap_signal!(self))
   }
}

impl<T> SignalWithUntracked for T
where
   T: StoredData + DefinedAt,
   T::Data: SignalWithUntracked,
{
   type Value = <<T as StoredData>::Data as SignalWithUntracked>::Value;

   #[track_caller]
   fn try_with_untracked<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> Option<U> {
      self.get_value().and_then(|n| n.try_with_untracked(fun))
   }
}

impl<T> Trigger for T
where
   T: StoredData,
   T::Data: Trigger,
{
   fn trigger(&self) {
      if let Some(inner) = self.get_value() {
         inner.trigger();
      }
   }
}

impl<T> SignalUpdateUntracked for T
where
   T: StoredData,
   T::Data: SignalUpdateUntracked,
{
   type Value = <<T as StoredData>::Data as SignalUpdateUntracked>::Value;

   fn try_update_untracked<U>(&self, fun: impl FnOnce(&mut Self::Value) -> U) -> Option<U> {
      self
         .get_value()
         .and_then(|inner| inner.try_update_untracked(fun))
   }
}
