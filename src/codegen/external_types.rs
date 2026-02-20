use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::types::ExternalType;

/// Map an ExternalType to a Rust type token stream.
pub fn external_type_tokens(ext: ExternalType) -> TokenStream {
    match ext {
        ExternalType::Boolean => quote! { bool },
        ExternalType::Integer => quote! { i64 },
        ExternalType::Real | ExternalType::Decimal => quote! { f64 },
        ExternalType::CharacterString
        | ExternalType::Uri
        | ExternalType::ScopedName
        | ExternalType::GenericName => quote! { String },
        ExternalType::Date
        | ExternalType::DateTime
        | ExternalType::TmPosition
        | ExternalType::TmDuration => quote! { String },
        ExternalType::Length | ExternalType::Measure | ExternalType::Area | ExternalType::Volume => {
            quote! { f64 }
        }
        ExternalType::Number => quote! { f64 },
        ExternalType::MeasureOrNilReasonList => quote! { Vec<f64> },
        ExternalType::Character => quote! { char },
        ExternalType::GmPoint | ExternalType::DirectPosition => {
            quote! { egml_core::model::geometry::DirectPosition }
        }
        ExternalType::GmMultiSurface => {
            quote! { egml_core::model::geometry::MultiSurface }
        }
        ExternalType::GmSolid => {
            quote! { egml_core::model::geometry::Solid }
        }
        ExternalType::GmSurface => {
            quote! { egml_core::model::geometry::Surface }
        }
        ExternalType::GmTriangulatedSurface => {
            quote! { egml_core::model::geometry::TriangulatedSurface }
        }
        ExternalType::GmMultiCurve => {
            // No direct egml equivalent yet — use Vec of curves
            quote! { Vec<egml_core::model::geometry::Surface> }
        }
        ExternalType::GmMultiPoint => {
            quote! { Vec<egml_core::model::geometry::DirectPosition> }
        }
        ExternalType::GmObject | ExternalType::AnyFeature => {
            quote! { Box<dyn std::any::Any> }
        }
        ExternalType::ScCrs | ExternalType::EngineeringCrs | ExternalType::UnitOfMeasure => {
            quote! { String }
        }
    }
}
