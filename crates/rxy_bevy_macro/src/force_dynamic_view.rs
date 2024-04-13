use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_quote, ItemFn, ReturnType};

pub fn common_impl_force_dynamic_view(
   is_current_crate: bool,
   is_into_view: bool,
   render: impl ToTokens,
   _input: TokenStream,
   item: TokenStream,
) -> TokenStream {
   let mut ast = parse_macro_input!(item as ItemFn);
   let path = if is_current_crate {
      quote!(crate)
   } else {
      quote!(rxy_core)
   };
   let block = ast.block;

   let ReturnType::Type(_, ty) = &ast.sig.output else {
      panic!("require return type")
   };
   ast.block = if is_into_view {
      parse_quote! {
          {
              use #path::IntoDynamicView;
              let r: #ty = #block;
              r.into_view().into_dynamic().into_view()
          }
      }
   } else {
      parse_quote! {
          {
              use #path::IntoDynamicView;
              let r: #ty = #block;
              r.into_dynamic().into_view()
          }
      }
   };

   ast.sig.output = parse_quote!(-> #path::BoxedDynamicViewView<#render>);

   ast.into_token_stream().into()
}
