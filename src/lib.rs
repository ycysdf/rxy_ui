#[cfg(feature = "bevy")]
pub use rxy_bevy as bevy;
#[cfg(feature = "bevy")]
pub use rxy_bevy_macro as bevy_macro;
pub use rxy_core::*;
#[cfg(feature = "web")]
pub use rxy_web_dom as web_dom;
#[cfg(feature = "signal")]
pub use xy_reactive as reactive;

#[cfg(feature = "style")]
pub mod style {
   pub use rxy_bevy::style::prelude;
   pub use rxy_bevy::style::prelude::StyleError;
   pub use rxy_bevy::style::Result;
   pub use rxy_bevy::style::*;
   pub use rxy_core::style::*;
}

pub mod prelude {
   #[cfg(feature = "bevy")]
   pub use rxy_bevy::prelude::*;
   #[cfg(feature = "bevy")]
   pub use rxy_bevy_macro::schema;
   pub use rxy_core::prelude::*;
   pub use rxy_macro::PropValueWrapper;
   #[cfg(feature = "web")]
   pub use rxy_web_dom::prelude::*;
   #[cfg(feature = "signal")]
   pub use xy_reactive::prelude::*;
   #[cfg(feature = "native")]
   pub use rxy_native::prelude::*;

}
