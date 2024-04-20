use rxy_ui::prelude::div;
use rxy_ui::prelude::*;

#[tokio::main]
async fn main() {
   tracing_subscriber::fmt().init();
   let mut app = XyApp::default();
   app.add_view((
      "Hel\nlo1",
      "Hello2",
      "He\nllo333333",
      "D",
      "Hello4",
      div().children((
         "A",
         "B",
         "Hello1",
         "Hello2",
         "Hello3",
         "D",
         "Hello4",
         "C",
         "D",
         // div().children("CXSXCXC23"),
         // div().children("CXSXCXC23"),
      )),
      "A",
      "B",
      "Hello1",
      "Hello2",
      "D",
      // div().children("CXSXCXC1"),
      // div().children("CXSXCXC1"),
      // div().children("CXSXCXC1"),
   ));
   tokio::task::block_in_place(|| {
      app.run();
   });
}
