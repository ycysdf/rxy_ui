use alloc::vec::Vec;
use core::fmt::Debug;
use core::{
   hash::{BuildHasherDefault, Hash},
   marker::PhantomData,
};

use drain_filter_polyfill::VecExt as VecDrainFilterExt;
use indexmap::IndexSet;

use crate::diff::{diff, Diff, DiffOpMove};
use crate::utils::AHasher;
use crate::{
   virtual_container, IntoView, MaybeSend, MaybeSync, MutableView, MutableViewKey, NodeTree,
   Renderer, RendererNodeId, RendererWorld, View, ViewCtx, ViewKey, VirtualContainer,
};

type FxIndexSet<T> = IndexSet<T, BuildHasherDefault<AHasher>>;

pub struct Keyed<K, V>(pub K, pub V);

impl<K, V> Keyed<K, V> {
   pub fn map<MV>(self, f: impl FnOnce(V) -> MV) -> Keyed<K, MV> {
      Keyed(self.0, f(self.1))
   }
}

// #[cfg_attr(feature = "dyn", force_into_dynamic_view)]
pub fn x_iter_keyed<I, K, IV, R>(items: I) -> ForKeyed<I, K, IV, R>
where
   I: IntoIterator<Item = Keyed<K, IV>>,
   K: Eq + Debug + Hash + MaybeSend + MaybeSync + 'static,
   IV: IntoView<R> + MaybeSend + 'static,
   R: Renderer,
{
//   let items = items.into_iter().collect::<Vec<_>>();
   ForKeyed {
      items,
      _marker: PhantomData,
   }
}

// #[cfg_attr(feature = "dyn", force_into_dynamic_view)]
pub fn x_iter<IV, R>(
   items: impl IntoIterator<Item = IV>,
) -> ForKeyed<Vec<Keyed<usize, IV>>, usize, IV, R>
where
   IV: IntoView<R> + MaybeSend + 'static,
   R: Renderer,
{
   ForKeyed {
      items: items
         .into_iter()
         .enumerate()
         .map(|(i, n)| Keyed(i, n))
         .collect::<Vec<_>>(),
      _marker: PhantomData,
   }
}

pub struct ForKeyed<I, K, IV, R>
where
   I: IntoIterator<Item = Keyed<K, IV>>,
   K: Eq + Debug + Hash + 'static,
   IV: IntoView<R>,
   R: Renderer,
{
   items: I,
   _marker: PhantomData<R>,
}

#[derive(Clone)]
pub struct ForKeyedState<DK> {
   data_keys: FxIndexSet<DK>,
}

impl<R: Renderer, K: ViewKey<R>> MutableViewKey<R> for Vec<Option<K>> {
   fn remove(self, world: &mut RendererWorld<R>) {
      for key in self.into_iter().flatten() {
         key.remove(world);
      }
   }

   fn insert_before(
      &self,
      world: &mut RendererWorld<R>,
      parent: Option<&RendererNodeId<R>>,
      before_node_id: Option<&RendererNodeId<R>>,
   ) {
      for key in self.iter().filter_map(|n| n.as_ref()) {
         key.insert_before(world, parent, before_node_id);
      }
   }

   fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool) {
      for key in self.iter().filter_map(|n| n.as_ref()) {
         key.set_visibility(world, hidden);
      }
   }

   fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
      self
         .iter()
         .filter_map(|n| n.as_ref())
         .find_map(|key| key.first_node_id(world))
   }

   fn state_node_id(&self) -> Option<RendererNodeId<R>> {
      self.first()?.state_node_id()
   }
}

impl<I, K, IV, R> MutableView<R> for ForKeyed<I, K, IV, R>
where
   I: IntoIterator<Item = Keyed<K, IV>> + MaybeSend + 'static,
   K: Eq + Debug + Hash + MaybeSend + MaybeSync + 'static,
   IV: IntoView<R>,
   R: Renderer,
{
   type Key = Vec<Option<<IV::View as View<R>>::Key>>;

   fn no_placeholder_when_no_rebuild() -> bool {
      true
   }

   fn build(self, ctx: ViewCtx<R>, placeholder_node_id: Option<RendererNodeId<R>>) -> Self::Key {
      let items = self.items.into_iter();
      let will_rebuild = placeholder_node_id.is_some();
      let (capacity, _) = items.size_hint();
      let mut data_keys = if will_rebuild {
         FxIndexSet::with_capacity_and_hasher(capacity, Default::default())
      } else {
         Default::default()
      };
      let mut view_keys = Vec::with_capacity(capacity);
      for Keyed(key, view) in items {
         if will_rebuild {
            data_keys.insert(key);
         }
         view_keys.push(Some(view.into_view().build(
            ViewCtx {
               world: &mut *ctx.world,
               parent: ctx.parent.clone(),
            },
            None,
            will_rebuild,
         )));
      }
      if let Some(state_node_id) = placeholder_node_id {
         ctx.world
            .set_node_state(&state_node_id, ForKeyedState { data_keys });
      }
      view_keys
   }

   fn rebuild(
      self,
      ctx: ViewCtx<R>,
      key: Self::Key,
      placeholder_node_id: RendererNodeId<R>,
   ) -> Option<Self::Key> {
      // return None;
      let new_items = self.items.into_iter();

      let (capacity, _) = new_items.size_hint();
      let mut new_data_keys = FxIndexSet::with_capacity_and_hasher(capacity, Default::default());

      let mut views = Vec::new();
      for Keyed(key, view) in new_items {
         new_data_keys.insert(key);
         views.push(Some(view.into_view()));
      }
      let cmds = {
         let Some(state) = ctx
            .world
            .get_node_state_ref::<ForKeyedState<K>>(&placeholder_node_id)
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
         &placeholder_node_id,
         cmds,
         views,
         key,
      );

      ctx.world.set_node_state(
         &placeholder_node_id,
         ForKeyedState {
            data_keys: new_data_keys,
         },
      );
      Some(new_view_keys)
   }
}

impl<I, K, IV, R> IntoView<R> for ForKeyed<I, K, IV, R>
where
   I: IntoIterator<Item = Keyed<K, IV>> + MaybeSend + 'static,
   K: Eq + Debug + Hash + MaybeSend + MaybeSync + 'static,
   IV: IntoView<R> + MaybeSend + 'static,
   R: Renderer,
{
   type View = VirtualContainer<R, Self>;

   fn into_view(self) -> Self::View {
      virtual_container(self, "[ForKeyed Placeholder]")
   }
}

fn apply_diff<V, R>(
   ctx: ViewCtx<R>,
   placeholder_id: &R::NodeId,
   diff: Diff,
   mut views: Vec<Option<V>>,
   mut children_keys: Vec<Option<V::Key>>,
) -> Vec<Option<V::Key>>
where
   V: View<R>,
   R: Renderer,
{
   match diff {
      Diff::NoChanged => children_keys,
      Diff::Cleared => {
         for key in children_keys.into_iter().flatten() {
            key.remove(&mut *ctx.world);
         }
         Default::default()
      }
      Diff::Replaced => views
         .into_iter()
         .filter_map(|n| {
            n.map(|v| {
               Some(v.build(
                  ViewCtx {
                     world: &mut *ctx.world,
                     parent: ctx.parent.clone(),
                  },
                  None,
                  true,
               ))
            })
         })
         .collect(),
      Diff::PartDiff {
         moved,
         removed,
         added,
         // moved_sum_count,
         no_changed,
         ..
      } => {
         for at in no_changed {
            views[at].take().unwrap().rebuild(
               ViewCtx {
                  world: &mut *ctx.world,
                  parent: ctx.parent.clone(),
               },
               children_keys[at].clone().unwrap(),
            );
         }

         for at in &removed {
            let key = children_keys[*at].take().unwrap();
            key.remove(&mut *ctx.world);
         }

         let mut moved_children = moved
            .iter()
            .map(|move_| children_keys[move_.from].take())
            .collect::<Vec<_>>();

         children_keys.resize_with(children_keys.len() + added.len(), || None);

         for (i, DiffOpMove { to, .. }) in moved
            .iter()
            .enumerate()
            .filter(|(_, move_)| move_.can_ignored)
         {
            children_keys[*to] = moved_children[i].take();
         }

         let get_next_some_node_id = |world: &R::NodeTree,
                                      children_keys: &mut Vec<Option<V::Key>>,
                                      to: usize|
          -> Option<R::NodeId> {
            Some(
               children_keys[to..]
                  .iter()
                  .find_map(|n| n.as_ref().and_then(|key| key.first_node_id(world)))
                  .unwrap_or_else(|| placeholder_id.clone()),
            )
         };
         for (i, DiffOpMove { to, .. }) in moved
            .into_iter()
            .enumerate()
            .filter(|(_, move_)| !move_.can_ignored)
         {
            let moved_key = moved_children[i].take().unwrap();

            let before_node_id = get_next_some_node_id(&mut *ctx.world, &mut children_keys, to);
            let parent = ctx.parent.clone();
            moved_key.insert_before(ctx.world, Some(&parent), before_node_id.as_ref());
            children_keys[to] = Some(moved_key.clone());

            views[to].take().unwrap().rebuild(
               ViewCtx {
                  world: &mut *ctx.world,
                  parent: ctx.parent.clone(),
               },
               moved_key,
            );
         }

         for at in added {
            let view = views[at].take().unwrap();
            let key = view.build(
               ViewCtx {
                  world: &mut *ctx.world,
                  parent: ctx.parent.clone(),
               },
               None,
               true,
            );

            let before_node_id = get_next_some_node_id(&mut *ctx.world, &mut children_keys, at);
            let parent = ctx.parent.clone();
            key.insert_before(&mut *ctx.world, Some(&parent), before_node_id.as_ref());

            children_keys[at] = Some(key);
         }

         #[allow(unstable_name_collisions)]
         children_keys.drain_filter(|c| c.is_none());
         children_keys
      }
   }
}
