use core::any::Any;
use std::any::TypeId;

use bevy_ecs::entity::Entity;
use bevy_ecs::world::World;
use bevy_hierarchy::BuildWorldChildren;

use rxy_bevy_macro::BevyIntoView;
use rxy_core::style::{StyleSheetCtx, StyleSheets};
use rxy_core::{IntoView, RendererNodeId, View, ViewCtx};

use super::node_style_state::NodeStyleSheetsState;
use super::plugin::RxySharedStyleContainer;
use super::rxy_bevy_crate::SchemaCtx;
use super::rxy_bevy_crate::{BevyRenderer, RendererState};
use super::{SharedStyleState, StyleWorldExt};

pub trait SchemaCtxExt {
   fn default_typed_style<SS>(
      &mut self,
      typed_style: impl TypedStyleLabel,
      style_f: impl FnOnce() -> SS,
   ) -> (bool, Entity)
   where
      SS: StyleSheets<BevyRenderer>;
}

impl SchemaCtxExt for SchemaCtx {
   #[inline]
   fn default_typed_style<SS>(
      &mut self,
      typed_style: impl TypedStyleLabel,
      style_f: impl FnOnce() -> SS,
   ) -> (bool, Entity)
   where
      SS: StyleSheets<BevyRenderer>,
   {
      self.world_mut_scoped(|world| world.default_typed_style(typed_style, style_f))
   }
}

pub trait TypedStyleWorldExt {
   fn spawn_typed_style<SS>(
      &mut self,
      reserve_key: Option<RendererNodeId<BevyRenderer>>,
      type_id: TypeId,
      style_sheets: SS,
   ) -> Entity
   where
      SS: StyleSheets<BevyRenderer>;
   fn default_typed_style<SS>(
      &mut self,
      typed_style: impl TypedStyleLabel,
      style_f: impl FnOnce() -> SS,
   ) -> (bool, Entity)
   where
      SS: StyleSheets<BevyRenderer>;
}

impl TypedStyleWorldExt for World {
   fn spawn_typed_style<SS>(
      &mut self,
      reserve_key: Option<RendererNodeId<BevyRenderer>>,
      type_id: TypeId,
      style_sheets: SS,
   ) -> Entity
   where
      SS: StyleSheets<BevyRenderer>,
   {
      let shared_style_container = self.resource::<RxySharedStyleContainer>().0;
      let name = bevy_core::Name::new("[shared_style]");
      let node_id = match reserve_key {
         None => self.spawn(name).set_parent(shared_style_container).id(),
         Some(reserve_key) => self
            .get_or_spawn(reserve_key)
            .unwrap()
            .insert(name)
            .set_parent(shared_style_container)
            .id(),
      };

      let (style_sheets, _info) = style_sheets.style_sheets(StyleSheetCtx {
         inline_style_sheet_index: 0,
         shared_style_sheet_index: 0,
         world: self,
         node_id,
      });
      let style_sheets_state: NodeStyleSheetsState = style_sheets.collect();
      self.entity_mut(node_id).insert((
         RendererState(style_sheets_state),
         RendererState(SharedStyleState::default()),
      ));
      self.insert_typed_entity(type_id, node_id);
      node_id
   }

   fn default_typed_style<SS>(
      &mut self,
      typed_style: impl TypedStyleLabel,
      style_f: impl FnOnce() -> SS,
   ) -> (bool, Entity)
   where
      SS: StyleSheets<BevyRenderer>,
   {
      if let Some(entity) = self.get_typed_entity(typed_style.type_id()) {
         return (false, entity);
      }
      (
         true,
         self.spawn_typed_style(None, typed_style.type_id(), style_f()),
      )
   }
}

#[derive(BevyIntoView)]
pub struct TypedSharedStyleView<SS>
where
   SS: StyleSheets<BevyRenderer>,
{
   type_id: TypeId,
   style_sheets: SS,
}

impl<SS> View<BevyRenderer> for TypedSharedStyleView<SS>
where
   SS: StyleSheets<BevyRenderer>,
{
   type Key = ();

   fn build(
      self,
      ctx: ViewCtx<BevyRenderer>,
      _reserve_key: Option<Self::Key>,
      _will_rebuild: bool,
   ) -> Self::Key {
      ctx.world
         .spawn_typed_style(None, self.type_id, self.style_sheets);
   }

   fn rebuild(self, _ctx: ViewCtx<BevyRenderer>, _key: Self::Key) {
      todo!()
   }
}

pub trait DefaultStyleDef {
   fn def_default() -> impl IntoView<BevyRenderer>;
}

pub trait TypedStyleLabel: Copy + Clone + Send + 'static {
   fn def<SS>(style: SS) -> TypedSharedStyleView<SS>
   where
      SS: StyleSheets<BevyRenderer>,
   {
      TypedSharedStyleView {
         type_id: TypeId::of::<Self>(),
         style_sheets: style,
      }
   }
}

// pub fn typed_res_style<F, Res, SS>(style_sheets: F) -> TypedSharedStyleView<XRes<F, Res>>
// where
//     F: Fn(&Res) -> SS + Send + 'static,
//     SS: StyleSheets<BevyRenderer>,
//     Res: Resource + FromWorld,
// {
//     TypedSharedStyleView {
//         type_id: TypeId::of::<Res>(),
//         style_sheets: res(style_sheets),
//     }
// }
