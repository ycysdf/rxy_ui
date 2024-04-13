use proc_macro::TokenStream;

use quote::quote;
use syn::{
   parse::{Parse, ParseStream},
   parse_macro_input, Ident, Result, Token,
};

struct IdentCount(usize);

impl Parse for IdentCount {
   fn parse(input: ParseStream) -> Result<Self> {
      Ok(IdentCount(
         input.parse_terminated(Ident::parse, Token![,])?.len(),
      ))
   }
}

pub fn ident_count(input: TokenStream) -> TokenStream {
   let ident_count = parse_macro_input!(input as IdentCount).0;

   TokenStream::from(quote! {
       #ident_count
   })
}
