mod renderer;

pub use renderer::*;
pub mod prelude {
    pub use crate::renderer::WebElement;
    pub use crate::renderer::common_renderer::*;

    pub use super::{
        attrs::CommonAttrsViewBuilder
    };
}