#![allow(unused_imports)]
#![allow(unused_variables)]

use proc_macro::TokenStream;

use convert_case::Casing;
use quote::{quote, ToTokens};

use schema::*;

use crate::force_dynamic_view::common_impl_force_dynamic_view;

mod force_dynamic_view;
mod into_view;
mod schema;

#[proc_macro_derive(BevyIntoView)]
pub fn bevy_into_view(input: TokenStream) -> TokenStream {
   into_view::bevy_into_view(input)
}

#[proc_macro_derive(Schema)]
pub fn bevy_struct_schema(input: TokenStream) -> TokenStream {
   struct_schema(input, quote!(BevyRenderer), false)
}

#[proc_macro_derive(ElementSchema)]
pub fn bevy_struct_element_schema(input: TokenStream) -> TokenStream {
   struct_schema(input, quote!(BevyRenderer), true)
}

#[proc_macro_attribute]
pub fn schema(_input: TokenStream, item: TokenStream) -> TokenStream {
   fn_schema(_input, item)
}

#[proc_macro_attribute]
pub fn bevy_force_dynamic_view(input: TokenStream, item: TokenStream) -> TokenStream {
   common_impl_force_dynamic_view(false, false, quote!(BevyRenderer), input, item)
}

#[proc_macro_attribute]
pub fn bevy_force_into_dynamic_view(input: TokenStream, item: TokenStream) -> TokenStream {
   common_impl_force_dynamic_view(false, true, quote!(BevyRenderer), input, item)
}
