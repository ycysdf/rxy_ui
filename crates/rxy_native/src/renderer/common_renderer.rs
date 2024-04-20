use crate::{all_attrs};
use crate::renderer::elements::{element_div, element_span};
use crate::renderer::{NativeElement, NativeRenderer};
use rxy_core::common_renderer::CommonRenderer;
use rxy_core::{
   define_common_view_fns, ElementAttrMember, ElementView, MapToAttrMarker, MemberOwner, XNest,
};
use crate::elements::element_img;

define_common_view_fns!(NativeRenderer);

#[cfg(not(feature = "dynamic_element"))]
impl CommonRenderer for NativeRenderer {
   type DivView = NativeElement<element_div, ()>;
   type TextView<T: ElementAttrMember<Self, Self::TextContentEA>> =
      NativeElement<element_span, (T,)>;
   type ButtonView = NativeElement<element_div, ()>;
   type ImgView = NativeElement<element_img, ()>;
   type TextContentEA = all_attrs::content;

   fn crate_text<T>(
      str: impl XNest<MapInner<MapToAttrMarker<Self::TextContentEA>> = T>,
   ) -> Self::TextView<T>
   where
      T: ElementAttrMember<Self, Self::TextContentEA>,
   {
      NativeElement::default().members(str.map_inner::<MapToAttrMarker<Self::TextContentEA>>())
   }

   fn crate_div() -> Self::DivView {
      NativeElement::default()
   }

   fn crate_button() -> Self::ButtonView {
      NativeElement::default()
   }

   fn crate_img() -> Self::ImgView {
      NativeElement::default()
   }
}

#[cfg(feature = "dynamic_element")]
use crate::DynamicNativeElement;
#[cfg(feature = "dynamic_element")]
impl CommonRenderer for NativeRenderer {
   type DivView = DynamicNativeElement<element_div>;
   type TextView<T: ElementAttrMember<Self, Self::TextContentEA>> =
   DynamicNativeElement<element_span>;
   type ButtonView = DynamicNativeElement<element_div>;
   type ImgView = DynamicNativeElement<element_img>;
   type TextContentEA = all_attrs::content;

   fn crate_text<T>(
      str: impl XNest<MapInner<MapToAttrMarker<Self::TextContentEA>> = T>,
   ) -> Self::TextView<T>
      where
          T: ElementAttrMember<Self, Self::TextContentEA>,
   {
      DynamicNativeElement::default().members(str.map_inner::<MapToAttrMarker<Self::TextContentEA>>())
   }

   fn crate_div() -> Self::DivView {
      DynamicNativeElement::default()
   }

   fn crate_button() -> Self::ButtonView {
      DynamicNativeElement::default()/*.members(x_bundle((
         FocusPolicy::default(),
         Interaction::default(),
         Button,
         Focusable::default(),
      )))*/
   }

   fn crate_img() -> Self::ImgView {
      DynamicNativeElement::default()
   }
}
