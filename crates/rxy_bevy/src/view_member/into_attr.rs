use core::marker::PhantomData;

use crate::{BevyRenderer, BevyWrapper, ViewAttr, XRes};
use bevy_ecs::prelude::Resource;
use bevy_reflect::{FromReflect, TypePath};
use futures_lite::{prelude::Future, StreamExt};
use rxy_bevy_element::{AttrIndex, AttrValue, ElementAttr, ElementUnitAttr, HasIndex};
use rxy_core::{Either, ViewMember, XFuture};

pub struct ElementAttrAgent<EAV>(PhantomData<EAV>);

impl<EAV> HasIndex for ElementAttrAgent<EAV> {
    const INDEX: AttrIndex = u8::MAX;
}

impl<EAV> ElementAttr for ElementAttrAgent<EAV>
where
    EAV: AttrValue + Clone + Sized + FromReflect + TypePath,
{
    type Value = EAV;

    const NAME: &'static str = "agent";

    fn set_value(
        _context: &mut rxy_bevy_element::SetAttrValueContext,
        _value: impl Into<Self::Value>,
    ) {
        unreachable!("agent attr should not be set")
    }
}

// todo: extract into_other_attr to other trait
pub trait IntoViewAttrMember<EA>
where
    EA: ElementUnitAttr,
{
    type Attr: ViewMember<BevyRenderer> + IntoViewAttrMember<EA>;
    type OtherAttr<OEA: ElementUnitAttr<Value = EA::Value>>: ViewMember<BevyRenderer>
        + IntoViewAttrMember<OEA>;

    fn into_attr(self) -> Self::Attr;

    fn into_other_attr<OEA: ElementUnitAttr<Value = EA::Value>>(self) -> Self::OtherAttr<OEA>;
}

impl<EA> IntoViewAttrMember<EA> for ViewAttr<EA>
where
    EA: ElementUnitAttr,
{
    type Attr = Self;

    type OtherAttr<OEA: ElementUnitAttr<Value = <EA>::Value>> = ViewAttr<OEA>;

    fn into_attr(self) -> Self::Attr {
        self
    }

    fn into_other_attr<OEA: ElementUnitAttr<Value = <EA>::Value>>(self) -> Self::OtherAttr<OEA> {
        ViewAttr(self.0)
    }
}

impl<EA, T> IntoViewAttrMember<EA> for T
where
    EA: ElementUnitAttr,
    T: Into<BevyWrapper<EA::Value>>,
{
    type Attr = ViewAttr<EA>;

    type OtherAttr<OEA: ElementUnitAttr<Value = <EA>::Value>> = ViewAttr<OEA>;

    fn into_attr(self) -> Self::Attr {
        ViewAttr(self.into().0)
    }

    fn into_other_attr<OEA: ElementUnitAttr<Value = <EA>::Value>>(self) -> Self::OtherAttr<OEA> {
        ViewAttr(self.into().0)
    }
}

impl<EA, T> IntoViewAttrMember<EA> for Option<T>
where
    EA: ElementUnitAttr,
    T: IntoViewAttrMember<EA>,
{
    type Attr = Option<T::Attr>;

    type OtherAttr<OEA: ElementUnitAttr<Value = <EA>::Value>> = Option<T::OtherAttr<OEA>>;

    fn into_attr(self) -> Self::Attr {
        self.map(|n| n.into_attr())
    }

    fn into_other_attr<OEA: ElementUnitAttr<Value = <EA>::Value>>(self) -> Self::OtherAttr<OEA> {
        self.map(|n| n.into_other_attr::<OEA>())
    }
}

impl<EA, L, R> IntoViewAttrMember<EA> for Either<L, R>
where
    EA: ElementUnitAttr,
    L: IntoViewAttrMember<EA>,
    R: IntoViewAttrMember<EA>,
{
    type Attr = Either<L::Attr, R::Attr>;

    type OtherAttr<OEA: ElementUnitAttr<Value = <EA>::Value>> =
        Either<L::OtherAttr<OEA>, R::OtherAttr<OEA>>;

    fn into_attr(self) -> Self::Attr {
        match self {
            Either::Left(n) => Either::Left(n.into_attr()),
            Either::Right(n) => Either::Right(n.into_attr()),
        }
    }

    fn into_other_attr<OEA: ElementUnitAttr<Value = <EA>::Value>>(self) -> Self::OtherAttr<OEA> {
        match self {
            Either::Left(n) => Either::Left(n.into_other_attr::<OEA>()),
            Either::Right(n) => Either::Right(n.into_other_attr::<OEA>()),
        }
    }
}

impl<EA, T> IntoViewAttrMember<EA> for futures_lite::stream::Boxed<T>
where
    EA: ElementUnitAttr,
    T: IntoViewAttrMember<EA> + 'static,
{
    type Attr = futures_lite::stream::Boxed<T::Attr>;

    type OtherAttr<OEA: ElementUnitAttr<Value = <EA>::Value>> =
        futures_lite::stream::Boxed<T::OtherAttr<OEA>>;

    fn into_attr(self) -> Self::Attr {
        self.map(|n| n.into_attr()).boxed()
    }

    fn into_other_attr<OEA: ElementUnitAttr<Value = <EA>::Value>>(self) -> Self::OtherAttr<OEA> {
        self.map(|n| n.into_other_attr()).boxed()
    }
}

impl<EA, T> IntoViewAttrMember<EA> for XFuture<T>
where
    EA: ElementUnitAttr,
    T: Future + Send + 'static,
    T::Output: IntoViewAttrMember<EA> + Send + 'static,
{
    type Attr = XFuture<futures_lite::future::Boxed<<T::Output as IntoViewAttrMember<EA>>::Attr>>;

    type OtherAttr<OEA: ElementUnitAttr<Value = <EA>::Value>> =
        XFuture<futures_lite::future::Boxed<<T::Output as IntoViewAttrMember<EA>>::OtherAttr<OEA>>>;

    fn into_attr(self) -> Self::Attr {
        use futures_lite::FutureExt;
        XFuture(
            (async {
                let n = self.0.await;
                n.into_attr()
            })
            .boxed(),
        )
    }

    fn into_other_attr<OEA: ElementUnitAttr<Value = <EA>::Value>>(self) -> Self::OtherAttr<OEA> {
        use futures_lite::FutureExt;

        XFuture(
            (async {
                let n = self.0.await;
                n.into_other_attr::<OEA>()
            })
            .boxed(),
        )
    }
}

impl<EA, T> IntoViewAttrMember<EA> for futures_lite::future::Boxed<T>
where
    EA: ElementUnitAttr,
    T: IntoViewAttrMember<EA> + Send + 'static,
{
    type Attr = XFuture<futures_lite::future::Boxed<T::Attr>>;

    type OtherAttr<OEA: ElementUnitAttr<Value = <EA>::Value>> =
        XFuture<futures_lite::future::Boxed<T::OtherAttr<OEA>>>;

    fn into_attr(self) -> Self::Attr {
        use futures_lite::FutureExt;
        XFuture(
            (async {
                let n = self.await;
                n.into_attr()
            })
            .boxed(),
        )
    }

    fn into_other_attr<OEA: ElementUnitAttr<Value = <EA>::Value>>(self) -> Self::OtherAttr<OEA> {
        use futures_lite::FutureExt;

        XFuture(
            (async {
                let n = self.await;
                n.into_other_attr::<OEA>()
            })
            .boxed(),
        )
    }
}

// todo: 
impl<EA, T, F, VM> IntoViewAttrMember<EA> for XRes<T, F, VM>
where
    EA: ElementUnitAttr,
    T: Resource,
    F: Fn(&T) -> VM + Clone + Send + Sync + 'static,
    VM: ViewMember<BevyRenderer>,
{
    type Attr = Self;

    type OtherAttr<OEA: ElementUnitAttr<Value = <EA>::Value>> = Self;

    fn into_attr(self) -> Self::Attr {
        self
    }

    fn into_other_attr<OEA: ElementUnitAttr<Value = <EA>::Value>>(self) -> Self::OtherAttr<OEA> {
        self
    }
}
