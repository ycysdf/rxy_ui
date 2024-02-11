mod renderer;

pub use renderer::*;
pub mod prelude {
    pub use super::attrs::CommonAttrsViewBuilder;
    pub use crate::build_on_body;
    pub use crate::renderer::common_renderer::*;
    pub use crate::renderer::event::HtmlElementEvents;
    pub use crate::renderer::WebElement;
    pub use crate::WebRenderer;
}
