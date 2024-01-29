use crate::{
    BoxedPropValue, PropHashMap, Renderer, RendererNodeId, RendererWorld, SchemaProp, SchemaPropCtx,
};
use bevy_utils::{all_tuples, HashMap};
use alloc::boxed::Box;

pub trait SchemaProps<R>: Sized + Send + 'static
where
    R: Renderer,
{
    type Props<P: SchemaProp<R>>: SchemaProps<R>;
    fn add<P>(self, p: P) -> Self::Props<P>
    where
        P: SchemaProp<R>,
        Self::Props<P>: SchemaProps<R>;
    fn get_init_values(&mut self) -> HashMap<core::any::TypeId, BoxedPropValue>;

    fn build(
        self,
        world: &mut RendererWorld<R>,
        state_node_id: RendererNodeId<R>,
        state: &mut PropHashMap<R>,
        will_rebuild: bool,
    );
    fn rebuild(
        self,
        world: &mut RendererWorld<R>,
        state_node_id: RendererNodeId<R>,
        state: &mut PropHashMap<R>,
    );
}

macro_rules! impl_schema_props_for_tuples {
    ($($ty:ident),*) => {
        #[allow(non_snake_case)]
        impl<R,$($ty),*> SchemaProps<R> for ($($ty,)*)
        where
            R: $crate::Renderer,
            $($ty: $crate::SchemaProp<R>),*
        {
            type Props<P:SchemaProp<R>> = ($($ty,)*P,);
            // type Props<P:SchemaProp<R>> = (Self,P);

            fn add<P>(self,p: P) -> Self::Props<P>
            where
                P:SchemaProp<R>,
                Self::Props<P>: SchemaProps<R>
            {
                let ($($ty,)*) = self;
                ($($ty,)*p,)
                // (self,p)
            }

            fn get_init_values(&mut self)->HashMap<core::any::TypeId, BoxedPropValue> {
                let mut _init_values = HashMap::<core::any::TypeId, BoxedPropValue>::default();
                let ($($ty,)*) = self;
                $(
                let prop_type_id = $ty::prop_type_id().unwrap();
                let value = $ty.get_init_value();
                if let Some(value) = value{
                    // todo: refactor
                    _init_values.insert(prop_type_id,Box::new(value));
                }
                )*
                _init_values
            }

            fn build(self,
                _world: &mut RendererWorld<R>,
                _state_node_id: RendererNodeId<R>, _state: &mut PropHashMap<R>, _will_rebuild: bool){
                let ($($ty,)*) = self;
                $(
                let prop_type_id = $ty::prop_type_id().unwrap();
                $ty.build(SchemaPropCtx{
                    world: &mut *_world,
                    state_node_id: _state_node_id.clone(),
                    prop_type_id,
                }, &mut **_state.get_mut(&prop_type_id).unwrap(), _will_rebuild);
                )*
            }

            fn rebuild(self,
                _world: &mut RendererWorld<R>,
                _state_node_id: RendererNodeId<R>, _state: &mut PropHashMap<R>){
                let ($($ty,)*) = self;
                $(
                let prop_type_id = $ty::prop_type_id().unwrap();
                $ty.rebuild(SchemaPropCtx{
                    world: &mut *_world,
                    state_node_id: _state_node_id.clone(),
                    prop_type_id,
                }, &mut **_state.get_mut(&prop_type_id).unwrap());
                )*
            }
        }
    };
    (END;$($ty:ident),*) => {
        #[allow(non_snake_case)]
        impl<R,$($ty),*> SchemaProps<R> for ($($ty,)*)
        where
            R: $crate::Renderer,
            $($ty: $crate::SchemaProp<R>),*
        {
            type Props<P:SchemaProp<R>> = ();

            fn add<P>(self,_p: P) -> Self::Props<P>
            where
                P:SchemaProp<R>,
                Self::Props<P>: SchemaProps<R>

            {
            }

            fn get_init_values(&mut self)->HashMap<core::any::TypeId, BoxedPropValue> {
                unimplemented!()
            }

            fn build(self,
                _world: &mut RendererWorld<R>,
                _state_node_id: RendererNodeId<R>, _state: &mut PropHashMap<R>, _will_rebuild: bool){

            }

            fn rebuild(self,
                _world: &mut RendererWorld<R>,
                _state_node_id: RendererNodeId<R>, _state: &mut PropHashMap<R>){
                unimplemented!()
            }
        }
    };
}

all_tuples!(impl_schema_props_for_tuples, 0, 17, T);
impl_schema_props_for_tuples!(END;T0 , T1 , T2 , T3 , T4 , T5 , T6 , T7 , T8 , T9 , T10 , T11 , T12 , T13 , T14 , T15 , T16, T17 );
