use bevy_ecs::entity::{Entity, EntityHashMap};
use bevy_ecs::prelude::Resource;
use bevy_hierarchy::Children;
use glam::Vec2;
use taffy::{AvailableSpace, print_tree, Size, TaffyTree};
use thiserror::Error;
use tracing::warn;

#[cfg(any(feature = "flexbox", feature = "grid"))]
pub use alignment::*;
#[cfg(feature = "flexbox")]
pub use flex::*;
pub use geometry::*;
#[cfg(feature = "grid")]
pub use grid::*;
pub use style::*;
pub use text::*;

#[cfg(any(feature = "flexbox", feature = "grid"))]
mod alignment;
pub mod convert;
#[cfg(feature = "flexbox")]
mod flex;
mod geometry;
#[cfg(feature = "grid")]
mod grid;
mod style;
mod text;

#[derive(Debug, Error)]
pub enum LayoutError {
   #[error("Invalid hierarchy")]
   InvalidHierarchy,
   #[error("Taffy error: {0}")]
   TaffyError(#[from] taffy::tree::TaffyError),
}

pub struct LayoutContext {
   pub scale_factor: f32,
   pub physical_size: Vec2,
   pub min_size: f32,
   pub max_size: f32,
}

impl LayoutContext {
   pub fn new(scale_factor: f32, physical_size: Vec2) -> Self {
      Self {
         scale_factor,
         physical_size,
         min_size: physical_size.x.min(physical_size.y) as _,
         max_size: physical_size.x.max(physical_size.y) as _,
      }
   }
}

#[derive(Resource)]
pub struct UiLayoutTree {
   pub entity_to_taffy: EntityHashMap<taffy::NodeId>,
   pub taffy_tree: TaffyTree,
}

impl UiLayoutTree {
   pub fn new() -> Self {
      Self {
         entity_to_taffy: Default::default(),
         taffy_tree: Default::default(),
      }
   }

   pub fn print_tree(&self, entity: Entity) {
      let Some(node_id) = self.entity_to_taffy.get(&entity) else {
         return;
      };
      print_tree(&self.taffy_tree, *node_id);
   }

   pub fn upsert_node(&mut self, entity: Entity, style: &Style, context: &LayoutContext) {
      let mut added = false;
      let taffy_tree = &mut self.taffy_tree;
      let taffy_node = self.entity_to_taffy.entry(entity).or_insert_with(|| {
         added = true;
         taffy_tree
            .new_leaf(convert::from_style(context, style))
            .unwrap()
      });

      if !added {
         self
            .taffy_tree
            .set_style(*taffy_node, convert::from_style(context, style))
            .unwrap();
      }
   }

   pub fn update_children(&mut self, entity: Entity, children: &Children) {
      let mut taffy_children = Vec::with_capacity(children.len());
      for child in children {
         if let Some(taffy_node) = self.entity_to_taffy.get(child) {
            taffy_children.push(*taffy_node);
         } else {
            warn!(
               "Unstyled child in a UI entity hierarchy. You are using an entity \
without UI components as a child of an entity with UI components, results may be unexpected."
            );
         }
      }

      let taffy_node = self.entity_to_taffy.get(&entity).unwrap();
      self
         .taffy_tree
         .set_children(*taffy_node, &taffy_children)
         .unwrap();
   }

   /// Removes children from the entity's taffy node if it exists. Does nothing otherwise.
   pub fn try_remove_children(&mut self, entity: Entity) {
      if let Some(taffy_node) = self.entity_to_taffy.get(&entity) {
         self.taffy_tree.set_children(*taffy_node, &[]).unwrap();
      }
   }

   // /// Removes the measure from the entity's taffy node if it exists. Does nothing otherwise.
   // pub fn try_remove_measure(&mut self, entity: Entity) {
   //    if let Some(taffy_node) = self.entity_to_taffy.get(&entity) {
   //       self.taffy_tree.set_measure(*taffy_node, None).unwrap();
   //    }
   // }

   pub fn compute_layout(
      &mut self,
      entity: Entity,
      available_space: impl Into<Size<AvailableSpace>>,
   ) -> Result<(), LayoutError> {
      let Some(node_id) = self.entity_to_taffy.get(&entity) else {
         return Ok(());
      };
      Ok(self
         .taffy_tree
         .compute_layout(*node_id, available_space.into())?)
   }

   /// Removes each entity from the internal map and then removes their associated node from taffy
   pub fn remove_entities(&mut self, entities: impl IntoIterator<Item = Entity>) {
      for entity in entities {
         if let Some(node) = self.entity_to_taffy.remove(&entity) {
            self.taffy_tree.remove(node).unwrap();
         }
      }
   }

   /// Get the layout geometry for the taffy node corresponding to the ui node [`Entity`].
   /// Does not compute the layout geometry, `compute_window_layouts` should be run before using this function.
   pub fn get_layout(&self, entity: Entity) -> Result<&taffy::tree::Layout, LayoutError> {
      if let Some(taffy_node) = self.entity_to_taffy.get(&entity) {
         self
            .taffy_tree
            .layout(*taffy_node)
            .map_err(LayoutError::TaffyError)
      } else {
         warn!(
            "Styled child in a non-UI entity hierarchy. You are using an entity \
with UI components as a child of an entity without UI components, results may be unexpected."
         );
         Err(LayoutError::InvalidHierarchy)
      }
   }
}
