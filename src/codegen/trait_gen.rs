use proc_macro2::TokenStream;
use quote::quote;

use crate::codegen::mapper::{prop_method_ident, prop_to_return_type, type_ident};
use crate::ir::model::UmlModel;
use crate::ir::types::UmlClass;

/// Generate a Rust trait from an abstract UML class.
pub fn generate_trait(cls: &UmlClass, model: &UmlModel) -> TokenStream {
    let trait_name = type_ident(&cls.name);

    // Build supertrait list from parent_ids
    let supertraits: Vec<TokenStream> = cls
        .parent_ids
        .iter()
        .filter_map(|pid| model.classes.get(pid.as_str()))
        .filter(|parent| parent.is_abstract)
        .map(|parent| {
            let parent_name = type_ident(&parent.name);
            quote! { #parent_name }
        })
        .collect();

    // Own property methods
    let methods: Vec<TokenStream> = cls
        .own_properties
        .iter()
        .map(|prop| {
            let method_name = prop_method_ident(&prop.name);
            let return_type = prop_to_return_type(prop, model);
            quote! {
                fn #method_name(&self) -> #return_type;
            }
        })
        .collect();

    if supertraits.is_empty() {
        quote! {
            pub trait #trait_name {
                #(#methods)*
            }
        }
    } else {
        quote! {
            pub trait #trait_name: #(#supertraits)+* {
                #(#methods)*
            }
        }
    }
}
