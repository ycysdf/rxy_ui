pub use attr_value::*;
pub use element_type::*;
pub use element_unit_attr::*;

mod attr_value;
mod element_type;
mod element_unit_attr;
mod r#macro;

pub trait RxyBevyAppExt {
    fn register_elements_type(&mut self) -> &mut Self;
}
