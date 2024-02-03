pub use traits::*;

#[cfg(feature = "bevy_reflect")]
mod traits {
    pub use bevy_reflect::FromReflect as MaybeFromReflect;
    pub use bevy_reflect::Reflect as MaybeReflect;
    pub use bevy_reflect::TypePath as MaybeTypePath;
}

#[cfg(not(feature = "bevy_reflect"))]
mod traits {
    pub trait MaybeReflect {}

    impl<T> MaybeReflect for T where T: ?Sized {}

    pub trait MaybeFromReflect {}

    impl<T> MaybeFromReflect for T where T: ?Sized {}

    pub trait MaybeTypePath {}

    impl<T> MaybeTypePath for T where T: ?Sized {}
}
