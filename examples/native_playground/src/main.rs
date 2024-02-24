use rxy_ui::prelude::div;
use rxy_ui::prelude::*;

fn main() {
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
    app.run();
}
