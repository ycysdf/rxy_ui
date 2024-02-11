use rxy_ui::{prelude::*, web_dom::log};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() {
    build_on_body(test_view());
}

fn test_view() -> impl IntoView<WebRenderer> {
    div().children((
        div()
            .padding_left(30)
            .border("2px solid red")
            .padding_right(Some(Some(Some("3px"))))
            .padding_bottom(Some(Some(Some("10px"))))
            .children(("Hello World!", "HHH", "HHH", "XX")),
        h1().children("Hello World!"),
        h2().children("Hello World2!"),
        div().display("flex").gap(5).children((
            button()
                .padding(40)
                .on_click(|_| {
                    log("click!!".into());
                })
                .children("Print Btn"),
            button()
                .padding(40)
                .on_click(|_| {
                    web_sys::window()
                        .unwrap()
                        .alert_with_message("Alert!")
                        .unwrap();
                })
                .children("Alert Btn"),
        )),
    ))
}
