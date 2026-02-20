#[derive(Debug, Clone)]
pub struct RawClass {
    pub xmi_id: String,
    pub name: String,
    pub is_abstract: bool,
    pub package_id: String,
    pub generalizations: Vec<String>,
    pub attributes: Vec<RawAttribute>,
}

#[derive(Debug, Clone)]
pub struct RawAttribute {
    pub xmi_id: String,
    pub name: String,
    pub type_idref: Option<String>,
    pub lower: i32,
    pub upper: i32,
    pub aggregation: Option<String>,
    pub association_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RawEnum {
    pub xmi_id: String,
    pub name: String,
    pub package_id: String,
    pub literals: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RawDataType {
    pub xmi_id: String,
    pub name: String,
    pub package_id: String,
    pub is_abstract: bool,
    pub attributes: Vec<RawAttribute>,
}

#[derive(Debug, Clone)]
pub struct RawEaStub {
    pub xmi_id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct RawPackage {
    pub xmi_id: String,
    pub name: String,
    pub parent_id: Option<String>,
}

/// All raw data extracted from one XMI file.
#[derive(Debug, Clone, Default)]
pub struct RawModel {
    pub packages: Vec<RawPackage>,
    pub classes: Vec<RawClass>,
    pub enumerations: Vec<RawEnum>,
    pub data_types: Vec<RawDataType>,
    pub ea_stubs: Vec<RawEaStub>,
    /// Geometry refs extracted from Extension connectors: xmi_idref → GM_ name
    pub geometry_refs: Vec<(String, String)>,
}
