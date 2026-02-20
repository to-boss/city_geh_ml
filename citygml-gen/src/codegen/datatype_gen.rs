use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::codegen::mapper::{prop_field_ident, prop_to_field_type};
use crate::ir::model::UmlModel;
use crate::ir::types::UmlDataType;

/// Generate Rust code for a UML DataType.
///
/// Rules:
/// - Abstract with no properties → empty trait
/// - Has properties → derive struct
/// - No properties, not abstract → codelist newtype
pub fn generate_datatype(dt: &UmlDataType, model: &UmlModel) -> TokenStream {
    let name = Ident::new(&dt.name, Span::call_site());

    // Skip ADEOf* and dead abstract types entirely — no trait or struct generated
    if dt.is_abstract && dt.properties.is_empty() {
        if dt.name.starts_with("ADEOf") {
            return TokenStream::new();
        }
        // Other abstract data types with no properties (e.g. AbstractGenericAttribute)
        // — also dead, skip them
        return TokenStream::new();
    }

    // Has properties → struct with fields (filter out ADEOf* fields)
    if !dt.properties.is_empty() {
        let fields: Vec<TokenStream> = dt
            .properties
            .iter()
            .filter(|prop| !model.should_skip_prop(prop))
            .map(|prop| {
                let field_name = prop_field_ident(&prop.name);
                let field_type = prop_to_field_type(prop, model);
                quote! { pub #field_name: #field_type }
            })
            .collect();

        let non_clone = model.non_cloneable_ids();
        let can_clone = !non_clone.contains(&dt.xmi_id);
        if can_clone {
            return quote! {
                #[derive(Debug, Clone, Default)]
                pub struct #name {
                    #(#fields,)*
                }
            };
        } else {
            return quote! {
                #[derive(Debug, Default)]
                pub struct #name {
                    #(#fields,)*
                }
            };
        }
    }

    // CodeList: no attrs, non-abstract → newtype around String
    quote! {
        #[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
        pub struct #name(pub String);
    }
}
