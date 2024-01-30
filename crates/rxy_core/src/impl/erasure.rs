use crate::{
    CloneableDynamicView, DynamicMutableViewKey, DynamicView, IntoView, MutableView,
    MutableViewKey, Renderer, RendererNodeId, RendererViewExt, RendererWorld, View, ViewCtx,
    ViewKey, ViewMember, ViewMemberCtx,
};
use alloc::boxed::Box;
use core::any::Any;
use core::hash::{Hash, Hasher};
use core::ops::Deref;

#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct ErasureViewFns<R>
where
    R: Renderer,
{
    #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
    pub remove_fn:
        Option<fn(key: Box<dyn Any>, world: &mut R::World, state_node_id: &RendererNodeId<R>)>,

    #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
    pub insert_before_fn: Option<
        fn(
            key: &dyn Any,
            world: &mut <R as Renderer>::World,
            parent: Option<&<R as Renderer>::NodeId>,
            before_node_id: Option<&<R as Renderer>::NodeId>,
            state_node_id: &RendererNodeId<R>,
        ),
    >,

    #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
    pub set_visibility_fn:
        Option<fn(key: &dyn Any, &mut R::World, hidden: bool, state_node_id: &RendererNodeId<R>)>,

    #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
    pub state_node_id: Option<fn(key: &dyn Any) -> Option<RendererNodeId<R>>>,

    #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
    pub reserve_key:
        Option<fn(world: &mut RendererWorld<R>, will_rebuild: bool) -> DynamicMutableViewKey<R>>,
    #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
    pub first_node_id:
        Option<fn(key: &dyn Any, world: &RendererWorld<R>) -> Option<RendererNodeId<R>>>,
}

pub fn get_erasure_view_fns<'a, R>(
    world: &'a RendererWorld<R>,
    state_node_id: &<R as Renderer>::NodeId,
) -> &'a ErasureViewFns<R>
where
    R: Renderer,
{
    let Some(erasure_fns) = R::get_state_ref::<ErasureViewFns<R>>(world, state_node_id) else {
        panic!("no found view type data!")
    };
    erasure_fns
}

pub fn set_erasure_view_fns<R: Renderer, V: View<R>>(
    world: &mut RendererWorld<R>,
    state_node_id: &<R as Renderer>::NodeId,
) {
    R::set_state(
        world,
        state_node_id,
        ErasureViewFns::<R> {
            remove_fn: Some(|key, world, _state_node_id| {
                let key = *key.downcast::<V::Key>().unwrap();
                key.remove(world);
            }),
            insert_before_fn: Some(|key, world, parent, before_node_id, _state_node_id| {
                let key = key.downcast_ref::<V::Key>().unwrap();
                key.insert_before(world, parent, before_node_id)
            }),
            set_visibility_fn: Some(|key, world, hidden, _state_node_id| {
                let key = key.downcast_ref::<V::Key>().unwrap();
                key.set_visibility(world, hidden)
            }),
            state_node_id: Some(|key| {
                let key = key.downcast_ref::<V::Key>().unwrap();
                key.state_node_id()
            }),
            reserve_key: Some(|world, will_rebuild| {
                let key = V::Key::reserve_key(world, will_rebuild);
                DynamicMutableViewKey::new::<V>(key)
            }),
            first_node_id: Some(|key, world| {
                let key = key.downcast_ref::<V::Key>().unwrap();
                key.first_node_id(world)
            }),
        },
    );
}

pub trait ErasureView<R>: Send + 'static
where
    R: Renderer,
{
    fn as_any(&self) -> &(dyn Any + Send);
    fn build(
        self: Box<Self>,
        ctx: ViewCtx<R>,
        reserve_key: Option<ErasureViewKey<R>>,
        will_rebuild: bool,
    ) -> ErasureViewKey<R>;

    fn rebuild(self: Box<Self>, ctx: ViewCtx<R>, key: ErasureViewKey<R>);
}

pub trait CloneableErasureView<R>: ErasureView<R>
where
    R: Renderer,
{
    fn as_dynamic(&self) -> &dyn ErasureView<R>;
    fn into_dynamic(self: Box<Self>) -> Box<dyn ErasureView<R>>;
    fn clone(&self) -> Box<dyn CloneableErasureView<R>>;
}

pub type BoxedErasureView<R> = Box<dyn ErasureView<R>>;
pub type BoxedCloneableErasureView<R> = Box<dyn CloneableErasureView<R>>;

pub trait IntoViewErasureExt<R> {
    /// # Safety
    /// The returned view must be the same type as the view that was
    /// .
    unsafe fn into_erasure_view(self) -> BoxedErasureView<R>
    where
        R: Renderer;
}

impl<R, V> IntoViewErasureExt<R> for V
where
    R: Renderer,
    V: IntoView<R>,
{
    unsafe fn into_erasure_view(self) -> BoxedErasureView<R>
    where
        R: Renderer,
    {
        Box::new(self.into_view())
    }
}

pub trait IntoViewCloneableErasureExt<R> {
    
    /// .
    ///
    /// # Safety
    /// The returned view must be the same type as the view that was
    /// .
    unsafe fn into_cloneable_erasure_view(self) -> BoxedCloneableErasureView<R>
    where
        R: Renderer;
}

impl<R, V> IntoViewCloneableErasureExt<R> for V
where
    R: Renderer,
    V: IntoView<R>,
    V::View: Clone,
{
    unsafe fn into_cloneable_erasure_view(self) -> BoxedCloneableErasureView<R>
    where
        R: Renderer,
    {
        Box::new(self.into_view())
    }
}

impl<R> Clone for BoxedCloneableErasureView<R>
where
    R: Renderer,
{
    fn clone(&self) -> Self {
        CloneableErasureView::clone(self.deref())
    }
}

impl<R, V> CloneableErasureView<R> for V
where
    R: Renderer,
    V: View<R> + Clone,
{
    fn as_dynamic(&self) -> &dyn ErasureView<R> {
        self
    }

    fn into_dynamic(self: Box<Self>) -> Box<dyn ErasureView<R>> {
        self
    }

    fn clone(&self) -> Box<dyn CloneableErasureView<R>> {
        let v = self.clone();
        Box::new(v)
    }
}

#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[derive(Clone, Debug)]
pub struct ErasureViewKey<R>
where
    R: Renderer,
{
    state_node_id: Option<RendererNodeId<R>>,
    is_reserve: bool,
}

impl<R> ErasureViewKey<R>
where
    R: Renderer,
{
    pub fn new<K>(state_node_id: Option<RendererNodeId<R>>, is_reserve: bool) -> Self
    where
        K: ViewKey<R>,
    {
        Self {
            state_node_id,
            is_reserve,
        }
    }
}

impl<R> Hash for ErasureViewKey<R>
where
    R: Renderer,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state_node_id.hash(state);
        self.is_reserve.hash(state);
    }
}

impl<R> ErasureViewKey<R>
where
    R: Renderer,
{
    pub fn get_view_key<K>(&self, world: &RendererWorld<R>) -> Option<K>
    where
        K: ViewKey<R>,
    {
        self.get_node_id_and_dyn_view_key(world)
            .and_then(|(_, n)| n.downcast::<K>().ok().map(|n| *n))
    }
    pub fn get_node_id_and_dyn_view_key(
        &self,
        world: &RendererWorld<R>,
    ) -> Option<(RendererNodeId<R>, Box<(dyn Any + Send + Sync)>)> {
        self.state_node_id.as_ref().and_then(|node_id| {
            R::get_state_ref::<ErasureViewKeyViewState>(world, node_id)
                .map(|n| {
                    let view_key = &**n.view_key.as_ref().unwrap();
                    n.clone_fn.unwrap()(view_key)
                })
                .map(|n| (node_id.clone(), n))
        })
    }
}

impl<R> ViewKey<R> for ErasureViewKey<R>
where
    R: Renderer,
{
    fn remove(self, world: &mut RendererWorld<R>) {
        let Some((state_node_id, view_key)) = self.get_node_id_and_dyn_view_key(world) else {
            return;
        };
        let erasure_fns = get_erasure_view_fns::<R>(world, &state_node_id);
        erasure_fns.remove_fn.unwrap()(view_key, world, &state_node_id);
    }

    fn insert_before(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
    ) {
        let Some((state_node_id, view_key)) = self.get_node_id_and_dyn_view_key(world) else {
            return;
        };
        let erasure_fns = get_erasure_view_fns::<R>(world, &state_node_id);
        erasure_fns.insert_before_fn.unwrap()(
            &*view_key,
            world,
            parent,
            before_node_id,
            &state_node_id,
        );
    }

    fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool) {
        let Some((state_node_id, view_key)) = self.get_node_id_and_dyn_view_key(world) else {
            return;
        };
        let erasure_fns = get_erasure_view_fns::<R>(world, &state_node_id);
        erasure_fns.set_visibility_fn.unwrap()(&*view_key, world, hidden, &state_node_id);
    }

    fn state_node_id(&self) -> Option<RendererNodeId<R>> {
        self.state_node_id.clone()
    }

    fn reserve_key(world: &mut RendererWorld<R>, _will_rebuild: bool) -> Self {
        ErasureViewKey {
            state_node_id: Some(R::spawn_data_node(world)),
            is_reserve: true,
        }
    }

    fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
        let (state_node_id, view_key) = self.get_node_id_and_dyn_view_key(world)?;
        let erasure_fns = get_erasure_view_fns::<R>(world, &state_node_id);
        erasure_fns.first_node_id.unwrap()(&*view_key, world)
    }
}

#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(type_path = false)
)]
pub struct ErasureViewKeyViewState {
    view_key: Option<Box<dyn Any + Sync + Send>>,
    #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
    clone_fn: Option<fn(key: &dyn Any) -> Box<dyn Any + Send + Sync>>,
}

impl ErasureViewKeyViewState {
    pub fn new<R, K>(view_key: K) -> Self
    where
        R: Renderer,
        K: ViewKey<R>,
    {
        Self {
            view_key: Some(Box::new(view_key) as _),
            clone_fn: Some(|view_key| {
                let key: &K = view_key.downcast_ref::<K>().unwrap();
                Box::new(key.clone()) as _
            }),
        }
    }
}

impl<R, V> ErasureView<R> for V
where
    R: Renderer,
    V: View<R>,
{
    fn as_any(&self) -> &(dyn Any + Send) {
        self
    }

    fn build(
        self: Box<Self>,
        ctx: ViewCtx<R>,
        reserve_key: Option<ErasureViewKey<R>>,
        will_rebuild: bool,
    ) -> ErasureViewKey<R> {
        let key = (*self).build(
            ViewCtx {
                world: &mut *ctx.world,
                parent: ctx.parent.clone(),
            },
            None,
            will_rebuild,
        );
        let state_node_id = if let Some(node_id) = key.state_node_id() {
            let state_node_id = if let Some(reserve_key) = reserve_key {
                assert!(reserve_key.is_reserve);
                assert!(reserve_key.state_node_id.is_some());
                reserve_key.state_node_id.unwrap()
            } else {
                node_id
            };
            set_erasure_view_fns::<R, V>(&mut *ctx.world, &state_node_id);
            if will_rebuild {
                R::set_state(ctx.world, &state_node_id, ErasureViewKeyViewState::new(key));
            }
            Some(state_node_id)
        } else {
            if let Some(_reserve_key) = reserve_key {
                R::remove_node(ctx.world, &_reserve_key.state_node_id.unwrap());
            }
            None
        };

        ErasureViewKey {
            state_node_id,
            is_reserve: false,
        }
    }

    fn rebuild(self: Box<Self>, ctx: ViewCtx<R>, key: ErasureViewKey<R>) {
        let Some(view_key) = key.get_view_key::<V::Key>(ctx.world) else {
            return;
        };
        (*self).rebuild(ctx, view_key);
    }
}

impl<R> View<R> for BoxedErasureView<R>
where
    R: Renderer,
{
    type Key = ErasureViewKey<R>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        ErasureView::build(self, ctx, reserve_key, will_rebuild)
    }

    fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key) {
        ErasureView::rebuild(self, ctx, key);
    }
}

impl<R> View<R> for BoxedCloneableErasureView<R>
where
    R: Renderer,
{
    type Key = ErasureViewKey<R>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        ErasureView::build(self, ctx, reserve_key, will_rebuild)
    }

    fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key) {
        ErasureView::rebuild(self, ctx, key);
    }
}

impl<R> IntoView<R> for BoxedErasureView<R>
where
    R: Renderer,
{
    type View = Self;

    fn into_view(self) -> Self {
        self
    }
}

impl<R> IntoView<R> for BoxedCloneableErasureView<R>
where
    R: Renderer,
{
    type View = Self;

    fn into_view(self) -> Self {
        self
    }
}
/*
pub trait ErasureViewMember<R>: Send + 'static
    where
        R: Renderer,
{
    fn build(self: Box<Self>, ctx: ViewMemberCtx<R>, will_rebuild: bool);
    fn rebuild(self: Box<Self>, ctx: ViewMemberCtx<R>);
}

pub trait ViewMemberExt<R: Renderer>: ViewMember<R> {
    fn into_erasure_view_member(self) -> Box<dyn ErasureViewMember<R>>
        where
            Self: Sized,
    {
        Box::new(self)
    }
}

impl<R: Renderer, VM: ViewMember<R>> ViewMemberExt<R> for VM {}


#[derive(Clone)]
pub struct UnBuildFnState<R: Renderer>(Option<fn(ViewMemberCtx<R>)>);

impl<R: Renderer> UnBuildFnState<R> {
    pub fn new(f: fn(ViewMemberCtx<R>)) -> Self {
        Self(Some(f))
    }
    pub fn call(self, ctx: ViewMemberCtx<R>) {
        self.0.unwrap()(ctx)
    }
}

impl<R: Renderer, T> ErasureViewMember<R> for T
    where
        T: ViewMember<R>,
{
    fn build(self: Box<Self>, mut ctx: ViewMemberCtx<R>, will_rebuild: bool) {
        ctx.set_view_member_state(UnBuildFnState::new(|ctx| T::unbuild(ctx)));
        T::build(*self, ctx, will_rebuild)
    }

    fn rebuild(self: Box<Self>, ctx: ViewMemberCtx<R>) {
        T::rebuild(*self, ctx)
    }
}

impl<R: Renderer> ViewMember<R> for Box<dyn ErasureViewMember<R>> {
    fn count() -> u8 {
        todo!()
    }

    fn unbuild(mut ctx: ViewMemberCtx<R>) {
        let Some(f) = ctx.view_member_state_mut::<UnBuildFnState<R>>().cloned() else {
            panic!("no found unbuild_fn!")
        };
        f.call(ctx);
    }

    fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
        let type_id = self.type_id();
        ErasureViewMember::<R>::build(
            self,
            ViewMemberCtx {
                index: ctx.index,
                type_id,
                world: &mut *ctx.world,
                node_id: ctx.node_id,
            },
            will_rebuild,
        )
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        let type_id = self.type_id();
        ErasureViewMember::<R>::rebuild(
            self,
            ViewMemberCtx {
                index: ctx.index,
                type_id,
                world: &mut *ctx.world,
                node_id: ctx.node_id,
            },
        )
    }
}*/
