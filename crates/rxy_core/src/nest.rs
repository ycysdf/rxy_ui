use crate::utils::all_tuples;
use crate::{
    smallbox, ElementAttr, ElementAttrViewMember, MaybeSend, MaybeSync, Renderer, ViewMember,
    ViewMemberCtx, ViewMemberIndex, ViewMemberOrigin,
};
use core::marker::PhantomData;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InnerIvmToVm<T, M>(pub T, PhantomData<M>);

impl<T, M> InnerIvmToVm<T, M> {
    #[inline]
    pub fn new(t: T) -> Self {
        Self(t, Default::default())
    }
}

// pub trait Mapper<T> {
//     type To;
//     fn map(self) -> Self::To;
// }
//
// pub struct VmMapper<R>(PhantomData<R>);
//
// impl<R, T> Mapper<VmMapper<R>> for T
// where
//     R: Renderer,
//     T: XNest<R>,
//     T::MapInner<VmMapper<R>>: ViewMember<R>,
// {
//     type To = T::MapInner<VmMapper<R>>;
//
//     fn map(self) -> Self::To {
//         self.into_member()
//     }
// }

pub trait XNest<R>
where
    R: Renderer,
{
    type Inner;

    type MapInner<M>;
    type MapInnerTo<U: 'static>: 'static;

    fn map_inner<M>(self) -> Self::MapInner<M>;

    fn map_inner_to<U: 'static>(
        self,
        f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
    ) -> Self::MapInnerTo<U>;

    fn is_static() -> bool;

    // fn into_member(self) -> Self::MapInner<VmMapper<R>>
    // where
    //     // Self::InnerMember: Mapper<VmMapper<R>>,
    //     Self::MapInner<VmMapper<R>>: ViewMember<R>,
    //     Self: Sized,
    // {
    //     // self.map()
    //     self.map_inner::<VmMapper<R>>()
    // }
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

pub struct MapValueWrapper<T, M>(pub T, PhantomData<M>);

impl<R, T> XNest<R> for T
where
    R: Renderer,
    T: Into<XValueWrapper<T>> + MaybeSend + 'static,
{
    type Inner = T;
    type MapInner<M> = MapValueWrapper<T, M>;
    type MapInnerTo<U: 'static> = U;

    fn map_inner<M>(self) -> Self::MapInner<M> {
        MapValueWrapper::<T, M>(self.into().0, Default::default())
    }

    fn map_inner_to<U: 'static>(
        self,
        f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
    ) -> Self::MapInnerTo<U> {
        f(self)
    }

    fn is_static() -> bool {
        true
    }
}

pub struct MapToAttrMarker<EA>(PhantomData<EA>);

const _: () = {
    use crate::{
        ElementAttr, ElementAttrViewMember, MapValueWrapper, MaybeSend, Renderer, ViewMember,
        ViewMemberCtx, ViewMemberIndex, ViewMemberOrigin, XNest, XValueWrapper,
    };

    impl<R, EA> XNest<R> for ElementAttrViewMember<R, EA>
    where
        R: Renderer,
        EA: ElementAttr<R>,
    {
        type Inner = Self;
        type MapInner<M> = Self;
        type MapInnerTo<U: 'static> = U;

        fn map_inner<M>(self) -> Self::MapInner<M>
        where
            Self::MapInner<M>: ViewMember<R>,
        {
            self
        }

        fn map_inner_to<U: 'static>(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo<U> {
            f(self)
        }

        fn is_static() -> bool {
            true
        }
    }

    impl<R, T, EA> ViewMemberOrigin<R> for MapValueWrapper<T, MapToAttrMarker<EA>>
    where
        R: Renderer,
        T: MaybeSend + 'static,
        T: Into<XValueWrapper<EA::Value>>,
        EA: ElementAttr<R>,
    {
        type Origin = ElementAttrViewMember<R, EA>;
    }

    impl<R, T, EA> ViewMember<R> for MapValueWrapper<T, MapToAttrMarker<EA>>
    where
        R: Renderer,
        T: MaybeSend + 'static,
        T: Into<XValueWrapper<EA::Value>>,
        EA: ElementAttr<R>,
    {
        #[inline]
        fn count() -> ViewMemberIndex {
            ElementAttrViewMember::<R, EA>::count()
        }

        #[inline]
        fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
            ElementAttrViewMember::<R, EA>::unbuild(ctx, view_removed);
        }

        #[inline]
        fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
            ElementAttrViewMember::<R, EA>::new(self.0.into().0).build(ctx, will_rebuild);
        }

        #[inline]
        fn rebuild(self, ctx: ViewMemberCtx<R>) {
            ElementAttrViewMember::<R, EA>::new(self.0.into().0).rebuild(ctx);
        }
    }
};

#[cfg(feature = "style")]
pub struct MapToStyleSheetsMarker<SS>(PhantomData<SS>);

#[cfg(feature = "style")]
const _: () = {
    use crate::style::{ApplyStyleSheets, StyleSheets};
    use crate::style::{StyleItemValue, StyleSheetCtx, StyleSheetItems};
    use crate::{
        MapValueWrapper, MaybeSend, Renderer, ViewMember, ViewMemberCtx, ViewMemberIndex,
        ViewMemberOrigin, XNest,
    };

    impl<R, T> XNest<R> for ApplyStyleSheets<T>
    where
        R: Renderer,
        T: StyleSheets<R>,
    {
        type Inner = Self;
        type MapInner<M> = Self;
        type MapInnerTo<U: 'static> = U;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            self
        }

        fn map_inner_to<U: 'static>(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo<U> {
            f(self)
        }

        fn is_static() -> bool {
            true
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
        EA: ElementAttr<R>,
    {
        #[inline]
        fn iter(self, ctx: StyleSheetCtx<R>) -> impl Iterator<Item = StyleItemValue> + 'static {
            ElementAttrViewMember::<R, EA>::new(self.0.into().0).iter(ctx)
        }
    }
};

#[cfg(feature = "xy_reactive")]
const _: () = {
    use crate::{rx, Reactive};
    use xy_reactive::prelude::{Memo, ReadSignal, RwSignal, SignalGet};
    impl<R, X> XNest<R> for Memo<X>
    where
        R: Renderer,
        X: XNest<R> + MaybeSend + MaybeSync + Clone + 'static,
    {
        type Inner = X::Inner;
        type MapInner<M> = InnerIvmToVm<Self, M>;
        type MapInnerTo<U: 'static> = Memo<U>;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            InnerIvmToVm::new(self)
        }

        fn map_inner_to<U: 'static>(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo<U> {
            Memo::new(f(self.get()))
        }

        fn is_static() -> bool {
            false
        }
    }

    impl<R, X> XNest<R> for ReadSignal<X>
    where
        R: Renderer,
        X: XNest<R> + MaybeSend + MaybeSync + Clone + 'static,
    {
        type Inner = X::Inner;
        type MapInner<M> = InnerIvmToVm<Self, M>;
        type MapInnerTo<U: 'static> = Memo<U>;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            InnerIvmToVm::new(self)
        }

        fn map_inner_to<U: 'static>(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo<U> {
            Memo::new(f(self.get()))
        }

        fn is_static() -> bool {
            false
        }
    }

    impl<R, X> XNest<R> for RwSignal<X>
    where
        R: Renderer,
        X: XNest<R> + MaybeSend + MaybeSync + Clone + 'static,
    {
        type Inner = X::Inner;
        type MapInner<M> = InnerIvmToVm<Self, M>;
        type MapInnerTo<U: 'static> = Memo<U>;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            InnerIvmToVm::new(self)
        }

        fn map_inner_to<U: 'static>(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo<U> {
            Memo::new(f(self.get()))
        }

        fn is_static() -> bool {
            false
        }
    }

    impl<R, F, X> XNest<R> for Reactive<F, X>
    where
        R: Renderer,
        F: Fn() -> X + MaybeSend + 'static,
        X: XNest<R> + MaybeSend + 'static,
    {
        type Inner = X::Inner;
        type MapInner<M> = InnerIvmToVm<Self, M>;
        type MapInnerTo<U: 'static> = Memo<U>;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            InnerIvmToVm::new(self)
        }

        fn map_inner_to<U: 'static>(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo<U> {
            Memo::new(f(self.0()))
        }

        fn is_static() -> bool {
            false
        }
    }

    impl<R, F, X, M, VM> ViewMemberOrigin<R> for InnerIvmToVm<Reactive<F, X>, M>
    where
        R: Renderer,
        F: Fn() -> X + MaybeSend + 'static,
        X: XNest<R, MapInner<M> = VM> + MaybeSend + 'static,
        VM: ViewMemberOrigin<R>,
        M: MaybeSend + 'static,
    {
        type Origin = VM::Origin;
    }

    impl<R, F, X, M, VM> ViewMember<R> for InnerIvmToVm<Reactive<F, X>, M>
    where
        R: Renderer,
        F: Fn() -> X + MaybeSend + 'static,
        X: XNest<R, MapInner<M> = VM> + MaybeSend + 'static,
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
        X: XNest<R, MapInner<M> = VM> + MaybeSync + Clone + 'static,
        VM: ViewMemberOrigin<R>,
        M: MaybeSend + 'static,
    {
        type Origin = VM::Origin;
    }

    impl<R, T, M, VM, X> ViewMember<R> for InnerIvmToVm<T, M>
    where
        R: Renderer,
        T: SignalGet<Value = X> + MaybeSend + 'static,
        X: XNest<R, MapInner<M> = VM> + MaybeSync + Clone + 'static,
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
};

pub mod std_impls {
    use crate::maybe_traits::{MaybeSendSyncFutureExit, MaybeSendSyncStreamExt};
    use crate::{
        x_future, InnerIvmToVm, MaybeSend, Renderer, ViewMember, ViewMemberCtx, ViewMemberIndex,
        ViewMemberOrigin, XFuture, XNest, XStream,
    };
    use futures_lite::stream::StreamExt;
    use futures_lite::Stream;
    use std::future::Future;

    impl<R, T> XNest<R> for Option<T>
    where
        R: Renderer,
        T: XNest<R>,
    {
        type Inner = T::Inner;
        type MapInner<M> = Option<T::MapInner<M>>;
        type MapInnerTo<U: 'static> = Option<T::MapInnerTo<U>>;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            self.map(|n| n.map_inner::<M>())
        }

        fn map_inner_to<U: 'static>(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo<U> {
            self.map(|n| n.map_inner_to::<U>(f))
        }

        fn is_static() -> bool {
            T::is_static()
        }
    }

    impl<R, X> XNest<R> for futures_lite::stream::Boxed<X>
    where
        R: Renderer,
        X: XNest<R> + 'static,
    {
        type Inner = X::Inner;
        type MapInner<M> = futures_lite::stream::Boxed<X::MapInner<M>>;
        type MapInnerTo<U: 'static> = futures_lite::stream::Boxed<X::MapInnerTo<U>>;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            self.map(|n| n.map_inner::<M>()).boxed_maybe_local()
        }

        fn map_inner_to<U: 'static>(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo<U> {
            self.map(move |n| n.map_inner_to::<U>(f.clone()))
                .boxed_maybe_local()
        }

        fn is_static() -> bool {
            false
        }
    }

    impl<R, S, X, VM, M> ViewMemberOrigin<R> for InnerIvmToVm<XStream<S>, M>
    where
        R: Renderer,
        S: Stream<Item = X> + MaybeSend + 'static,
        X: XNest<R, MapInner<M> = VM> + MaybeSend + 'static,
        VM: ViewMemberOrigin<R>,
        M: MaybeSend + 'static,
    {
        type Origin = VM::Origin;
    }

    impl<R, S, M, X, VM> ViewMember<R> for InnerIvmToVm<XStream<S>, M>
    where
        R: Renderer,
        S: Stream<Item = X> + MaybeSend + 'static,
        X: XNest<R, MapInner<M> = VM> + MaybeSend + 'static,
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
    impl<R, S, X> XNest<R> for XStream<S>
    where
        R: Renderer,
        S: Stream<Item = X> + MaybeSend + 'static,
        X: XNest<R> + MaybeSend + 'static,
    {
        type Inner = X::Inner;
        type MapInner<M> = InnerIvmToVm<XStream<S>, M>;
        type MapInnerTo<U: 'static> = futures_lite::stream::Boxed<X::MapInnerTo<U>>;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            InnerIvmToVm::new(self)
        }

        fn map_inner_to<U: 'static>(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo<U> {
            self.stream
                .map(move |n| n.map_inner_to::<U>(f.clone()))
                .boxed_maybe_local()
        }

        fn is_static() -> bool {
            false
        }
    }

    impl<R, TO, VM, M> ViewMemberOrigin<R> for InnerIvmToVm<XFuture<TO>, M>
    where
        R: Renderer,
        TO: Future + MaybeSend + 'static,
        VM: ViewMemberOrigin<R>,
        TO::Output: XNest<R, MapInner<M> = VM> + MaybeSend + 'static,
        M: MaybeSend + 'static,
    {
        type Origin = VM::Origin;
    }

    impl<R, TO, VM, M> ViewMember<R> for InnerIvmToVm<XFuture<TO>, M>
    where
        R: Renderer,
        TO: Future + MaybeSend + 'static,
        VM: ViewMember<R>,
        TO::Output: XNest<R, MapInner<M> = VM> + MaybeSend + 'static,
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

    impl<R, F, X> XNest<R> for XFuture<F>
    where
        R: Renderer,
        F: Future<Output = X> + MaybeSend + 'static,
        X: XNest<R> + MaybeSend + 'static,
    {
        type Inner = X::Inner;
        type MapInner<M> = InnerIvmToVm<Self, X::MapInner<M>>;

        type MapInnerTo<U: 'static> = futures_lite::future::Boxed<X::MapInnerTo<U>>;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            InnerIvmToVm::new(self)
        }

        fn map_inner_to<U: 'static>(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo<U> {
            async move { self.0.await.map_inner_to::<U>(f) }.boxed_maybe_local()
        }

        fn is_static() -> bool {
            false
        }
    }

    impl<R, X> XNest<R> for futures_lite::future::Boxed<X>
    where
        R: Renderer,
        X: XNest<R> + MaybeSend + 'static,
    {
        type Inner = X::Inner;
        type MapInner<M> = futures_lite::future::Boxed<X::MapInner<M>>;
        type MapInnerTo<U: 'static> = futures_lite::future::Boxed<X::MapInnerTo<U>>;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            async move { self.await.map_inner::<M>() }.boxed_maybe_local()
        }

        fn map_inner_to<U: 'static>(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo<U> {
            async move { self.await.map_inner_to::<U>(f) }.boxed_maybe_local()
        }

        fn is_static() -> bool {
            false
        }
    }
}

pub mod builder {
    use crate::{
        member_builder, BuildFlags, InnerIvmToVm, MaybeSend, Renderer, ViewMember, ViewMemberCtx,
        ViewMemberIndex, ViewMemberOrigin, XBuilder, XNest,
    };

    impl<R, F, X> XNest<R> for XBuilder<R, F>
    where
        F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> X + MaybeSend + 'static,
        R: Renderer,
        X: XNest<R>,
    {
        type Inner = X::Inner;
        type MapInner<M> = InnerIvmToVm<Self, X::MapInner<M>>;
        type MapInnerTo<U: 'static> = XBuilder<
            R,
            Box<dyn FnOnce(ViewMemberCtx<R>, BuildFlags) -> X::MapInnerTo<U> + MaybeSend + 'static>,
        >;

        fn map_inner<M>(self) -> Self::MapInner<M> {
            InnerIvmToVm::new(self)
        }

        fn map_inner_to<U: 'static>(
            self,
            f: impl FnOnce(Self::Inner) -> U + MaybeSend + Clone + 'static,
        ) -> Self::MapInnerTo<U> {
            member_builder(Box::new(move |ctx, flags| {
                self.0(ctx, flags).map_inner_to::<U>(f)
            }))
        }

        fn is_static() -> bool {
            X::is_static()
        }
    }

    impl<R, F, VM, X, M> ViewMemberOrigin<R> for InnerIvmToVm<XBuilder<R, F>, M>
    where
        F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> X + MaybeSend + 'static,
        R: Renderer,
        X: XNest<R, MapInner<M> = VM>,
        VM: ViewMemberOrigin<R>,
        M: MaybeSend + 'static,
    {
        type Origin = VM::Origin;
    }

    impl<R, F, M, VM, X> ViewMember<R> for InnerIvmToVm<XBuilder<R, F>, M>
    where
        F: FnOnce(ViewMemberCtx<R>, BuildFlags) -> X + MaybeSend + 'static,
        R: Renderer,
        X: XNest<R, MapInner<M> = VM>,
        VM: ViewMember<R>,
        M: MaybeSend + 'static,
    {
        fn count() -> ViewMemberIndex {
            1
        }

        fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
            VM::unbuild(ctx, view_removed)
        }

        fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
            member_builder(|ctx, flags| self.0 .0(ctx, flags).map_inner::<M>())
                .build(ctx, will_rebuild)
        }

        fn rebuild(self, ctx: ViewMemberCtx<R>) {
            member_builder(|ctx, flags| self.0 .0(ctx, flags).map_inner::<M>()).rebuild(ctx)
        }
    }
}
