#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::force_dynamic_view::common_impl_force_dynamic_view;
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, FnArg, GenericArgument, GenericParam, ItemFn, Pat, PatType, PathArguments,
    ReturnType, Type, TypeParam, TypeParamBound,
};

mod force_dynamic_view;
mod into_view;
mod schema;

use schema::*;

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
    let input_item: ItemFn = parse_macro_input!(item as ItemFn);

    let renderer = quote!(BevyRenderer);

    let is_element = {
        let return_type_error = "schema attribute only support fn with return type 'impl IntoElementView' or 'impl IntoView'";
        let ReturnType::Type(_, ty) = &input_item.sig.output else {
            panic!("{}", return_type_error)
        };
        let Type::ImplTrait(ty) = &**ty else {
            panic!("{}", return_type_error)
        };
        let TypeParamBound::Trait(ty) = ty.bounds.first().unwrap() else {
            panic!("{}", return_type_error)
        };
        let impl_name = ty.path.segments.first().unwrap().ident.to_string();

        if impl_name == "IntoElementView" {
            true
        } else if impl_name == "IntoView" {
            false
        } else {
            panic!("{}", return_type_error)
        }
    };

    let (token_stream, extra_token_stream) = schema_common(
        renderer,
        is_element,
        &input_item.sig.ident,
        {
            let ident_str = input_item.sig.ident.to_string();
            ident_str
                .strip_prefix("schema_")
                .expect("schema attribute only support fn with name start with 'schema_'")
                .to_string()
        },
        input_item.sig.inputs.iter().map(|arg| {
            let FnArg::Typed(arg) = arg else {
                panic!("schema attribute only support fn with arguments")
            };

            let param_ident = get_parameter_ident(&arg.pat);

            (param_ident, &*arg.ty)
        }),
        &input_item.sig.generics,
        false,
        |schema_generic_params_with_bound,
         renderer,
         where_clause,
         schema_generic_params,
         schema_id| {
            quote! {
                #input_item
                #[allow(unused_parens)]
                #[derive(Clone, Copy, Default, Debug)]
                pub struct #schema_id<#schema_generic_params>(core::marker::PhantomData<(#schema_generic_params)>);

            }
        },
    );

    (quote! {
        #extra_token_stream
        #token_stream
    })
    .into()
}

fn get_parameter_ident(pat: &Pat) -> &Ident {
    if let Pat::TupleStruct(tuple_struct) = pat {
        if tuple_struct.elems.len() != 1 {
            panic!("TupleStruct must have one element")
        }
        let pat = tuple_struct.elems.first().unwrap();
        get_parameter_ident(if let Pat::TupleStruct(tuple_struct) = pat {
            if tuple_struct.elems.len() != 1 {
                panic!("TupleStruct must have one element")
            }
            tuple_struct.elems.first().unwrap()
        } else {
            pat
        })
    } else if let Pat::Ident(pat) = pat {
        &pat.ident
    } else {
        panic!("parse parameter name failed!")
    }
}

#[proc_macro_attribute]
pub fn bevy_force_dynamic_view(input: TokenStream, item: TokenStream) -> TokenStream {
    common_impl_force_dynamic_view(false, false, quote!(BevyRenderer), input, item)
}

#[proc_macro_attribute]
pub fn bevy_force_into_dynamic_view(input: TokenStream, item: TokenStream) -> TokenStream {
    common_impl_force_dynamic_view(false, true, quote!(BevyRenderer), input, item)
}
