use crate::utils::SyncCell;
use crate::{Either, IntoView, MaybeSend, MaybeSync, NodeTree, Renderer, RendererNodeId, RendererWorld, View, ViewCtx, ViewKey};
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;

#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[derive(Clone, Debug)]
pub struct RecyclableViewKey<R, K>
where
    R: Renderer,
    K: ViewKey<R>,
{
    pub key: K,
    #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
    _marker: PhantomData<R>,
}

impl<R, K> RecyclableViewKey<R, K>
where
    R: Renderer,
    K: ViewKey<R>,
{
    pub fn new(key: K) -> Self {
        Self {
            key,
            _marker: PhantomData,
        }
    }
}
impl<R, K> ViewKey<R> for RecyclableViewKey<R, K>
where
    R: Renderer,
    K: ViewKey<R>,
{
    fn remove(mut self, world: &mut RendererWorld<R>) {
        world.recycle_node(&self.key);
        if let Some(key) = self.key.state_node_id() {
            let _ = SyncCell::to_inner(
                world
                    .take_node_state::<RecyclableViewKeySender<K>>(&key)
                    .unwrap()
                    .sender,
            )
            .send(self.key);
        }
    }

    fn insert_before(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
    ) {
        self.key.insert_before(world, parent, before_node_id)
    }

    fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool) {
        self.key.set_visibility(world, hidden)
    }

    fn state_node_id(&self) -> Option<RendererNodeId<R>> {
        self.key.state_node_id()
    }

    fn reserve_key(
        world: &mut RendererWorld<R>,
        will_rebuild: bool,
        parent: RendererNodeId<R>,
        spawn: bool,
    ) -> Self {
        RecyclableViewKey::new(K::reserve_key(world, will_rebuild, parent, spawn))
    }

    fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
        self.key.first_node_id(world)
    }
}

pub struct RecyclableView<R, V>
where
    V: View<R>,
    R: Renderer,
{
    view: Either<V, RecyclableViewKey<R, V::Key>>,
    sender: Option<oneshot::Sender<V::Key>>,
}

impl<R, V> RecyclableView<R, V>
where
    V: View<R>,
    R: Renderer,
{
    pub fn new(
        view: Either<V, RecyclableViewKey<R, V::Key>>,
        sender: oneshot::Sender<V::Key>,
    ) -> Self {
        Self {
            view,
            sender: Some(sender),
        }
    }
}

impl<R, V> IntoView<R> for RecyclableView<R, V>
where
    R: Renderer,
    V: View<R>,
{
    type View = Self;

    fn into_view(self) -> Self::View {
        self
    }
}

pub struct RecyclableViewKeySender<K>
where
    K: MaybeSend + MaybeSync + 'static,
{
    sender: SyncCell<oneshot::Sender<K>>,
}

impl<R, V> View<R> for RecyclableView<R, V>
where
    R: Renderer,
    V: View<R>,
{
    type Key = RecyclableViewKey<R, V::Key>;

    fn build(
        mut self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        let key = match self.view {
            Either::Left(view) => RecyclableViewKey::new(view.build(
                ViewCtx {
                    world: &mut *ctx.world,
                    parent: ctx.parent,
                },
                reserve_key.map(|n| n.key),
                will_rebuild,
            )),
            Either::Right(key) => {
                ctx.world.cancel_recycle_node(&key.key);
                key
            }
        };
        assert!(self.sender.is_some());
        if let Some(state_node_id) = key.state_node_id() {
            ctx.world.set_node_state(
                &state_node_id,
                RecyclableViewKeySender {
                    sender: SyncCell::new(self.sender.unwrap()),
                },
            );
        }
        key
    }

    fn rebuild(mut self, ctx: ViewCtx<R>, key: Self::Key) {
        if let Some(state_node_id) = key.state_node_id() {
            if let Some(sender) = self.sender.take() {
                ctx.world.set_node_state(
                    &state_node_id,
                    RecyclableViewKeySender {
                        sender: SyncCell::new(sender),
                    },
                );
            }
        }
        match self.view {
            Either::Left(view) => view.rebuild(ctx, key.key),
            Either::Right(_key) => {}
        }
    }
}
