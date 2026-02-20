use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::codegen::mapper::{prop_field_ident, prop_to_field_type};
use crate::ir::model::UmlModel;
use crate::ir::types::UmlDataType;

/// Generate Rust code for a UML DataType.
///
/// Rules:
/// - Name starts with "ADEOf" and is abstract → empty trait
/// - Has properties → derive struct
/// - No properties, not abstract, name ends with "Value" → codelist newtype
/// - Otherwise → empty struct
pub fn generate_datatype(dt: &UmlDataType, model: &UmlModel) -> TokenStream {
    let name = Ident::new(&dt.name, Span::call_site());

    // ADEOf* abstract types → trait
    if dt.name.starts_with("ADEOf") && dt.is_abstract {
        return quote! {
            pub trait #name {}
        };
    }

    // Has properties → struct with fields
    if !dt.properties.is_empty() {
        let fields: Vec<TokenStream> = dt
            .properties
            .iter()
            .map(|prop| {
                let field_name = prop_field_ident(&prop.name);
                let field_type = prop_to_field_type(prop, model);
                quote! { pub #field_name: #field_type }
            })
            .collect();

        return quote! {
            #[derive(Debug, Clone)]
            pub struct #name {
                #(#fields,)*
            }
        };
    }

    // CodeList: no attrs, non-abstract, name ends with "Value" or is just a named type
    // Use a newtype around String
    quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct #name(pub String);
    }
}
