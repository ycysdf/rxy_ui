use core::fmt::Debug;

use crate::utils::all_tuples;

use crate::{MaybeFromReflect, MaybeGetTypeRegistration, MaybeReflect, MaybeSend, MaybeSync, MaybeTypePath, NodeTree, Renderer, RendererNodeId, RendererWorld, ViewCtx};

pub trait View<R: Renderer>: Sized + MaybeSend + 'static {
   type Key: ViewKey<R>;

   fn build(self, ctx: ViewCtx<R>, reserve_key: Option<Self::Key>, will_rebuild: bool)
      -> Self::Key;

   fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key);
}

pub trait ViewKey<R: Renderer>:
   MaybeReflect
   + MaybeFromReflect
   + MaybeTypePath
   + MaybeGetTypeRegistration
   + MaybeSend
   + MaybeSync
   + Clone
   + Debug
   + 'static
{
   fn remove(self, world: &mut RendererWorld<R>);

   fn insert_before(
      &self,
      world: &mut RendererWorld<R>,
      parent: Option<&RendererNodeId<R>>,
      before_node_id: Option<&RendererNodeId<R>>,
   );

   fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool);

   // You need to make sure that it doesn't change
   fn state_node_id(&self) -> Option<RendererNodeId<R>>;

   // Implements it and returns Some(Self) when state_node_id returns None
   fn new_with_no_state_node() -> Option<Self> {
      None
   }

   fn reserve_key(
      world: &mut RendererWorld<R>,
      will_rebuild: bool,
      parent: RendererNodeId<R>,
      spawn: bool,
   ) -> Self;
   fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>>;
}

pub trait SoloView<R: Renderer>: View<R> {
   fn node_id(key: &Self::Key) -> &RendererNodeId<R>;
}

impl<R> View<R> for ()
where
   R: Renderer,
{
   type Key = ();

   fn build(
      self,
      _ctx: ViewCtx<R>,
      _reserve_key: Option<Self::Key>,
      _will_rebuild: bool,
   ) -> Self::Key {
   }

   fn rebuild(self, _ctx: ViewCtx<R>, _key: Self::Key) {}
}

impl<R> ViewKey<R> for ()
where
   R: Renderer,
{
   fn remove(self, _world: &mut RendererWorld<R>) {}
   fn insert_before(
      &self,
      _world: &mut RendererWorld<R>,
      _parent: Option<&RendererNodeId<R>>,
      _before_node_id: Option<&RendererNodeId<R>>,
   ) {
   }

   fn set_visibility(&self, _world: &mut RendererWorld<R>, _hidden: bool) {}

   fn state_node_id(&self) -> Option<RendererNodeId<R>> {
      None
   }

   fn new_with_no_state_node() -> Option<Self> {
      Some(())
   }

   fn reserve_key(
      _world: &mut RendererWorld<R>,
      _will_rebuild: bool,
      _parent: RendererNodeId<R>,
      _spawn: bool,
   ) -> Self {
   }

   fn first_node_id(&self, _world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
      None
   }
}

#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[derive(Clone, Debug)]
pub struct DataNodeId<R: Renderer>(pub RendererNodeId<R>);

impl<R> ViewKey<R> for DataNodeId<R>
where
   R: Renderer,
{
   fn remove(self, _world: &mut RendererWorld<R>) {}
   fn insert_before(
      &self,
      _world: &mut RendererWorld<R>,
      _parent: Option<&RendererNodeId<R>>,
      _before_node_id: Option<&RendererNodeId<R>>,
   ) {
   }

   fn set_visibility(&self, _world: &mut RendererWorld<R>, _hidden: bool) {}

   fn state_node_id(&self) -> Option<RendererNodeId<R>> {
      Some(self.0.clone())
   }

   fn reserve_key(
      world: &mut RendererWorld<R>,
      _will_rebuild: bool,
      _parent: RendererNodeId<R>,
      _spawn: bool,
   ) -> Self {
      DataNodeId(world.reserve_node_id())
   }

   fn first_node_id(&self, _world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
      None
   }
}

macro_rules! impl_view_key_for_tuples {
    ($first:ident) => {
        impl_view_key_for_tuples!($first,);
    };
    ($first:ident,$($ty:ident),*$(,)?) => {
        paste::paste! {
            impl<R,$first,$($ty),*> $crate::ViewKey<R> for ($first,$($ty,)*)
            where
                R: $crate::Renderer,
                $first: $crate::ViewKey<R>,
                $($ty: $crate::ViewKey<R>),*
            {

                fn remove(self, world: &mut RendererWorld<R>) {
                    let ([<$first:lower>], $([<$ty:lower>],)*) = self;
                    [<$first:lower>].remove(world);
                    $([<$ty:lower>].remove(world);)*
                }

                fn insert_before(
                    &self,
                    world: &mut RendererWorld<R>,
                    parent: Option<&RendererNodeId<R>>,
                    before_node_id: Option<&RendererNodeId<R>>,
                ) {
                    let ([<$first:lower>], $([<$ty:lower>],)*) = self;
                    [<$first:lower>].insert_before(world, parent, before_node_id);
                    $([<$ty:lower>].insert_before(world, parent, before_node_id);)*
                }

                fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool){
                    let ([<$first:lower>], $([<$ty:lower>],)*) = self;
                    [<$first:lower>].set_visibility(world, hidden);
                    $([<$ty:lower>].set_visibility(world, hidden);)*
                }

                fn state_node_id(&self) -> Option<RendererNodeId<R>> {
                    let ([<$first:lower>], $([<$ty:lower>],)*) = self;
                    [<$first:lower>].state_node_id()
                    $(
                        .or_else(|| [<$ty:lower>].state_node_id())
                    )*
                }

                fn reserve_key(world: &mut RendererWorld<R>, will_rebuild: bool,
                    parent: RendererNodeId<R>,
                    spawn: bool,
                ) -> Self {
                    let [<$first:lower _key>]=[<$first>]::reserve_key(world, will_rebuild, parent.clone(),spawn);
                    $(
                        let [<$ty:lower _key>]=[<$ty>]::reserve_key(world, will_rebuild, parent.clone(),spawn);
                    )*
                    (
                        [<$first:lower _key>],
                        $([<$ty:lower _key>]),*
                    )
                }
                fn first_node_id(
                    &self,
                    world: &RendererWorld<R>,
                ) -> Option<RendererNodeId<R>> {
                    let ([<$first:lower>], $([<$ty:lower>],)*) = self;
                    [<$first:lower>].first_node_id(world)
                    $(
                        .or_else(|| [<$ty:lower>].first_node_id(world))
                    )*
                }
            }
        }
    }
}

macro_rules! impl_view_for_tuples {
    ($first:ident) => {
        impl_view_for_tuples!($first,);
    };
    ($first:ident,$($ty:ident),*$(,)?) => {
        paste::paste! {
            impl<R,$first,$($ty),*> $crate::View<R> for ($first,$($ty,)*)
            where
                R: $crate::Renderer,
                $first: $crate::View<R>,
                $($ty: $crate::View<R>),*
            {
                type Key = ($first::Key, $($ty::Key,)*);

                fn build(
                    self,
                    ctx: ViewCtx<R>,
                    reserve_key: Option<Self::Key>,
                    will_rebuild: bool,
                ) -> Self::Key {
                    let ([<$first:lower>], $([<$ty:lower>],)*) = self;
                    let ([<$first:lower _reserve_key>], $([<$ty:lower _reserve_key>],)*) = if let Some(reserve_key) = reserve_key{
                        let ([<$first:lower _reserve_key>], $([<$ty:lower _reserve_key>],)*) = reserve_key;
                        (
                            Some([<$first:lower _reserve_key>]),
                            $(
                                Some([<$ty:lower _reserve_key>]),
                            )*
                        )
                    }else{
                        (
                            None,
                            $(
                                {
                                    let [<$ty:lower _reserve_key>] = None;
                                    [<$ty:lower _reserve_key>]
                                },
                            )*
                        )
                    };

                    let [<$first:lower _key>]=[<$first:lower>].build(ViewCtx{
                        world: &mut *ctx.world,
                        parent: ctx.parent.clone(),
                    }, [<$first:lower _reserve_key>], will_rebuild);
                    $(let [<$ty:lower _key>]=[<$ty:lower>].build(ViewCtx{
                        world: &mut *ctx.world,
                        parent: ctx.parent.clone(),
                    }, [<$ty:lower _reserve_key>], will_rebuild);)*
                    (
                        [<$first:lower _key>],
                        $([<$ty:lower _key>]),*
                    )
                }

                fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key){
                    let ([<$first:lower>], $([<$ty:lower>],)*) = self;
                    let ([<$first:lower _key>], $([<$ty:lower _key>],)*) = key;
                    [<$first:lower>].rebuild(ViewCtx{
                        world: &mut *ctx.world,
                        parent: ctx.parent.clone(),
                    },[<$first:lower _key>]);
                    $([<$ty:lower>].rebuild(ViewCtx{
                        world: &mut *ctx.world,
                        parent: ctx.parent.clone(),
                    },[<$ty:lower _key>]);)*
                }
            }
        }
    }
}

all_tuples!(impl_view_for_tuples, 1, 12, T);
all_tuples!(impl_view_key_for_tuples, 1, 12, T);
