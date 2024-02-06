use bevy_app::App;
pub use div::*;
pub use span::*;

mod div;
mod span;

pub trait ElementTypeRegisterAppExt {
    fn register_element_types(&mut self) -> &mut Self;
}

impl ElementTypeRegisterAppExt for App {
    fn register_element_types(&mut self) -> &mut Self {
        self.register_type::<element_div>()
            .register_type::<element_span>()
    }
}
