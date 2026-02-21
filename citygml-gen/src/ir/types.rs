/// Reference to a type in the UML model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UmlTypeRef {
    /// References a class, enum, or datatype defined in the model (by xmi:id).
    Known(String),
    /// Maps to a well-known external type (GML geometry, ISO primitive, etc.).
    External(ExternalType),
    /// Could not be resolved — will emit a `()` placeholder with a warning.
    Unresolved(String),
}

/// Well-known external types from ISO 19100, GML, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExternalType {
    Boolean,
    Integer,
    Real,
    Decimal,
    CharacterString,
    Date,
    DateTime,
    Uri,
    Length,
    Measure,
    Area,
    Volume,
    UnitOfMeasure,
    Character,
    ScopedName,
    GenericName,
    TmPosition,
    TmDuration,
    Number,
    MeasureOrNilReasonList,
    GmObject,
    GmPoint,
    GmMultiPoint,
    GmSurface,
    GmMultiSurface,
    GmMultiCurve,
    GmSolid,
    GmTriangulatedSurface,
    ScCrs,
    EngineeringCrs,
    DirectPosition,
    AnyFeature,
}

impl ExternalType {
    /// Try to parse an EA stub or geometry name to an ExternalType.
    pub fn from_name(name: &str) -> Option<ExternalType> {
        match name {
            "Boolean" => Some(ExternalType::Boolean),
            "Integer" => Some(ExternalType::Integer),
            "Real" => Some(ExternalType::Real),
            "Decimal" => Some(ExternalType::Decimal),
            "CharacterString" => Some(ExternalType::CharacterString),
            "Date" => Some(ExternalType::Date),
            "DateTime" => Some(ExternalType::DateTime),
            "URI" => Some(ExternalType::Uri),
            "Length" => Some(ExternalType::Length),
            "Measure" => Some(ExternalType::Measure),
            "Area" => Some(ExternalType::Area),
            "Volume" => Some(ExternalType::Volume),
            "UnitOfMeasure" => Some(ExternalType::UnitOfMeasure),
            "Character" => Some(ExternalType::Character),
            "ScopedName" => Some(ExternalType::ScopedName),
            "GenericName" => Some(ExternalType::GenericName),
            "TM_Position" => Some(ExternalType::TmPosition),
            "TM_Duration" => Some(ExternalType::TmDuration),
            "Number" => Some(ExternalType::Number),
            "MeasureOrNilReasonList" => Some(ExternalType::MeasureOrNilReasonList),
            "GM_Object" => Some(ExternalType::GmObject),
            "GM_Point" => Some(ExternalType::GmPoint),
            "GM_MultiPoint" => Some(ExternalType::GmMultiPoint),
            "GM_Surface" => Some(ExternalType::GmSurface),
            "GM_MultiSurface" => Some(ExternalType::GmMultiSurface),
            "GM_MultiCurve" => Some(ExternalType::GmMultiCurve),
            "GM_Solid" => Some(ExternalType::GmSolid),
            "GM_TriangulatedSurface" => Some(ExternalType::GmTriangulatedSurface),
            "SC_CRS" => Some(ExternalType::ScCrs),
            "EngineeringCRS" => Some(ExternalType::EngineeringCrs),
            "DirectPosition" => Some(ExternalType::DirectPosition),
            "AnyFeature" => Some(ExternalType::AnyFeature),
            _ => None,
        }
    }
}

/// Multiplicity of a UML property.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Multiplicity {
    /// lower=1, upper=1
    Required,
    /// lower=0, upper=1
    Optional,
    /// lower=0, upper=-1 (unbounded)
    Many,
    /// lower=1, upper=-1 (unbounded, at least one)
    RequiredMany,
}

impl Multiplicity {
    pub fn from_bounds(lower: i32, upper: i32) -> Multiplicity {
        // EA quirk: treat -1 lower as 0
        let lo = if lower < 0 { 0 } else { lower };
        let is_unbounded = !(0..=1).contains(&upper);
        match (lo, is_unbounded) {
            (0, false) => Multiplicity::Optional,
            (1, false) => Multiplicity::Required,
            (0, true) => Multiplicity::Many,
            (_, true) => Multiplicity::RequiredMany,
            _ => Multiplicity::Optional,
        }
    }
}

/// A resolved UML property (attribute or navigable association end).
#[derive(Debug, Clone)]
pub struct UmlProperty {
    pub name: String,
    pub type_ref: UmlTypeRef,
    pub multiplicity: Multiplicity,
    pub is_association: bool,
}

/// A resolved UML class.
#[derive(Debug, Clone)]
pub struct UmlClass {
    pub xmi_id: String,
    pub name: String,
    pub package_id: String,
    pub is_abstract: bool,
    pub parent_ids: Vec<String>,
    pub own_properties: Vec<UmlProperty>,
}

/// A resolved UML enumeration.
#[derive(Debug, Clone)]
pub struct UmlEnum {
    pub xmi_id: String,
    pub name: String,
    pub package_id: String,
    pub literals: Vec<String>,
}

/// A resolved UML data type.
#[derive(Debug, Clone)]
pub struct UmlDataType {
    pub xmi_id: String,
    pub name: String,
    pub package_id: String,
    pub is_abstract: bool,
    pub properties: Vec<UmlProperty>,
}

/// A package in the model.
#[derive(Debug, Clone)]
pub struct UmlPackage {
    pub xmi_id: String,
    pub name: String,
    pub parent_id: Option<String>,
}
