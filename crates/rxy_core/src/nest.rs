use crate::utils::all_tuples;
use crate::MaybeSend;
use core::marker::PhantomData;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InnerIvmToVm<T, M>(pub T, PhantomData<M>);

impl<T, M> InnerIvmToVm<T, M> {
    #[inline]
    pub fn new(t: T) -> Self {
        Self(t, Default::default())
    }
}

pub trait XNestMapper<U>: XNest
where
    U: 'static,
{
    type MapInnerTo: 'static;

    fn map_inner_to(
        self,
        f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
    ) -> Self::MapInnerTo;
}

pub trait XNest {
    type Inner;

    type MapInner<M>;

    fn map_inner<M>(self) -> Self::MapInner<M>;
    fn is_static() -> bool;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XValueWrapper<T>(pub T);

#[macro_export]
macro_rules! impl_x_value_wrappers {
    ($($ty:ty),*) => {
        $(
            impl Into<XValueWrapper<Self>> for $ty
            {
                fn into(self) -> XValueWrapper<Self> {
                    XValueWrapper(self)
                }
            }
        )*
    };
}

macro_rules! impl_x_value_wrappers_for_tuple {
    ($($t:ident),*) => {
        impl<$($t),*> Into<XValueWrapper<Self>> for ($($t,)*) {
            fn into(self) -> XValueWrapper<Self> {
                XValueWrapper(self)
            }
        }
    }
}

all_tuples!(impl_x_value_wrappers_for_tuple, 1, 6, T);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapValueWrapper<T, M>(pub T, PhantomData<M>);

impl<T> XNest for T
where
    T: Into<XValueWrapper<T>> + MaybeSend + 'static,
{
    type Inner = T;
    type MapInner<M> = MapValueWrapper<T, M>;

    fn map_inner<M>(self) -> Self::MapInner<M> {
        MapValueWrapper::<T, M>(self.into().0, Default::default())
    }

    fn is_static() -> bool {
        true
    }
}

impl<T, U> XNestMapper<U> for T
where
    U: 'static,
    T: Into<XValueWrapper<T>> + MaybeSend + 'static,
{
    type MapInnerTo = U;

    fn map_inner_to(
        self,
        f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
    ) -> Self::MapInnerTo {
        f(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapToAttrMarker<EA>(PhantomData<EA>);

pub mod impl_attr {
    use crate::{
        ElementAttr, ElementAttrType, MapToAttrMarker, MapValueWrapper, MaybeSend, Renderer,
        ViewMember, ViewMemberCtx, ViewMemberIndex, ViewMemberOrigin, XNest, XNestMapper,
        XValueWrapper,
    };

    impl<R, EA> XNest for ElementAttr<R, EA>
    where
        R: Renderer,
        EA: ElementAttrType<R>,
    {
        type Inner = Self;
        type MapInner<M> = Self;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            self
        }

        fn is_static() -> bool {
            true
        }
    }

    impl<R, EA, U> XNestMapper<U> for ElementAttr<R, EA>
    where
        U: 'static,
        R: Renderer,
        EA: ElementAttrType<R>,
    {
        type MapInnerTo = U;

        fn map_inner_to(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo {
            f(self)
        }
    }

    impl<R, T, EA> ViewMemberOrigin<R> for MapValueWrapper<T, MapToAttrMarker<EA>>
    where
        R: Renderer,
        T: MaybeSend + 'static,
        T: Into<XValueWrapper<EA::Value>>,
        EA: ElementAttrType<R>,
    {
        type Origin = ElementAttr<R, EA>;
    }

    impl<R, T, EA> ViewMember<R> for MapValueWrapper<T, MapToAttrMarker<EA>>
    where
        R: Renderer,
        T: MaybeSend + 'static,
        T: Into<XValueWrapper<EA::Value>>,
        EA: ElementAttrType<R>,
    {
        #[inline]
        fn count() -> ViewMemberIndex {
            ElementAttr::<R, EA>::count()
        }

        #[inline]
        fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
            ElementAttr::<R, EA>::unbuild(ctx, view_removed);
        }

        #[inline]
        fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
            ElementAttr::<R, EA>::new(self.0.into().0).build(ctx, will_rebuild);
        }

        #[inline]
        fn rebuild(self, ctx: ViewMemberCtx<R>) {
            ElementAttr::<R, EA>::new(self.0.into().0).rebuild(ctx);
        }
    }
}

#[cfg(feature = "style")]
pub struct MapToStyleSheetsMarker<SS>(PhantomData<SS>);

#[cfg(feature = "style")]
pub mod impl_style {
    use crate::style::{ApplyStyleSheets, StyledNodeTree, StyleSheets};
    use crate::style::{StyleItemValue, StyleSheetCtx, StyleSheetItems};
    use crate::{
        ElementAttr, ElementAttrType, MapToAttrMarker, MapToStyleSheetsMarker, MapValueWrapper,
        MaybeSend, Renderer, ViewMember, ViewMemberCtx, ViewMemberIndex, ViewMemberOrigin, XNest,
        XNestMapper, XValueWrapper,
    };

    impl<T> XNest for ApplyStyleSheets<T>
    where
        T: 'static,
    {
        type Inner = Self;
        type MapInner<M> = Self;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            self
        }

        fn is_static() -> bool {
            true
        }
    }

    impl<T, U> XNestMapper<U> for ApplyStyleSheets<T>
    where
        U: 'static,
        T: 'static,
    {
        type MapInnerTo = U;

        fn map_inner_to(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo {
            f(self)
        }
    }

    impl<R, SS> ViewMemberOrigin<R> for MapValueWrapper<SS, MapToStyleSheetsMarker<SS>>
    where
        R: Renderer,
        SS: StyleSheets<R>,
    {
        type Origin = ApplyStyleSheets<SS>;
    }

    impl<R, SS> ViewMember<R> for MapValueWrapper<SS, MapToStyleSheetsMarker<SS>>
    where
        R: Renderer,
        R::NodeTree: StyledNodeTree<R>,
        SS: StyleSheets<R>,
    {
        #[inline]
        fn count() -> ViewMemberIndex {
            ApplyStyleSheets::<SS>::count()
        }

        #[inline]
        fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
            ApplyStyleSheets::<SS>::unbuild(ctx, view_removed);
        }

        #[inline]
        fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
            ApplyStyleSheets::<SS>(self.0).build(ctx, will_rebuild);
        }

        #[inline]
        fn rebuild(self, ctx: ViewMemberCtx<R>) {
            ApplyStyleSheets::<SS>(self.0).rebuild(ctx);
        }
    }

    impl<R, T, EA> StyleSheetItems<R> for MapValueWrapper<T, MapToAttrMarker<EA>>
    where
        R: Renderer,
        T: MaybeSend + 'static,
        T: Into<XValueWrapper<EA::Value>>,
        EA: ElementAttrType<R>,
    {
        #[inline]
        fn iter(self, ctx: StyleSheetCtx<R>) -> impl Iterator<Item = StyleItemValue> + 'static {
            ElementAttr::<R, EA>::new(self.0.into().0).iter(ctx)
        }
    }
}

#[cfg(feature = "xy_reactive")]
pub mod impl_reactive {
    use crate::{
        rx, InnerIvmToVm, MaybeSend, MaybeSync, Reactive, Renderer, ViewMember, ViewMemberCtx,
        ViewMemberIndex, ViewMemberOrigin, XNest, XNestMapper,
    };
    use xy_reactive::prelude::{Memo, ReadSignal, RwSignal, SignalGet};

    macro_rules! impl_x_nest_for_signal {
        ($ty:ty) => {
            impl<X> XNest for $ty
            where
                X: XNest + MaybeSend + MaybeSync + Clone + 'static,
            {
                type Inner = X::Inner;
                type MapInner<M> = InnerIvmToVm<Self, M>;

                fn map_inner<M>(self) -> Self::MapInner<M> {
                    InnerIvmToVm::new(self)
                }

                fn is_static() -> bool {
                    false
                }
            }

            impl<X, U> XNestMapper<U> for $ty
            where
                X: XNestMapper<U> + MaybeSend + MaybeSync + Clone + 'static,
                U: 'static,
            {
                type MapInnerTo =
                    Reactive<Box<dyn Fn() -> X::MapInnerTo + MaybeSend + 'static>, X::MapInnerTo>;

                fn map_inner_to(
                    self,
                    f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
                ) -> Self::MapInnerTo {
                    rx(Box::new(move || self.get().map_inner_to(f.clone())))
                }
            }
        };
    }

    impl_x_nest_for_signal!(Memo<X>);
    impl_x_nest_for_signal!(ReadSignal<X>);
    impl_x_nest_for_signal!(RwSignal<X>);

    impl<F, X> XNest for Reactive<F, X>
    where
        F: Fn() -> X + MaybeSend + 'static,
        X: XNest + MaybeSend + 'static,
    {
        type Inner = X::Inner;
        type MapInner<M> = InnerIvmToVm<Self, M>;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            InnerIvmToVm::new(self)
        }

        fn is_static() -> bool {
            false
        }
    }

    impl<F, X, U> XNestMapper<U> for Reactive<F, X>
    where
        F: Fn() -> X + MaybeSend + 'static,
        X: XNestMapper<U> + MaybeSend + 'static,
        U: 'static,
    {
        type MapInnerTo =
            Reactive<Box<dyn Fn() -> X::MapInnerTo + MaybeSend + 'static>, X::MapInnerTo>;

        fn map_inner_to(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo {
            rx(Box::new(move || self.0().map_inner_to(f.clone())))
        }
    }

    impl<R, F, X, M, VM> ViewMemberOrigin<R> for InnerIvmToVm<Reactive<F, X>, M>
    where
        R: Renderer,
        F: Fn() -> X + MaybeSend + 'static,
        X: XNest<MapInner<M> = VM> + MaybeSend + 'static,
        VM: ViewMemberOrigin<R>,
        M: MaybeSend + 'static,
    {
        type Origin = VM::Origin;
    }

    impl<R, F, X, M, VM> ViewMember<R> for InnerIvmToVm<Reactive<F, X>, M>
    where
        R: Renderer,
        F: Fn() -> X + MaybeSend + 'static,
        X: XNest<MapInner<M> = VM> + MaybeSend + 'static,
        VM: ViewMember<R>,
        M: MaybeSend + 'static,
    {
        fn count() -> ViewMemberIndex {
            VM::count()
        }

        fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
            VM::unbuild(ctx, view_removed);
        }

        fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
            rx(move || self.0 .0().map_inner::<M>()).build(ctx, will_rebuild);
        }

        fn rebuild(self, ctx: ViewMemberCtx<R>) {
            rx(move || self.0 .0().map_inner::<M>()).rebuild(ctx);
        }
    }

    impl<R, T, VM, X, M> ViewMemberOrigin<R> for InnerIvmToVm<T, M>
    where
        R: Renderer,
        T: SignalGet<Value = X> + MaybeSend + 'static,
        X: XNest<MapInner<M> = VM> + MaybeSync + Clone + 'static,
        VM: ViewMemberOrigin<R>,
        M: MaybeSend + 'static,
    {
        type Origin = VM::Origin;
    }

    impl<R, T, M, VM, X> ViewMember<R> for InnerIvmToVm<T, M>
    where
        R: Renderer,
        T: SignalGet<Value = X> + MaybeSend + 'static,
        X: XNest<MapInner<M> = VM> + MaybeSync + Clone + 'static,
        VM: ViewMember<R>,
        M: MaybeSend + 'static,
    {
        fn count() -> ViewMemberIndex {
            VM::count()
        }

        fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
            VM::unbuild(ctx, view_removed);
        }

        fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
            rx(move || self.0.get().map_inner::<M>()).build(ctx, will_rebuild);
        }

        fn rebuild(self, ctx: ViewMemberCtx<R>) {
            rx(move || self.0.get().map_inner::<M>()).rebuild(ctx);
        }
    }
}

pub mod core_impls {
    use crate::maybe_traits::{MaybeSendSyncFutureExit, MaybeSendSyncStreamExt};
    use crate::{
        x_future, BoxedFutureMaybeLocal, BoxedStreamMaybeLocal, InnerIvmToVm, MaybeSend, Renderer,
        ViewMember, ViewMemberCtx, ViewMemberIndex, ViewMemberOrigin, XFuture, XNest, XNestMapper,
        XStream,
    };
    use core::future::Future;
    use futures_lite::stream::StreamExt;
    use futures_lite::Stream;

    impl<X> XNest for Option<X>
    where
        X: XNest,
    {
        type Inner = X::Inner;
        type MapInner<M> = Option<X::MapInner<M>>;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            self.map(|n| n.map_inner::<M>())
        }

        fn is_static() -> bool {
            X::is_static()
        }
    }

    impl<X, U> XNestMapper<U> for Option<X>
    where
        U: 'static,
        X: XNestMapper<U>,
    {
        type MapInnerTo = Option<X::MapInnerTo>;
        fn map_inner_to(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo {
            self.map(move |n| n.map_inner_to(f))
        }
    }

    impl<X> XNest for BoxedStreamMaybeLocal<X>
    where
        X: XNest + 'static,
    {
        type Inner = X::Inner;
        type MapInner<M> = BoxedStreamMaybeLocal<X::MapInner<M>>;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            self.map(|n| n.map_inner::<M>()).boxed_maybe_local()
        }

        fn is_static() -> bool {
            false
        }
    }

    impl<X, U> XNestMapper<U> for BoxedStreamMaybeLocal<X>
    where
        U: 'static,
        X: XNestMapper<U> + 'static,
    {
        type MapInnerTo = BoxedStreamMaybeLocal<X::MapInnerTo>;
        fn map_inner_to(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo {
            self.map(move |n| n.map_inner_to(f.clone()))
                .boxed_maybe_local()
        }
    }

    impl<R, S, X, VM, M> ViewMemberOrigin<R> for InnerIvmToVm<XStream<S>, M>
    where
        R: Renderer,
        S: Stream<Item = X> + MaybeSend + 'static,
        X: XNest<MapInner<M> = VM> + MaybeSend + 'static,
        VM: ViewMemberOrigin<R>,
        M: MaybeSend + 'static,
    {
        type Origin = VM::Origin;
    }

    impl<R, S, M, X, VM> ViewMember<R> for InnerIvmToVm<XStream<S>, M>
    where
        R: Renderer,
        S: Stream<Item = X> + MaybeSend + 'static,
        X: XNest<MapInner<M> = VM> + MaybeSend + 'static,
        VM: ViewMember<R>,
        M: MaybeSend + 'static,
    {
        fn count() -> ViewMemberIndex {
            VM::count()
        }

        fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
            VM::unbuild(ctx, view_removed)
        }

        fn build(self, ctx: ViewMemberCtx<R>, _will_rebuild: bool) {
            let stream = self.0.map(|n| n.map_inner::<M>());
            stream.build(ctx, true);
        }

        fn rebuild(self, ctx: ViewMemberCtx<R>) {
            let stream = self.0.map(|n| n.map_inner::<M>());
            stream.rebuild(ctx);
        }
    }
    impl<S, X> XNest for XStream<S>
    where
        S: Stream<Item = X> + MaybeSend + 'static,
        X: XNest + MaybeSend + 'static,
    {
        type Inner = X::Inner;
        type MapInner<M> = InnerIvmToVm<XStream<S>, M>;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            InnerIvmToVm::new(self)
        }

        fn is_static() -> bool {
            false
        }
    }

    impl<S, X, U> XNestMapper<U> for XStream<S>
    where
        U: 'static,
        S: Stream<Item = X> + MaybeSend + 'static,
        X: XNestMapper<U> + MaybeSend + 'static,
    {
        type MapInnerTo = XStream<BoxedStreamMaybeLocal<X::MapInnerTo>>;
        fn map_inner_to(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo {
            let value = self.value.map({
                let f = f.clone();
                move |n| n.map_inner_to(f)
            });
            XStream {
                stream: self
                    .stream
                    .map(move |n| n.map_inner_to(f.clone()))
                    .boxed_maybe_local(),
                value,
                already_end: self.already_end,
            }
        }
    }

    impl<R, T, VM, X, M> ViewMemberOrigin<R> for InnerIvmToVm<XFuture<T>, M>
    where
        R: Renderer,
        T: Future<Output = X> + MaybeSend + 'static,
        VM: ViewMemberOrigin<R>,
        X: XNest<MapInner<M> = VM> + MaybeSend + 'static,
        M: MaybeSend + 'static,
    {
        type Origin = VM::Origin;
    }

    impl<R, T, VM, X, M> ViewMember<R> for InnerIvmToVm<XFuture<T>, M>
    where
        R: Renderer,
        T: Future<Output = X> + MaybeSend + 'static,
        VM: ViewMember<R>,
        X: XNest<MapInner<M> = VM> + MaybeSend + 'static,
        M: MaybeSend + 'static,
    {
        fn count() -> ViewMemberIndex {
            VM::count()
        }

        fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
            VM::unbuild(ctx, view_removed)
        }

        fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
            let reactive = x_future(async move { self.0 .0.await.map_inner::<M>() });
            reactive.build(ctx, will_rebuild)
        }

        fn rebuild(self, ctx: ViewMemberCtx<R>) {
            let reactive = x_future(async move { self.0 .0.await.map_inner::<M>() });
            reactive.rebuild(ctx)
        }
    }

    impl<F, X> XNest for XFuture<F>
    where
        F: Future<Output = X> + MaybeSend + 'static,
        X: XNest + MaybeSend + 'static,
    {
        type Inner = X::Inner;
        type MapInner<M> = InnerIvmToVm<Self, M>;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            InnerIvmToVm::new(self)
        }

        fn is_static() -> bool {
            false
        }
    }

    impl<F, X, U> XNestMapper<U> for XFuture<F>
    where
        U: 'static,
        F: Future<Output = X> + MaybeSend + 'static,
        X: XNestMapper<U> + MaybeSend + 'static,
    {
        type MapInnerTo = BoxedFutureMaybeLocal<X::MapInnerTo>;
        fn map_inner_to(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo {
            async move { self.0.await.map_inner_to(f) }.boxed_maybe_local()
        }
    }

    impl<X> XNest for BoxedFutureMaybeLocal<X>
    where
        X: XNest + 'static,
    {
        type Inner = X::Inner;
        type MapInner<M> = BoxedFutureMaybeLocal<X::MapInner<M>>;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            async move { self.await.map_inner::<M>() }.boxed_maybe_local()
        }

        fn is_static() -> bool {
            false
        }
    }

    impl<X, U> XNestMapper<U> for BoxedFutureMaybeLocal<X>
    where
        U: 'static,
        X: XNestMapper<U> + 'static,
    {
        type MapInnerTo = BoxedFutureMaybeLocal<X::MapInnerTo>;
        fn map_inner_to(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo {
            async move { self.await.map_inner_to(f) }.boxed_maybe_local()
        }
    }
}

pub mod builder {
    use crate::{
        member_builder, BuildFlags, InnerIvmToVm, MaybeSend, Renderer, ViewMember, ViewMemberCtx,
        ViewMemberIndex, ViewMemberOrigin, XBuilder, XNest, XNestMapper,
    };
    use alloc::boxed::Box;

    impl<R, F, X> XNest for XBuilder<R, F>
    where
        F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> X + MaybeSend + 'static,
        R: Renderer,
        X: XNest,
    {
        type Inner = X::Inner;
        type MapInner<M> = InnerIvmToVm<Self, M>;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            InnerIvmToVm::new(self)
        }

        fn is_static() -> bool {
            X::is_static()
        }
    }
    impl<R, F, X, U> XNestMapper<U> for XBuilder<R, F>
    where
        U: 'static,
        F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> X + MaybeSend + 'static,
        R: Renderer,
        X: XNestMapper<U>,
    {
        #[cfg(feature = "send_sync")]
        type MapInnerTo = XBuilder<
            R,
            Box<dyn FnOnce(ViewMemberCtx<R>, BuildFlags) -> X::MapInnerTo + MaybeSend + 'static>,
        >;
        #[cfg(not(feature = "send_sync"))]
        type MapInnerTo =
            XBuilder<R, Box<dyn FnOnce(ViewMemberCtx<R>, BuildFlags) -> X::MapInnerTo + 'static>>;

        #[inline]
        fn map_inner_to(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo {
            member_builder(Box::new(move |ctx, flags| {
                self.0(ctx, flags).map_inner_to(f.clone())
            }))
        }
    }

    impl<R, F, VM, X, M> ViewMemberOrigin<R> for InnerIvmToVm<XBuilder<R, F>, M>
    where
        F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> X + MaybeSend + 'static,
        R: Renderer,
        X: XNest<MapInner<M> = VM>,
        VM: ViewMemberOrigin<R>,
        M: MaybeSend + 'static,
    {
        type Origin = VM::Origin;
    }

    impl<R, F, M, VM, X> ViewMember<R> for InnerIvmToVm<XBuilder<R, F>, M>
    where
        F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> X + MaybeSend + 'static,
        R: Renderer,
        X: XNest<MapInner<M> = VM>,
        VM: ViewMember<R>,
        M: MaybeSend + 'static,
    {
        #[inline]
        fn count() -> ViewMemberIndex {
            1
        }

        #[inline]
        fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
            VM::unbuild(ctx, view_removed)
        }

        #[inline]
        fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
            member_builder(|ctx, flags| self.0 .0(ctx, flags).map_inner::<M>())
                .build(ctx, will_rebuild)
        }

        #[inline]
        fn rebuild(self, ctx: ViewMemberCtx<R>) {
            member_builder(|ctx, flags| self.0 .0(ctx, flags).map_inner::<M>()).rebuild(ctx)
        }
    }
}
