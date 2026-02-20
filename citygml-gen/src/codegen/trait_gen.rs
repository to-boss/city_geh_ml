use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;

use crate::codegen::mapper::{prop_method_ident, prop_to_return_type, type_ident};
use crate::ir::model::UmlModel;
use crate::ir::types::UmlClass;
use crate::util::naming::{escape_keyword, to_snake_case};

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

    // Collect all method names already declared in ancestor traits,
    // so we don't redeclare overridden properties.
    let mut ancestor_methods: HashSet<String> = HashSet::new();
    let ancestors = model.ancestor_chain(&cls.xmi_id);
    for ancestor in &ancestors {
        if ancestor.is_abstract {
            for prop in &ancestor.own_properties {
                ancestor_methods.insert(escape_keyword(&to_snake_case(&prop.name)));
            }
        }
    }

    // Own property methods — skip those already declared in an ancestor trait
    let methods: Vec<TokenStream> = cls
        .own_properties
        .iter()
        .filter(|prop| {
            let method_key = escape_keyword(&to_snake_case(&prop.name));
            !ancestor_methods.contains(&method_key)
        })
        .map(|prop| {
            let method_name = prop_method_ident(&prop.name);
            let return_type = prop_to_return_type(prop, model);
            quote! {
                fn #method_name(&self) -> #return_type;
            }
        })
        .collect();

    if supertraits.is_empty() {
        // Root traits need Debug for use in trait objects within #[derive(Debug)] structs
        quote! {
            pub trait #trait_name: std::fmt::Debug {
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
