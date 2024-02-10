use crate::prelude::ElementEventIds;
use crate::{EventViewMember, XBundle};
use bevy_ecs::prelude::{Bundle, IntoSystem};
use rxy_core::{XNest, XNestMapper};

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
