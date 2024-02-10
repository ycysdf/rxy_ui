use core::cmp::Ordering;
use std::borrow::Cow;

use bevy_core::Name;
use bevy_ecs::prelude::Entity;
use bevy_ecs::prelude::World;
use bevy_hierarchy::{BuildWorldChildren, DespawnRecursiveExt};
use bevy_hierarchy::{Children, Parent};
use bevy_render::prelude::Visibility;
use bevy_ui::prelude::NodeBundle;
use bevy_ui::Display;
use bevy_ui::Style;

use rxy_core::{
    AttrIndex, DeferredNodeTreeScoped, ElementAttrType, ElementType, NodeTree, RendererNodeId,
};

use crate::{
    BevyDeferredWorldScoped, BevyRenderer, BevyWorldExt, CmdSender, ElementEntityExtraData,
    ElementEntityWorldMutExt, ElementStyleEntityExt, RendererState,
};

impl NodeTree<BevyRenderer> for World {
    fn prepare_set_attr_and_get_is_init(
        &mut self,
        node_id: &RendererNodeId<BevyRenderer>,
        attr_index: AttrIndex,
    ) -> bool {
        let mut entity_mut = self.entity_mut(*node_id);
        let mut extra_data = entity_mut.get_mut::<ElementEntityExtraData>().unwrap();
        let is_init = extra_data.is_init_attr(attr_index);
        if !is_init {
            extra_data.init_attr(attr_index, true);
        }
        is_init
    }

    fn build_attr<A: ElementAttrType<BevyRenderer>>(
        &mut self,
        entity: RendererNodeId<BevyRenderer>,
        value: A::Value,
    ) {
        A::first_set_value(self, entity, value);
        let Some(mut entity_world_mut) = self.get_entity_mut(entity) else {
            return;
        };
        entity_world_mut
            .as_entity_mut()
            .get_element_extra_data_mut()
            .unwrap() // todo: error handle
            .set_attr(A::INDEX, true);
    }
    fn rebuild_attr<A: ElementAttrType<BevyRenderer>>(
        &mut self,
        entity: RendererNodeId<BevyRenderer>,
        value: A::Value,
    ) {
        A::update_value(self, entity, value);
        let Some(mut entity_world_mut) = self.get_entity_mut(entity) else {
            return;
        };
        entity_world_mut
            .as_entity_mut()
            .get_element_extra_data_mut()
            .unwrap() // todo: error handle
            .set_attr(A::INDEX, true);
    }

    fn unbuild_attr<A: ElementAttrType<BevyRenderer>>(&mut self, entity: RendererNodeId<BevyRenderer>) {
        A::set_value(self, entity, None::<A::Value>);
        let Some(mut entity_world_mut) = self.get_entity_mut(entity) else {
            return;
        };
        entity_world_mut
            .as_entity_mut()
            .get_element_extra_data_mut()
            .unwrap() // todo: error handle
            .set_attr(A::INDEX, false);
    }

    fn deferred_world_scoped(&mut self) -> impl DeferredNodeTreeScoped<BevyRenderer> {
        BevyDeferredWorldScoped {
            cmd_sender: self.resource::<CmdSender>().clone(),
        }
    }

    fn get_node_state_mut<S: Send + Sync + 'static>(
        &mut self,
        node_id: &RendererNodeId<BevyRenderer>,
    ) -> Option<&mut S> {
        self.get_mut::<RendererState<S>>(*node_id)
            .map(|n| &mut n.into_inner().0)
    }

    fn get_node_state_ref<S: Send + Sync + 'static>(
        &self,
        node_id: &RendererNodeId<BevyRenderer>,
    ) -> Option<&S> {
        self.get::<RendererState<S>>(*node_id).map(|n| &n.0)
    }

    fn take_node_state<S: Send + Sync + 'static>(
        &mut self,
        node_id: &RendererNodeId<BevyRenderer>,
    ) -> Option<S> {
        self.entity_mut(*node_id)
            .take::<RendererState<S>>()
            .map(|n| n.0)
    }

    fn set_node_state<S: Send + Sync + 'static>(
        &mut self,
        node_id: &RendererNodeId<BevyRenderer>,
        state: S,
    ) {
        self.entity_mut(*node_id).insert(RendererState(state));
    }

    fn exist_node_id(&mut self, node_id: &RendererNodeId<BevyRenderer>) -> bool {
        self.entities().contains(*node_id)
    }

    fn reserve_node_id(&mut self) -> RendererNodeId<BevyRenderer> {
        self.entities().reserve_entity()
    }

    fn spawn_placeholder(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        parent: Option<&RendererNodeId<BevyRenderer>>,
        reserve_node_id: Option<RendererNodeId<BevyRenderer>>,
    ) -> RendererNodeId<BevyRenderer> {
        let mut entity_mut = match reserve_node_id {
            None => self.spawn_empty(),
            Some(node_id) => self.get_or_spawn(node_id).unwrap(),
        };
        let entity = entity_mut.id();
        entity_mut.insert((
            NodeBundle {
                visibility: Visibility::Hidden,
                style: Style {
                    display: Display::None,
                    ..Default::default()
                },
                ..Default::default()
            },
            Name::new(format!("{} ({:?})", name.into(), entity)),
        ));
        if let Some(parent) = parent {
            entity_mut.set_parent(*parent);
        }
        entity_mut.id()
    }

    fn ensure_spawn(&mut self, reserve_node_id: RendererNodeId<BevyRenderer>) {
        self.get_or_spawn(reserve_node_id)
            .unwrap()
            .insert(Name::new("[TEMP DATA]"));
    }

    fn spawn_empty_node(
        &mut self,
        parent: Option<&RendererNodeId<BevyRenderer>>,
        reserve_node_id: Option<RendererNodeId<BevyRenderer>>,
    ) -> RendererNodeId<BevyRenderer> {
        self.get_or_spawn_empty(parent, reserve_node_id).id()
    }

    fn spawn_data_node(&mut self) -> RendererNodeId<BevyRenderer> {
        // spawn to container
        self.spawn((Name::new("[DATA]"),)).id()
    }

    fn spawn_node<E: ElementType<BevyRenderer>>(
        &mut self,
        parent: Option<&RendererNodeId<BevyRenderer>>,
        reserve_node_id: Option<RendererNodeId<BevyRenderer>>,
    ) -> RendererNodeId<BevyRenderer> {
        let node_id = E::spawn(self, parent, reserve_node_id);
        {
            let entity_extra_data = ElementEntityExtraData::new(E::get());
            self.entity_mut(node_id).insert(entity_extra_data);
        };
        node_id
    }

    fn get_parent(
        &self,
        node_id: &RendererNodeId<BevyRenderer>,
    ) -> Option<RendererNodeId<BevyRenderer>> {
        self.get::<Parent>(*node_id).map(|n| n.get())
    }

    #[inline]
    fn remove_node(&mut self, _node_id: &RendererNodeId<BevyRenderer>) {
        self.entity_mut(*_node_id).despawn_recursive();
    }

    #[inline]
    fn insert_before(
        &mut self,
        parent: Option<&RendererNodeId<BevyRenderer>>,
        before_node_id: Option<&RendererNodeId<BevyRenderer>>,
        inserted_node_ids: &[RendererNodeId<BevyRenderer>],
    ) {
        let parent = parent
            .cloned()
            .or_else(|| before_node_id.and_then(|n| self.get::<Parent>(*n).map(|n| n.get())));
        let Some(parent) = parent else {
            return;
        };
        if let Some(before_node_id) = before_node_id {
            let children: Vec<Entity> = self
                .get::<Children>(parent)
                .unwrap()
                .iter()
                .cloned()
                .collect();
            let entity_index = children.iter().position(|n| n == before_node_id).unwrap();
            let mut parent_ref = self.entity_mut(parent);
            let mut less_count = 0;
            for x in inserted_node_ids {
                if let Some(i) = children.iter().position(|n| n == x) {
                    match i.cmp(&entity_index) {
                        Ordering::Less => {
                            less_count += 1;
                            parent_ref.insert_children(
                                entity_index - less_count,
                                core::slice::from_ref(x),
                            );
                        }
                        Ordering::Equal => {}
                        Ordering::Greater => {
                            parent_ref.insert_children(entity_index, core::slice::from_ref(x));
                        }
                    }
                } else {
                    parent_ref.insert_children(entity_index, core::slice::from_ref(x));
                }
            }
        } else {
            self.entity_mut(parent).push_children(inserted_node_ids);
        }
    }

    fn set_visibility(&mut self, hidden: bool, node_id: &RendererNodeId<BevyRenderer>) {
        if let Some(mut visibility) = self.get_mut::<Visibility>(*node_id) {
            *visibility = if hidden {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            };
        }
    }

    fn get_visibility(&self, node_id: &RendererNodeId<BevyRenderer>) -> bool {
        self.get::<Visibility>(*node_id)
            .is_some_and(|n| *n == Visibility::Hidden)
    }
}
