use crate::{into_view, BoxedCloneableErasureView, BoxedErasureView, BoxedPropValue, ConstIndex, DataNodeId, InnerSchemaCtx, IntoCloneableView, IntoSchemaProp, IntoView, IntoViewCloneableErasureExt, IntoViewErasureExt, NodeTree, PropHashMap, Renderer, RendererNodeId, RendererWorld, Schema, SchemaProp, SchemaProps, View, ViewCtx, ViewKey, MaybeSend};
use alloc::boxed::Box;
use bevy_utils::synccell::SyncCell;
use bevy_utils::HashMap;
use core::any::TypeId;
use core::marker::PhantomData;
use rxy_macro::IntoView;

#[derive(IntoView)]
pub struct SchemaView<R, U, P = (), M = ()>
where
    R: Renderer,
    U: Schema<R>,
    P: SchemaProps<R>,
    M: MaybeSend + 'static,
{
    u: U,
    props: Option<P>,
    static_values: HashMap<TypeId, BoxedPropValue>,
    slots: HashMap<TypeId, BoxedErasureView<R>>,
    param_index_to_prop_type_id: HashMap<usize, TypeId>,
    cloneable_slots: HashMap<TypeId, BoxedCloneableErasureView<R>>,
    _marker: PhantomData<M>,
}

impl<R, U, M> Default for SchemaView<R, U, (), M>
where
    R: Renderer,
    U: Schema<R> + Default,
    M: MaybeSend + 'static,
{
    fn default() -> Self {
        Self {
            u: Default::default(),
            props: Some(()),
            static_values: Default::default(),
            slots: Default::default(),
            cloneable_slots: Default::default(),
            param_index_to_prop_type_id: Default::default(),
            _marker: Default::default(),
        }
    }
}

impl<R, U, M> SchemaView<R, U, (), M>
where
    R: Renderer,
    U: Schema<R>,
    M: MaybeSend + 'static,
{
    #[inline]
    pub fn new(u: U) -> Self {
        SchemaView {
            u,
            props: Some(()),
            static_values: Default::default(),
            slots: Default::default(),
            cloneable_slots: Default::default(),
            param_index_to_prop_type_id: Default::default(),
            _marker: Default::default(),
        }
    }
}

impl<R, U, P, M> SchemaView<R, U, P, M>
where
    R: Renderer,
    U: Schema<R>,
    P: SchemaProps<R>,
    M: MaybeSend + 'static,
{
    #[inline(always)]
    pub fn map<MU>(self, f: impl FnOnce(U) -> MU) -> SchemaView<R, MU, P, M>
    where
        MU: Schema<R>,
    {
        SchemaView {
            u: f(self.u),
            props: self.props,
            static_values: self.static_values,
            slots: self.slots,
            cloneable_slots: self.cloneable_slots,
            param_index_to_prop_type_id: self.param_index_to_prop_type_id,
            _marker: Default::default(),
        }
    }

    #[inline(always)]
    pub fn indexed_slot<const I: usize>(mut self, view: impl IntoView<R>) -> Self {
        let type_id = TypeId::of::<ConstIndex<I>>();
        self.slots
            .insert(type_id, unsafe { view.into_erasure_view() });
        self
    }

    #[inline(always)]
    pub fn cloneable_indexed_slot<const I: usize>(
        mut self,
        view: impl IntoCloneableView<R>,
    ) -> Self {
        let type_id = TypeId::of::<ConstIndex<I>>();
        self.cloneable_slots.insert(type_id, unsafe {
            into_view(view.into_cloneable_view()).into_cloneable_erasure_view()
        });
        self
    }

    #[inline(always)]
    pub fn set_indexed_prop<const I: usize, ISP, IT>(
        self,
        value: ISP,
    ) -> SchemaView<R, U, P::Props<ConstIndex<I, ISP::Prop>>, M>
    where
        P::Props<ConstIndex<I, ISP::Prop>>: SchemaProps<R>,
        ISP: IntoSchemaProp<R, IT>,
        IT: MaybeSend + 'static,
    {
        SchemaView {
            u: self.u,
            props: self
                .props
                .map(|n| n.add(ConstIndex::<I, ISP::Prop>(value.into_schema_prop::<I>()))),
            static_values: self.static_values,
            slots: self.slots,
            param_index_to_prop_type_id: self.param_index_to_prop_type_id,
            cloneable_slots: self.cloneable_slots,
            _marker: Default::default(),
        }
    }

    #[inline(always)]
    pub fn set_static_indexed_prop<const I: usize, ISP, IT>(mut self, value: ISP) -> Self
    where
        ISP: IntoSchemaProp<R, IT>,
        IT: MaybeSend + 'static,
    {
        let type_id = TypeId::of::<ConstIndex<I>>();
        let mut prop = value.into_schema_prop::<I>();
        if let Some(value) = prop.get_init_value() {
            self.static_values.insert(type_id, Box::new(value));
        }
        self
    }
}

pub struct SchemaViewState<R> {
    prop_state: SyncCell<Option<PropHashMap<R>>>,
    #[cfg(feature = "xy_reactive")]
    _other_state: Vec<xy_reactive::effect::ErasureEffect>,
}

pub fn scheme_state_scoped<R, U>(
    world: &mut RendererWorld<R>,
    node_id: &RendererNodeId<R>,
    f: impl FnOnce(&mut RendererWorld<R>, &mut PropHashMap<R>) -> U,
) -> Option<U>
where
    R: Renderer,
{
    let mut taken_map = world.get_node_state_mut::<SchemaViewState<R>>(node_id)
        .and_then(|n| n.prop_state.get().take())?;
    let u = f(&mut *world, &mut taken_map);

    let option = world.get_node_state_mut::<SchemaViewState<R>>(node_id)
        .unwrap()
        .prop_state
        .get();
    *option = Some(taken_map);
    Some(u)
}

// Makes SchemaView state_node_id even if it's empty view
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[derive(Clone, Debug)]
pub struct ViewKeyOrDataNodeId<R, K>
where
    R: Renderer,
    K: ViewKey<R>,
{
    pub data_node_id: Option<DataNodeId<R>>,
    pub key: K,
}


impl<R, K> ViewKey<R> for ViewKeyOrDataNodeId<R, K>
where
    R: Renderer,
    K: ViewKey<R>,
{
    fn remove(self, world: &mut RendererWorld<R>) {
        if let Some(data_node_id) = self.data_node_id {
            world.remove_node(&data_node_id.0);
        }
        self.key.remove(world);
    }

    fn insert_before(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
    ) {
        self.key.insert_before(world, parent, before_node_id);
    }

    fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool) {
        self.key.set_visibility(world, hidden);
    }

    fn state_node_id(&self) -> Option<RendererNodeId<R>> {
        self.key.state_node_id()
    }

    fn reserve_key(world: &mut RendererWorld<R>, will_rebuild: bool) -> Self {
        Self {
            data_node_id: if TypeId::of::<K>() == TypeId::of::<()>() {
                Some(DataNodeId(world.spawn_data_node()))
            } else {
                None
            },
            key: K::reserve_key(world, will_rebuild),
        }
    }

    fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
        self.key.first_node_id(world)
    }
}

pub fn schema_view_build<R, U, P, M>(
    mut schema_view: SchemaView<R, U, P, M>,
    ctx: ViewCtx<R>,
    reserve_key: Option<ViewKeyOrDataNodeId<R, <U::View as View<R>>::Key>>,
    will_rebuild: bool,
    // view_build_f: impl FnOnce(U::View, ViewCtx<R>, Option<<U::View as View<R>>::Key>) -> <U::View as View<R>>::Key,
) -> ViewKeyOrDataNodeId<R, <<U as Schema<R>>::View as View<R>>::Key>
where
    R: Renderer,
    U: Schema<R>,
    P: SchemaProps<R>,
    M: MaybeSend + 'static,
{
    let mut props = schema_view.props.take().unwrap();
    let mut init_values = props.get_init_values();
    init_values.extend(schema_view.static_values);

    let mut prop_state = PropHashMap::<R>::default();
    #[cfg(feature = "xy_reactive")]
    let mut _effect_state = vec![];
    let view = schema_view.u.view(InnerSchemaCtx {
        world: &mut *ctx.world,
        parent: ctx.parent.clone(),
        slots: &mut schema_view.slots,
        cloneable_slots: &mut schema_view.cloneable_slots,
        init_values,
        prop_state: &mut prop_state,
        #[cfg(feature = "xy_reactive")]
        effect_state: &mut _effect_state,
        _marker: Default::default(),
    });
    let (data_node_id, reserve_key) = reserve_key.map(|k| (k.data_node_id, k.key)).unzip();
    let key = view.build(
        ViewCtx {
            world: &mut *ctx.world,
            parent: ctx.parent,
        },
        reserve_key,
        false,
    );
    let (data_node_id, state_node_id) = {
        let data_state_node_id = |data_node_id: Option<Option<DataNodeId<R>>>,
                                  world: &mut RendererWorld<R>| {
            let state_node_id = data_node_id
                .map(|n| n.unwrap())
                .unwrap_or_else(|| DataNodeId(world.spawn_data_node()));
            (Some(state_node_id.clone()), state_node_id.0)
        };
        if let Some(state_node_id) = key.state_node_id() {
            // Whether there is a schema nest, occupying state_node_id
            if ctx
                .world
                .get_node_state_ref::<SchemaViewState<R>>(&state_node_id)
                .is_some()
            {
                data_state_node_id(data_node_id, ctx.world)
            } else {
                (None, state_node_id)
            }
        } else {
            data_state_node_id(data_node_id, ctx.world)
        }
    };

    props.build(
        &mut *ctx.world,
        state_node_id.clone(),
        &mut prop_state,
        will_rebuild,
    );

    ctx.world.set_node_state::<SchemaViewState<R>>(
        &state_node_id,
        SchemaViewState {
            prop_state: SyncCell::new(Some(prop_state)),
            #[cfg(feature = "xy_reactive")]
            _other_state: _effect_state,
        },
    );

    ViewKeyOrDataNodeId { data_node_id, key }
}

impl<R, U, P, M> View<R> for SchemaView<R, U, P, M>
where
    R: Renderer,
    U: Schema<R>,
    P: SchemaProps<R>,
    M: MaybeSend + 'static,
{
    type Key = ViewKeyOrDataNodeId<R, <U::View as View<R>>::Key>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        schema_view_build(self, ctx, reserve_key, will_rebuild)
    }

    fn rebuild(mut self, ctx: ViewCtx<R>, key: Self::Key) {
        let ViewKeyOrDataNodeId { data_node_id, key } = key;
        let state_node_id = key.state_node_id().or(data_node_id.map(|n| n.0)).unwrap();

        scheme_state_scoped(&mut *ctx.world, &state_node_id, {
            let state_node_id = state_node_id.clone();
            move |world, prop_map| {
                let props = self.props.take().unwrap();
                props.rebuild(world, state_node_id, prop_map);
            }
        });
    }
}
