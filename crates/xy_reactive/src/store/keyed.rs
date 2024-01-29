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

#[derive(Debug)]
pub struct KeyedField<Inner, Prev, T, Key>
where
    Inner: StoreField<Prev>,
    for<'a> &'a T: IntoIterator,
{
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    path_segment: StorePathSegment,
    inner: Inner,
    read: fn(&Prev) -> &T,
    write: fn(&mut Prev) -> &mut T,
    key_fn: fn(&<&T as IntoIterator>::Item) -> Key,
    ty: PhantomData<T>,
}

pub trait StoreFieldKeyed<Inner, Prev, Row, Key> {
    fn key(
        self,
        key: Key,
        key_fn: fn(&Row) -> Key,
    ) -> AtKey<Inner, Prev, Row, Key>;
}

impl<Inner, Prev, Row, Key> StoreFieldKeyed<Inner, Prev, Row, Key> for Inner
where
    Inner: StoreField<Prev> + Send + Sync + Clone + 'static,
    for<'a> &'a Prev: IntoIterator<Item = &'a Row>,
{
    #[track_caller]
    fn key(
        self,
        key: Key,
        key_fn: fn(&Row) -> Key,
    ) -> AtKey<Inner, Prev, Row, Key> {
        AtKey {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            inner: self,
            key,
            key_fn,
            ty: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct AtKey<Inner, Prev, Row, Key> {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    inner: Inner,
    key: Key,
    key_fn: fn(&Row) -> Key,
    ty: PhantomData<Prev>,
}

impl<Inner, Prev, Row, Key> Clone for AtKey<Inner, Prev, Row, Key>
where
    Inner: Clone,
    Key: Clone,
{
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            inner: self.inner.clone(),
            key: self.key.clone(),
            key_fn: self.key_fn,
            ty: PhantomData,
        }
    }
}

impl<Inner, Prev, Row, Key> Copy for AtKey<Inner, Prev, Row, Key>
where
    Inner: Copy,
    Key: Copy,
{
}

impl<Inner, Prev, Row, Key> StoreField<Row> for AtKey<Inner, Prev, Row, Key>
where
    Inner: StoreField<Prev> + Send + Sync + Clone + 'static,
    Prev: Index<Key, Output = Row> + IndexMut<Key, Output = Row> + 'static,
    Key: Clone + PartialEq + Send + Sync + 'static,
    for<'a> &'a Prev: IntoIterator<Item = &'a Row>,
    Row: 'static,
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
        let segment = {
            let data = self.inner.data();
            let inner_reader = self.inner.reader();
            let inner_data = inner_reader(&data);
            let value = inner_data
                .into_iter()
                .enumerate()
                .find_map(|(idx, row)| {
                    if (self.key_fn)(row) == self.key {
                        Some(idx.into())
                    } else {
                        None
                    }
                })
                .unwrap(); // TODO
            value
        };
        self.inner.path().chain(iter::once(segment))
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
        let key = self.key.clone();
        let key_fn = self.key_fn;
        move |lock| {
            let inner = inner.reader();
            let lock = inner(lock);
            MappedRwLockReadGuard::map(lock, |prev| {
                (&prev).into_iter().find(|row| key_fn(row) == key).unwrap()
            })
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
            let key = self.key.clone();
            MappedRwLockWriteGuard::map(lock, |prev| &mut prev[key])
        }
    }
}

impl<Inner, Prev, Row, Key> DefinedAt for AtKey<Inner, Prev, Row, Key> {
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

impl<Inner, Prev, Row, Key> Track for AtKey<Inner, Prev, Row, Key>
where
    Inner: StoreField<Prev> + Send + Sync + Clone + 'static,
    Prev: Index<Key, Output = Row> + IndexMut<Key, Output = Row> + 'static,
    Key: Clone + PartialEq + Send + Sync + 'static,
    for<'a> &'a Prev: IntoIterator<Item = &'a Row>,
    Row: 'static,
{
    fn track(&self) {
        let trigger = self.get_trigger(self.path().collect());
        trigger.track();
    }
}

impl<Inner, Prev, Row, Key> SignalWithUntracked for AtKey<Inner, Prev, Row, Key>
where
    Inner: SignalWithUntracked<Value = Prev> + Send + Sync + Clone + 'static,
    Prev: Index<Key, Output = Row> + IndexMut<Key, Output = Row> + 'static,
    Key: Clone + PartialEq + Send + Sync + 'static,
    for<'a> &'a Prev: IntoIterator<Item = &'a Row>,
    Row: 'static,
{
    type Value = Prev::Output;

    fn try_with_untracked<U>(
        &self,
        fun: impl FnOnce(&Self::Value) -> U,
    ) -> Option<U> {
        self.inner.try_with_untracked(|prev| {
            fun((&prev)
                .into_iter()
                .find(|n| (self.key_fn)(n) == self.key)
                .unwrap())
        })
    }
}

impl<Inner, Prev, Row, Key> SignalIsDisposed for AtKey<Inner, Prev, Row, Key> {
    fn is_disposed(&self) -> bool {
        false
    }
}

impl<Inner, Prev, Row, Key> Trigger for AtKey<Inner, Prev, Row, Key>
where
    Inner: StoreField<Prev> + Send + Sync + Clone + 'static,
    Prev: Index<Key, Output = Row> + IndexMut<Key, Output = Row> + 'static,
    Key: Clone + PartialEq + Send + Sync + 'static,
    for<'a> &'a Prev: IntoIterator<Item = &'a Row>,
    Row: 'static,
{
    fn trigger(&self) {
        self.get_trigger(self.path().collect()).notify();
    }
}

impl<Inner, Prev, Row, Key> SignalUpdateUntracked
    for AtKey<Inner, Prev, Row, Key>
where
    Inner: StoreField<Prev> + SignalUpdateUntracked<Value = Prev>,
    Prev: Index<Key> + IndexMut<Key> + 'static,
    Prev::Output: Sized,
    Key: Clone,
{
    type Value = Prev::Output;

    fn try_update_untracked<U>(
        &self,
        fun: impl FnOnce(&mut Self::Value) -> U,
    ) -> Option<U> {
        self.inner.try_update_untracked(|prev: &mut Prev| {
            let this = &mut prev[self.key.clone()];
            fun(this)
        })
    }
}

pub trait KeyedStoreFieldIterator<Prev, Row, Key>: Sized {
    fn iter_keyed(
        self,
        key_fn: fn(&Row) -> Key,
    ) -> KeyedStoreFieldIter<Self, Prev, Row, Key>;
}

impl<Inner, Prev, Row, Key> KeyedStoreFieldIterator<Prev, Row, Key> for Inner
where
    Inner: StoreField<Prev>,
    Prev::Output: Sized,
    Prev: Index<usize> + AsRef<[Prev::Output]>,
{
    fn iter_keyed(
        self,
        key_fn: fn(&Row) -> Key,
    ) -> KeyedStoreFieldIter<Inner, Prev, Row, Key> {
        // reactively track changes to this field
        let trigger = self.get_trigger(self.path().collect());
        trigger.track();

        // get the current length of the field by accessing slice
        let reader = self.reader();
        let len = reader(&self.data()).as_ref().len();

        // return the iterator
        KeyedStoreFieldIter {
            field: self,
            idx: 0,
            len,
            ty: PhantomData,
            key_fn,
        }
    }
}

pub struct KeyedStoreFieldIter<Inner, Prev, Row, Key> {
    field: Inner,
    idx: usize,
    len: usize,
    ty: PhantomData<Prev>,
    key_fn: fn(&Row) -> Key,
}

impl<Inner, Prev, Row, Key> Iterator
    for KeyedStoreFieldIter<Inner, Prev, Row, Key>
where
    Inner: StoreField<Prev> + Send + Sync + Clone + 'static,
    Prev: Index<usize, Output = Row> + IndexMut<usize, Output = Row> + 'static,
    Key: Clone + PartialEq + Send + Sync + 'static,
    for<'a> &'a Prev: IntoIterator<Item = &'a Row>,
    Row: 'static,
{
    type Item = AtKey<Inner, Prev, Row, Key>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.len {
            let data = self.field.data();
            let reader = self.field.reader();
            let key = (self.key_fn)(&reader(&data)[self.idx]);
            let field = self.field.clone().key(key, self.key_fn);
            self.idx += 1;
            Some(field)
        } else {
            None
        }
    }
}
