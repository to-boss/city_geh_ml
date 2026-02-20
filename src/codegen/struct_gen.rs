use proc_macro2::TokenStream;
use quote::quote;

use crate::codegen::mapper::{
    prop_field_ident, prop_method_ident, prop_to_field_type, prop_to_impl_body,
    prop_to_return_type, type_ident,
};
use crate::ir::model::UmlModel;
use crate::ir::types::UmlClass;

/// Generate a Rust struct + trait impls for a concrete (non-abstract) UML class.
pub fn generate_struct(cls: &UmlClass, model: &UmlModel) -> TokenStream {
    let struct_name = type_ident(&cls.name);

    // All properties: ancestor root first, own properties last
    let all_props = model.all_properties(&cls.xmi_id);

    // Struct fields
    let fields: Vec<TokenStream> = all_props
        .iter()
        .map(|prop| {
            let field_name = prop_field_ident(&prop.name);
            let field_type = prop_to_field_type(prop, model);
            quote! { pub #field_name: #field_type }
        })
        .collect();

    let struct_def = quote! {
        #[derive(Debug, Clone)]
        pub struct #struct_name {
            #(#fields,)*
        }
    };

    // Generate impl blocks for each ancestor trait + own trait (if abstract parent)
    let ancestors = model.ancestor_chain(&cls.xmi_id);
    let mut impl_blocks: Vec<TokenStream> = Vec::new();

    // Impl for each abstract ancestor (root first → most derived last)
    for ancestor in ancestors.iter().rev() {
        if ancestor.is_abstract {
            let trait_name = type_ident(&ancestor.name);
            let methods: Vec<TokenStream> = ancestor
                .own_properties
                .iter()
                .map(|prop| {
                    let method_name = prop_method_ident(&prop.name);
                    let return_type = prop_to_return_type(prop, model);
                    let body = prop_to_impl_body(prop, model);
                    quote! {
                        fn #method_name(&self) -> #return_type { #body }
                    }
                })
                .collect();

            impl_blocks.push(quote! {
                impl #trait_name for #struct_name {
                    #(#methods)*
                }
            });
        }
    }

    // If the class itself has abstract parents that define its own trait, also impl for direct parents
    // that are abstract — already covered above since ancestor_chain includes direct parents.

    quote! {
        #struct_def
        #(#impl_blocks)*
    }
}

/// Generate a newtype struct for a CodeList class (no attrs, no parents, non-abstract).
pub fn generate_codelist(cls: &UmlClass) -> TokenStream {
    let name = type_ident(&cls.name);
    quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct #name(pub String);
    }
}
