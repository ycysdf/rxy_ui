
// use crate::renderer::{WebDomNodeStates, WebRenderer};
// use rxy_core::{RendererNodeId, RendererWorld};
// use rxy_element::{AttrIndex};
//
//
//

//
// pub struct WebWrapper<T>(T);
//
//
// pub struct test_attr;
// pub struct test_attr2;
//
// impl<R> ElementAttr<R> for test_attr
//     where
//         R: Renderer,
// {
//     type Value = u32;
//     const NAME: &'static str = "";
//     const INDEX: AttrIndex = 0;
//
//     fn update_value(
//         world:&mut RendererWorld<R>,
//         node_id: RendererNodeId<R>,
//         value: impl Into<Self::Value>,
//     ) {
//         todo!()
//     }
// }
// impl<R> ElementAttr<R> for test_attr2
//     where
//         R: Renderer,
// {
//     type Value = u32;
//     const NAME: &'static str = "";
//     const INDEX: AttrIndex = 0;
//
//     fn update_value(
//         world:&mut RendererWorld<R>,
//         node_id: RendererNodeId<R>,
//         value: impl Into<Self::Value>,
//     ) {
//         todo!()
//     }
// }
//
//
// mod xx {
//     use crate::renderer::WebRenderer;
//     use rxy_core::{IntoViewMember, Renderer, RendererWorld};
//     use rxy_element::into_attr::{test_attr, test_attr2, AttrValueWrapper, ElementAttrViewMember};
//     use rxy_element::{ElementAttr, ElementAttrMember};
//
//     pub fn testt<T>(a: impl IntoViewMember<WebRenderer, T>)
//     where
//         T: ElementAttrMember<WebRenderer, EA = test_attr2>,
//     {
//         let _ = a.into_member();
//     }
//     // Into<IntoViewMemberWrapper<ElementAttrViewMember<R, test_attr>>>
//     pub fn testt2<T>(a: impl IntoViewMember<WebRenderer, T>)
//     where
//         T: ElementAttrMember<WebRenderer>,
//         <T::EA as ElementAttr<WebRenderer>>::Value:
//             Into<AttrValueWrapper<WebRenderer, T::EA>>,
//     {
//         let _ = a.into_member();
//     }
//
//     impl<R, EA> Into<AttrValueWrapper<R, EA>> for AAA
//     where
//         EA: ElementAttr<R>,
//         EA::Value: From<AAA>,
//         R: Renderer,
//     {
//         fn into(self) -> AttrValueWrapper<R, EA> {
//             AttrValueWrapper(self.into())
//         }
//     }
//
//     pub struct AAA;
//     impl From<AAA> for u32 {
//         fn from(value: AAA) -> Self {
//             112
//         }
//     }
//
//     pub fn xx() {
//         testt(Some(Some(1221)));
//         testt(Some(Some(AAA)));
//         // testt2(Some(Some(1221)));
//     }
// }
pub mod renderer;
pub mod prelude {}