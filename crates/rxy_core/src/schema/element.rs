use rxy_macro::IntoView;

use crate::schema::view::RendererSchemaView;
use crate::{
    schema_view_build, ConstIndex, ElementView, IntoCloneableView, IntoSchemaProp, IntoView,
    MaybeSend, MemberOwner, Renderer, RendererNodeId, Schema, SchemaProps, SoloView, View, ViewCtx,
    ViewKeyOrDataNodeId, ViewMember, ViewMemberCtx, ViewMemberIndex,
};

#[derive(IntoView)]
pub struct RendererSchemaElementView<R, U, VM = (), P = (), M = ()>
where
    R: Renderer,
    VM: ViewMember<R>,
    U: Schema<R>,
    U::View: ElementView<R>,
    P: SchemaProps<R>,
    M: MaybeSend + 'static,
{
    members: VM,
    schema_view: RendererSchemaView<R, U, P, M>,
}

impl<R, U, M> RendererSchemaElementView<R, U, (), (), M>
where
    R: Renderer,
    U: Schema<R>,
    U::View: ElementView<R>,
    M: MaybeSend + 'static,
{
    #[inline]
    pub fn new(u: U) -> RendererSchemaElementView<R, U, (), (), M> {
        RendererSchemaElementView {
            members: (),
            schema_view: RendererSchemaView::new(u),
        }
    }
}

impl<R, U, VM, P, M> RendererSchemaElementView<R, U, VM, P, M>
where
    R: Renderer,
    VM: ViewMember<R>,
    U: Schema<R>,
    U::View: ElementView<R>,
    P: SchemaProps<R>,
    M: MaybeSend + 'static,
{
    #[inline]
    pub fn map<MU>(self, f: impl FnOnce(U) -> MU) -> RendererSchemaElementView<R, MU, VM, P, M>
    where
        MU: Schema<R>,
        MU::View: ElementView<R>,
    {
        RendererSchemaElementView {
            members: self.members,
            schema_view: self.schema_view.map(f),
        }
    }

    #[inline]
    pub fn indexed_slot<const I: usize>(self, view: impl IntoView<R>) -> Self {
        RendererSchemaElementView {
            members: self.members,
            schema_view: self.schema_view.indexed_slot::<I>(view),
        }
    }

    #[inline]
    pub fn cloneable_indexed_slot<const I: usize>(self, view: impl IntoCloneableView<R>) -> Self {
        RendererSchemaElementView {
            members: self.members,
            schema_view: self.schema_view.cloneable_indexed_slot::<I>(view),
        }
    }

    #[inline]
    pub fn set_indexed_prop<const I: usize, ISP, T>(
        self,
        value: ISP,
    ) -> RendererSchemaElementView<R, U, VM, P::Props<ConstIndex<I, ISP::Prop>>, M>
    where
        P::Props<ConstIndex<I, ISP::Prop>>: SchemaProps<R>,
        ISP: IntoSchemaProp<R, T>,
        T: MaybeSend + 'static,
    {
        RendererSchemaElementView {
            members: self.members,
            schema_view: self.schema_view.set_indexed_prop::<I, ISP, T>(value),
        }
    }

    #[inline]
    pub fn set_static_indexed_prop<const I: usize, ISP, IT>(self, value: ISP) -> Self
    where
        ISP: IntoSchemaProp<R, IT>,
        IT: MaybeSend + 'static,
    {
        RendererSchemaElementView {
            members: self.members,
            schema_view: self
                .schema_view
                .set_static_indexed_prop::<I, ISP, IT>(value),
        }
    }
}

impl<R, U, VM, P, M> ElementView<R> for RendererSchemaElementView<R, U, VM, P, M>
where
    R: Renderer,
    VM: ViewMember<R>,
    U: Schema<R>,
    U::View: ElementView<R>,
    P: SchemaProps<R>,
    M: MaybeSend + 'static,
{
    fn element_node_id(key: &Self::Key) -> &RendererNodeId<R> {
        U::View::element_node_id(&key.key)
    }

    type E = <U::View as ElementView<R>>::E;
    type AddMember<AddedMember: ViewMember<R>> =
        RendererSchemaElementView<R, U, (VM, AddedMember), P, M>;
    type SetMembers<Members: ViewMember<R> + MemberOwner<R>> =
        RendererSchemaElementView<R, U, Members, P, M>;

    fn member_count(&self) -> ViewMemberIndex {
        VM::count()
    }

    fn member<T>(self, member: T) -> Self::AddMember<T>
    where
        (VM, T): ViewMember<R>,
        T: ViewMember<R>,
    {
        RendererSchemaElementView {
            members: (self.members, member),
            schema_view: self.schema_view,
        }
    }

    fn members<T>(self, members: T) -> Self::SetMembers<(T,)>
    where
        T: ViewMember<R>,
    {
        RendererSchemaElementView {
            members: (members,),
            schema_view: self.schema_view,
        }
    }
}

impl<R, U, VM, P, M> SoloView<R> for RendererSchemaElementView<R, U, VM, P, M>
where
    R: Renderer,
    VM: ViewMember<R>,
    U: Schema<R>,
    U::View: SoloView<R> + ElementView<R>,
    P: SchemaProps<R>,
    M: MaybeSend + 'static,
{
    fn node_id(key: &Self::Key) -> &RendererNodeId<R> {
        U::View::node_id(&key.key)
    }
}

impl<R, U, VM, P, M> View<R> for RendererSchemaElementView<R, U, VM, P, M>
where
    R: Renderer,
    VM: ViewMember<R>,
    U: Schema<R>,
    U::View: ElementView<R>,
    P: SchemaProps<R>,
    M: MaybeSend + 'static,
{
    type Key = ViewKeyOrDataNodeId<R, <U::View as View<R>>::Key>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        let (key, member_count) = schema_view_build(
            self.schema_view,
            ViewCtx {
                world: &mut *ctx.world,
                parent: ctx.parent,
            },
            reserve_key,
            will_rebuild,
            Some(|n: &U::View| n.member_count()),
        );
        self.members.build(
            ViewMemberCtx {
                index: member_count.unwrap(),
                world: ctx.world,
                node_id: U::View::element_node_id(&key.key).clone(),
            },
            will_rebuild,
        );
        key
    }

    fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key) {
        self.schema_view.rebuild(ctx, key)
    }
}

// impl<R, VM, U, P, M> MemberOwner<R> for ElementSchemaView<R, U, VM, P, M>
// where
//     R: Renderer,
//     VM: ViewMember<R>,
//     U: Schema<R>,
//     U::View: ElementView<R>,
//     P: SchemaProps<R>,
//     M: MaybeSend + 'static,
// {
// }

// pub struct SoloWrapper<T>(pub T, PhantomData<OVM>);
//
// impl<T> SoloWrapper<T> {
//     pub fn new(t: T) -> Self {
//         SoloWrapper(t, Default::default())
//     }
// }
//
// pub type DynSoloViewComponent<R, U> =
//     SoloViewComponent<R, DynViewComponentWrapper<U>, (), Box<dyn DynamicViewMember<R>>>;
//
// impl<R, T> ViewComponentType<R> for SoloWrapper<T>
// where
//     R: Renderer,
//     OVM: MaybeSend + 'static,
//     T: SoloViewComponentType<R>,
// {
//     type View = T::View;
//
//     fn view(&mut self, ctx: SchemaCtx<R, Self>) -> Self::View {
//         self.0.view(ctx.cast())
//     }
// }
//
// pub trait SoloViewComponentType<R>: MaybeSend + 'static
// where
//     R: Renderer,
// {
//     type View: ElementView<R, VM = OVM>;
//     fn view(&mut self, ctx: SchemaCtx<R, Self>) -> Self::View;
// }
//
// pub trait DynSoloViewComponentType<R>: MaybeSend + 'static
// where
//     R: Renderer,
// {
//     type E: RendererElementType<R>;
//     fn view(
//         &mut self,
//         ctx: SchemaCtx<R, impl IntoView<R>, Self>,
//     ) -> impl IntoElementView<R, E = Self::E>;
// }
//
// impl<R, T> SoloViewComponentType<R, Box<dyn DynamicViewMember<R>>> for DynViewComponentWrapper<T>
// where
//     T: DynSoloViewComponentType<R>,
//     R: Renderer,
// {
//     type View = ElementViewVariant<R, T::E>;
//
//     fn view(&mut self, ctx: SchemaCtx<R, Self>) -> Self::View {
//         self.0.view(ctx.cast()).into_element_view()
//     }
// }
//
// impl<R, M, T> SoloViewComponentType<R> for SoloWrapper<T, M>
// where
//     R: Renderer,
//     OVM: MaybeSend + 'static,
//     M: MaybeSend + 'static,
//     T: SoloViewComponentType<R>,
// {
//     type View = T::View;
//
//     fn view(&mut self, ctx: SchemaCtx<R, Self>) -> Self::View {
//         self.0.view(ctx.cast())
//     }
// }
//
// #[inline]
// pub fn dyn_solo_view_component<R, U>(u: U, content: SC) -> DynSoloViewComponent<R, U>
// where
//     R: Renderer,
//     U: DynSoloViewComponentType<R>,
//     SC: SlotContents<R>,
// {
//     solo_view_component::<R, DynViewComponentWrapper<U>, Box<dyn DynamicViewMember<R>>>(
//         DynViewComponentWrapper(u),
//         content,
//     )
// }
