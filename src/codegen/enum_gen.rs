use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::ir::types::UmlEnum;
use crate::util::naming::to_pascal_case;

/// Generate a Rust enum from a UML enumeration.
pub fn generate_enum(uml_enum: &UmlEnum) -> TokenStream {
    let name = Ident::new(&uml_enum.name, Span::call_site());

    let variants: Vec<TokenStream> = uml_enum
        .literals
        .iter()
        .map(|lit| {
            let variant = Ident::new(&to_pascal_case(lit), Span::call_site());
            quote! { #variant }
        })
        .collect();

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum #name {
            #(#variants,)*
        }
    }
}
