use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::codegen::mapper::{is_trait_object_prop, prop_field_ident, prop_to_field_type};
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

    // Abstract types with no properties → trait (ADEOf* and others like AbstractGenericAttribute)
    if dt.is_abstract && dt.properties.is_empty() {
        return quote! {
            pub trait #name: std::fmt::Debug {}
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

        let has_trait_objects = dt
            .properties
            .iter()
            .any(|prop| is_trait_object_prop(prop, model));

        if has_trait_objects {
            return quote! {
                #[derive(Debug, Default)]
                pub struct #name {
                    #(#fields,)*
                }
            };
        } else {
            return quote! {
                #[derive(Debug, Clone, Default)]
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
