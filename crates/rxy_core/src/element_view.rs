use crate::{
    IntoView, MemberOwner, Renderer, RendererNodeId, SoloView, View, ViewCtx,
    ViewMember, ViewMemberCtx,
};
use rxy_macro::IntoView;

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
/*
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

pub trait ElementView<R>: MemberOwner<R> + View<R>
where
    R: Renderer,
{
    fn element_node_id(key: &Self::Key) -> &RendererNodeId<R>;
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

#[derive(IntoView, Clone, Debug)]
pub struct ElementViewExtraMembers<R, EV, VM>
where
    R: Renderer,
    EV: ElementView<R>,
    VM: ViewMember<R>,
{
    pub element_view: EV,
    pub view_members: VM,
    _marker: core::marker::PhantomData<R>,
}

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
                type_id: core::any::TypeId::of::<VM>(),
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
            type_id: core::any::TypeId::of::<VM>(),
            world: &mut *ctx.world,
            node_id: EV::element_node_id(&key).clone(),
        });
        self.element_view.rebuild(ctx, key);
    }
}
