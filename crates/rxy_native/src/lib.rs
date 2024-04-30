#![allow(unused_imports)]
#![allow(unused_variables)]
pub use renderer::*;
use rxy_core::{Element, ElementAttr, ElementViewChildren};

mod app;
mod renderer;
mod running_app;
mod user_event;
mod window;
mod layout;
mod draw;
mod draw_text;
pub mod world_ext;

pub mod all_attrs {
   pub use crate::attrs::*;
   // pub use crate::elements::input_attrs::*;
   pub use crate::elements::element_span_attrs::*;
}
pub mod prelude {
   pub use crate::app::XyApp;
   pub use crate::renderer::common_renderer::*;
   pub use crate::renderer::*;

   pub use crate::Val;
   pub use vello::peniko::Color;

   pub use crate::elements::prelude::*;
   pub use crate::renderer::NativeElement;

   pub use super::all_attrs::{CommonAttrsElementViewBuilder, CommonAttrsViewBuilder};
   // pub use super::renderer::event::*;
   // pub use super::renderer::view_builder_ext::*;
}



#[cfg(feature = "dynamic_element")]
pub type DynamicNativeElement<E> = rxy_core::DynamicElement<NativeRenderer, E>;

pub type NativeElementViewChildren<CV, E, VM> =
ElementViewChildren<NativeRenderer, Element<NativeRenderer, E, VM>, CV>;

pub type NativeElementAttrMember<EA> = ElementAttr<NativeRenderer, EA>;

pub type TaskState = rxy_core::TaskState<NativeRenderer>;