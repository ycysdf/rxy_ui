use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn bevy_into_view(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics rxy_core::IntoView<BevyRenderer> for #struct_name #type_generics #where_clause {
            type View = #struct_name #type_generics;

            fn into_view(self) -> Self::View{
                self
            }
        }
    })
}
