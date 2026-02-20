use std::collections::HashSet;

use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::quote;

use crate::codegen::mapper::{
    prop_field_ident, prop_method_ident, prop_to_field_type,
    prop_to_impl_body, prop_to_return_type, trait_ident, type_ident,
};
use crate::ir::model::UmlModel;
use crate::ir::types::{UmlClass, UmlProperty};
use crate::util::naming::{escape_keyword, to_snake_case};

/// Deduplicate properties by snake_case field name.
/// First occurrence (ancestor/root) wins — this ensures struct field types
/// match the most general ancestor trait definition.
fn dedup_properties<'a>(props: &[&'a UmlProperty]) -> Vec<&'a UmlProperty> {
    let mut map: IndexMap<String, &'a UmlProperty> = IndexMap::new();
    for prop in props {
        let key = escape_keyword(&to_snake_case(&prop.name));
        map.entry(key).or_insert(prop);
    }
    map.into_values().collect()
}

/// Generate a Rust struct + trait impls for a concrete (non-abstract) UML class.
pub fn generate_struct(cls: &UmlClass, model: &UmlModel) -> TokenStream {
    let struct_name = type_ident(&cls.name);

    // All properties: ancestor root first, own properties last
    // Filter out ADEOf* and dead abstract type properties
    let all_props_raw = model.all_properties(&cls.xmi_id);
    let all_props: Vec<&UmlProperty> = dedup_properties(&all_props_raw)
        .into_iter()
        .filter(|prop| !model.should_skip_prop(prop))
        .collect();

    // Struct fields
    let fields: Vec<TokenStream> = all_props
        .iter()
        .map(|prop| {
            let field_name = prop_field_ident(&prop.name);
            let field_type = prop_to_field_type(prop, model);
            quote! { pub #field_name: #field_type }
        })
        .collect();

    // Check if this class is non-cloneable (transitively contains Box<dyn Any>)
    let non_clone = model.non_cloneable_ids();
    let can_clone = !non_clone.contains(&cls.xmi_id);
    let struct_def = if can_clone {
        quote! {
            #[derive(Debug, Clone, Default)]
            pub struct #struct_name {
                #(#fields,)*
            }
        }
    } else {
        quote! {
            #[derive(Debug, Default)]
            pub struct #struct_name {
                #(#fields,)*
            }
        }
    };

    // Generate impl blocks for each ancestor trait + own trait (if abstract parent)
    let ancestors = model.ancestor_chain(&cls.xmi_id);
    let mut impl_blocks: Vec<TokenStream> = Vec::new();

    // Track method names already implemented to avoid duplicates from overridden properties
    let mut seen_methods: HashSet<String> = HashSet::new();

    // Impl for each abstract ancestor (root first → most derived last)
    for ancestor in ancestors.iter().rev() {
        if ancestor.is_abstract {
            let trait_name = trait_ident(&ancestor.name);
            let methods: Vec<TokenStream> = ancestor
                .own_properties
                .iter()
                .filter(|prop| !model.should_skip_prop(prop))
                .filter(|prop| {
                    let method_key = escape_keyword(&to_snake_case(&prop.name));
                    seen_methods.insert(method_key)
                })
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

    quote! {
        #struct_def
        #(#impl_blocks)*
    }
}

/// Generate a newtype struct for a CodeList class (no attrs, no parents, non-abstract).
pub fn generate_codelist(cls: &UmlClass) -> TokenStream {
    let name = type_ident(&cls.name);
    quote! {
        #[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
        pub struct #name(pub String);
    }
}
