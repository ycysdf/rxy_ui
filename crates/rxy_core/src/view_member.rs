use core::any::TypeId;

use bevy_utils::synccell::SyncCell;
use bevy_utils::{all_tuples, HashMap};

use crate::{Renderer, ViewMemberCtx};

pub trait ViewMember<R>: Send + 'static
where
    R: Renderer,
{
    fn count() -> u8;
    fn unbuild(ctx: ViewMemberCtx<R>);
    fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool);
    fn rebuild(self, ctx: ViewMemberCtx<R>);
}

impl<R> ViewMember<R> for ()
where
    R: Renderer,
{

    fn count() -> u8 {
        0
    }

    fn unbuild(_ctx: ViewMemberCtx<R>) {}

    fn build(self, _ctx: ViewMemberCtx<R>, _will_rebuild: bool) {}
    fn rebuild(self, _ctx: ViewMemberCtx<R>) {}
}

impl<R, T, F> ViewMember<R> for F
where
    F: Fn() -> T + Send + 'static,
    R: Renderer,
    T: ViewMember<R>,
{

    fn count() -> u8 {
        T::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>) {
        T::unbuild(ctx)
    }

    fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
        self().build(ctx, will_rebuild)
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        self().rebuild(ctx)
    }
}

macro_rules! impl_view_member_for_tuples {
    ($first:ident) => {
        impl_view_member_for_tuples!($first,);
    };
    ($first:ident,$($ty:ident),*$(,)?) => {
        #[allow(unused_assignments)]
        impl<R,$first,$($ty),*> $crate::ViewMember<R> for ($first,$($ty,)*)
		where
            R: $crate::Renderer,
			$first: $crate::ViewMember<R>,
			$($ty: $crate::ViewMember<R>),*
        {
            fn count() -> u8 {
                $first::count() $(+ $ty::count())*
            }

            fn unbuild(ctx: ViewMemberCtx<R>) {
                let mut index = ctx.index;
                $first::unbuild(ViewMemberCtx{
                    index,
                    type_id: core::any::TypeId::of::<$first>(),
                    world: &mut *ctx.world,
                    node_id: ctx.node_id.clone(),
                });
                index += $first::count();
                $(
                $ty::unbuild(ViewMemberCtx{
                    index,
                    type_id: core::any::TypeId::of::<$ty>(),
                    world: &mut *ctx.world,
                    node_id: ctx.node_id.clone(),
                });
                index += $ty::count();
                )*
            }
            fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
                let mut index = ctx.index;
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
                    [<$first:lower>].build(ViewMemberCtx{
                        index,
                        type_id: core::any::TypeId::of::<$first>(),
                        world: &mut *ctx.world,
                        node_id: ctx.node_id.clone(),
                    },will_rebuild);
                    index += $first::count();
                    $(

                    [<$ty:lower>].build(ViewMemberCtx{
                        index,
                        type_id: core::any::TypeId::of::<$ty>(),
                        world: &mut *ctx.world,
                        node_id: ctx.node_id.clone(),
                    },will_rebuild);
                    index += $ty::count();

                    )*
				}
            }

            fn rebuild(self, ctx: ViewMemberCtx<R>) {
                let mut index = ctx.index;
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
                    [<$first:lower>].rebuild(ViewMemberCtx{
                        index,
                        type_id: core::any::TypeId::of::<$first>(),
                        world: &mut *ctx.world,
                        node_id: ctx.node_id.clone(),
                    });
                    index += $first::count();
                    $(
                    [<$ty:lower>].rebuild(ViewMemberCtx{
                        index,
                        type_id: core::any::TypeId::of::<$ty>(),
                        world: &mut *ctx.world,
                        node_id: ctx.node_id.clone(),
                    });
                    index += $ty::count();
                    )*
				}
            }
        }
    }
}

all_tuples!(impl_view_member_for_tuples, 1, 12, M);

pub struct TypeIdHashMapState<S: Send + 'static>(pub SyncCell<HashMap<TypeId, S>>);
pub struct MemberHashMapState<S: Send + 'static>(pub SyncCell<HashMap<u8, S>>);

impl<'a, R: Renderer> ViewMemberCtx<'a, R> {
    pub fn view_member_state_mut<S: Send + 'static>(&mut self) -> Option<&mut S> {
        R::get_state_mut::<TypeIdHashMapState<S>>(self.world, &self.node_id)
            .map(|s| s.0.get().get_mut(&self.type_id))
            .flatten()
    }
    pub fn take_view_member_state<S: Send + 'static>(&mut self) -> Option<S> {
        R::get_state_mut::<TypeIdHashMapState<S>>(self.world, &self.node_id)
            .map(|s| s.0.get().remove(&self.type_id))
            .flatten()
    }
    pub fn set_view_member_state<S: Send + 'static>(&mut self, state: S) {
        if let Some(map) =
            R::get_state_mut::<TypeIdHashMapState<S>>(&mut *self.world, &self.node_id)
        {
            map.0.get().insert(self.type_id, state);
        } else {
            let mut map = HashMap::default();
            map.insert(self.type_id, state);
            R::set_state(
                &mut *self.world,
                &self.node_id,
                TypeIdHashMapState(SyncCell::new(map)),
            );
        }
    }

    pub fn indexed_view_member_state_mut<S: Send + 'static>(&mut self) -> Option<&mut S> {
        R::get_state_mut::<MemberHashMapState<S>>(self.world, &self.node_id)
            .map(|s| s.0.get().get_mut(&self.index))
            .flatten()
    }
    pub fn take_indexed_view_member_state<S: Send + 'static>(&mut self) -> Option<S> {
        R::get_state_mut::<MemberHashMapState<S>>(self.world, &self.node_id)
            .map(|s| s.0.get().remove(&self.index))
            .flatten()
    }
    pub fn set_indexed_view_member_state<S: Send + 'static>(&mut self, state: S) {
        if let Some(map) =
            R::get_state_mut::<MemberHashMapState<S>>(&mut *self.world, &self.node_id)
        {
            map.0.get().insert(self.index, state);
        } else {
            let mut map = HashMap::default();
            map.insert(self.index, state);
            R::set_state(
                &mut *self.world,
                &self.node_id,
                MemberHashMapState(SyncCell::new(map)),
            );
        }
    }

}
