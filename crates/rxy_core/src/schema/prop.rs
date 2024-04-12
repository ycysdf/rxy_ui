use alloc::boxed::Box;
use core::any::TypeId;

use crate::utils::HashMap;
use crate::utils::SyncCell;
use crate::{ConstIndex, MaybeSend, NodeTree, PropState, Renderer, RendererNodeId, RendererWorld};

pub struct TypeIdHashMapState<S: MaybeSend + 'static>(pub SyncCell<HashMap<TypeId, S>>);

pub struct SchemaPropCtx<'a, R: Renderer> {
   pub world: &'a mut RendererWorld<R>,
   pub state_node_id: RendererNodeId<R>,
   pub prop_type_id: TypeId,
}

impl<'a, R: Renderer> SchemaPropCtx<'a, R> {
   pub fn prop_state_mut<S: MaybeSend + 'static>(&mut self) -> Option<&mut S> {
      let state_node_id = self.state_node_id.clone();
      let prop_type_id = self.prop_type_id;
      self
         .world
         .get_node_state_mut::<TypeIdHashMapState<S>>(&state_node_id)
         .and_then(|s| s.0.get().get_mut(&prop_type_id))
   }
   pub fn take_prop_state<S: MaybeSend + 'static>(&mut self) -> Option<S> {
      let state_node_id = self.state_node_id.clone();
      let prop_type_id = self.prop_type_id;
      self
         .world
         .get_node_state_mut::<TypeIdHashMapState<S>>(&state_node_id)
         .and_then(|s| s.0.get().remove(&prop_type_id))
   }
   pub fn set_prop_state<S: MaybeSend + 'static>(&mut self, state: S) {
      let state_node_id = self.state_node_id.clone();
      let prop_type_id = self.prop_type_id;
      if let Some(map) = self
         .world
         .get_node_state_mut::<TypeIdHashMapState<S>>(&state_node_id)
      {
         map.0.get().insert(prop_type_id, state);
      } else {
         let mut map = HashMap::default();
         map.insert(prop_type_id, state);
         self
            .world
            .set_node_state(&state_node_id, TypeIdHashMapState(SyncCell::new(map)));
      }
   }
}

#[derive(Clone)]
pub struct SchemaPropValue<T>(Option<T>);

impl<T> SchemaPropValue<T> {
   pub fn new(value: T) -> Self {
      Self(Some(value))
   }
}

impl<R, T> SchemaProp<R> for SchemaPropValue<T>
where
   R: Renderer,
   T: MaybeSend + 'static,
{
   type Value = T;

   fn get_init_value(&mut self) -> Option<Self::Value> {
      Some(self.0.take().unwrap())
   }

   fn build(self, _ctx: SchemaPropCtx<R>, _state: &mut dyn PropState<R>, _will_rebuild: bool) {}

   fn rebuild(mut self, ctx: SchemaPropCtx<R>, state: &mut dyn PropState<R>) {
      state.apply(Box::new(self.0.take().unwrap()), ctx.world);
   }
}

pub trait IntoSchemaProp<R, T>
where
   R: Renderer,
   T: MaybeSend + 'static,
{
   // refactor variant
   type Prop: SchemaProp<R, Value = T>;
   fn into_schema_prop<const I: usize>(self) -> Self::Prop;
}

pub trait SchemaProp<R>: MaybeSend + 'static
where
   R: Renderer,
{
   type Value: MaybeSend + 'static;

   fn prop_type_id() -> Option<TypeId> {
      None
   }

   fn get_init_value(&mut self) -> Option<Self::Value>;
   fn build(self, ctx: SchemaPropCtx<R>, state: &mut dyn PropState<R>, will_rebuild: bool);
   fn rebuild(self, ctx: SchemaPropCtx<R>, state: &mut dyn PropState<R>);
}

impl<R, T> IntoSchemaProp<R, T> for SchemaPropValue<T>
where
   R: Renderer,
   T: MaybeSend + 'static,
{
   type Prop = Self;

   fn into_schema_prop<const I: usize>(self) -> Self::Prop {
      self
   }
}

impl<R, const I: usize, T> SchemaProp<R> for ConstIndex<I, T>
where
   R: Renderer,
   T: SchemaProp<R>,
{
   type Value = T::Value;

   fn prop_type_id() -> Option<TypeId> {
      Some(TypeId::of::<ConstIndex<I>>())
   }

   fn get_init_value(&mut self) -> Option<Self::Value> {
      self.0.get_init_value()
   }

   fn build(self, ctx: SchemaPropCtx<R>, state: &mut dyn PropState<R>, will_rebuild: bool) {
      self.0.build(ctx, state, will_rebuild);
   }

   fn rebuild(self, ctx: SchemaPropCtx<R>, state: &mut dyn PropState<R>) {
      self.0.rebuild(ctx, state);
   }
}

impl<R, T> SchemaProp<R> for Option<T>
where
   R: Renderer,
   T: SchemaProp<R>,
{
   type Value = T::Value;

   fn get_init_value(&mut self) -> Option<Self::Value> {
      self.as_mut().and_then(|v| v.get_init_value())
   }

   fn build(self, _ctx: SchemaPropCtx<R>, _state: &mut dyn PropState<R>, _will_rebuild: bool) {
      if let Some(v) = self {
         v.build(_ctx, _state, _will_rebuild);
      }
   }

   fn rebuild(mut self, ctx: SchemaPropCtx<R>, state: &mut dyn PropState<R>) {
      if let Some(v) = self.take() {
         v.rebuild(ctx, state);
      }
   }
}

impl<R, T, PV> IntoSchemaProp<R, PV> for Option<T>
where
   PV: MaybeSend + 'static,
   R: Renderer,
   T: IntoSchemaProp<R, PV>,
{
   type Prop = Option<T::Prop>;

   fn into_schema_prop<const I: usize>(self) -> Self::Prop {
      self.map(|v| v.into_schema_prop::<I>())
   }
}
