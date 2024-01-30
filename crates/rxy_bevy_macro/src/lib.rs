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

#[proc_macro_derive(BevyIntoView)]
pub fn bevy_into_view(input: TokenStream) -> TokenStream {
    into_view::bevy_into_view(input)
}

enum ParamType {
    Prop,
    Static,
    Event,
    Slot,
    CloneableSlot,
}

#[proc_macro_attribute]
pub fn schema(_input: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let schema_ident = &item_fn.sig.ident;
    let ident_str = item_fn.sig.ident.to_string();
    let ident_str = ident_str
        .strip_prefix("schema_")
        .expect("schema attribute only support fn with name start with 'schema_'");
    let pascal_name = ident_str.to_case(Case::Pascal);
    let ident = Ident::new(ident_str, Span::call_site());
    let fn_name = &ident;
    let trait_name = format_ident!("{}Props", pascal_name);
    let schema_ident_name = format_ident!("{}Schema", pascal_name);
    let (impl_generics, ty_generics, where_clause) = item_fn.sig.generics.split_for_impl();
    let where_clause_predicates = where_clause.map(|n| &n.predicates);

    let schema_generic_params = item_fn
        .sig
        .generics
        .type_params()
        .map(|param| &param.ident)
        .collect::<Punctuated<&Ident, syn::token::Comma>>();
    let schema_generic_params_with_bound = item_fn
        .sig
        .generics
        .type_params()
        .collect::<Punctuated<&TypeParam, syn::token::Comma>>();
    let schema_generic_params_must_with_bound = item_fn
        .sig
        .generics
        .type_params()
        .filter(|n| !n.bounds.is_empty())
        .collect::<Punctuated<&TypeParam, syn::token::Comma>>();

    let schema_generic_params_with_bound = if schema_generic_params_with_bound.is_empty() {
        None
    } else {
        Some(quote! { #schema_generic_params_with_bound , })
    };
    let ty_generics_turbofish = ty_generics.as_turbofish();
    let schema_id = quote! {
        #schema_ident_name #ty_generics_turbofish
    };
    let return_type_error = "schema attribute only support fn with return type 'impl IntoElementView' or 'impl IntoView'";
    let ReturnType::Type(_, ty) = &item_fn.sig.output else {
        panic!("{}", return_type_error)
    };
    let Type::ImplTrait(ty) = &**ty else {
        panic!("{}", return_type_error)
    };
    let TypeParamBound::Trait(ty) = ty.bounds.first().unwrap() else {
        panic!("{}", return_type_error)
    };
    let impl_name = ty.path.segments.first().unwrap().ident.to_string();

    let is_element = if impl_name == "IntoElementView" {
        true
    } else if impl_name == "IntoView" {
        false
    } else {
        panic!("{}", return_type_error)
    };
    #[derive(PartialEq)]
    enum RequiredPropType {
        Prop,
        Slot,
        CloneableSlot,
    }
    let mut required_props: Vec<(&Ident, proc_macro2::TokenStream, RequiredPropType)> = vec![];

    let (prop_fn_sig, prop_fn_impl): (Vec<_>, Vec<_>) = item_fn
        .sig
        .inputs
        .iter()
        .enumerate()
        .filter_map(|(i, arg)| {
            let FnArg::Typed(arg) = arg else {
                panic!("schema attribute only support fn with arguments")
            };

            let Type::Path(path) = &*arg.ty else {
                panic!("schema attribute only support fn with arguments")
            };
            let last_segment = path.path.segments.last();
            if let Some(last_segment) = last_segment {
                let param_type_ident = last_segment.ident.to_string();
                let (param_type_ident, prop_value_ty, is_required_prop) = if param_type_ident
                    == "Required"
                {
                    let PathArguments::AngleBracketed(prop_value_ty) =
                        &last_segment.arguments else {
                        return None;
                    };

                    let GenericArgument::Type(ty) = prop_value_ty.args.first().unwrap() else {
                        return None;
                    };

                    let Type::Path(path) = ty else { return None };
                    let last_segment = path.path.segments.last()?;

                    (last_segment.ident.to_string(), last_segment, true)
                } else {
                    (param_type_ident, last_segment, false)
                };

                let pat = get_parameter_ident(&arg.pat);
                let param_type = if param_type_ident == "Sender" {
                    ParamType::Event
                } else if param_type_ident == "Static" {
                    if is_required_prop {
                        let PathArguments::AngleBracketed(prop_value_ty) =
                            &prop_value_ty.arguments else {
                            return None;
                        };
                        required_props.push((
                            pat,
                            prop_value_ty.args.last().unwrap().to_token_stream(),
                            RequiredPropType::Prop,
                        ));
                    }
                    ParamType::Static
                } else if param_type_ident == "Slot" {
                    if is_required_prop {
                        required_props.push((
                            pat,
                            prop_value_ty.to_token_stream(),
                            RequiredPropType::Slot,
                        ));
                    }
                    ParamType::Slot
                } else if param_type_ident == "CloneableSlot" {
                    if is_required_prop {
                        required_props.push((
                            pat,
                            prop_value_ty.to_token_stream(),
                            RequiredPropType::CloneableSlot,
                        ));
                    }
                    ParamType::CloneableSlot
                } else if param_type_ident.ends_with("Prop") || param_type_ident == "ReadSignal"
                {
                    if is_required_prop {
                        let PathArguments::AngleBracketed(prop_value_ty) =
                            &prop_value_ty.arguments else {
                            return None;
                        };
                        required_props.push((
                            pat,
                            prop_value_ty.args.last().unwrap().to_token_stream(),
                            RequiredPropType::Prop,
                        ));
                    }
                    ParamType::Prop
                } else {
                    return None;
                };
                return Some((i, pat, prop_value_ty, param_type));
            }
            None
        })
        .map(|(index, name, ty, param_type)| {
            let inner_ty = {
                if let PathArguments::AngleBracketed(prop_value_ty) =
                    &ty.arguments {
                    Some(prop_value_ty.args.last().unwrap())
                }else{
                    None
                }
            };
            let static_token = quote! {
                fn #name<ISP: rxy_ui::IntoSchemaProp<R, #inner_ty>>(
                    self,
                    value: ISP,
                ) -> Self
            };
            let slot_token = {
                let name = format_ident!("slot_{}", name.to_token_stream().to_string());
                quote! {
                    fn #name(
                        self,
                        value: impl rxy_ui::IntoView<R>,
                    ) -> Self
                }
            };
            let cloneable_slot_token = {
                let name = format_ident!("slot_{}", name.to_token_stream().to_string());
                quote! {
                    fn #name(
                        self,
                        value: impl rxy_ui::IntoCloneableView<R>,
                    ) -> Self
                }
            };
            let prop_fn_sig = if is_element {
                match param_type {
                    ParamType::Prop => quote! {
                        fn #name<ISP: rxy_ui::IntoSchemaProp<R, #inner_ty>>(
                            self,
                            value: ISP,
                        ) -> rxy_ui::ElementSchemaView<R, U, VM, P::Props<rxy_ui::ConstIndex<#index,ISP::Prop>>,#schema_id>
                        where
                            P::Props<rxy_ui::ConstIndex<#index,ISP::Prop>>: rxy_ui::SchemaProps<R>
                    },
                    ParamType::Static => static_token,
                    ParamType::Event => quote! {
                        fn #name<ISP: rxy_ui::IntoSchemaProp<R, rxy_ui::EventHandler<#inner_ty>>>(
                            self,
                            value: ISP,
                        ) -> rxy_ui::ElementSchemaView<R, U, VM, P::Props<rxy_ui::ConstIndex<#index,ISP::Prop>>,#schema_id>
                        where
                            P::Props<rxy_ui::ConstIndex<#index,ISP::Prop>>: rxy_ui::SchemaProps<R>
                    },
                    ParamType::Slot => slot_token,
                    ParamType::CloneableSlot => cloneable_slot_token,
                }
            } else {
                match param_type {
                    ParamType::Prop => quote! {
                        fn #name<ISP: rxy_ui::IntoSchemaProp<R, #inner_ty>>(
                            self,
                            value: ISP,
                        ) -> rxy_ui::SchemaView<R, U, P::Props<rxy_ui::ConstIndex<#index,ISP::Prop>>,#schema_id>
                        where
                            P::Props<rxy_ui::ConstIndex<#index,ISP::Prop>>: rxy_ui::SchemaProps<R>
                    },
                    ParamType::Static => static_token,
                    ParamType::Event => quote! {
                        fn #name<ISP: rxy_ui::IntoSchemaProp<R, rxy_ui::EventHandler<#inner_ty>>>(
                            self,
                            value: ISP,
                        ) -> rxy_ui::SchemaView<R, U, P::Props<rxy_ui::ConstIndex<#index,ISP::Prop>>,#schema_id>
                        where
                            P::Props<rxy_ui::ConstIndex<#index,ISP::Prop>>: rxy_ui::SchemaProps<R>
                    },
                    ParamType::Slot => slot_token,
                    ParamType::CloneableSlot => cloneable_slot_token,
                }
            };

            let prop_fn_impl = match param_type {
                ParamType::Prop => quote! {
                    #prop_fn_sig
                    {
                        self.set_indexed_prop::<#index, ISP, #inner_ty>(value)
                    }
                },
                ParamType::Event => quote! {
                    #prop_fn_sig
                    {
                        self.set_indexed_prop::<#index, ISP, rxy_ui::EventHandler<#inner_ty>>(value)
                    }
                },
                ParamType::Static => quote! {
                    #prop_fn_sig
                    {
                        self.set_static_indexed_prop::<#index, ISP, #inner_ty>(value)
                    }
                },
                ParamType::Slot => quote! {
                    #prop_fn_sig
                    {
                        self.indexed_slot::<#index>(value)
                    }
                },
                ParamType::CloneableSlot => quote! {
                    #prop_fn_sig
                    {
                        self.cloneable_indexed_slot::<#index>(value)
                    }
                },
            };

            (prop_fn_sig, prop_fn_impl)
        })
        .unzip();

    let renderer = quote!(BevyRenderer);
    let required_fn_generic_params = required_props
        .iter()
        .filter(|n| n.2 == RequiredPropType::Prop)
        .enumerate()
        .map(|(i, (pat, ty, _is_slot))| {
            let isp = format_ident!("T{}", i);
            quote! {
               #isp: rxy_ui::IntoSchemaProp<#renderer, #ty>
            }
        });

    let required_fn_params = required_props
        .iter()
        .enumerate()
        .map(|(i, (pat, ty, prop_type))| match prop_type {
            RequiredPropType::Prop => quote! {
                #pat: impl rxy_ui::IntoSchemaProp<#renderer, #ty>
            },
            RequiredPropType::Slot => quote! {
                #pat: impl rxy_ui::IntoView<#renderer>
            },
            RequiredPropType::CloneableSlot => quote! {
                #pat: impl rxy_ui::IntoCloneableView<#renderer>
            },
        });
    let required_fn_params_set =
        required_props
            .iter()
            .map(|(pat, ty, prop_type)| match prop_type {
                RequiredPropType::Prop => quote! {
                    .#pat(#pat)
                },
                RequiredPropType::Slot | RequiredPropType::CloneableSlot => {
                    let slot_prop = format_ident!("slot_{}", pat.to_token_stream().to_string());
                    quote! {
                        .#slot_prop(#pat)
                    }
                }
            });

    let schema_fn_def = if is_element {
        quote! {
            #[inline(always)]
            pub fn #fn_name<#schema_generic_params_with_bound>(#(#required_fn_params,)*) -> rxy_ui::ElementSchemaView<
                #renderer,
                rxy_ui::ElementSchemaBoundWrapper<impl rxy_ui::SchemaWithElementViewBound<#renderer>>,
                (),
                impl rxy_ui::SchemaProps<BevyRenderer>,
                #schema_id,
            > #where_clause {
                rxy_ui::element_schema_view(#schema_ident #ty_generics_turbofish, <#schema_id as core::default::Default>::default()).map(rxy_ui::ElementSchemaBoundWrapper)
                    #(#required_fn_params_set)*
            }
        }
    } else {
        quote! {
            pub fn #fn_name<#schema_generic_params_with_bound>(#(#required_fn_params,)*) -> rxy_ui::SchemaView<
                #renderer,
                impl rxy_ui::Schema<#renderer>,
                impl rxy_ui::SchemaProps<BevyRenderer>,
                #schema_id,
            > #where_clause {
                rxy_ui::schema_view(#schema_ident #ty_generics_turbofish, <#schema_id as core::default::Default>::default())
                    #(#required_fn_params_set)*
            }
        }
    };

    let trait_def_and_impl = if is_element {
        quote! {
            pub trait #trait_name<R, U, VM, P,#schema_generic_params_with_bound>
            where
                R: rxy_ui::Renderer,
                VM: rxy_ui::ViewMember<R>,
                U: rxy_ui::Schema<R>,
                U::View: rxy_ui::ElementView<R>,
                P: rxy_ui::SchemaProps<R>,
                #where_clause_predicates
            {
                #(#prop_fn_sig;)*
            }

            impl<R, U, VM, P,#schema_generic_params> #trait_name<R, U, VM, P,#schema_generic_params> for rxy_ui::ElementSchemaView<R, U, VM, P,#schema_id>
            where
                R: rxy_ui::Renderer,
                VM: rxy_ui::ViewMember<R>,
                U: rxy_ui::Schema<R>,
                U::View: rxy_ui::ElementView<R>,
                P: rxy_ui::SchemaProps<R>,
                #schema_generic_params_must_with_bound
                #where_clause_predicates
            {
                #(#prop_fn_impl)*
            }
        }
    } else {
        quote! {
            pub trait #trait_name<R, U, P,#schema_generic_params_with_bound>
            where
                R: rxy_ui::Renderer,
                U: rxy_ui::Schema<R>,
                P: rxy_ui::SchemaProps<R>,
                #where_clause_predicates
            {
                #(#prop_fn_sig;)*
            }

            impl<R, U, P,#schema_generic_params> #trait_name<R, U, P,#schema_generic_params> for rxy_ui::SchemaView<R, U, P,#schema_id>
            where
                R: rxy_ui::Renderer,
                U: rxy_ui::Schema<R>,
                P: rxy_ui::SchemaProps<R>,
                #schema_generic_params_with_bound
                #where_clause_predicates
            {
                #(#prop_fn_impl)*
            }
        }
    };

    (quote! {
        #item_fn
        #[allow(unused_parens)]
        #[derive(Clone, Copy, Default, Debug)]
        pub struct #schema_ident_name<#schema_generic_params>(core::marker::PhantomData<(#schema_generic_params)>);
        #schema_fn_def

        #trait_def_and_impl
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
