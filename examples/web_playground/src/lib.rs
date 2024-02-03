use std::fmt::Debug;
use std::hash::Hash;

use wasm_bindgen::prelude::*;

use rxy_ui::prelude::*;
use rxy_ui::web_dom::renderer::dom_build;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn test_ui() {
    let build = dom_build(div().children(("Hello World!", "HHH")));
}
