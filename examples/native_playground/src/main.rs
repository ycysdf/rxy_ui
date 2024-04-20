use rxy_ui::prelude::*;

fn test_view() -> impl IntoView<NativeRenderer> {
   let signal = use_rw_signal(1);
   div().gap(Val::Px(12.)).children((
      div().flex_col().gap(Val::Px(12.)).children((
         "Header Test",
         div()
            .border(Val::Px(2.))
            .border_color(Color::RED)
            .children(("He\nllo", "Hello")),
         "Footer Test",
      )),
      div()
         .flex_col()
         .p(Val::Px(12.))
         .bg_color(Color::PINK)
         .width(Val::Px(240.))
         .height(Val::Px(260.))
         .border(Val::Px(20.))
         .border_color(Color::RED)
         .outline_width(Val::Px(20.))
         .outline_color(Color::BLUE)
         .outline_offset(Val::Px(20.))
         .justify_end()
         .items_center()
         .children((
            "OuterDiv",
            div()
               .center()
               .height(Val::Px(100.))
               .m(Val::Px(4.))
               .bg_color(Color::CYAN)
               .border(Val::Px(4.))
               .border_color(Color::MOCCASIN)
               .children("InterDiv"),
            "CCC",
         )),
      "A",
      "B",
      "Hello1",
      "Hello2",
      "D",
      // div().children("CXSXCXC1"),
      // div().children("CXSXCXC1"),
      // div().children("CXSXCXC1"),
   ))
}

#[tokio::main]
async fn main() {
   tracing_subscriber::fmt().init();
   let mut app = XyApp::default();
   app.add_view(test_view());
   tokio::task::block_in_place(|| {
      app.run();
   });
}
