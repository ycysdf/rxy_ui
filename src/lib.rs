pub use rxy_core::*;

#[cfg(feature = "bevy")]
pub use rxy_bevy as bevy;
#[cfg(feature = "bevy")]
pub use rxy_bevy_macro as bevy_macro;

#[cfg(feature = "web")]
pub use rxy_web_dom as web_dom;

#[cfg(feature = "style")]
pub mod style {
    pub use rxy_bevy::style::prelude;
    pub use rxy_bevy::style::prelude::StyleError;
    pub use rxy_core::style::*;
    pub use rxy_bevy::style::*;
    pub use rxy_bevy::style::Result;
}

#[cfg(feature = "signal")]
pub use xy_reactive as reactive;

pub mod prelude {
    pub use rxy_core::prelude::*;
    pub use rxy_macro::PropValueWrapper;

    #[cfg(feature = "bevy")]
    pub use rxy_bevy::prelude::*;
    #[cfg(feature = "bevy")]
    pub use rxy_bevy_macro::schema;

    #[cfg(feature = "web")]
    pub use rxy_web_dom::prelude::*;

    #[cfg(feature = "signal")]
    pub use xy_reactive::prelude::*;
}
