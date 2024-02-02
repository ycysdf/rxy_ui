use alloc::boxed::Box;
use core::any::{Any, TypeId};
use core::hash::{Hash, Hasher};
use core::ops::Deref;

use ahash::AHasher;

use crate::r#impl::erasure::{get_erasure_view_fns, set_erasure_view_fns, ErasureViewFns};
use crate::{
    IntoView, MutableView, MutableViewKey, Renderer, RendererNodeId,
    RendererWorld, View, ViewCtx, ViewKey, ViewMember, ViewMemberCtx, VirtualContainer,
    VirtualContainerNodeId,
};

use super::virtual_container;

pub trait CloneableDynamicView<R>: DynamicView<R>
where
    R: Renderer,
{
    fn as_dynamic(&self) -> &dyn DynamicView<R>;
    fn into_dynamic(self: Box<Self>) -> Box<dyn DynamicView<R>>;
    fn clone(&self) -> Box<dyn CloneableDynamicView<R>>;
}

pub type BoxedDynamicView<R> = Box<dyn DynamicView<R>>;
pub type BoxedDynamicViewView<R> = <Box<dyn DynamicView<R>> as IntoView<R>>::View;
pub type BoxedCloneableDynamicView<R> = Box<dyn CloneableDynamicView<R>>;

pub trait DynamicView<R>: Send + 'static
where
    R: Renderer,
{
    fn as_any(&self) -> &(dyn Any + Send);
    fn build(
        self: Box<Self>,
        ctx: ViewCtx<R>,
        state_node_id: Option<RendererNodeId<R>>,
    ) -> DynamicMutableViewKey<R>;

    fn rebuild(
        self: Box<Self>,
        ctx: ViewCtx<R>,
        key: DynamicMutableViewKey<R>,
        state_node_id: RendererNodeId<R>,
    ) -> Option<DynamicMutableViewKey<R>>;
}

impl<R> Clone for Box<dyn CloneableDynamicView<R>>
where
    R: Renderer,
{
    fn clone(&self) -> Self {
        CloneableDynamicView::clone(self.deref())
    }
}

pub trait IntoDynamicView<R>
where
    R: Renderer,
{
    fn into_dynamic(self) -> Box<dyn DynamicView<R>>;
}

impl<R, IV> IntoDynamicView<R> for IV
where
    R: Renderer,
    IV: IntoView<R>,
{
    fn into_dynamic(self) -> Box<dyn DynamicView<R>> {
        let view = self.into_view();
        view_to_dynamic(view)
    }
}

#[inline(always)]
pub fn view_to_dynamic<R, V>(view: V) -> BoxedDynamicView<R>
where
    R: Renderer,
    V: View<R>,
{
    if TypeId::of::<BoxedDynamicView<R>>() == TypeId::of::<V>() {
        // todo: nest BoxedDynamicView
        panic!("nest BoxedDynamicView!");
        // return unsafe { core::mem::transmute(view) };
    }
    Box::new(view)
}

pub trait IntoCloneableDynamicView<R>
where
    R: Renderer,
{
    fn into_cloneable_dynamic(self) -> Box<dyn CloneableDynamicView<R>>;
}

impl<R, IV> IntoCloneableDynamicView<R> for IV
where
    R: Renderer,
    IV: IntoView<R>,
    IV::View: View<R> + Clone,
{
    fn into_cloneable_dynamic(self) -> Box<dyn CloneableDynamicView<R>> {
        let view = self.into_view();
        // if TypeId::of::<BoxedCloneableDynamicView<R>>() == TypeId::of::<IV::View>() {
        //     return unsafe { core::mem::transmute(view) };
        // }
        Box::new(view)
    }
}

impl<R> MutableView<R> for Box<dyn DynamicView<R>>
where
    R: Renderer,
{
    type Key = DynamicMutableViewKey<R>;

    fn no_placeholder_when_no_rebuild() -> bool {
        true
    }

    fn build(self, ctx: ViewCtx<R>, placeholder_node_id: Option<RendererNodeId<R>>) -> Self::Key {
        DynamicView::build(self, ctx, placeholder_node_id)
    }

    fn rebuild(
        self,
        ctx: ViewCtx<R>,
        key: Self::Key,
        placeholder_node_id: RendererNodeId<R>,
    ) -> Option<Self::Key> {
        let view_type_id = self.as_any().type_id();
        if view_type_id != key.view_type_id() {
            <DynamicMutableViewKey<R> as MutableViewKey<R>>::remove(key, &mut *ctx.world);
            Some(DynamicView::build(self, ctx, Some(placeholder_node_id)))
        } else {
            DynamicView::rebuild(self, ctx, key, placeholder_node_id);
            None
        }
    }
}

impl<R> MutableView<R> for Box<dyn CloneableDynamicView<R>>
where
    R: Renderer,
{
    type Key = DynamicMutableViewKey<R>;

    fn no_placeholder_when_no_rebuild() -> bool {
        true
    }

    fn build(self, ctx: ViewCtx<R>, placeholder_node_id: Option<RendererNodeId<R>>) -> Self::Key {
        DynamicView::build(self, ctx, placeholder_node_id)
    }

    fn rebuild(
        self,
        ctx: ViewCtx<R>,
        key: Self::Key,
        placeholder_node_id: RendererNodeId<R>,
    ) -> Option<Self::Key> {
        let view_type_id = self.as_any().type_id();
        if view_type_id != key.view_type_id() {
            <DynamicMutableViewKey<R> as MutableViewKey<R>>::remove(key, &mut *ctx.world);
            Some(DynamicView::build(self, ctx, Some(placeholder_node_id)))
        } else {
            DynamicView::rebuild(self, ctx, key, placeholder_node_id);
            None
        }
    }
}

impl<R, V> DynamicView<R> for V
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
        state_node_id: Option<RendererNodeId<R>>,
    ) -> DynamicMutableViewKey<R> {
        let key = (*self).build(
            ViewCtx {
                world: &mut *ctx.world,
                parent: ctx.parent,
            },
            None,
            state_node_id.is_some(),
        );
        if let Some(state_node_id) = key.state_node_id() {
            set_erasure_view_fns::<R, V>(ctx.world, &state_node_id);
        }

        DynamicMutableViewKey::new::<V>(key)
    }

    fn rebuild(
        self: Box<Self>,
        ctx: ViewCtx<R>,
        key: DynamicMutableViewKey<R>,
        _state_node_id: RendererNodeId<R>,
    ) -> Option<DynamicMutableViewKey<R>> {
        let key = key.key_ref().downcast_ref::<V::Key>().unwrap().clone();
        (*self).rebuild(ctx, key);

        None
    }
}

impl<R, V> CloneableDynamicView<R> for V
where
    R: Renderer,
    V: View<R> + Clone,
{
    fn as_dynamic(&self) -> &dyn DynamicView<R> {
        self
    }

    fn into_dynamic(self: Box<Self>) -> Box<dyn DynamicView<R>> {
        self
    }

    fn clone(&self) -> BoxedCloneableDynamicView<R> {
        let v = self.clone();
        Box::new(v)
    }
}

pub type DynamicViewKey<R> = VirtualContainerNodeId<R, DynamicMutableViewKey<R>>;

#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[derive(Debug)]
pub struct DynamicMutableViewKey<R>
where
    R: Renderer,
{
    state_node_id: Option<RendererNodeId<R>>,
    #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
    key: Option<Box<dyn Any + Send + Sync>>,
    #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
    view_type_id: Option<TypeId>,
    #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
    clone_fn: Option<fn(key: &dyn Any) -> Box<dyn Any + Send + Sync>>,
    #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
    hash_fn: Option<fn(key: &dyn Any) -> Option<u64>>,
}

impl<R> Clone for DynamicMutableViewKey<R>
where
    R: Renderer,
{
    fn clone(&self) -> Self {
        Self {
            key: Some(self.clone_fn.unwrap()(self.key_ref())),
            view_type_id: self.view_type_id,
            clone_fn: self.clone_fn,
            hash_fn: self.hash_fn,
            state_node_id: self.state_node_id.clone(),
        }
    }
}

impl<R> Hash for DynamicMutableViewKey<R>
where
    R: Renderer,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash_fn.unwrap()(self.key_ref()).unwrap());
        self.view_type_id.hash(state)
    }
}

impl<R> DynamicMutableViewKey<R>
where
    R: Renderer,
{
    pub fn new<V>(key: V::Key) -> Self
    where
        V: View<R>,
    {
        let state_node_id = key.state_node_id();
        let reflect_key: Box<dyn Any + Send + Sync> = Box::new(key) as _;
        Self {
            state_node_id,
            key: Some(reflect_key),
            view_type_id: Some(TypeId::of::<V>()),
            clone_fn: Some(|key| {
                let key = key.downcast_ref::<V::Key>().unwrap();
                Box::new(key.clone()) as _
            }),
            hash_fn: Some(|key| {
                let key = key.downcast_ref::<V::Key>().unwrap();
                let mut hasher = AHasher::default();
                key.hash(&mut hasher);
                Some(hasher.finish())
            }),
        }
    }
    pub fn key_ref(&self) -> &dyn Any {
        self.key.as_ref().unwrap().as_ref()
    }

    pub fn view_type_id(&self) -> TypeId {
        self.view_type_id.unwrap()
    }
}

impl<R> MutableViewKey<R> for DynamicMutableViewKey<R>
where
    R: Renderer,
{
    fn remove(self, world: &mut RendererWorld<R>) {
        let Some(state_node_id) = self.state_node_id.as_ref() else {
            return;
        };
        let erasure_fns = get_erasure_view_fns::<R>(world, state_node_id);
        erasure_fns.remove_fn.unwrap()(self.key.unwrap(), world, state_node_id)
    }

    fn insert_before(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
    ) {
        let Some(state_node_id) = self.state_node_id.as_ref() else {
            return;
        };
        let erasure_fns = get_erasure_view_fns::<R>(world, state_node_id);
        erasure_fns.insert_before_fn.unwrap()(
            self.key_ref(),
            world,
            parent,
            before_node_id,
            state_node_id,
        )
    }

    fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool) {
        let Some(state_node_id) = self.state_node_id.as_ref() else {
            return;
        };
        let erasure_fns = get_erasure_view_fns::<R>(world, state_node_id);
        erasure_fns.set_visibility_fn.unwrap()(self.key_ref(), world, hidden, state_node_id)
    }

    fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
        let state_node_id = self.state_node_id.as_ref()?;
        let erasure_fns = get_erasure_view_fns::<R>(world, state_node_id);
        erasure_fns.first_node_id.unwrap()(self.key_ref(), world)
    }

    fn state_node_id(&self) -> Option<RendererNodeId<R>> {
        self.state_node_id.clone()
    }
}

impl<R> IntoView<R> for Box<dyn DynamicView<R>>
where
    R: Renderer,
{
    type View = VirtualContainer<R, Self>;

    fn into_view(self) -> Self::View {
        virtual_container(self, "[DynamicView Placeholder]")
    }
}

impl<R> IntoView<R> for Box<dyn CloneableDynamicView<R>>
where
    R: Renderer,
{
    type View = VirtualContainer<R, Self>;

    fn into_view(self) -> Self::View {
        virtual_container(self, "[DynamicView Placeholder]")
    }
}
/*
pub trait DynamicViewMember<R>: Send + 'static
where
    R: Renderer,
{
    fn build(self: Box<Self>, ctx: ViewMemberCtx<R>, will_rebuild: bool);
    fn rebuild(self: Box<Self>, ctx: ViewMemberCtx<R>);
}

pub trait ViewMemberExt<R: Renderer>: ViewMember<R> {
    fn into_dynamic(self) -> Box<dyn DynamicViewMember<R>>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

impl<R: Renderer, VM: ViewMember<R>> ViewMemberExt<R> for VM {}

impl<R: Renderer> ViewMember<R> for Box<dyn DynamicViewMember<R>> {
    fn unbuild(mut ctx: ViewMemberCtx<R>) {
        let Some(f) = ctx.view_member_state_mut::<UnBuildFnState<R>>().cloned() else {
            panic!("no found unbuild_fn!")
        };
        f.call(ctx);
    }

    fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
        let type_id = self.type_id();
        DynamicViewMember::<R>::build(
            self,
            ViewMemberCtx {
                type_id,
                world: &mut *ctx.world,
                node_id: ctx.node_id,
            },
            will_rebuild,
        )
    }

    fn rebuild(self, ctx: ViewMemberCtx<R>) {
        let type_id = self.type_id();
        DynamicViewMember::<R>::rebuild(
            self,
            ViewMemberCtx {
                type_id,
                world: &mut *ctx.world,
                node_id: ctx.node_id,
            },
        )
    }
}

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

impl<R: Renderer, T> DynamicViewMember<R> for T
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
*/
