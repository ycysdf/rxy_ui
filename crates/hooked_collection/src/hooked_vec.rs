use crate::VecExt;
use alloc::collections::TryReserveError;
use alloc::vec::Vec;
use core::cmp::{Ord, Ordering};
use core::iter::{once, ExactSizeIterator, Extend, IntoIterator, Iterator};
use core::ops::{Deref, FnMut, Index};
use core::slice::SliceIndex;

pub trait HookVec {
    type Item;

    #[inline]
    fn on_push<'a>(&'a mut self, _items: impl Iterator<Item = &'a Self::Item>) {}
    #[inline]
    fn on_pop<'a>(&'a mut self, _items: impl Iterator<Item = &'a Self::Item>) {}
    #[inline]
    fn on_insert<'a>(&'a mut self, _index: usize, _items: impl Iterator<Item = &'a Self::Item>) {}
    #[inline]
    fn on_update(&mut self, _index: usize, _item: &Self::Item) {}
    #[inline]
    fn on_patch(&mut self, _index: usize) {}
    #[inline]
    fn on_remove<'a>(&'a mut self, _index: usize, _items: impl Iterator<Item = &'a Self::Item>) {}
    #[inline]
    fn on_clear(&mut self) {}
    #[inline]
    fn on_move(&mut self, _from: usize, _to: usize) {}
}

pub struct HookedVec<T, O> {
    pub(crate) vec: Vec<T>,
    pub(crate) observer: O,
}

impl<T, O> HookedVec<T, O> {
    pub fn new(observer: O) -> Self {
        Self {
            vec: Vec::new(),
            observer,
        }
    }

    pub fn into_inner(self) -> (Vec<T>, O) {
        (self.vec, self.observer)
    }

    pub fn from_vec(vec: Vec<T>, observer: O) -> Self {
        Self { vec, observer }
    }

    pub fn with_capacity(capacity: usize, observer: O) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
            observer,
        }
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.vec.reserve(additional);
    }

    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.vec.reserve_exact(additional);
    }

    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.vec.shrink_to(min_capacity);
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.vec.shrink_to_fit();
    }

    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.vec.try_reserve(additional)
    }

    #[inline]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.vec.try_reserve_exact(additional)
    }
}

impl<T, I, O> Index<I> for HookedVec<T, O>
where
    I: SliceIndex<[T], Output = T>,
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        self.vec.index(index)
    }
}

impl<T, O> Deref for HookedVec<T, O> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<T, O> Extend<T> for HookedVec<T, O>
where
    O: HookVec<Item = T>,
{
    fn extend<II: IntoIterator<Item = T>>(&mut self, iter: II) {
        let prev_len = self.vec.len();
        self.vec.extend(iter);
        self.observer.on_push(self.vec.iter().skip(prev_len));
    }
}

impl<T, O> HookedVec<T, O>
where
    O: HookVec<Item = T>,
{
    pub fn push(&mut self, item: T) {
        self.observer.on_push(once(&item));
        self.vec.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        let item = self.vec.pop();
        if let Some(item) = &item {
            self.observer.on_pop(once(item));
        }
        item
    }

    pub fn insert(&mut self, index: usize, item: T) {
        self.observer.on_insert(index, once(&item));
        self.vec.insert(index, item);
    }

    pub fn update(&mut self, index: usize, item: T) {
        self.observer.on_update(index, &item);
        self.vec[index] = item;
    }

    pub fn patch(&mut self, index: usize, f: impl FnOnce(&mut T)) {
        f(&mut self.vec[index]);
        self.observer.on_patch(index);
    }

    pub fn clear(&mut self) {
        if self.len() == 0 {
            return;
        }
        self.observer.on_clear();
        self.vec.clear();
    }

    pub fn remove(&mut self, index: usize) -> T {
        let item = self.vec.remove(index);
        self.observer.on_remove(index, once(&item));
        item
    }

    pub fn resize(&mut self, new_len: usize, value: T)
    where
        T: Clone,
    {
        let prev_len = self.vec.len();
        let is_push = match new_len.cmp(&prev_len) {
            Ordering::Less => {
                self.observer.on_pop(self.vec.iter().skip(new_len));
                false
            }
            Ordering::Equal => return,
            Ordering::Greater => true,
        };
        self.vec.resize(new_len, value);
        if is_push {
            self.observer.on_push(self.vec.iter().skip(prev_len))
        }
    }

    pub fn resize_with<F: FnMut() -> T>(&mut self, new_len: usize, f: F) {
        let prev_len = self.vec.len();
        let is_push = match new_len.cmp(&prev_len) {
            Ordering::Less => {
                self.observer.on_pop(self.vec.iter().skip(new_len));
                false
            }
            Ordering::Equal => return,
            Ordering::Greater => true,
        };
        self.vec.resize_with(new_len, f);
        if is_push {
            self.observer.on_push(self.vec.iter().skip(prev_len))
        }
    }

    pub fn swap_remove(&mut self, index: usize) -> T {
        let item = self.vec.swap_remove(index);
        self.observer.on_remove(index, once(&item));
        self.observer.on_move(self.vec.len() - 1, index);
        item
    }

    pub fn truncate(&mut self, len: usize) {
        self.observer.on_pop(self.vec.iter().skip(len));
        self.vec.truncate(len);
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.move_item(a, b);
        self.move_item(b - 1, a);
        // self.observer.on_remove(a, once(&self.vec[a]));
        // self.observer.on_remove(b, once(&self.vec[b]));
        // self.observer.on_insert(a, once(&self.vec[b]));
        // self.observer.on_insert(b, once(&self.vec[a]));
        // self.vec.swap(a, b);
    }
}
