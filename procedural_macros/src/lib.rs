
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Ident};
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let ident_builder = Ident::new(&format!("{}Builder", ident), ident.span());

    quote!(
        impl #ident {
            pub fn builder() -> #ident_builder {
                #ident_builder::default()
            }
        }
        pub struct #ident_builder;
    ).into()
}
