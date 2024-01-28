use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(TypedStyle)]
pub fn typed_style(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);

    let ident = item_struct.ident;
    let render = quote!(rxy_bevy::BevyRenderer);
    TokenStream::from(quote! {
        impl Copy for #ident {}
        impl Clone for #ident {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl rxy_bevy_style::TypedStyleLabel for #ident {}

        impl rxy_bevy_style::StyleSheets<#render> for #ident {
            fn style_sheets(
                self,
                ctx: rxy_style::StyleSheetCtx<#render>,
            ) -> (
                impl Iterator<Item = rxy_bevy_style::AppliedStyleSheet> + Send + 'static,
                rxy_bevy_style::StyleSheetsInfo,
            ) {
                rxy_bevy_style::typed_shared_style_sheets(core::any::TypeId::of::<Self>(), ctx)
            }
        }
    })
}