use crate::{
   arena::{Owner, Stored, StoredData},
   signal_traits::*,
   source::{
      AnySource, AnySubscriber, ReactiveNode, ReactiveNodeState, Source, SourceSet, Subscriber,
      SubscriberSet, ToAnySource, ToAnySubscriber,
   },
   Observer,
};
use parking_lot::RwLock;
use std::{
   fmt::Debug,
   panic::Location,
   sync::{Arc, Weak},
};

pub fn use_memo<T>(fun: impl Fn(Option<&T>) -> T + Send + Sync + 'static) -> Memo<T>
where
   T: PartialEq + Send + Sync + 'static,
{
   Memo::new(fun)
}

pub struct Memo<T: Send + Sync + 'static> {
   inner: Stored<ArcMemo<T>>,
}

impl<T: Send + Sync + 'static> Memo<T> {
   #[track_caller]
   #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", skip_all,))]
   pub fn new(fun: impl Fn(Option<&T>) -> T + Send + Sync + 'static) -> Self
   where
      T: PartialEq,
   {
      Self {
         inner: Stored::new(ArcMemo::new(fun)),
      }
   }

   pub fn debug_log_inner(&self, name: &str) {
      self.inner.get().unwrap().debug_log_inner(name);
   }
}

impl<T: Send + Sync + 'static> Copy for Memo<T> {}

impl<T: Send + Sync + 'static> Clone for Memo<T> {
   fn clone(&self) -> Self {
      *self
   }
}

impl<T: Send + Sync + 'static> Debug for Memo<T> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Memo")
         .field("type", &std::any::type_name::<T>())
         .field("store", &self.inner)
         .finish()
   }
}

impl<T: Send + Sync + 'static> StoredData for Memo<T> {
   type Data = ArcMemo<T>;

   fn get_value(&self) -> Option<Self::Data> {
      self.inner.get()
   }

   fn dispose(&self) {
      self.inner.dispose();
   }
}

pub struct ArcMemo<T> {
   #[cfg(debug_assertions)]
   defined_at: &'static Location<'static>,
   inner: Arc<RwLock<MemoInner<T>>>,
}

impl<T: Send + Sync + 'static> ArcMemo<T> {
   #[track_caller]
   #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all,))]
   pub fn new(fun: impl Fn(Option<&T>) -> T + Send + Sync + 'static) -> Self
   where
      T: PartialEq,
   {
      let inner = Arc::new_cyclic(|weak| {
         let subscriber = AnySubscriber(
            weak.as_ptr() as usize,
            Weak::clone(weak) as Weak<dyn Subscriber + Send + Sync>,
         );

         RwLock::new(MemoInner::new(
            Arc::new(fun),
            |lhs, rhs| lhs.as_ref() == rhs.as_ref(),
            subscriber,
         ))
      });
      Self {
         #[cfg(debug_assertions)]
         defined_at: Location::caller(),
         inner,
      }
   }

   pub fn debug_log_inner(&self, name: &str) {
      println!("{name}: {:?}", Arc::as_ptr(&self.inner));
   }
}

impl<T> Clone for ArcMemo<T> {
   fn clone(&self) -> Self {
      Self {
         #[cfg(debug_assertions)]
         defined_at: self.defined_at,
         inner: Arc::clone(&self.inner),
      }
   }
}

impl<T> Debug for ArcMemo<T> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("ArcMemo")
         .field("type", &std::any::type_name::<T>())
         .field("data", &self.inner.data_ptr())
         .finish()
   }
}

struct MemoInner<T> {
   value: Option<T>,
   #[allow(clippy::type_complexity)]
   fun: Arc<dyn Fn(Option<&T>) -> T + Send + Sync>,
   compare_with: fn(Option<&T>, Option<&T>) -> bool,
   owner: Owner,
   state: ReactiveNodeState,
   sources: SourceSet,
   subscribers: SubscriberSet,
   any_subscriber: AnySubscriber,
}

impl<T: Send + Sync + 'static> ReactiveNode for RwLock<MemoInner<T>> {
   fn mark_dirty(&self) {
      self.write().state = ReactiveNodeState::Dirty;
      self.mark_subscribers_check();
   }

   fn mark_check(&self) {
      {
         let mut lock = self.write();
         lock.state = ReactiveNodeState::Check;
      }
      for sub in (&self.read().subscribers).into_iter() {
         sub.mark_check();
      }
   }

   fn mark_subscribers_check(&self) {
      let lock = self.read();
      for sub in (&lock.subscribers).into_iter() {
         sub.mark_check();
      }
   }

   fn update_if_necessary(&self) -> bool {
      let (state, sources) = {
         let inner = self.read();
         (inner.state, inner.sources.clone())
      };

      let needs_update = match state {
         ReactiveNodeState::Clean => false,
         ReactiveNodeState::Dirty => true,
         ReactiveNodeState::Check => (&sources).into_iter().any(|source| {
            source.update_if_necessary() || self.read().state == ReactiveNodeState::Dirty
         }),
      };

      if needs_update {
         let (fun, value, compare_with, owner) = {
            let mut lock = self.write();
            (
               lock.fun.clone(),
               lock.value.take(),
               lock.compare_with,
               lock.owner.clone(),
            )
         };

         let any_subscriber = { self.read().any_subscriber.clone() };
         any_subscriber.clear_sources(&any_subscriber);
         let new_value =
            owner.with_cleanup(|| any_subscriber.with_observer(|| fun(value.as_ref())));

         let changed = !compare_with(Some(&new_value), value.as_ref());
         let mut lock = self.write();
         lock.value = Some(new_value);
         lock.state = ReactiveNodeState::Clean;

         if changed {
            let subs = lock.subscribers.clone();
            drop(lock);
            for sub in subs {
               // don't trigger reruns of effects/memos
               // basically: if one of the observers has triggered this memo to
               // run, it doesn't need to be re-triggered because of this change
               if !Observer::is(&sub) {
                  sub.mark_dirty();
               }
            }
         }

         changed
      } else {
         let mut lock = self.write();
         lock.state = ReactiveNodeState::Clean;
         false
      }
   }
}

impl<T: Send + Sync + 'static> ReactiveNode for ArcMemo<T> {
   fn mark_dirty(&self) {
      self.inner.mark_dirty();
   }

   fn mark_check(&self) {
      self.inner.mark_check();
   }

   fn mark_subscribers_check(&self) {
      self.inner.mark_subscribers_check();
   }

   fn update_if_necessary(&self) -> bool {
      self.inner.update_if_necessary()
   }
}

impl<T: Send + Sync + 'static> ToAnySource for ArcMemo<T> {
   fn to_any_source(&self) -> AnySource {
      AnySource(
         self.inner.data_ptr() as usize,
         Arc::downgrade(&self.inner) as Weak<dyn Source + Send + Sync>,
      )
   }
}

impl<T: Send + Sync + 'static> Source for RwLock<MemoInner<T>> {
   fn add_subscriber(&self, subscriber: AnySubscriber) {
      self.write().subscribers.subscribe(subscriber);
   }

   fn remove_subscriber(&self, subscriber: &AnySubscriber) {
      self.write().subscribers.unsubscribe(subscriber);
   }

   fn clear_subscribers(&self) {
      self.write().subscribers.take();
   }
}

impl<T: Send + Sync + 'static> Subscriber for RwLock<MemoInner<T>> {
   fn add_source(&self, source: AnySource) {
      self.write().sources.insert(source);
   }

   fn clear_sources(&self, subscriber: &AnySubscriber) {
      self.write().sources.clear_sources(subscriber);
   }
}

impl<T: Send + Sync + 'static> Source for ArcMemo<T> {
   fn add_subscriber(&self, subscriber: AnySubscriber) {
      self.inner.add_subscriber(subscriber);
   }

   fn remove_subscriber(&self, subscriber: &AnySubscriber) {
      self.inner.remove_subscriber(subscriber);
   }

   fn clear_subscribers(&self) {
      self.inner.clear_subscribers();
   }
}

impl<T: Send + Sync + 'static> ToAnySubscriber for ArcMemo<T> {
   fn to_any_subscriber(&self) -> AnySubscriber {
      AnySubscriber(
         self.inner.data_ptr() as usize,
         Arc::downgrade(&self.inner) as Weak<dyn Subscriber + Send + Sync>,
      )
   }
}

impl<T: Send + Sync + 'static> Subscriber for ArcMemo<T> {
   fn add_source(&self, source: AnySource) {
      self.inner.write().sources.insert(source);
   }

   fn clear_sources(&self, subscriber: &AnySubscriber) {
      self.inner.write().sources.clear_sources(subscriber);
   }
}

impl<T: Send + Sync + 'static> MemoInner<T> {
   #[allow(clippy::type_complexity)]
   pub fn new(
      fun: Arc<dyn Fn(Option<&T>) -> T + Send + Sync>,
      compare_with: fn(Option<&T>, Option<&T>) -> bool,
      any_subscriber: AnySubscriber,
   ) -> Self {
      Self {
         value: None,
         fun,
         compare_with,
         owner: Owner::new(),
         state: ReactiveNodeState::Dirty,
         sources: Default::default(),
         subscribers: SubscriberSet::new(),
         any_subscriber,
      }
   }
}

impl<T> DefinedAt for ArcMemo<T> {
   #[inline(always)]
   fn defined_at(&self) -> Option<&'static Location<'static>> {
      #[cfg(debug_assertions)]
      {
         Some(self.defined_at)
      }
      #[cfg(not(debug_assertions))]
      {
         None
      }
   }
}

impl<T: Send + Sync + 'static> SignalWithUntracked for ArcMemo<T> {
   type Value = T;

   #[track_caller]
   #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all,))]
   fn try_with_untracked<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> Option<U> {
      self.update_if_necessary();

      // safe to unwrap here because update_if_necessary
      // guarantees the value is Some
      let lock = self.inner.read();
      let value = lock.value.as_ref()?;
      Some(fun(value))
   }
}
