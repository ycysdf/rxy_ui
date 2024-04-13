use rxy_ui::prelude::div;
use rxy_ui::prelude::*;

#[tokio::main]
async fn main() {
    let mut app = XyApp::default();
    app.add_view(div().children((
        "SDFDSFSDF",
        div().children("CXSXCXC"),
        "SDFSDFSD",
        "SDFSDFSD",
        "SDFSDFSD",
        "SDFSDFSD",
        "SDFSDFSD",
        "SDFSDFSD",
    )));
    tokio::task::block_in_place(|| {
        app.run();
    });
}
