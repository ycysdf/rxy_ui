use rxy_ui::prelude::*;
use rxy_ui::prelude::div;

#[tokio::main]
async fn main() {
   tracing_subscriber::fmt().init();
   let mut app = XyApp::default();
   app.add_view((
      "SDFDSFSDF",
      div().children("CXSXCXC23"),
      div().children("CXSXCXC1"),
      div().children("CXSXCXC1"),
      div().children("CXSXCXC1"),
   ));
   tokio::task::block_in_place(|| {
      app.run();
   });
}
