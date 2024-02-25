use bevy_app::App;
pub use div::*;
pub use img::*;
pub use span::*;

mod div;
mod img;
mod span;

pub mod prelude {
    use crate::all_attrs::COMMON_ATTRS;
    use crate::BevyRenderer;
    use rxy_core::AttrIndex;
    use super::*;
    use crate::element_attrs_fn_define;
    use rxy_core::{attrs_fn_define, impl_attrs_for_element_type, impl_index_for_tys};

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
        self.register_type::<element_div>()
            .register_type::<element_span>()
    }
}
