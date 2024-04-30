use bevy_app::App;

pub use div::*;
pub use img::*;
pub use span::*;

mod div;
mod img;
mod span;

pub mod prelude {
   use rxy_core::AttrIndex;
   use rxy_core::{attrs_fn_define, impl_attrs_for_element_type, impl_index_for_tys};

   use crate::all_attrs::COMMON_ATTRS;
   use crate::element_attrs_fn_define;
   use crate::BevyRenderer;

   use super::*;

   element_attrs_fn_define! {
       [element_div]
       attrs = []

       [element_span]
       attrs = [
           content
       ]

       [element_img]
       attrs = [
           src
           flip_x
           flip_y
       ]
   }
}

pub trait ElementTypeRegisterAppExt {
   fn register_element_types(&mut self) -> &mut Self;
}

impl ElementTypeRegisterAppExt for App {
   fn register_element_types(&mut self) -> &mut Self {
      self
         .register_type::<element_div>()
         .register_type::<element_span>()
         .register_type::<element_img>();

      #[cfg(feature = "dynamic_element")]
      use rxy_core::ElementTypeTypeInfo;
      #[cfg(feature = "dynamic_element")]
      self
         .register_type_data::<element_div, ElementTypeTypeInfo<crate::BevyRenderer>>()
         .register_type_data::<element_span, ElementTypeTypeInfo<crate::BevyRenderer>>()
         .register_type_data::<element_img, ElementTypeTypeInfo<crate::BevyRenderer>>();
      self
   }
}
