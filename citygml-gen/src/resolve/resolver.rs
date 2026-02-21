use indexmap::IndexMap;

use crate::ir::model::{self, UmlModel};
use crate::ir::types::*;
use crate::resolve::topology::topological_sort;
use crate::xmi::raw::RawModel;

/// Build a resolved UML model from raw XMI data.
pub fn build_model(raw: &RawModel, verbose: bool) -> UmlModel {
    // 1. Register packages
    let mut packages: IndexMap<String, UmlPackage> = IndexMap::new();
    for rp in &raw.packages {
        packages.insert(
            rp.xmi_id.clone(),
            UmlPackage {
                xmi_id: rp.xmi_id.clone(),
                name: rp.name.clone(),
                parent_id: rp.parent_id.clone(),
            },
        );
    }

    // 2. Build stub_map from EAStubs
    let mut stub_map: IndexMap<String, ExternalType> = IndexMap::new();
    for stub in &raw.ea_stubs {
        if let Some(ext) = ExternalType::from_name(&stub.name) {
            stub_map.insert(stub.xmi_id.clone(), ext);
        } else if verbose {
            eprintln!("  [warn] Unknown EAStub: {} ({})", stub.name, stub.xmi_id);
        }
    }

    // 3. Add geometry refs from connector metadata
    //    These map xmi:idref → GM_ name for geometry types not in EAStubs
    for (idref, name) in &raw.geometry_refs {
        if !stub_map.contains_key(idref.as_str()) {
            if let Some(ext) = ExternalType::from_name(name) {
                stub_map.insert(idref.clone(), ext);
            } else if verbose {
                eprintln!("  [warn] Unknown geometry ref: {name} ({idref})");
            }
        }
    }

    // 4. Index all type IDs for resolution
    let mut all_type_ids: IndexMap<String, TypeKind> = IndexMap::new();
    for rc in &raw.classes {
        all_type_ids.insert(rc.xmi_id.clone(), TypeKind::Class);
    }
    for re in &raw.enumerations {
        all_type_ids.insert(re.xmi_id.clone(), TypeKind::Enum);
    }
    for rd in &raw.data_types {
        all_type_ids.insert(rd.xmi_id.clone(), TypeKind::DataType);
    }

    // 5. Build resolved classes
    let mut classes: IndexMap<String, UmlClass> = IndexMap::new();
    for rc in &raw.classes {
        let own_properties = rc
            .attributes
            .iter()
            .map(|ra| resolve_attribute(ra, &all_type_ids, &stub_map, verbose))
            .collect();

        classes.insert(
            rc.xmi_id.clone(),
            UmlClass {
                xmi_id: rc.xmi_id.clone(),
                name: rc.name.clone(),
                package_id: rc.package_id.clone(),
                is_abstract: rc.is_abstract,
                parent_ids: rc.generalizations.clone(),
                own_properties,
            },
        );
    }

    // 6. Build resolved enumerations
    let mut enumerations: IndexMap<String, UmlEnum> = IndexMap::new();
    for re in &raw.enumerations {
        enumerations.insert(
            re.xmi_id.clone(),
            UmlEnum {
                xmi_id: re.xmi_id.clone(),
                name: re.name.clone(),
                package_id: re.package_id.clone(),
                literals: re.literals.clone(),
            },
        );
    }

    // 7. Build resolved data types
    let mut data_types: IndexMap<String, UmlDataType> = IndexMap::new();
    for rd in &raw.data_types {
        let properties = rd
            .attributes
            .iter()
            .map(|ra| resolve_attribute(ra, &all_type_ids, &stub_map, verbose))
            .collect();

        data_types.insert(
            rd.xmi_id.clone(),
            UmlDataType {
                xmi_id: rd.xmi_id.clone(),
                name: rd.name.clone(),
                package_id: rd.package_id.clone(),
                is_abstract: rd.is_abstract,
                properties,
            },
        );
    }

    // 8. Topological sort
    let sorted_class_ids = topological_sort(&classes);

    let mut model = model::new_uml_model(
        packages,
        classes,
        enumerations,
        data_types,
        sorted_class_ids,
    );
    model.compute_caches();
    model
}

#[derive(Debug, Clone, Copy)]
enum TypeKind {
    Class,
    Enum,
    DataType,
}

fn resolve_attribute(
    ra: &crate::xmi::raw::RawAttribute,
    all_type_ids: &IndexMap<String, TypeKind>,
    stub_map: &IndexMap<String, ExternalType>,
    verbose: bool,
) -> UmlProperty {
    let type_ref = match &ra.type_idref {
        Some(idref) => {
            if all_type_ids.contains_key(idref.as_str()) {
                UmlTypeRef::Known(idref.clone())
            } else if let Some(&ext) = stub_map.get(idref.as_str()) {
                UmlTypeRef::External(ext)
            } else {
                if verbose {
                    eprintln!(
                        "  [warn] Unresolved type ref: {} for attribute '{}'",
                        idref, ra.name
                    );
                }
                UmlTypeRef::Unresolved(idref.clone())
            }
        }
        None => {
            if verbose {
                eprintln!("  [warn] No type ref for attribute '{}'", ra.name);
            }
            UmlTypeRef::Unresolved("(none)".into())
        }
    };

    let multiplicity = Multiplicity::from_bounds(ra.lower, ra.upper);

    UmlProperty {
        name: ra.name.clone(),
        type_ref,
        multiplicity,
        is_association: ra.association_id.is_some(),
    }
}
