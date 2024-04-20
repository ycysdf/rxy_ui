pub use renderer::*;

mod app;
mod renderer;
mod running_app;
mod user_event;
mod window;
mod layout;
mod draw;
mod draw_text;

pub mod all_attrs {
   // pub use crate::attrs::*;
   // pub use crate::elements::input_attrs::*;
   pub use crate::elements::attrs::*;
}
pub mod prelude {
   pub use crate::app::XyApp;
   pub use crate::renderer::common_renderer::*;
   pub use crate::renderer::*;

   // pub use crate::attrs::element_view_builder::*;
}
