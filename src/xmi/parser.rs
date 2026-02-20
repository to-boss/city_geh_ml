use std::path::Path;

use quick_xml::events::Event;

use crate::error::GenError;
use crate::xmi::raw::*;
use crate::xmi::reader::open_xmi;

/// Parse an XMI file and produce a RawModel.
pub fn parse_xmi(path: &Path) -> Result<RawModel, GenError> {
    let mut reader = open_xmi(path)?;
    let mut buf = Vec::new();

    let mut model = RawModel::default();

    // State stacks
    let mut package_stack: Vec<String> = Vec::new();
    let mut element_stack: Vec<StackEntry> = Vec::new();

    // Temporary accumulators
    let mut current_class: Option<ClassBuilder> = None;
    let mut current_enum: Option<EnumBuilder> = None;
    let mut current_datatype: Option<DataTypeBuilder> = None;
    let mut current_attr: Option<AttrBuilder> = None;
    let mut in_owned_end = false;

    // Extension section parsing
    let mut in_extension = false;
    let mut in_connector = false;
    let mut connector_target_idref: Option<String> = None;
    let mut connector_target_name: Option<String> = None;

    loop {
        let is_empty;
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(Event::Start(ref e)) => {
                is_empty = false;
                let local_name = e.local_name();
                let local = local_name.as_ref();

                handle_start_or_empty(
                    local,
                    e,
                    &mut model,
                    &mut package_stack,
                    &mut element_stack,
                    &mut current_class,
                    &mut current_enum,
                    &mut current_datatype,
                    &mut current_attr,
                    &mut in_owned_end,
                    &mut in_extension,
                    &mut in_connector,
                    &mut connector_target_idref,
                    &mut connector_target_name,
                )?;
                let _ = is_empty; // suppress warning
            }
            Ok(Event::Empty(ref e)) => {
                is_empty = true;
                let local_name = e.local_name();
                let local_bytes = local_name.as_ref().to_vec();

                handle_start_or_empty(
                    &local_bytes,
                    e,
                    &mut model,
                    &mut package_stack,
                    &mut element_stack,
                    &mut current_class,
                    &mut current_enum,
                    &mut current_datatype,
                    &mut current_attr,
                    &mut in_owned_end,
                    &mut in_extension,
                    &mut in_connector,
                    &mut connector_target_idref,
                    &mut connector_target_name,
                )?;

                // For self-closing elements, immediately simulate an End event
                if is_empty {
                    handle_end(
                        &local_bytes,
                        &mut model,
                        &mut package_stack,
                        &mut element_stack,
                        &mut current_class,
                        &mut current_enum,
                        &mut current_datatype,
                        &mut current_attr,
                        &mut in_owned_end,
                        &mut in_extension,
                        &mut in_connector,
                        &mut connector_target_idref,
                        &mut connector_target_name,
                    );
                }
            }
            Ok(Event::End(ref e)) => {
                let local = e.local_name();
                handle_end(
                    local.as_ref(),
                    &mut model,
                    &mut package_stack,
                    &mut element_stack,
                    &mut current_class,
                    &mut current_enum,
                    &mut current_datatype,
                    &mut current_attr,
                    &mut in_owned_end,
                    &mut in_extension,
                    &mut in_connector,
                    &mut connector_target_idref,
                    &mut connector_target_name,
                );
            }
            Err(e) => return Err(GenError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(model)
}

// ── Builder structs ──────────────────────────────────────────────

#[derive(Debug)]
struct ClassBuilder {
    xmi_id: String,
    name: String,
    is_abstract: bool,
    package_id: String,
    generalizations: Vec<String>,
    attributes: Vec<RawAttribute>,
}

#[derive(Debug)]
struct EnumBuilder {
    xmi_id: String,
    name: String,
    package_id: String,
    literals: Vec<String>,
}

#[derive(Debug)]
struct DataTypeBuilder {
    xmi_id: String,
    name: String,
    package_id: String,
    is_abstract: bool,
    attributes: Vec<RawAttribute>,
}

#[derive(Debug)]
struct AttrBuilder {
    xmi_id: String,
    name: String,
    type_idref: Option<String>,
    lower: i32,
    upper: i32,
    aggregation: Option<String>,
    association_id: Option<String>,
}

#[derive(Debug, Clone)]
enum StackEntry {
    Package(String),
    Class,
    Enum,
    DataType,
    Attribute,
    OwnedEnd,
    Association,
    Other,
}

// ── Attribute helpers ────────────────────────────────────────────

fn get_attr_value(e: &quick_xml::events::BytesStart<'_>, name: &[u8]) -> Option<String> {
    e.attributes().filter_map(|a| a.ok()).find_map(|a| {
        if a.key.as_ref() == name {
            Some(String::from_utf8_lossy(&a.value).into_owned())
        } else {
            None
        }
    })
}

fn get_xmi_attr(e: &quick_xml::events::BytesStart<'_>, local: &str) -> Option<String> {
    let xmi_name = format!("xmi:{local}");
    e.attributes().filter_map(|a| a.ok()).find_map(|a| {
        let key_bytes = a.key.as_ref();
        if key_bytes == xmi_name.as_bytes() {
            Some(String::from_utf8_lossy(&a.value).into_owned())
        } else {
            None
        }
    })
}

// ── Main dispatch ────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn handle_start_or_empty(
    local: &[u8],
    e: &quick_xml::events::BytesStart<'_>,
    model: &mut RawModel,
    package_stack: &mut Vec<String>,
    element_stack: &mut Vec<StackEntry>,
    current_class: &mut Option<ClassBuilder>,
    current_enum: &mut Option<EnumBuilder>,
    current_datatype: &mut Option<DataTypeBuilder>,
    current_attr: &mut Option<AttrBuilder>,
    in_owned_end: &mut bool,
    in_extension: &mut bool,
    in_connector: &mut bool,
    connector_target_idref: &mut Option<String>,
    connector_target_name: &mut Option<String>,
) -> Result<(), GenError> {
    // ── Extension section ────────────────────────────────────────
    if local == b"Extension" {
        *in_extension = true;
        return Ok(());
    }

    if *in_extension {
        handle_extension_element(
            local,
            e,
            model,
            in_connector,
            connector_target_idref,
            connector_target_name,
        );
        return Ok(());
    }

    // ── Main UML model section ───────────────────────────────────
    if local == b"packagedElement" || local == b"nestedPackage" {
        let xmi_type = get_xmi_attr(e, "type").unwrap_or_default();
        let xmi_id = get_xmi_attr(e, "id").unwrap_or_default();
        let name = get_attr_value(e, b"name").unwrap_or_default();

        match xmi_type.as_str() {
            "uml:Package" => {
                let parent_id = package_stack.last().cloned();
                model.packages.push(RawPackage {
                    xmi_id: xmi_id.clone(),
                    name: name.clone(),
                    parent_id,
                });
                package_stack.push(xmi_id.clone());
                element_stack.push(StackEntry::Package(xmi_id));
                return Ok(());
            }
            "uml:Class" => {
                let is_abstract =
                    get_attr_value(e, b"isAbstract").is_some_and(|v| v == "true");
                let pkg = package_stack.last().cloned().unwrap_or_default();
                *current_class = Some(ClassBuilder {
                    xmi_id,
                    name,
                    is_abstract,
                    package_id: pkg,
                    generalizations: Vec::new(),
                    attributes: Vec::new(),
                });
                element_stack.push(StackEntry::Class);
                return Ok(());
            }
            "uml:Enumeration" => {
                let pkg = package_stack.last().cloned().unwrap_or_default();
                *current_enum = Some(EnumBuilder {
                    xmi_id,
                    name,
                    package_id: pkg,
                    literals: Vec::new(),
                });
                element_stack.push(StackEntry::Enum);
                return Ok(());
            }
            "uml:DataType" => {
                let is_abstract =
                    get_attr_value(e, b"isAbstract").is_some_and(|v| v == "true");
                let pkg = package_stack.last().cloned().unwrap_or_default();
                *current_datatype = Some(DataTypeBuilder {
                    xmi_id,
                    name,
                    is_abstract,
                    package_id: pkg,
                    attributes: Vec::new(),
                });
                element_stack.push(StackEntry::DataType);
                return Ok(());
            }
            "uml:Association" => {
                element_stack.push(StackEntry::Association);
                return Ok(());
            }
            _ => {
                element_stack.push(StackEntry::Other);
                return Ok(());
            }
        }
    }

    // ── Inside a class or datatype ───────────────────────────────
    if local == b"generalization" {
        if let Some(cls) = current_class.as_mut() {
            if let Some(general) = get_attr_value(e, b"general") {
                cls.generalizations.push(general);
            }
        }
        return Ok(());
    }

    if local == b"ownedEnd" {
        *in_owned_end = true;
        element_stack.push(StackEntry::OwnedEnd);
        return Ok(());
    }

    if local == b"ownedAttribute" && !*in_owned_end {
        if element_stack.last().is_some_and(|s| matches!(s, StackEntry::Association)) {
            return Ok(());
        }

        let xmi_id = get_xmi_attr(e, "id").unwrap_or_default();
        let name = get_attr_value(e, b"name").unwrap_or_default();
        let aggregation = get_attr_value(e, b"aggregation");
        let association_id = get_attr_value(e, b"association");

        *current_attr = Some(AttrBuilder {
            xmi_id,
            name,
            type_idref: None,
            lower: 1,
            upper: 1,
            aggregation,
            association_id,
        });
        element_stack.push(StackEntry::Attribute);
        return Ok(());
    }

    if local == b"ownedLiteral" {
        if let Some(en) = current_enum.as_mut() {
            if let Some(name) = get_attr_value(e, b"name") {
                en.literals.push(name);
            }
        }
        return Ok(());
    }

    // Type reference inside an attribute
    if local == b"type" {
        if let Some(attr) = current_attr.as_mut() {
            if let Some(idref) = get_xmi_attr(e, "idref") {
                attr.type_idref = Some(idref);
            }
        }
        return Ok(());
    }

    // Multiplicity values
    if local == b"lowerValue" {
        if let Some(attr) = current_attr.as_mut() {
            if let Some(val) = get_attr_value(e, b"value") {
                attr.lower = val.parse::<i32>().unwrap_or(0);
            }
        }
        return Ok(());
    }

    if local == b"upperValue" {
        if let Some(attr) = current_attr.as_mut() {
            if let Some(val) = get_attr_value(e, b"value") {
                attr.upper = val.parse::<i32>().unwrap_or(1);
            }
        }
        return Ok(());
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn handle_end(
    local: &[u8],
    model: &mut RawModel,
    package_stack: &mut Vec<String>,
    element_stack: &mut Vec<StackEntry>,
    current_class: &mut Option<ClassBuilder>,
    current_enum: &mut Option<EnumBuilder>,
    current_datatype: &mut Option<DataTypeBuilder>,
    current_attr: &mut Option<AttrBuilder>,
    in_owned_end: &mut bool,
    in_extension: &mut bool,
    in_connector: &mut bool,
    connector_target_idref: &mut Option<String>,
    connector_target_name: &mut Option<String>,
) {
    if local == b"Extension" {
        *in_extension = false;
        return;
    }

    if *in_extension {
        if local == b"connector" {
            *in_connector = false;
            if let (Some(idref), Some(name)) =
                (connector_target_idref.take(), connector_target_name.take())
            {
                if name.starts_with("GM_")
                    || name == "SC_CRS"
                    || name == "AnyFeature"
                    || name == "EngineeringCRS"
                    || name == "DirectPosition"
                {
                    model.geometry_refs.push((idref, name));
                }
            }
        }
        return;
    }

    if local == b"ownedEnd" {
        *in_owned_end = false;
        if element_stack
            .last()
            .is_some_and(|e| matches!(e, StackEntry::OwnedEnd))
        {
            element_stack.pop();
        }
        return;
    }

    if local == b"ownedAttribute" && !*in_owned_end {
        if let Some(attr_builder) = current_attr.take() {
            let raw_attr = RawAttribute {
                xmi_id: attr_builder.xmi_id,
                name: attr_builder.name,
                type_idref: attr_builder.type_idref,
                lower: attr_builder.lower,
                upper: attr_builder.upper,
                aggregation: attr_builder.aggregation,
                association_id: attr_builder.association_id,
            };
            if let Some(cls) = current_class.as_mut() {
                cls.attributes.push(raw_attr);
            } else if let Some(dt) = current_datatype.as_mut() {
                dt.attributes.push(raw_attr);
            }
        }
        if element_stack
            .last()
            .is_some_and(|e| matches!(e, StackEntry::Attribute))
        {
            element_stack.pop();
        }
        return;
    }

    if local == b"packagedElement" || local == b"nestedPackage" {
        if let Some(entry) = element_stack.pop() {
            match entry {
                StackEntry::Package(_) => {
                    package_stack.pop();
                }
                StackEntry::Class => {
                    if let Some(cls) = current_class.take() {
                        model.classes.push(RawClass {
                            xmi_id: cls.xmi_id,
                            name: cls.name,
                            is_abstract: cls.is_abstract,
                            package_id: cls.package_id,
                            generalizations: cls.generalizations,
                            attributes: cls.attributes,
                        });
                    }
                }
                StackEntry::Enum => {
                    if let Some(en) = current_enum.take() {
                        model.enumerations.push(RawEnum {
                            xmi_id: en.xmi_id,
                            name: en.name,
                            package_id: en.package_id,
                            literals: en.literals,
                        });
                    }
                }
                StackEntry::DataType => {
                    if let Some(dt) = current_datatype.take() {
                        model.data_types.push(RawDataType {
                            xmi_id: dt.xmi_id,
                            name: dt.name,
                            package_id: dt.package_id,
                            is_abstract: dt.is_abstract,
                            attributes: dt.attributes,
                        });
                    }
                }
                StackEntry::Association | StackEntry::Other => {}
                _ => {}
            }
        }
    }
}

fn handle_extension_element(
    local: &[u8],
    e: &quick_xml::events::BytesStart<'_>,
    model: &mut RawModel,
    in_connector: &mut bool,
    connector_target_idref: &mut Option<String>,
    connector_target_name: &mut Option<String>,
) {
    if local == b"connector" {
        *in_connector = true;
        *connector_target_idref = None;
        *connector_target_name = None;
        return;
    }

    if local == b"EAStub" {
        let xmi_id = get_xmi_attr(e, "id").unwrap_or_default();
        let name = get_attr_value(e, b"name").unwrap_or_default();
        if !xmi_id.is_empty() && !name.is_empty() {
            model.ea_stubs.push(RawEaStub { xmi_id, name });
        }
        return;
    }

    if *in_connector {
        if local == b"target" {
            if let Some(idref) = get_xmi_attr(e, "idref") {
                *connector_target_idref = Some(idref);
            }
            return;
        }

        if local == b"model" && connector_target_idref.is_some() {
            if let Some(name) = get_attr_value(e, b"name") {
                if name.starts_with("GM_")
                    || name == "SC_CRS"
                    || name == "AnyFeature"
                    || name == "EngineeringCRS"
                    || name == "DirectPosition"
                {
                    *connector_target_name = Some(name);
                }
            }
        }
    }
}
