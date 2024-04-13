use core::{
    hash::{BuildHasherDefault, Hash},
    marker::PhantomData,
};

use alloc::vec;
use alloc::vec::Vec;
use drain_filter_polyfill::VecExt as VecDrainFilterExt;
use indexmap::IndexSet;

use crate::{
    IntoView, MutableView, MutableViewKey, Renderer, RendererNodeId, RendererViewExt,
    RendererWorld, View, ViewCtx, ViewKey, virtual_container, VirtualContainer,
};
use crate::utils::AHasher;

type FxIndexSet<T> = IndexSet<T, BuildHasherDefault<AHasher>>;

pub fn for_keyed<T, I, K, KF, VF, V, R>(
    items: I,
    key_fn: KF,
    view_fn: VF,
) -> Keyed<T, I, K, KF, VF, V, R>
    where
        I: IntoIterator<Item=T>,
        K: Eq + Hash + 'static,
        KF: Fn(&T) -> K,
        V: IntoView<R>,
        VF: Fn(T) -> V,
        R: Renderer,
{
    Keyed {
        items,
        key_fn,
        view_fn,
        _marker: PhantomData,
    }
}

pub struct Keyed<T, I, K, KF, VF, IV, R>
    where
        I: IntoIterator<Item=T>,
        K: Eq + Hash + 'static,
        KF: Fn(&T) -> K,
        IV: IntoView<R>,
        VF: Fn(T) -> IV,
        R: Renderer,
{
    items: I,
    key_fn: KF,
    view_fn: VF,
    _marker: PhantomData<R>,
}

#[derive(Clone)]
pub struct KeyedState<DK> {
    data_keys: FxIndexSet<DK>,
}

impl<R: Renderer, K: ViewKey<R>> MutableViewKey<R> for Vec<Option<K>> {
    fn remove(self, world: &mut RendererWorld<R>, _state_node_id: &RendererNodeId<R>) {
        for key in self.iter().filter_map(|n| n.as_ref()) {
            key.remove(world);
        }
    }

    fn insert_before(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
        _state_node_id: &RendererNodeId<R>,
    ) {
        for key in self.iter().filter_map(|n| n.as_ref()) {
            key.insert_before(world, parent, before_node_id);
        }
    }

    fn set_visibility(
        &self,
        world: &mut RendererWorld<R>,
        hidden: bool,
        _state_node_id: &RendererNodeId<R>,
    ) {
        for key in self.iter().filter_map(|n| n.as_ref()) {
            key.set_visibility(world, hidden);
        }
    }

    fn first_node_id(
        &self,
        world: &RendererWorld<R>,
        _state_node_id: &RendererNodeId<R>,
    ) -> Option<RendererNodeId<R>> {
        self.iter()
            .filter_map(|n| n.as_ref())
            .find_map(|key| key.first_node_id(world))
    }
}

impl<T, I, K, KF, VF, IV, R> MutableView<R> for Keyed<T, I, K, KF, VF, IV, R>
    where
        T: 'static,
        I: IntoIterator<Item=T> + MaybeSend + 'static,
        K: Eq + Hash + MaybeSend + MaybeSync + 'static,
        KF: Fn(&T) -> K + MaybeSend + 'static,
        IV: IntoView<R> + MaybeSend + 'static,
        VF: Fn(T) -> IV + MaybeSend + 'static,
        R: Renderer,
{
    type Key = Vec<Option<<IV::View as View<R>>::Key>>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        will_rebuild: bool,
        state_node_id: RendererNodeId<R>,
    ) -> Self::Key {
        let items = self.items.into_iter();
        let (capacity, _) = items.size_hint();
        let mut data_keys = if will_rebuild {
            FxIndexSet::with_capacity_and_hasher(capacity, Default::default())
        } else {
            Default::default()
        };
        let mut view_keys = Vec::with_capacity(capacity);
        for item in items {
            if will_rebuild {
                data_keys.insert((self.key_fn)(&item));
            }
            let view = (self.view_fn)(item);
            view_keys.push(Some(view.into_view().build(
                ViewCtx {
                    world: &mut *ctx.world,
                    parent: ctx.parent.clone(),
                },
                None,
                will_rebuild,
            )));
        }
        if will_rebuild {
            R::set_view_state(ctx.world, &state_node_id, KeyedState { data_keys });
        }
        view_keys
    }

    fn rebuild(
        self,
        ctx: ViewCtx<R>,
        key: Self::Key,
        state_node_id: RendererNodeId<R>,
    ) -> Option<Self::Key> {
        let new_items = self.items.into_iter();
        let (capacity, _) = new_items.size_hint();
        let mut new_data_keys = FxIndexSet::with_capacity_and_hasher(capacity, Default::default());

        let mut items = Vec::new();
        for item in new_items {
            new_data_keys.insert((self.key_fn)(&item));
            items.push(Some(item));
        }
        let cmds = {
            let Some(state) = R::get_view_state_ref::<KeyedState<K>>(&ctx.world, &state_node_id)
                else {
                    panic!("no found keyd state!")
                };

            diff(&state.data_keys, &new_data_keys)
        };

        let new_view_keys = apply_diff(
            ViewCtx {
                world: &mut *ctx.world,
                parent: ctx.parent,
            },
            &state_node_id,
            cmds,
            key,
            &self.view_fn,
            items,
        );

        R::set_view_state(
            ctx.world,
            &state_node_id,
            KeyedState {
                data_keys: new_data_keys,
            },
        );
        Some(new_view_keys)
    }
}

impl<T, I, K, KF, VF, IV, R> IntoView<R> for Keyed<T, I, K, KF, VF, IV, R>
    where
        T: 'static,
        I: IntoIterator<Item=T> + MaybeSend + 'static,
        K: Eq + Hash + MaybeSend + MaybeSync + 'static,
        KF: Fn(&T) -> K + MaybeSend + 'static,
        IV: IntoView<R> + MaybeSend + 'static,
        VF: Fn(T) -> IV + MaybeSend + 'static,
        R: Renderer,
{
    type View = VirtualContainer<R, Self>;

    fn into_view(self) -> Self::View {
        virtual_container(self)
    }
}

fn diff<K: Eq + Hash>(from: &FxIndexSet<K>, to: &FxIndexSet<K>) -> Diff {
    if from.is_empty() && to.is_empty() {
        return Diff::default();
    } else if to.is_empty() {
        return Diff {
            clear: true,
            ..Default::default()
        };
    } else if from.is_empty() {
        return Diff {
            added: to
                .iter()
                .enumerate()
                .map(|(at, _)| DiffOpAdd {
                    at,
                    mode: DiffOpAddMode::Append,
                })
                .collect(),
            ..Default::default()
        };
    }

    let mut removed = vec![];
    let mut moved = vec![];
    let mut added = vec![];
    let max_len = core::cmp::max(from.len(), to.len());

    for index in 0..max_len {
        let from_item = from.get_index(index);
        let to_item = to.get_index(index);

        // if they're the same, do nothing
        if from_item != to_item {
            // if it's only in old, not new, remove it
            if from_item.is_some() && !to.contains(from_item.unwrap()) {
                let op = DiffOpRemove { at: index };
                removed.push(op);
            }
            // if it's only in new, not old, add it
            if to_item.is_some() && !from.contains(to_item.unwrap()) {
                let op = DiffOpAdd {
                    at: index,
                    mode: DiffOpAddMode::Normal,
                };
                added.push(op);
            }
            // if it's in both old and new, it can either
            // 1) be moved (and need to move in the DOM)
            // 2) be moved (but not need to move in the DOM)
            //    * this would happen if, for example, 2 items
            //      have been added before it, and it has moved by 2
            if let Some(from_item) = from_item {
                if let Some(to_item) = to.get_full(from_item) {
                    let moves_forward_by = (to_item.0 as i32) - (index as i32);
                    let move_in_dom =
                        moves_forward_by != (added.len() as i32) - (removed.len() as i32);

                    let op = DiffOpMove {
                        from: index,
                        len: 1,
                        to: to_item.0,
                        move_in_dom,
                    };
                    moved.push(op);
                }
            }
        }
    }

    moved = group_adjacent_moves(moved);

    Diff {
        removed,
        items_to_move: moved.iter().map(|m| m.len).sum(),
        moved,
        added,
        clear: false,
    }
}

/// Group adjacent items that are being moved as a group.
/// For example from `[2, 3, 5, 6]` to `[1, 2, 3, 4, 5, 6]` should result
/// in a move for `2,3` and `5,6` rather than 4 individual moves.
fn group_adjacent_moves(moved: Vec<DiffOpMove>) -> Vec<DiffOpMove> {
    let mut prev: Option<DiffOpMove> = None;
    let mut new_moved = Vec::with_capacity(moved.len());
    for m in moved {
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
    new_moved
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Diff {
    removed: Vec<DiffOpRemove>,
    moved: Vec<DiffOpMove>,
    items_to_move: usize,
    added: Vec<DiffOpAdd>,
    clear: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DiffOpMove {
    /// The index this range is starting relative to `from`.
    from: usize,
    /// The number of elements included in this range.
    len: usize,
    /// The starting index this range will be moved to relative to `to`.
    to: usize,
    /// Marks this move to be applied to the DOM, or just to the underlying
    /// storage
    move_in_dom: bool,
}

impl Default for DiffOpMove {
    fn default() -> Self {
        Self {
            from: 0,
            to: 0,
            len: 1,
            move_in_dom: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct DiffOpAdd {
    at: usize,
    mode: DiffOpAddMode,
}

#[derive(Debug, PartialEq, Eq)]
struct DiffOpRemove {
    at: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DiffOpAddMode {
    Normal,
    Append,
}

impl Default for DiffOpAddMode {
    fn default() -> Self {
        Self::Normal
    }
}

fn apply_diff<T, IV, R>(
    mut ctx: ViewCtx<R>,
    placeholder_id: &R::NodeId,
    diff: Diff,
    mut children_keys: Vec<Option<<IV::View as View<R>>::Key>>,
    view_fn: impl Fn(T) -> IV,
    mut items: Vec<Option<T>>,
) -> Vec<Option<<IV::View as View<R>>::Key>>
    where
        IV: IntoView<R>,
        R: Renderer,
{
    // The order of cmds needs to be:
    // 1. Clear
    // 2. Removals
    // 3. Move out
    // 4. Resize
    // 5. Move in
    // 6. Additions
    // 7. Removes holes
    if diff.clear {
        for key in children_keys.drain(..).filter_map(|n| n) {
            key.remove(&mut *ctx.world);
        }
        if diff.added.is_empty() {
            return children_keys;
        }
    }

    for DiffOpRemove { at } in &diff.removed {
        let key = children_keys[*at].take().unwrap();
        key.remove(&mut *ctx.world);
    }

    let (move_cmds, add_cmds) = unpack_moves(&diff);

    let mut moved_children = move_cmds
        .iter()
        .map(|move_| children_keys[move_.from].take())
        .collect::<Vec<_>>();

    children_keys.resize_with(children_keys.len() + diff.added.len(), || None);

    for (i, DiffOpMove { to, .. }) in move_cmds
        .iter()
        .enumerate()
        .filter(|(_, move_)| !move_.move_in_dom)
    {
        children_keys[*to] = moved_children[i].take();
    }

    let get_next_some_node_id = |world: &R::World,
                                 children_keys: &mut Vec<Option<<IV::View as View<R>>::Key>>,
                                 to: usize|
                                 -> Option<R::NodeId> {
        Some(
            children_keys[to..]
                .iter()
                .find_map(|n| n.as_ref().and_then(|key| key.first_node_id(world)))
                .unwrap_or_else(|| placeholder_id.clone()),
        )
    };
    for (i, DiffOpMove { to, .. }) in move_cmds
        .into_iter()
        .enumerate()
        .filter(|(_, move_)| move_.move_in_dom)
    {
        let moved_key = moved_children[i].take().unwrap();

        let before_node_id = get_next_some_node_id(&mut *ctx.world, &mut children_keys, to);
        let parent = ctx.parent.clone();
        moved_key.insert_before(&mut ctx.world, Some(&parent), before_node_id.as_ref());

        children_keys[to] = Some(moved_key);
    }

    for DiffOpAdd { at, mode } in add_cmds {
        let item = items[at].take().unwrap();
        let item = view_fn(item);
        let key = item.into_view().build(
            ViewCtx {
                world: &mut *ctx.world,
                parent: ctx.parent.clone(),
            },
            None,
            true,
        );

        let before_node_id = match mode {
            DiffOpAddMode::Normal => get_next_some_node_id(&mut *ctx.world, &mut children_keys, at),
            DiffOpAddMode::Append => None,
        };
        let parent = ctx.parent.clone();
        key.insert_before(&mut *ctx.world, Some(&parent), before_node_id.as_ref());

        children_keys[at] = Some(key);
    }

    #[allow(unstable_name_collisions)]
    children_keys.drain_filter(|c| c.is_none());
    children_keys
}

fn unpack_moves(diff: &Diff) -> (Vec<DiffOpMove>, Vec<DiffOpAdd>) {
    let mut moves = Vec::with_capacity(diff.items_to_move);
    let mut adds = Vec::with_capacity(diff.added.len());

    let mut removes_iter = diff.removed.iter();
    let mut adds_iter = diff.added.iter();
    let mut moves_iter = diff.moved.iter();

    let mut removes_next = removes_iter.next();
    let mut adds_next = adds_iter.next();
    let mut moves_next = moves_iter.next().copied();

    for i in 0..diff.items_to_move + diff.added.len() + diff.removed.len() {
        if let Some(DiffOpRemove { at, .. }) = removes_next {
            if i == *at {
                removes_next = removes_iter.next();

                continue;
            }
        }

        match (adds_next, &mut moves_next) {
            (Some(add), Some(move_)) => {
                if add.at == i {
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
