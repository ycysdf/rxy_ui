use core::marker::PhantomData;

use crate::{BoxedErasureView, ElementSoloView, ElementView, IntoView, IntoViewErasureExt, MemberOwner, NodeTree, Renderer, RendererNodeId, SoloView, View, ViewCtx, ViewMember};

pub fn view_children<R, V, CV>(view: V, children: CV) -> ElementViewChildren<V, CV::View, R>
    where
        R: Renderer,
        V: SoloView<R>,
        CV: IntoView<R>,
{
    ElementViewChildren {
        view,
        children: children.into_view(),
        _marker: Default::default(),
    }
}

#[derive(Clone)]
pub struct ElementViewChildrenState<K> {
    pub children_key: K,
}

#[derive(Clone)]
pub struct ElementViewChildren<V, CV, R> {
    pub view: V,
    pub children: CV,
    pub _marker: PhantomData<R>,
}

impl<V, CV, R> ElementViewChildren<V, CV, R>
    where
        R: Renderer,
{
    pub fn new(view: V, children: CV) -> Self {
        Self {
            view,
            children,
            _marker: Default::default(),
        }
    }

    #[cfg(not(feature = "view_erasure"))]
    pub fn children<CV2>(self, children: CV2) -> ElementViewChildren<V, CV2, R>
        where
            CV: IntoView<R>,
    {
        ElementViewChildren {
            view: self.view,
            children,
            _marker: Default::default(),
        }
    }

    #[inline(always)]
    #[cfg(feature = "view_erasure")]
    pub fn children<CV2>(self, children: CV2) -> ElementViewChildren<V, BoxedErasureView<R>, R>
        where
            CV2: IntoView<R>,
    {
        self.erasure_children(children)
    }

    
    #[inline(always)]
    pub fn erasure_children<CV2>(self, children: CV2) -> ElementViewChildren<V, BoxedErasureView<R>, R>
        where
            CV2: IntoView<R>,
    {
        ElementViewChildren {
            view: self.view,
            children: unsafe { children.into_erasure_view() },
            _marker: Default::default(),
        }
    }
}

impl<V, CV, R> SoloView<R> for ElementViewChildren<V, CV, R>
    where
        V: SoloView<R>,
        CV: View<R>,
        R: Renderer,
{
    fn node_id(key: &Self::Key) -> &RendererNodeId<R> {
        V::node_id(key)
    }
}

impl<V, CV, R> View<R> for ElementViewChildren<V, CV, R>
    where
        V: SoloView<R>,
        CV: View<R>,
        R: Renderer,
{
    type Key = V::Key;

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        let key = self.view.build(
            ViewCtx {
                world: &mut *ctx.world,
                parent: ctx.parent,
            },
            reserve_key,
            will_rebuild,
        );

        let children_key = self.children.build(
            ViewCtx {
                world: ctx.world,
                parent: V::node_id(&key).clone(),
            },
            None,
            will_rebuild,
        );
        if will_rebuild {
            ctx.world.set_node_state(
                V::node_id(&key),
                ElementViewChildrenState { children_key },
            );
        }
        key
    }

    fn rebuild(self, ctx: ViewCtx<R>, state_key: Self::Key) {
        {
            let Some(children_key) = ctx.world.get_node_state_ref::<ElementViewChildrenState<CV::Key>>(
                V::node_id(&state_key),
            )
                .map(|n| n.children_key.clone()) else {
                panic!("children_key not found!")
            };
            let children_ctx = ViewCtx {
                world: &mut *ctx.world,
                parent: V::node_id(&state_key).clone(),
            };
            self.children.rebuild(children_ctx, children_key);
        }

        self.view.rebuild(ctx, state_key);
    }
}

impl<V, CV, R> IntoView<R> for ElementViewChildren<V, CV, R>
    where
        V: SoloView<R>,
        CV: View<R>,
        R: Renderer,
{
    type View = Self;

    fn into_view(self) -> Self::View {
        self
    }
}

impl<R, VM, CV, V> MemberOwner<R> for ElementViewChildren<V, CV, R>
    where
        R: Renderer,
        VM: ViewMember<R>,
        CV: View<R>,
        V: MemberOwner<R, VM=VM>,
{
    type E = V::E;
    type VM = VM;
    type AddMember<T: ViewMember<R>> = ElementViewChildren<V::AddMember<T>, CV, R>;
    type SetMembers<T: ViewMember<R> + MemberOwner<R>> =
    ElementViewChildren<V::SetMembers<T>, CV, R>;
    fn member<T>(self, member: T) -> Self::AddMember<T>
        where
            (VM, T): ViewMember<R>,
            T: ViewMember<R>,
    {
        ElementViewChildren::new(self.view.member(member), self.children)
    }

    fn members<T>(self, members: T) -> Self::SetMembers<(T, )>
        where
            T: ViewMember<R>,
    {
        ElementViewChildren::new(self.view.members(members), self.children)
    }
}

impl<R, VM, CV, V> ElementView<R> for ElementViewChildren<V, CV, R>
    where
        V: SoloView<R> + MemberOwner<R, VM=VM>,
        CV: View<R>,
        R: Renderer,
        VM: ViewMember<R>,
{
    fn element_node_id(key: &Self::Key) -> &RendererNodeId<R> {
        V::node_id(key)
    }
}

// #[derive(Clone)]
// pub struct ElementSoloViewMemberOwnerWrapper<T>(pub T);
//
// pub trait ElementSoloViewMemberOwner<R: Renderer>: MaybeSend + 'static {
//     type E: MaybeSend + 'static;
//     type VM: ViewMember<R>;
//     type AddMember<T: ViewMember<R>>: ElementSoloView<R>;
//     type SetMembers<T: ViewMember<R>>: ElementSoloView<R>;
//     fn member<T>(self, member: T) -> Self::AddMember<T>
//         where
//             (Self::VM, T): ViewMember<R>,
//             T: ViewMember<R>;
//     fn members<T: ViewMember<R>>(self, members: T) -> Self::SetMembers<(T,)>
//         where
//             T: ViewMember<R>;
// }
//
// impl<R, T> MemberOwner<R> for ElementSoloViewMemberOwnerWrapper<T>
//     where
//         R: Renderer,
//         T: ElementSoloViewMemberOwner<R>,
// {
//     type E = T::E;
//     type VM = T::VM;
//     type AddMember<VM: ViewMember<R>> = T::AddMember<VM>;
//     type SetMembers<VM: ViewMember<R>> = T::SetMembers<VM>;
//
//     fn member<VM>(self, member: VM) -> Self::AddMember<VM>
//         where
//             (Self::VM, VM): ViewMember<R>,
//             VM: ViewMember<R>,
//     {
//         self.0.member(member)
//     }
//
//     fn members<VM: ViewMember<R>>(self, members: VM) -> Self::SetMembers<VM>
//         where
//             VM: ViewMember<R>,
//     {
//         self.0.members(members)
//     }
// }

/*

impl<R, T> ElementSoloViewMemberOwner<R> for T
where
    R: Renderer,
    T: MemberOwner<R>,
{
    type E = T::E;
    type VM = T::VM;
    type AddMember<VM: ViewMember<R>> = T::AddMember<VM>;
    type SetMembers<VM: ViewMember<R>> = T::SetMembers<VM>;

    fn member<VM>(self, member: VM) -> Self::AddMember<VM>
    where
        (Self::VM, VM): ViewMember<R>,
        VM: ViewMember<R>,
    {
        self.member(member)
    }

    fn members<VM: ViewMember<R>>(self, members: VM) -> Self::SetMembers<VM>
    where
        VM: ViewMember<R>,
    {
        self.members(members)
    }
}*/
