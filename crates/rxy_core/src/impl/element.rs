use core::fmt::Debug;
use core::marker::PhantomData;
use core::{any::TypeId, hash::Hash};

use crate::{ElementSoloView, ElementView, IntoView, MemberOwner, NodeTree, Renderer, RendererElementType, RendererNodeId, SoloView, View, ViewCtx, ViewKey, ViewMember, ViewMemberCtx};

#[derive(Clone)]
pub struct Element<R, E, VM> {
    pub members: VM,
    pub _marker: PhantomData<(R, E)>,
}

impl<R, E> Default for Element<R, E, ()>
where
    R: Renderer,
{
    fn default() -> Self {
        Self {
            members: (),
            _marker: Default::default(),
        }
    }
}

impl<R, E, VM> MemberOwner<R> for Element<R, E, VM>
where
    R: Renderer,
    E: RendererElementType<R>,
    VM: ViewMember<R>,
{
    type E = E;
    type VM = VM;
    type AddMember<T: ViewMember<R>> = Element<R, E, (VM, T)>;
    type SetMembers<T: ViewMember<R> + MemberOwner<R>> = Element<R, E, T>;

    fn member<T>(self, member: T) -> Self::AddMember<T>
    where
        (VM, T): ViewMember<R>,
        T: ViewMember<R>,
    {
        Element {
            members: (self.members, member),
            _marker: self._marker,
        }
    }

    fn members<T>(self, members: T) -> Self::SetMembers<(T,)>
    where
        T: ViewMember<R>,
    {
        Element {
            members: (members,),
            _marker: self._marker,
        }
    }
}

pub type ElementStateKey<R> = <R as Renderer>::NodeId;

impl<R, E, VM> SoloView<R> for Element<R, E, VM>
where
    E: RendererElementType<R>,
    R: Renderer,
    VM: ViewMember<R>,
{
    fn node_id(key: &Self::Key) -> &RendererNodeId<R> {
        &key.0
    }
}
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct ElementViewKey<R, VM>(
    pub RendererNodeId<R>,
    #[cfg_attr(feature = "bevy_reflect", reflect(ignore))] PhantomData<VM>,
)
where
    R: Renderer,
    VM: ViewMember<R>;

impl<R, VM> Debug for ElementViewKey<R, VM>
where
    R: Renderer,
    VM: ViewMember<R>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ElementViewKey").field(&self.0).finish()
    }
}

unsafe impl<R, VM> Send for ElementViewKey<R, VM>
where
    R: Renderer,
    VM: ViewMember<R>,
{
}

unsafe impl<R, VM> Sync for ElementViewKey<R, VM>
where
    R: Renderer,
    VM: ViewMember<R>,
{
}

impl<R, VM> Clone for ElementViewKey<R, VM>
where
    R: Renderer,
    VM: ViewMember<R>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), Default::default())
    }
}

impl<R, VM> Hash for ElementViewKey<R, VM>
where
    R: Renderer,
    VM: ViewMember<R>,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<R, VM> ViewKey<R> for ElementViewKey<R, VM>
where
    R: Renderer,
    VM: ViewMember<R>,
{
    #[inline]
    fn remove(self, world: &mut crate::RendererWorld<R>) {
        VM::unbuild(
            ViewMemberCtx {
                index: 0,
                world,
                node_id: self.0.clone(),
            },
            true,
        );
        self.0.remove(world);
    }

    #[inline]
    fn insert_before(
        &self,
        world: &mut crate::RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
    ) {
        self.0.insert_before(world, parent, before_node_id);
    }

    fn set_visibility(&self, world: &mut crate::RendererWorld<R>, hidden: bool) {
        self.0.set_visibility(world, hidden);
    }

    fn state_node_id(&self) -> Option<RendererNodeId<R>> {
        Some(self.0.clone())
    }

    fn reserve_key(world: &mut crate::RendererWorld<R>, will_rebuild: bool) -> Self {
        Self(
            <RendererNodeId<R> as ViewKey<R>>::reserve_key(world, will_rebuild),
            Default::default(),
        )
    }

    fn first_node_id(&self, world: &crate::RendererWorld<R>) -> Option<RendererNodeId<R>> {
        self.0.first_node_id(world)
    }
}

impl<R, E, VM> View<R> for Element<R, E, VM>
where
    E: RendererElementType<R>,
    R: Renderer,
    VM: ViewMember<R>,
{
    type Key = ElementViewKey<R, VM>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        let spawned_node_id = {
            let parent = ctx.parent.clone();
            ctx.world.spawn_node::<E>(Some(parent), reserve_key.map(|n| n.0))
        };
        self.members.build(
            ViewMemberCtx {
                index: 0,
                world: &mut *ctx.world,
                node_id: spawned_node_id.clone(),
            },
            will_rebuild,
        );
        ElementViewKey(spawned_node_id, Default::default())
    }

    fn rebuild(self, ctx: ViewCtx<R>, state_key: Self::Key) {
        self.members.rebuild(ViewMemberCtx {
            index: 0,
            world: ctx.world,
            node_id: state_key.0,
        });
    }
}

impl<R, E, VM> IntoView<R> for Element<R, E, VM>
where
    E: RendererElementType<R>,
    R: Renderer,
    VM: ViewMember<R>,
{
    type View = Element<R, E, VM>;

    fn into_view(self) -> Self::View {
        self
    }
}
