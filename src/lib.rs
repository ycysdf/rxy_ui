pub use rxy_core::*;

#[cfg(feature = "bevy")]
pub use rxy_bevy as bevy;
#[cfg(feature = "bevy")]
pub use rxy_bevy_macro as bevy_macro;

#[cfg(feature = "style_sheet")]
pub mod style {
    pub use rxy_bevy_style::prelude;
    pub use rxy_bevy_style::prelude::StyleError;
    pub use rxy_bevy_style::Result;
    pub use rxy_bevy_style::*;
    pub use rxy_style::*;
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

    #[cfg(feature = "style_sheet")]
    pub use rxy_bevy_style::prelude::StyleError;
    #[cfg(feature = "style_sheet")]
    pub use rxy_bevy_style::prelude::*;
    #[cfg(feature = "style_sheet")]
    pub use rxy_style::prelude::*;

    #[cfg(feature = "signal")]
    pub use xy_reactive::prelude::*;
}
