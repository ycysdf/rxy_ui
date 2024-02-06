// use crate::{BevyRenderer};
// use rxy_core::{rx, ViewMember, ViewMemberCtx, ViewMemberIndex};
// use std::marker::PhantomData;
//
// pub struct IntoAttrMemberWrapper<T, M>(T, PhantomData<M>);
//
// impl<EA, F, IA> IntoViewAttrMember<EA> for IntoAttrMemberWrapper<rxy_core::Reactive<F, IA>, EA>
// where
//     EA: ElementUnitAttr,
//     F: Fn() -> IA + Send + 'static,
//     IA: IntoViewAttrMember<EA> + Send + 'static,
// {
//     type Attr = Self;
//
//     fn into_attr(self) -> Self::Attr {
//         self
//     }
//
//     type OtherAttr<OEA: ElementUnitAttr<Value = <EA>::Value>> = IntoAttrMemberWrapper<
//         rxy_core::Reactive<Box<dyn Fn() -> IA::OtherAttr<OEA> + Send>, IA::OtherAttr<OEA>>,
//         OEA,
//     >;
//
//     fn into_other_attr<OEA: ElementUnitAttr<Value = <EA>::Value>>(self) -> Self::OtherAttr<OEA> {
//         IntoAttrMemberWrapper(
//             rx(Box::new(move || self.0 .0().into_other_attr())),
//             Default::default(),
//         )
//     }
// }
//
// // impl<EA, T> IntoViewAttrMember<EA> for IntoAttrMemberWrapper<T>
// // where
// //     EA: ElementUnitAttr,
// //     T: IntoViewAttrMember<EA> + ViewMember<BevyRenderer>,
// // {
// //     type Attr = T;
//
// //     fn into_attr(self) -> Self::Attr {
// //         self.0
// //     }
//
// //     type OtherAttr<OEA: ElementUnitAttr<Value = <EA>::Value>> = T::OtherAttr<OEA>;
//
// //     fn into_other_attr<OEA: ElementUnitAttr<Value = <EA>::Value>>(self) -> Self::OtherAttr<OEA> {
// //         self.0.into_other_attr::<OEA>()
// //     }
// // }
//
// impl<IA, F, EA> IntoViewAttrMember<EA> for rxy_core::Reactive<F, IA>
// where
//     EA: ElementUnitAttr,
//     F: Fn() -> IA + Send + 'static,
//     IA: IntoViewAttrMember<EA> + Send + 'static,
// {
//     type Attr = IntoAttrMemberWrapper<rxy_core::Reactive<F, IA>, EA>;
//
//     fn into_attr(self) -> Self::Attr {
//         IntoAttrMemberWrapper(self, Default::default())
//     }
//
//     type OtherAttr<OEA: ElementUnitAttr<Value = <EA>::Value>> = IntoAttrMemberWrapper<
//         rxy_core::Reactive<Box<dyn Fn() -> IA::OtherAttr<OEA> + Send>, IA::OtherAttr<OEA>>,
//         OEA,
//     >;
//
//     fn into_other_attr<OEA: ElementUnitAttr<Value = <EA>::Value>>(self) -> Self::OtherAttr<OEA> {
//         IntoAttrMemberWrapper(
//             rx(Box::new(move || self.0().into_other_attr())),
//             Default::default(),
//         )
//     }
// }
//
// impl<EA, F, IA> ViewMember<BevyRenderer> for IntoAttrMemberWrapper<rxy_core::Reactive<F, IA>, EA>
// where
//     EA: ElementUnitAttr,
//     F: Fn() -> IA + Send + 'static,
//     IA: IntoViewAttrMember<EA> + Send + 'static,
// {
//     fn count() -> ViewMemberIndex {
//         IA::Attr::count()
//     }
//
//     #[inline(always)]
//     fn unbuild(ctx: ViewMemberCtx<BevyRenderer>, view_removed: bool) {
//         if view_removed {
//             return;
//         }
//         ctx.world.unbuild_attr::<EA>(ctx.node_id);
//     }
//
//     #[inline(always)]
//     fn build(self, ctx: ViewMemberCtx<BevyRenderer>, will_rebuild: bool) {
//         let reactive = rx(move || self.0 .0().into_attr());
//         reactive.build(ctx, will_rebuild);
//     }
//
//     #[inline(always)]
//     fn rebuild(self, ctx: ViewMemberCtx<BevyRenderer>) {
//         let reactive = rx(move || self.0 .0().into_attr());
//         reactive.rebuild(ctx);
//     }
// }
//
// macro_rules! impl_into_view_attr_member_for_signal {
//     ($ty:ty) => {
//         paste::paste!{
//             impl<IA, EA> IntoViewAttrMember<EA> for $ty::<IA>
//             where
//                 EA: ElementUnitAttr,
//                 IA: IntoViewAttrMember<EA> + Clone + Send + Sync + 'static,
//             {
//                 type Attr = IntoAttrMemberWrapper<$ty::<IA>, EA>;
//
//                 fn into_attr(self) -> Self::Attr {
//                     IntoAttrMemberWrapper(self, Default::default())
//                 }
//
//                 type OtherAttr<OEA: ElementUnitAttr<Value = <EA>::Value>> = IntoAttrMemberWrapper<rxy_core::Reactive<Box<dyn Fn() -> IA::OtherAttr<OEA> + Send>,IA::OtherAttr<OEA>>, OEA>;
//
//                 fn into_other_attr<OEA: ElementUnitAttr<Value = <EA>::Value>>(self) -> Self::OtherAttr<OEA> {
//                     use xy_reactive::prelude::SignalGet;
//                     IntoAttrMemberWrapper(rx(Box::new(move || self.get().into_other_attr::<OEA>())), Default::default())
//                 }
//             }
//
//             impl<EA, IA> IntoViewAttrMember<EA> for IntoAttrMemberWrapper<$ty::<IA>, EA>
//             where
//                 EA: ElementUnitAttr,
//                 IA: IntoViewAttrMember<EA> + Clone + Send + Sync + 'static,
//             {
//                 type Attr = IntoAttrMemberWrapper<$ty::<IA>, EA>;
//
//                 fn into_attr(self) -> Self::Attr {
//                     self
//                 }
//
//                 type OtherAttr<OEA: ElementUnitAttr<Value = <EA>::Value>> = IntoAttrMemberWrapper<rxy_core::Reactive<Box<dyn Fn() -> IA::OtherAttr<OEA> + Send>,IA::OtherAttr<OEA>>, OEA>;
//
//                 fn into_other_attr<OEA: ElementUnitAttr<Value = <EA>::Value>>(self) -> Self::OtherAttr<OEA> {
//                     self.0.into_other_attr::<OEA>()
//                 }
//             }
//
//             impl<EA, IA> ViewMember<BevyRenderer> for IntoAttrMemberWrapper<$ty::<IA>, EA>
//             where
//                 EA: ElementUnitAttr,
//                 IA: IntoViewAttrMember<EA> + Clone + Send + Sync + 'static,
//             {
//                 fn count() -> rxy_core::ViewMemberIndex {
//                     IA::Attr::count()
//                 }
//
//                 #[inline(always)]
//                 fn unbuild(ctx: ViewMemberCtx<BevyRenderer>, view_removed: bool) {
//                     if view_removed {
//                         return;
//                     }
//                     ctx.world.unbuild_attr::<EA>(ctx.node_id);
//                 }
//
//                 #[inline(always)]
//                 fn build(self, ctx: ViewMemberCtx<BevyRenderer>, will_rebuild: bool) {
//                     use xy_reactive::prelude::SignalGet;
//                     let reactive = rx(move || self.0.get().into_attr());
//                     reactive.build(ctx, will_rebuild);
//                 }
//
//                 #[inline(always)]
//                 fn rebuild(self, ctx: ViewMemberCtx<BevyRenderer>) {
//                     use xy_reactive::prelude::SignalGet;
//                     let reactive = rx(move || self.0.get().into_attr());
//                     reactive.rebuild(ctx);
//                 }
//             }
//         }
//     };
// }
//
// impl_into_view_attr_member_for_signal!(xy_reactive::prelude::Memo);
// impl_into_view_attr_member_for_signal!(xy_reactive::prelude::ReadSignal);
// impl_into_view_attr_member_for_signal!(xy_reactive::prelude::RwSignal);
