use crate::{BevyRenderer, IntoViewAttrMember};
use rxy_bevy_element::{ElementUnitAttr, WorldViewAttrExt};
use rxy_core::{rx, ViewMember, ViewMemberCtx};
use std::marker::PhantomData;

pub struct IntoAttrMemberWrapper<T, M>(T, PhantomData<M>);

impl<IA, F, EA> IntoViewAttrMember<EA> for rxy_core::Reactive<F, IA>
where
    EA: ElementUnitAttr,
    F: Fn() -> IA + Send + 'static,
    IA: IntoViewAttrMember<EA> + Send + 'static,
{
    type Attr = IntoAttrMemberWrapper<rxy_core::Reactive<F, IA>, EA>;

    fn into_attr(self) -> Self::Attr {
        IntoAttrMemberWrapper(self, Default::default())
    }
}

impl<EA, F, IA> ViewMember<BevyRenderer> for IntoAttrMemberWrapper<rxy_core::Reactive<F, IA>, EA>
where
    EA: ElementUnitAttr,
    F: Fn() -> IA + Send + 'static,
    IA: IntoViewAttrMember<EA> + Send + 'static,
{
    fn count() -> u8 {
        IA::Attr::count()
    }

    #[inline(always)]
    fn unbuild(ctx: ViewMemberCtx<BevyRenderer>) {
        ctx.world.unbuild_attr::<EA>(ctx.node_id);
    }

    #[inline(always)]
    fn build(self, ctx: ViewMemberCtx<BevyRenderer>, will_rebuild: bool) {
        let reactive = rx(move || self.0 .0().into_attr());
        reactive.build(ctx, will_rebuild);
    }

    #[inline(always)]
    fn rebuild(self, ctx: ViewMemberCtx<BevyRenderer>) {
        let reactive = rx(move || self.0 .0().into_attr());
        reactive.rebuild(ctx);
    }
}

macro_rules! impl_into_view_attr_member_for_signal {
    ($ty:ty) => {
        paste::paste!{
            impl<IA, EA> IntoViewAttrMember<EA> for $ty::<IA>
            where
                EA: ElementUnitAttr,
                IA: IntoViewAttrMember<EA> + Clone + Send + Sync + 'static,
            {
                type Attr = IntoAttrMemberWrapper<$ty::<IA>, EA>;

                fn into_attr(self) -> Self::Attr {
                    IntoAttrMemberWrapper(self, Default::default())
                }
            }

            impl<EA, IA> ViewMember<BevyRenderer> for IntoAttrMemberWrapper<$ty::<IA>, EA>
            where
                EA: ElementUnitAttr,
                IA: IntoViewAttrMember<EA> + Clone + Send + Sync + 'static,
            {
                fn count() -> u8 {
                    IA::Attr::count()
                }

                #[inline(always)]
                fn unbuild(ctx: ViewMemberCtx<BevyRenderer>) {
                    ctx.world.unbuild_attr::<EA>(ctx.node_id);
                }

                #[inline(always)]
                fn build(self, ctx: ViewMemberCtx<BevyRenderer>, will_rebuild: bool) {
                    use xy_reactive::prelude::SignalGet;
                    let reactive = rx(move || self.0.get().into_attr());
                    reactive.build(ctx, will_rebuild);
                }

                #[inline(always)]
                fn rebuild(self, ctx: ViewMemberCtx<BevyRenderer>) {
                    use xy_reactive::prelude::SignalGet;
                    let reactive = rx(move || self.0.get().into_attr());
                    reactive.rebuild(ctx);
                }
            }
        }
    };
}

impl_into_view_attr_member_for_signal!(xy_reactive::prelude::Memo);
impl_into_view_attr_member_for_signal!(xy_reactive::prelude::ReadSignal);
impl_into_view_attr_member_for_signal!(xy_reactive::prelude::RwSignal);

/*pub trait ViewAttrMember: ViewMember<BevyRenderer> {
    type EA: ElementUnitAttr;
}

impl<EA, VM> ViewAttrMember for futures_lite::stream::Boxed<VM>
    where
        EA: ElementUnitAttr,
        VM: ViewAttrMember<EA = EA>,
{
    type EA = EA;
}


impl<EA, F, IA> ViewAttrMember for IntoAttrMemberWrapper<rxy_core::Reactive<F, IA>, EA>
    where
        EA: ElementUnitAttr,
        F: Fn() -> IA + Send + 'static,
        IA: IntoViewAttrMember<EA> + Send + 'static,
{
    type EA = EA;
}
impl<EA, IA> ViewAttrMember for IntoAttrMemberWrapper<xy_reactive::prelude::Memo<IA>, EA>
    where
        EA: ElementUnitAttr,
        IA: IntoViewAttrMember<EA> + Clone + Send + Sync + 'static,
{
    type EA = EA;
}
*/
