use core::marker::PhantomData;
use alloc::boxed::Box;
use smallvec::SmallVec;

use crate::{HookMap, HookVec, HookedVec, MapOperation, VecOperation};

pub type VecOperationRecord<T> = SmallVec<[VecOperation<T>; 2]>;
pub type MapOperationRecord<K, T> = SmallVec<[MapOperation<K, T>; 2]>;

pub struct HookFn<F, M>(pub F, PhantomData<M>);

pub type BoxedHookFn<T> = HookFn<Box<dyn FnMut(VecOperation<&T>)>, T>;

pub fn hook_fn<F, T>(f: F) -> HookFn<F, T>
    where
        F: FnMut(VecOperation<&T>),
{
    HookFn(f, PhantomData)
}

impl<T, F> HookVec for HookFn<F, T>
    where
        F: FnMut(VecOperation<&T>),
{
    type Item = T;
    fn on_push<'a>(&'a mut self, _items: impl Iterator<Item=&'a Self::Item>) {
        for item in _items {
            self.0(VecOperation::Push { item });
        }
    }

    fn on_pop<'a>(&'a mut self, _items: impl Iterator<Item=&'a Self::Item>) {
        self.0(VecOperation::Pop);
    }

    fn on_insert<'a>(&'a mut self, _index: usize, _items: impl Iterator<Item=&'a Self::Item>) {
        for item in _items {
            self.0(VecOperation::Insert {
                index: _index,
                item,
            });
        }
    }

    fn on_update(&mut self, _index: usize, item: &Self::Item) {
        self.0(VecOperation::Update {
            index: _index,
            item,
        });
    }

    fn on_patch(&mut self, _index: usize) {
        self.0(VecOperation::Patch { index: _index });
    }

    fn on_remove<'a>(&'a mut self, _index: usize, _items: impl Iterator<Item=&'a Self::Item>) {
        for _ in _items {
            self.0(VecOperation::Remove { index: _index });
        }
    }

    fn on_clear(&mut self) {
        self.0(VecOperation::Clear);
    }

    fn on_move(&mut self, _from: usize, _to: usize) {
        self.0(VecOperation::Move {
            from: _from,
            to: _to,
        });
    }

    fn on_swap(&mut self, _from: usize, _to: usize) {
        self.0(VecOperation::Swap {
            from: _from,
            to: _to,
        });
    }
}

impl<T> HookVec for VecOperationRecord<T>
    where
        T: Clone,
{
    type Item = T;

    fn on_push<'a>(&'a mut self, _items: impl Iterator<Item=&'a Self::Item>) {
        for item in _items {
            self.push(VecOperation::Push { item: item.clone() });
        }
    }

    fn on_pop<'a>(&'a mut self, _items: impl Iterator<Item=&'a Self::Item>) {
        self.push(VecOperation::Pop);
    }

    fn on_insert<'a>(&'a mut self, _index: usize, _items: impl Iterator<Item=&'a Self::Item>) {
        for item in _items {
            self.push(VecOperation::Insert {
                index: _index,
                item: item.clone(),
            });
        }
    }

    fn on_update(&mut self, _index: usize, _item: &Self::Item) {
        self.push(VecOperation::Update {
            index: _index,
            item: _item.clone(),
        });
    }

    fn on_remove<'a>(&'a mut self, _index: usize, _items: impl Iterator<Item=&'a Self::Item>) {
        for _ in _items {
            self.push(VecOperation::Remove { index: _index });
        }
    }

    fn on_clear(&mut self) {
        self.push(VecOperation::Clear);
    }

    fn on_move(&mut self, _from: usize, _to: usize) {
        self.push(VecOperation::Move {
            from: _from,
            to: _to,
        });
    }

    fn on_patch(&mut self, _index: usize) {
        self.push(VecOperation::Patch { index: _index });
    }

    fn on_swap(&mut self, _from: usize, _to: usize) {
        self.push(VecOperation::Swap {
            from: _from,
            to: _to,
        });
    }
}

impl<K, T> HookMap for MapOperationRecord<K, T>
    where
        T: Clone,
        K: Clone,
{
    type Key = K;
    type Value = T;

    fn on_insert<'a>(&'a mut self, _key: &'a Self::Key, _value: &'a Self::Value) {
        self.push(MapOperation::Insert {
            key: _key.clone(),
            value: _value.clone(),
        });
    }

    fn on_remove<'a>(&'a mut self, _key: &'a Self::Key, _value: &'a Self::Value) {
        self.push(MapOperation::Remove { key: _key.clone() });
    }

    fn on_clear(&mut self) {
        self.push(MapOperation::Clear {});
    }

    fn on_patch(&mut self, _key: &Self::Key) {
        self.push(MapOperation::Patch { key: _key.clone() });
    }
}
