#![allow(unused_imports, unused_mut, unused_variables)]
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RelativeToTerrain {
    #[default]
    EntirelyAboveTerrain,
    SubstantiallyAboveTerrain,
    SubstantiallyAboveAndBelowTerrain,
    SubstantiallyBelowTerrain,
    EntirelyBelowTerrain,
}
impl RelativeToTerrain {
    pub fn from_gml_text(text: &str) -> Result<Self, crate::error::ReaderError> {
        match text.trim() {
            "entirelyAboveTerrain" => Ok(RelativeToTerrain::EntirelyAboveTerrain),
            "substantiallyAboveTerrain" => {
                Ok(RelativeToTerrain::SubstantiallyAboveTerrain)
            }
            "substantiallyAboveAndBelowTerrain" => {
                Ok(RelativeToTerrain::SubstantiallyAboveAndBelowTerrain)
            }
            "substantiallyBelowTerrain" => {
                Ok(RelativeToTerrain::SubstantiallyBelowTerrain)
            }
            "entirelyBelowTerrain" => Ok(RelativeToTerrain::EntirelyBelowTerrain),
            other => {
                Err(crate::error::ReaderError::Parse {
                    message: format!(
                        "Unknown {} value: '{}'", stringify!(RelativeToTerrain), other,
                    ),
                })
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RelativeToWater {
    #[default]
    EntirelyAboveWaterSurface,
    SubstantiallyAboveWaterSurface,
    SubstantiallyAboveAndBelowWaterSurface,
    SubstantiallyBelowWaterSurface,
    EntirelyBelowWaterSurface,
    TemporarilyAboveAndBelowWaterSurface,
}
impl RelativeToWater {
    pub fn from_gml_text(text: &str) -> Result<Self, crate::error::ReaderError> {
        match text.trim() {
            "entirelyAboveWaterSurface" => Ok(RelativeToWater::EntirelyAboveWaterSurface),
            "substantiallyAboveWaterSurface" => {
                Ok(RelativeToWater::SubstantiallyAboveWaterSurface)
            }
            "substantiallyAboveAndBelowWaterSurface" => {
                Ok(RelativeToWater::SubstantiallyAboveAndBelowWaterSurface)
            }
            "substantiallyBelowWaterSurface" => {
                Ok(RelativeToWater::SubstantiallyBelowWaterSurface)
            }
            "entirelyBelowWaterSurface" => Ok(RelativeToWater::EntirelyBelowWaterSurface),
            "temporarilyAboveAndBelowWaterSurface" => {
                Ok(RelativeToWater::TemporarilyAboveAndBelowWaterSurface)
            }
            other => {
                Err(crate::error::ReaderError::Parse {
                    message: format!(
                        "Unknown {} value: '{}'", stringify!(RelativeToWater), other,
                    ),
                })
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SpaceType {
    #[default]
    Closed,
    Open,
    SemiOpen,
}
impl SpaceType {
    pub fn from_gml_text(text: &str) -> Result<Self, crate::error::ReaderError> {
        match text.trim() {
            "closed" => Ok(SpaceType::Closed),
            "open" => Ok(SpaceType::Open),
            "semiOpen" => Ok(SpaceType::SemiOpen),
            other => {
                Err(crate::error::ReaderError::Parse {
                    message: format!(
                        "Unknown {} value: '{}'", stringify!(SpaceType), other,
                    ),
                })
            }
        }
    }
}
pub trait AbstractGenericAttribute: std::fmt::Debug {}
pub trait ADEOfAbstractAppearance: std::fmt::Debug {}
pub trait ADEOfAbstractCityObject: std::fmt::Debug {}
pub trait ADEOfAbstractDynamizer: std::fmt::Debug {}
pub trait ADEOfAbstractFeature: std::fmt::Debug {}
pub trait ADEOfAbstractFeatureWithLifespan: std::fmt::Debug {}
pub trait ADEOfAbstractLogicalSpace: std::fmt::Debug {}
pub trait ADEOfAbstractOccupiedSpace: std::fmt::Debug {}
pub trait ADEOfAbstractPhysicalSpace: std::fmt::Debug {}
pub trait ADEOfAbstractPointCloud: std::fmt::Debug {}
pub trait ADEOfAbstractSpace: std::fmt::Debug {}
pub trait ADEOfAbstractSpaceBoundary: std::fmt::Debug {}
pub trait ADEOfAbstractThematicSurface: std::fmt::Debug {}
pub trait ADEOfAbstractUnoccupiedSpace: std::fmt::Debug {}
pub trait ADEOfAbstractVersion: std::fmt::Debug {}
pub trait ADEOfAbstractVersionTransition: std::fmt::Debug {}
pub trait ADEOfAddress: std::fmt::Debug {}
pub trait ADEOfCityModel: std::fmt::Debug {}
pub trait ADEOfClosureSurface: std::fmt::Debug {}
#[derive(Debug, Default)]
pub struct CityModelMember {
    pub city_object_member: Option<Box<dyn AbstractCityObject>>,
    pub appearance_member: Option<Box<dyn AbstractAppearance>>,
    pub version_member: Option<Box<dyn AbstractVersion>>,
    pub version_transition_member: Option<Box<dyn AbstractVersionTransition>>,
    pub feature_member: Option<Box<dyn AbstractFeature>>,
}
impl crate::from_gml::FromGml for CityModelMember {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut city_object_member = None;
        let mut appearance_member = None;
        let mut version_member = None;
        let mut version_transition_member = None;
        let mut feature_member = None;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "cityObjectMember") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        city_object_member = Some(
                            super::dispatchers::parse_dyn_abstract_city_object(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    }
                }
                (crate::namespace::NS_CORE, "appearanceMember") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        appearance_member = Some(
                            super::dispatchers::parse_dyn_abstract_appearance(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    }
                }
                (crate::namespace::NS_CORE, "versionMember") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        version_member = Some(
                            super::dispatchers::parse_dyn_abstract_version(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    }
                }
                (crate::namespace::NS_CORE, "versionTransitionMember") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        version_transition_member = Some(
                            super::dispatchers::parse_dyn_abstract_version_transition(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    }
                }
                (crate::namespace::NS_CORE, "featureMember") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        feature_member = Some(
                            super::dispatchers::parse_dyn_abstract_feature(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    }
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(CityModelMember {
            city_object_member,
            appearance_member,
            version_member,
            version_transition_member,
            feature_member,
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct DoubleOrNilReason {
    pub value: f64,
    pub nil_reason: NilReason,
}
impl crate::from_gml::FromGml for DoubleOrNilReason {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut value = 0.0f64;
        let mut nil_reason = Default::default();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "value") => {
                    value = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_CORE, "nilReason") => {
                    nil_reason = NilReason::from_gml(&mut sub)?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(DoubleOrNilReason {
            value,
            nil_reason,
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct ExternalReference {
    pub target_resource: String,
    pub information_system: Option<String>,
    pub relation_type: Option<String>,
}
impl crate::from_gml::FromGml for ExternalReference {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut target_resource = Default::default();
        let mut information_system = None;
        let mut relation_type = None;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "targetResource") => {
                    target_resource = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_CORE, "informationSystem") => {
                    information_system = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_CORE, "relationType") => {
                    relation_type = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(ExternalReference {
            target_resource,
            information_system,
            relation_type,
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct NilReason {
    pub nil_reason_enumeration: NilReasonEnumeration,
    pub uri: String,
}
impl crate::from_gml::FromGml for NilReason {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut nil_reason_enumeration = Default::default();
        let mut uri = Default::default();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "nilReasonEnumeration") => {
                    nil_reason_enumeration = NilReasonEnumeration(sub.read_text()?);
                }
                (crate::namespace::NS_CORE, "URI") => {
                    uri = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(NilReason {
            nil_reason_enumeration,
            uri,
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct Occupancy {
    pub number_of_occupants: i64,
    pub interval: Option<IntervalValue>,
    pub occupant_type: Option<OccupantTypeValue>,
}
impl crate::from_gml::FromGml for Occupancy {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut number_of_occupants = 0i64;
        let mut interval = None;
        let mut occupant_type = None;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "numberOfOccupants") => {
                    number_of_occupants = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_CORE, "interval") => {
                    interval = Some(IntervalValue(sub.read_text()?));
                }
                (crate::namespace::NS_CORE, "occupantType") => {
                    occupant_type = Some(OccupantTypeValue(sub.read_text()?));
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Occupancy {
            number_of_occupants,
            interval,
            occupant_type,
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct QualifiedArea {
    pub area: f64,
    pub type_of_area: QualifiedAreaTypeValue,
}
impl crate::from_gml::FromGml for QualifiedArea {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut area = 0.0f64;
        let mut type_of_area = Default::default();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "area") => {
                    area = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_CORE, "typeOfArea") => {
                    type_of_area = QualifiedAreaTypeValue(sub.read_text()?);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(QualifiedArea {
            area,
            type_of_area,
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct QualifiedVolume {
    pub volume: f64,
    pub type_of_volume: QualifiedVolumeTypeValue,
}
impl crate::from_gml::FromGml for QualifiedVolume {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut volume = 0.0f64;
        let mut type_of_volume = Default::default();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "volume") => {
                    volume = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_CORE, "typeOfVolume") => {
                    type_of_volume = QualifiedVolumeTypeValue(sub.read_text()?);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(QualifiedVolume {
            volume,
            type_of_volume,
        })
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct XALAddress(pub String);
impl crate::from_gml::FromGml for XALAddress {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(XALAddress(reader.read_text()?))
    }
}
pub trait AbstractFeature: std::fmt::Debug {
    fn feature_id(&self) -> &ID;
    fn identifier(&self) -> Option<&String>;
    fn name(&self) -> &[String];
    fn description(&self) -> Option<&String>;
    fn ade_of_abstract_feature(&self) -> &[Box<dyn ADEOfAbstractFeature>];
}
#[derive(Debug, Clone, Default)]
pub struct Code {
    pub code_space: Option<String>,
}
impl Code {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut code_space = None;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "codeSpace") => {
                    code_space = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Code { code_space })
    }
}
impl crate::from_gml::FromGml for Code {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        let info = crate::gml_reader::ElementInfo {
            namespace: String::new(),
            local_name: String::new(),
            attributes: Vec::new(),
        };
        Self::from_gml_with_info(reader, &info)
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct DoubleBetween0and1(pub String);
impl crate::from_gml::FromGml for DoubleBetween0and1 {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(DoubleBetween0and1(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default)]
pub struct DoubleBetween0and1List {
    pub list: DoubleBetween0and1,
}
impl DoubleBetween0and1List {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut list = Default::default();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "list") => {
                    list = DoubleBetween0and1(sub.read_text()?);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(DoubleBetween0and1List { list })
    }
}
impl crate::from_gml::FromGml for DoubleBetween0and1List {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        let info = crate::gml_reader::ElementInfo {
            namespace: String::new(),
            local_name: String::new(),
            attributes: Vec::new(),
        };
        Self::from_gml_with_info(reader, &info)
    }
}
#[derive(Debug, Clone, Default)]
pub struct DoubleList {
    pub list: f64,
}
impl DoubleList {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut list = 0.0f64;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "list") => {
                    list = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(DoubleList { list })
    }
}
impl crate::from_gml::FromGml for DoubleList {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        let info = crate::gml_reader::ElementInfo {
            namespace: String::new(),
            local_name: String::new(),
            attributes: Vec::new(),
        };
        Self::from_gml_with_info(reader, &info)
    }
}
#[derive(Debug, Clone, Default)]
pub struct DoubleOrNilReasonList {
    pub list: DoubleOrNilReason,
}
impl DoubleOrNilReasonList {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut list = Default::default();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "list") => {
                    list = DoubleOrNilReason::from_gml(&mut sub)?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(DoubleOrNilReasonList { list })
    }
}
impl crate::from_gml::FromGml for DoubleOrNilReasonList {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        let info = crate::gml_reader::ElementInfo {
            namespace: String::new(),
            local_name: String::new(),
            attributes: Vec::new(),
        };
        Self::from_gml_with_info(reader, &info)
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ID(pub String);
impl crate::from_gml::FromGml for ID {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(ID(reader.read_text()?))
    }
}
#[derive(Debug, Default)]
pub struct ImplicitGeometry {
    pub object_id: ID,
    pub transformation_matrix: TransformationMatrix4x4,
    pub mime_type: Option<MimeTypeValue>,
    pub library_object: Option<String>,
    pub relative_geometry: Option<Box<dyn std::any::Any>>,
    pub reference_point: crate::geometry::DirectPosition,
    pub appearance: Vec<Box<dyn AbstractAppearance>>,
}
impl ImplicitGeometry {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut object_id = Default::default();
        let mut transformation_matrix = Default::default();
        let mut mime_type = None;
        let mut library_object = None;
        let mut relative_geometry = None;
        let mut reference_point = Default::default();
        let mut appearance = Vec::new();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "objectID") => {
                    object_id = ID(sub.read_text()?);
                }
                (crate::namespace::NS_CORE, "transformationMatrix") => {
                    let child_info = crate::gml_reader::ElementInfo {
                        namespace: info.namespace.clone(),
                        local_name: info.local_name.clone(),
                        attributes: info.attributes.clone(),
                    };
                    transformation_matrix = TransformationMatrix4x4::from_gml_with_info(
                        &mut sub,
                        &child_info,
                    )?;
                }
                (crate::namespace::NS_CORE, "mimeType") => {
                    mime_type = Some(MimeTypeValue(sub.read_text()?));
                }
                (crate::namespace::NS_CORE, "libraryObject") => {
                    library_object = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "relativeGeometry") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "referencePoint") => {
                    reference_point = {
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
                    };
                }
                (crate::namespace::NS_CORE, "appearance") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        appearance
                            .push(
                                super::dispatchers::parse_dyn_abstract_appearance(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(ImplicitGeometry {
            object_id,
            transformation_matrix,
            mime_type,
            library_object,
            relative_geometry,
            reference_point,
            appearance,
        })
    }
}
impl crate::from_gml::FromGml for ImplicitGeometry {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        let info = crate::gml_reader::ElementInfo {
            namespace: String::new(),
            local_name: String::new(),
            attributes: Vec::new(),
        };
        Self::from_gml_with_info(reader, &info)
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct IntegerBetween0and3(pub String);
impl crate::from_gml::FromGml for IntegerBetween0and3 {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(IntegerBetween0and3(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct IntervalValue(pub String);
impl crate::from_gml::FromGml for IntervalValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(IntervalValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct MimeTypeValue(pub String);
impl crate::from_gml::FromGml for MimeTypeValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(MimeTypeValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct NilReasonEnumeration(pub String);
impl crate::from_gml::FromGml for NilReasonEnumeration {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(NilReasonEnumeration(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct OccupantTypeValue(pub String);
impl crate::from_gml::FromGml for OccupantTypeValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(OccupantTypeValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct QualifiedAreaTypeValue(pub String);
impl crate::from_gml::FromGml for QualifiedAreaTypeValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(QualifiedAreaTypeValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct QualifiedVolumeTypeValue(pub String);
impl crate::from_gml::FromGml for QualifiedVolumeTypeValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(QualifiedVolumeTypeValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct RelationTypeValue(pub String);
impl crate::from_gml::FromGml for RelationTypeValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(RelationTypeValue(reader.read_text()?))
    }
}
pub trait AbstractFeatureWithLifespan: AbstractFeature {
    fn creation_date(&self) -> Option<&String>;
    fn termination_date(&self) -> Option<&String>;
    fn valid_from(&self) -> Option<&String>;
    fn valid_to(&self) -> Option<&String>;
    fn ade_of_abstract_feature_with_lifespan(
        &self,
    ) -> &[Box<dyn ADEOfAbstractFeatureWithLifespan>];
}
pub trait AbstractPointCloud: AbstractFeature {
    fn ade_of_abstract_point_cloud(&self) -> &[Box<dyn ADEOfAbstractPointCloud>];
}
#[derive(Debug, Default)]
pub struct Address {
    pub feature_id: ID,
    pub identifier: Option<String>,
    pub name: Vec<String>,
    pub description: Option<String>,
    pub ade_of_abstract_feature: Vec<Box<dyn ADEOfAbstractFeature>>,
    pub ade_of_address: Vec<Box<dyn ADEOfAddress>>,
    pub multi_point: Option<Vec<crate::geometry::DirectPosition>>,
    pub xal_address: XALAddress,
}
impl AbstractFeature for Address {
    fn feature_id(&self) -> &ID {
        &self.feature_id
    }
    fn identifier(&self) -> Option<&String> {
        self.identifier.as_ref()
    }
    fn name(&self) -> &[String] {
        &self.name
    }
    fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    fn ade_of_abstract_feature(&self) -> &[Box<dyn ADEOfAbstractFeature>] {
        &self.ade_of_abstract_feature
    }
}
impl Address {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut identifier = None;
        let mut name = Vec::new();
        let mut description = None;
        let mut ade_of_abstract_feature = Vec::new();
        let mut ade_of_address = Vec::new();
        let mut multi_point = None;
        let mut xal_address = Default::default();
        let mut feature_id = ID(_gml_id);
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "featureID") => {
                    feature_id = ID(sub.read_text()?);
                }
                (crate::namespace::NS_GML, "identifier") => {
                    identifier = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_GML, "name") => {
                    name.push(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_GML, "description") => {
                    description = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "adeOfAbstractFeature") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "adeOfAddress") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "multiPoint") => {
                    multi_point = Some({
                        let mut points = Vec::new();
                        let mut geom_sub = sub.subtree();
                        while let Some(geom_info) = geom_sub.next_element()? {
                            if geom_info.local_name == "Point" {
                                points
                                    .push(crate::gml_geometry::parse_point(&mut geom_sub)?);
                            } else {
                                geom_sub.skip_element()?;
                            }
                        }
                        points
                    });
                }
                (crate::namespace::NS_CORE, "xalAddress") => {
                    xal_address = XALAddress(sub.read_text()?);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Address {
            feature_id,
            identifier,
            name,
            description,
            ade_of_abstract_feature,
            ade_of_address,
            multi_point,
            xal_address,
        })
    }
}
impl crate::from_gml::FromGml for Address {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        let info = crate::gml_reader::ElementInfo {
            namespace: String::new(),
            local_name: String::new(),
            attributes: Vec::new(),
        };
        Self::from_gml_with_info(reader, &info)
    }
}
#[derive(Debug, Clone, Default)]
pub struct TransformationMatrix2x2 {
    pub list: f64,
}
impl TransformationMatrix2x2 {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut list = 0.0f64;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "list") => {
                    list = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(TransformationMatrix2x2 { list })
    }
}
impl crate::from_gml::FromGml for TransformationMatrix2x2 {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        let info = crate::gml_reader::ElementInfo {
            namespace: String::new(),
            local_name: String::new(),
            attributes: Vec::new(),
        };
        Self::from_gml_with_info(reader, &info)
    }
}
#[derive(Debug, Clone, Default)]
pub struct TransformationMatrix3x4 {
    pub list: f64,
}
impl TransformationMatrix3x4 {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut list = 0.0f64;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "list") => {
                    list = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(TransformationMatrix3x4 { list })
    }
}
impl crate::from_gml::FromGml for TransformationMatrix3x4 {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        let info = crate::gml_reader::ElementInfo {
            namespace: String::new(),
            local_name: String::new(),
            attributes: Vec::new(),
        };
        Self::from_gml_with_info(reader, &info)
    }
}
#[derive(Debug, Clone, Default)]
pub struct TransformationMatrix4x4 {
    pub list: f64,
}
impl TransformationMatrix4x4 {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut list = 0.0f64;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "list") => {
                    list = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(TransformationMatrix4x4 { list })
    }
}
impl crate::from_gml::FromGml for TransformationMatrix4x4 {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        let info = crate::gml_reader::ElementInfo {
            namespace: String::new(),
            local_name: String::new(),
            attributes: Vec::new(),
        };
        Self::from_gml_with_info(reader, &info)
    }
}
#[derive(Debug, Clone, Default)]
pub struct MeasureOrNilReasonList {
    pub list: DoubleOrNilReason,
    pub uom: String,
}
impl MeasureOrNilReasonList {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut list = Default::default();
        let mut uom = Default::default();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "list") => {
                    list = DoubleOrNilReason::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_CORE, "uom") => {
                    uom = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(MeasureOrNilReasonList {
            list,
            uom,
        })
    }
}
impl crate::from_gml::FromGml for MeasureOrNilReasonList {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        let info = crate::gml_reader::ElementInfo {
            namespace: String::new(),
            local_name: String::new(),
            attributes: Vec::new(),
        };
        Self::from_gml_with_info(reader, &info)
    }
}
#[derive(Debug, Clone, Default)]
pub struct OtherRelationTypeValue {}
impl OtherRelationTypeValue {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(OtherRelationTypeValue {})
    }
}
impl crate::from_gml::FromGml for OtherRelationTypeValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        let info = crate::gml_reader::ElementInfo {
            namespace: String::new(),
            local_name: String::new(),
            attributes: Vec::new(),
        };
        Self::from_gml_with_info(reader, &info)
    }
}
#[derive(Debug, Clone, Default)]
pub struct TemporalRelationTypeValue {}
impl TemporalRelationTypeValue {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(TemporalRelationTypeValue {})
    }
}
impl crate::from_gml::FromGml for TemporalRelationTypeValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        let info = crate::gml_reader::ElementInfo {
            namespace: String::new(),
            local_name: String::new(),
            attributes: Vec::new(),
        };
        Self::from_gml_with_info(reader, &info)
    }
}
#[derive(Debug, Clone, Default)]
pub struct TopologicalRelationTypeValue {}
impl TopologicalRelationTypeValue {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(TopologicalRelationTypeValue {})
    }
}
impl crate::from_gml::FromGml for TopologicalRelationTypeValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        let info = crate::gml_reader::ElementInfo {
            namespace: String::new(),
            local_name: String::new(),
            attributes: Vec::new(),
        };
        Self::from_gml_with_info(reader, &info)
    }
}
pub trait AbstractAppearance: AbstractFeatureWithLifespan {
    fn ade_of_abstract_appearance(&self) -> &[Box<dyn ADEOfAbstractAppearance>];
}
pub trait AbstractCityObject: AbstractFeatureWithLifespan {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain>;
    fn relative_to_water(&self) -> Option<RelativeToWater>;
    fn ade_of_abstract_city_object(&self) -> &[Box<dyn ADEOfAbstractCityObject>];
    fn appearance(&self) -> &[Box<dyn AbstractAppearance>];
    fn generic_attribute(&self) -> &[Box<dyn AbstractGenericAttribute>];
    fn generalizes_to(&self) -> &[Box<dyn AbstractCityObject>];
    fn external_reference(&self) -> &[ExternalReference];
    fn related_to(&self) -> &[Box<dyn AbstractCityObject>];
    fn dynamizer(&self) -> &[Box<dyn AbstractDynamizer>];
}
pub trait AbstractDynamizer: AbstractFeatureWithLifespan {
    fn ade_of_abstract_dynamizer(&self) -> &[Box<dyn ADEOfAbstractDynamizer>];
}
pub trait AbstractVersion: AbstractFeatureWithLifespan {
    fn ade_of_abstract_version(&self) -> &[Box<dyn ADEOfAbstractVersion>];
}
pub trait AbstractVersionTransition: AbstractFeatureWithLifespan {
    fn ade_of_abstract_version_transition(
        &self,
    ) -> &[Box<dyn ADEOfAbstractVersionTransition>];
}
#[derive(Debug, Default)]
pub struct CityModel {
    pub feature_id: ID,
    pub identifier: Option<String>,
    pub name: Vec<String>,
    pub description: Option<String>,
    pub ade_of_abstract_feature: Vec<Box<dyn ADEOfAbstractFeature>>,
    pub creation_date: Option<String>,
    pub termination_date: Option<String>,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    pub ade_of_abstract_feature_with_lifespan: Vec<
        Box<dyn ADEOfAbstractFeatureWithLifespan>,
    >,
    pub engineering_crs: Option<String>,
    pub ade_of_city_model: Vec<Box<dyn ADEOfCityModel>>,
    pub city_model_member: Vec<CityModelMember>,
}
impl AbstractFeature for CityModel {
    fn feature_id(&self) -> &ID {
        &self.feature_id
    }
    fn identifier(&self) -> Option<&String> {
        self.identifier.as_ref()
    }
    fn name(&self) -> &[String] {
        &self.name
    }
    fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    fn ade_of_abstract_feature(&self) -> &[Box<dyn ADEOfAbstractFeature>] {
        &self.ade_of_abstract_feature
    }
}
impl AbstractFeatureWithLifespan for CityModel {
    fn creation_date(&self) -> Option<&String> {
        self.creation_date.as_ref()
    }
    fn termination_date(&self) -> Option<&String> {
        self.termination_date.as_ref()
    }
    fn valid_from(&self) -> Option<&String> {
        self.valid_from.as_ref()
    }
    fn valid_to(&self) -> Option<&String> {
        self.valid_to.as_ref()
    }
    fn ade_of_abstract_feature_with_lifespan(
        &self,
    ) -> &[Box<dyn ADEOfAbstractFeatureWithLifespan>] {
        &self.ade_of_abstract_feature_with_lifespan
    }
}
impl CityModel {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut identifier = None;
        let mut name = Vec::new();
        let mut description = None;
        let mut ade_of_abstract_feature = Vec::new();
        let mut creation_date = None;
        let mut termination_date = None;
        let mut valid_from = None;
        let mut valid_to = None;
        let mut ade_of_abstract_feature_with_lifespan = Vec::new();
        let mut engineering_crs = None;
        let mut ade_of_city_model = Vec::new();
        let mut city_model_member = Vec::new();
        let mut feature_id = ID(_gml_id);
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "featureID") => {
                    feature_id = ID(sub.read_text()?);
                }
                (crate::namespace::NS_GML, "identifier") => {
                    identifier = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_GML, "name") => {
                    name.push(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_GML, "description") => {
                    description = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "adeOfAbstractFeature") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "creationDate") => {
                    creation_date = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "terminationDate") => {
                    termination_date = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_CORE, "validFrom") => {
                    valid_from = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "validTo") => {
                    valid_to = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "adeOfAbstractFeatureWithLifespan") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "engineeringCRS") => {
                    engineering_crs = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_CORE, "adeOfCityModel") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "cityModelMember") => {
                    city_model_member.push(CityModelMember::from_gml(&mut sub)?);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(CityModel {
            feature_id,
            identifier,
            name,
            description,
            ade_of_abstract_feature,
            creation_date,
            termination_date,
            valid_from,
            valid_to,
            ade_of_abstract_feature_with_lifespan,
            engineering_crs,
            ade_of_city_model,
            city_model_member,
        })
    }
}
impl crate::from_gml::FromGml for CityModel {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        let info = crate::gml_reader::ElementInfo {
            namespace: String::new(),
            local_name: String::new(),
            attributes: Vec::new(),
        };
        Self::from_gml_with_info(reader, &info)
    }
}
pub trait AbstractSpace: AbstractCityObject {
    fn space_type(&self) -> Option<SpaceType>;
    fn volume(&self) -> &[QualifiedVolume];
    fn area(&self) -> &[QualifiedArea];
    fn ade_of_abstract_space(&self) -> &[Box<dyn ADEOfAbstractSpace>];
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve>;
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface>;
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface>;
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid>;
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid>;
    fn boundary(&self) -> &[Box<dyn AbstractSpaceBoundary>];
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve>;
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid>;
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition>;
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve>;
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface>;
}
pub trait AbstractSpaceBoundary: AbstractCityObject {
    fn ade_of_abstract_space_boundary(&self) -> &[Box<dyn ADEOfAbstractSpaceBoundary>];
}
pub trait AbstractLogicalSpace: AbstractSpace {
    fn ade_of_abstract_logical_space(&self) -> &[Box<dyn ADEOfAbstractLogicalSpace>];
}
pub trait AbstractPhysicalSpace: AbstractSpace {
    fn ade_of_abstract_physical_space(&self) -> &[Box<dyn ADEOfAbstractPhysicalSpace>];
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve>;
    fn point_cloud(&self) -> Option<&Box<dyn AbstractPointCloud>>;
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve>;
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve>;
}
pub trait AbstractThematicSurface: AbstractSpaceBoundary {
    fn area(&self) -> &[QualifiedArea];
    fn ade_of_abstract_thematic_surface(
        &self,
    ) -> &[Box<dyn ADEOfAbstractThematicSurface>];
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface>;
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface>;
    fn point_cloud(&self) -> Option<&Box<dyn AbstractPointCloud>>;
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve>;
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface>;
    fn lod1_multi_surface(&self) -> Option<&crate::geometry::MultiSurface>;
}
pub trait AbstractOccupiedSpace: AbstractPhysicalSpace {
    fn ade_of_abstract_occupied_space(&self) -> &[Box<dyn ADEOfAbstractOccupiedSpace>];
    fn lod3_implicit_representation(&self) -> Option<&ImplicitGeometry>;
    fn lod2_implicit_representation(&self) -> Option<&ImplicitGeometry>;
    fn lod1_implicit_representation(&self) -> Option<&ImplicitGeometry>;
}
pub trait AbstractUnoccupiedSpace: AbstractPhysicalSpace {
    fn ade_of_abstract_unoccupied_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractUnoccupiedSpace>];
}
#[derive(Debug, Default)]
pub struct ClosureSurface {
    pub feature_id: ID,
    pub identifier: Option<String>,
    pub name: Vec<String>,
    pub description: Option<String>,
    pub ade_of_abstract_feature: Vec<Box<dyn ADEOfAbstractFeature>>,
    pub creation_date: Option<String>,
    pub termination_date: Option<String>,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    pub ade_of_abstract_feature_with_lifespan: Vec<
        Box<dyn ADEOfAbstractFeatureWithLifespan>,
    >,
    pub relative_to_terrain: Option<RelativeToTerrain>,
    pub relative_to_water: Option<RelativeToWater>,
    pub ade_of_abstract_city_object: Vec<Box<dyn ADEOfAbstractCityObject>>,
    pub appearance: Vec<Box<dyn AbstractAppearance>>,
    pub generic_attribute: Vec<Box<dyn AbstractGenericAttribute>>,
    pub generalizes_to: Vec<Box<dyn AbstractCityObject>>,
    pub external_reference: Vec<ExternalReference>,
    pub related_to: Vec<Box<dyn AbstractCityObject>>,
    pub dynamizer: Vec<Box<dyn AbstractDynamizer>>,
    pub ade_of_abstract_space_boundary: Vec<Box<dyn ADEOfAbstractSpaceBoundary>>,
    pub area: Vec<QualifiedArea>,
    pub ade_of_abstract_thematic_surface: Vec<Box<dyn ADEOfAbstractThematicSurface>>,
    pub lod3_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod2_multi_surface: Option<crate::geometry::MultiSurface>,
    pub point_cloud: Option<Box<dyn AbstractPointCloud>>,
    pub lod0_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod0_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod1_multi_surface: Option<crate::geometry::MultiSurface>,
    pub ade_of_closure_surface: Vec<Box<dyn ADEOfClosureSurface>>,
}
impl AbstractFeature for ClosureSurface {
    fn feature_id(&self) -> &ID {
        &self.feature_id
    }
    fn identifier(&self) -> Option<&String> {
        self.identifier.as_ref()
    }
    fn name(&self) -> &[String] {
        &self.name
    }
    fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    fn ade_of_abstract_feature(&self) -> &[Box<dyn ADEOfAbstractFeature>] {
        &self.ade_of_abstract_feature
    }
}
impl AbstractFeatureWithLifespan for ClosureSurface {
    fn creation_date(&self) -> Option<&String> {
        self.creation_date.as_ref()
    }
    fn termination_date(&self) -> Option<&String> {
        self.termination_date.as_ref()
    }
    fn valid_from(&self) -> Option<&String> {
        self.valid_from.as_ref()
    }
    fn valid_to(&self) -> Option<&String> {
        self.valid_to.as_ref()
    }
    fn ade_of_abstract_feature_with_lifespan(
        &self,
    ) -> &[Box<dyn ADEOfAbstractFeatureWithLifespan>] {
        &self.ade_of_abstract_feature_with_lifespan
    }
}
impl AbstractCityObject for ClosureSurface {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        self.relative_to_terrain
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        self.relative_to_water
    }
    fn ade_of_abstract_city_object(&self) -> &[Box<dyn ADEOfAbstractCityObject>] {
        &self.ade_of_abstract_city_object
    }
    fn appearance(&self) -> &[Box<dyn AbstractAppearance>] {
        &self.appearance
    }
    fn generic_attribute(&self) -> &[Box<dyn AbstractGenericAttribute>] {
        &self.generic_attribute
    }
    fn generalizes_to(&self) -> &[Box<dyn AbstractCityObject>] {
        &self.generalizes_to
    }
    fn external_reference(&self) -> &[ExternalReference] {
        &self.external_reference
    }
    fn related_to(&self) -> &[Box<dyn AbstractCityObject>] {
        &self.related_to
    }
    fn dynamizer(&self) -> &[Box<dyn AbstractDynamizer>] {
        &self.dynamizer
    }
}
impl AbstractSpaceBoundary for ClosureSurface {
    fn ade_of_abstract_space_boundary(&self) -> &[Box<dyn ADEOfAbstractSpaceBoundary>] {
        &self.ade_of_abstract_space_boundary
    }
}
impl AbstractThematicSurface for ClosureSurface {
    fn area(&self) -> &[QualifiedArea] {
        &self.area
    }
    fn ade_of_abstract_thematic_surface(
        &self,
    ) -> &[Box<dyn ADEOfAbstractThematicSurface>] {
        &self.ade_of_abstract_thematic_surface
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod3_multi_surface.as_ref()
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod2_multi_surface.as_ref()
    }
    fn point_cloud(&self) -> Option<&Box<dyn AbstractPointCloud>> {
        self.point_cloud.as_ref()
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod0_multi_curve.as_ref()
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod0_multi_surface.as_ref()
    }
    fn lod1_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod1_multi_surface.as_ref()
    }
}
impl ClosureSurface {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut identifier = None;
        let mut name = Vec::new();
        let mut description = None;
        let mut ade_of_abstract_feature = Vec::new();
        let mut creation_date = None;
        let mut termination_date = None;
        let mut valid_from = None;
        let mut valid_to = None;
        let mut ade_of_abstract_feature_with_lifespan = Vec::new();
        let mut relative_to_terrain = None;
        let mut relative_to_water = None;
        let mut ade_of_abstract_city_object = Vec::new();
        let mut appearance = Vec::new();
        let mut generic_attribute = Vec::new();
        let mut generalizes_to = Vec::new();
        let mut external_reference = Vec::new();
        let mut related_to = Vec::new();
        let mut dynamizer = Vec::new();
        let mut ade_of_abstract_space_boundary = Vec::new();
        let mut area = Vec::new();
        let mut ade_of_abstract_thematic_surface = Vec::new();
        let mut lod3_multi_surface = None;
        let mut lod2_multi_surface = None;
        let mut point_cloud = None;
        let mut lod0_multi_curve = None;
        let mut lod0_multi_surface = None;
        let mut lod1_multi_surface = None;
        let mut ade_of_closure_surface = Vec::new();
        let mut feature_id = ID(_gml_id);
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "featureID") => {
                    feature_id = ID(sub.read_text()?);
                }
                (crate::namespace::NS_GML, "identifier") => {
                    identifier = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_GML, "name") => {
                    name.push(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_GML, "description") => {
                    description = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "adeOfAbstractFeature") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "creationDate") => {
                    creation_date = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "terminationDate") => {
                    termination_date = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_CORE, "validFrom") => {
                    valid_from = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "validTo") => {
                    valid_to = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "adeOfAbstractFeatureWithLifespan") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "relativeToTerrain") => {
                    relative_to_terrain = Some(
                        RelativeToTerrain::from_gml_text(&sub.read_text()?)?,
                    );
                }
                (crate::namespace::NS_CORE, "relativeToWater") => {
                    relative_to_water = Some(
                        RelativeToWater::from_gml_text(&sub.read_text()?)?,
                    );
                }
                (crate::namespace::NS_CORE, "adeOfAbstractCityObject") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "appearance") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        appearance
                            .push(
                                super::dispatchers::parse_dyn_abstract_appearance(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_CORE, "genericAttribute") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "generalizesTo") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        generalizes_to
                            .push(
                                super::dispatchers::parse_dyn_abstract_city_object(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_CORE, "externalReference") => {
                    external_reference.push(ExternalReference::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "relatedTo") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        related_to
                            .push(
                                super::dispatchers::parse_dyn_abstract_city_object(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_CORE, "dynamizer") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        dynamizer
                            .push(
                                super::dispatchers::parse_dyn_abstract_dynamizer(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_CORE, "adeOfAbstractSpaceBoundary") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "area") => {
                    area.push(QualifiedArea::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "adeOfAbstractThematicSurface") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod3MultiSurface") => {
                    lod3_multi_surface = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod2MultiSurface") => {
                    lod2_multi_surface = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "pointCloud") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        point_cloud = Some(
                            super::dispatchers::parse_dyn_abstract_point_cloud(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    }
                }
                (crate::namespace::NS_CORE, "lod0MultiCurve") => {
                    lod0_multi_curve = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod0MultiSurface") => {
                    lod0_multi_surface = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod1MultiSurface") => {
                    lod1_multi_surface = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "adeOfClosureSurface") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(ClosureSurface {
            feature_id,
            identifier,
            name,
            description,
            ade_of_abstract_feature,
            creation_date,
            termination_date,
            valid_from,
            valid_to,
            ade_of_abstract_feature_with_lifespan,
            relative_to_terrain,
            relative_to_water,
            ade_of_abstract_city_object,
            appearance,
            generic_attribute,
            generalizes_to,
            external_reference,
            related_to,
            dynamizer,
            ade_of_abstract_space_boundary,
            area,
            ade_of_abstract_thematic_surface,
            lod3_multi_surface,
            lod2_multi_surface,
            point_cloud,
            lod0_multi_curve,
            lod0_multi_surface,
            lod1_multi_surface,
            ade_of_closure_surface,
        })
    }
}
impl crate::from_gml::FromGml for ClosureSurface {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        let info = crate::gml_reader::ElementInfo {
            namespace: String::new(),
            local_name: String::new(),
            attributes: Vec::new(),
        };
        Self::from_gml_with_info(reader, &info)
    }
}
