use crate::utils::all_tuples;
use crate::{
    smallbox, ElementAttr, ElementAttrViewMember, MaybeSend, MaybeSync, Renderer,
    ViewMember, ViewMemberCtx, ViewMemberIndex, ViewMemberOrigin,
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

pub trait Mapper<T> {
    type To;
    fn map(self) -> Self::To;
}

pub struct VmMapper<R>(PhantomData<R>);

impl<R, T> Mapper<VmMapper<R>> for T
where
    R: Renderer,
    T: XNest<R>,
    T::MapMember<VmMapper<R>>: ViewMember<R>,
{
    type To = T::MapMember<VmMapper<R>>;

    fn map(self) -> Self::To {
        self.into_member()
    }
}

pub trait XNest<R>
where
    R: Renderer,
{
    type InnerMember;

    type MapMember<M>;
    fn map_inner<M>(self) -> Self::MapMember<M>;

    fn into_member(self) -> Self::MapMember<VmMapper<R>>
    where
        // Self::InnerMember: Mapper<VmMapper<R>>,
        Self::MapMember<VmMapper<R>>: ViewMember<R>,
        Self: Sized,
    {
        // self.map()
        self.map_inner::<VmMapper<R>>()
    }
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
    type InnerMember = T;
    type MapMember<M> = MapValueWrapper<T, M>;

    fn map_inner<M>(self) -> Self::MapMember<M> {
        MapValueWrapper::<T,M>(self.into().0, Default::default())
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
        type InnerMember = Self;
        type MapMember<M> = Self;

        fn map_inner<M>(self) -> Self::MapMember<M>
        where
            Self::MapMember<M>: ViewMember<R>,
        {
            self
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
    use crate::style::{StyleItemValue, StyleSheetCtx, StyleSheetItems};
    use crate::style::{ApplyStyleSheets, StyleSheets};
    use crate::{
        MapValueWrapper, MaybeSend, Renderer, ViewMember, ViewMemberCtx, ViewMemberIndex,
        ViewMemberOrigin, XNest,
    };

    impl<R, T> XNest<R> for ApplyStyleSheets<T>
    where
        R: Renderer,
        T: StyleSheets<R>,
    {
        type InnerMember = Self;
        type MapMember<M> = Self;

        fn map_inner<M>(self) -> Self::MapMember<M> {
            self
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
    use xy_reactive::prelude::{Memo, ReadSignal, RwSignal, SignalGet,};
    use crate::{rx,Reactive};
    impl<R, IVM> XNest<R> for Memo<IVM>
    where
        R: Renderer,
        IVM: XNest<R> + MaybeSend + MaybeSync + Clone + 'static,
    {
        type InnerMember = IVM::InnerMember;
        type MapMember<M> = InnerIvmToVm<Self, M>;

        fn map_inner<M>(self) -> Self::MapMember<M> {
            InnerIvmToVm::new(self)
        }
    }

    impl<R, IVM> XNest<R> for ReadSignal<IVM>
    where
        R: Renderer,
        IVM: XNest<R> + MaybeSend + MaybeSync + Clone + 'static,
    {
        type InnerMember = IVM::InnerMember;
        type MapMember<M> = InnerIvmToVm<Self, M>;

        fn map_inner<M>(self) -> Self::MapMember<M> {
            InnerIvmToVm::new(self)
        }
    }

    impl<R, IVM> XNest<R> for RwSignal<IVM>
    where
        R: Renderer,
        IVM: XNest<R> + MaybeSend + MaybeSync + Clone + 'static,
    {
        type InnerMember = IVM::InnerMember;
        type MapMember<M> = InnerIvmToVm<Self, M>;

        fn map_inner<M>(self) -> Self::MapMember<M> {
            InnerIvmToVm::new(self)
        }
    }

    impl<R, F, IVM> XNest<R> for Reactive<F, IVM>
    where
        R: Renderer,
        F: Fn() -> IVM + MaybeSend + 'static,
        IVM: XNest<R> + MaybeSend + 'static,
    {
        type InnerMember = IVM::InnerMember;
        type MapMember<M> = InnerIvmToVm<Self, M>;

        fn map_inner<M>(self) -> Self::MapMember<M> {
            InnerIvmToVm::new(self)
        }
    }

    impl<R, F, IVM, M, VM> ViewMemberOrigin<R> for InnerIvmToVm<Reactive<F, IVM>, M>
    where
        R: Renderer,
        F: Fn() -> IVM + MaybeSend + 'static,
        IVM: XNest<R, MapMember<M> = VM> + MaybeSend + 'static,
        VM: ViewMemberOrigin<R>,
        M: MaybeSend + 'static,
    {
        type Origin = VM::Origin;
    }

    impl<R, F, IVM, M, VM> ViewMember<R> for InnerIvmToVm<Reactive<F, IVM>, M>
    where
        R: Renderer,
        F: Fn() -> IVM + MaybeSend + 'static,
        IVM: XNest<R, MapMember<M> = VM> + MaybeSend + 'static,
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

    impl<R, T, VM, IVM, M> ViewMemberOrigin<R> for InnerIvmToVm<T, M>
    where
        R: Renderer,
        T: SignalGet<Value = IVM> + MaybeSend + 'static,
        IVM: XNest<R, MapMember<M> = VM> + MaybeSync + Clone + 'static,
        VM: ViewMemberOrigin<R>,
        M: MaybeSend + 'static,
    {
        type Origin = VM::Origin;
    }

    impl<R, T, M, VM, IVM> ViewMember<R> for InnerIvmToVm<T, M>
    where
        R: Renderer,
        T: SignalGet<Value = IVM> + MaybeSend + 'static,
        IVM: XNest<R, MapMember<M> = VM> + MaybeSync + Clone + 'static,
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
