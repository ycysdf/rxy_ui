use crate::force_dynamic_view::common_impl_force_dynamic_view;
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
   parse_macro_input, FnArg, GenericArgument, GenericParam, Generics, ItemFn, ItemStruct, Pat,
   PatType, PathArguments, ReturnType, Type, TypeParam, TypeParamBound, WhereClause,
};

pub enum ParamType {
   Prop,
   Static,
   Event,
   Slot,
   CloneableSlot,
}

#[derive(PartialEq)]
pub enum RequiredPropType {
   Prop,
   Slot,
   CloneableSlot,
}

pub fn schema_common<'a, U>(
   renderer: proc_macro2::TokenStream,
   is_element: bool,
   ident: &Ident,
   schema_name: String,
   items: impl Iterator<Item = (&'a Ident, &'a Type)>,
   generics: &Generics,
   is_struct: bool,
   extra_f: impl FnOnce(
      Option<proc_macro2::TokenStream>,
      proc_macro2::TokenStream,
      Option<&WhereClause>,
      Punctuated<&Ident, Comma>,
      (proc_macro2::TokenStream, Ident),
   ) -> U,
) -> (proc_macro2::TokenStream, U) {
   let schema_snack_name = schema_name.to_case(Case::Snake);
   let schema_pascal_name = schema_name.to_case(Case::Pascal);
   let fn_name = Ident::new(&schema_snack_name, Span::call_site());
   let trait_name = format_ident!("{}SchemaProps", schema_pascal_name);

   let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
   let where_clause_predicates = where_clause.map(|n| &n.predicates);

   let schema_generic_params = generics
      .type_params()
      .map(|param| &param.ident)
      .collect::<Punctuated<&Ident, syn::token::Comma>>();

   let schema_generic_params_with_bound = generics
      .type_params()
      .collect::<Punctuated<&TypeParam, syn::token::Comma>>();

   let schema_generic_params_must_with_bound = generics
      .type_params()
      .filter(|n| !n.bounds.is_empty())
      .collect::<Punctuated<&TypeParam, syn::token::Comma>>();

   let schema_generic_params_with_bound = if schema_generic_params_with_bound.is_empty() {
      None
   } else {
      Some(quote! { #schema_generic_params_with_bound , })
   };
   let ty_generics_turbofish = ty_generics.as_turbofish();
   let schema_ident = Ident::new(&schema_pascal_name, Span::call_site());
   let schema_id = {
      quote! {
          #schema_ident #ty_generics_turbofish
      }
   };

   let mut required_props: Vec<(&Ident, proc_macro2::TokenStream, RequiredPropType)> = vec![];

   let (prop_fn_sig, prop_fn_impl): (Vec<_>, Vec<_>) =
        items
            .enumerate()
            .filter_map(|(i, (param_ident, ty))| {
                let Type::Path(path) = ty else {
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

                        let Type::Path(path) = ty else { return None; };
                        let last_segment = path.path.segments.last()?;

                        (last_segment.ident.to_string(), last_segment, true)
                    } else {
                        (param_type_ident, last_segment, false)
                    };

                    let param_type = if param_type_ident == "Sender" {
                        ParamType::Event
                    } else if param_type_ident == "Static" {
                        if is_required_prop {
                            let PathArguments::AngleBracketed(prop_value_ty) =
                                &prop_value_ty.arguments else {
                                return None;
                            };
                            required_props.push((
                                param_ident,
                                prop_value_ty.args.last().unwrap().to_token_stream(),
                                RequiredPropType::Prop,
                            ));
                        }
                        ParamType::Static
                    } else if param_type_ident == "Slot" {
                        if is_required_prop {
                            required_props.push((
                                param_ident,
                                prop_value_ty.to_token_stream(),
                                RequiredPropType::Slot,
                            ));
                        }
                        ParamType::Slot
                    } else if param_type_ident == "CloneableSlot" {
                        if is_required_prop {
                            required_props.push((
                                param_ident,
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
                                param_ident,
                                prop_value_ty.args.last().unwrap().to_token_stream(),
                                RequiredPropType::Prop,
                            ));
                        }
                        ParamType::Prop
                    } else {
                        return None;
                    };
                    return Some((i, param_ident, prop_value_ty, param_type));
                }
                None
            })
            .map(|(index, name, ty, param_type)| {
                let inner_ty = {
                    if let PathArguments::AngleBracketed(prop_value_ty) =
                        &ty.arguments {
                        Some(prop_value_ty.args.last().unwrap())
                    } else {
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
                        ) -> rxy_ui::RendererSchemaElementView<R, U, VM, P::Props<rxy_ui::ConstIndex<#index,ISP::Prop>>,#schema_id>
                        where
                            P::Props<rxy_ui::ConstIndex<#index,ISP::Prop>>: rxy_ui::SchemaProps<R>
                    },
                        ParamType::Static => static_token,
                        ParamType::Event => quote! {
                        fn #name<ISP: rxy_ui::IntoSchemaProp<R, rxy_ui::EventHandler<#inner_ty>>>(
                            self,
                            value: ISP,
                        ) -> rxy_ui::RendererSchemaElementView<R, U, VM, P::Props<rxy_ui::ConstIndex<#index,ISP::Prop>>,#schema_id>
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
                        ) -> rxy_ui::RendererSchemaView<R, U, P::Props<rxy_ui::ConstIndex<#index,ISP::Prop>>,#schema_id>
                        where
                            P::Props<rxy_ui::ConstIndex<#index,ISP::Prop>>: rxy_ui::SchemaProps<R>
                    },
                        ParamType::Static => static_token,
                        ParamType::Event => quote! {
                        fn #name<ISP: rxy_ui::IntoSchemaProp<R, rxy_ui::EventHandler<#inner_ty>>>(
                            self,
                            value: ISP,
                        ) -> rxy_ui::RendererSchemaView<R, U, P::Props<rxy_ui::ConstIndex<#index,ISP::Prop>>,#schema_id>
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
   let required_fn_params_set = required_props
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

          impl<R, U, VM, P,#schema_generic_params> #trait_name<R, U, VM, P,#schema_generic_params> for rxy_ui::RendererSchemaElementView<R, U, VM, P,#schema_id>
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

          impl<R, U, P,#schema_generic_params> #trait_name<R, U, P,#schema_generic_params> for rxy_ui::RendererSchemaView<R, U, P,#schema_id>
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
   let schema_fn_impl_def = if is_struct {
      if is_element {
         quote! {
             rxy_ui::struct_element_schema_view(<#schema_id as rxy_ui::SchemaElementView<#renderer>>::view).map(rxy_ui::ElementSchemaBoundWrapper)
                 #(#required_fn_params_set)*
         }
      } else {
         quote! {
             rxy_ui::struct_schema_view(<#schema_id as rxy_ui::RendererSchemaView<#renderer>>::view)
                 #(#required_fn_params_set)*
         }
      }
   } else {
      if is_element {
         quote! {
             rxy_ui::element_schema_view(#ident #ty_generics_turbofish, <#schema_id as core::default::Default>::default()).map(rxy_ui::ElementSchemaBoundWrapper)
                 #(#required_fn_params_set)*
         }
      } else {
         quote! {
             rxy_ui::schema_view(#ident #ty_generics_turbofish, <#schema_id as core::default::Default>::default())
                 #(#required_fn_params_set)*
         }
      }
   };

   let schema_fn_def = if is_element {
      quote! {
          #[inline]
          pub fn #fn_name<#schema_generic_params_with_bound>(#(#required_fn_params,)*) -> rxy_ui::RendererSchemaElementView<
              #renderer,
              rxy_ui::ElementSchemaBoundWrapper<impl rxy_ui::SchemaWithElementViewBound<#renderer>>,
              (),
              impl rxy_ui::SchemaProps<#renderer>,
              #schema_id,
          > #where_clause {
              #schema_fn_impl_def
          }
      }
   } else {
      quote! {
          pub fn #fn_name<#schema_generic_params_with_bound>(#(#required_fn_params,)*) -> rxy_ui::RendererSchemaView<
              #renderer,
              impl rxy_ui::Schema<#renderer>,
              impl rxy_ui::SchemaProps<#renderer>,
              #schema_id,
          > #where_clause {
              #schema_fn_impl_def
          }
      }
   };

   let extra = extra_f(
      schema_generic_params_with_bound,
      renderer,
      where_clause,
      schema_generic_params,
      (schema_id, schema_ident),
   );

   (
      quote! {
          #trait_def_and_impl
          #schema_fn_def
      },
      extra,
   )
}

pub fn struct_schema(
   input: TokenStream,
   renderer: proc_macro2::TokenStream,
   is_element: bool,
) -> TokenStream {
   let input_item = parse_macro_input!(input as ItemStruct);

   let (token_stream, extra_token_stream) = schema_common(
      renderer,
      is_element,
      &input_item.ident,
      input_item.ident.to_string(),
      input_item.fields.iter().map(|field| {
         (
            field
               .ident
               .as_ref()
               .expect("schema attribute only support named fields"),
            &field.ty,
         )
      }),
      &input_item.generics,
      true,
      |schema_generic_params_with_bound,
       renderer,
       where_clause,
       schema_generic_params,
       (schema_id, _)| {
         let fields_assign = input_item
            .fields
            .iter()
            .enumerate()
            .map(|(i, n)| {
               let ty = &n.ty;
               let ident = &n.ident;
               quote! {
                   #ident: <#ty as rxy_ui::SchemaParam<#renderer>>::from::<#i>(ctx)
               }
            })
            .collect::<Punctuated<_, syn::token::Comma>>();
         quote! {
             impl<#schema_generic_params_with_bound> rxy_ui::SchemaParam<#renderer> for #schema_id
             #where_clause
             {
                 fn from<const I: usize>(ctx: &mut rxy_ui::InnerSchemaCtx<#renderer>) -> Self {
                     Self {
                         #fields_assign
                     }
                 }
             }
         }
      },
   );

   (quote! {
       #extra_token_stream
       #token_stream
   })
   .into()
}

pub fn fn_schema(_input: TokenStream, item: TokenStream) -> TokenStream {
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
       (_, schema_id)| {
         quote! {
             #[allow(unused_parens)]
             #[derive(Clone, Copy, Default, Debug)]
             pub struct #schema_id<#schema_generic_params>(core::marker::PhantomData<(#schema_generic_params)>);

         }
      },
   );

   (quote! {
       #input_item
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
