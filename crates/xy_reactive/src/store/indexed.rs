use super::{StoreField, StorePath, StorePathSegment};
use crate::{
    prelude::{
        DefinedAt, SignalIsDisposed, SignalUpdateUntracked,
        SignalWithUntracked, Trigger,
    },
    signal::trigger::ArcTrigger,
    source::Track,
};
use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock};
use std::{
    iter,
    marker::PhantomData,
    ops::{Index, IndexMut},
    panic::Location,
    sync::Arc,
};

pub trait StoreFieldIndex<Inner, Prev, Idx> {
    fn index(self, index: Idx) -> AtIndex<Inner, Prev, Idx>;
}

impl<Inner, Prev, Idx> StoreFieldIndex<Inner, Prev, Idx> for Inner
where
    Inner: StoreField<Prev> + Send + Sync + Clone + 'static,
    Prev: Index<Idx> + IndexMut<Idx>,
{
    #[track_caller]
    fn index(self, index: Idx) -> AtIndex<Inner, Prev, Idx> {
        AtIndex {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            inner: self,
            idx: index,
            prev: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct AtIndex<Inner, Prev, Idx> {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    inner: Inner,
    idx: Idx,
    prev: PhantomData<Prev>,
}

impl<Inner, Prev, Idx> Clone for AtIndex<Inner, Prev, Idx>
where
    Inner: Clone,
    Idx: Clone,
{
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            inner: self.inner.clone(),
            idx: self.idx.clone(),
            prev: PhantomData,
        }
    }
}

impl<Inner, Prev, Idx> Copy for AtIndex<Inner, Prev, Idx>
where
    Inner: Copy,
    Idx: Copy,
{
}

impl<Inner, Prev, Idx> StoreField<Prev::Output> for AtIndex<Inner, Prev, Idx>
where
    Inner: StoreField<Prev> + Send + Sync + Clone + 'static,
    Prev: Index<Idx> + IndexMut<Idx> + 'static,
    Prev::Output: Sized,
    for<'a> &'a Idx: Into<StorePathSegment>,
    Idx: Clone + Send + Sync + 'static,
{
    type Orig = Inner::Orig;

    #[inline(always)]
    fn data(&self) -> Arc<RwLock<Self::Orig>> {
        self.inner.data()
    }

    #[inline(always)]
    fn get_trigger(&self, path: StorePath) -> ArcTrigger {
        self.inner.get_trigger(path)
    }

    #[inline(always)]
    fn path(&self) -> impl Iterator<Item = StorePathSegment> {
        self.inner.path().chain(iter::once((&self.idx).into()))
    }

    fn reader(
        &self,
    ) -> impl for<'a> Fn(
        &'a RwLock<Self::Orig>,
    ) -> MappedRwLockReadGuard<'a, Prev::Output>
           + Send
           + Sync
           + 'static {
        let inner = self.inner.clone();
        let idx = self.idx.clone();
        move |lock| {
            let inner = inner.reader();
            let lock = inner(lock);
            let idx = idx.clone();
            MappedRwLockReadGuard::map(lock, |prev| &prev[idx])
        }
    }

    fn writer(
        self,
    ) -> impl for<'a> Fn(
        &'a RwLock<Self::Orig>,
    ) -> MappedRwLockWriteGuard<'a, Prev::Output>
           + Send
           + Sync
           + 'static {
        move |lock| {
            let inner = self.inner.clone().writer();
            let lock = inner(lock);
            let idx = self.idx.clone();
            MappedRwLockWriteGuard::map(lock, |prev| &mut prev[idx])
        }
    }
}

impl<Inner, Prev, Idx> DefinedAt for AtIndex<Inner, Prev, Idx> {
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

impl<Inner, Prev, Idx> Track for AtIndex<Inner, Prev, Idx>
where
    Inner: StoreField<Prev> + Send + Sync + Clone + 'static,
    Prev: Index<Idx> + IndexMut<Idx> + 'static,
    Prev::Output: Sized,
    for<'a> &'a Idx: Into<StorePathSegment>,
    Idx: Clone + Send + Sync + 'static,
{
    fn track(&self) {
        let trigger = self.get_trigger(self.path().collect());
        trigger.track();
    }
}

impl<Inner, Prev, Idx> SignalWithUntracked for AtIndex<Inner, Prev, Idx>
where
    Inner: StoreField<Prev> + SignalWithUntracked<Value = Prev>,
    Prev: Index<Idx> + IndexMut<Idx> + 'static,
    Prev::Output: Sized,
    Idx: Clone + 'static,
{
    type Value = Prev::Output;

    fn try_with_untracked<U>(
        &self,
        fun: impl FnOnce(&Self::Value) -> U,
    ) -> Option<U> {
        self.inner
            .try_with_untracked(|prev| fun(&prev[self.idx.clone()]))
    }
}

impl<Inner, Prev, Idx> SignalIsDisposed for AtIndex<Inner, Prev, Idx> {
    fn is_disposed(&self) -> bool {
        false
    }
}

impl<Inner, Prev, Idx> Trigger for AtIndex<Inner, Prev, Idx>
where
    Inner: StoreField<Prev> + Send + Sync + Clone + 'static,
    Prev: Index<Idx> + IndexMut<Idx> + 'static,
    Prev::Output: Sized,
    for<'a> &'a Idx: Into<StorePathSegment>,
    Idx: Clone + Send + Sync + 'static,
{
    fn trigger(&self) {
        self.get_trigger(self.path().collect()).notify();
    }
}

impl<Inner, Prev, Idx> SignalUpdateUntracked for AtIndex<Inner, Prev, Idx>
where
    Inner: StoreField<Prev> + SignalUpdateUntracked<Value = Prev>,
    Prev: Index<Idx> + IndexMut<Idx> + 'static,
    Prev::Output: Sized,
    Idx: Clone,
{
    type Value = Prev::Output;

    fn try_update_untracked<U>(
        &self,
        fun: impl FnOnce(&mut Self::Value) -> U,
    ) -> Option<U> {
        self.inner.try_update_untracked(|prev: &mut Prev| {
            let this = &mut prev[self.idx.clone()];
            fun(this)
        })
    }
}

pub trait StoreFieldIterator<Prev>: Sized {
    fn iter(self) -> StoreFieldIter<Self, Prev>;
}

impl<Inner, Prev> StoreFieldIterator<Prev> for Inner
where
    Inner: StoreField<Prev>,
    Prev::Output: Sized,
    Prev: Index<usize> + AsRef<[Prev::Output]>,
{
    fn iter(self) -> StoreFieldIter<Inner, Prev> {
        // reactively track changes to this field
        let trigger = self.get_trigger(self.path().collect());
        trigger.track();

        // get the current length of the field by accessing slice
        let reader = self.reader();
        let len = reader(&self.data()).as_ref().len();

        // return the iterator
        StoreFieldIter {
            field: self,
            idx: 0,
            len,
            ty: PhantomData,
        }
    }
}

pub struct StoreFieldIter<Inner, Prev> {
    field: Inner,
    idx: usize,
    len: usize,
    ty: PhantomData<Prev>,
}

impl<Inner, Prev> Iterator for StoreFieldIter<Inner, Prev>
where
    Inner: StoreField<Prev> + Send + Sync + Clone + 'static,
    Inner::Orig: 'static,
    Prev: Index<usize> + IndexMut<usize> + Clone + 'static,
    Prev::Output: Sized + 'static,
{
    type Item = AtIndex<Inner, Prev, usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.len {
            let field = self.field.clone().index(self.idx);
            self.idx += 1;
            Some(field)
        } else {
            None
        }
    }
}
