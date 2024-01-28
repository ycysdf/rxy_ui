use core::borrow::Borrow;
use core::cmp::Eq;
use core::hash::{BuildHasher, Hash};
use core::iter::{once, ExactSizeIterator, Extend, IntoIterator, Iterator};
use core::marker::Sized;
use core::ops::Deref;
use hashbrown::{HashMap, TryReserveError};

pub trait HookMap {
    type Key;
    type Value;

    #[inline(always)]
    fn on_insert<'a>(&'a mut self, _key: &'a Self::Key, _value: &'a Self::Value) {}
    #[inline(always)]
    fn on_remove<'a>(&'a mut self, _key: &'a Self::Key, _value: &'a Self::Value) {}
    #[inline(always)]
    fn on_clear(&mut self) {}
    #[inline(always)]
    fn on_patch(&mut self, _key: &'_ Self::Key) {}
}

pub struct HookedHashMap<K, V, O> {
    map: HashMap<K, V>,
    observer: O,
}

impl<K, V, O> HookedHashMap<K, V, O>
where
    K: Eq + Hash,
{
    pub fn new(observer: O) -> Self {
        Self {
            map: HashMap::new(),
            observer,
        }
    }

    pub fn with_capacity(capacity: usize, observer: O) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
            observer,
        }
    }

    #[inline(always)]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.map.shrink_to(min_capacity);
    }

    #[inline(always)]
    pub fn shrink_to_fit(&mut self) {
        self.map.shrink_to_fit();
    }

    #[inline(always)]
    pub fn reserve(&mut self, additional: usize) {
        self.map.reserve(additional);
    }

    #[inline(always)]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.map.try_reserve(additional)
    }
}

impl<K, V, O> Deref for HookedHashMap<K, V, O> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<K, V, O> Extend<(K, V)> for HookedHashMap<K, V, O>
where
    K: Eq + Hash,
    O: HookMap<Key = K, Value = V>,
{
    #[inline]
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        let iter = iter.into_iter();
        let reserve = if self.map.is_empty() {
            iter.size_hint().0
        } else {
            (iter.size_hint().0 + 1) / 2
        };
        self.map.reserve(reserve);
        iter.for_each(move |(k, v)| {
            self.observer.on_insert(&k, &v);
            self.map.insert(k, v);
        });
    }
}

impl<K, V, O> HookedHashMap<K, V, O>
where
    K: Eq + Hash,
    O: HookMap<Key = K, Value = V>,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.observer.on_insert(&key, &value);
        self.map.insert(key, value)
    }

    pub fn patch(&mut self, key: &K, f: impl FnOnce(&mut V)) {
        if let Some(value) = self.map.get_mut(key) {
            f(value);
            self.observer.on_patch(key);
        }
    }

    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let entry = self.map.remove_entry(key);

        if let Some((key, value)) = &entry {
            self.observer.on_remove(key, value);
        }
        entry.map(|(_, v)| v)
    }

    pub fn remove_entry<Q: ?Sized>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let entry = self.map.remove_entry(key);

        if let Some((key, value)) = &entry {
            self.observer.on_remove(key, value);
        }
        entry
    }

    pub fn clear(&mut self) {
        if self.len() == 0 {
            return;
        }
        self.observer.on_clear();
        self.map.clear();
    }
}
