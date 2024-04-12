use super::{
   ArcReadStoreField, ArcRwStoreField, ArcStore, ArcWriteStoreField, ReadStoreField, RwStoreField,
   Store, WriteStoreField,
};
use crate::{
   arena::Stored,
   prelude::{DefinedAt, SignalIsDisposed, SignalUpdateUntracked, SignalWithUntracked, Trigger},
   signal::trigger::ArcTrigger,
   source::Track,
   unwrap_signal,
};
use parking_lot::{
   MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use std::{
   iter::{self},
   marker::PhantomData,
   panic::Location,
   sync::Arc,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StorePath(Vec<StorePathSegment>);

impl From<Vec<StorePathSegment>> for StorePath {
   fn from(value: Vec<StorePathSegment>) -> Self {
      Self(value)
   }
}

impl StorePath {
   pub fn push(&mut self, segment: impl Into<StorePathSegment>) {
      self.0.push(segment.into());
   }

   pub fn pop(&mut self) -> Option<StorePathSegment> {
      self.0.pop()
   }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StorePathSegment(usize);

impl From<usize> for StorePathSegment {
   fn from(value: usize) -> Self {
      Self(value)
   }
}

impl From<&usize> for StorePathSegment {
   fn from(value: &usize) -> Self {
      Self(*value)
   }
}

impl FromIterator<StorePathSegment> for StorePath {
   fn from_iter<T: IntoIterator<Item = StorePathSegment>>(iter: T) -> Self {
      Self(Vec::from_iter(iter))
   }
}

pub trait StoreField<T>: Sized {
   type Orig;

   fn data(&self) -> Arc<RwLock<Self::Orig>>;

   fn get_trigger(&self, path: StorePath) -> ArcTrigger;

   fn path(&self) -> impl Iterator<Item = StorePathSegment>;

   fn reader(
      &self,
   ) -> impl for<'a> Fn(&'a RwLock<Self::Orig>) -> MappedRwLockReadGuard<'a, T> + Send + Sync + 'static;

   fn writer(
      self,
   ) -> impl for<'a> Fn(&'a RwLock<Self::Orig>) -> MappedRwLockWriteGuard<'a, T> + Send + Sync + 'static;

   #[track_caller]
   fn rw(self) -> RwStoreField<Self::Orig, T>
   where
      Self: Clone,
      Self::Orig: Send + Sync,
      T: Send + Sync,
   {
      RwStoreField {
         inner: Stored::new(self.arc_rw()),
      }
   }

   #[track_caller]
   fn read(self) -> ReadStoreField<Self::Orig, T>
   where
      Self: Clone,
      Self::Orig: Send + Sync,
      T: Send + Sync,
   {
      ReadStoreField {
         inner: Stored::new(self.arc_read()),
      }
   }

   #[track_caller]
   fn write(self) -> WriteStoreField<Self::Orig, T>
   where
      Self: Clone,
      Self::Orig: Send + Sync,
      T: Send + Sync,
   {
      WriteStoreField {
         inner: Stored::new(self.arc_write()),
      }
   }

   #[track_caller]
   fn arc_read(self) -> ArcReadStoreField<Self::Orig, T> {
      ArcReadStoreField {
         #[cfg(debug_assertions)]
         defined_at: std::panic::Location::caller(),
         data: self.data(),
         trigger: self.get_trigger(self.path().collect()),
         read: Arc::new({
            let read = self.reader();
            move |orig| read(orig)
         }),
      }
   }

   #[track_caller]
   fn arc_write(self) -> ArcWriteStoreField<Self::Orig, T> {
      ArcWriteStoreField {
         #[cfg(debug_assertions)]
         defined_at: std::panic::Location::caller(),
         data: self.data(),
         trigger: self.get_trigger(self.path().collect()),
         write: Arc::new({
            let write = self.writer();
            move |orig| write(orig)
         }),
      }
   }

   #[track_caller]
   fn arc_rw(self) -> ArcRwStoreField<Self::Orig, T>
   where
      Self: Clone,
   {
      ArcRwStoreField {
         #[cfg(debug_assertions)]
         defined_at: std::panic::Location::caller(),
         data: self.data(),
         trigger: self.get_trigger(self.path().collect()),
         read: Arc::new({
            let read = self.clone().reader();
            move |orig| read(orig)
         }),
         write: Arc::new({
            let write = self.writer();
            move |orig| write(orig)
         }),
      }
   }
}

impl<T> StoreField<T> for ArcStore<T> {
   type Orig = T;

   fn data(&self) -> Arc<RwLock<Self::Orig>> {
      Arc::clone(&self.value)
   }

   fn get_trigger(&self, path: StorePath) -> ArcTrigger {
      let triggers = &self.signals;
      let trigger = triggers.write().get_or_insert(path);
      trigger
   }

   fn path(&self) -> impl Iterator<Item = StorePathSegment> {
      iter::empty()
   }

   fn reader(
      &self,
   ) -> impl for<'a> Fn(&'a RwLock<Self::Orig>) -> MappedRwLockReadGuard<'a, T> + Send + Sync + 'static
   {
      |lock| {
         let guard = lock.read();
         RwLockReadGuard::map(guard, |n| n)
      }
   }

   fn writer(
      self,
   ) -> impl for<'a> Fn(&'a RwLock<Self::Orig>) -> MappedRwLockWriteGuard<'a, T> + Send + Sync + 'static
   {
      |lock| {
         let guard = lock.write();
         RwLockWriteGuard::map(guard, |n| n)
      }
   }
}

impl<T> DefinedAt for ArcStore<T> {
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

impl<T: Send + Sync + 'static> StoreField<T> for Store<T> {
   type Orig = T;

   fn data(&self) -> Arc<RwLock<Self::Orig>> {
      self
         .inner
         .get()
         .map(|inner| inner.data())
         .unwrap_or_else(unwrap_signal!(self))
   }

   fn get_trigger(&self, path: StorePath) -> ArcTrigger {
      self
         .inner
         .get()
         .map(|inner| inner.get_trigger(path))
         .unwrap_or_else(unwrap_signal!(self))
   }

   fn path(&self) -> impl Iterator<Item = StorePathSegment> {
      iter::empty()
   }

   fn reader(
      &self,
   ) -> impl for<'a> Fn(&'a RwLock<Self::Orig>) -> MappedRwLockReadGuard<'a, T> + Send + Sync + 'static
   {
      |lock| {
         let guard = lock.read();
         RwLockReadGuard::map(guard, |n| n)
      }
   }

   fn writer(
      self,
   ) -> impl for<'a> Fn(&'a RwLock<Self::Orig>) -> MappedRwLockWriteGuard<'a, T> + Send + Sync + 'static
   {
      |lock| {
         let guard = lock.write();
         RwLockWriteGuard::map(guard, |n| n)
      }
   }
}

#[derive(Debug)]
pub struct Subfield<Inner, Prev, T>
where
   Inner: StoreField<Prev>,
{
   #[cfg(debug_assertions)]
   defined_at: &'static Location<'static>,
   path_segment: StorePathSegment,
   inner: Inner,
   read: fn(&Prev) -> &T,
   write: fn(&mut Prev) -> &mut T,
   ty: PhantomData<T>,
}

impl<Inner, Prev, T> Clone for Subfield<Inner, Prev, T>
where
   Inner: StoreField<Prev> + Clone,
{
   fn clone(&self) -> Self {
      Self {
         #[cfg(debug_assertions)]
         defined_at: self.defined_at,
         path_segment: self.path_segment,
         inner: self.inner.clone(),
         read: self.read,
         write: self.write,
         ty: self.ty,
      }
   }
}

impl<Inner, Prev, T> Copy for Subfield<Inner, Prev, T> where Inner: StoreField<Prev> + Copy {}

impl<Inner, Prev, T> Subfield<Inner, Prev, T>
where
   Inner: StoreField<Prev>,
{
   #[track_caller]
   pub fn new(
      inner: Inner,
      path_segment: StorePathSegment,
      read: fn(&Prev) -> &T,
      write: fn(&mut Prev) -> &mut T,
   ) -> Self {
      Self {
         #[cfg(debug_assertions)]
         defined_at: Location::caller(),
         inner,
         path_segment,
         read,
         write,
         ty: PhantomData,
      }
   }
}

impl<Inner, Prev, T> StoreField<T> for Subfield<Inner, Prev, T>
where
   Inner: StoreField<Prev> + Send + Sync + Clone + 'static,
   Prev: 'static,
   T: 'static,
{
   type Orig = Inner::Orig;

   fn path(&self) -> impl Iterator<Item = StorePathSegment> {
      self.inner.path().chain(iter::once(self.path_segment))
   }

   fn data(&self) -> Arc<RwLock<Self::Orig>> {
      self.inner.data()
   }

   fn get_trigger(&self, path: StorePath) -> ArcTrigger {
      self.inner.get_trigger(path)
   }

   fn reader(
      &self,
   ) -> impl for<'a> Fn(&'a RwLock<Self::Orig>) -> MappedRwLockReadGuard<'a, T> + Send + Sync + 'static
   {
      let inner = self.inner.clone();
      let read = self.read;
      move |lock| {
         let inner = inner.reader();
         let lock = inner(lock);
         MappedRwLockReadGuard::map(lock, |inner| (read)(inner))
      }
   }

   fn writer(
      self,
   ) -> impl for<'a> Fn(&'a RwLock<Self::Orig>) -> MappedRwLockWriteGuard<'a, T> + Send + Sync + 'static
   {
      move |lock| {
         let inner = self.inner.clone().writer();
         let lock = inner(lock);
         MappedRwLockWriteGuard::map(lock, |inner| (self.write)(inner))
      }
   }
}

impl<Inner, Prev, T> DefinedAt for Subfield<Inner, Prev, T>
where
   Inner: StoreField<Prev>,
{
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

impl<Inner, Prev, T> Track for Subfield<Inner, Prev, T>
where
   Inner: StoreField<Prev> + Send + Sync + Clone + 'static,
   Prev: 'static,
   T: 'static,
{
   fn track(&self) {
      let trigger = self.get_trigger(self.path().collect());
      trigger.track();
   }
}

impl<Inner, Prev, T> SignalWithUntracked for Subfield<Inner, Prev, T>
where
   Inner: StoreField<Prev> + SignalWithUntracked<Value = Prev>,
{
   type Value = T;

   fn try_with_untracked<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> Option<U> {
      self.inner.try_with_untracked(|prev| fun((self.read)(prev)))
   }
}

impl<Inner, Prev, T> SignalIsDisposed for Subfield<Inner, Prev, T>
where
   Inner: StoreField<Prev>,
{
   fn is_disposed(&self) -> bool {
      false
   }
}

impl<Inner, Prev, T> Trigger for Subfield<Inner, Prev, T>
where
   Inner: StoreField<Prev> + Send + Sync + Clone + 'static,
   Prev: 'static,
   T: 'static,
{
   fn trigger(&self) {
      self.get_trigger(self.path().collect()).notify();
   }
}

impl<Inner, Prev, T> SignalUpdateUntracked for Subfield<Inner, Prev, T>
where
   Inner: StoreField<Prev> + SignalUpdateUntracked<Value = Prev>,
{
   type Value = T;

   fn try_update_untracked<U>(&self, fun: impl FnOnce(&mut Self::Value) -> U) -> Option<U> {
      self.inner.try_update_untracked(|prev| {
         let this = (self.write)(prev);
         fun(this)
      })
   }
}
