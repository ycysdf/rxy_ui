use crate::{
   prelude::SignalWith, render_effect::RenderEffect, signal::ArcRwSignal,
   signal_traits::SignalUpdate,
};
use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use std::{hash::Hash, sync::Arc};

/// A conditional signal that only notifies subscribers when a change
/// in the source signal’s value changes whether the given function is true.
#[derive(Clone)]
pub struct Selector<T>
where
   T: PartialEq + Eq + Clone + Hash + 'static,
{
   subs: Arc<RwLock<FxHashMap<T, ArcRwSignal<bool>>>>,
   v: Arc<RwLock<Option<T>>>,
   #[allow(clippy::type_complexity)]
   f: Arc<dyn Fn(&T, &T) -> bool>,
}

impl<T> Selector<T>
where
   T: PartialEq + Eq + Clone + Hash + 'static,
{
   pub fn new(source: impl Fn() -> T + Clone + 'static) -> Self {
      Self::new_with_fn(source, PartialEq::eq)
   }

   pub fn new_with_fn(
      source: impl Fn() -> T + Clone + 'static,
      f: impl Fn(&T, &T) -> bool + Clone + 'static,
   ) -> Self {
      let subs: Arc<RwLock<FxHashMap<T, ArcRwSignal<bool>>>> = Default::default();
      let v: Arc<RwLock<Option<T>>> = Default::default();
      let f = Arc::new(f) as Arc<dyn Fn(&T, &T) -> bool>;

      RenderEffect::new({
         let subs = Arc::clone(&subs);
         let f = Arc::clone(&f);
         let v = Arc::clone(&v);
         move |prev: Option<T>| {
            let next_value = source();
            *v.write() = Some(next_value.clone());
            if prev.as_ref() != Some(&next_value) {
               for (key, signal) in &*subs.read() {
                  if f(key, &next_value) || (prev.is_some() && f(key, prev.as_ref().unwrap())) {
                     signal.update(|n| *n = true);
                  }
               }
            }
            next_value
         }
      });

      Selector { subs, v, f }
   }

   /// Reactively checks whether the given key is selected.
   pub fn selected(&self, key: T) -> bool {
      let read = {
         let mut subs = self.subs.write();
         subs
            .entry(key.clone())
            .or_insert_with(|| ArcRwSignal::new(false))
            .clone()
      };
      _ = read.try_with(|n| *n);
      (self.f)(&key, self.v.read().as_ref().unwrap())
   }

   /// Removes the listener for the given key.
   pub fn remove(&self, key: &T) {
      let mut subs = self.subs.write();
      subs.remove(key);
   }

   /// Clears the listeners for all keys.
   pub fn clear(&self) {
      let mut subs = self.subs.write();
      subs.clear();
   }
}
