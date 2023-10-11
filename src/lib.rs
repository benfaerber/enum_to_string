use proc_macro;
use syn;
use quote::quote;

use proc_macro::TokenStream;

#[proc_macro_derive(ToJsonString)]
pub fn derive_all_variants(input: TokenStream) -> TokenStream {
    let syn_item: syn::DeriveInput = syn::parse(input).unwrap();
    let enum_name = syn_item.ident;

    let expanded = quote! {
        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let encoded = serde_json::json!(self);
                match encoded.as_str() {
                    Some(ser) => write!(f, "{}", ser),
                    None => write!(f, ""), 
                }
            } 
        }
    };
    expanded.into()
}
