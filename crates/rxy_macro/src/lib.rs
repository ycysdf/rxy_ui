use crate::force_dynamic_view::common_impl_force_dynamic_view;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemStruct};

mod into_view;

mod all_tuples;
mod force_dynamic_view;
mod ident_count;

#[proc_macro]
pub fn all_tuples(input: TokenStream) -> TokenStream {
   all_tuples::all_tuples(input)
}

#[proc_macro]
pub fn all_tuples_with_size(input: TokenStream) -> TokenStream {
   all_tuples::all_tuples_with_size(input)
}

fn impl_into_prop_value_wrapper(input: TokenStream) -> TokenStream {
   let ast = parse_macro_input!(input as DeriveInput);

   let struct_name = &ast.ident;
   let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

   TokenStream::from(quote! {
       impl #impl_generics rxy_ui::IntoSchemaPropValue<rxy_ui::IntoSchemaPropValueWrapper<Self>> for #struct_name #type_generics #where_clause{
           fn into(self) -> rxy_ui::IntoSchemaPropValueWrapper<Self> {
               rxy_ui::IntoSchemaPropValueWrapper(self)
           }
       }
   })
}

#[proc_macro_derive(PropValueWrapper)]
pub fn prop_value_wrapper(input: TokenStream) -> TokenStream {
   impl_into_prop_value_wrapper(input)
}

#[proc_macro_attribute]
pub fn force_dynamic_view(input: TokenStream, item: TokenStream) -> TokenStream {
   common_impl_force_dynamic_view(true, false, quote!(R), input, item)
}

#[proc_macro_attribute]
pub fn force_into_dynamic_view(input: TokenStream, item: TokenStream) -> TokenStream {
   common_impl_force_dynamic_view(true, true, quote!(R), input, item)
}

#[proc_macro]
pub fn ident_count(input: TokenStream) -> TokenStream {
   ident_count::ident_count(input)
}

#[proc_macro]
pub fn impl_into_view(input: TokenStream) -> TokenStream {
   into_view::impl_into_view(input)
}

#[proc_macro_derive(IntoView)]
pub fn into_view(input: TokenStream) -> TokenStream {
   into_view::into_view(input)
}

#[proc_macro_derive(TypedStyle)]
pub fn typed_style(input: TokenStream) -> TokenStream {
   let item_struct = parse_macro_input!(input as ItemStruct);

   let ident = item_struct.ident;
   let render = quote!(rxy_ui::prelude::BevyRenderer);
   TokenStream::from(quote! {
       impl Copy for #ident {}
       impl Clone for #ident {
           fn clone(&self) -> Self {
               *self
           }
       }

       impl rxy_ui::style::TypedStyleLabel for #ident {}

       impl Into<rxy_ui::XValueWrapper<Self>> for #ident {
           fn into(self) -> rxy_ui::XValueWrapper<Self> {
               rxy_ui::XValueWrapper(self)
           }
       }

       impl rxy_ui::style::StyleSheets<#render> for #ident {
           fn style_sheets(
               self,
               ctx: rxy_ui::style::StyleSheetCtx<#render>,
           ) -> (
               impl Iterator<Item = rxy_ui::style::AppliedStyleSheet<#render>> + Send + 'static,
               rxy_ui::style::StyleSheetsInfo,
           ) {
               rxy_ui::style::typed_shared_style_sheets(core::any::TypeId::of::<Self>(), ctx)
           }
       }
   })
}
