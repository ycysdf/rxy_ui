use alloc::boxed::Box;
use core::any::Any;
use futures_lite::FutureExt;
use core::future::Future;
use core::pin::Pin;
use futures_lite::StreamExt;
pub use reflect::*;
pub use send_sync::*;

#[cfg(feature = "bevy_reflect")]
mod reflect {
    pub use bevy_reflect::FromReflect as MaybeFromReflect;
    pub use bevy_reflect::Reflect as MaybeReflect;
    pub use bevy_reflect::TypePath as MaybeTypePath;
}

#[cfg(not(feature = "bevy_reflect"))]
mod reflect {
    pub trait MaybeReflect {}

    impl<T> MaybeReflect for T where T: ?Sized {}

    pub trait MaybeFromReflect {}

    impl<T> MaybeFromReflect for T where T: ?Sized {}

    pub trait MaybeTypePath {}

    impl<T> MaybeTypePath for T where T: ?Sized {}
}

#[cfg(feature = "send_sync")]
mod send_sync {
    pub use core::marker::Send as MaybeSend;
    pub use core::marker::Sync as MaybeSync;
}

#[cfg(not(feature = "send_sync"))]
mod send_sync {
    pub unsafe trait MaybeSend {}
    unsafe impl<T> MaybeSend for T where T: ?Sized {}

    pub unsafe trait MaybeSync {}
    unsafe impl<T> MaybeSync for T where T: ?Sized {}
}

#[cfg(feature = "send_sync")]
pub type MaybeSendAnyRef<'a> = &'a (dyn Any + MaybeSend + 'static);

#[cfg(not(feature = "send_sync"))]
pub type MaybeSendAnyRef<'a> = &'a (dyn Any + 'static);

#[cfg(feature = "send_sync")]
pub type MaybeSendSyncAnyRef<'a> = &'a (dyn Any + MaybeSend + MaybeSync + 'static);

#[cfg(not(feature = "send_sync"))]
pub type MaybeSendSyncAnyRef<'a> = &'a (dyn Any + 'static);

#[cfg(feature = "send_sync")]
pub type MaybeSendAnyBox = Box<dyn Any + MaybeSend + 'static>;

#[cfg(not(feature = "send_sync"))]
pub type MaybeSendAnyBox = Box<dyn Any + 'static>;

#[cfg(feature = "send_sync")]
pub type MaybeSendSyncAnyBox = Box<dyn Any + MaybeSend + MaybeSync + 'static>;

#[cfg(not(feature = "send_sync"))]
pub type MaybeSendSyncAnyBox = Box<dyn Any + 'static>;

#[cfg(feature = "send_sync")]
pub type BoxedFutureMaybeLocal<T> = futures_lite::future::Boxed<T>;

#[cfg(not(feature = "send_sync"))]
pub type BoxedFutureMaybeLocal<T> = futures_lite::future::BoxedLocal<T>;
#[cfg(feature = "send_sync")]
pub type BoxedStreamMaybeLocal<T> = futures_lite::stream::Boxed<T>;

#[cfg(not(feature = "send_sync"))]
pub type BoxedStreamMaybeLocal<T> = futures_lite::stream::BoxedLocal<T>;

pub trait MaybeSendSyncFutureExit<T>: Future<Output = T> {
    #[cfg(feature = "send_sync")]
    fn boxed_maybe_local<'a>(self) -> Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>>
    where
        Self: Sized + Send + 'a,
    {
        self.boxed::<'a>()
    }
    #[cfg(not(feature = "send_sync"))]
    fn boxed_maybe_local<'a>(self) -> Pin<Box<dyn Future<Output = Self::Output> + 'a>>
    where
        Self: Sized + 'a,
    {
        self.boxed_local::<'a>()
    }
}

impl<T, F> MaybeSendSyncFutureExit<T> for F where F: Future<Output = T> {}

pub trait MaybeSendSyncStreamExt<T>: futures_lite::Stream<Item = T> {
    #[cfg(feature = "send_sync")]
    fn boxed_maybe_local<'a>(self) -> Pin<Box<dyn futures_lite::Stream<Item = T> + Send + 'a>>
    where
        Self: Sized + Send + 'a,
    {
        self.boxed::<'a>()
    }
    #[cfg(not(feature = "send_sync"))]
    fn boxed_maybe_local<'a>(self) -> Pin<Box<dyn futures_lite::Stream<Item = T> + 'a>>
    where
        Self: Sized + 'a,
    {
        self.boxed_local::<'a>()
    }
}

impl<T, S> MaybeSendSyncStreamExt<T> for S where S: futures_lite::Stream<Item = T> {}
