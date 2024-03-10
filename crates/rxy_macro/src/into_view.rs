use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::quote;
use syn::parse::Parse;
use syn::{parse_macro_input, parse_quote, DeriveInput, GenericArgument, Generics, Type};

pub fn into_view(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics crate::IntoView<R> for #struct_name #type_generics #where_clause {
            type View = #struct_name #type_generics;

            fn into_view(self) -> Self::View{
                self
            }
        }
    })
}

struct ImplIntoViewInput {
    ty: Type,
    where_predicate: Option<syn::WhereClause>,
}

impl Parse for ImplIntoViewInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ty: input.parse()?,
            where_predicate: input.parse().ok(),
        })
    }
}

pub fn impl_into_view(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ImplIntoViewInput);

    let type_path = if let Type::Path(type_path) = &ast.ty {
        Some(type_path)
    } else {
        None
    };
    let generic_args = type_path.and_then(|n| {
        n.path.segments.iter().last().and_then(|n| {
            if let syn::PathArguments::AngleBracketed(n) = &n.arguments {
                Some(n.args.iter().map(|n| {
                    if let GenericArgument::Type(n) = n {
                        n
                    } else {
                        panic!("not type")
                    }
                }))
            } else {
                None
            }
        })
    });

    let mut generics = Generics {
        where_clause: ast.where_predicate,
        ..Default::default()
    };
    if let Some(generic_args) = generic_args {
        generic_args.for_each(|n| {
            generics.params.push(parse_quote! {
                #n
            });
        });
    }
    let r: Ident = parse_quote! {R};
    generics.params.push(parse_quote! {#r});
    generics.make_where_clause().predicates.push(parse_quote! {
        #r: Renderer
    });
    let (impl_generics, _type_generics, where_clause) = &generics.split_for_impl();

    let ty = &ast.ty;
    TokenStream::from(quote! {
        impl #impl_generics IntoView<#r> for #ty #where_clause {
            type View = #ty;

            #[inline]
            fn into_view(self) -> Self::View{
                self
            }
        }
    })
}
