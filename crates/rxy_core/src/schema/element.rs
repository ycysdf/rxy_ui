use rxy_macro::IntoView;

use crate::schema::view::SchemaView;
use crate::{ConstIndex, ElementSoloView, ElementView, IntoCloneableView, IntoSchemaProp, IntoView, MemberOwner, Renderer, RendererNodeId, Schema, SchemaProps, SoloView, View, ViewCtx, ViewKeyOrDataNodeId, ViewMember, ViewMemberCtx};

#[derive(IntoView)]
pub struct ElementSchemaView<R, U, VM = (), P = (), M = ()>
    where
        R: Renderer,
        VM: ViewMember<R>,
        U: Schema<R>,
        U::View: ElementView<R>,
        P: SchemaProps<R>,
        M: Send + 'static,
{
    members: VM,
    schema_view: SchemaView<R, U, P, M>,
}

impl<R, U, M> ElementSchemaView<R, U, (), (), M>
    where
        R: Renderer,
        U: Schema<R>,
        U::View: ElementView<R>,
        M: Send + 'static,
{
    #[inline]
    pub fn new(u: U) -> ElementSchemaView<R, U, (), (), M> {
        ElementSchemaView {
            members: (),
            schema_view: SchemaView::new(u),
        }
    }
}

impl<R, U, VM, P, M> ElementSchemaView<R, U, VM, P, M>
    where
        R: Renderer,
        VM: ViewMember<R>,
        U: Schema<R>,
        U::View: ElementView<R>,
        P: SchemaProps<R>,
        M: Send + 'static,
{
    #[inline(always)]
    pub fn map<MU>(self, f: impl FnOnce(U) -> MU) -> ElementSchemaView<R, MU, VM, P, M>
        where
            MU: Schema<R>,
            MU::View: ElementView<R>,
    {
        ElementSchemaView {
            members: self.members,
            schema_view: self.schema_view.map(f),
        }
    }

    #[inline(always)]
    pub fn indexed_slot<const I: usize>(self, view: impl IntoView<R>) -> Self
    {
        ElementSchemaView {
            members: self.members,
            schema_view: self.schema_view.indexed_slot::<I>(view),
        }
    }

    #[inline(always)]
    pub fn cloneable_indexed_slot<const I: usize>(self, view: impl IntoCloneableView<R>) -> Self
    {
        ElementSchemaView {
            members: self.members,
            schema_view: self.schema_view.cloneable_indexed_slot::<I>(view),
        }
    }

    #[inline(always)]
    pub fn set_indexed_prop<const I: usize, ISP, T>(
        self,
        value: ISP,
    ) -> ElementSchemaView<R, U, VM, P::Props<ConstIndex<I, ISP::Prop>>, M>
        where
            P::Props<ConstIndex<I, ISP::Prop>>: SchemaProps<R>,
            ISP: IntoSchemaProp<R, T>,
            T: Send + 'static,
    {
        ElementSchemaView {
            members: self.members,
            schema_view: self.schema_view.set_indexed_prop::<I, ISP, T>(value),
        }
    }

    #[inline(always)]
    pub fn set_static_indexed_prop<const I: usize, ISP, IT>(self, value: ISP) -> Self
        where
            ISP: IntoSchemaProp<R, IT>,
            IT: Send + 'static,
    {
        ElementSchemaView {
            members: self.members,
            schema_view: self.schema_view.set_static_indexed_prop::<I, ISP, IT>(value),
        }
    }
}

impl<R, U, VM, P, M> ElementView<R> for ElementSchemaView<R, U, VM, P, M>
    where
        R: Renderer,
        VM: ViewMember<R>,
        U: Schema<R>,
        U::View: ElementView<R>,
        P: SchemaProps<R>,
        M: Send + 'static,
{
    fn element_node_id(key: &Self::Key) -> &RendererNodeId<R> {
        U::View::element_node_id(&key.key)
    }
}

impl<R, U, VM, P, M> SoloView<R> for ElementSchemaView<R, U, VM, P, M>
    where
        R: Renderer,
        VM: ViewMember<R>,
        U: Schema<R>,
        U::View: ElementSoloView<R>,
        P: SchemaProps<R>,
        M: Send + 'static,
{
    fn node_id(key: &Self::Key) -> &RendererNodeId<R> {
        U::View::node_id(&key.key)
    }
}


impl<R, U, VM, P, M> View<R> for ElementSchemaView<R, U, VM, P, M>
    where
        R: Renderer,
        VM: ViewMember<R>,
        U: Schema<R>,
        U::View: ElementView<R>,
        P: SchemaProps<R>,
        M: Send + 'static,
{
    type Key = ViewKeyOrDataNodeId<R, <U::View as View<R>>::Key>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        let key = View::build(
            self.schema_view,
            ViewCtx {
                world: &mut *ctx.world,
                parent: ctx.parent,
            },
            reserve_key,
            will_rebuild,
        );
        self.members.build(
            ViewMemberCtx {
                index: <U::View as MemberOwner<R>>::VM::count(),
                type_id: core::any::TypeId::of::<VM>(),
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

impl<R, VM, U, P, M> MemberOwner<R> for ElementSchemaView<R, U, VM, P, M>
    where
        R: Renderer,
        VM: ViewMember<R>,
        U: Schema<R>,
        U::View: ElementView<R>,
        P: SchemaProps<R>,
        M: Send + 'static,
{
    type E = <U::View as MemberOwner<R>>::E;
    type VM = VM;
    type AddMember<AddedMember: ViewMember<R>> = ElementSchemaView<R, U, (VM, AddedMember), P, M>;
    type SetMembers<Members: ViewMember<R> + MemberOwner<R>> =
    ElementSchemaView<R, U, Members, P, M>;

    fn member<T>(self, member: T) -> Self::AddMember<T>
        where
            (VM, T): ViewMember<R>, T: ViewMember<R>
    {
        ElementSchemaView {
            members: (self.members, member),
            schema_view: self.schema_view,
        }
    }

    fn members<T>(self, members: T) -> Self::SetMembers<(T, )>
        where
            T: ViewMember<R>,
    {
        ElementSchemaView {
            members: (members, ),
            schema_view: self.schema_view,
        }
    }
}

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
//     OVM: Send + 'static,
//     T: SoloViewComponentType<R>,
// {
//     type View = T::View;
//
//     fn view(&mut self, ctx: SchemaCtx<R, Self>) -> Self::View {
//         self.0.view(ctx.cast())
//     }
// }
//
// pub trait SoloViewComponentType<R>: Send + 'static
// where
//     R: Renderer,
// {
//     type View: ElementView<R, VM = OVM>;
//     fn view(&mut self, ctx: SchemaCtx<R, Self>) -> Self::View;
// }
//
// pub trait DynSoloViewComponentType<R>: Send + 'static
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
//     OVM: Send + 'static,
//     M: Send + 'static,
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
