/* use crate::{
    Either, EitherExt, IntoView, Renderer, RendererNodeId, RendererViewExt, RendererWorld, View,
    ViewCtx, ViewKey,
};

#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[derive(Debug, Hash, Clone)]
pub enum ReflectVariant<Struct, TupleStruct, Tuple, List, Array, Map, Enum> {
    Struct(Struct),
    TupleStruct(TupleStruct),
    Tuple(Tuple),
    List(List),
    Array(Array),
    Map(Map),
    Enum(Enum),
}

impl<R, Struct, TupleStruct, Tuple, List, Array, Map, Enum> ViewKey<R>
    for Either<
        RendererNodeId<R>,
        ReflectVariant<Struct, TupleStruct, Tuple, List, Array, Map, Enum>,
    >
where
    R: Renderer,
    Struct: ViewKey<R>,
    TupleStruct: ViewKey<R>,
    Tuple: ViewKey<R>,
    List: ViewKey<R>,
    Array: ViewKey<R>,
    Map: ViewKey<R>,
    Enum: ViewKey<R>,
{
    fn remove(self, world: &mut RendererWorld<R>) {
        let variant = self
            .map_left(|state_node_id| {
                let variant = R::take_state::<
                    ReflectVariant<Struct, TupleStruct, Tuple, List, Array, Map, Enum>,
                >(world, &state_node_id)
                .unwrap();
                R::remove_node(world, &state_node_id);
                variant
            })
            .into_inner();
        match variant {
            ReflectVariant::Struct(n) => n.remove(world),
            ReflectVariant::TupleStruct(n) => n.remove(world),
            ReflectVariant::Tuple(n) => n.remove(world),
            ReflectVariant::List(n) => n.remove(world),
            ReflectVariant::Array(n) => n.remove(world),
            ReflectVariant::Map(n) => n.remove(world),
            ReflectVariant::Enum(n) => n.remove(world),
        }
    }

    fn insert_before(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
    ) {
        let variant = self
            .as_ref()
            .map_left(|state_node_id| {
                R::get_view_state_ref::<
                    ReflectVariant<Struct, TupleStruct, Tuple, List, Array, Map, Enum>,
                >(world, state_node_id)
                .unwrap()
            })
            .into_inner()
            .to_owned();
        match variant {
            ReflectVariant::Struct(n) => n.insert_before(world, parent, before_node_id),
            ReflectVariant::TupleStruct(n) => n.insert_before(world, parent, before_node_id),
            ReflectVariant::Tuple(n) => n.insert_before(world, parent, before_node_id),
            ReflectVariant::List(n) => n.insert_before(world, parent, before_node_id),
            ReflectVariant::Array(n) => n.insert_before(world, parent, before_node_id),
            ReflectVariant::Map(n) => n.insert_before(world, parent, before_node_id),
            ReflectVariant::Enum(n) => n.insert_before(world, parent, before_node_id),
        }
    }

    fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool) {
        let variant = self
            .as_ref()
            .map_left(|state_node_id| {
                R::get_view_state_ref::<
                    ReflectVariant<Struct, TupleStruct, Tuple, List, Array, Map, Enum>,
                >(world, state_node_id)
                .unwrap()
            })
            .into_inner()
            .to_owned();
        match variant {
            ReflectVariant::Struct(n) => n.set_visibility(world, hidden),
            ReflectVariant::TupleStruct(n) => n.set_visibility(world, hidden),
            ReflectVariant::Tuple(n) => n.set_visibility(world, hidden),
            ReflectVariant::List(n) => n.set_visibility(world, hidden),
            ReflectVariant::Array(n) => n.set_visibility(world, hidden),
            ReflectVariant::Map(n) => n.set_visibility(world, hidden),
            ReflectVariant::Enum(n) => n.set_visibility(world, hidden),
        }
    }

    fn state_node_id(&self) -> Option<RendererNodeId<R>> {
        match self {
            Either::Left(n) => Some(n.clone()),
            Either::Right(n) => match n {
                ReflectVariant::Struct(n) => n.state_node_id(),
                ReflectVariant::TupleStruct(n) => n.state_node_id(),
                ReflectVariant::Tuple(n) => n.state_node_id(),
                ReflectVariant::List(n) => n.state_node_id(),
                ReflectVariant::Array(n) => n.state_node_id(),
                ReflectVariant::Map(n) => n.state_node_id(),
                ReflectVariant::Enum(n) => n.state_node_id(),
            },
        }
    }

    fn reserve_key(world: &mut RendererWorld<R>, _will_rebuild: bool) -> Self {
        Either::Left(R::spawn_data_node(world))
    }

    fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
        let variant = self
            .as_ref()
            .map_left(|state_node_id| {
                R::get_view_state_ref::<
                    ReflectVariant<Struct, TupleStruct, Tuple, List, Array, Map, Enum>,
                >(world, state_node_id)
                .unwrap()
            })
            .into_inner()
            .to_owned();
        match variant {
            ReflectVariant::Struct(n) => n.first_node_id(world),
            ReflectVariant::TupleStruct(n) => n.first_node_id(world),
            ReflectVariant::Tuple(n) => n.first_node_id(world),
            ReflectVariant::List(n) => n.first_node_id(world),
            ReflectVariant::Array(n) => n.first_node_id(world),
            ReflectVariant::Map(n) => n.first_node_id(world),
            ReflectVariant::Enum(n) => n.first_node_id(world),
        }
    }
}

impl<R, Struct, TupleStruct, Tuple, List, Array, Map, Enum> View<R>
    for ReflectVariant<Struct, TupleStruct, Tuple, List, Array, Map, Enum>
where
    R: Renderer,
    Struct: View<R>,
    TupleStruct: View<R>,
    Tuple: View<R>,
    List: View<R>,
    Array: View<R>,
    Map: View<R>,
    Enum: View<R>,
{
    type Key = Either<
        RendererNodeId<R>,
        ReflectVariant<
            Struct::Key,
            TupleStruct::Key,
            Tuple::Key,
            List::Key,
            Array::Key,
            Map::Key,
            Enum::Key,
        >,
    >;

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        let key_variant = {
            let ctx = ViewCtx {
                world: &mut *ctx.world,
                parent: ctx.parent.clone(),
            };
            match self {
                ReflectVariant::Struct(n) => {
                    ReflectVariant::Struct(n.build(ctx, None, will_rebuild))
                }
                ReflectVariant::TupleStruct(n) => {
                    ReflectVariant::TupleStruct(n.build(ctx, None, will_rebuild))
                }
                ReflectVariant::Tuple(n) => ReflectVariant::Tuple(n.build(ctx, None, will_rebuild)),
                ReflectVariant::List(n) => ReflectVariant::List(n.build(ctx, None, will_rebuild)),
                ReflectVariant::Array(n) => ReflectVariant::Array(n.build(ctx, None, will_rebuild)),
                ReflectVariant::Map(n) => ReflectVariant::Map(n.build(ctx, None, will_rebuild)),
                ReflectVariant::Enum(n) => ReflectVariant::Enum(n.build(ctx, None, will_rebuild)),
            }
        };
        if let Some(reserve_key) = reserve_key {
            let data_node_id = reserve_key.unwrap_left();
            R::set_view_state::<
                ReflectVariant<
                    Struct::Key,
                    TupleStruct::Key,
                    Tuple::Key,
                    List::Key,
                    Array::Key,
                    Map::Key,
                    Enum::Key,
                >,
            >(ctx.world, &data_node_id, key_variant);
            data_node_id.either_left()
        } else {
            key_variant.either_right()
        }
    }

    fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key) {
        let key_variant = key
            .map_left(|data_node_id| {
                R::get_view_state_ref::<
                    ReflectVariant<
                        Struct::Key,
                        TupleStruct::Key,
                        Tuple::Key,
                        List::Key,
                        Array::Key,
                        Map::Key,
                        Enum::Key,
                    >,
                >(ctx.world, &data_node_id)
                .cloned()
                .unwrap()
            })
            .into_inner()
            .to_owned();
        // todo:
        match self {
            ReflectVariant::Struct(n) => match key_variant {
                ReflectVariant::Struct(key) => {
                    n.rebuild(
                        ViewCtx {
                            world: &mut *ctx.world,
                            parent: ctx.parent.clone(),
                        },
                        key,
                    );
                }
                ReflectVariant::TupleStruct(_key) => {

                }
                ReflectVariant::Tuple(_key) => {}
                ReflectVariant::List(_key) => {}
                ReflectVariant::Array(_key) => {}
                ReflectVariant::Map(_key) => {}
                ReflectVariant::Enum(_key) => {}
            },
            ReflectVariant::TupleStruct(_n) => {}
            ReflectVariant::Tuple(_n) => {}
            ReflectVariant::List(_n) => {}
            ReflectVariant::Array(_n) => {}
            ReflectVariant::Map(_n) => {}
            ReflectVariant::Enum(_n) => {}
        }
    }
}

impl<R, Struct, TupleStruct, Tuple, List, Array, Map, Enum> IntoView<R>
    for ReflectVariant<Struct, TupleStruct, Tuple, List, Array, Map, Enum>
where
    R: Renderer,
    Struct: View<R>,
    TupleStruct: View<R>,
    Tuple: View<R>,
    List: View<R>,
    Array: View<R>,
    Map: View<R>,
    Enum: View<R>,
{
    type View = Self;

    fn into_view(self) -> Self::View {
        self
    }
}
 */