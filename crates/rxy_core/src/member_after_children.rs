use crate::{
    member_builder, BuildFlags, ElementView, MaybeSend, MemberOwner, Renderer, RendererNodeId,
    SoloView, View, ViewCtx, ViewMember, ViewMemberCtx, ViewMemberIndex, XBuilder,
};
use rxy_macro::IntoView;

#[derive(Clone, IntoView, Debug)]
pub struct MemberAfterChildren<R, EV, VM>
where
    R: Renderer,
    EV: ElementView<R>,
    VM: ViewMember<R>,
{
    pub element_view: EV,
    pub view_members: VM,
    _marker: core::marker::PhantomData<R>,
}

impl<R, EV, VM> View<R> for MemberAfterChildren<R, EV, VM>
where
    R: Renderer,
    EV: ElementView<R>,
    VM: ViewMember<R>,
{
    type Key = EV::Key;

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        let member_count = self.element_view.member_count();
        let key = self.element_view.build(
            ViewCtx {
                world: &mut *ctx.world,
                parent: ctx.parent,
            },
            reserve_key,
            will_rebuild,
        );
        self.view_members.build(
            ViewMemberCtx {
                index: member_count,
                world: ctx.world,
                node_id: EV::element_node_id(&key).clone(),
            },
            will_rebuild,
        );
        key
    }

    fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key) {
        self.view_members.rebuild(ViewMemberCtx {
            index: self.element_view.member_count(),
            world: &mut *ctx.world,
            node_id: EV::element_node_id(&key).clone(),
        });
        self.element_view.rebuild(ctx, key);
    }
}

impl<R, EV, VM> SoloView<R> for MemberAfterChildren<R, EV, VM>
where
    R: Renderer,
    EV: ElementView<R>,
    VM: ViewMember<R>,
{
    fn node_id(key: &Self::Key) -> &RendererNodeId<R> {
        EV::node_id(key)
    }
}

impl<R, EV, VM> ElementView<R> for MemberAfterChildren<R, EV, VM>
where
    R: Renderer,
    EV: ElementView<R>,
    VM: ViewMember<R>,
{
    type E = EV::E;
    type AddMember<T: ViewMember<R>> = MemberAfterChildren<R, EV::AddMember<T>, VM>;
    type SetMembers<T: ViewMember<R> + MemberOwner<R>> =
        MemberAfterChildren<R, EV::SetMembers<T>, VM>;

    fn member_count(&self) -> ViewMemberIndex {
        self.element_view.member_count()
    }

    fn member<T>(self, member: T) -> Self::AddMember<T>
    where
        (VM, T): ViewMember<R>,
        T: ViewMember<R>,
    {
        MemberAfterChildren {
            element_view: self.element_view.member(member),
            view_members: self.view_members,
            _marker: Default::default(),
        }
    }

    fn members<T>(self, members: T) -> Self::SetMembers<(T,)>
    where
        T: ViewMember<R>,
    {
        MemberAfterChildren {
            element_view: self.element_view.members(members),
            view_members: self.view_members,
            _marker: Default::default(),
        }
    }
}

pub trait MemberAfterChildrenExt<R>: ElementView<R>
where
    R: Renderer,
{
    #[inline]
    fn member_after_children<T>(self, member: T) -> MemberAfterChildren<R, Self, T>
    where
        T: ViewMember<R>,
    {
        MemberAfterChildren {
            element_view: self,
            view_members: member,
            _marker: Default::default(),
        }
    }

    #[inline]
    fn on_build_after_children<F, T>(self, f: F) -> MemberAfterChildren<R, Self, XBuilder<R, F>>
    where
        F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> T + MaybeSend + 'static,
        T: ViewMember<R>,
    {
        self.member_after_children(member_builder(f))
    }
}

impl<R, EV> MemberAfterChildrenExt<R> for EV
where
    R: Renderer,
    EV: ElementView<R>,
{
}
