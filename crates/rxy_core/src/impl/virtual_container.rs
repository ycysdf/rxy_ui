use alloc::borrow::Cow;
use core::fmt::Debug;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;

use rxy_macro::IntoView;

use crate::mutable_view::{MutableView, MutableViewKey};
use crate::{
    Either, EitherExt, IntoView, NodeTree, Renderer, RendererNodeId, RendererWorld, View, ViewCtx,
    ViewKey,
};

#[derive(Clone, IntoView)]
pub struct VirtualContainer<R, V>(V, Cow<'static, str>, PhantomData<R>)
where
    R: Renderer,
    V: MutableView<R>;

impl<R, V> VirtualContainer<R, V>
where
    R: Renderer,
    V: MutableView<R>,
{
    pub fn new(view: V, name: impl Into<Cow<'static, str>>) -> Self {
        Self(view, name.into(), Default::default())
    }
}

pub struct VirtualContainerChildrenViewKey<T>(pub T);

#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[derive(Clone, Debug)]
pub struct VirtualContainerNodeId<R, VK>
where
    R: Renderer,
    VK: MutableViewKey<R>,
{
    pub placeholder_node_id: RendererNodeId<R>,
    #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
    _marker: PhantomData<VK>,
}

impl<R, VK> VirtualContainerNodeId<R, VK>
where
    VK: MutableViewKey<R>,
    R: Renderer,
{
    fn new(node_id: RendererNodeId<R>) -> Self {
        Self {
            placeholder_node_id: node_id,
            _marker: Default::default(),
        }
    }
    fn set_view_key(&self, world: &mut R::NodeTree, view_key: VK) {
        world.set_node_state::<VirtualContainerChildrenViewKey<VK>>(
            &self.placeholder_node_id,
            VirtualContainerChildrenViewKey(view_key),
        )
    }

    fn get_view_key<'a>(&self, world: &'a R::NodeTree) -> Option<&'a VK> {
        world
            .get_node_state_ref::<VirtualContainerChildrenViewKey<VK>>(&self.placeholder_node_id)
            .map(move |n| &n.0)
    }
}

impl<R, VK> ViewKey<R> for VirtualContainerNodeId<R, VK>
where
    VK: MutableViewKey<R>,
    R: Renderer,
{
    #[inline]
    fn state_node_id(&self) -> Option<RendererNodeId<R>> {
        Some(self.placeholder_node_id.clone())
    }

    #[inline]
    fn remove(self, world: &mut RendererWorld<R>) {
        let Some(view_key) = self.get_view_key(world).cloned() else {
            panic!("view_key is None")
        };

        view_key.remove(world);
        world.remove_node(&self.placeholder_node_id);
    }

    #[inline]
    fn insert_before(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
    ) {
        let Some(view_key) = self.get_view_key(world).cloned() else {
            panic!("view_key is None")
        };
        view_key.insert_before(world, parent, before_node_id);
        world.insert_before(
            parent,
            before_node_id,
            core::slice::from_ref(&self.placeholder_node_id),
        );
    }

    #[inline]
    fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool) {
        let Some(view_key) = self.get_view_key(world).cloned() else {
            panic!("view_key is None")
        };
        view_key.set_visibility(world, hidden)
    }

    #[inline]
    fn reserve_key(
        world: &mut RendererWorld<R>,
        _will_rebuild: bool,
        _parent: RendererNodeId<R>,
        _spawn: bool,
    ) -> Self {
        VirtualContainerNodeId::new(world.reserve_node_id_or_spawn(_parent, _spawn))
    }

    #[inline]
    fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
        let Some(view_key) = self.get_view_key(world).cloned() else {
            panic!("view_key is None")
        };
        Some(view_key.first_node_id(world).unwrap_or_else(|| {
            // todo: ?
            self.placeholder_node_id.clone()
        }))
    }
}

impl<LK, RK, R> ViewKey<R> for Either<VirtualContainerNodeId<R, LK>, RK>
where
    LK: MutableViewKey<R>,
    RK: MutableViewKey<R>,
    R: Renderer,
{
    fn remove(self, world: &mut RendererWorld<R>) {
        match self {
            Either::Left(n) => n.remove(world),
            Either::Right(n) => n.remove(world),
        }
    }

    fn insert_before(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
    ) {
        match self {
            Either::Left(n) => n.insert_before(world, parent, before_node_id),
            Either::Right(n) => n.insert_before(world, parent, before_node_id),
        }
    }

    fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool) {
        match self {
            Either::Left(n) => n.set_visibility(world, hidden),
            Either::Right(n) => n.set_visibility(world, hidden),
        }
    }

    fn state_node_id(&self) -> Option<RendererNodeId<R>> {
        match self {
            Either::Left(n) => n.state_node_id(),
            Either::Right(n) => n.state_node_id(),
        }
    }

    fn reserve_key(
        world: &mut RendererWorld<R>,
        will_rebuild: bool,
        parent: RendererNodeId<R>,
        spawn: bool,
    ) -> Self {
        Either::Left(VirtualContainerNodeId::<R, LK>::reserve_key(
            world,
            will_rebuild,
            parent,
            spawn,
        ))
    }

    fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
        match self {
            Either::Left(n) => n.first_node_id(world),
            Either::Right(n) => n.first_node_id(world),
        }
    }
}

impl<R, MV> View<R> for VirtualContainer<R, MV>
where
    R: Renderer,
    MV: MutableView<R>,
{
    type Key = Either<VirtualContainerNodeId<R, MV::Key>, MV::Key>;
    // type Key = VirtualContainerNodeId<R, V::Key>;

    fn build(
        self,
        ViewCtx { world, parent }: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        let reserve_placeholder_node_id = reserve_key.map(|n| n.unwrap_left().placeholder_node_id);

        if reserve_placeholder_node_id.is_some()
            || will_rebuild
            || !MV::no_placeholder_when_no_rebuild()
        {
            let placeholder_node_id =
                world.spawn_placeholder(self.1, Some(&parent), reserve_placeholder_node_id);
            let view_key = self.0.build(
                ViewCtx {
                    world: &mut *world,
                    parent: parent.clone(),
                },
                Some(placeholder_node_id.clone()),
            );
            world.insert_before(
                Some(&parent),
                None,
                core::slice::from_ref(&placeholder_node_id),
            );
            let key = VirtualContainerNodeId::<R, MV::Key>::new(placeholder_node_id);
            key.set_view_key(&mut *world, view_key);
            key.either_left()
        } else {
            let view_key = self.0.build(
                ViewCtx {
                    world: &mut *world,
                    parent: parent.clone(),
                },
                None,
            );
            view_key.either_right()
        }
    }

    fn rebuild(self, ViewCtx { world, parent }: ViewCtx<R>, key: Self::Key) {
        let key = key.unwrap_left();
        let Some(view_key) = key.get_view_key(world).cloned() else {
            panic!("view_key is None")
        };
        let result = self.0.rebuild(
            ViewCtx {
                world: &mut *world,
                parent,
            },
            view_key.clone(),
            key.placeholder_node_id.clone(),
        );
        if let Some(new_view_key) = result {
            key.set_view_key(world, new_view_key);
        }
    }
}

#[inline]
pub fn virtual_container<R: Renderer, V: MutableView<R>>(
    view: V,
    name: impl Into<Cow<'static, str>>,
) -> VirtualContainer<R, V> {
    VirtualContainer::new(view, name)
}
