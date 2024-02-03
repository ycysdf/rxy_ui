use alloc::boxed::Box;
use core::marker::PhantomData;

use crate::{
    ConstIndex, Either, EitherExt, FnSchema, IntoSchemaProp, IntoView, MaybeSend, RebuildFnReceiver,
    Renderer, schema_view, SchemaView, ToMutableWrapper, VirtualContainer,
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
/*
    impl<R, C, V, V2> IfView<R, C, V, V2>
    where
        R: Renderer,
        C: MaybeSend + 'static,
        V: IntoView<R> + Clone,
        V2: IntoView<R> + Clone,
    {
pub fn else_if<EV, EC>(self, view: EV, c: EC) -> IfView<R, C, V, IfView<R, EC, EV, ()>>
        where
            EV: IntoView<R>,
        {
            IfView {
                view: self.view,
                else_view: IfView {
                    view,
                    else_view: (),
                    condition: c,
                    _marker: Default::default(),
                },
                condition: self.condition,
                _marker: Default::default(),
            }
        }
    }
*/

pub type IfResultView<R, V, EV> = RebuildFnReceiver<
    R,
    VirtualContainer<
        R,
        Either<
            ToMutableWrapper<<V as IntoView<R>>::View>,
            ToMutableWrapper<<EV as IntoView<R>>::View>,
        >,
    >,
>;

#[cfg(feature = "send_sync")]
pub type BoxedRebuildFnReceiver<R, IV, EV> =
    Box<dyn FnOnce(RebuildFnReceiver<R, bool>) -> IfResultView<R, IV, EV> + MaybeSend>;
#[cfg(not(feature = "send_sync"))]
pub type BoxedRebuildFnReceiver<R, IV, EV> =
    Box<dyn FnOnce(RebuildFnReceiver<R, bool>) -> IfResultView<R, IV, EV>>;

impl<R, C, IV, EV> IntoView<R> for XIf<R, C, IV, EV>
where
    R: Renderer,
    C: IntoSchemaProp<R, bool> + MaybeSend + 'static,
    IV: IntoView<R> + MaybeSend + Clone,
    EV: IntoView<R> + MaybeSend + Clone,
{
    type View = SchemaView<
        R,
        FnSchema<BoxedRebuildFnReceiver<R, IV, EV>, (RebuildFnReceiver<R, bool>,)>,
        (ConstIndex<0, C::Prop>,),
    >;

    fn into_view(self) -> Self::View {
        x_if_else(self.condition, self.view, self.else_view).map(|n| n.map(|f| Box::new(f) as _))
    }
}

// #[cfg_attr(feature = "dyn", force_into_dynamic_view)]
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

pub fn x_if_else<R, V, EV, C>(
    condition: C,
    v: V,
    else_view: EV,
) -> SchemaView<
    R,
    FnSchema<
        impl FnOnce(RebuildFnReceiver<R, bool>) -> IfResultView<R, V, EV> + MaybeSend,
        (RebuildFnReceiver<R, bool>,),
    >,
    (ConstIndex<0, C::Prop>,),
>
where
    R: Renderer,
    V: IntoView<R> + Clone + MaybeSend,
    EV: IntoView<R> + Clone + MaybeSend,
    C: IntoSchemaProp<R, bool> + 'static,
{
    schema_view(
        move |condition: RebuildFnReceiver<R, bool>| {
            condition.map(move |condition| {
                (if condition {
                    v.clone().either_left()
                } else {
                    else_view.clone().either_right()
                })
                .into_view()
            })
        },
        (),
    )
    .set_indexed_prop::<0, C, bool>(condition)
}
