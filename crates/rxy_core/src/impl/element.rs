use core::any::TypeId;
use core::marker::PhantomData;

use crate::{
    ElementSoloView, ElementView, IntoView, MemberOwner, Renderer, RendererElementType,
    RendererNodeId, SoloView, View, ViewCtx, ViewMember, ViewMemberCtx,
};

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
        key
    }
}

impl<R, E, VM> View<R> for Element<R, E, VM>
where
    E: RendererElementType<R>,
    R: Renderer,
    VM: ViewMember<R>,
{
    type Key = RendererNodeId<R>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        let spawned_node_id = {
            let parent = ctx.parent.clone();
            R::spawn_node::<E>(&mut *ctx.world, Some(parent), reserve_key)
        };
        self.members.build(
            ViewMemberCtx {
                index: 0,
                type_id: TypeId::of::<VM>(),
                world: &mut *ctx.world,
                node_id: spawned_node_id.clone(),
            },
            will_rebuild,
        );
        spawned_node_id
    }

    fn rebuild(self, ctx: ViewCtx<R>, state_key: Self::Key) {
        self.members.rebuild(ViewMemberCtx {
            index: 0,
            type_id: TypeId::of::<VM>(),
            world: ctx.world,
            node_id: state_key,
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
