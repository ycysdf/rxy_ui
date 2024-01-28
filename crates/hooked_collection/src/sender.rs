use crate::{HookMap, HookVec, MapOperation, VecOperation};
use alloc::vec;
use async_channel::Sender;

impl<T> HookVec for Sender<VecOperation<T>>
where
    T: Clone,
{
    type Item = T;

    fn on_push<'a>(&'a mut self, items: impl Iterator<Item = &'a Self::Item>) {
        for item in items {
            let _ = self.send_blocking(VecOperation::Push { item: item.clone() });
        }
    }

    fn on_pop<'a>(&'a mut self, _items: impl Iterator<Item = &'a Self::Item>) {
        let _ = self.send_blocking(VecOperation::Pop);
    }
    fn on_insert<'a>(&'a mut self, index: usize, items: impl Iterator<Item = &'a Self::Item>) {
        for item in items {
            let _ = self.send_blocking(VecOperation::Insert {
                index,
                item: item.clone(),
            });
        }
    }
    fn on_update(&mut self, index: usize, item: &Self::Item) {
        let _ = self.send_blocking(VecOperation::Update {
            index,
            item: item.clone(),
        });
    }
    fn on_remove<'a>(&'a mut self, index: usize, items: impl Iterator<Item = &'a Self::Item>) {
        for _ in items {
            let _ = self.send_blocking(VecOperation::Remove { index });
        }
    }
    fn on_clear(&mut self) {
        let _ = self.send_blocking(VecOperation::Clear);
    }
    fn on_move(&mut self, from: usize, to: usize) {
        let _ = self.send_blocking(VecOperation::Move { from, to });
    }
}

impl<K, V> HookMap for Sender<MapOperation<K, V>>
where
    K: Clone,
    V: Clone,
{
    type Key = K;
    type Value = V;

    fn on_insert<'a>(&'a mut self, _key: &'a Self::Key, _value: &'a Self::Value) {
        let _ = self.send_blocking(MapOperation::Insert {
            key: _key.clone(),
            value: _value.clone(),
        });
    }

    fn on_remove<'a>(&'a mut self, _key: &'a Self::Key, _value: &'a Self::Value) {
        let _ = self.send_blocking(MapOperation::Remove { key: _key.clone() });
    }

    fn on_clear(&mut self) {
        let _ = self.send_blocking(MapOperation::Clear {});
    }
}

#[cfg(test)]
mod tests {
    use crate::{HookedHashMap, HookedVec};

    use super::*;

    #[test]
    fn vec_observe() {
        let (sender, receiver) = async_channel::unbounded();
        let mut vec = HookedVec::new(sender);
        vec.push(1);
        assert_eq!(receiver.try_recv().unwrap(), VecOperation::Push { item: 1 });
        vec.push(2);
        assert_eq!(receiver.try_recv().unwrap(), VecOperation::Push { item: 2 });
        vec.pop();
        assert_eq!(receiver.try_recv().unwrap(), VecOperation::Pop);
        vec.clear();
        assert_eq!(receiver.try_recv().unwrap(), VecOperation::Clear);
        vec.extend(vec![1, 2, 3]);
        assert_eq!(receiver.try_recv().unwrap(), VecOperation::Push { item: 1 });
        assert_eq!(receiver.try_recv().unwrap(), VecOperation::Push { item: 2 });
        assert_eq!(receiver.try_recv().unwrap(), VecOperation::Push { item: 3 });
        vec.insert(1, 4);
        assert_eq!(
            receiver.try_recv().unwrap(),
            VecOperation::Insert { index: 1, item: 4 }
        );
        vec.remove(1);
        assert_eq!(
            receiver.try_recv().unwrap(),
            VecOperation::Remove { index: 1 }
        );
        vec.swap_remove(0);
        let pre_len = vec.len() - 1;
        assert_eq!(
            receiver.try_recv().unwrap(),
            VecOperation::Remove { index: 0 }
        );
        assert_eq!(
            receiver.try_recv().unwrap(),
            VecOperation::Move {
                from: pre_len,
                to: 0
            }
        );
        vec.update(0, 5);
        assert_eq!(
            receiver.try_recv().unwrap(),
            VecOperation::Update { index: 0, item: 5 }
        );
    }

    #[test]
    fn map_observe() {
        let (sender, receiver) = async_channel::unbounded();

        let mut map = HookedHashMap::new(sender);
        map.insert(1, 1);
        assert_eq!(
            receiver.try_recv().unwrap(),
            MapOperation::Insert { key: 1, value: 1 }
        );
        map.remove(&1);
        assert_eq!(
            receiver.try_recv().unwrap(),
            MapOperation::Remove { key: 1 }
        );
        map.extend(vec![(1, 1), (2, 2)]);
        assert_eq!(
            receiver.try_recv().unwrap(),
            MapOperation::Insert { key: 1, value: 1 }
        );
        assert_eq!(
            receiver.try_recv().unwrap(),
            MapOperation::Insert { key: 2, value: 2 }
        );
        map.clear();
        assert_eq!(receiver.try_recv().unwrap(), MapOperation::Clear {});
    }
}
