use crate::hooked_map::{HookMap, HookedHashMap};
use core::cmp::Eq;
use core::hash::Hash;
use core::iter::IntoIterator;
use hashbrown::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MapOperation<K, V> {
    Insert { key: K, value: V },
    Update { key: K, value: V },
    Patch { key: K },
    Remove { key: K },
    Clear {},
}

pub trait ApplyMapOperation<K, V> {
    fn apply_map_op(&mut self, diff: MapOperation<K, V>);
    fn apply_map_ops(&mut self, diff: impl IntoIterator<Item = MapOperation<K, V>>) {
        for diff in diff {
            self.apply_map_op(diff);
        }
    }
}

impl<K, V> ApplyMapOperation<K, V> for HashMap<K, V>
where
    K: Hash + Eq,
{
    fn apply_map_op(&mut self, diff: MapOperation<K, V>) {
        match diff {
            MapOperation::Insert { key, value } => {
                self.insert(key, value);
            }
            MapOperation::Update { key, value } => {
                self.insert(key, value);
            }
            MapOperation::Remove { key } => {
                self.remove(&key);
            }
            MapOperation::Clear {} => {
                self.clear();
            }
            MapOperation::Patch { key: _ } => {
                unimplemented!("Patch not implemented for HashMap")
            }
        }
    }
}

impl<K, V, O> ApplyMapOperation<K, V> for HookedHashMap<K, V, O>
where
    K: Hash + Eq,
    O: HookMap<Key = K, Value = V>,
{
    fn apply_map_op(&mut self, diff: MapOperation<K, V>) {
        match diff {
            MapOperation::Insert { key, value } => {
                self.insert(key, value);
            }
            MapOperation::Update { key, value } => {
                self.insert(key, value);
            }
            MapOperation::Remove { key } => {
                self.remove(&key);
            }
            MapOperation::Clear {} => {
                self.clear();
            }
            MapOperation::Patch { key: _ } => {
                unimplemented!("Patch not implemented for HashMap")
            }
        }
    }
}
