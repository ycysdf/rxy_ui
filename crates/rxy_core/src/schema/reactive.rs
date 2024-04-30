use alloc::boxed::Box;
use core::any::Any;
use core::any::TypeId;
use core::marker::PhantomData;

use xy_reactive::prelude::ReadSignal;
use xy_reactive::prelude::{
   create_effect, use_memo, Memo, RwSignal, SignalGet, SignalGetUntracked, SignalSet, WriteSignal,
};

use crate::{
   scheme_state_scoped, BoxedPropValue, IntoSchemaProp, PropHashMap, PropState, Reactive,
   ReactiveDisposerState, SchemaProp, SchemaPropCtx,
};
use crate::{
   ConstIndex, InnerSchemaCtx, MaybeSend, MaybeSync, Renderer, Required, SchemaParam,
   SchemaParamDefault,
};
use crate::{DeferredNodeTreeScoped, NodeTree};

// impl<R, T> SchemaPropParam<R> for ReadSignal<T>
// where
//     R: Renderer,
//     T: MaybeSend /* + Default */ + 'static,
// {
//     type Value = T;
// }

impl<R, T> SchemaParam<R> for ReadSignal<T>
where
   R: Renderer,
   T: MaybeSend + MaybeSync + SchemaParamDefault<R> + 'static,
{
   fn from<const I: usize>(ctx: &mut InnerSchemaCtx<R>) -> Self {
      use xy_reactive::prelude::use_signal;
      let type_id = TypeId::of::<ConstIndex<I>>();

      let value: T = ctx
         .get_init_value::<T>(type_id)
         .unwrap_or_else(|| T::param_default(ctx));

      let (read_signal, write_signal) = use_signal(value);

      ctx.prop_state()
         .entry(type_id)
         .or_insert_with(|| Box::new(SignalPropState::new(write_signal)));

      read_signal
   }
}

impl<R, T> SchemaParam<R> for Required<ReadSignal<T>>
where
   R: Renderer,
   T: MaybeSend + MaybeSync + 'static,
{
   fn from<const I: usize>(ctx: &mut InnerSchemaCtx<R>) -> Self {
      use xy_reactive::prelude::use_signal;

      let type_id = TypeId::of::<ConstIndex<I>>();

      let value: T = ctx.get_init_value::<T>(type_id).unwrap();

      let (read_signal, write_signal) = use_signal(value);

      ctx.prop_state()
         .entry(type_id)
         .or_insert_with(|| Box::new(SignalPropState::new(write_signal)));

      Required(read_signal)
   }
}

fn schema_prop_build<R, S>(signal: S, mut ctx: SchemaPropCtx<R>)
where
   R: Renderer,
   S: SignalGet + MaybeSend + 'static,
   S::Value: Clone + MaybeSend + 'static,
{
   let state_node_id = ctx.state_node_id.clone();
   let prop_type_id = ctx.prop_type_id;
   let world_scoped = ctx.world.world_scoped();
   let effect = create_effect(move |_| {
      let value = signal.get();
      let state_node_id = state_node_id.clone();
      world_scoped.scoped(move |world| {
         if !world.exist_node_id(&state_node_id) {
            return;
         }
         let is_no_found = scheme_state_scoped(
            &mut *world,
            &state_node_id,
            |world, schema_state: &mut PropHashMap<R>| {
               let Some(state) = schema_state.get_mut(&prop_type_id) else {
                  // todo: tracing
                  // error!("prop not found by index!");
                  return;
               };
               state.apply(Box::new(value), &mut *world);
            },
         )
         .is_none();
         if is_no_found {
            // todo: tracing
            // error!("not found schema prop_map");
         }
      });
   });

   ctx.set_prop_state(ReactiveDisposerState(effect.erase()));
}

impl<R, T> SchemaProp<R> for ReadSignal<T>
where
   R: Renderer,
   T: Clone + MaybeSend + MaybeSync + 'static,
{
   type Value = T;

   fn get_init_value(&mut self) -> Option<Self::Value> {
      Some(self.get_untracked())
   }

   fn build(self, ctx: SchemaPropCtx<R>, _state: &mut dyn PropState<R>, _will_rebuild: bool) {
      schema_prop_build(self, ctx);
   }

   fn rebuild(self, mut ctx: SchemaPropCtx<R>, _state: &mut dyn PropState<R>) {
      drop(ctx.take_prop_state::<ReactiveDisposerState>());
      schema_prop_build(self, ctx);
   }
}

impl<R, T> SchemaProp<R> for Memo<T>
where
   R: Renderer,
   T: Clone + MaybeSend + MaybeSync + 'static,
{
   type Value = T;

   fn get_init_value(&mut self) -> Option<Self::Value> {
      Some(self.get_untracked())
   }

   fn build(self, ctx: SchemaPropCtx<R>, _state: &mut dyn PropState<R>, _will_rebuild: bool) {
      schema_prop_build(self, ctx);
   }

   fn rebuild(self, mut ctx: SchemaPropCtx<R>, _state: &mut dyn PropState<R>) {
      drop(ctx.take_prop_state::<ReactiveDisposerState>());
      schema_prop_build(self, ctx);
   }
}

// impl<R, T> SchemaProp<R> for Memo<T>
// where
//     R: Renderer,
//     T: SchemaProp<R> + MaybeSync,
// {
//     type Value = T::Value;

//     fn get_init_value(&mut self) -> Option<Self::Value> {
//         self.get_untracked().get_init_value()
//     }

//     fn build(self, ctx: SchemaPropCtx<R>, _state: &mut dyn PropState<R>, _will_rebuild: bool) {
//         schema_prop_build(self, ctx);
//     }

//     fn rebuild(self, mut ctx: SchemaPropCtx<R>, _state: &mut dyn PropState<R>) {
//         drop(ctx.take_prop_state::<ReactiveDisposerState>());
//         schema_prop_build(self, ctx);
//     }
// }

// impl<R, PV, T> IntoSchemaProp<R, PV> for ReadSignal<T>
// where
//     R: Renderer,
//     T: IntoSchemaProp<R, PV> + MaybeSend + MaybeSync + 'static,
//     PV: Clone + MaybeSend + MaybeSync + 'static,
// {
//     type Prop = Memo<T::Prop>;

//     fn into_schema_prop<const I: usize>(self) -> Self::Prop {
//         use_memo(move |_| self.get().into_schema_prop::<I>())
//     }
// }

impl<R, T> IntoSchemaProp<R, T> for ReadSignal<T>
where
   R: Renderer,
   T: Clone + MaybeSend + MaybeSync + 'static,
{
   type Prop = Self;

   fn into_schema_prop<const I: usize>(self) -> Self::Prop {
      self
   }
}

impl<R, T> IntoSchemaProp<R, T> for Memo<T>
where
   R: Renderer,
   T: Clone + MaybeSend + MaybeSync + 'static,
{
   type Prop = Self;

   fn into_schema_prop<const I: usize>(self) -> Self::Prop {
      self
   }
}

impl<R, T> IntoSchemaProp<R, T> for RwSignal<T>
where
   R: Renderer,
   T: Clone + MaybeSend + MaybeSync + 'static,
{
   type Prop = ReadSignal<T>;

   fn into_schema_prop<const I: usize>(self) -> Self::Prop {
      self.read_only()
   }
}

impl<R, F, T> IntoSchemaProp<R, T> for Reactive<F, T>
where
   R: Renderer,
   F: Fn() -> T + MaybeSend + MaybeSync + 'static,
   T: Clone + MaybeSend + MaybeSync + PartialEq + 'static,
{
   type Prop = Memo<T>;

   fn into_schema_prop<const I: usize>(self) -> Self::Prop {
      use_memo(move |_| self.0())
   }
}

pub struct SignalPropState<R, T>
where
   R: Renderer,
   T: MaybeSend + MaybeSync + 'static,
{
   write_signal: WriteSignal<T>,
   _marker: PhantomData<R>,
}

impl<R, T> SignalPropState<R, T>
where
   R: Renderer,
   T: MaybeSend + MaybeSync + 'static,
{
   pub fn new(write_signal: WriteSignal<T>) -> Self {
      Self {
         write_signal,
         _marker: Default::default(),
      }
   }
}

impl<R, T> PropState<R> for SignalPropState<R, T>
where
   R: Renderer,
   T: MaybeSend + MaybeSync + 'static,
{
   fn apply(&mut self, new_value: BoxedPropValue, _world: &mut R::NodeTree) {
      let Ok(new_value) = new_value.downcast::<T>().map(|n| *n) else {
         return;
      };
      self.write_signal.set(new_value);
   }

   fn as_any_mut(&mut self) -> &mut (dyn Any + MaybeSend) {
      self
   }
}
