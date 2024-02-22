use crate::{
    IntoView, MaybeSend, MemberOwner, Renderer, RendererNodeId, SoloView, View, ViewCtx,
    ViewMember, ViewMemberCtx,
};
use rxy_macro::IntoView;

/*
pub trait ElementSoloView<R>: ElementView<R> + SoloView<R>
where
    R: Renderer,
{
}

impl<R, A> ElementSoloView<R> for A
where
    R: Renderer,
    A: ElementView<R> + SoloView<R>,
{
}
impl<R, A> ElementView<R> for A
where
    R: Renderer,
    A: MemberOwner<R> + SoloView<R>,
{
    #[inline(always)]
    fn element_node_id(key: &Self::Key) -> &RendererNodeId<R> {
        A::node_id(key)
    }
}*/

pub trait ElementView<R>: SoloView<R> + View<R>
where
    R: Renderer,
{
    fn element_node_id(key: &Self::Key) -> &RendererNodeId<R>;

    type E: MaybeSend + 'static;
    type VM: ViewMember<R>;
    type AddMember<VM: ViewMember<R>>: ElementView<R>;
    type SetMembers<VM: ViewMember<R> + MemberOwner<R>>: ElementView<R>;
    fn member<VM>(self, member: VM) -> Self::AddMember<VM>
    where
        (Self::VM, VM): ViewMember<R>,
        VM: ViewMember<R>;
    fn members<VM: ViewMember<R>>(self, members: VM) -> Self::SetMembers<(VM,)>
    where
        VM: ViewMember<R>;
}

pub trait IntoElementView<R>: 'static
where
    R: Renderer,
{
    type View: ElementView<R>;
    fn into_element_view(self) -> Self::View;
}

impl<R, T> IntoElementView<R> for T
where
    R: Renderer,
    T: IntoView<R>,
    T::View: ElementView<R>,
{
    type View = T::View;

    #[inline(always)]
    fn into_element_view(self) -> Self::View {
        self.into_view()
    }
}

#[derive(Clone, Debug)]
pub struct ElementViewExtraMembers<R, EV, VM> {
    pub element_view: EV,
    pub view_members: VM,
    _marker: core::marker::PhantomData<R>,
}

impl<R, EV, VM> crate::IntoView<R> for ElementViewExtraMembers<R, EV, VM>
where
    R: Renderer,
    EV: ElementView<R>,
    VM: ViewMember<R>,
{
    type View = ElementViewExtraMembers<R, EV, VM>;
    fn into_view(self) -> Self::View {
        self
    }
}

/// Members are built after children are built
pub fn add_members<R, EV, VM>(
    element_view: EV,
    view_members: VM,
) -> ElementViewExtraMembers<R, EV::View, VM>
where
    R: Renderer,
    EV: IntoElementView<R>,
    VM: ViewMember<R>,
{
    ElementViewExtraMembers {
        element_view: element_view.into_element_view(),
        view_members,
        _marker: Default::default(),
    }
}

impl<R, EV, VM> View<R> for ElementViewExtraMembers<R, EV, VM>
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
                index: <EV::VM as ViewMember<R>>::count(),
                world: ctx.world,
                node_id: EV::element_node_id(&key).clone(),
            },
            will_rebuild,
        );
        key
    }

    fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key) {
        self.view_members.rebuild(ViewMemberCtx {
            index: <EV::VM as ViewMember<R>>::count(),
            world: &mut *ctx.world,
            node_id: EV::element_node_id(&key).clone(),
        });
        self.element_view.rebuild(ctx, key);
    }
}

impl<R, EV, VM> SoloView<R> for ElementViewExtraMembers<R, EV, VM>
where
    R: Renderer,
    EV: ElementView<R>,
    VM: ViewMember<R>,
{
    fn node_id(key: &Self::Key) -> &RendererNodeId<R> {
        EV::node_id(key)
    }
}

impl<R, EV, VM> ElementView<R> for ElementViewExtraMembers<R, EV, VM>
where
    R: Renderer,
    EV: ElementView<R>,
    VM: ViewMember<R>,
{
    fn element_node_id(key: &Self::Key) -> &RendererNodeId<R> {
        EV::element_node_id(key)
    }

    type E = EV::E;
    type VM = EV::VM;
    type AddMember<T: ViewMember<R>> = ElementViewExtraMembers<R, EV::AddMember<T>, VM>;
    type SetMembers<T: ViewMember<R> + MemberOwner<R>> =
        ElementViewExtraMembers<R, EV::SetMembers<T>, VM>;

    fn member<T>(self, member: T) -> Self::AddMember<T>
    where
        (VM, T): ViewMember<R>,
        T: ViewMember<R>,
    {
        ElementViewExtraMembers {
            element_view: self.element_view.member(member),
            view_members: self.view_members,
            _marker: Default::default(),
        }
    }

    fn members<T>(self, members: T) -> Self::SetMembers<(T,)>
    where
        T: ViewMember<R>,
    {
        ElementViewExtraMembers {
            element_view: self.element_view.members(members),
            view_members: self.view_members,
            _marker: Default::default(),
        }
    }
}
/*

impl<R, EV, VM> ElementView<R> for ElementViewExtraMembers<R, EV, VM>
where
    R: Renderer,
    EV: ElementView<R>,
    VM: ViewMember<R>,
{
    fn element_node_id(key: &Self::Key) -> &RendererNodeId<R> {
        EV::element_node_id(key)
    }
}
*/
