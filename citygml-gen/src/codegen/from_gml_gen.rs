use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::codegen::external_types::is_geometry_type;
use crate::codegen::mapper::{is_abstract_prop, prop_field_ident, type_ident};
use crate::ir::model::UmlModel;
use crate::ir::types::*;
use crate::util::naming::{escape_keyword, to_snake_case};

/// Properties inherited from GML (gml:AbstractGMLType / gml:AbstractFeatureType)
/// that must use the GML namespace in CityGML 3.0 encoding.
fn is_gml_inherited_property(prop_name: &str) -> bool {
    matches!(
        prop_name,
        "name" | "description" | "identifier" | "boundedBy" | "metaDataProperty" | "location"
    )
}

/// Determine the namespace constant name for a package name.
fn ns_const_for_package(package_name: &str) -> Option<&'static str> {
    match package_name {
        "Core" => Some("crate::namespace::NS_CORE"),
        "Building" => Some("crate::namespace::NS_BUILDING"),
        "Construction" => Some("crate::namespace::NS_CONSTRUCTION"),
        "Bridge" => Some("crate::namespace::NS_BRIDGE"),
        "Tunnel" => Some("crate::namespace::NS_TUNNEL"),
        "Transportation" => Some("crate::namespace::NS_TRANSPORTATION"),
        "Vegetation" => Some("crate::namespace::NS_VEGETATION"),
        "WaterBody" => Some("crate::namespace::NS_WATER_BODY"),
        "LandUse" => Some("crate::namespace::NS_LAND_USE"),
        "Relief" => Some("crate::namespace::NS_RELIEF"),
        "CityFurniture" => Some("crate::namespace::NS_CITY_FURNITURE"),
        "Generics" => Some("crate::namespace::NS_GENERICS"),
        "CityObjectGroup" => Some("crate::namespace::NS_CITY_OBJECT_GROUP"),
        "Appearance" => Some("crate::namespace::NS_APPEARANCE"),
        "PointCloud" => Some("crate::namespace::NS_POINT_CLOUD"),
        "Dynamizer" => Some("crate::namespace::NS_DYNAMIZER"),
        "Versioning" => Some("crate::namespace::NS_VERSIONING"),
        _ => None,
    }
}

/// Information about a property and which namespace it belongs to (from its defining class).
struct PropertyWithNs<'a> {
    prop: &'a UmlProperty,
    ns_const: &'static str,
}

/// Resolve the namespace for a property — uses GML namespace for
/// properties inherited from gml:AbstractGMLType/AbstractFeatureType.
fn ns_for_property(prop: &UmlProperty, package_ns: &'static str) -> &'static str {
    if is_gml_inherited_property(&prop.name) {
        "crate::namespace::NS_GML"
    } else {
        package_ns
    }
}

/// Collect all properties for a class with their defining namespace.
/// Returns properties root-first, own-last.
fn collect_properties_with_ns<'a>(
    cls: &'a UmlClass,
    model: &'a UmlModel,
) -> Vec<PropertyWithNs<'a>> {
    let mut result = Vec::new();

    // Ancestors in reverse (root first)
    let ancestors = model.ancestor_chain(&cls.xmi_id);
    for ancestor in ancestors.iter().rev() {
        let pkg_ns = model
            .package_name(&ancestor.package_id)
            .and_then(ns_const_for_package)
            .unwrap_or("crate::namespace::NS_CORE");
        for prop in &ancestor.own_properties {
            let ns = ns_for_property(prop, pkg_ns);
            result.push(PropertyWithNs { prop, ns_const: ns });
        }
    }

    // Own properties
    let own_pkg_ns = model
        .package_name(&cls.package_id)
        .and_then(ns_const_for_package)
        .unwrap_or("crate::namespace::NS_CORE");
    for prop in &cls.own_properties {
        let ns = ns_for_property(prop, own_pkg_ns);
        result.push(PropertyWithNs {
            prop,
            ns_const: ns,
        });
    }

    result
}

/// Generate a field parsing expression based on property type and multiplicity.
fn gen_field_parse(
    field_ident: &Ident,
    prop: &UmlProperty,
    model: &UmlModel,
) -> TokenStream {
    let is_abstract_class = matches!(&prop.type_ref, UmlTypeRef::Known(id) if model.is_abstract(id));
    let is_enum = matches!(&prop.type_ref, UmlTypeRef::Known(id) if model.enumerations.contains_key(id.as_str()));
    let is_datatype = matches!(&prop.type_ref, UmlTypeRef::Known(id) if model.data_types.contains_key(id.as_str()));
    // Concrete class with actual properties/parents (not a codelist newtype)
    let is_concrete_class = matches!(&prop.type_ref, UmlTypeRef::Known(id)
        if model.classes.get(id.as_str()).is_some_and(|c| {
            !c.is_abstract && (!c.own_properties.is_empty() || !c.parent_ids.is_empty())
        }));
    // Codelist class (no properties, no parents, concrete) — treat like string newtype
    let is_codelist_class = matches!(&prop.type_ref, UmlTypeRef::Known(id)
        if model.classes.get(id.as_str()).is_some_and(|c| {
            !c.is_abstract && c.own_properties.is_empty() && c.parent_ids.is_empty()
        }));
    let is_geometry = matches!(&prop.type_ref, UmlTypeRef::External(ext) if is_geometry_type(*ext));
    let is_unresolved = matches!(&prop.type_ref, UmlTypeRef::Unresolved(_));

    // Skip unresolved types
    if is_unresolved {
        return quote! { sub.skip_element()?; };
    }

    if is_abstract_class
        && let UmlTypeRef::Known(id) = &prop.type_ref
        && let Some(cls) = model.classes.get(id.as_str())
    {
        // For abstract class references, dispatch to the parser fn
        let dispatcher_fn = Ident::new(
            &format!("parse_{}", to_snake_case(&cls.name)),
            Span::call_site(),
        );
        match prop.multiplicity {
            Multiplicity::Required => {
                return quote! {
                    // Read the wrapper element's child to find the concrete type
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        #field_ident = super::dispatchers::#dispatcher_fn(
                            &mut wrapper, &child_info,
                        )?;
                    }
                };
            }
            Multiplicity::Optional => {
                return quote! {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        #field_ident = Some(super::dispatchers::#dispatcher_fn(
                            &mut wrapper, &child_info,
                        )?);
                    }
                };
            }
            Multiplicity::Many | Multiplicity::RequiredMany => {
                return quote! {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        #field_ident.push(super::dispatchers::#dispatcher_fn(
                            &mut wrapper, &child_info,
                        )?);
                    }
                };
            }
        }
    }

    if is_geometry {
        // Geometry types need special handling via gml_geometry parser
        let parse_expr = match &prop.type_ref {
            UmlTypeRef::External(ExternalType::GmMultiSurface) => {
                quote! {
                    let mut geom_sub = sub.subtree();
                    if let Some(geom_info) = geom_sub.next_element()? {
                        if geom_info.local_name == "MultiSurface" {
                            crate::gml_geometry::parse_multi_surface(&mut geom_sub)?
                        } else {
                            geom_sub.skip_element()?;
                            crate::geometry::MultiSurface::default()
                        }
                    } else {
                        crate::geometry::MultiSurface::default()
                    }
                }
            }
            UmlTypeRef::External(ExternalType::GmSolid) => {
                quote! {
                    let mut geom_sub = sub.subtree();
                    if let Some(geom_info) = geom_sub.next_element()? {
                        if geom_info.local_name == "Solid" {
                            crate::gml_geometry::parse_solid(&mut geom_sub)?
                        } else {
                            geom_sub.skip_element()?;
                            crate::geometry::Solid::default()
                        }
                    } else {
                        crate::geometry::Solid::default()
                    }
                }
            }
            UmlTypeRef::External(ExternalType::GmTriangulatedSurface) => {
                quote! {
                    let mut geom_sub = sub.subtree();
                    if let Some(geom_info) = geom_sub.next_element()? {
                        if geom_info.local_name == "TriangulatedSurface" || geom_info.local_name == "Tin" {
                            crate::gml_geometry::parse_triangulated_surface(&mut geom_sub)?
                        } else {
                            geom_sub.skip_element()?;
                            crate::geometry::TriangulatedSurface::default()
                        }
                    } else {
                        crate::geometry::TriangulatedSurface::default()
                    }
                }
            }
            UmlTypeRef::External(ExternalType::GmPoint | ExternalType::DirectPosition) => {
                quote! {
                    let mut geom_sub = sub.subtree();
                    if let Some(geom_info) = geom_sub.next_element()? {
                        if geom_info.local_name == "Point" {
                            crate::gml_geometry::parse_point(&mut geom_sub)?
                        } else {
                            geom_sub.skip_element()?;
                            crate::geometry::DirectPosition::default()
                        }
                    } else {
                        crate::geometry::DirectPosition::default()
                    }
                }
            }
            UmlTypeRef::External(ExternalType::GmMultiCurve) => {
                quote! {
                    let mut geom_sub = sub.subtree();
                    if let Some(geom_info) = geom_sub.next_element()? {
                        if geom_info.local_name == "MultiCurve" {
                            crate::gml_geometry::parse_multi_curve(&mut geom_sub)?
                        } else {
                            geom_sub.skip_element()?;
                            crate::geometry::MultiCurve::default()
                        }
                    } else {
                        crate::geometry::MultiCurve::default()
                    }
                }
            }
            UmlTypeRef::External(ExternalType::GmSurface) => {
                quote! {
                    let mut geom_sub = sub.subtree();
                    if let Some(geom_info) = geom_sub.next_element()? {
                        if geom_info.local_name == "Polygon" {
                            crate::gml_geometry::parse_polygon(&mut geom_sub)?
                        } else {
                            geom_sub.skip_element()?;
                            crate::geometry::Polygon::default()
                        }
                    } else {
                        crate::geometry::Polygon::default()
                    }
                }
            }
            UmlTypeRef::External(ExternalType::GmMultiPoint) => {
                quote! {
                    // MultiPoint → collect pos elements
                    let mut points = Vec::new();
                    let mut geom_sub = sub.subtree();
                    while let Some(geom_info) = geom_sub.next_element()? {
                        if geom_info.local_name == "Point" {
                            points.push(crate::gml_geometry::parse_point(&mut geom_sub)?);
                        } else {
                            geom_sub.skip_element()?;
                        }
                    }
                    points
                }
            }
            _ => quote! { sub.skip_element()?; Default::default() },
        };

        match prop.multiplicity {
            Multiplicity::Required => {
                return quote! { #field_ident = { #parse_expr }; };
            }
            Multiplicity::Optional => {
                return quote! { #field_ident = Some({ #parse_expr }); };
            }
            Multiplicity::Many | Multiplicity::RequiredMany => {
                return quote! { #field_ident.push({ #parse_expr }); };
            }
        }
    }

    // Simple types (String, i64, f64, bool, char, enums, codelists, concrete classes, data types)
    if is_enum
        && let UmlTypeRef::Known(id) = &prop.type_ref
        && let Some(en) = model.enumerations.get(id.as_str())
    {
        // Enum: read text and parse
        let enum_name = type_ident(&en.name);
        match prop.multiplicity {
            Multiplicity::Required => {
                return quote! {
                    #field_ident = #enum_name::from_gml_text(&sub.read_text()?)?;
                };
            }
            Multiplicity::Optional => {
                return quote! {
                    #field_ident = Some(#enum_name::from_gml_text(&sub.read_text()?)?);
                };
            }
            Multiplicity::Many | Multiplicity::RequiredMany => {
                return quote! {
                    #field_ident.push(#enum_name::from_gml_text(&sub.read_text()?)?);
                };
            }
        }
    }

    // Check if it's a codelist (string newtype) or datatype struct
    if is_datatype
        && let UmlTypeRef::Known(id) = &prop.type_ref
        && let Some(dt) = model.data_types.get(id.as_str())
    {
        if dt.properties.is_empty() && !dt.is_abstract {
            // Codelist / string newtype: just read text
            let dt_name = type_ident(&dt.name);
            match prop.multiplicity {
                Multiplicity::Required => {
                    return quote! {
                        #field_ident = #dt_name(sub.read_text()?);
                    };
                }
                Multiplicity::Optional => {
                    return quote! {
                        #field_ident = Some(#dt_name(sub.read_text()?));
                    };
                }
                Multiplicity::Many | Multiplicity::RequiredMany => {
                    return quote! {
                        #field_ident.push(#dt_name(sub.read_text()?));
                    };
                }
            }
        } else if !dt.properties.is_empty() {
            // DataType struct with properties → use FromGml
            let dt_name = type_ident(&dt.name);
            match prop.multiplicity {
                Multiplicity::Required => {
                    return quote! {
                        #field_ident = #dt_name::from_gml(&mut sub)?;
                    };
                }
                Multiplicity::Optional => {
                    return quote! {
                        #field_ident = Some(#dt_name::from_gml(&mut sub)?);
                    };
                }
                Multiplicity::Many | Multiplicity::RequiredMany => {
                    return quote! {
                        #field_ident.push(#dt_name::from_gml(&mut sub)?);
                    };
                }
            }
        } else {
            // Abstract datatype (ADEOf*) → skip
            return quote! { sub.skip_element()?; };
        }
    }

    if is_codelist_class
        && let UmlTypeRef::Known(id) = &prop.type_ref
        && let Some(cls) = model.classes.get(id.as_str())
    {
        // Codelist class (string newtype) → just read text
        let cls_name = type_ident(&cls.name);
        match prop.multiplicity {
            Multiplicity::Required => {
                return quote! {
                    #field_ident = #cls_name(sub.read_text()?);
                };
            }
            Multiplicity::Optional => {
                return quote! {
                    #field_ident = Some(#cls_name(sub.read_text()?));
                };
            }
            Multiplicity::Many | Multiplicity::RequiredMany => {
                return quote! {
                    #field_ident.push(#cls_name(sub.read_text()?));
                };
            }
        }
    }

    if is_concrete_class
        && let UmlTypeRef::Known(id) = &prop.type_ref
        && let Some(cls) = model.classes.get(id.as_str())
    {
        // Concrete class with properties → use from_gml_with_info
        let cls_name = type_ident(&cls.name);
        match prop.multiplicity {
            Multiplicity::Required => {
                return quote! {
                    // The wrapper element may directly contain properties,
                    // or may contain a typed child element.
                    // For concrete class references embedded directly:
                    let child_info = crate::gml_reader::ElementInfo {
                        namespace: info.namespace.clone(),
                        local_name: info.local_name.clone(),
                        attributes: info.attributes.clone(),
                    };
                    #field_ident = #cls_name::from_gml_with_info(&mut sub, &child_info)?;
                };
            }
            Multiplicity::Optional => {
                return quote! {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        #field_ident = Some(#cls_name::from_gml_with_info(&mut wrapper, &child_info)?);
                    } else {
                        // Inline properties under the wrapper
                        #field_ident = Some(#cls_name::from_gml(&mut sub)?);
                    }
                };
            }
            Multiplicity::Many | Multiplicity::RequiredMany => {
                return quote! {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        #field_ident.push(#cls_name::from_gml_with_info(&mut wrapper, &child_info)?);
                    }
                };
            }
        }
    }

    // Special external types
    if let UmlTypeRef::External(ext) = &prop.type_ref {
        match ext {
            ExternalType::MeasureOrNilReasonList => {
                // Space-separated list of floats
                return match prop.multiplicity {
                    Multiplicity::Required => {
                        quote! {
                            #field_ident = sub.read_text()?
                                .split_whitespace()
                                .filter_map(|s| s.parse::<f64>().ok())
                                .collect();
                        }
                    }
                    Multiplicity::Optional => {
                        quote! {
                            let vals: Vec<f64> = sub.read_text()?
                                .split_whitespace()
                                .filter_map(|s| s.parse::<f64>().ok())
                                .collect();
                            #field_ident = Some(vals);
                        }
                    }
                    Multiplicity::Many | Multiplicity::RequiredMany => {
                        quote! {
                            let vals: Vec<f64> = sub.read_text()?
                                .split_whitespace()
                                .filter_map(|s| s.parse::<f64>().ok())
                                .collect();
                            #field_ident.push(vals);
                        }
                    }
                };
            }
            ExternalType::GmObject | ExternalType::AnyFeature => {
                // Can't parse generic Any/GmObject — skip
                return quote! { sub.skip_element()?; };
            }
            _ => {}
        }
    }

    // Fallback: simple FromGml types (String, i64, f64, bool, char)
    match prop.multiplicity {
        Multiplicity::Required => {
            quote! {
                #field_ident = crate::from_gml::FromGml::from_gml(&mut sub)?;
            }
        }
        Multiplicity::Optional => {
            quote! {
                #field_ident = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
            }
        }
        Multiplicity::Many | Multiplicity::RequiredMany => {
            quote! {
                #field_ident.push(crate::from_gml::FromGml::from_gml(&mut sub)?);
            }
        }
    }
}

/// Deduplicate properties by snake_case field name (first occurrence wins).
fn dedup_props<'a>(props: &[&'a UmlProperty]) -> Vec<&'a UmlProperty> {
    let mut map: indexmap::IndexMap<String, &'a UmlProperty> = indexmap::IndexMap::new();
    for prop in props {
        let key = escape_keyword(&to_snake_case(&prop.name));
        map.entry(key).or_insert(prop);
    }
    map.into_values().collect()
}

/// Generate a field initializer expression for FromGml.
fn gen_field_init(prop: &UmlProperty, model: &UmlModel) -> TokenStream {
    let field_name = prop_field_ident(&prop.name);
    let is_abstract = is_abstract_prop(prop, model);

    // Unresolved external types map to () — avoid `let mut x = Default::default()` for unit
    if matches!(&prop.type_ref, UmlTypeRef::Unresolved(_)) {
        return quote! { let #field_name = (); };
    }

    // Required abstract types are promoted to Optional in the struct
    if is_abstract && prop.multiplicity == Multiplicity::Required {
        return quote! { let mut #field_name = None; };
    }

    match prop.multiplicity {
        Multiplicity::Required => {
            match &prop.type_ref {
                UmlTypeRef::External(ExternalType::Boolean) => {
                    quote! { let mut #field_name = false; }
                }
                UmlTypeRef::External(ExternalType::Integer) => {
                    quote! { let mut #field_name = 0i64; }
                }
                UmlTypeRef::External(
                    ExternalType::Real
                    | ExternalType::Decimal
                    | ExternalType::Length
                    | ExternalType::Measure
                    | ExternalType::Area
                    | ExternalType::Volume
                    | ExternalType::Number,
                ) => {
                    quote! { let mut #field_name = 0.0f64; }
                }
                UmlTypeRef::External(ExternalType::Character) => {
                    quote! { let mut #field_name = '\0'; }
                }
                _ => {
                    quote! { let mut #field_name = Default::default(); }
                }
            }
        }
        Multiplicity::Optional => {
            quote! { let mut #field_name = None; }
        }
        Multiplicity::Many | Multiplicity::RequiredMany => {
            quote! { let mut #field_name = Vec::new(); }
        }
    }
}

/// Generate `FromGml` impl for a concrete class.
pub fn generate_from_gml_class(cls: &UmlClass, model: &UmlModel) -> TokenStream {
    let struct_name = type_ident(&cls.name);
    let props_with_ns = collect_properties_with_ns(cls, model);

    // Dedup all properties (first/ancestor occurrence wins), filter out skipped
    let all_props_raw = model.all_properties(&cls.xmi_id);
    let all_props: Vec<&UmlProperty> = dedup_props(&all_props_raw)
        .into_iter()
        .filter(|prop| !model.should_skip_prop(prop))
        .collect();

    // Check if the struct has a featureID property (from AbstractFeature)
    let has_feature_id = all_props.iter().any(|p| p.name == "featureID");

    // Generate field default initializations (skip featureID — it's set from gml:id)
    let field_inits: Vec<TokenStream> = all_props
        .iter()
        .filter(|p| !(has_feature_id && p.name == "featureID"))
        .map(|prop| gen_field_init(prop, model))
        .collect();

    // Dedup match arms by field name — first occurrence (ancestor) wins.
    let mut seen_fields: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut seen_arms: indexmap::IndexMap<String, TokenStream> = indexmap::IndexMap::new();
    for pwn in &props_with_ns {
        let prop = pwn.prop;
        // Skip ADEOf* and dead abstract type properties
        if model.should_skip_prop(prop) {
            continue;
        }
        let field_key = escape_keyword(&to_snake_case(&prop.name));
        if !seen_fields.insert(field_key.clone()) {
            continue; // Skip duplicate property (already handled by ancestor)
        }
        let ns_path: TokenStream = pwn.ns_const.parse().unwrap();
        let xml_name = &prop.name;
        let field_ident = prop_field_ident(&prop.name);
        let is_abstract = is_abstract_prop(prop, model);
        // For Required abstract types (promoted to Optional), parse as Optional
        let parse_expr = if is_abstract && prop.multiplicity == Multiplicity::Required {
            let mut opt_prop = prop.clone();
            opt_prop.multiplicity = Multiplicity::Optional;
            gen_field_parse(&field_ident, &opt_prop, model)
        } else {
            gen_field_parse(&field_ident, prop, model)
        };
        let arm = quote! {
            (#ns_path, #xml_name) => {
                #parse_expr
            }
        };
        seen_arms.insert(field_key, arm);
    }
    let deduped_arms: Vec<TokenStream> = seen_arms.into_values().collect();

    // Build the struct at the end
    let field_names: Vec<Ident> = all_props
        .iter()
        .map(|prop| prop_field_ident(&prop.name))
        .collect();

    let feature_id_init = if has_feature_id {
        // featureID type is ID (a string newtype) — init directly from gml:id attribute
        quote! { let mut feature_id = ID(_gml_id); }
    } else {
        quote! {}
    };

    // When there are no property arms, skip the match entirely to avoid match_single_binding
    let parse_loop = if deduped_arms.is_empty() {
        quote! {
            let mut sub = reader.subtree();
            while let Some(_info) = sub.next_element()? {
                sub.skip_element()?;
            }
        }
    } else {
        quote! {
            let mut sub = reader.subtree();
            while let Some(info) = sub.next_element()? {
                match (info.namespace.as_str(), info.local_name.as_str()) {
                    #(#deduped_arms)*
                    _ => {
                        sub.skip_element()?;
                    }
                }
            }
        }
    };

    quote! {
        impl #struct_name {
            pub fn from_gml_with_info(
                reader: &mut crate::gml_reader::SubtreeReader<'_>,
                info: &crate::gml_reader::ElementInfo,
            ) -> Result<Self, crate::error::ReaderError> {
                use crate::from_gml::FromGml;

                // Extract gml:id attribute
                let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info)
                    .unwrap_or_default();

                // Initialize fields
                #(#field_inits)*
                // Set feature_id from gml:id attribute if the struct has that field
                #feature_id_init

                // Parse children
                #parse_loop

                Ok(#struct_name {
                    #(#field_names,)*
                })
            }
        }

        impl crate::from_gml::FromGml for #struct_name {
            fn from_gml(
                reader: &mut crate::gml_reader::SubtreeReader<'_>,
            ) -> Result<Self, crate::error::ReaderError> {
                // When called without info, we need to read the element ourselves
                // This is used when a class appears inline as a property value
                let info = crate::gml_reader::ElementInfo {
                    namespace: String::new(),
                    local_name: String::new(),
                    attributes: Vec::new(),
                };
                Self::from_gml_with_info(reader, &info)
            }
        }
    }
}

/// Generate `from_gml_text` method for an enum.
pub fn generate_from_gml_enum(uml_enum: &UmlEnum) -> TokenStream {
    let name = type_ident(&uml_enum.name);
    let match_arms: Vec<TokenStream> = uml_enum
        .literals
        .iter()
        .map(|lit| {
            let variant = Ident::new(
                &crate::util::naming::to_pascal_case(lit),
                Span::call_site(),
            );
            let lit_str = lit.as_str();
            quote! { #lit_str => Ok(#name::#variant), }
        })
        .collect();

    quote! {
        impl #name {
            pub fn from_gml_text(text: &str) -> Result<Self, crate::error::ReaderError> {
                match text.trim() {
                    #(#match_arms)*
                    other => Err(crate::error::ReaderError::Parse {
                        message: format!(
                            "Unknown {} value: '{}'",
                            stringify!(#name),
                            other,
                        ),
                    }),
                }
            }
        }
    }
}

/// Generate `FromGml` impl for a codelist (string newtype).
pub fn generate_from_gml_codelist(name: &str) -> TokenStream {
    let name_ident = type_ident(name);
    quote! {
        impl crate::from_gml::FromGml for #name_ident {
            fn from_gml(
                reader: &mut crate::gml_reader::SubtreeReader<'_>,
            ) -> Result<Self, crate::error::ReaderError> {
                Ok(#name_ident(reader.read_text()?))
            }
        }
    }
}

/// Generate `FromGml` impl for a data type struct.
pub fn generate_from_gml_datatype(dt: &UmlDataType, model: &UmlModel) -> TokenStream {
    let name = type_ident(&dt.name);

    if dt.properties.is_empty() {
        // Codelist newtype or empty struct
        if dt.is_abstract {
            return TokenStream::new(); // Skip abstract data types
        }
        return generate_from_gml_codelist(&dt.name);
    }

    // Skip ADEOf* data types
    if dt.name.starts_with("ADEOf") {
        return TokenStream::new();
    }

    let own_ns = model
        .package_name(&dt.package_id)
        .and_then(ns_const_for_package)
        .unwrap_or("crate::namespace::NS_CORE");

    // Filter out skipped properties
    let props: Vec<&UmlProperty> = dt
        .properties
        .iter()
        .filter(|prop| !model.should_skip_prop(prop))
        .collect();

    // Field initialization
    let field_inits: Vec<TokenStream> = props
        .iter()
        .map(|prop| gen_field_init(prop, model))
        .collect();

    let match_arms: Vec<TokenStream> = props
        .iter()
        .map(|prop| {
            let actual_ns = ns_for_property(prop, own_ns);
            let ns_path: TokenStream = actual_ns.parse().unwrap();
            let xml_name = &prop.name;
            let field_ident = prop_field_ident(&prop.name);
            let is_abstract = is_abstract_prop(prop, model);
            // For Required abstract types (promoted to Optional), parse as Optional
            let parse_expr = if is_abstract && prop.multiplicity == Multiplicity::Required {
                let mut opt_prop = (*prop).clone();
                opt_prop.multiplicity = Multiplicity::Optional;
                gen_field_parse(&field_ident, &opt_prop, model)
            } else {
                gen_field_parse(&field_ident, prop, model)
            };
            quote! {
                (#ns_path, #xml_name) => {
                    #parse_expr
                }
            }
        })
        .collect();

    let field_names: Vec<Ident> = props
        .iter()
        .map(|prop| prop_field_ident(&prop.name))
        .collect();

    // When there are no property arms, skip the match entirely to avoid match_single_binding
    let parse_loop = if match_arms.is_empty() {
        quote! {
            let mut sub = reader.subtree();
            while let Some(_info) = sub.next_element()? {
                sub.skip_element()?;
            }
        }
    } else {
        quote! {
            let mut sub = reader.subtree();
            while let Some(info) = sub.next_element()? {
                match (info.namespace.as_str(), info.local_name.as_str()) {
                    #(#match_arms)*
                    _ => {
                        sub.skip_element()?;
                    }
                }
            }
        }
    };

    quote! {
        impl crate::from_gml::FromGml for #name {
            fn from_gml(
                reader: &mut crate::gml_reader::SubtreeReader<'_>,
            ) -> Result<Self, crate::error::ReaderError> {
                use crate::from_gml::FromGml;
                #(#field_inits)*

                #parse_loop

                Ok(#name {
                    #(#field_names,)*
                })
            }
        }
    }
}

/// Generate dispatcher functions for all abstract classes with concrete descendants.
/// Returns enum dispatch types instead of `Box<dyn Trait>`.
pub fn generate_dispatchers(model: &UmlModel) -> TokenStream {
    let mut dispatchers = Vec::new();

    for (abs_id, abs_cls) in &model.classes {
        if !abs_cls.is_abstract {
            continue;
        }

        let descendants = model.concrete_descendants(abs_id);
        if descendants.is_empty() {
            continue;
        }

        let enum_name = type_ident(&abs_cls.name);
        let fn_name = Ident::new(
            &format!("parse_{}", to_snake_case(&abs_cls.name)),
            Span::call_site(),
        );
        let use_box = descendants.len() > 8;

        // Collect match arms for each concrete descendant
        let mut match_arms = Vec::new();
        for cls in &descendants {
            let cls_name = type_ident(&cls.name);
            let variant_name = type_ident(&cls.name);
            let ns = model
                .package_name(&cls.package_id)
                .and_then(ns_const_for_package);
            if let Some(ns_const) = ns {
                let ns_path: TokenStream = ns_const.parse().unwrap();
                let xml_name = &cls.name;
                let construct = if use_box {
                    quote! {
                        Ok(super::#enum_name::#variant_name(
                            Box::new(super::#cls_name::from_gml_with_info(reader, info)?)
                        ))
                    }
                } else {
                    quote! {
                        Ok(super::#enum_name::#variant_name(
                            super::#cls_name::from_gml_with_info(reader, info)?
                        ))
                    }
                };
                match_arms.push(quote! {
                    (#ns_path, #xml_name) => {
                        #construct
                    }
                });
            }
        }

        if match_arms.is_empty() {
            continue;
        }

        dispatchers.push(quote! {
            pub fn #fn_name(
                reader: &mut crate::gml_reader::SubtreeReader<'_>,
                info: &crate::gml_reader::ElementInfo,
            ) -> Result<super::#enum_name, crate::error::ReaderError> {
                match (info.namespace.as_str(), info.local_name.as_str()) {
                    #(#match_arms)*
                    _ => Err(crate::error::ReaderError::UnsupportedFeature {
                        namespace: info.namespace.clone(),
                        local_name: info.local_name.clone(),
                    }),
                }
            }
        });
    }

    quote! {
        #(#dispatchers)*
    }
}
