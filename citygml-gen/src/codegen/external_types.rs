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
            quote! { crate::geometry::DirectPosition }
        }
        ExternalType::GmMultiSurface => {
            quote! { crate::geometry::MultiSurface }
        }
        ExternalType::GmSolid => {
            quote! { crate::geometry::Solid }
        }
        ExternalType::GmSurface => {
            quote! { crate::geometry::Polygon }
        }
        ExternalType::GmTriangulatedSurface => {
            quote! { crate::geometry::TriangulatedSurface }
        }
        ExternalType::GmMultiCurve => {
            quote! { crate::geometry::MultiCurve }
        }
        ExternalType::GmMultiPoint => {
            quote! { Vec<crate::geometry::DirectPosition> }
        }
        ExternalType::GmObject | ExternalType::AnyFeature => {
            quote! { Box<dyn std::any::Any> }
        }
        ExternalType::ScCrs | ExternalType::EngineeringCrs | ExternalType::UnitOfMeasure => {
            quote! { String }
        }
    }
}

/// Check if an ExternalType is a geometry type that can be parsed from GML.
pub fn is_geometry_type(ext: ExternalType) -> bool {
    matches!(
        ext,
        ExternalType::GmPoint
            | ExternalType::DirectPosition
            | ExternalType::GmMultiSurface
            | ExternalType::GmSolid
            | ExternalType::GmSurface
            | ExternalType::GmTriangulatedSurface
            | ExternalType::GmMultiCurve
            | ExternalType::GmMultiPoint
    )
}
