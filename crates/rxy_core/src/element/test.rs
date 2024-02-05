// use crate::attr_value::AttrValue;
// use crate::element_attr::{AttrIndex, ElementAttr};
// use crate::element_attr_member::ElementAttrMember;
// use core::hint::unreachable_unchecked;
// use core::marker::PhantomData;
// use crate::ElementNodeTree;
// use rxy_core::{MaybeFromReflect, MaybeTypePath, Renderer, RendererNodeId, RendererWorld};
//
// pub struct TestEA<T>(PhantomData<T>);
//
// #[allow(unused_variables)]
// impl<R, T> ElementAttr<R> for TestEA<T>
// where
//     R: Renderer,
//     RendererWorld<R>: ElementNodeTree<R>,
//     T: AttrValue + Clone + Sized + MaybeFromReflect + MaybeTypePath,
// {
//     type Value = T;
//     const NAME: &'static str = "";
//     const INDEX: AttrIndex = 0;
//
//     fn update_value(
//         world: RendererWorld<R>,
//         node_id: RendererNodeId<R>,
//         value: impl Into<Self::Value>,
//     ) {
//         unsafe { unreachable_unchecked() }
//     }
// }
//
// impl<R, T> ElementAttrMember<R> for TestVM<T>
// where
//     R: Renderer,
//     RendererWorld<R>: ElementNodeTree<R>,
//     T: ElementAttr<R>,
// {
//     type EA = T;
// }
