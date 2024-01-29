use crate::hooked_vec::{HookVec, HookedVec};
use crate::map_operation::MapOperation;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::any::Any;
use core::fmt::Debug;
use core::iter::IntoIterator;
use core::marker::PhantomData;
use std::future::Future;
use std::pin::{pin, Pin};
use std::task::{Context, Poll};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VecOperation<T> {
    Push { item: T },
    Pop,
    Insert { index: usize, item: T },
    Update { index: usize, item: T },
    Patch { index: usize },
    Remove { index: usize },
    Clear,
    Move { from: usize, to: usize },
}

impl<T> VecOperation<T> {
    pub fn as_ref(&self) -> VecOperation<&T> {
        match self {
            VecOperation::Push { item } => VecOperation::Push { item },
            VecOperation::Pop => VecOperation::Pop,
            VecOperation::Insert { index, item } => VecOperation::Insert {
                index: *index,
                item,
            },
            VecOperation::Update { index, item } => VecOperation::Update {
                index: *index,
                item,
            },
            VecOperation::Remove { index } => VecOperation::Remove { index: *index },
            VecOperation::Clear => VecOperation::Clear,
            VecOperation::Move { from, to } => VecOperation::Move {
                from: *from,
                to: *to,
            },
            VecOperation::Patch { index } => VecOperation::Patch { index: *index },
        }
    }
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> VecOperation<U> {
        match self {
            VecOperation::Push { item } => VecOperation::Push { item: f(item) },
            VecOperation::Insert { index, item } => VecOperation::Insert {
                index,
                item: f(item),
            },
            VecOperation::Update { index, item } => VecOperation::Update {
                index,
                item: f(item),
            },
            VecOperation::Remove { index } => VecOperation::Remove { index },
            VecOperation::Pop => VecOperation::Pop,
            VecOperation::Clear => VecOperation::Clear,
            VecOperation::Move { from, to } => VecOperation::Move { from, to },
            VecOperation::Patch { index } => VecOperation::Patch { index },
        }
    }
}

pub enum ApplyVecOperationResult<'a, T> {
    Push(&'a T),
    Pop(Option<T>),
    Insert { item: &'a T, index: usize },
    Update { item: &'a T, index: usize },
    Patch { index: usize },
    Remove { item: T, index: usize },
    Clear,
    Move { from: usize, to: usize },
}

pub trait ApplyVecOperation<T> {
    fn apply_op(&mut self, diff: VecOperation<T>) -> ApplyVecOperationResult<T>;
    fn apply_ops(&mut self, diff: impl IntoIterator<Item = VecOperation<T>>) {
        for diff in diff {
            self.apply_op(diff);
        }
    }
}

impl<T> ApplyVecOperation<T> for Vec<T> {
    fn apply_op(&mut self, diff: VecOperation<T>) -> ApplyVecOperationResult<T> {
        match diff {
            VecOperation::Push { item } => {
                self.push(item);
                ApplyVecOperationResult::Push(self.last().unwrap())
            }
            VecOperation::Pop => ApplyVecOperationResult::Pop(self.pop()),
            VecOperation::Insert { index, item } => {
                self.insert(index, item);
                ApplyVecOperationResult::Insert {
                    item: &self[index],
                    index,
                }
            }
            VecOperation::Update { index, item } => {
                self[index] = item;
                ApplyVecOperationResult::Update {
                    item: &self[index],
                    index,
                }
            }
            VecOperation::Remove { index } => ApplyVecOperationResult::Remove {
                item: self.remove(index),
                index,
            },
            VecOperation::Clear => {
                self.clear();
                ApplyVecOperationResult::Clear
            }
            VecOperation::Move { from, to } => {
                self.move_item(from, to);
                ApplyVecOperationResult::Move { from, to }
            }
            VecOperation::Patch { index: _ } => {
                unimplemented!("Patch not implemented for HookedVec");
                // ApplyVecOperationResult::Patch { index }
            }
        }
    }
}

pub trait VecExt<T> {
    fn move_item(&mut self, from: usize, to: usize);
}

impl<T> VecExt<T> for Vec<T> {
    fn move_item(&mut self, from: usize, to: usize) {
        let item = self.remove(from);
        if from < to {
            self.insert(to - 1, item);
        } else {
            self.insert(to, item);
        }
    }
}

impl<T, O> VecExt<T> for HookedVec<T, O>
where
    O: HookVec<Item = T>,
{
    fn move_item(&mut self, from: usize, to: usize) {
        let item = self.remove(from);
        if from < to {
            self.insert(to - 1, item);
        } else {
            self.insert(to, item);
        }
    }
}

impl<T, O> ApplyVecOperation<T> for HookedVec<T, O>
where
    O: HookVec<Item = T>,
{
    fn apply_op(&mut self, diff: VecOperation<T>) -> ApplyVecOperationResult<T> {
        match diff {
            VecOperation::Push { item } => {
                self.push(item);
                ApplyVecOperationResult::Push(self.last().unwrap())
            }
            VecOperation::Pop => ApplyVecOperationResult::Pop(self.pop()),
            VecOperation::Insert { index, item } => {
                self.insert(index, item);
                ApplyVecOperationResult::Insert {
                    item: &self[index],
                    index,
                }
            }
            VecOperation::Update { index, item } => {
                self.update(index, item);
                ApplyVecOperationResult::Update {
                    item: &self[index],
                    index,
                }
            }
            VecOperation::Remove { index } => ApplyVecOperationResult::Remove {
                item: self.remove(index),
                index,
            },
            VecOperation::Clear => {
                self.clear();
                ApplyVecOperationResult::Clear
            }
            VecOperation::Move { from, to } => {
                self.move_item(from, to);
                ApplyVecOperationResult::Move { from, to }
            }
            VecOperation::Patch { index: _ } => {
                unimplemented!("Patch not implemented for HookedVec");
                // ApplyVecOperationResult::Patch { index }
            }
        }
    }
}
