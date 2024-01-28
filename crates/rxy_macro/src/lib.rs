use crate::force_dynamic_view::common_impl_force_dynamic_view;
use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, DeriveInput};

mod into_view;

mod force_dynamic_view;
mod ident_count;

fn impl_into_prop_value_wrapper(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics rxy_core::IntoSchemaPropValue<rxy_core::IntoSchemaPropValueWrapper<Self>> for #struct_name #type_generics #where_clause{
            fn into(self) -> rxy_core::IntoSchemaPropValueWrapper<Self> {
                rxy_core::IntoSchemaPropValueWrapper(self)
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