// Much of the code was copied from https://github.com/gbj/tachys/blob/main/tachydom/src/view/keyed.rs

use alloc::vec;
use alloc::vec::Vec;
use core::clone::Clone;
use core::cmp::Eq;
use core::fmt::Debug;
use core::hash::{BuildHasherDefault, Hash};

use indexmap::IndexSet;

use crate::utils::AHasher;

type FxIndexSet<T> = IndexSet<T, BuildHasherDefault<AHasher>>;

pub fn diff<K: Eq + Debug + Hash>(from: &FxIndexSet<K>, to: &FxIndexSet<K>) -> Diff {
   if from.is_empty() && to.is_empty() {
      return Diff::NoChanged;
   }
   if to.is_empty() {
      return Diff::Cleared;
   }
   if from.is_empty() {
      return Diff::Replaced;
   }

   let mut removed = vec![];
   let mut moved = vec![];
   let mut added = vec![];
   let mut no_changed = vec![];
   let max_len = core::cmp::max(from.len(), to.len());

   for index in 0..max_len {
      let from_item = from.get_index(index);
      let to_item = to.get_index(index);

      // if they're the same, do nothing
      if from_item == to_item {
         if from_item.is_some() {
            no_changed.push(index);
         }
         continue;
      }

      // if it's only in old, not new, remove it
      if let Some(from_item) = from_item {
         if !to.contains(from_item) {
            removed.push(index);
         }
      }
      // if it's only in new, not old, add it
      if let Some(to_item) = to_item {
         if !from.contains(to_item) {
            added.push(index);
         }
      }
      // if it's in both old and new, it can either
      // 1) be moved (and need to move in the DOM)
      // 2) be moved (but not need to move in the DOM)
      //    * this would happen if, for example, 2 items
      //      have been added before it, and it has moved by 2
      if let Some(from_item) = from_item {
         if let Some(to_item) = to.get_full(from_item) {
            let moves_forward_by = (to_item.0 as i32) - (index as i32);

            moved.push(DiffOpMove {
               from: index,
               len: 1,
               to: to_item.0,
               can_ignored: moves_forward_by == (added.len() as i32) - (removed.len() as i32),
            });
         }
      }
   }

   Diff::PartDiff {
      removed,
      moved_sum_count: moved.len(),
      moved,
      added,
      no_changed,
   }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiffOpMove {
   pub from: usize,
   pub to: usize,
   pub len: usize,
   pub can_ignored: bool,
}

impl Default for DiffOpMove {
   fn default() -> Self {
      Self {
         from: 0,
         to: 0,
         len: 1,
         can_ignored: true,
      }
   }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Diff {
   NoChanged,
   Cleared,
   Replaced,
   PartDiff {
      removed: Vec<usize>,
      moved: Vec<DiffOpMove>,
      moved_sum_count: usize,
      added: Vec<usize>,
      no_changed: Vec<usize>,
   },
}

impl Default for Diff {
   fn default() -> Self {
      Self::NoChanged
   }
}

impl Diff {
   //     pub fn get_no_changed(&self, from_len: usize) -> Vec<usize> {
   //         if let Diff::NoChanged = self {
   //             return (0..from_len).into_iter().collect();
   //         };
   //         if let Diff::PartDiff { moved, removed, .. } = self {
   //             let set = IndexSet::new();
   // let s = 0..1;
   //
   //             let indices = (0..from_len).into_iter().collect::<Vec<_>>();
   //             for item in moved {
   //                 for i in item.from..(item.from + item.len) {
   //
   //                 }
   //             }
   //
   //         };
   //         Default::default()
   //     }

   /// Group adjacent items that are being moved as a group.
   /// For example from `[2, 3, 5, 6]` to `[1, 2, 3, 4, 5, 6]` should result
   /// in a move for `2,3` and `5,6` rather than 4 individual moves.
   pub fn combine_move(&mut self) {
      let Diff::PartDiff {
         moved,
         moved_sum_count,
         ..
      } = self
      else {
         return;
      };
      let mut prev: Option<DiffOpMove> = None;
      let mut new_moved = Vec::with_capacity(*moved_sum_count);
      for m in moved.drain(..) {
         match prev {
            Some(mut p) => {
               if (m.from == p.from + p.len) && (m.to == p.to + p.len) {
                  p.len += 1;
                  prev = Some(p);
               } else {
                  new_moved.push(prev.take().unwrap());
                  prev = Some(m);
               }
            }
            None => prev = Some(m),
         }
      }
      if let Some(prev) = prev {
         new_moved.push(prev)
      }
      *moved = new_moved;
   }

   pub fn unpack_moves(&self) -> (Vec<DiffOpMove>, Vec<usize>) {
      let Diff::PartDiff {
         moved,
         moved_sum_count,
         added,
         removed,
         ..
      } = self
      else {
         return (Default::default(), Default::default());
      };

      let mut moves = Vec::with_capacity(*moved_sum_count);
      let mut adds = Vec::with_capacity(added.len());

      let mut removes_iter = removed.iter();
      let mut adds_iter = added.iter();
      let mut moves_iter = moved.iter();

      let mut removes_next = removes_iter.next();
      let mut adds_next = adds_iter.next();
      let mut moves_next = moves_iter.next().copied();

      for i in 0..moved_sum_count + added.len() + removed.len() {
         if let Some(at) = removes_next {
            if i == *at {
               removes_next = removes_iter.next();

               continue;
            }
         }

         match (adds_next, &mut moves_next) {
            (Some(add), Some(move_)) => {
               if *add == i {
                  adds.push(*add);

                  adds_next = adds_iter.next();
               } else {
                  let mut single_move = *move_;
                  single_move.len = 1;

                  moves.push(single_move);

                  move_.len -= 1;
                  move_.from += 1;
                  move_.to += 1;

                  if move_.len == 0 {
                     moves_next = moves_iter.next().copied();
                  }
               }
            }
            (Some(add), None) => {
               adds.push(*add);

               adds_next = adds_iter.next();
            }
            (None, Some(move_)) => {
               let mut single_move = *move_;
               single_move.len = 1;

               moves.push(single_move);

               move_.len -= 1;
               move_.from += 1;
               move_.to += 1;

               if move_.len == 0 {
                  moves_next = moves_iter.next().copied();
               }
            }
            (None, None) => break,
         }
      }

      (moves, adds)
   }
}

// fn apply_diff<V, R>(
//     mut ctx: ViewCtx<R>,
//     placeholder_id: &R::NodeId,
//     diff: Diff,
//     mut children_keys: Vec<Option<V::Key>>,
//     mut items: Vec<Option<V>>,
// ) -> Vec<Option<V::Key>>
//     where
//         V: View<R>,
//         R: Renderer,
// {
//     // The order of cmds needs to be:
//     // 1. Clear
//     // 2. Removals
//     // 3. Move out
//     // 4. Resize
//     // 5. Move in
//     // 6. Additions
//     // 7. Removes holes
//     if diff.clear {
//         for key in children_keys.drain(..).filter_map(|n| n) {
//             key.remove(&mut *ctx.world);
//         }
//         if diff.added.is_empty() {
//             return children_keys;
//         }
//     }
//
//     for DiffOpRemove { at } in &diff.removed {
//         let key = children_keys[*at].take().unwrap();
//         key.remove(&mut *ctx.world);
//     }
//
//     let (move_cmds, add_cmds) = unpack_moves(&diff);
//
//     let mut moved_children = move_cmds
//         .iter()
//         .map(|move_| children_keys[move_.from].take())
//         .collect::<Vec<_>>();
//
//     children_keys.resize_with(children_keys.len() + diff.added.len(), || None);
//
//     for (i, DiffOpMove { to, .. }) in move_cmds
//         .iter()
//         .enumerate()
//         .filter(|(_, move_)| !move_.move_in_dom)
//     {
//         children_keys[*to] = moved_children[i].take();
//     }
//
//     let get_next_some_node_id = |world: &R::World,
//                                  children_keys: &mut Vec<Option<V::Key>>,
//                                  to: usize|
//                                  -> Option<R::NodeId> {
//         Some(
//             children_keys[to..]
//                 .iter()
//                 .find_map(|n| n.as_ref().and_then(|key| key.first_node_id(world)))
//                 .unwrap_or_else(|| placeholder_id.clone()),
//         )
//     };
//     for (i, DiffOpMove { to, .. }) in move_cmds
//         .into_iter()
//         .enumerate()
//         .filter(|(_, move_)| move_.move_in_dom)
//     {
//         let moved_key = moved_children[i].take().unwrap();
//
//         let before_node_id = get_next_some_node_id(&mut *ctx.world, &mut children_keys, to);
//         let parent = ctx.parent.clone();
//         moved_key.insert_before(&mut ctx.world, Some(&parent), before_node_id.as_ref());
//         children_keys[to] = Some(moved_key);
//     }
//
//     for DiffOpAdd { at, mode } in add_cmds {
//         let view = items[at].take().unwrap();
//         let key = view.build(
//             ViewCtx {
//                 world: &mut *ctx.world,
//                 parent: ctx.parent.clone(),
//             },
//             None,
//             true,
//         );
//
//         let before_node_id = match mode {
//             DiffOpAddMode::Normal => get_next_some_node_id(&mut *ctx.world, &mut children_keys, at),
//             DiffOpAddMode::Append => Some(placeholder_id.clone()),
//         };
//         let parent = ctx.parent.clone();
//         key.insert_before(&mut *ctx.world, Some(&parent), before_node_id.as_ref());
//
//         children_keys[at] = Some(key);
//     }
//
//     #[allow(unstable_name_collisions)]
//     children_keys.drain_filter(|c| c.is_none());
//     children_keys
// }
