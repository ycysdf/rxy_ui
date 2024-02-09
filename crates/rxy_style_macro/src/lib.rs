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

        impl rxy_ui::style::TypedStyleLabel for #ident {}

        impl rxy_ui::XValueWrapper<Self> for #ident {
            fn into_x_value_wrapper(self) -> rxy_ui::XValueWrapper<Self> {
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