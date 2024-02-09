use std::fmt::Debug;
use std::hash::Hash;
use wasm_bindgen::prelude::*;

use rxy_ui::prelude::div;
use rxy_ui::prelude::*;
use rxy_ui::web_dom::{dom_build, window, WebRenderer};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn test_ui() {
    alert("Hello, rxy-ui22333!");
    let build = dom_build(test_view());
}

fn test_view() -> impl IntoView<WebRenderer> {
    let children = div()
        .padding_left(30)
        .padding_right(Some(Some(Some("30"))))
        .padding_bottom(Some(Some(Some("10"))))
        .children(("Hello World!", "HHH", "HHH"));
    children
}
