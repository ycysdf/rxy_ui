
pub use div::*;
pub use img::*;
pub use span::*;
use crate::NativeRenderer;

mod div;
mod img;
mod span;

pub mod prelude {
    use rxy_core::AttrIndex;
    use rxy_core::{attrs_fn_define, impl_attrs_for_element_type, impl_index_for_tys};


    use crate::all_attrs::COMMON_ATTRS;
    use crate::element_attrs_fn_define;
    use crate::NativeRenderer;

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
           // src
           // flip_x
           // flip_y
       ]
   }
}

#[cfg(feature = "reflect")]
pub trait ElementTypeRegisterExt {
   fn register_element_types(&mut self) -> &mut Self;
}

#[cfg(feature = "reflect")]
impl ElementTypeRegisterExt for bevy_reflect::TypeRegistry {
   fn register_element_types(&mut self) -> &mut Self {
      self.register::<element_div>();
      self.register::<element_span>();
      self.register::<element_img>();

      #[cfg(feature = "dynamic_element")]
      use rxy_core::ElementTypeTypeInfo;
      #[cfg(feature = "dynamic_element")]
      self
         .register_type_data::<element_div, ElementTypeTypeInfo<NativeRenderer>>()
         .register_type_data::<element_span, ElementTypeTypeInfo<NativeRenderer>>()
         .register_type_data::<element_img, ElementTypeTypeInfo<NativeRenderer>>();
      self
   }
}
