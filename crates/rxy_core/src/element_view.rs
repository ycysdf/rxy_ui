use crate::{IntoView, MaybeSend, MemberOwner, Renderer, RendererNodeId, SoloView, View, ViewMember, ViewMemberIndex};

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
    #[inline]
    fn element_node_id(key: &Self::Key) -> &RendererNodeId<R> {
        A::node_id(key)
    }
}*/

pub trait ElementView<R>: SoloView<R> + View<R>
where
    R: Renderer,
{
    fn element_node_id(key: &Self::Key) -> &RendererNodeId<R> {
        Self::node_id(key)
    }

    type E: MaybeSend + 'static;
    type AddMember<VM: ViewMember<R>>: ElementView<R>;
    type SetMembers<VM: ViewMember<R> + MemberOwner<R>>: ElementView<R>;
    fn member_count(&self)-> ViewMemberIndex;
    fn member<VM>(self, member: VM) -> Self::AddMember<VM>
    where
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

    #[inline]
    fn into_element_view(self) -> Self::View {
        self.into_view()
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
