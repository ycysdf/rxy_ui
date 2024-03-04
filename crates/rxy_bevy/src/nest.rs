use crate::prelude::ElementEventIds;
use crate::{x_res, BevyRenderer, EventViewMember, XBundle, XRes};
use bevy_ecs::prelude::{Bundle, IntoSystem, Resource};
use rxy_core::{InnerIvmToVm, MaybeSend, ViewMember, XFuture, XNest, XNestMapper};
use std::future::Future;

impl<T> XNest for XBundle<T>
where
    T: Bundle,
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

impl<T, U> XNestMapper<U> for XBundle<T>
where
    U: 'static,
    T: Bundle,
{
    type MapInnerTo = U;

    fn map_inner_to(
        self,
        f: impl FnOnce(Self::Inner) -> U + Send + Clone + 'static,
    ) -> Self::MapInnerTo {
        f(self)
    }
}

impl<T, S, TM> XNest for EventViewMember<T, S, TM>
where
    T: ElementEventIds,
    S: IntoSystem<(), (), TM> + Send + 'static,
    TM: Send + 'static,
{
    type Inner = Self;
    type MapInner<M> = Self;

    fn map_inner<U>(self) -> Self::MapInner<U> {
        self
    }

    fn is_static() -> bool {
        true
    }
}

impl<T, S, TM, U> XNestMapper<U> for EventViewMember<T, S, TM>
where
    U: 'static,
    T: ElementEventIds,
    S: IntoSystem<(), (), TM> + Send + 'static,
    TM: Send + 'static,
{
    type MapInnerTo = U;

    fn map_inner_to(
        self,
        f: impl FnOnce(Self::Inner) -> U + Send + Clone + 'static,
    ) -> Self::MapInnerTo {
        f(self)
    }
}

impl<T, F, X> XNest for XRes<T, F>
where
    T: Resource,
    F: Fn(&T) -> X + Send + Sync + 'static,
    X: XNest + MaybeSend + 'static,
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

impl<T, F, X, U> XNestMapper<U> for XRes<T, F>
where
    T: Resource,
    F: Fn(&T) -> X + Send + Sync + 'static,
    U: 'static,
    X: XNestMapper<U> + MaybeSend + 'static,
    X::MapInnerTo: Send + Sync,
{
    type MapInnerTo = XRes<T, Box<dyn Fn(&T) -> X::MapInnerTo + Send + Sync>>;

    fn map_inner_to(
        self,
        f: impl FnOnce(Self::Inner) -> U + Send + Clone + 'static,
    ) -> Self::MapInnerTo {
        todo!()
        // x_res(Box::new(move |r: &T| {
        //     let x = (self.f)(r);
        //     x.map_inner_to(f.clone())
        // }))
    }
}
