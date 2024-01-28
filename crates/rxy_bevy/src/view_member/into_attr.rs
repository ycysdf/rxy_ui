use futures_lite::StreamExt;
use rxy_bevy_element::ElementUnitAttr;
use rxy_core::{Either, ViewMember};
use crate::{BevyRenderer, BevyWrapper, ViewAttr};

pub trait IntoViewAttrMember<EA> {
    // type Attr: ViewAttrMember<EA = EA>;
    type Attr: ViewMember<BevyRenderer>;
    fn into_attr(self) -> Self::Attr;
}

impl<EA, T> IntoViewAttrMember<EA> for T
    where
        EA: ElementUnitAttr,
        T: Into<BevyWrapper<EA::Value>>,
{
    type Attr = ViewAttr<EA>;

    fn into_attr(self) -> Self::Attr {
        ViewAttr(self.into().0)
    }
}

impl<EA, T> IntoViewAttrMember<EA> for Option<T>
    where
        EA: ElementUnitAttr,
        T: Into<BevyWrapper<EA::Value>>,
{
    type Attr = Option<ViewAttr<EA>>;

    fn into_attr(self) -> Self::Attr {
        self.map(|n| ViewAttr(n.into().0))
    }
}

impl<EA, L, R> IntoViewAttrMember<EA> for Either<L, R>
    where
        EA: ElementUnitAttr,
        L: Into<BevyWrapper<EA::Value>>,
        R: Into<BevyWrapper<EA::Value>>,
{
    type Attr = Either<ViewAttr<EA>, ViewAttr<EA>>;

    fn into_attr(self) -> Self::Attr {
        match self {
            Either::Left(n) => Either::Left(ViewAttr(n.into().0)),
            Either::Right(n) => Either::Right(ViewAttr(n.into().0)),
        }
    }
}

impl<EA, T> IntoViewAttrMember<EA> for futures_lite::stream::Boxed<T>
    where
        EA: ElementUnitAttr,
        T: IntoViewAttrMember<EA> + 'static,
{
    type Attr = futures_lite::stream::Boxed<T::Attr>;

    fn into_attr(self) -> Self::Attr {
        self.map(|n| n.into_attr()).boxed()
    }
}