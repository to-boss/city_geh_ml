use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::codegen::external_types::external_type_tokens;
use crate::ir::model::UmlModel;
use crate::ir::types::*;
use crate::util::naming::{escape_keyword, to_snake_case};

/// Convert a UmlTypeRef to a base Rust type token stream (no Option/Vec wrapping).
/// Abstract classes with concrete descendants use the enum dispatch type (same name).
pub fn type_ref_to_tokens(type_ref: &UmlTypeRef, model: &UmlModel) -> TokenStream {
    match type_ref {
        UmlTypeRef::Known(id) => {
            // Look up the name from classes, enums, or datatypes
            if let Some(cls) = model.classes.get(id.as_str()) {
                let name = Ident::new(&cls.name, Span::call_site());
                // Abstract classes → enum dispatch type (same name as the trait)
                // The enum is generated alongside the trait.
                quote! { #name }
            } else if let Some(en) = model.enumerations.get(id.as_str()) {
                let name = Ident::new(&en.name, Span::call_site());
                quote! { #name }
            } else if let Some(dt) = model.data_types.get(id.as_str()) {
                let name = Ident::new(&dt.name, Span::call_site());
                quote! { #name }
            } else {
                // Shouldn't happen if resolution is correct
                quote! { () }
            }
        }
        UmlTypeRef::External(ext) => external_type_tokens(*ext),
        UmlTypeRef::Unresolved(_) => quote! { () },
    }
}

/// Wrap a base type with Option/Vec based on multiplicity — for struct fields.
/// Required abstract types (trait objects) are promoted to Optional since we
/// cannot construct a default value for `Box<dyn Trait>`.
pub fn prop_to_field_type(prop: &UmlProperty, model: &UmlModel) -> TokenStream {
    let base = type_ref_to_tokens(&prop.type_ref, model);
    let is_abstract = is_abstract_prop(prop, model);
    match prop.multiplicity {
        Multiplicity::Required => {
            if is_abstract {
                quote! { Option<#base> }
            } else {
                base
            }
        }
        Multiplicity::Optional => quote! { Option<#base> },
        Multiplicity::Many => quote! { Vec<#base> },
        Multiplicity::RequiredMany => quote! { Vec<#base> },
    }
}

/// Return type for trait method — uses references.
pub fn prop_to_return_type(prop: &UmlProperty, model: &UmlModel) -> TokenStream {
    let base = type_ref_to_tokens(&prop.type_ref, model);
    let is_abstract = is_abstract_prop(prop, model);
    // Check if the base type is a simple Copy type
    let is_copy = matches!(
        &prop.type_ref,
        UmlTypeRef::External(
            ExternalType::Boolean
                | ExternalType::Integer
                | ExternalType::Real
                | ExternalType::Decimal
                | ExternalType::Length
                | ExternalType::Measure
                | ExternalType::Area
                | ExternalType::Volume
                | ExternalType::Number
                | ExternalType::Character
        )
    );
    let is_enum = matches!(&prop.type_ref, UmlTypeRef::Known(id) if model.enumerations.contains_key(id.as_str()));

    match prop.multiplicity {
        Multiplicity::Required => {
            if is_abstract {
                // Abstract types promoted to Optional
                quote! { Option<&#base> }
            } else if is_copy || is_enum {
                base
            } else {
                quote! { &#base }
            }
        }
        Multiplicity::Optional => {
            if is_copy || is_enum {
                quote! { Option<#base> }
            } else {
                quote! { Option<&#base> }
            }
        }
        Multiplicity::Many | Multiplicity::RequiredMany => quote! { &[#base] },
    }
}

/// Generate the body expression for a trait method implementation.
pub fn prop_to_impl_body(prop: &UmlProperty, model: &UmlModel) -> TokenStream {
    let field_name = Ident::new(&escape_keyword(&to_snake_case(&prop.name)), Span::call_site());
    let is_abstract = is_abstract_prop(prop, model);

    let is_copy = matches!(
        &prop.type_ref,
        UmlTypeRef::External(
            ExternalType::Boolean
                | ExternalType::Integer
                | ExternalType::Real
                | ExternalType::Decimal
                | ExternalType::Length
                | ExternalType::Measure
                | ExternalType::Area
                | ExternalType::Volume
                | ExternalType::Number
                | ExternalType::Character
        )
    );
    let is_enum = matches!(&prop.type_ref, UmlTypeRef::Known(id) if model.enumerations.contains_key(id.as_str()));

    match prop.multiplicity {
        Multiplicity::Required => {
            if is_abstract {
                // Abstract types promoted to Optional
                quote! { self.#field_name.as_ref() }
            } else if is_copy || is_enum {
                quote! { self.#field_name }
            } else {
                quote! { &self.#field_name }
            }
        }
        Multiplicity::Optional => {
            if is_copy || is_enum {
                quote! { self.#field_name }
            } else {
                quote! { self.#field_name.as_ref() }
            }
        }
        Multiplicity::Many | Multiplicity::RequiredMany => {
            quote! { &self.#field_name }
        }
    }
}

/// Convert a property name to a snake_case Rust identifier for use as a field name.
pub fn prop_field_ident(name: &str) -> Ident {
    Ident::new(&escape_keyword(&to_snake_case(name)), Span::call_site())
}

/// Convert a property name to a snake_case Rust identifier for use as a method name.
pub fn prop_method_ident(name: &str) -> Ident {
    Ident::new(&escape_keyword(&to_snake_case(name)), Span::call_site())
}

/// Convert a class/trait name to a PascalCase Rust identifier.
pub fn type_ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}

/// Convert an abstract class name to its trait identifier (appends "Trait" suffix).
/// This avoids name collisions with the enum dispatch type which uses the plain name.
pub fn trait_ident(name: &str) -> Ident {
    Ident::new(&format!("{name}Trait"), Span::call_site())
}

/// Convert an abstract class name to its accessor trait identifier (appends "Accessors" suffix).
/// E.g., "AbstractSpaceBoundary" → `AbstractSpaceBoundaryAccessors`.
pub fn accessor_trait_ident(name: &str) -> Ident {
    Ident::new(&format!("{name}Accessors"), Span::call_site())
}

/// Convert a concrete class name to a pluralized snake_case method name for accessor traits.
/// E.g., "WallSurface" → `wall_surfaces`.
pub fn accessor_method_ident(name: &str) -> Ident {
    let snake = to_snake_case(name);
    Ident::new(&escape_keyword(&format!("{snake}s")), Span::call_site())
}

/// Returns true if a property's type resolves to an abstract type (class or data type).
pub fn is_abstract_prop(prop: &UmlProperty, model: &UmlModel) -> bool {
    match &prop.type_ref {
        UmlTypeRef::Known(id) => {
            if let Some(cls) = model.classes.get(id.as_str()) {
                cls.is_abstract
            } else if let Some(dt) = model.data_types.get(id.as_str()) {
                dt.is_abstract
            } else {
                false
            }
        }
        _ => false,
    }
}
