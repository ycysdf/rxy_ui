use alloc::boxed::Box;
use core::any::Any;

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
