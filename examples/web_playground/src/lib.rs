use rxy_ui::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() {
    build_on_body(test_view());
}

fn test_view() -> impl IntoView<WebRenderer> {
    let children = div()
        .padding_left(30)
        .padding_right(Some(Some(Some("3px"))))
        .padding_bottom(Some(Some(Some("10px"))))
        .children(("Hello World!", "HHH", "HHH", "XX"));
    children
}
