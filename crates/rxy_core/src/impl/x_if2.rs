use crate::renderer::DeferredNodeTreeScoped;
use alloc::boxed::Box;
use core::marker::PhantomData;

use crate::{
    member_builder, schema_view, ConstIndex, Either, EitherExt, FnSchema, IntoSchemaProp, IntoView,
    MaybeSend, MutableView, NodeTree, RebuildFnReceiver, Renderer, SchemaView, ToMutableWrapper,
    View, ViewCtx, ViewMember, ViewMemberCtx, VirtualContainer, XNest,
};

pub struct XIf<R, C, V, V2 = ()>
where
    R: Renderer,
    V: IntoView<R> + Clone,
    V2: IntoView<R> + Clone,
{
    view: V,
    else_view: V2,
    condition: C,
    _marker: PhantomData<R>,
}

impl<R, C, V> XIf<R, C, V, ()>
where
    R: Renderer,
    C: MaybeSend + 'static,
    V: IntoView<R> + Clone,
{
    pub fn else_view<V2: IntoView<R> + Clone>(self, else_view: V2) -> XIf<R, C, V, V2> {
        XIf {
            view: self.view,
            else_view,
            condition: self.condition,
            _marker: Default::default(),
        }
    }
}

impl<R, V, V2, C> View<R> for XIf<R, C, V, V2>
where
    R: Renderer,
    V: IntoView<R> + Clone + MaybeSend,
    V2: IntoView<R> + Clone + MaybeSend,
    C: XNest<R, Inner = bool> + MaybeSend + 'static,
    C::MapInnerTo<()>: ViewMember<R>,
{
    type Key = ();

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        let parent = ctx.parent.clone();
        let world_scoped = ctx.world.deferred_world_scoped();
        // todo: use reserve_key as placeholder
        let member = self.condition.map_inner_to(move |condition| {
            // view build
            let view = if condition {
                self.view.clone().either_left()
            } else {
                self.else_view.clone().either_right()
            }
            .into_view();
            world_scoped.scoped(move |world| {
                let key = view.build(  ViewCtx { world, parent }, None, will_rebuild);
            });
        });
        member.build(
            ViewMemberCtx {
                index: 0,
                world: ctx.world,
                node_id: ctx.parent,
            },
            will_rebuild,
        )
    }

    fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key) {
        todo!()
    }
}

pub fn x_if<R, IV, C>(condition: C, v: IV) -> XIf<R, C, IV>
where
    R: Renderer,
    C: IntoSchemaProp<R, bool> + MaybeSend + 'static,
    IV: IntoView<R> + MaybeSend + Clone,
{
    XIf {
        view: v,
        else_view: (),
        condition,
        _marker: Default::default(),
    }
}
//
// pub fn x_if_else<R, V, EV, C>(
//     condition: C,
//     v: V,
//     else_view: EV,
// ) -> SchemaView<
//     R,
//     FnSchema<
//         impl FnOnce(RebuildFnReceiver<R, bool>) -> IfResultView<R, V, EV> + MaybeSend,
//         (RebuildFnReceiver<R, bool>,),
//     >,
//     (ConstIndex<0, C::Prop>,),
// >
// where
//     R: Renderer,
//     V: IntoView<R> + Clone + MaybeSend,
//     EV: IntoView<R> + Clone + MaybeSend,
//     C: IntoSchemaProp<R, bool> + 'static,
// {
//     schema_view(
//         move |condition: RebuildFnReceiver<R, bool>| {
//             condition.map(move |condition| {
//                 (if condition {
//                     v.clone().either_left()
//                 } else {
//                     else_view.clone().either_right()
//                 })
//                 .into_view()
//             })
//         },
//         (),
//     )
//     .set_indexed_prop::<0, C, bool>(condition)
// }
