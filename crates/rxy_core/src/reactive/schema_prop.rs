use crate::DeferredWorldScoped;
use crate::{
    scheme_state_scoped, BoxedPropValue, IntoSchemaProp, PropHashMap, PropState, Reactive,
    ReactiveDisposerState, Renderer, SchemaProp, SchemaPropCtx,
};
use alloc::boxed::Box;
use bevy_utils::tracing::error;
use core::any::Any;
use core::marker::PhantomData;
use xy_reactive::prelude::{
    create_effect, use_memo, Memo, ReadSignal, RwSignal, SignalGet, SignalGetUntracked, SignalSet,
    WriteSignal,
};

fn schema_prop_build<R, S>(signal: S, mut ctx: SchemaPropCtx<R>)
where
    R: Renderer,
    S: SignalGet + Send + 'static,
    S::Value: Clone + Send + 'static,
{
    let state_node_id = ctx.state_node_id.clone();
    let prop_type_id = ctx.prop_type_id;
    let world_scoped = R::deferred_world_scoped(&mut *ctx.world);
    let effect = create_effect(move |_| {
        let value = signal.get();
        let state_node_id = state_node_id.clone();
        world_scoped.scoped(move |world| {
            if !R::exist_node_id(world, &state_node_id) {
                return;
            }
            let is_no_found = scheme_state_scoped(
                &mut *world,
                &state_node_id,
                |world, schema_state: &mut PropHashMap<R>| {
                    let Some(state) = schema_state.get_mut(&prop_type_id) else {
                        error!("prop not found by index!");
                        return;
                    };
                    state.apply(Box::new(value), &mut *world);
                },
            )
            .is_none();
            if is_no_found {
                error!("not found schema prop_map");
            }
        });
    });

    ctx.set_prop_state(ReactiveDisposerState(effect.erase()));
}

impl<R, T> SchemaProp<R> for ReadSignal<T>
where
    R: Renderer,
    T: Clone + Send + Sync + 'static,
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
    T: Clone + Send + Sync + 'static,
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
//     T: SchemaProp<R> + Sync,
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
//     T: IntoSchemaProp<R, PV> + Send + Sync + 'static,
//     PV: Clone + Send + Sync + 'static,
// {
//     type Prop = Memo<T::Prop>;

//     fn into_schema_prop<const I: usize>(self) -> Self::Prop {
//         use_memo(move |_| self.get().into_schema_prop::<I>())
//     }
// }

impl<R, T> IntoSchemaProp<R, T> for ReadSignal<T>
where
    R: Renderer,
    T: Clone + Send + Sync + 'static,
{
    type Prop = Self;

    fn into_schema_prop<const I: usize>(self) -> Self::Prop {
        self
    }
}

impl<R, T> IntoSchemaProp<R, T> for Memo<T>
where
    R: Renderer,
    T: Clone + Send + Sync + 'static,
{
    type Prop = Self;

    fn into_schema_prop<const I: usize>(self) -> Self::Prop {
        self
    }
}

impl<R, T> IntoSchemaProp<R, T> for RwSignal<T>
where
    R: Renderer,
    T: Clone + Send + Sync + 'static,
{
    type Prop = ReadSignal<T>;

    fn into_schema_prop<const I: usize>(self) -> Self::Prop {
        self.read_only()
    }
}

impl<R, F, T> IntoSchemaProp<R, T> for Reactive<F, T>
where
    R: Renderer,
    F: Fn() -> T + Send + Sync + 'static,
    T: Clone + Send + Sync + PartialEq + 'static,
{
    type Prop = Memo<T>;

    fn into_schema_prop<const I: usize>(self) -> Self::Prop {
        use_memo(move |_| self.0())
    }
}

pub struct SignalPropState<R, T>
where
    R: Renderer,
    T: Send + Sync + 'static,
{
    write_signal: WriteSignal<T>,
    _marker: PhantomData<R>,
}

impl<R, T> SignalPropState<R, T>
where
    R: Renderer,
    T: Send + Sync + 'static,
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
    T: Send + Sync + 'static,
{
    fn apply(&mut self, new_value: BoxedPropValue, _world: &mut R::World) {
        let Ok(new_value) = new_value.downcast::<T>().map(|n| *n) else {
            return;
        };
        self.write_signal.set(new_value);
    }

    fn as_any_mut(&mut self) -> &mut (dyn Any + Send) {
        self
    }
}
