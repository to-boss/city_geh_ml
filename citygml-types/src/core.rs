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
#[derive(Debug, Clone, Default)]
pub struct CityModelMember {
    pub city_object_member: Option<AbstractCityObject>,
    pub appearance_member: Option<AbstractAppearance>,
    pub version_member: Option<AbstractVersion>,
    pub version_transition_member: Option<AbstractVersionTransition>,
    pub feature_member: Option<AbstractFeature>,
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
                            super::dispatchers::parse_abstract_city_object(
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
                            super::dispatchers::parse_abstract_appearance(
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
                            super::dispatchers::parse_abstract_version(
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
                            super::dispatchers::parse_abstract_version_transition(
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
                            super::dispatchers::parse_abstract_feature(
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
pub trait AbstractFeatureTrait: std::fmt::Debug {
    fn feature_id(&self) -> &ID;
    fn identifier(&self) -> Option<&String>;
    fn name(&self) -> &[String];
    fn description(&self) -> Option<&String>;
}
#[derive(Debug, Clone)]
pub enum AbstractFeature {
    CeilingSurface(Box<CeilingSurface>),
    Door(Box<Door>),
    DoorSurface(Box<DoorSurface>),
    FloorSurface(Box<FloorSurface>),
    GroundSurface(Box<GroundSurface>),
    InteriorWallSurface(Box<InteriorWallSurface>),
    OtherConstruction(Box<OtherConstruction>),
    OuterCeilingSurface(Box<OuterCeilingSurface>),
    OuterFloorSurface(Box<OuterFloorSurface>),
    RoofSurface(Box<RoofSurface>),
    WallSurface(Box<WallSurface>),
    Window(Box<Window>),
    WindowSurface(Box<WindowSurface>),
    CompositeTimeseries(Box<CompositeTimeseries>),
    Dynamizer(Box<Dynamizer>),
    GenericTimeseries(Box<GenericTimeseries>),
    StandardFileTimeseries(Box<StandardFileTimeseries>),
    TabulatedFileTimeseries(Box<TabulatedFileTimeseries>),
    PointCloud(Box<PointCloud>),
    Version(Box<Version>),
    VersionTransition(Box<VersionTransition>),
    Appearance(Box<Appearance>),
    GeoreferencedTexture(Box<GeoreferencedTexture>),
    ParameterizedTexture(Box<ParameterizedTexture>),
    X3DMaterial(Box<X3DMaterial>),
    Bridge(Box<Bridge>),
    BridgeConstructiveElement(Box<BridgeConstructiveElement>),
    BridgeFurniture(Box<BridgeFurniture>),
    BridgeInstallation(Box<BridgeInstallation>),
    BridgePart(Box<BridgePart>),
    BridgeRoom(Box<BridgeRoom>),
    Building(Box<Building>),
    BuildingConstructiveElement(Box<BuildingConstructiveElement>),
    BuildingFurniture(Box<BuildingFurniture>),
    BuildingInstallation(Box<BuildingInstallation>),
    BuildingPart(Box<BuildingPart>),
    BuildingRoom(Box<BuildingRoom>),
    BuildingUnit(Box<BuildingUnit>),
    Storey(Box<Storey>),
    CityFurniture(Box<CityFurniture>),
    CityObjectGroup(Box<CityObjectGroup>),
    Address(Box<Address>),
    CityModel(Box<CityModel>),
    ClosureSurface(Box<ClosureSurface>),
    GenericLogicalSpace(Box<GenericLogicalSpace>),
    GenericOccupiedSpace(Box<GenericOccupiedSpace>),
    GenericThematicSurface(Box<GenericThematicSurface>),
    GenericUnoccupiedSpace(Box<GenericUnoccupiedSpace>),
    LandUse(Box<LandUse>),
    BreaklineRelief(Box<BreaklineRelief>),
    MassPointRelief(Box<MassPointRelief>),
    RasterRelief(Box<RasterRelief>),
    ReliefFeature(Box<ReliefFeature>),
    TINRelief(Box<TINRelief>),
    AuxiliaryTrafficArea(Box<AuxiliaryTrafficArea>),
    AuxiliaryTrafficSpace(Box<AuxiliaryTrafficSpace>),
    ClearanceSpace(Box<ClearanceSpace>),
    Hole(Box<Hole>),
    HoleSurface(Box<HoleSurface>),
    Intersection(Box<Intersection>),
    Marking(Box<Marking>),
    Railway(Box<Railway>),
    Road(Box<Road>),
    Section(Box<Section>),
    Square(Box<Square>),
    Track(Box<Track>),
    TrafficArea(Box<TrafficArea>),
    TrafficSpace(Box<TrafficSpace>),
    Waterway(Box<Waterway>),
    HollowSpace(Box<HollowSpace>),
    Tunnel(Box<Tunnel>),
    TunnelConstructiveElement(Box<TunnelConstructiveElement>),
    TunnelFurniture(Box<TunnelFurniture>),
    TunnelInstallation(Box<TunnelInstallation>),
    TunnelPart(Box<TunnelPart>),
    PlantCover(Box<PlantCover>),
    SolitaryVegetationObject(Box<SolitaryVegetationObject>),
    WaterBody(Box<WaterBody>),
    WaterGroundSurface(Box<WaterGroundSurface>),
    WaterSurface(Box<WaterSurface>),
}
impl Default for AbstractFeature {
    fn default() -> Self {
        Self::CeilingSurface(Box::default())
    }
}
impl AbstractFeatureTrait for AbstractFeature {
    fn feature_id(&self) -> &ID {
        match self {
            Self::CeilingSurface(v) => v.feature_id(),
            Self::Door(v) => v.feature_id(),
            Self::DoorSurface(v) => v.feature_id(),
            Self::FloorSurface(v) => v.feature_id(),
            Self::GroundSurface(v) => v.feature_id(),
            Self::InteriorWallSurface(v) => v.feature_id(),
            Self::OtherConstruction(v) => v.feature_id(),
            Self::OuterCeilingSurface(v) => v.feature_id(),
            Self::OuterFloorSurface(v) => v.feature_id(),
            Self::RoofSurface(v) => v.feature_id(),
            Self::WallSurface(v) => v.feature_id(),
            Self::Window(v) => v.feature_id(),
            Self::WindowSurface(v) => v.feature_id(),
            Self::CompositeTimeseries(v) => v.feature_id(),
            Self::Dynamizer(v) => v.feature_id(),
            Self::GenericTimeseries(v) => v.feature_id(),
            Self::StandardFileTimeseries(v) => v.feature_id(),
            Self::TabulatedFileTimeseries(v) => v.feature_id(),
            Self::PointCloud(v) => v.feature_id(),
            Self::Version(v) => v.feature_id(),
            Self::VersionTransition(v) => v.feature_id(),
            Self::Appearance(v) => v.feature_id(),
            Self::GeoreferencedTexture(v) => v.feature_id(),
            Self::ParameterizedTexture(v) => v.feature_id(),
            Self::X3DMaterial(v) => v.feature_id(),
            Self::Bridge(v) => v.feature_id(),
            Self::BridgeConstructiveElement(v) => v.feature_id(),
            Self::BridgeFurniture(v) => v.feature_id(),
            Self::BridgeInstallation(v) => v.feature_id(),
            Self::BridgePart(v) => v.feature_id(),
            Self::BridgeRoom(v) => v.feature_id(),
            Self::Building(v) => v.feature_id(),
            Self::BuildingConstructiveElement(v) => v.feature_id(),
            Self::BuildingFurniture(v) => v.feature_id(),
            Self::BuildingInstallation(v) => v.feature_id(),
            Self::BuildingPart(v) => v.feature_id(),
            Self::BuildingRoom(v) => v.feature_id(),
            Self::BuildingUnit(v) => v.feature_id(),
            Self::Storey(v) => v.feature_id(),
            Self::CityFurniture(v) => v.feature_id(),
            Self::CityObjectGroup(v) => v.feature_id(),
            Self::Address(v) => v.feature_id(),
            Self::CityModel(v) => v.feature_id(),
            Self::ClosureSurface(v) => v.feature_id(),
            Self::GenericLogicalSpace(v) => v.feature_id(),
            Self::GenericOccupiedSpace(v) => v.feature_id(),
            Self::GenericThematicSurface(v) => v.feature_id(),
            Self::GenericUnoccupiedSpace(v) => v.feature_id(),
            Self::LandUse(v) => v.feature_id(),
            Self::BreaklineRelief(v) => v.feature_id(),
            Self::MassPointRelief(v) => v.feature_id(),
            Self::RasterRelief(v) => v.feature_id(),
            Self::ReliefFeature(v) => v.feature_id(),
            Self::TINRelief(v) => v.feature_id(),
            Self::AuxiliaryTrafficArea(v) => v.feature_id(),
            Self::AuxiliaryTrafficSpace(v) => v.feature_id(),
            Self::ClearanceSpace(v) => v.feature_id(),
            Self::Hole(v) => v.feature_id(),
            Self::HoleSurface(v) => v.feature_id(),
            Self::Intersection(v) => v.feature_id(),
            Self::Marking(v) => v.feature_id(),
            Self::Railway(v) => v.feature_id(),
            Self::Road(v) => v.feature_id(),
            Self::Section(v) => v.feature_id(),
            Self::Square(v) => v.feature_id(),
            Self::Track(v) => v.feature_id(),
            Self::TrafficArea(v) => v.feature_id(),
            Self::TrafficSpace(v) => v.feature_id(),
            Self::Waterway(v) => v.feature_id(),
            Self::HollowSpace(v) => v.feature_id(),
            Self::Tunnel(v) => v.feature_id(),
            Self::TunnelConstructiveElement(v) => v.feature_id(),
            Self::TunnelFurniture(v) => v.feature_id(),
            Self::TunnelInstallation(v) => v.feature_id(),
            Self::TunnelPart(v) => v.feature_id(),
            Self::PlantCover(v) => v.feature_id(),
            Self::SolitaryVegetationObject(v) => v.feature_id(),
            Self::WaterBody(v) => v.feature_id(),
            Self::WaterGroundSurface(v) => v.feature_id(),
            Self::WaterSurface(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.identifier(),
            Self::Door(v) => v.identifier(),
            Self::DoorSurface(v) => v.identifier(),
            Self::FloorSurface(v) => v.identifier(),
            Self::GroundSurface(v) => v.identifier(),
            Self::InteriorWallSurface(v) => v.identifier(),
            Self::OtherConstruction(v) => v.identifier(),
            Self::OuterCeilingSurface(v) => v.identifier(),
            Self::OuterFloorSurface(v) => v.identifier(),
            Self::RoofSurface(v) => v.identifier(),
            Self::WallSurface(v) => v.identifier(),
            Self::Window(v) => v.identifier(),
            Self::WindowSurface(v) => v.identifier(),
            Self::CompositeTimeseries(v) => v.identifier(),
            Self::Dynamizer(v) => v.identifier(),
            Self::GenericTimeseries(v) => v.identifier(),
            Self::StandardFileTimeseries(v) => v.identifier(),
            Self::TabulatedFileTimeseries(v) => v.identifier(),
            Self::PointCloud(v) => v.identifier(),
            Self::Version(v) => v.identifier(),
            Self::VersionTransition(v) => v.identifier(),
            Self::Appearance(v) => v.identifier(),
            Self::GeoreferencedTexture(v) => v.identifier(),
            Self::ParameterizedTexture(v) => v.identifier(),
            Self::X3DMaterial(v) => v.identifier(),
            Self::Bridge(v) => v.identifier(),
            Self::BridgeConstructiveElement(v) => v.identifier(),
            Self::BridgeFurniture(v) => v.identifier(),
            Self::BridgeInstallation(v) => v.identifier(),
            Self::BridgePart(v) => v.identifier(),
            Self::BridgeRoom(v) => v.identifier(),
            Self::Building(v) => v.identifier(),
            Self::BuildingConstructiveElement(v) => v.identifier(),
            Self::BuildingFurniture(v) => v.identifier(),
            Self::BuildingInstallation(v) => v.identifier(),
            Self::BuildingPart(v) => v.identifier(),
            Self::BuildingRoom(v) => v.identifier(),
            Self::BuildingUnit(v) => v.identifier(),
            Self::Storey(v) => v.identifier(),
            Self::CityFurniture(v) => v.identifier(),
            Self::CityObjectGroup(v) => v.identifier(),
            Self::Address(v) => v.identifier(),
            Self::CityModel(v) => v.identifier(),
            Self::ClosureSurface(v) => v.identifier(),
            Self::GenericLogicalSpace(v) => v.identifier(),
            Self::GenericOccupiedSpace(v) => v.identifier(),
            Self::GenericThematicSurface(v) => v.identifier(),
            Self::GenericUnoccupiedSpace(v) => v.identifier(),
            Self::LandUse(v) => v.identifier(),
            Self::BreaklineRelief(v) => v.identifier(),
            Self::MassPointRelief(v) => v.identifier(),
            Self::RasterRelief(v) => v.identifier(),
            Self::ReliefFeature(v) => v.identifier(),
            Self::TINRelief(v) => v.identifier(),
            Self::AuxiliaryTrafficArea(v) => v.identifier(),
            Self::AuxiliaryTrafficSpace(v) => v.identifier(),
            Self::ClearanceSpace(v) => v.identifier(),
            Self::Hole(v) => v.identifier(),
            Self::HoleSurface(v) => v.identifier(),
            Self::Intersection(v) => v.identifier(),
            Self::Marking(v) => v.identifier(),
            Self::Railway(v) => v.identifier(),
            Self::Road(v) => v.identifier(),
            Self::Section(v) => v.identifier(),
            Self::Square(v) => v.identifier(),
            Self::Track(v) => v.identifier(),
            Self::TrafficArea(v) => v.identifier(),
            Self::TrafficSpace(v) => v.identifier(),
            Self::Waterway(v) => v.identifier(),
            Self::HollowSpace(v) => v.identifier(),
            Self::Tunnel(v) => v.identifier(),
            Self::TunnelConstructiveElement(v) => v.identifier(),
            Self::TunnelFurniture(v) => v.identifier(),
            Self::TunnelInstallation(v) => v.identifier(),
            Self::TunnelPart(v) => v.identifier(),
            Self::PlantCover(v) => v.identifier(),
            Self::SolitaryVegetationObject(v) => v.identifier(),
            Self::WaterBody(v) => v.identifier(),
            Self::WaterGroundSurface(v) => v.identifier(),
            Self::WaterSurface(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::CeilingSurface(v) => v.name(),
            Self::Door(v) => v.name(),
            Self::DoorSurface(v) => v.name(),
            Self::FloorSurface(v) => v.name(),
            Self::GroundSurface(v) => v.name(),
            Self::InteriorWallSurface(v) => v.name(),
            Self::OtherConstruction(v) => v.name(),
            Self::OuterCeilingSurface(v) => v.name(),
            Self::OuterFloorSurface(v) => v.name(),
            Self::RoofSurface(v) => v.name(),
            Self::WallSurface(v) => v.name(),
            Self::Window(v) => v.name(),
            Self::WindowSurface(v) => v.name(),
            Self::CompositeTimeseries(v) => v.name(),
            Self::Dynamizer(v) => v.name(),
            Self::GenericTimeseries(v) => v.name(),
            Self::StandardFileTimeseries(v) => v.name(),
            Self::TabulatedFileTimeseries(v) => v.name(),
            Self::PointCloud(v) => v.name(),
            Self::Version(v) => v.name(),
            Self::VersionTransition(v) => v.name(),
            Self::Appearance(v) => v.name(),
            Self::GeoreferencedTexture(v) => v.name(),
            Self::ParameterizedTexture(v) => v.name(),
            Self::X3DMaterial(v) => v.name(),
            Self::Bridge(v) => v.name(),
            Self::BridgeConstructiveElement(v) => v.name(),
            Self::BridgeFurniture(v) => v.name(),
            Self::BridgeInstallation(v) => v.name(),
            Self::BridgePart(v) => v.name(),
            Self::BridgeRoom(v) => v.name(),
            Self::Building(v) => v.name(),
            Self::BuildingConstructiveElement(v) => v.name(),
            Self::BuildingFurniture(v) => v.name(),
            Self::BuildingInstallation(v) => v.name(),
            Self::BuildingPart(v) => v.name(),
            Self::BuildingRoom(v) => v.name(),
            Self::BuildingUnit(v) => v.name(),
            Self::Storey(v) => v.name(),
            Self::CityFurniture(v) => v.name(),
            Self::CityObjectGroup(v) => v.name(),
            Self::Address(v) => v.name(),
            Self::CityModel(v) => v.name(),
            Self::ClosureSurface(v) => v.name(),
            Self::GenericLogicalSpace(v) => v.name(),
            Self::GenericOccupiedSpace(v) => v.name(),
            Self::GenericThematicSurface(v) => v.name(),
            Self::GenericUnoccupiedSpace(v) => v.name(),
            Self::LandUse(v) => v.name(),
            Self::BreaklineRelief(v) => v.name(),
            Self::MassPointRelief(v) => v.name(),
            Self::RasterRelief(v) => v.name(),
            Self::ReliefFeature(v) => v.name(),
            Self::TINRelief(v) => v.name(),
            Self::AuxiliaryTrafficArea(v) => v.name(),
            Self::AuxiliaryTrafficSpace(v) => v.name(),
            Self::ClearanceSpace(v) => v.name(),
            Self::Hole(v) => v.name(),
            Self::HoleSurface(v) => v.name(),
            Self::Intersection(v) => v.name(),
            Self::Marking(v) => v.name(),
            Self::Railway(v) => v.name(),
            Self::Road(v) => v.name(),
            Self::Section(v) => v.name(),
            Self::Square(v) => v.name(),
            Self::Track(v) => v.name(),
            Self::TrafficArea(v) => v.name(),
            Self::TrafficSpace(v) => v.name(),
            Self::Waterway(v) => v.name(),
            Self::HollowSpace(v) => v.name(),
            Self::Tunnel(v) => v.name(),
            Self::TunnelConstructiveElement(v) => v.name(),
            Self::TunnelFurniture(v) => v.name(),
            Self::TunnelInstallation(v) => v.name(),
            Self::TunnelPart(v) => v.name(),
            Self::PlantCover(v) => v.name(),
            Self::SolitaryVegetationObject(v) => v.name(),
            Self::WaterBody(v) => v.name(),
            Self::WaterGroundSurface(v) => v.name(),
            Self::WaterSurface(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.description(),
            Self::Door(v) => v.description(),
            Self::DoorSurface(v) => v.description(),
            Self::FloorSurface(v) => v.description(),
            Self::GroundSurface(v) => v.description(),
            Self::InteriorWallSurface(v) => v.description(),
            Self::OtherConstruction(v) => v.description(),
            Self::OuterCeilingSurface(v) => v.description(),
            Self::OuterFloorSurface(v) => v.description(),
            Self::RoofSurface(v) => v.description(),
            Self::WallSurface(v) => v.description(),
            Self::Window(v) => v.description(),
            Self::WindowSurface(v) => v.description(),
            Self::CompositeTimeseries(v) => v.description(),
            Self::Dynamizer(v) => v.description(),
            Self::GenericTimeseries(v) => v.description(),
            Self::StandardFileTimeseries(v) => v.description(),
            Self::TabulatedFileTimeseries(v) => v.description(),
            Self::PointCloud(v) => v.description(),
            Self::Version(v) => v.description(),
            Self::VersionTransition(v) => v.description(),
            Self::Appearance(v) => v.description(),
            Self::GeoreferencedTexture(v) => v.description(),
            Self::ParameterizedTexture(v) => v.description(),
            Self::X3DMaterial(v) => v.description(),
            Self::Bridge(v) => v.description(),
            Self::BridgeConstructiveElement(v) => v.description(),
            Self::BridgeFurniture(v) => v.description(),
            Self::BridgeInstallation(v) => v.description(),
            Self::BridgePart(v) => v.description(),
            Self::BridgeRoom(v) => v.description(),
            Self::Building(v) => v.description(),
            Self::BuildingConstructiveElement(v) => v.description(),
            Self::BuildingFurniture(v) => v.description(),
            Self::BuildingInstallation(v) => v.description(),
            Self::BuildingPart(v) => v.description(),
            Self::BuildingRoom(v) => v.description(),
            Self::BuildingUnit(v) => v.description(),
            Self::Storey(v) => v.description(),
            Self::CityFurniture(v) => v.description(),
            Self::CityObjectGroup(v) => v.description(),
            Self::Address(v) => v.description(),
            Self::CityModel(v) => v.description(),
            Self::ClosureSurface(v) => v.description(),
            Self::GenericLogicalSpace(v) => v.description(),
            Self::GenericOccupiedSpace(v) => v.description(),
            Self::GenericThematicSurface(v) => v.description(),
            Self::GenericUnoccupiedSpace(v) => v.description(),
            Self::LandUse(v) => v.description(),
            Self::BreaklineRelief(v) => v.description(),
            Self::MassPointRelief(v) => v.description(),
            Self::RasterRelief(v) => v.description(),
            Self::ReliefFeature(v) => v.description(),
            Self::TINRelief(v) => v.description(),
            Self::AuxiliaryTrafficArea(v) => v.description(),
            Self::AuxiliaryTrafficSpace(v) => v.description(),
            Self::ClearanceSpace(v) => v.description(),
            Self::Hole(v) => v.description(),
            Self::HoleSurface(v) => v.description(),
            Self::Intersection(v) => v.description(),
            Self::Marking(v) => v.description(),
            Self::Railway(v) => v.description(),
            Self::Road(v) => v.description(),
            Self::Section(v) => v.description(),
            Self::Square(v) => v.description(),
            Self::Track(v) => v.description(),
            Self::TrafficArea(v) => v.description(),
            Self::TrafficSpace(v) => v.description(),
            Self::Waterway(v) => v.description(),
            Self::HollowSpace(v) => v.description(),
            Self::Tunnel(v) => v.description(),
            Self::TunnelConstructiveElement(v) => v.description(),
            Self::TunnelFurniture(v) => v.description(),
            Self::TunnelInstallation(v) => v.description(),
            Self::TunnelPart(v) => v.description(),
            Self::PlantCover(v) => v.description(),
            Self::SolitaryVegetationObject(v) => v.description(),
            Self::WaterBody(v) => v.description(),
            Self::WaterGroundSurface(v) => v.description(),
            Self::WaterSurface(v) => v.description(),
        }
    }
}
impl From<CeilingSurface> for AbstractFeature {
    fn from(v: CeilingSurface) -> Self {
        Self::CeilingSurface(Box::new(v))
    }
}
impl From<Door> for AbstractFeature {
    fn from(v: Door) -> Self {
        Self::Door(Box::new(v))
    }
}
impl From<DoorSurface> for AbstractFeature {
    fn from(v: DoorSurface) -> Self {
        Self::DoorSurface(Box::new(v))
    }
}
impl From<FloorSurface> for AbstractFeature {
    fn from(v: FloorSurface) -> Self {
        Self::FloorSurface(Box::new(v))
    }
}
impl From<GroundSurface> for AbstractFeature {
    fn from(v: GroundSurface) -> Self {
        Self::GroundSurface(Box::new(v))
    }
}
impl From<InteriorWallSurface> for AbstractFeature {
    fn from(v: InteriorWallSurface) -> Self {
        Self::InteriorWallSurface(Box::new(v))
    }
}
impl From<OtherConstruction> for AbstractFeature {
    fn from(v: OtherConstruction) -> Self {
        Self::OtherConstruction(Box::new(v))
    }
}
impl From<OuterCeilingSurface> for AbstractFeature {
    fn from(v: OuterCeilingSurface) -> Self {
        Self::OuterCeilingSurface(Box::new(v))
    }
}
impl From<OuterFloorSurface> for AbstractFeature {
    fn from(v: OuterFloorSurface) -> Self {
        Self::OuterFloorSurface(Box::new(v))
    }
}
impl From<RoofSurface> for AbstractFeature {
    fn from(v: RoofSurface) -> Self {
        Self::RoofSurface(Box::new(v))
    }
}
impl From<WallSurface> for AbstractFeature {
    fn from(v: WallSurface) -> Self {
        Self::WallSurface(Box::new(v))
    }
}
impl From<Window> for AbstractFeature {
    fn from(v: Window) -> Self {
        Self::Window(Box::new(v))
    }
}
impl From<WindowSurface> for AbstractFeature {
    fn from(v: WindowSurface) -> Self {
        Self::WindowSurface(Box::new(v))
    }
}
impl From<CompositeTimeseries> for AbstractFeature {
    fn from(v: CompositeTimeseries) -> Self {
        Self::CompositeTimeseries(Box::new(v))
    }
}
impl From<Dynamizer> for AbstractFeature {
    fn from(v: Dynamizer) -> Self {
        Self::Dynamizer(Box::new(v))
    }
}
impl From<GenericTimeseries> for AbstractFeature {
    fn from(v: GenericTimeseries) -> Self {
        Self::GenericTimeseries(Box::new(v))
    }
}
impl From<StandardFileTimeseries> for AbstractFeature {
    fn from(v: StandardFileTimeseries) -> Self {
        Self::StandardFileTimeseries(Box::new(v))
    }
}
impl From<TabulatedFileTimeseries> for AbstractFeature {
    fn from(v: TabulatedFileTimeseries) -> Self {
        Self::TabulatedFileTimeseries(Box::new(v))
    }
}
impl From<PointCloud> for AbstractFeature {
    fn from(v: PointCloud) -> Self {
        Self::PointCloud(Box::new(v))
    }
}
impl From<Version> for AbstractFeature {
    fn from(v: Version) -> Self {
        Self::Version(Box::new(v))
    }
}
impl From<VersionTransition> for AbstractFeature {
    fn from(v: VersionTransition) -> Self {
        Self::VersionTransition(Box::new(v))
    }
}
impl From<Appearance> for AbstractFeature {
    fn from(v: Appearance) -> Self {
        Self::Appearance(Box::new(v))
    }
}
impl From<GeoreferencedTexture> for AbstractFeature {
    fn from(v: GeoreferencedTexture) -> Self {
        Self::GeoreferencedTexture(Box::new(v))
    }
}
impl From<ParameterizedTexture> for AbstractFeature {
    fn from(v: ParameterizedTexture) -> Self {
        Self::ParameterizedTexture(Box::new(v))
    }
}
impl From<X3DMaterial> for AbstractFeature {
    fn from(v: X3DMaterial) -> Self {
        Self::X3DMaterial(Box::new(v))
    }
}
impl From<Bridge> for AbstractFeature {
    fn from(v: Bridge) -> Self {
        Self::Bridge(Box::new(v))
    }
}
impl From<BridgeConstructiveElement> for AbstractFeature {
    fn from(v: BridgeConstructiveElement) -> Self {
        Self::BridgeConstructiveElement(Box::new(v))
    }
}
impl From<BridgeFurniture> for AbstractFeature {
    fn from(v: BridgeFurniture) -> Self {
        Self::BridgeFurniture(Box::new(v))
    }
}
impl From<BridgeInstallation> for AbstractFeature {
    fn from(v: BridgeInstallation) -> Self {
        Self::BridgeInstallation(Box::new(v))
    }
}
impl From<BridgePart> for AbstractFeature {
    fn from(v: BridgePart) -> Self {
        Self::BridgePart(Box::new(v))
    }
}
impl From<BridgeRoom> for AbstractFeature {
    fn from(v: BridgeRoom) -> Self {
        Self::BridgeRoom(Box::new(v))
    }
}
impl From<Building> for AbstractFeature {
    fn from(v: Building) -> Self {
        Self::Building(Box::new(v))
    }
}
impl From<BuildingConstructiveElement> for AbstractFeature {
    fn from(v: BuildingConstructiveElement) -> Self {
        Self::BuildingConstructiveElement(Box::new(v))
    }
}
impl From<BuildingFurniture> for AbstractFeature {
    fn from(v: BuildingFurniture) -> Self {
        Self::BuildingFurniture(Box::new(v))
    }
}
impl From<BuildingInstallation> for AbstractFeature {
    fn from(v: BuildingInstallation) -> Self {
        Self::BuildingInstallation(Box::new(v))
    }
}
impl From<BuildingPart> for AbstractFeature {
    fn from(v: BuildingPart) -> Self {
        Self::BuildingPart(Box::new(v))
    }
}
impl From<BuildingRoom> for AbstractFeature {
    fn from(v: BuildingRoom) -> Self {
        Self::BuildingRoom(Box::new(v))
    }
}
impl From<BuildingUnit> for AbstractFeature {
    fn from(v: BuildingUnit) -> Self {
        Self::BuildingUnit(Box::new(v))
    }
}
impl From<Storey> for AbstractFeature {
    fn from(v: Storey) -> Self {
        Self::Storey(Box::new(v))
    }
}
impl From<CityFurniture> for AbstractFeature {
    fn from(v: CityFurniture) -> Self {
        Self::CityFurniture(Box::new(v))
    }
}
impl From<CityObjectGroup> for AbstractFeature {
    fn from(v: CityObjectGroup) -> Self {
        Self::CityObjectGroup(Box::new(v))
    }
}
impl From<Address> for AbstractFeature {
    fn from(v: Address) -> Self {
        Self::Address(Box::new(v))
    }
}
impl From<CityModel> for AbstractFeature {
    fn from(v: CityModel) -> Self {
        Self::CityModel(Box::new(v))
    }
}
impl From<ClosureSurface> for AbstractFeature {
    fn from(v: ClosureSurface) -> Self {
        Self::ClosureSurface(Box::new(v))
    }
}
impl From<GenericLogicalSpace> for AbstractFeature {
    fn from(v: GenericLogicalSpace) -> Self {
        Self::GenericLogicalSpace(Box::new(v))
    }
}
impl From<GenericOccupiedSpace> for AbstractFeature {
    fn from(v: GenericOccupiedSpace) -> Self {
        Self::GenericOccupiedSpace(Box::new(v))
    }
}
impl From<GenericThematicSurface> for AbstractFeature {
    fn from(v: GenericThematicSurface) -> Self {
        Self::GenericThematicSurface(Box::new(v))
    }
}
impl From<GenericUnoccupiedSpace> for AbstractFeature {
    fn from(v: GenericUnoccupiedSpace) -> Self {
        Self::GenericUnoccupiedSpace(Box::new(v))
    }
}
impl From<LandUse> for AbstractFeature {
    fn from(v: LandUse) -> Self {
        Self::LandUse(Box::new(v))
    }
}
impl From<BreaklineRelief> for AbstractFeature {
    fn from(v: BreaklineRelief) -> Self {
        Self::BreaklineRelief(Box::new(v))
    }
}
impl From<MassPointRelief> for AbstractFeature {
    fn from(v: MassPointRelief) -> Self {
        Self::MassPointRelief(Box::new(v))
    }
}
impl From<RasterRelief> for AbstractFeature {
    fn from(v: RasterRelief) -> Self {
        Self::RasterRelief(Box::new(v))
    }
}
impl From<ReliefFeature> for AbstractFeature {
    fn from(v: ReliefFeature) -> Self {
        Self::ReliefFeature(Box::new(v))
    }
}
impl From<TINRelief> for AbstractFeature {
    fn from(v: TINRelief) -> Self {
        Self::TINRelief(Box::new(v))
    }
}
impl From<AuxiliaryTrafficArea> for AbstractFeature {
    fn from(v: AuxiliaryTrafficArea) -> Self {
        Self::AuxiliaryTrafficArea(Box::new(v))
    }
}
impl From<AuxiliaryTrafficSpace> for AbstractFeature {
    fn from(v: AuxiliaryTrafficSpace) -> Self {
        Self::AuxiliaryTrafficSpace(Box::new(v))
    }
}
impl From<ClearanceSpace> for AbstractFeature {
    fn from(v: ClearanceSpace) -> Self {
        Self::ClearanceSpace(Box::new(v))
    }
}
impl From<Hole> for AbstractFeature {
    fn from(v: Hole) -> Self {
        Self::Hole(Box::new(v))
    }
}
impl From<HoleSurface> for AbstractFeature {
    fn from(v: HoleSurface) -> Self {
        Self::HoleSurface(Box::new(v))
    }
}
impl From<Intersection> for AbstractFeature {
    fn from(v: Intersection) -> Self {
        Self::Intersection(Box::new(v))
    }
}
impl From<Marking> for AbstractFeature {
    fn from(v: Marking) -> Self {
        Self::Marking(Box::new(v))
    }
}
impl From<Railway> for AbstractFeature {
    fn from(v: Railway) -> Self {
        Self::Railway(Box::new(v))
    }
}
impl From<Road> for AbstractFeature {
    fn from(v: Road) -> Self {
        Self::Road(Box::new(v))
    }
}
impl From<Section> for AbstractFeature {
    fn from(v: Section) -> Self {
        Self::Section(Box::new(v))
    }
}
impl From<Square> for AbstractFeature {
    fn from(v: Square) -> Self {
        Self::Square(Box::new(v))
    }
}
impl From<Track> for AbstractFeature {
    fn from(v: Track) -> Self {
        Self::Track(Box::new(v))
    }
}
impl From<TrafficArea> for AbstractFeature {
    fn from(v: TrafficArea) -> Self {
        Self::TrafficArea(Box::new(v))
    }
}
impl From<TrafficSpace> for AbstractFeature {
    fn from(v: TrafficSpace) -> Self {
        Self::TrafficSpace(Box::new(v))
    }
}
impl From<Waterway> for AbstractFeature {
    fn from(v: Waterway) -> Self {
        Self::Waterway(Box::new(v))
    }
}
impl From<HollowSpace> for AbstractFeature {
    fn from(v: HollowSpace) -> Self {
        Self::HollowSpace(Box::new(v))
    }
}
impl From<Tunnel> for AbstractFeature {
    fn from(v: Tunnel) -> Self {
        Self::Tunnel(Box::new(v))
    }
}
impl From<TunnelConstructiveElement> for AbstractFeature {
    fn from(v: TunnelConstructiveElement) -> Self {
        Self::TunnelConstructiveElement(Box::new(v))
    }
}
impl From<TunnelFurniture> for AbstractFeature {
    fn from(v: TunnelFurniture) -> Self {
        Self::TunnelFurniture(Box::new(v))
    }
}
impl From<TunnelInstallation> for AbstractFeature {
    fn from(v: TunnelInstallation) -> Self {
        Self::TunnelInstallation(Box::new(v))
    }
}
impl From<TunnelPart> for AbstractFeature {
    fn from(v: TunnelPart) -> Self {
        Self::TunnelPart(Box::new(v))
    }
}
impl From<PlantCover> for AbstractFeature {
    fn from(v: PlantCover) -> Self {
        Self::PlantCover(Box::new(v))
    }
}
impl From<SolitaryVegetationObject> for AbstractFeature {
    fn from(v: SolitaryVegetationObject) -> Self {
        Self::SolitaryVegetationObject(Box::new(v))
    }
}
impl From<WaterBody> for AbstractFeature {
    fn from(v: WaterBody) -> Self {
        Self::WaterBody(Box::new(v))
    }
}
impl From<WaterGroundSurface> for AbstractFeature {
    fn from(v: WaterGroundSurface) -> Self {
        Self::WaterGroundSurface(Box::new(v))
    }
}
impl From<WaterSurface> for AbstractFeature {
    fn from(v: WaterSurface) -> Self {
        Self::WaterSurface(Box::new(v))
    }
}
pub trait AbstractFeatureAccessors {
    fn ceiling_surfaces(&self) -> impl Iterator<Item = &CeilingSurface>;
    fn doors(&self) -> impl Iterator<Item = &Door>;
    fn door_surfaces(&self) -> impl Iterator<Item = &DoorSurface>;
    fn floor_surfaces(&self) -> impl Iterator<Item = &FloorSurface>;
    fn ground_surfaces(&self) -> impl Iterator<Item = &GroundSurface>;
    fn interior_wall_surfaces(&self) -> impl Iterator<Item = &InteriorWallSurface>;
    fn other_constructions(&self) -> impl Iterator<Item = &OtherConstruction>;
    fn outer_ceiling_surfaces(&self) -> impl Iterator<Item = &OuterCeilingSurface>;
    fn outer_floor_surfaces(&self) -> impl Iterator<Item = &OuterFloorSurface>;
    fn roof_surfaces(&self) -> impl Iterator<Item = &RoofSurface>;
    fn wall_surfaces(&self) -> impl Iterator<Item = &WallSurface>;
    fn windows(&self) -> impl Iterator<Item = &Window>;
    fn window_surfaces(&self) -> impl Iterator<Item = &WindowSurface>;
    fn composite_timeseriess(&self) -> impl Iterator<Item = &CompositeTimeseries>;
    fn dynamizers(&self) -> impl Iterator<Item = &Dynamizer>;
    fn generic_timeseriess(&self) -> impl Iterator<Item = &GenericTimeseries>;
    fn standard_file_timeseriess(&self) -> impl Iterator<Item = &StandardFileTimeseries>;
    fn tabulated_file_timeseriess(
        &self,
    ) -> impl Iterator<Item = &TabulatedFileTimeseries>;
    fn point_clouds(&self) -> impl Iterator<Item = &PointCloud>;
    fn versions(&self) -> impl Iterator<Item = &Version>;
    fn version_transitions(&self) -> impl Iterator<Item = &VersionTransition>;
    fn appearances(&self) -> impl Iterator<Item = &Appearance>;
    fn georeferenced_textures(&self) -> impl Iterator<Item = &GeoreferencedTexture>;
    fn parameterized_textures(&self) -> impl Iterator<Item = &ParameterizedTexture>;
    fn x3_d_materials(&self) -> impl Iterator<Item = &X3DMaterial>;
    fn bridges(&self) -> impl Iterator<Item = &Bridge>;
    fn bridge_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BridgeConstructiveElement>;
    fn bridge_furnitures(&self) -> impl Iterator<Item = &BridgeFurniture>;
    fn bridge_installations(&self) -> impl Iterator<Item = &BridgeInstallation>;
    fn bridge_parts(&self) -> impl Iterator<Item = &BridgePart>;
    fn bridge_rooms(&self) -> impl Iterator<Item = &BridgeRoom>;
    fn buildings(&self) -> impl Iterator<Item = &Building>;
    fn building_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BuildingConstructiveElement>;
    fn building_furnitures(&self) -> impl Iterator<Item = &BuildingFurniture>;
    fn building_installations(&self) -> impl Iterator<Item = &BuildingInstallation>;
    fn building_parts(&self) -> impl Iterator<Item = &BuildingPart>;
    fn building_rooms(&self) -> impl Iterator<Item = &BuildingRoom>;
    fn building_units(&self) -> impl Iterator<Item = &BuildingUnit>;
    fn storeys(&self) -> impl Iterator<Item = &Storey>;
    fn city_furnitures(&self) -> impl Iterator<Item = &CityFurniture>;
    fn city_object_groups(&self) -> impl Iterator<Item = &CityObjectGroup>;
    fn addresss(&self) -> impl Iterator<Item = &Address>;
    fn city_models(&self) -> impl Iterator<Item = &CityModel>;
    fn closure_surfaces(&self) -> impl Iterator<Item = &ClosureSurface>;
    fn generic_logical_spaces(&self) -> impl Iterator<Item = &GenericLogicalSpace>;
    fn generic_occupied_spaces(&self) -> impl Iterator<Item = &GenericOccupiedSpace>;
    fn generic_thematic_surfaces(&self) -> impl Iterator<Item = &GenericThematicSurface>;
    fn generic_unoccupied_spaces(&self) -> impl Iterator<Item = &GenericUnoccupiedSpace>;
    fn land_uses(&self) -> impl Iterator<Item = &LandUse>;
    fn breakline_reliefs(&self) -> impl Iterator<Item = &BreaklineRelief>;
    fn mass_point_reliefs(&self) -> impl Iterator<Item = &MassPointRelief>;
    fn raster_reliefs(&self) -> impl Iterator<Item = &RasterRelief>;
    fn relief_features(&self) -> impl Iterator<Item = &ReliefFeature>;
    fn tin_reliefs(&self) -> impl Iterator<Item = &TINRelief>;
    fn auxiliary_traffic_areas(&self) -> impl Iterator<Item = &AuxiliaryTrafficArea>;
    fn auxiliary_traffic_spaces(&self) -> impl Iterator<Item = &AuxiliaryTrafficSpace>;
    fn clearance_spaces(&self) -> impl Iterator<Item = &ClearanceSpace>;
    fn holes(&self) -> impl Iterator<Item = &Hole>;
    fn hole_surfaces(&self) -> impl Iterator<Item = &HoleSurface>;
    fn intersections(&self) -> impl Iterator<Item = &Intersection>;
    fn markings(&self) -> impl Iterator<Item = &Marking>;
    fn railways(&self) -> impl Iterator<Item = &Railway>;
    fn roads(&self) -> impl Iterator<Item = &Road>;
    fn sections(&self) -> impl Iterator<Item = &Section>;
    fn squares(&self) -> impl Iterator<Item = &Square>;
    fn tracks(&self) -> impl Iterator<Item = &Track>;
    fn traffic_areas(&self) -> impl Iterator<Item = &TrafficArea>;
    fn traffic_spaces(&self) -> impl Iterator<Item = &TrafficSpace>;
    fn waterways(&self) -> impl Iterator<Item = &Waterway>;
    fn hollow_spaces(&self) -> impl Iterator<Item = &HollowSpace>;
    fn tunnels(&self) -> impl Iterator<Item = &Tunnel>;
    fn tunnel_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &TunnelConstructiveElement>;
    fn tunnel_furnitures(&self) -> impl Iterator<Item = &TunnelFurniture>;
    fn tunnel_installations(&self) -> impl Iterator<Item = &TunnelInstallation>;
    fn tunnel_parts(&self) -> impl Iterator<Item = &TunnelPart>;
    fn plant_covers(&self) -> impl Iterator<Item = &PlantCover>;
    fn solitary_vegetation_objects(
        &self,
    ) -> impl Iterator<Item = &SolitaryVegetationObject>;
    fn water_bodys(&self) -> impl Iterator<Item = &WaterBody>;
    fn water_ground_surfaces(&self) -> impl Iterator<Item = &WaterGroundSurface>;
    fn water_surfaces(&self) -> impl Iterator<Item = &WaterSurface>;
}
impl AbstractFeatureAccessors for [AbstractFeature] {
    fn ceiling_surfaces(&self) -> impl Iterator<Item = &CeilingSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::CeilingSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn doors(&self) -> impl Iterator<Item = &Door> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Door(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn door_surfaces(&self) -> impl Iterator<Item = &DoorSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::DoorSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn floor_surfaces(&self) -> impl Iterator<Item = &FloorSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::FloorSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn ground_surfaces(&self) -> impl Iterator<Item = &GroundSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::GroundSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn interior_wall_surfaces(&self) -> impl Iterator<Item = &InteriorWallSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::InteriorWallSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn other_constructions(&self) -> impl Iterator<Item = &OtherConstruction> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::OtherConstruction(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn outer_ceiling_surfaces(&self) -> impl Iterator<Item = &OuterCeilingSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::OuterCeilingSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn outer_floor_surfaces(&self) -> impl Iterator<Item = &OuterFloorSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::OuterFloorSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn roof_surfaces(&self) -> impl Iterator<Item = &RoofSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::RoofSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn wall_surfaces(&self) -> impl Iterator<Item = &WallSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::WallSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn windows(&self) -> impl Iterator<Item = &Window> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Window(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn window_surfaces(&self) -> impl Iterator<Item = &WindowSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::WindowSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn composite_timeseriess(&self) -> impl Iterator<Item = &CompositeTimeseries> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::CompositeTimeseries(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn dynamizers(&self) -> impl Iterator<Item = &Dynamizer> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Dynamizer(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_timeseriess(&self) -> impl Iterator<Item = &GenericTimeseries> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::GenericTimeseries(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn standard_file_timeseriess(
        &self,
    ) -> impl Iterator<Item = &StandardFileTimeseries> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::StandardFileTimeseries(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tabulated_file_timeseriess(
        &self,
    ) -> impl Iterator<Item = &TabulatedFileTimeseries> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::TabulatedFileTimeseries(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn point_clouds(&self) -> impl Iterator<Item = &PointCloud> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::PointCloud(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn versions(&self) -> impl Iterator<Item = &Version> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Version(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn version_transitions(&self) -> impl Iterator<Item = &VersionTransition> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::VersionTransition(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn appearances(&self) -> impl Iterator<Item = &Appearance> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Appearance(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn georeferenced_textures(&self) -> impl Iterator<Item = &GeoreferencedTexture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::GeoreferencedTexture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn parameterized_textures(&self) -> impl Iterator<Item = &ParameterizedTexture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::ParameterizedTexture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn x3_d_materials(&self) -> impl Iterator<Item = &X3DMaterial> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::X3DMaterial(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridges(&self) -> impl Iterator<Item = &Bridge> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Bridge(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BridgeConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::BridgeConstructiveElement(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_furnitures(&self) -> impl Iterator<Item = &BridgeFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::BridgeFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_installations(&self) -> impl Iterator<Item = &BridgeInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::BridgeInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_parts(&self) -> impl Iterator<Item = &BridgePart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::BridgePart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_rooms(&self) -> impl Iterator<Item = &BridgeRoom> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::BridgeRoom(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn buildings(&self) -> impl Iterator<Item = &Building> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Building(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BuildingConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::BuildingConstructiveElement(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_furnitures(&self) -> impl Iterator<Item = &BuildingFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::BuildingFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_installations(&self) -> impl Iterator<Item = &BuildingInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::BuildingInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_parts(&self) -> impl Iterator<Item = &BuildingPart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::BuildingPart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_rooms(&self) -> impl Iterator<Item = &BuildingRoom> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::BuildingRoom(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_units(&self) -> impl Iterator<Item = &BuildingUnit> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::BuildingUnit(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn storeys(&self) -> impl Iterator<Item = &Storey> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Storey(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn city_furnitures(&self) -> impl Iterator<Item = &CityFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::CityFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn city_object_groups(&self) -> impl Iterator<Item = &CityObjectGroup> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::CityObjectGroup(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn addresss(&self) -> impl Iterator<Item = &Address> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Address(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn city_models(&self) -> impl Iterator<Item = &CityModel> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::CityModel(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn closure_surfaces(&self) -> impl Iterator<Item = &ClosureSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::ClosureSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_logical_spaces(&self) -> impl Iterator<Item = &GenericLogicalSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::GenericLogicalSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_occupied_spaces(&self) -> impl Iterator<Item = &GenericOccupiedSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::GenericOccupiedSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_thematic_surfaces(
        &self,
    ) -> impl Iterator<Item = &GenericThematicSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::GenericThematicSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_unoccupied_spaces(
        &self,
    ) -> impl Iterator<Item = &GenericUnoccupiedSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::GenericUnoccupiedSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn land_uses(&self) -> impl Iterator<Item = &LandUse> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::LandUse(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn breakline_reliefs(&self) -> impl Iterator<Item = &BreaklineRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::BreaklineRelief(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn mass_point_reliefs(&self) -> impl Iterator<Item = &MassPointRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::MassPointRelief(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn raster_reliefs(&self) -> impl Iterator<Item = &RasterRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::RasterRelief(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn relief_features(&self) -> impl Iterator<Item = &ReliefFeature> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::ReliefFeature(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tin_reliefs(&self) -> impl Iterator<Item = &TINRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::TINRelief(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn auxiliary_traffic_areas(&self) -> impl Iterator<Item = &AuxiliaryTrafficArea> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::AuxiliaryTrafficArea(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn auxiliary_traffic_spaces(&self) -> impl Iterator<Item = &AuxiliaryTrafficSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::AuxiliaryTrafficSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn clearance_spaces(&self) -> impl Iterator<Item = &ClearanceSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::ClearanceSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn holes(&self) -> impl Iterator<Item = &Hole> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Hole(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn hole_surfaces(&self) -> impl Iterator<Item = &HoleSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::HoleSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn intersections(&self) -> impl Iterator<Item = &Intersection> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Intersection(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn markings(&self) -> impl Iterator<Item = &Marking> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Marking(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn railways(&self) -> impl Iterator<Item = &Railway> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Railway(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn roads(&self) -> impl Iterator<Item = &Road> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Road(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn sections(&self) -> impl Iterator<Item = &Section> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Section(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn squares(&self) -> impl Iterator<Item = &Square> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Square(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tracks(&self) -> impl Iterator<Item = &Track> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Track(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn traffic_areas(&self) -> impl Iterator<Item = &TrafficArea> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::TrafficArea(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn traffic_spaces(&self) -> impl Iterator<Item = &TrafficSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::TrafficSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn waterways(&self) -> impl Iterator<Item = &Waterway> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Waterway(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn hollow_spaces(&self) -> impl Iterator<Item = &HollowSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::HollowSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnels(&self) -> impl Iterator<Item = &Tunnel> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::Tunnel(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &TunnelConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::TunnelConstructiveElement(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_furnitures(&self) -> impl Iterator<Item = &TunnelFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::TunnelFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_installations(&self) -> impl Iterator<Item = &TunnelInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::TunnelInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_parts(&self) -> impl Iterator<Item = &TunnelPart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::TunnelPart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn plant_covers(&self) -> impl Iterator<Item = &PlantCover> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::PlantCover(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn solitary_vegetation_objects(
        &self,
    ) -> impl Iterator<Item = &SolitaryVegetationObject> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::SolitaryVegetationObject(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn water_bodys(&self) -> impl Iterator<Item = &WaterBody> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::WaterBody(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn water_ground_surfaces(&self) -> impl Iterator<Item = &WaterGroundSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::WaterGroundSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn water_surfaces(&self) -> impl Iterator<Item = &WaterSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeature::WaterSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
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
#[derive(Debug, Clone, Default)]
pub struct ImplicitGeometry {
    pub object_id: ID,
    pub transformation_matrix: TransformationMatrix4x4,
    pub mime_type: Option<MimeTypeValue>,
    pub library_object: Option<String>,
    pub relative_geometry: Option<()>,
    pub reference_point: crate::geometry::DirectPosition,
    pub appearance: Vec<AbstractAppearance>,
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
                                super::dispatchers::parse_abstract_appearance(
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
pub trait AbstractFeatureWithLifespanTrait: AbstractFeatureTrait {
    fn creation_date(&self) -> Option<&String>;
    fn termination_date(&self) -> Option<&String>;
    fn valid_from(&self) -> Option<&String>;
    fn valid_to(&self) -> Option<&String>;
}
#[derive(Debug, Clone)]
pub enum AbstractFeatureWithLifespan {
    CeilingSurface(Box<CeilingSurface>),
    Door(Box<Door>),
    DoorSurface(Box<DoorSurface>),
    FloorSurface(Box<FloorSurface>),
    GroundSurface(Box<GroundSurface>),
    InteriorWallSurface(Box<InteriorWallSurface>),
    OtherConstruction(Box<OtherConstruction>),
    OuterCeilingSurface(Box<OuterCeilingSurface>),
    OuterFloorSurface(Box<OuterFloorSurface>),
    RoofSurface(Box<RoofSurface>),
    WallSurface(Box<WallSurface>),
    Window(Box<Window>),
    WindowSurface(Box<WindowSurface>),
    Dynamizer(Box<Dynamizer>),
    Version(Box<Version>),
    VersionTransition(Box<VersionTransition>),
    Appearance(Box<Appearance>),
    Bridge(Box<Bridge>),
    BridgeConstructiveElement(Box<BridgeConstructiveElement>),
    BridgeFurniture(Box<BridgeFurniture>),
    BridgeInstallation(Box<BridgeInstallation>),
    BridgePart(Box<BridgePart>),
    BridgeRoom(Box<BridgeRoom>),
    Building(Box<Building>),
    BuildingConstructiveElement(Box<BuildingConstructiveElement>),
    BuildingFurniture(Box<BuildingFurniture>),
    BuildingInstallation(Box<BuildingInstallation>),
    BuildingPart(Box<BuildingPart>),
    BuildingRoom(Box<BuildingRoom>),
    BuildingUnit(Box<BuildingUnit>),
    Storey(Box<Storey>),
    CityFurniture(Box<CityFurniture>),
    CityObjectGroup(Box<CityObjectGroup>),
    CityModel(Box<CityModel>),
    ClosureSurface(Box<ClosureSurface>),
    GenericLogicalSpace(Box<GenericLogicalSpace>),
    GenericOccupiedSpace(Box<GenericOccupiedSpace>),
    GenericThematicSurface(Box<GenericThematicSurface>),
    GenericUnoccupiedSpace(Box<GenericUnoccupiedSpace>),
    LandUse(Box<LandUse>),
    BreaklineRelief(Box<BreaklineRelief>),
    MassPointRelief(Box<MassPointRelief>),
    RasterRelief(Box<RasterRelief>),
    ReliefFeature(Box<ReliefFeature>),
    TINRelief(Box<TINRelief>),
    AuxiliaryTrafficArea(Box<AuxiliaryTrafficArea>),
    AuxiliaryTrafficSpace(Box<AuxiliaryTrafficSpace>),
    ClearanceSpace(Box<ClearanceSpace>),
    Hole(Box<Hole>),
    HoleSurface(Box<HoleSurface>),
    Intersection(Box<Intersection>),
    Marking(Box<Marking>),
    Railway(Box<Railway>),
    Road(Box<Road>),
    Section(Box<Section>),
    Square(Box<Square>),
    Track(Box<Track>),
    TrafficArea(Box<TrafficArea>),
    TrafficSpace(Box<TrafficSpace>),
    Waterway(Box<Waterway>),
    HollowSpace(Box<HollowSpace>),
    Tunnel(Box<Tunnel>),
    TunnelConstructiveElement(Box<TunnelConstructiveElement>),
    TunnelFurniture(Box<TunnelFurniture>),
    TunnelInstallation(Box<TunnelInstallation>),
    TunnelPart(Box<TunnelPart>),
    PlantCover(Box<PlantCover>),
    SolitaryVegetationObject(Box<SolitaryVegetationObject>),
    WaterBody(Box<WaterBody>),
    WaterGroundSurface(Box<WaterGroundSurface>),
    WaterSurface(Box<WaterSurface>),
}
impl Default for AbstractFeatureWithLifespan {
    fn default() -> Self {
        Self::CeilingSurface(Box::default())
    }
}
impl AbstractFeatureTrait for AbstractFeatureWithLifespan {
    fn feature_id(&self) -> &ID {
        match self {
            Self::CeilingSurface(v) => v.feature_id(),
            Self::Door(v) => v.feature_id(),
            Self::DoorSurface(v) => v.feature_id(),
            Self::FloorSurface(v) => v.feature_id(),
            Self::GroundSurface(v) => v.feature_id(),
            Self::InteriorWallSurface(v) => v.feature_id(),
            Self::OtherConstruction(v) => v.feature_id(),
            Self::OuterCeilingSurface(v) => v.feature_id(),
            Self::OuterFloorSurface(v) => v.feature_id(),
            Self::RoofSurface(v) => v.feature_id(),
            Self::WallSurface(v) => v.feature_id(),
            Self::Window(v) => v.feature_id(),
            Self::WindowSurface(v) => v.feature_id(),
            Self::Dynamizer(v) => v.feature_id(),
            Self::Version(v) => v.feature_id(),
            Self::VersionTransition(v) => v.feature_id(),
            Self::Appearance(v) => v.feature_id(),
            Self::Bridge(v) => v.feature_id(),
            Self::BridgeConstructiveElement(v) => v.feature_id(),
            Self::BridgeFurniture(v) => v.feature_id(),
            Self::BridgeInstallation(v) => v.feature_id(),
            Self::BridgePart(v) => v.feature_id(),
            Self::BridgeRoom(v) => v.feature_id(),
            Self::Building(v) => v.feature_id(),
            Self::BuildingConstructiveElement(v) => v.feature_id(),
            Self::BuildingFurniture(v) => v.feature_id(),
            Self::BuildingInstallation(v) => v.feature_id(),
            Self::BuildingPart(v) => v.feature_id(),
            Self::BuildingRoom(v) => v.feature_id(),
            Self::BuildingUnit(v) => v.feature_id(),
            Self::Storey(v) => v.feature_id(),
            Self::CityFurniture(v) => v.feature_id(),
            Self::CityObjectGroup(v) => v.feature_id(),
            Self::CityModel(v) => v.feature_id(),
            Self::ClosureSurface(v) => v.feature_id(),
            Self::GenericLogicalSpace(v) => v.feature_id(),
            Self::GenericOccupiedSpace(v) => v.feature_id(),
            Self::GenericThematicSurface(v) => v.feature_id(),
            Self::GenericUnoccupiedSpace(v) => v.feature_id(),
            Self::LandUse(v) => v.feature_id(),
            Self::BreaklineRelief(v) => v.feature_id(),
            Self::MassPointRelief(v) => v.feature_id(),
            Self::RasterRelief(v) => v.feature_id(),
            Self::ReliefFeature(v) => v.feature_id(),
            Self::TINRelief(v) => v.feature_id(),
            Self::AuxiliaryTrafficArea(v) => v.feature_id(),
            Self::AuxiliaryTrafficSpace(v) => v.feature_id(),
            Self::ClearanceSpace(v) => v.feature_id(),
            Self::Hole(v) => v.feature_id(),
            Self::HoleSurface(v) => v.feature_id(),
            Self::Intersection(v) => v.feature_id(),
            Self::Marking(v) => v.feature_id(),
            Self::Railway(v) => v.feature_id(),
            Self::Road(v) => v.feature_id(),
            Self::Section(v) => v.feature_id(),
            Self::Square(v) => v.feature_id(),
            Self::Track(v) => v.feature_id(),
            Self::TrafficArea(v) => v.feature_id(),
            Self::TrafficSpace(v) => v.feature_id(),
            Self::Waterway(v) => v.feature_id(),
            Self::HollowSpace(v) => v.feature_id(),
            Self::Tunnel(v) => v.feature_id(),
            Self::TunnelConstructiveElement(v) => v.feature_id(),
            Self::TunnelFurniture(v) => v.feature_id(),
            Self::TunnelInstallation(v) => v.feature_id(),
            Self::TunnelPart(v) => v.feature_id(),
            Self::PlantCover(v) => v.feature_id(),
            Self::SolitaryVegetationObject(v) => v.feature_id(),
            Self::WaterBody(v) => v.feature_id(),
            Self::WaterGroundSurface(v) => v.feature_id(),
            Self::WaterSurface(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.identifier(),
            Self::Door(v) => v.identifier(),
            Self::DoorSurface(v) => v.identifier(),
            Self::FloorSurface(v) => v.identifier(),
            Self::GroundSurface(v) => v.identifier(),
            Self::InteriorWallSurface(v) => v.identifier(),
            Self::OtherConstruction(v) => v.identifier(),
            Self::OuterCeilingSurface(v) => v.identifier(),
            Self::OuterFloorSurface(v) => v.identifier(),
            Self::RoofSurface(v) => v.identifier(),
            Self::WallSurface(v) => v.identifier(),
            Self::Window(v) => v.identifier(),
            Self::WindowSurface(v) => v.identifier(),
            Self::Dynamizer(v) => v.identifier(),
            Self::Version(v) => v.identifier(),
            Self::VersionTransition(v) => v.identifier(),
            Self::Appearance(v) => v.identifier(),
            Self::Bridge(v) => v.identifier(),
            Self::BridgeConstructiveElement(v) => v.identifier(),
            Self::BridgeFurniture(v) => v.identifier(),
            Self::BridgeInstallation(v) => v.identifier(),
            Self::BridgePart(v) => v.identifier(),
            Self::BridgeRoom(v) => v.identifier(),
            Self::Building(v) => v.identifier(),
            Self::BuildingConstructiveElement(v) => v.identifier(),
            Self::BuildingFurniture(v) => v.identifier(),
            Self::BuildingInstallation(v) => v.identifier(),
            Self::BuildingPart(v) => v.identifier(),
            Self::BuildingRoom(v) => v.identifier(),
            Self::BuildingUnit(v) => v.identifier(),
            Self::Storey(v) => v.identifier(),
            Self::CityFurniture(v) => v.identifier(),
            Self::CityObjectGroup(v) => v.identifier(),
            Self::CityModel(v) => v.identifier(),
            Self::ClosureSurface(v) => v.identifier(),
            Self::GenericLogicalSpace(v) => v.identifier(),
            Self::GenericOccupiedSpace(v) => v.identifier(),
            Self::GenericThematicSurface(v) => v.identifier(),
            Self::GenericUnoccupiedSpace(v) => v.identifier(),
            Self::LandUse(v) => v.identifier(),
            Self::BreaklineRelief(v) => v.identifier(),
            Self::MassPointRelief(v) => v.identifier(),
            Self::RasterRelief(v) => v.identifier(),
            Self::ReliefFeature(v) => v.identifier(),
            Self::TINRelief(v) => v.identifier(),
            Self::AuxiliaryTrafficArea(v) => v.identifier(),
            Self::AuxiliaryTrafficSpace(v) => v.identifier(),
            Self::ClearanceSpace(v) => v.identifier(),
            Self::Hole(v) => v.identifier(),
            Self::HoleSurface(v) => v.identifier(),
            Self::Intersection(v) => v.identifier(),
            Self::Marking(v) => v.identifier(),
            Self::Railway(v) => v.identifier(),
            Self::Road(v) => v.identifier(),
            Self::Section(v) => v.identifier(),
            Self::Square(v) => v.identifier(),
            Self::Track(v) => v.identifier(),
            Self::TrafficArea(v) => v.identifier(),
            Self::TrafficSpace(v) => v.identifier(),
            Self::Waterway(v) => v.identifier(),
            Self::HollowSpace(v) => v.identifier(),
            Self::Tunnel(v) => v.identifier(),
            Self::TunnelConstructiveElement(v) => v.identifier(),
            Self::TunnelFurniture(v) => v.identifier(),
            Self::TunnelInstallation(v) => v.identifier(),
            Self::TunnelPart(v) => v.identifier(),
            Self::PlantCover(v) => v.identifier(),
            Self::SolitaryVegetationObject(v) => v.identifier(),
            Self::WaterBody(v) => v.identifier(),
            Self::WaterGroundSurface(v) => v.identifier(),
            Self::WaterSurface(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::CeilingSurface(v) => v.name(),
            Self::Door(v) => v.name(),
            Self::DoorSurface(v) => v.name(),
            Self::FloorSurface(v) => v.name(),
            Self::GroundSurface(v) => v.name(),
            Self::InteriorWallSurface(v) => v.name(),
            Self::OtherConstruction(v) => v.name(),
            Self::OuterCeilingSurface(v) => v.name(),
            Self::OuterFloorSurface(v) => v.name(),
            Self::RoofSurface(v) => v.name(),
            Self::WallSurface(v) => v.name(),
            Self::Window(v) => v.name(),
            Self::WindowSurface(v) => v.name(),
            Self::Dynamizer(v) => v.name(),
            Self::Version(v) => v.name(),
            Self::VersionTransition(v) => v.name(),
            Self::Appearance(v) => v.name(),
            Self::Bridge(v) => v.name(),
            Self::BridgeConstructiveElement(v) => v.name(),
            Self::BridgeFurniture(v) => v.name(),
            Self::BridgeInstallation(v) => v.name(),
            Self::BridgePart(v) => v.name(),
            Self::BridgeRoom(v) => v.name(),
            Self::Building(v) => v.name(),
            Self::BuildingConstructiveElement(v) => v.name(),
            Self::BuildingFurniture(v) => v.name(),
            Self::BuildingInstallation(v) => v.name(),
            Self::BuildingPart(v) => v.name(),
            Self::BuildingRoom(v) => v.name(),
            Self::BuildingUnit(v) => v.name(),
            Self::Storey(v) => v.name(),
            Self::CityFurniture(v) => v.name(),
            Self::CityObjectGroup(v) => v.name(),
            Self::CityModel(v) => v.name(),
            Self::ClosureSurface(v) => v.name(),
            Self::GenericLogicalSpace(v) => v.name(),
            Self::GenericOccupiedSpace(v) => v.name(),
            Self::GenericThematicSurface(v) => v.name(),
            Self::GenericUnoccupiedSpace(v) => v.name(),
            Self::LandUse(v) => v.name(),
            Self::BreaklineRelief(v) => v.name(),
            Self::MassPointRelief(v) => v.name(),
            Self::RasterRelief(v) => v.name(),
            Self::ReliefFeature(v) => v.name(),
            Self::TINRelief(v) => v.name(),
            Self::AuxiliaryTrafficArea(v) => v.name(),
            Self::AuxiliaryTrafficSpace(v) => v.name(),
            Self::ClearanceSpace(v) => v.name(),
            Self::Hole(v) => v.name(),
            Self::HoleSurface(v) => v.name(),
            Self::Intersection(v) => v.name(),
            Self::Marking(v) => v.name(),
            Self::Railway(v) => v.name(),
            Self::Road(v) => v.name(),
            Self::Section(v) => v.name(),
            Self::Square(v) => v.name(),
            Self::Track(v) => v.name(),
            Self::TrafficArea(v) => v.name(),
            Self::TrafficSpace(v) => v.name(),
            Self::Waterway(v) => v.name(),
            Self::HollowSpace(v) => v.name(),
            Self::Tunnel(v) => v.name(),
            Self::TunnelConstructiveElement(v) => v.name(),
            Self::TunnelFurniture(v) => v.name(),
            Self::TunnelInstallation(v) => v.name(),
            Self::TunnelPart(v) => v.name(),
            Self::PlantCover(v) => v.name(),
            Self::SolitaryVegetationObject(v) => v.name(),
            Self::WaterBody(v) => v.name(),
            Self::WaterGroundSurface(v) => v.name(),
            Self::WaterSurface(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.description(),
            Self::Door(v) => v.description(),
            Self::DoorSurface(v) => v.description(),
            Self::FloorSurface(v) => v.description(),
            Self::GroundSurface(v) => v.description(),
            Self::InteriorWallSurface(v) => v.description(),
            Self::OtherConstruction(v) => v.description(),
            Self::OuterCeilingSurface(v) => v.description(),
            Self::OuterFloorSurface(v) => v.description(),
            Self::RoofSurface(v) => v.description(),
            Self::WallSurface(v) => v.description(),
            Self::Window(v) => v.description(),
            Self::WindowSurface(v) => v.description(),
            Self::Dynamizer(v) => v.description(),
            Self::Version(v) => v.description(),
            Self::VersionTransition(v) => v.description(),
            Self::Appearance(v) => v.description(),
            Self::Bridge(v) => v.description(),
            Self::BridgeConstructiveElement(v) => v.description(),
            Self::BridgeFurniture(v) => v.description(),
            Self::BridgeInstallation(v) => v.description(),
            Self::BridgePart(v) => v.description(),
            Self::BridgeRoom(v) => v.description(),
            Self::Building(v) => v.description(),
            Self::BuildingConstructiveElement(v) => v.description(),
            Self::BuildingFurniture(v) => v.description(),
            Self::BuildingInstallation(v) => v.description(),
            Self::BuildingPart(v) => v.description(),
            Self::BuildingRoom(v) => v.description(),
            Self::BuildingUnit(v) => v.description(),
            Self::Storey(v) => v.description(),
            Self::CityFurniture(v) => v.description(),
            Self::CityObjectGroup(v) => v.description(),
            Self::CityModel(v) => v.description(),
            Self::ClosureSurface(v) => v.description(),
            Self::GenericLogicalSpace(v) => v.description(),
            Self::GenericOccupiedSpace(v) => v.description(),
            Self::GenericThematicSurface(v) => v.description(),
            Self::GenericUnoccupiedSpace(v) => v.description(),
            Self::LandUse(v) => v.description(),
            Self::BreaklineRelief(v) => v.description(),
            Self::MassPointRelief(v) => v.description(),
            Self::RasterRelief(v) => v.description(),
            Self::ReliefFeature(v) => v.description(),
            Self::TINRelief(v) => v.description(),
            Self::AuxiliaryTrafficArea(v) => v.description(),
            Self::AuxiliaryTrafficSpace(v) => v.description(),
            Self::ClearanceSpace(v) => v.description(),
            Self::Hole(v) => v.description(),
            Self::HoleSurface(v) => v.description(),
            Self::Intersection(v) => v.description(),
            Self::Marking(v) => v.description(),
            Self::Railway(v) => v.description(),
            Self::Road(v) => v.description(),
            Self::Section(v) => v.description(),
            Self::Square(v) => v.description(),
            Self::Track(v) => v.description(),
            Self::TrafficArea(v) => v.description(),
            Self::TrafficSpace(v) => v.description(),
            Self::Waterway(v) => v.description(),
            Self::HollowSpace(v) => v.description(),
            Self::Tunnel(v) => v.description(),
            Self::TunnelConstructiveElement(v) => v.description(),
            Self::TunnelFurniture(v) => v.description(),
            Self::TunnelInstallation(v) => v.description(),
            Self::TunnelPart(v) => v.description(),
            Self::PlantCover(v) => v.description(),
            Self::SolitaryVegetationObject(v) => v.description(),
            Self::WaterBody(v) => v.description(),
            Self::WaterGroundSurface(v) => v.description(),
            Self::WaterSurface(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractFeatureWithLifespan {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.creation_date(),
            Self::Door(v) => v.creation_date(),
            Self::DoorSurface(v) => v.creation_date(),
            Self::FloorSurface(v) => v.creation_date(),
            Self::GroundSurface(v) => v.creation_date(),
            Self::InteriorWallSurface(v) => v.creation_date(),
            Self::OtherConstruction(v) => v.creation_date(),
            Self::OuterCeilingSurface(v) => v.creation_date(),
            Self::OuterFloorSurface(v) => v.creation_date(),
            Self::RoofSurface(v) => v.creation_date(),
            Self::WallSurface(v) => v.creation_date(),
            Self::Window(v) => v.creation_date(),
            Self::WindowSurface(v) => v.creation_date(),
            Self::Dynamizer(v) => v.creation_date(),
            Self::Version(v) => v.creation_date(),
            Self::VersionTransition(v) => v.creation_date(),
            Self::Appearance(v) => v.creation_date(),
            Self::Bridge(v) => v.creation_date(),
            Self::BridgeConstructiveElement(v) => v.creation_date(),
            Self::BridgeFurniture(v) => v.creation_date(),
            Self::BridgeInstallation(v) => v.creation_date(),
            Self::BridgePart(v) => v.creation_date(),
            Self::BridgeRoom(v) => v.creation_date(),
            Self::Building(v) => v.creation_date(),
            Self::BuildingConstructiveElement(v) => v.creation_date(),
            Self::BuildingFurniture(v) => v.creation_date(),
            Self::BuildingInstallation(v) => v.creation_date(),
            Self::BuildingPart(v) => v.creation_date(),
            Self::BuildingRoom(v) => v.creation_date(),
            Self::BuildingUnit(v) => v.creation_date(),
            Self::Storey(v) => v.creation_date(),
            Self::CityFurniture(v) => v.creation_date(),
            Self::CityObjectGroup(v) => v.creation_date(),
            Self::CityModel(v) => v.creation_date(),
            Self::ClosureSurface(v) => v.creation_date(),
            Self::GenericLogicalSpace(v) => v.creation_date(),
            Self::GenericOccupiedSpace(v) => v.creation_date(),
            Self::GenericThematicSurface(v) => v.creation_date(),
            Self::GenericUnoccupiedSpace(v) => v.creation_date(),
            Self::LandUse(v) => v.creation_date(),
            Self::BreaklineRelief(v) => v.creation_date(),
            Self::MassPointRelief(v) => v.creation_date(),
            Self::RasterRelief(v) => v.creation_date(),
            Self::ReliefFeature(v) => v.creation_date(),
            Self::TINRelief(v) => v.creation_date(),
            Self::AuxiliaryTrafficArea(v) => v.creation_date(),
            Self::AuxiliaryTrafficSpace(v) => v.creation_date(),
            Self::ClearanceSpace(v) => v.creation_date(),
            Self::Hole(v) => v.creation_date(),
            Self::HoleSurface(v) => v.creation_date(),
            Self::Intersection(v) => v.creation_date(),
            Self::Marking(v) => v.creation_date(),
            Self::Railway(v) => v.creation_date(),
            Self::Road(v) => v.creation_date(),
            Self::Section(v) => v.creation_date(),
            Self::Square(v) => v.creation_date(),
            Self::Track(v) => v.creation_date(),
            Self::TrafficArea(v) => v.creation_date(),
            Self::TrafficSpace(v) => v.creation_date(),
            Self::Waterway(v) => v.creation_date(),
            Self::HollowSpace(v) => v.creation_date(),
            Self::Tunnel(v) => v.creation_date(),
            Self::TunnelConstructiveElement(v) => v.creation_date(),
            Self::TunnelFurniture(v) => v.creation_date(),
            Self::TunnelInstallation(v) => v.creation_date(),
            Self::TunnelPart(v) => v.creation_date(),
            Self::PlantCover(v) => v.creation_date(),
            Self::SolitaryVegetationObject(v) => v.creation_date(),
            Self::WaterBody(v) => v.creation_date(),
            Self::WaterGroundSurface(v) => v.creation_date(),
            Self::WaterSurface(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.termination_date(),
            Self::Door(v) => v.termination_date(),
            Self::DoorSurface(v) => v.termination_date(),
            Self::FloorSurface(v) => v.termination_date(),
            Self::GroundSurface(v) => v.termination_date(),
            Self::InteriorWallSurface(v) => v.termination_date(),
            Self::OtherConstruction(v) => v.termination_date(),
            Self::OuterCeilingSurface(v) => v.termination_date(),
            Self::OuterFloorSurface(v) => v.termination_date(),
            Self::RoofSurface(v) => v.termination_date(),
            Self::WallSurface(v) => v.termination_date(),
            Self::Window(v) => v.termination_date(),
            Self::WindowSurface(v) => v.termination_date(),
            Self::Dynamizer(v) => v.termination_date(),
            Self::Version(v) => v.termination_date(),
            Self::VersionTransition(v) => v.termination_date(),
            Self::Appearance(v) => v.termination_date(),
            Self::Bridge(v) => v.termination_date(),
            Self::BridgeConstructiveElement(v) => v.termination_date(),
            Self::BridgeFurniture(v) => v.termination_date(),
            Self::BridgeInstallation(v) => v.termination_date(),
            Self::BridgePart(v) => v.termination_date(),
            Self::BridgeRoom(v) => v.termination_date(),
            Self::Building(v) => v.termination_date(),
            Self::BuildingConstructiveElement(v) => v.termination_date(),
            Self::BuildingFurniture(v) => v.termination_date(),
            Self::BuildingInstallation(v) => v.termination_date(),
            Self::BuildingPart(v) => v.termination_date(),
            Self::BuildingRoom(v) => v.termination_date(),
            Self::BuildingUnit(v) => v.termination_date(),
            Self::Storey(v) => v.termination_date(),
            Self::CityFurniture(v) => v.termination_date(),
            Self::CityObjectGroup(v) => v.termination_date(),
            Self::CityModel(v) => v.termination_date(),
            Self::ClosureSurface(v) => v.termination_date(),
            Self::GenericLogicalSpace(v) => v.termination_date(),
            Self::GenericOccupiedSpace(v) => v.termination_date(),
            Self::GenericThematicSurface(v) => v.termination_date(),
            Self::GenericUnoccupiedSpace(v) => v.termination_date(),
            Self::LandUse(v) => v.termination_date(),
            Self::BreaklineRelief(v) => v.termination_date(),
            Self::MassPointRelief(v) => v.termination_date(),
            Self::RasterRelief(v) => v.termination_date(),
            Self::ReliefFeature(v) => v.termination_date(),
            Self::TINRelief(v) => v.termination_date(),
            Self::AuxiliaryTrafficArea(v) => v.termination_date(),
            Self::AuxiliaryTrafficSpace(v) => v.termination_date(),
            Self::ClearanceSpace(v) => v.termination_date(),
            Self::Hole(v) => v.termination_date(),
            Self::HoleSurface(v) => v.termination_date(),
            Self::Intersection(v) => v.termination_date(),
            Self::Marking(v) => v.termination_date(),
            Self::Railway(v) => v.termination_date(),
            Self::Road(v) => v.termination_date(),
            Self::Section(v) => v.termination_date(),
            Self::Square(v) => v.termination_date(),
            Self::Track(v) => v.termination_date(),
            Self::TrafficArea(v) => v.termination_date(),
            Self::TrafficSpace(v) => v.termination_date(),
            Self::Waterway(v) => v.termination_date(),
            Self::HollowSpace(v) => v.termination_date(),
            Self::Tunnel(v) => v.termination_date(),
            Self::TunnelConstructiveElement(v) => v.termination_date(),
            Self::TunnelFurniture(v) => v.termination_date(),
            Self::TunnelInstallation(v) => v.termination_date(),
            Self::TunnelPart(v) => v.termination_date(),
            Self::PlantCover(v) => v.termination_date(),
            Self::SolitaryVegetationObject(v) => v.termination_date(),
            Self::WaterBody(v) => v.termination_date(),
            Self::WaterGroundSurface(v) => v.termination_date(),
            Self::WaterSurface(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.valid_from(),
            Self::Door(v) => v.valid_from(),
            Self::DoorSurface(v) => v.valid_from(),
            Self::FloorSurface(v) => v.valid_from(),
            Self::GroundSurface(v) => v.valid_from(),
            Self::InteriorWallSurface(v) => v.valid_from(),
            Self::OtherConstruction(v) => v.valid_from(),
            Self::OuterCeilingSurface(v) => v.valid_from(),
            Self::OuterFloorSurface(v) => v.valid_from(),
            Self::RoofSurface(v) => v.valid_from(),
            Self::WallSurface(v) => v.valid_from(),
            Self::Window(v) => v.valid_from(),
            Self::WindowSurface(v) => v.valid_from(),
            Self::Dynamizer(v) => v.valid_from(),
            Self::Version(v) => v.valid_from(),
            Self::VersionTransition(v) => v.valid_from(),
            Self::Appearance(v) => v.valid_from(),
            Self::Bridge(v) => v.valid_from(),
            Self::BridgeConstructiveElement(v) => v.valid_from(),
            Self::BridgeFurniture(v) => v.valid_from(),
            Self::BridgeInstallation(v) => v.valid_from(),
            Self::BridgePart(v) => v.valid_from(),
            Self::BridgeRoom(v) => v.valid_from(),
            Self::Building(v) => v.valid_from(),
            Self::BuildingConstructiveElement(v) => v.valid_from(),
            Self::BuildingFurniture(v) => v.valid_from(),
            Self::BuildingInstallation(v) => v.valid_from(),
            Self::BuildingPart(v) => v.valid_from(),
            Self::BuildingRoom(v) => v.valid_from(),
            Self::BuildingUnit(v) => v.valid_from(),
            Self::Storey(v) => v.valid_from(),
            Self::CityFurniture(v) => v.valid_from(),
            Self::CityObjectGroup(v) => v.valid_from(),
            Self::CityModel(v) => v.valid_from(),
            Self::ClosureSurface(v) => v.valid_from(),
            Self::GenericLogicalSpace(v) => v.valid_from(),
            Self::GenericOccupiedSpace(v) => v.valid_from(),
            Self::GenericThematicSurface(v) => v.valid_from(),
            Self::GenericUnoccupiedSpace(v) => v.valid_from(),
            Self::LandUse(v) => v.valid_from(),
            Self::BreaklineRelief(v) => v.valid_from(),
            Self::MassPointRelief(v) => v.valid_from(),
            Self::RasterRelief(v) => v.valid_from(),
            Self::ReliefFeature(v) => v.valid_from(),
            Self::TINRelief(v) => v.valid_from(),
            Self::AuxiliaryTrafficArea(v) => v.valid_from(),
            Self::AuxiliaryTrafficSpace(v) => v.valid_from(),
            Self::ClearanceSpace(v) => v.valid_from(),
            Self::Hole(v) => v.valid_from(),
            Self::HoleSurface(v) => v.valid_from(),
            Self::Intersection(v) => v.valid_from(),
            Self::Marking(v) => v.valid_from(),
            Self::Railway(v) => v.valid_from(),
            Self::Road(v) => v.valid_from(),
            Self::Section(v) => v.valid_from(),
            Self::Square(v) => v.valid_from(),
            Self::Track(v) => v.valid_from(),
            Self::TrafficArea(v) => v.valid_from(),
            Self::TrafficSpace(v) => v.valid_from(),
            Self::Waterway(v) => v.valid_from(),
            Self::HollowSpace(v) => v.valid_from(),
            Self::Tunnel(v) => v.valid_from(),
            Self::TunnelConstructiveElement(v) => v.valid_from(),
            Self::TunnelFurniture(v) => v.valid_from(),
            Self::TunnelInstallation(v) => v.valid_from(),
            Self::TunnelPart(v) => v.valid_from(),
            Self::PlantCover(v) => v.valid_from(),
            Self::SolitaryVegetationObject(v) => v.valid_from(),
            Self::WaterBody(v) => v.valid_from(),
            Self::WaterGroundSurface(v) => v.valid_from(),
            Self::WaterSurface(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.valid_to(),
            Self::Door(v) => v.valid_to(),
            Self::DoorSurface(v) => v.valid_to(),
            Self::FloorSurface(v) => v.valid_to(),
            Self::GroundSurface(v) => v.valid_to(),
            Self::InteriorWallSurface(v) => v.valid_to(),
            Self::OtherConstruction(v) => v.valid_to(),
            Self::OuterCeilingSurface(v) => v.valid_to(),
            Self::OuterFloorSurface(v) => v.valid_to(),
            Self::RoofSurface(v) => v.valid_to(),
            Self::WallSurface(v) => v.valid_to(),
            Self::Window(v) => v.valid_to(),
            Self::WindowSurface(v) => v.valid_to(),
            Self::Dynamizer(v) => v.valid_to(),
            Self::Version(v) => v.valid_to(),
            Self::VersionTransition(v) => v.valid_to(),
            Self::Appearance(v) => v.valid_to(),
            Self::Bridge(v) => v.valid_to(),
            Self::BridgeConstructiveElement(v) => v.valid_to(),
            Self::BridgeFurniture(v) => v.valid_to(),
            Self::BridgeInstallation(v) => v.valid_to(),
            Self::BridgePart(v) => v.valid_to(),
            Self::BridgeRoom(v) => v.valid_to(),
            Self::Building(v) => v.valid_to(),
            Self::BuildingConstructiveElement(v) => v.valid_to(),
            Self::BuildingFurniture(v) => v.valid_to(),
            Self::BuildingInstallation(v) => v.valid_to(),
            Self::BuildingPart(v) => v.valid_to(),
            Self::BuildingRoom(v) => v.valid_to(),
            Self::BuildingUnit(v) => v.valid_to(),
            Self::Storey(v) => v.valid_to(),
            Self::CityFurniture(v) => v.valid_to(),
            Self::CityObjectGroup(v) => v.valid_to(),
            Self::CityModel(v) => v.valid_to(),
            Self::ClosureSurface(v) => v.valid_to(),
            Self::GenericLogicalSpace(v) => v.valid_to(),
            Self::GenericOccupiedSpace(v) => v.valid_to(),
            Self::GenericThematicSurface(v) => v.valid_to(),
            Self::GenericUnoccupiedSpace(v) => v.valid_to(),
            Self::LandUse(v) => v.valid_to(),
            Self::BreaklineRelief(v) => v.valid_to(),
            Self::MassPointRelief(v) => v.valid_to(),
            Self::RasterRelief(v) => v.valid_to(),
            Self::ReliefFeature(v) => v.valid_to(),
            Self::TINRelief(v) => v.valid_to(),
            Self::AuxiliaryTrafficArea(v) => v.valid_to(),
            Self::AuxiliaryTrafficSpace(v) => v.valid_to(),
            Self::ClearanceSpace(v) => v.valid_to(),
            Self::Hole(v) => v.valid_to(),
            Self::HoleSurface(v) => v.valid_to(),
            Self::Intersection(v) => v.valid_to(),
            Self::Marking(v) => v.valid_to(),
            Self::Railway(v) => v.valid_to(),
            Self::Road(v) => v.valid_to(),
            Self::Section(v) => v.valid_to(),
            Self::Square(v) => v.valid_to(),
            Self::Track(v) => v.valid_to(),
            Self::TrafficArea(v) => v.valid_to(),
            Self::TrafficSpace(v) => v.valid_to(),
            Self::Waterway(v) => v.valid_to(),
            Self::HollowSpace(v) => v.valid_to(),
            Self::Tunnel(v) => v.valid_to(),
            Self::TunnelConstructiveElement(v) => v.valid_to(),
            Self::TunnelFurniture(v) => v.valid_to(),
            Self::TunnelInstallation(v) => v.valid_to(),
            Self::TunnelPart(v) => v.valid_to(),
            Self::PlantCover(v) => v.valid_to(),
            Self::SolitaryVegetationObject(v) => v.valid_to(),
            Self::WaterBody(v) => v.valid_to(),
            Self::WaterGroundSurface(v) => v.valid_to(),
            Self::WaterSurface(v) => v.valid_to(),
        }
    }
}
impl From<CeilingSurface> for AbstractFeatureWithLifespan {
    fn from(v: CeilingSurface) -> Self {
        Self::CeilingSurface(Box::new(v))
    }
}
impl From<Door> for AbstractFeatureWithLifespan {
    fn from(v: Door) -> Self {
        Self::Door(Box::new(v))
    }
}
impl From<DoorSurface> for AbstractFeatureWithLifespan {
    fn from(v: DoorSurface) -> Self {
        Self::DoorSurface(Box::new(v))
    }
}
impl From<FloorSurface> for AbstractFeatureWithLifespan {
    fn from(v: FloorSurface) -> Self {
        Self::FloorSurface(Box::new(v))
    }
}
impl From<GroundSurface> for AbstractFeatureWithLifespan {
    fn from(v: GroundSurface) -> Self {
        Self::GroundSurface(Box::new(v))
    }
}
impl From<InteriorWallSurface> for AbstractFeatureWithLifespan {
    fn from(v: InteriorWallSurface) -> Self {
        Self::InteriorWallSurface(Box::new(v))
    }
}
impl From<OtherConstruction> for AbstractFeatureWithLifespan {
    fn from(v: OtherConstruction) -> Self {
        Self::OtherConstruction(Box::new(v))
    }
}
impl From<OuterCeilingSurface> for AbstractFeatureWithLifespan {
    fn from(v: OuterCeilingSurface) -> Self {
        Self::OuterCeilingSurface(Box::new(v))
    }
}
impl From<OuterFloorSurface> for AbstractFeatureWithLifespan {
    fn from(v: OuterFloorSurface) -> Self {
        Self::OuterFloorSurface(Box::new(v))
    }
}
impl From<RoofSurface> for AbstractFeatureWithLifespan {
    fn from(v: RoofSurface) -> Self {
        Self::RoofSurface(Box::new(v))
    }
}
impl From<WallSurface> for AbstractFeatureWithLifespan {
    fn from(v: WallSurface) -> Self {
        Self::WallSurface(Box::new(v))
    }
}
impl From<Window> for AbstractFeatureWithLifespan {
    fn from(v: Window) -> Self {
        Self::Window(Box::new(v))
    }
}
impl From<WindowSurface> for AbstractFeatureWithLifespan {
    fn from(v: WindowSurface) -> Self {
        Self::WindowSurface(Box::new(v))
    }
}
impl From<Dynamizer> for AbstractFeatureWithLifespan {
    fn from(v: Dynamizer) -> Self {
        Self::Dynamizer(Box::new(v))
    }
}
impl From<Version> for AbstractFeatureWithLifespan {
    fn from(v: Version) -> Self {
        Self::Version(Box::new(v))
    }
}
impl From<VersionTransition> for AbstractFeatureWithLifespan {
    fn from(v: VersionTransition) -> Self {
        Self::VersionTransition(Box::new(v))
    }
}
impl From<Appearance> for AbstractFeatureWithLifespan {
    fn from(v: Appearance) -> Self {
        Self::Appearance(Box::new(v))
    }
}
impl From<Bridge> for AbstractFeatureWithLifespan {
    fn from(v: Bridge) -> Self {
        Self::Bridge(Box::new(v))
    }
}
impl From<BridgeConstructiveElement> for AbstractFeatureWithLifespan {
    fn from(v: BridgeConstructiveElement) -> Self {
        Self::BridgeConstructiveElement(Box::new(v))
    }
}
impl From<BridgeFurniture> for AbstractFeatureWithLifespan {
    fn from(v: BridgeFurniture) -> Self {
        Self::BridgeFurniture(Box::new(v))
    }
}
impl From<BridgeInstallation> for AbstractFeatureWithLifespan {
    fn from(v: BridgeInstallation) -> Self {
        Self::BridgeInstallation(Box::new(v))
    }
}
impl From<BridgePart> for AbstractFeatureWithLifespan {
    fn from(v: BridgePart) -> Self {
        Self::BridgePart(Box::new(v))
    }
}
impl From<BridgeRoom> for AbstractFeatureWithLifespan {
    fn from(v: BridgeRoom) -> Self {
        Self::BridgeRoom(Box::new(v))
    }
}
impl From<Building> for AbstractFeatureWithLifespan {
    fn from(v: Building) -> Self {
        Self::Building(Box::new(v))
    }
}
impl From<BuildingConstructiveElement> for AbstractFeatureWithLifespan {
    fn from(v: BuildingConstructiveElement) -> Self {
        Self::BuildingConstructiveElement(Box::new(v))
    }
}
impl From<BuildingFurniture> for AbstractFeatureWithLifespan {
    fn from(v: BuildingFurniture) -> Self {
        Self::BuildingFurniture(Box::new(v))
    }
}
impl From<BuildingInstallation> for AbstractFeatureWithLifespan {
    fn from(v: BuildingInstallation) -> Self {
        Self::BuildingInstallation(Box::new(v))
    }
}
impl From<BuildingPart> for AbstractFeatureWithLifespan {
    fn from(v: BuildingPart) -> Self {
        Self::BuildingPart(Box::new(v))
    }
}
impl From<BuildingRoom> for AbstractFeatureWithLifespan {
    fn from(v: BuildingRoom) -> Self {
        Self::BuildingRoom(Box::new(v))
    }
}
impl From<BuildingUnit> for AbstractFeatureWithLifespan {
    fn from(v: BuildingUnit) -> Self {
        Self::BuildingUnit(Box::new(v))
    }
}
impl From<Storey> for AbstractFeatureWithLifespan {
    fn from(v: Storey) -> Self {
        Self::Storey(Box::new(v))
    }
}
impl From<CityFurniture> for AbstractFeatureWithLifespan {
    fn from(v: CityFurniture) -> Self {
        Self::CityFurniture(Box::new(v))
    }
}
impl From<CityObjectGroup> for AbstractFeatureWithLifespan {
    fn from(v: CityObjectGroup) -> Self {
        Self::CityObjectGroup(Box::new(v))
    }
}
impl From<CityModel> for AbstractFeatureWithLifespan {
    fn from(v: CityModel) -> Self {
        Self::CityModel(Box::new(v))
    }
}
impl From<ClosureSurface> for AbstractFeatureWithLifespan {
    fn from(v: ClosureSurface) -> Self {
        Self::ClosureSurface(Box::new(v))
    }
}
impl From<GenericLogicalSpace> for AbstractFeatureWithLifespan {
    fn from(v: GenericLogicalSpace) -> Self {
        Self::GenericLogicalSpace(Box::new(v))
    }
}
impl From<GenericOccupiedSpace> for AbstractFeatureWithLifespan {
    fn from(v: GenericOccupiedSpace) -> Self {
        Self::GenericOccupiedSpace(Box::new(v))
    }
}
impl From<GenericThematicSurface> for AbstractFeatureWithLifespan {
    fn from(v: GenericThematicSurface) -> Self {
        Self::GenericThematicSurface(Box::new(v))
    }
}
impl From<GenericUnoccupiedSpace> for AbstractFeatureWithLifespan {
    fn from(v: GenericUnoccupiedSpace) -> Self {
        Self::GenericUnoccupiedSpace(Box::new(v))
    }
}
impl From<LandUse> for AbstractFeatureWithLifespan {
    fn from(v: LandUse) -> Self {
        Self::LandUse(Box::new(v))
    }
}
impl From<BreaklineRelief> for AbstractFeatureWithLifespan {
    fn from(v: BreaklineRelief) -> Self {
        Self::BreaklineRelief(Box::new(v))
    }
}
impl From<MassPointRelief> for AbstractFeatureWithLifespan {
    fn from(v: MassPointRelief) -> Self {
        Self::MassPointRelief(Box::new(v))
    }
}
impl From<RasterRelief> for AbstractFeatureWithLifespan {
    fn from(v: RasterRelief) -> Self {
        Self::RasterRelief(Box::new(v))
    }
}
impl From<ReliefFeature> for AbstractFeatureWithLifespan {
    fn from(v: ReliefFeature) -> Self {
        Self::ReliefFeature(Box::new(v))
    }
}
impl From<TINRelief> for AbstractFeatureWithLifespan {
    fn from(v: TINRelief) -> Self {
        Self::TINRelief(Box::new(v))
    }
}
impl From<AuxiliaryTrafficArea> for AbstractFeatureWithLifespan {
    fn from(v: AuxiliaryTrafficArea) -> Self {
        Self::AuxiliaryTrafficArea(Box::new(v))
    }
}
impl From<AuxiliaryTrafficSpace> for AbstractFeatureWithLifespan {
    fn from(v: AuxiliaryTrafficSpace) -> Self {
        Self::AuxiliaryTrafficSpace(Box::new(v))
    }
}
impl From<ClearanceSpace> for AbstractFeatureWithLifespan {
    fn from(v: ClearanceSpace) -> Self {
        Self::ClearanceSpace(Box::new(v))
    }
}
impl From<Hole> for AbstractFeatureWithLifespan {
    fn from(v: Hole) -> Self {
        Self::Hole(Box::new(v))
    }
}
impl From<HoleSurface> for AbstractFeatureWithLifespan {
    fn from(v: HoleSurface) -> Self {
        Self::HoleSurface(Box::new(v))
    }
}
impl From<Intersection> for AbstractFeatureWithLifespan {
    fn from(v: Intersection) -> Self {
        Self::Intersection(Box::new(v))
    }
}
impl From<Marking> for AbstractFeatureWithLifespan {
    fn from(v: Marking) -> Self {
        Self::Marking(Box::new(v))
    }
}
impl From<Railway> for AbstractFeatureWithLifespan {
    fn from(v: Railway) -> Self {
        Self::Railway(Box::new(v))
    }
}
impl From<Road> for AbstractFeatureWithLifespan {
    fn from(v: Road) -> Self {
        Self::Road(Box::new(v))
    }
}
impl From<Section> for AbstractFeatureWithLifespan {
    fn from(v: Section) -> Self {
        Self::Section(Box::new(v))
    }
}
impl From<Square> for AbstractFeatureWithLifespan {
    fn from(v: Square) -> Self {
        Self::Square(Box::new(v))
    }
}
impl From<Track> for AbstractFeatureWithLifespan {
    fn from(v: Track) -> Self {
        Self::Track(Box::new(v))
    }
}
impl From<TrafficArea> for AbstractFeatureWithLifespan {
    fn from(v: TrafficArea) -> Self {
        Self::TrafficArea(Box::new(v))
    }
}
impl From<TrafficSpace> for AbstractFeatureWithLifespan {
    fn from(v: TrafficSpace) -> Self {
        Self::TrafficSpace(Box::new(v))
    }
}
impl From<Waterway> for AbstractFeatureWithLifespan {
    fn from(v: Waterway) -> Self {
        Self::Waterway(Box::new(v))
    }
}
impl From<HollowSpace> for AbstractFeatureWithLifespan {
    fn from(v: HollowSpace) -> Self {
        Self::HollowSpace(Box::new(v))
    }
}
impl From<Tunnel> for AbstractFeatureWithLifespan {
    fn from(v: Tunnel) -> Self {
        Self::Tunnel(Box::new(v))
    }
}
impl From<TunnelConstructiveElement> for AbstractFeatureWithLifespan {
    fn from(v: TunnelConstructiveElement) -> Self {
        Self::TunnelConstructiveElement(Box::new(v))
    }
}
impl From<TunnelFurniture> for AbstractFeatureWithLifespan {
    fn from(v: TunnelFurniture) -> Self {
        Self::TunnelFurniture(Box::new(v))
    }
}
impl From<TunnelInstallation> for AbstractFeatureWithLifespan {
    fn from(v: TunnelInstallation) -> Self {
        Self::TunnelInstallation(Box::new(v))
    }
}
impl From<TunnelPart> for AbstractFeatureWithLifespan {
    fn from(v: TunnelPart) -> Self {
        Self::TunnelPart(Box::new(v))
    }
}
impl From<PlantCover> for AbstractFeatureWithLifespan {
    fn from(v: PlantCover) -> Self {
        Self::PlantCover(Box::new(v))
    }
}
impl From<SolitaryVegetationObject> for AbstractFeatureWithLifespan {
    fn from(v: SolitaryVegetationObject) -> Self {
        Self::SolitaryVegetationObject(Box::new(v))
    }
}
impl From<WaterBody> for AbstractFeatureWithLifespan {
    fn from(v: WaterBody) -> Self {
        Self::WaterBody(Box::new(v))
    }
}
impl From<WaterGroundSurface> for AbstractFeatureWithLifespan {
    fn from(v: WaterGroundSurface) -> Self {
        Self::WaterGroundSurface(Box::new(v))
    }
}
impl From<WaterSurface> for AbstractFeatureWithLifespan {
    fn from(v: WaterSurface) -> Self {
        Self::WaterSurface(Box::new(v))
    }
}
pub trait AbstractFeatureWithLifespanAccessors {
    fn ceiling_surfaces(&self) -> impl Iterator<Item = &CeilingSurface>;
    fn doors(&self) -> impl Iterator<Item = &Door>;
    fn door_surfaces(&self) -> impl Iterator<Item = &DoorSurface>;
    fn floor_surfaces(&self) -> impl Iterator<Item = &FloorSurface>;
    fn ground_surfaces(&self) -> impl Iterator<Item = &GroundSurface>;
    fn interior_wall_surfaces(&self) -> impl Iterator<Item = &InteriorWallSurface>;
    fn other_constructions(&self) -> impl Iterator<Item = &OtherConstruction>;
    fn outer_ceiling_surfaces(&self) -> impl Iterator<Item = &OuterCeilingSurface>;
    fn outer_floor_surfaces(&self) -> impl Iterator<Item = &OuterFloorSurface>;
    fn roof_surfaces(&self) -> impl Iterator<Item = &RoofSurface>;
    fn wall_surfaces(&self) -> impl Iterator<Item = &WallSurface>;
    fn windows(&self) -> impl Iterator<Item = &Window>;
    fn window_surfaces(&self) -> impl Iterator<Item = &WindowSurface>;
    fn dynamizers(&self) -> impl Iterator<Item = &Dynamizer>;
    fn versions(&self) -> impl Iterator<Item = &Version>;
    fn version_transitions(&self) -> impl Iterator<Item = &VersionTransition>;
    fn appearances(&self) -> impl Iterator<Item = &Appearance>;
    fn bridges(&self) -> impl Iterator<Item = &Bridge>;
    fn bridge_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BridgeConstructiveElement>;
    fn bridge_furnitures(&self) -> impl Iterator<Item = &BridgeFurniture>;
    fn bridge_installations(&self) -> impl Iterator<Item = &BridgeInstallation>;
    fn bridge_parts(&self) -> impl Iterator<Item = &BridgePart>;
    fn bridge_rooms(&self) -> impl Iterator<Item = &BridgeRoom>;
    fn buildings(&self) -> impl Iterator<Item = &Building>;
    fn building_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BuildingConstructiveElement>;
    fn building_furnitures(&self) -> impl Iterator<Item = &BuildingFurniture>;
    fn building_installations(&self) -> impl Iterator<Item = &BuildingInstallation>;
    fn building_parts(&self) -> impl Iterator<Item = &BuildingPart>;
    fn building_rooms(&self) -> impl Iterator<Item = &BuildingRoom>;
    fn building_units(&self) -> impl Iterator<Item = &BuildingUnit>;
    fn storeys(&self) -> impl Iterator<Item = &Storey>;
    fn city_furnitures(&self) -> impl Iterator<Item = &CityFurniture>;
    fn city_object_groups(&self) -> impl Iterator<Item = &CityObjectGroup>;
    fn city_models(&self) -> impl Iterator<Item = &CityModel>;
    fn closure_surfaces(&self) -> impl Iterator<Item = &ClosureSurface>;
    fn generic_logical_spaces(&self) -> impl Iterator<Item = &GenericLogicalSpace>;
    fn generic_occupied_spaces(&self) -> impl Iterator<Item = &GenericOccupiedSpace>;
    fn generic_thematic_surfaces(&self) -> impl Iterator<Item = &GenericThematicSurface>;
    fn generic_unoccupied_spaces(&self) -> impl Iterator<Item = &GenericUnoccupiedSpace>;
    fn land_uses(&self) -> impl Iterator<Item = &LandUse>;
    fn breakline_reliefs(&self) -> impl Iterator<Item = &BreaklineRelief>;
    fn mass_point_reliefs(&self) -> impl Iterator<Item = &MassPointRelief>;
    fn raster_reliefs(&self) -> impl Iterator<Item = &RasterRelief>;
    fn relief_features(&self) -> impl Iterator<Item = &ReliefFeature>;
    fn tin_reliefs(&self) -> impl Iterator<Item = &TINRelief>;
    fn auxiliary_traffic_areas(&self) -> impl Iterator<Item = &AuxiliaryTrafficArea>;
    fn auxiliary_traffic_spaces(&self) -> impl Iterator<Item = &AuxiliaryTrafficSpace>;
    fn clearance_spaces(&self) -> impl Iterator<Item = &ClearanceSpace>;
    fn holes(&self) -> impl Iterator<Item = &Hole>;
    fn hole_surfaces(&self) -> impl Iterator<Item = &HoleSurface>;
    fn intersections(&self) -> impl Iterator<Item = &Intersection>;
    fn markings(&self) -> impl Iterator<Item = &Marking>;
    fn railways(&self) -> impl Iterator<Item = &Railway>;
    fn roads(&self) -> impl Iterator<Item = &Road>;
    fn sections(&self) -> impl Iterator<Item = &Section>;
    fn squares(&self) -> impl Iterator<Item = &Square>;
    fn tracks(&self) -> impl Iterator<Item = &Track>;
    fn traffic_areas(&self) -> impl Iterator<Item = &TrafficArea>;
    fn traffic_spaces(&self) -> impl Iterator<Item = &TrafficSpace>;
    fn waterways(&self) -> impl Iterator<Item = &Waterway>;
    fn hollow_spaces(&self) -> impl Iterator<Item = &HollowSpace>;
    fn tunnels(&self) -> impl Iterator<Item = &Tunnel>;
    fn tunnel_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &TunnelConstructiveElement>;
    fn tunnel_furnitures(&self) -> impl Iterator<Item = &TunnelFurniture>;
    fn tunnel_installations(&self) -> impl Iterator<Item = &TunnelInstallation>;
    fn tunnel_parts(&self) -> impl Iterator<Item = &TunnelPart>;
    fn plant_covers(&self) -> impl Iterator<Item = &PlantCover>;
    fn solitary_vegetation_objects(
        &self,
    ) -> impl Iterator<Item = &SolitaryVegetationObject>;
    fn water_bodys(&self) -> impl Iterator<Item = &WaterBody>;
    fn water_ground_surfaces(&self) -> impl Iterator<Item = &WaterGroundSurface>;
    fn water_surfaces(&self) -> impl Iterator<Item = &WaterSurface>;
}
impl AbstractFeatureWithLifespanAccessors for [AbstractFeatureWithLifespan] {
    fn ceiling_surfaces(&self) -> impl Iterator<Item = &CeilingSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::CeilingSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn doors(&self) -> impl Iterator<Item = &Door> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Door(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn door_surfaces(&self) -> impl Iterator<Item = &DoorSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::DoorSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn floor_surfaces(&self) -> impl Iterator<Item = &FloorSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::FloorSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn ground_surfaces(&self) -> impl Iterator<Item = &GroundSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::GroundSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn interior_wall_surfaces(&self) -> impl Iterator<Item = &InteriorWallSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::InteriorWallSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn other_constructions(&self) -> impl Iterator<Item = &OtherConstruction> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::OtherConstruction(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn outer_ceiling_surfaces(&self) -> impl Iterator<Item = &OuterCeilingSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::OuterCeilingSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn outer_floor_surfaces(&self) -> impl Iterator<Item = &OuterFloorSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::OuterFloorSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn roof_surfaces(&self) -> impl Iterator<Item = &RoofSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::RoofSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn wall_surfaces(&self) -> impl Iterator<Item = &WallSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::WallSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn windows(&self) -> impl Iterator<Item = &Window> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Window(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn window_surfaces(&self) -> impl Iterator<Item = &WindowSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::WindowSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn dynamizers(&self) -> impl Iterator<Item = &Dynamizer> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Dynamizer(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn versions(&self) -> impl Iterator<Item = &Version> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Version(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn version_transitions(&self) -> impl Iterator<Item = &VersionTransition> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::VersionTransition(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn appearances(&self) -> impl Iterator<Item = &Appearance> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Appearance(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridges(&self) -> impl Iterator<Item = &Bridge> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Bridge(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BridgeConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::BridgeConstructiveElement(v) => {
                    Some(v.as_ref())
                }
                _ => None,
            })
    }
    fn bridge_furnitures(&self) -> impl Iterator<Item = &BridgeFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::BridgeFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_installations(&self) -> impl Iterator<Item = &BridgeInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::BridgeInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_parts(&self) -> impl Iterator<Item = &BridgePart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::BridgePart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_rooms(&self) -> impl Iterator<Item = &BridgeRoom> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::BridgeRoom(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn buildings(&self) -> impl Iterator<Item = &Building> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Building(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BuildingConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::BuildingConstructiveElement(v) => {
                    Some(v.as_ref())
                }
                _ => None,
            })
    }
    fn building_furnitures(&self) -> impl Iterator<Item = &BuildingFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::BuildingFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_installations(&self) -> impl Iterator<Item = &BuildingInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::BuildingInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_parts(&self) -> impl Iterator<Item = &BuildingPart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::BuildingPart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_rooms(&self) -> impl Iterator<Item = &BuildingRoom> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::BuildingRoom(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_units(&self) -> impl Iterator<Item = &BuildingUnit> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::BuildingUnit(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn storeys(&self) -> impl Iterator<Item = &Storey> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Storey(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn city_furnitures(&self) -> impl Iterator<Item = &CityFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::CityFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn city_object_groups(&self) -> impl Iterator<Item = &CityObjectGroup> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::CityObjectGroup(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn city_models(&self) -> impl Iterator<Item = &CityModel> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::CityModel(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn closure_surfaces(&self) -> impl Iterator<Item = &ClosureSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::ClosureSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_logical_spaces(&self) -> impl Iterator<Item = &GenericLogicalSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::GenericLogicalSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_occupied_spaces(&self) -> impl Iterator<Item = &GenericOccupiedSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::GenericOccupiedSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_thematic_surfaces(
        &self,
    ) -> impl Iterator<Item = &GenericThematicSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::GenericThematicSurface(v) => {
                    Some(v.as_ref())
                }
                _ => None,
            })
    }
    fn generic_unoccupied_spaces(
        &self,
    ) -> impl Iterator<Item = &GenericUnoccupiedSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::GenericUnoccupiedSpace(v) => {
                    Some(v.as_ref())
                }
                _ => None,
            })
    }
    fn land_uses(&self) -> impl Iterator<Item = &LandUse> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::LandUse(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn breakline_reliefs(&self) -> impl Iterator<Item = &BreaklineRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::BreaklineRelief(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn mass_point_reliefs(&self) -> impl Iterator<Item = &MassPointRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::MassPointRelief(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn raster_reliefs(&self) -> impl Iterator<Item = &RasterRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::RasterRelief(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn relief_features(&self) -> impl Iterator<Item = &ReliefFeature> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::ReliefFeature(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tin_reliefs(&self) -> impl Iterator<Item = &TINRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::TINRelief(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn auxiliary_traffic_areas(&self) -> impl Iterator<Item = &AuxiliaryTrafficArea> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::AuxiliaryTrafficArea(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn auxiliary_traffic_spaces(&self) -> impl Iterator<Item = &AuxiliaryTrafficSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::AuxiliaryTrafficSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn clearance_spaces(&self) -> impl Iterator<Item = &ClearanceSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::ClearanceSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn holes(&self) -> impl Iterator<Item = &Hole> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Hole(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn hole_surfaces(&self) -> impl Iterator<Item = &HoleSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::HoleSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn intersections(&self) -> impl Iterator<Item = &Intersection> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Intersection(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn markings(&self) -> impl Iterator<Item = &Marking> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Marking(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn railways(&self) -> impl Iterator<Item = &Railway> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Railway(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn roads(&self) -> impl Iterator<Item = &Road> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Road(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn sections(&self) -> impl Iterator<Item = &Section> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Section(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn squares(&self) -> impl Iterator<Item = &Square> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Square(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tracks(&self) -> impl Iterator<Item = &Track> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Track(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn traffic_areas(&self) -> impl Iterator<Item = &TrafficArea> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::TrafficArea(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn traffic_spaces(&self) -> impl Iterator<Item = &TrafficSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::TrafficSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn waterways(&self) -> impl Iterator<Item = &Waterway> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Waterway(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn hollow_spaces(&self) -> impl Iterator<Item = &HollowSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::HollowSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnels(&self) -> impl Iterator<Item = &Tunnel> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::Tunnel(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &TunnelConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::TunnelConstructiveElement(v) => {
                    Some(v.as_ref())
                }
                _ => None,
            })
    }
    fn tunnel_furnitures(&self) -> impl Iterator<Item = &TunnelFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::TunnelFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_installations(&self) -> impl Iterator<Item = &TunnelInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::TunnelInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_parts(&self) -> impl Iterator<Item = &TunnelPart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::TunnelPart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn plant_covers(&self) -> impl Iterator<Item = &PlantCover> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::PlantCover(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn solitary_vegetation_objects(
        &self,
    ) -> impl Iterator<Item = &SolitaryVegetationObject> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::SolitaryVegetationObject(v) => {
                    Some(v.as_ref())
                }
                _ => None,
            })
    }
    fn water_bodys(&self) -> impl Iterator<Item = &WaterBody> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::WaterBody(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn water_ground_surfaces(&self) -> impl Iterator<Item = &WaterGroundSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::WaterGroundSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn water_surfaces(&self) -> impl Iterator<Item = &WaterSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractFeatureWithLifespan::WaterSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
}
pub trait AbstractPointCloudTrait: AbstractFeatureTrait {}
#[derive(Debug, Clone)]
pub enum AbstractPointCloud {
    PointCloud(PointCloud),
}
impl Default for AbstractPointCloud {
    fn default() -> Self {
        Self::PointCloud(Default::default())
    }
}
impl AbstractFeatureTrait for AbstractPointCloud {
    fn feature_id(&self) -> &ID {
        match self {
            Self::PointCloud(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::PointCloud(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::PointCloud(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::PointCloud(v) => v.description(),
        }
    }
}
impl AbstractPointCloudTrait for AbstractPointCloud {}
impl From<PointCloud> for AbstractPointCloud {
    fn from(v: PointCloud) -> Self {
        Self::PointCloud(v)
    }
}
pub trait AbstractPointCloudAccessors {
    fn point_clouds(&self) -> impl Iterator<Item = &PointCloud>;
}
impl AbstractPointCloudAccessors for [AbstractPointCloud] {
    fn point_clouds(&self) -> impl Iterator<Item = &PointCloud> {
        self.iter()
            .map(|item| match item {
                AbstractPointCloud::PointCloud(v) => v,
            })
    }
}
#[derive(Debug, Clone, Default)]
pub struct Address {
    pub feature_id: ID,
    pub identifier: Option<String>,
    pub name: Vec<String>,
    pub description: Option<String>,
    pub multi_point: Option<Vec<crate::geometry::DirectPosition>>,
    pub xal_address: XALAddress,
}
impl AbstractFeatureTrait for Address {
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
        while let Some(_info) = sub.next_element()? {
            sub.skip_element()?;
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
        while let Some(_info) = sub.next_element()? {
            sub.skip_element()?;
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
        while let Some(_info) = sub.next_element()? {
            sub.skip_element()?;
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
pub trait AbstractAppearanceTrait: AbstractFeatureWithLifespanTrait {}
#[derive(Debug, Clone)]
pub enum AbstractAppearance {
    Appearance(Appearance),
}
impl Default for AbstractAppearance {
    fn default() -> Self {
        Self::Appearance(Default::default())
    }
}
impl AbstractFeatureTrait for AbstractAppearance {
    fn feature_id(&self) -> &ID {
        match self {
            Self::Appearance(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::Appearance(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::Appearance(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::Appearance(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractAppearance {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::Appearance(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::Appearance(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::Appearance(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::Appearance(v) => v.valid_to(),
        }
    }
}
impl AbstractAppearanceTrait for AbstractAppearance {}
impl From<Appearance> for AbstractAppearance {
    fn from(v: Appearance) -> Self {
        Self::Appearance(v)
    }
}
pub trait AbstractAppearanceAccessors {
    fn appearances(&self) -> impl Iterator<Item = &Appearance>;
}
impl AbstractAppearanceAccessors for [AbstractAppearance] {
    fn appearances(&self) -> impl Iterator<Item = &Appearance> {
        self.iter()
            .map(|item| match item {
                AbstractAppearance::Appearance(v) => v,
            })
    }
}
pub trait AbstractCityObjectTrait: AbstractFeatureWithLifespanTrait {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain>;
    fn relative_to_water(&self) -> Option<RelativeToWater>;
    fn appearance(&self) -> &[AbstractAppearance];
    fn generalizes_to(&self) -> &[AbstractCityObject];
    fn external_reference(&self) -> &[ExternalReference];
    fn related_to(&self) -> &[AbstractCityObject];
    fn dynamizer(&self) -> &[AbstractDynamizer];
}
#[derive(Debug, Clone)]
pub enum AbstractCityObject {
    CeilingSurface(Box<CeilingSurface>),
    Door(Box<Door>),
    DoorSurface(Box<DoorSurface>),
    FloorSurface(Box<FloorSurface>),
    GroundSurface(Box<GroundSurface>),
    InteriorWallSurface(Box<InteriorWallSurface>),
    OtherConstruction(Box<OtherConstruction>),
    OuterCeilingSurface(Box<OuterCeilingSurface>),
    OuterFloorSurface(Box<OuterFloorSurface>),
    RoofSurface(Box<RoofSurface>),
    WallSurface(Box<WallSurface>),
    Window(Box<Window>),
    WindowSurface(Box<WindowSurface>),
    Bridge(Box<Bridge>),
    BridgeConstructiveElement(Box<BridgeConstructiveElement>),
    BridgeFurniture(Box<BridgeFurniture>),
    BridgeInstallation(Box<BridgeInstallation>),
    BridgePart(Box<BridgePart>),
    BridgeRoom(Box<BridgeRoom>),
    Building(Box<Building>),
    BuildingConstructiveElement(Box<BuildingConstructiveElement>),
    BuildingFurniture(Box<BuildingFurniture>),
    BuildingInstallation(Box<BuildingInstallation>),
    BuildingPart(Box<BuildingPart>),
    BuildingRoom(Box<BuildingRoom>),
    BuildingUnit(Box<BuildingUnit>),
    Storey(Box<Storey>),
    CityFurniture(Box<CityFurniture>),
    CityObjectGroup(Box<CityObjectGroup>),
    ClosureSurface(Box<ClosureSurface>),
    GenericLogicalSpace(Box<GenericLogicalSpace>),
    GenericOccupiedSpace(Box<GenericOccupiedSpace>),
    GenericThematicSurface(Box<GenericThematicSurface>),
    GenericUnoccupiedSpace(Box<GenericUnoccupiedSpace>),
    LandUse(Box<LandUse>),
    BreaklineRelief(Box<BreaklineRelief>),
    MassPointRelief(Box<MassPointRelief>),
    RasterRelief(Box<RasterRelief>),
    ReliefFeature(Box<ReliefFeature>),
    TINRelief(Box<TINRelief>),
    AuxiliaryTrafficArea(Box<AuxiliaryTrafficArea>),
    AuxiliaryTrafficSpace(Box<AuxiliaryTrafficSpace>),
    ClearanceSpace(Box<ClearanceSpace>),
    Hole(Box<Hole>),
    HoleSurface(Box<HoleSurface>),
    Intersection(Box<Intersection>),
    Marking(Box<Marking>),
    Railway(Box<Railway>),
    Road(Box<Road>),
    Section(Box<Section>),
    Square(Box<Square>),
    Track(Box<Track>),
    TrafficArea(Box<TrafficArea>),
    TrafficSpace(Box<TrafficSpace>),
    Waterway(Box<Waterway>),
    HollowSpace(Box<HollowSpace>),
    Tunnel(Box<Tunnel>),
    TunnelConstructiveElement(Box<TunnelConstructiveElement>),
    TunnelFurniture(Box<TunnelFurniture>),
    TunnelInstallation(Box<TunnelInstallation>),
    TunnelPart(Box<TunnelPart>),
    PlantCover(Box<PlantCover>),
    SolitaryVegetationObject(Box<SolitaryVegetationObject>),
    WaterBody(Box<WaterBody>),
    WaterGroundSurface(Box<WaterGroundSurface>),
    WaterSurface(Box<WaterSurface>),
}
impl Default for AbstractCityObject {
    fn default() -> Self {
        Self::CeilingSurface(Box::default())
    }
}
impl AbstractFeatureTrait for AbstractCityObject {
    fn feature_id(&self) -> &ID {
        match self {
            Self::CeilingSurface(v) => v.feature_id(),
            Self::Door(v) => v.feature_id(),
            Self::DoorSurface(v) => v.feature_id(),
            Self::FloorSurface(v) => v.feature_id(),
            Self::GroundSurface(v) => v.feature_id(),
            Self::InteriorWallSurface(v) => v.feature_id(),
            Self::OtherConstruction(v) => v.feature_id(),
            Self::OuterCeilingSurface(v) => v.feature_id(),
            Self::OuterFloorSurface(v) => v.feature_id(),
            Self::RoofSurface(v) => v.feature_id(),
            Self::WallSurface(v) => v.feature_id(),
            Self::Window(v) => v.feature_id(),
            Self::WindowSurface(v) => v.feature_id(),
            Self::Bridge(v) => v.feature_id(),
            Self::BridgeConstructiveElement(v) => v.feature_id(),
            Self::BridgeFurniture(v) => v.feature_id(),
            Self::BridgeInstallation(v) => v.feature_id(),
            Self::BridgePart(v) => v.feature_id(),
            Self::BridgeRoom(v) => v.feature_id(),
            Self::Building(v) => v.feature_id(),
            Self::BuildingConstructiveElement(v) => v.feature_id(),
            Self::BuildingFurniture(v) => v.feature_id(),
            Self::BuildingInstallation(v) => v.feature_id(),
            Self::BuildingPart(v) => v.feature_id(),
            Self::BuildingRoom(v) => v.feature_id(),
            Self::BuildingUnit(v) => v.feature_id(),
            Self::Storey(v) => v.feature_id(),
            Self::CityFurniture(v) => v.feature_id(),
            Self::CityObjectGroup(v) => v.feature_id(),
            Self::ClosureSurface(v) => v.feature_id(),
            Self::GenericLogicalSpace(v) => v.feature_id(),
            Self::GenericOccupiedSpace(v) => v.feature_id(),
            Self::GenericThematicSurface(v) => v.feature_id(),
            Self::GenericUnoccupiedSpace(v) => v.feature_id(),
            Self::LandUse(v) => v.feature_id(),
            Self::BreaklineRelief(v) => v.feature_id(),
            Self::MassPointRelief(v) => v.feature_id(),
            Self::RasterRelief(v) => v.feature_id(),
            Self::ReliefFeature(v) => v.feature_id(),
            Self::TINRelief(v) => v.feature_id(),
            Self::AuxiliaryTrafficArea(v) => v.feature_id(),
            Self::AuxiliaryTrafficSpace(v) => v.feature_id(),
            Self::ClearanceSpace(v) => v.feature_id(),
            Self::Hole(v) => v.feature_id(),
            Self::HoleSurface(v) => v.feature_id(),
            Self::Intersection(v) => v.feature_id(),
            Self::Marking(v) => v.feature_id(),
            Self::Railway(v) => v.feature_id(),
            Self::Road(v) => v.feature_id(),
            Self::Section(v) => v.feature_id(),
            Self::Square(v) => v.feature_id(),
            Self::Track(v) => v.feature_id(),
            Self::TrafficArea(v) => v.feature_id(),
            Self::TrafficSpace(v) => v.feature_id(),
            Self::Waterway(v) => v.feature_id(),
            Self::HollowSpace(v) => v.feature_id(),
            Self::Tunnel(v) => v.feature_id(),
            Self::TunnelConstructiveElement(v) => v.feature_id(),
            Self::TunnelFurniture(v) => v.feature_id(),
            Self::TunnelInstallation(v) => v.feature_id(),
            Self::TunnelPart(v) => v.feature_id(),
            Self::PlantCover(v) => v.feature_id(),
            Self::SolitaryVegetationObject(v) => v.feature_id(),
            Self::WaterBody(v) => v.feature_id(),
            Self::WaterGroundSurface(v) => v.feature_id(),
            Self::WaterSurface(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.identifier(),
            Self::Door(v) => v.identifier(),
            Self::DoorSurface(v) => v.identifier(),
            Self::FloorSurface(v) => v.identifier(),
            Self::GroundSurface(v) => v.identifier(),
            Self::InteriorWallSurface(v) => v.identifier(),
            Self::OtherConstruction(v) => v.identifier(),
            Self::OuterCeilingSurface(v) => v.identifier(),
            Self::OuterFloorSurface(v) => v.identifier(),
            Self::RoofSurface(v) => v.identifier(),
            Self::WallSurface(v) => v.identifier(),
            Self::Window(v) => v.identifier(),
            Self::WindowSurface(v) => v.identifier(),
            Self::Bridge(v) => v.identifier(),
            Self::BridgeConstructiveElement(v) => v.identifier(),
            Self::BridgeFurniture(v) => v.identifier(),
            Self::BridgeInstallation(v) => v.identifier(),
            Self::BridgePart(v) => v.identifier(),
            Self::BridgeRoom(v) => v.identifier(),
            Self::Building(v) => v.identifier(),
            Self::BuildingConstructiveElement(v) => v.identifier(),
            Self::BuildingFurniture(v) => v.identifier(),
            Self::BuildingInstallation(v) => v.identifier(),
            Self::BuildingPart(v) => v.identifier(),
            Self::BuildingRoom(v) => v.identifier(),
            Self::BuildingUnit(v) => v.identifier(),
            Self::Storey(v) => v.identifier(),
            Self::CityFurniture(v) => v.identifier(),
            Self::CityObjectGroup(v) => v.identifier(),
            Self::ClosureSurface(v) => v.identifier(),
            Self::GenericLogicalSpace(v) => v.identifier(),
            Self::GenericOccupiedSpace(v) => v.identifier(),
            Self::GenericThematicSurface(v) => v.identifier(),
            Self::GenericUnoccupiedSpace(v) => v.identifier(),
            Self::LandUse(v) => v.identifier(),
            Self::BreaklineRelief(v) => v.identifier(),
            Self::MassPointRelief(v) => v.identifier(),
            Self::RasterRelief(v) => v.identifier(),
            Self::ReliefFeature(v) => v.identifier(),
            Self::TINRelief(v) => v.identifier(),
            Self::AuxiliaryTrafficArea(v) => v.identifier(),
            Self::AuxiliaryTrafficSpace(v) => v.identifier(),
            Self::ClearanceSpace(v) => v.identifier(),
            Self::Hole(v) => v.identifier(),
            Self::HoleSurface(v) => v.identifier(),
            Self::Intersection(v) => v.identifier(),
            Self::Marking(v) => v.identifier(),
            Self::Railway(v) => v.identifier(),
            Self::Road(v) => v.identifier(),
            Self::Section(v) => v.identifier(),
            Self::Square(v) => v.identifier(),
            Self::Track(v) => v.identifier(),
            Self::TrafficArea(v) => v.identifier(),
            Self::TrafficSpace(v) => v.identifier(),
            Self::Waterway(v) => v.identifier(),
            Self::HollowSpace(v) => v.identifier(),
            Self::Tunnel(v) => v.identifier(),
            Self::TunnelConstructiveElement(v) => v.identifier(),
            Self::TunnelFurniture(v) => v.identifier(),
            Self::TunnelInstallation(v) => v.identifier(),
            Self::TunnelPart(v) => v.identifier(),
            Self::PlantCover(v) => v.identifier(),
            Self::SolitaryVegetationObject(v) => v.identifier(),
            Self::WaterBody(v) => v.identifier(),
            Self::WaterGroundSurface(v) => v.identifier(),
            Self::WaterSurface(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::CeilingSurface(v) => v.name(),
            Self::Door(v) => v.name(),
            Self::DoorSurface(v) => v.name(),
            Self::FloorSurface(v) => v.name(),
            Self::GroundSurface(v) => v.name(),
            Self::InteriorWallSurface(v) => v.name(),
            Self::OtherConstruction(v) => v.name(),
            Self::OuterCeilingSurface(v) => v.name(),
            Self::OuterFloorSurface(v) => v.name(),
            Self::RoofSurface(v) => v.name(),
            Self::WallSurface(v) => v.name(),
            Self::Window(v) => v.name(),
            Self::WindowSurface(v) => v.name(),
            Self::Bridge(v) => v.name(),
            Self::BridgeConstructiveElement(v) => v.name(),
            Self::BridgeFurniture(v) => v.name(),
            Self::BridgeInstallation(v) => v.name(),
            Self::BridgePart(v) => v.name(),
            Self::BridgeRoom(v) => v.name(),
            Self::Building(v) => v.name(),
            Self::BuildingConstructiveElement(v) => v.name(),
            Self::BuildingFurniture(v) => v.name(),
            Self::BuildingInstallation(v) => v.name(),
            Self::BuildingPart(v) => v.name(),
            Self::BuildingRoom(v) => v.name(),
            Self::BuildingUnit(v) => v.name(),
            Self::Storey(v) => v.name(),
            Self::CityFurniture(v) => v.name(),
            Self::CityObjectGroup(v) => v.name(),
            Self::ClosureSurface(v) => v.name(),
            Self::GenericLogicalSpace(v) => v.name(),
            Self::GenericOccupiedSpace(v) => v.name(),
            Self::GenericThematicSurface(v) => v.name(),
            Self::GenericUnoccupiedSpace(v) => v.name(),
            Self::LandUse(v) => v.name(),
            Self::BreaklineRelief(v) => v.name(),
            Self::MassPointRelief(v) => v.name(),
            Self::RasterRelief(v) => v.name(),
            Self::ReliefFeature(v) => v.name(),
            Self::TINRelief(v) => v.name(),
            Self::AuxiliaryTrafficArea(v) => v.name(),
            Self::AuxiliaryTrafficSpace(v) => v.name(),
            Self::ClearanceSpace(v) => v.name(),
            Self::Hole(v) => v.name(),
            Self::HoleSurface(v) => v.name(),
            Self::Intersection(v) => v.name(),
            Self::Marking(v) => v.name(),
            Self::Railway(v) => v.name(),
            Self::Road(v) => v.name(),
            Self::Section(v) => v.name(),
            Self::Square(v) => v.name(),
            Self::Track(v) => v.name(),
            Self::TrafficArea(v) => v.name(),
            Self::TrafficSpace(v) => v.name(),
            Self::Waterway(v) => v.name(),
            Self::HollowSpace(v) => v.name(),
            Self::Tunnel(v) => v.name(),
            Self::TunnelConstructiveElement(v) => v.name(),
            Self::TunnelFurniture(v) => v.name(),
            Self::TunnelInstallation(v) => v.name(),
            Self::TunnelPart(v) => v.name(),
            Self::PlantCover(v) => v.name(),
            Self::SolitaryVegetationObject(v) => v.name(),
            Self::WaterBody(v) => v.name(),
            Self::WaterGroundSurface(v) => v.name(),
            Self::WaterSurface(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.description(),
            Self::Door(v) => v.description(),
            Self::DoorSurface(v) => v.description(),
            Self::FloorSurface(v) => v.description(),
            Self::GroundSurface(v) => v.description(),
            Self::InteriorWallSurface(v) => v.description(),
            Self::OtherConstruction(v) => v.description(),
            Self::OuterCeilingSurface(v) => v.description(),
            Self::OuterFloorSurface(v) => v.description(),
            Self::RoofSurface(v) => v.description(),
            Self::WallSurface(v) => v.description(),
            Self::Window(v) => v.description(),
            Self::WindowSurface(v) => v.description(),
            Self::Bridge(v) => v.description(),
            Self::BridgeConstructiveElement(v) => v.description(),
            Self::BridgeFurniture(v) => v.description(),
            Self::BridgeInstallation(v) => v.description(),
            Self::BridgePart(v) => v.description(),
            Self::BridgeRoom(v) => v.description(),
            Self::Building(v) => v.description(),
            Self::BuildingConstructiveElement(v) => v.description(),
            Self::BuildingFurniture(v) => v.description(),
            Self::BuildingInstallation(v) => v.description(),
            Self::BuildingPart(v) => v.description(),
            Self::BuildingRoom(v) => v.description(),
            Self::BuildingUnit(v) => v.description(),
            Self::Storey(v) => v.description(),
            Self::CityFurniture(v) => v.description(),
            Self::CityObjectGroup(v) => v.description(),
            Self::ClosureSurface(v) => v.description(),
            Self::GenericLogicalSpace(v) => v.description(),
            Self::GenericOccupiedSpace(v) => v.description(),
            Self::GenericThematicSurface(v) => v.description(),
            Self::GenericUnoccupiedSpace(v) => v.description(),
            Self::LandUse(v) => v.description(),
            Self::BreaklineRelief(v) => v.description(),
            Self::MassPointRelief(v) => v.description(),
            Self::RasterRelief(v) => v.description(),
            Self::ReliefFeature(v) => v.description(),
            Self::TINRelief(v) => v.description(),
            Self::AuxiliaryTrafficArea(v) => v.description(),
            Self::AuxiliaryTrafficSpace(v) => v.description(),
            Self::ClearanceSpace(v) => v.description(),
            Self::Hole(v) => v.description(),
            Self::HoleSurface(v) => v.description(),
            Self::Intersection(v) => v.description(),
            Self::Marking(v) => v.description(),
            Self::Railway(v) => v.description(),
            Self::Road(v) => v.description(),
            Self::Section(v) => v.description(),
            Self::Square(v) => v.description(),
            Self::Track(v) => v.description(),
            Self::TrafficArea(v) => v.description(),
            Self::TrafficSpace(v) => v.description(),
            Self::Waterway(v) => v.description(),
            Self::HollowSpace(v) => v.description(),
            Self::Tunnel(v) => v.description(),
            Self::TunnelConstructiveElement(v) => v.description(),
            Self::TunnelFurniture(v) => v.description(),
            Self::TunnelInstallation(v) => v.description(),
            Self::TunnelPart(v) => v.description(),
            Self::PlantCover(v) => v.description(),
            Self::SolitaryVegetationObject(v) => v.description(),
            Self::WaterBody(v) => v.description(),
            Self::WaterGroundSurface(v) => v.description(),
            Self::WaterSurface(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractCityObject {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.creation_date(),
            Self::Door(v) => v.creation_date(),
            Self::DoorSurface(v) => v.creation_date(),
            Self::FloorSurface(v) => v.creation_date(),
            Self::GroundSurface(v) => v.creation_date(),
            Self::InteriorWallSurface(v) => v.creation_date(),
            Self::OtherConstruction(v) => v.creation_date(),
            Self::OuterCeilingSurface(v) => v.creation_date(),
            Self::OuterFloorSurface(v) => v.creation_date(),
            Self::RoofSurface(v) => v.creation_date(),
            Self::WallSurface(v) => v.creation_date(),
            Self::Window(v) => v.creation_date(),
            Self::WindowSurface(v) => v.creation_date(),
            Self::Bridge(v) => v.creation_date(),
            Self::BridgeConstructiveElement(v) => v.creation_date(),
            Self::BridgeFurniture(v) => v.creation_date(),
            Self::BridgeInstallation(v) => v.creation_date(),
            Self::BridgePart(v) => v.creation_date(),
            Self::BridgeRoom(v) => v.creation_date(),
            Self::Building(v) => v.creation_date(),
            Self::BuildingConstructiveElement(v) => v.creation_date(),
            Self::BuildingFurniture(v) => v.creation_date(),
            Self::BuildingInstallation(v) => v.creation_date(),
            Self::BuildingPart(v) => v.creation_date(),
            Self::BuildingRoom(v) => v.creation_date(),
            Self::BuildingUnit(v) => v.creation_date(),
            Self::Storey(v) => v.creation_date(),
            Self::CityFurniture(v) => v.creation_date(),
            Self::CityObjectGroup(v) => v.creation_date(),
            Self::ClosureSurface(v) => v.creation_date(),
            Self::GenericLogicalSpace(v) => v.creation_date(),
            Self::GenericOccupiedSpace(v) => v.creation_date(),
            Self::GenericThematicSurface(v) => v.creation_date(),
            Self::GenericUnoccupiedSpace(v) => v.creation_date(),
            Self::LandUse(v) => v.creation_date(),
            Self::BreaklineRelief(v) => v.creation_date(),
            Self::MassPointRelief(v) => v.creation_date(),
            Self::RasterRelief(v) => v.creation_date(),
            Self::ReliefFeature(v) => v.creation_date(),
            Self::TINRelief(v) => v.creation_date(),
            Self::AuxiliaryTrafficArea(v) => v.creation_date(),
            Self::AuxiliaryTrafficSpace(v) => v.creation_date(),
            Self::ClearanceSpace(v) => v.creation_date(),
            Self::Hole(v) => v.creation_date(),
            Self::HoleSurface(v) => v.creation_date(),
            Self::Intersection(v) => v.creation_date(),
            Self::Marking(v) => v.creation_date(),
            Self::Railway(v) => v.creation_date(),
            Self::Road(v) => v.creation_date(),
            Self::Section(v) => v.creation_date(),
            Self::Square(v) => v.creation_date(),
            Self::Track(v) => v.creation_date(),
            Self::TrafficArea(v) => v.creation_date(),
            Self::TrafficSpace(v) => v.creation_date(),
            Self::Waterway(v) => v.creation_date(),
            Self::HollowSpace(v) => v.creation_date(),
            Self::Tunnel(v) => v.creation_date(),
            Self::TunnelConstructiveElement(v) => v.creation_date(),
            Self::TunnelFurniture(v) => v.creation_date(),
            Self::TunnelInstallation(v) => v.creation_date(),
            Self::TunnelPart(v) => v.creation_date(),
            Self::PlantCover(v) => v.creation_date(),
            Self::SolitaryVegetationObject(v) => v.creation_date(),
            Self::WaterBody(v) => v.creation_date(),
            Self::WaterGroundSurface(v) => v.creation_date(),
            Self::WaterSurface(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.termination_date(),
            Self::Door(v) => v.termination_date(),
            Self::DoorSurface(v) => v.termination_date(),
            Self::FloorSurface(v) => v.termination_date(),
            Self::GroundSurface(v) => v.termination_date(),
            Self::InteriorWallSurface(v) => v.termination_date(),
            Self::OtherConstruction(v) => v.termination_date(),
            Self::OuterCeilingSurface(v) => v.termination_date(),
            Self::OuterFloorSurface(v) => v.termination_date(),
            Self::RoofSurface(v) => v.termination_date(),
            Self::WallSurface(v) => v.termination_date(),
            Self::Window(v) => v.termination_date(),
            Self::WindowSurface(v) => v.termination_date(),
            Self::Bridge(v) => v.termination_date(),
            Self::BridgeConstructiveElement(v) => v.termination_date(),
            Self::BridgeFurniture(v) => v.termination_date(),
            Self::BridgeInstallation(v) => v.termination_date(),
            Self::BridgePart(v) => v.termination_date(),
            Self::BridgeRoom(v) => v.termination_date(),
            Self::Building(v) => v.termination_date(),
            Self::BuildingConstructiveElement(v) => v.termination_date(),
            Self::BuildingFurniture(v) => v.termination_date(),
            Self::BuildingInstallation(v) => v.termination_date(),
            Self::BuildingPart(v) => v.termination_date(),
            Self::BuildingRoom(v) => v.termination_date(),
            Self::BuildingUnit(v) => v.termination_date(),
            Self::Storey(v) => v.termination_date(),
            Self::CityFurniture(v) => v.termination_date(),
            Self::CityObjectGroup(v) => v.termination_date(),
            Self::ClosureSurface(v) => v.termination_date(),
            Self::GenericLogicalSpace(v) => v.termination_date(),
            Self::GenericOccupiedSpace(v) => v.termination_date(),
            Self::GenericThematicSurface(v) => v.termination_date(),
            Self::GenericUnoccupiedSpace(v) => v.termination_date(),
            Self::LandUse(v) => v.termination_date(),
            Self::BreaklineRelief(v) => v.termination_date(),
            Self::MassPointRelief(v) => v.termination_date(),
            Self::RasterRelief(v) => v.termination_date(),
            Self::ReliefFeature(v) => v.termination_date(),
            Self::TINRelief(v) => v.termination_date(),
            Self::AuxiliaryTrafficArea(v) => v.termination_date(),
            Self::AuxiliaryTrafficSpace(v) => v.termination_date(),
            Self::ClearanceSpace(v) => v.termination_date(),
            Self::Hole(v) => v.termination_date(),
            Self::HoleSurface(v) => v.termination_date(),
            Self::Intersection(v) => v.termination_date(),
            Self::Marking(v) => v.termination_date(),
            Self::Railway(v) => v.termination_date(),
            Self::Road(v) => v.termination_date(),
            Self::Section(v) => v.termination_date(),
            Self::Square(v) => v.termination_date(),
            Self::Track(v) => v.termination_date(),
            Self::TrafficArea(v) => v.termination_date(),
            Self::TrafficSpace(v) => v.termination_date(),
            Self::Waterway(v) => v.termination_date(),
            Self::HollowSpace(v) => v.termination_date(),
            Self::Tunnel(v) => v.termination_date(),
            Self::TunnelConstructiveElement(v) => v.termination_date(),
            Self::TunnelFurniture(v) => v.termination_date(),
            Self::TunnelInstallation(v) => v.termination_date(),
            Self::TunnelPart(v) => v.termination_date(),
            Self::PlantCover(v) => v.termination_date(),
            Self::SolitaryVegetationObject(v) => v.termination_date(),
            Self::WaterBody(v) => v.termination_date(),
            Self::WaterGroundSurface(v) => v.termination_date(),
            Self::WaterSurface(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.valid_from(),
            Self::Door(v) => v.valid_from(),
            Self::DoorSurface(v) => v.valid_from(),
            Self::FloorSurface(v) => v.valid_from(),
            Self::GroundSurface(v) => v.valid_from(),
            Self::InteriorWallSurface(v) => v.valid_from(),
            Self::OtherConstruction(v) => v.valid_from(),
            Self::OuterCeilingSurface(v) => v.valid_from(),
            Self::OuterFloorSurface(v) => v.valid_from(),
            Self::RoofSurface(v) => v.valid_from(),
            Self::WallSurface(v) => v.valid_from(),
            Self::Window(v) => v.valid_from(),
            Self::WindowSurface(v) => v.valid_from(),
            Self::Bridge(v) => v.valid_from(),
            Self::BridgeConstructiveElement(v) => v.valid_from(),
            Self::BridgeFurniture(v) => v.valid_from(),
            Self::BridgeInstallation(v) => v.valid_from(),
            Self::BridgePart(v) => v.valid_from(),
            Self::BridgeRoom(v) => v.valid_from(),
            Self::Building(v) => v.valid_from(),
            Self::BuildingConstructiveElement(v) => v.valid_from(),
            Self::BuildingFurniture(v) => v.valid_from(),
            Self::BuildingInstallation(v) => v.valid_from(),
            Self::BuildingPart(v) => v.valid_from(),
            Self::BuildingRoom(v) => v.valid_from(),
            Self::BuildingUnit(v) => v.valid_from(),
            Self::Storey(v) => v.valid_from(),
            Self::CityFurniture(v) => v.valid_from(),
            Self::CityObjectGroup(v) => v.valid_from(),
            Self::ClosureSurface(v) => v.valid_from(),
            Self::GenericLogicalSpace(v) => v.valid_from(),
            Self::GenericOccupiedSpace(v) => v.valid_from(),
            Self::GenericThematicSurface(v) => v.valid_from(),
            Self::GenericUnoccupiedSpace(v) => v.valid_from(),
            Self::LandUse(v) => v.valid_from(),
            Self::BreaklineRelief(v) => v.valid_from(),
            Self::MassPointRelief(v) => v.valid_from(),
            Self::RasterRelief(v) => v.valid_from(),
            Self::ReliefFeature(v) => v.valid_from(),
            Self::TINRelief(v) => v.valid_from(),
            Self::AuxiliaryTrafficArea(v) => v.valid_from(),
            Self::AuxiliaryTrafficSpace(v) => v.valid_from(),
            Self::ClearanceSpace(v) => v.valid_from(),
            Self::Hole(v) => v.valid_from(),
            Self::HoleSurface(v) => v.valid_from(),
            Self::Intersection(v) => v.valid_from(),
            Self::Marking(v) => v.valid_from(),
            Self::Railway(v) => v.valid_from(),
            Self::Road(v) => v.valid_from(),
            Self::Section(v) => v.valid_from(),
            Self::Square(v) => v.valid_from(),
            Self::Track(v) => v.valid_from(),
            Self::TrafficArea(v) => v.valid_from(),
            Self::TrafficSpace(v) => v.valid_from(),
            Self::Waterway(v) => v.valid_from(),
            Self::HollowSpace(v) => v.valid_from(),
            Self::Tunnel(v) => v.valid_from(),
            Self::TunnelConstructiveElement(v) => v.valid_from(),
            Self::TunnelFurniture(v) => v.valid_from(),
            Self::TunnelInstallation(v) => v.valid_from(),
            Self::TunnelPart(v) => v.valid_from(),
            Self::PlantCover(v) => v.valid_from(),
            Self::SolitaryVegetationObject(v) => v.valid_from(),
            Self::WaterBody(v) => v.valid_from(),
            Self::WaterGroundSurface(v) => v.valid_from(),
            Self::WaterSurface(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.valid_to(),
            Self::Door(v) => v.valid_to(),
            Self::DoorSurface(v) => v.valid_to(),
            Self::FloorSurface(v) => v.valid_to(),
            Self::GroundSurface(v) => v.valid_to(),
            Self::InteriorWallSurface(v) => v.valid_to(),
            Self::OtherConstruction(v) => v.valid_to(),
            Self::OuterCeilingSurface(v) => v.valid_to(),
            Self::OuterFloorSurface(v) => v.valid_to(),
            Self::RoofSurface(v) => v.valid_to(),
            Self::WallSurface(v) => v.valid_to(),
            Self::Window(v) => v.valid_to(),
            Self::WindowSurface(v) => v.valid_to(),
            Self::Bridge(v) => v.valid_to(),
            Self::BridgeConstructiveElement(v) => v.valid_to(),
            Self::BridgeFurniture(v) => v.valid_to(),
            Self::BridgeInstallation(v) => v.valid_to(),
            Self::BridgePart(v) => v.valid_to(),
            Self::BridgeRoom(v) => v.valid_to(),
            Self::Building(v) => v.valid_to(),
            Self::BuildingConstructiveElement(v) => v.valid_to(),
            Self::BuildingFurniture(v) => v.valid_to(),
            Self::BuildingInstallation(v) => v.valid_to(),
            Self::BuildingPart(v) => v.valid_to(),
            Self::BuildingRoom(v) => v.valid_to(),
            Self::BuildingUnit(v) => v.valid_to(),
            Self::Storey(v) => v.valid_to(),
            Self::CityFurniture(v) => v.valid_to(),
            Self::CityObjectGroup(v) => v.valid_to(),
            Self::ClosureSurface(v) => v.valid_to(),
            Self::GenericLogicalSpace(v) => v.valid_to(),
            Self::GenericOccupiedSpace(v) => v.valid_to(),
            Self::GenericThematicSurface(v) => v.valid_to(),
            Self::GenericUnoccupiedSpace(v) => v.valid_to(),
            Self::LandUse(v) => v.valid_to(),
            Self::BreaklineRelief(v) => v.valid_to(),
            Self::MassPointRelief(v) => v.valid_to(),
            Self::RasterRelief(v) => v.valid_to(),
            Self::ReliefFeature(v) => v.valid_to(),
            Self::TINRelief(v) => v.valid_to(),
            Self::AuxiliaryTrafficArea(v) => v.valid_to(),
            Self::AuxiliaryTrafficSpace(v) => v.valid_to(),
            Self::ClearanceSpace(v) => v.valid_to(),
            Self::Hole(v) => v.valid_to(),
            Self::HoleSurface(v) => v.valid_to(),
            Self::Intersection(v) => v.valid_to(),
            Self::Marking(v) => v.valid_to(),
            Self::Railway(v) => v.valid_to(),
            Self::Road(v) => v.valid_to(),
            Self::Section(v) => v.valid_to(),
            Self::Square(v) => v.valid_to(),
            Self::Track(v) => v.valid_to(),
            Self::TrafficArea(v) => v.valid_to(),
            Self::TrafficSpace(v) => v.valid_to(),
            Self::Waterway(v) => v.valid_to(),
            Self::HollowSpace(v) => v.valid_to(),
            Self::Tunnel(v) => v.valid_to(),
            Self::TunnelConstructiveElement(v) => v.valid_to(),
            Self::TunnelFurniture(v) => v.valid_to(),
            Self::TunnelInstallation(v) => v.valid_to(),
            Self::TunnelPart(v) => v.valid_to(),
            Self::PlantCover(v) => v.valid_to(),
            Self::SolitaryVegetationObject(v) => v.valid_to(),
            Self::WaterBody(v) => v.valid_to(),
            Self::WaterGroundSurface(v) => v.valid_to(),
            Self::WaterSurface(v) => v.valid_to(),
        }
    }
}
impl AbstractCityObjectTrait for AbstractCityObject {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::CeilingSurface(v) => v.relative_to_terrain(),
            Self::Door(v) => v.relative_to_terrain(),
            Self::DoorSurface(v) => v.relative_to_terrain(),
            Self::FloorSurface(v) => v.relative_to_terrain(),
            Self::GroundSurface(v) => v.relative_to_terrain(),
            Self::InteriorWallSurface(v) => v.relative_to_terrain(),
            Self::OtherConstruction(v) => v.relative_to_terrain(),
            Self::OuterCeilingSurface(v) => v.relative_to_terrain(),
            Self::OuterFloorSurface(v) => v.relative_to_terrain(),
            Self::RoofSurface(v) => v.relative_to_terrain(),
            Self::WallSurface(v) => v.relative_to_terrain(),
            Self::Window(v) => v.relative_to_terrain(),
            Self::WindowSurface(v) => v.relative_to_terrain(),
            Self::Bridge(v) => v.relative_to_terrain(),
            Self::BridgeConstructiveElement(v) => v.relative_to_terrain(),
            Self::BridgeFurniture(v) => v.relative_to_terrain(),
            Self::BridgeInstallation(v) => v.relative_to_terrain(),
            Self::BridgePart(v) => v.relative_to_terrain(),
            Self::BridgeRoom(v) => v.relative_to_terrain(),
            Self::Building(v) => v.relative_to_terrain(),
            Self::BuildingConstructiveElement(v) => v.relative_to_terrain(),
            Self::BuildingFurniture(v) => v.relative_to_terrain(),
            Self::BuildingInstallation(v) => v.relative_to_terrain(),
            Self::BuildingPart(v) => v.relative_to_terrain(),
            Self::BuildingRoom(v) => v.relative_to_terrain(),
            Self::BuildingUnit(v) => v.relative_to_terrain(),
            Self::Storey(v) => v.relative_to_terrain(),
            Self::CityFurniture(v) => v.relative_to_terrain(),
            Self::CityObjectGroup(v) => v.relative_to_terrain(),
            Self::ClosureSurface(v) => v.relative_to_terrain(),
            Self::GenericLogicalSpace(v) => v.relative_to_terrain(),
            Self::GenericOccupiedSpace(v) => v.relative_to_terrain(),
            Self::GenericThematicSurface(v) => v.relative_to_terrain(),
            Self::GenericUnoccupiedSpace(v) => v.relative_to_terrain(),
            Self::LandUse(v) => v.relative_to_terrain(),
            Self::BreaklineRelief(v) => v.relative_to_terrain(),
            Self::MassPointRelief(v) => v.relative_to_terrain(),
            Self::RasterRelief(v) => v.relative_to_terrain(),
            Self::ReliefFeature(v) => v.relative_to_terrain(),
            Self::TINRelief(v) => v.relative_to_terrain(),
            Self::AuxiliaryTrafficArea(v) => v.relative_to_terrain(),
            Self::AuxiliaryTrafficSpace(v) => v.relative_to_terrain(),
            Self::ClearanceSpace(v) => v.relative_to_terrain(),
            Self::Hole(v) => v.relative_to_terrain(),
            Self::HoleSurface(v) => v.relative_to_terrain(),
            Self::Intersection(v) => v.relative_to_terrain(),
            Self::Marking(v) => v.relative_to_terrain(),
            Self::Railway(v) => v.relative_to_terrain(),
            Self::Road(v) => v.relative_to_terrain(),
            Self::Section(v) => v.relative_to_terrain(),
            Self::Square(v) => v.relative_to_terrain(),
            Self::Track(v) => v.relative_to_terrain(),
            Self::TrafficArea(v) => v.relative_to_terrain(),
            Self::TrafficSpace(v) => v.relative_to_terrain(),
            Self::Waterway(v) => v.relative_to_terrain(),
            Self::HollowSpace(v) => v.relative_to_terrain(),
            Self::Tunnel(v) => v.relative_to_terrain(),
            Self::TunnelConstructiveElement(v) => v.relative_to_terrain(),
            Self::TunnelFurniture(v) => v.relative_to_terrain(),
            Self::TunnelInstallation(v) => v.relative_to_terrain(),
            Self::TunnelPart(v) => v.relative_to_terrain(),
            Self::PlantCover(v) => v.relative_to_terrain(),
            Self::SolitaryVegetationObject(v) => v.relative_to_terrain(),
            Self::WaterBody(v) => v.relative_to_terrain(),
            Self::WaterGroundSurface(v) => v.relative_to_terrain(),
            Self::WaterSurface(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::CeilingSurface(v) => v.relative_to_water(),
            Self::Door(v) => v.relative_to_water(),
            Self::DoorSurface(v) => v.relative_to_water(),
            Self::FloorSurface(v) => v.relative_to_water(),
            Self::GroundSurface(v) => v.relative_to_water(),
            Self::InteriorWallSurface(v) => v.relative_to_water(),
            Self::OtherConstruction(v) => v.relative_to_water(),
            Self::OuterCeilingSurface(v) => v.relative_to_water(),
            Self::OuterFloorSurface(v) => v.relative_to_water(),
            Self::RoofSurface(v) => v.relative_to_water(),
            Self::WallSurface(v) => v.relative_to_water(),
            Self::Window(v) => v.relative_to_water(),
            Self::WindowSurface(v) => v.relative_to_water(),
            Self::Bridge(v) => v.relative_to_water(),
            Self::BridgeConstructiveElement(v) => v.relative_to_water(),
            Self::BridgeFurniture(v) => v.relative_to_water(),
            Self::BridgeInstallation(v) => v.relative_to_water(),
            Self::BridgePart(v) => v.relative_to_water(),
            Self::BridgeRoom(v) => v.relative_to_water(),
            Self::Building(v) => v.relative_to_water(),
            Self::BuildingConstructiveElement(v) => v.relative_to_water(),
            Self::BuildingFurniture(v) => v.relative_to_water(),
            Self::BuildingInstallation(v) => v.relative_to_water(),
            Self::BuildingPart(v) => v.relative_to_water(),
            Self::BuildingRoom(v) => v.relative_to_water(),
            Self::BuildingUnit(v) => v.relative_to_water(),
            Self::Storey(v) => v.relative_to_water(),
            Self::CityFurniture(v) => v.relative_to_water(),
            Self::CityObjectGroup(v) => v.relative_to_water(),
            Self::ClosureSurface(v) => v.relative_to_water(),
            Self::GenericLogicalSpace(v) => v.relative_to_water(),
            Self::GenericOccupiedSpace(v) => v.relative_to_water(),
            Self::GenericThematicSurface(v) => v.relative_to_water(),
            Self::GenericUnoccupiedSpace(v) => v.relative_to_water(),
            Self::LandUse(v) => v.relative_to_water(),
            Self::BreaklineRelief(v) => v.relative_to_water(),
            Self::MassPointRelief(v) => v.relative_to_water(),
            Self::RasterRelief(v) => v.relative_to_water(),
            Self::ReliefFeature(v) => v.relative_to_water(),
            Self::TINRelief(v) => v.relative_to_water(),
            Self::AuxiliaryTrafficArea(v) => v.relative_to_water(),
            Self::AuxiliaryTrafficSpace(v) => v.relative_to_water(),
            Self::ClearanceSpace(v) => v.relative_to_water(),
            Self::Hole(v) => v.relative_to_water(),
            Self::HoleSurface(v) => v.relative_to_water(),
            Self::Intersection(v) => v.relative_to_water(),
            Self::Marking(v) => v.relative_to_water(),
            Self::Railway(v) => v.relative_to_water(),
            Self::Road(v) => v.relative_to_water(),
            Self::Section(v) => v.relative_to_water(),
            Self::Square(v) => v.relative_to_water(),
            Self::Track(v) => v.relative_to_water(),
            Self::TrafficArea(v) => v.relative_to_water(),
            Self::TrafficSpace(v) => v.relative_to_water(),
            Self::Waterway(v) => v.relative_to_water(),
            Self::HollowSpace(v) => v.relative_to_water(),
            Self::Tunnel(v) => v.relative_to_water(),
            Self::TunnelConstructiveElement(v) => v.relative_to_water(),
            Self::TunnelFurniture(v) => v.relative_to_water(),
            Self::TunnelInstallation(v) => v.relative_to_water(),
            Self::TunnelPart(v) => v.relative_to_water(),
            Self::PlantCover(v) => v.relative_to_water(),
            Self::SolitaryVegetationObject(v) => v.relative_to_water(),
            Self::WaterBody(v) => v.relative_to_water(),
            Self::WaterGroundSurface(v) => v.relative_to_water(),
            Self::WaterSurface(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::CeilingSurface(v) => v.appearance(),
            Self::Door(v) => v.appearance(),
            Self::DoorSurface(v) => v.appearance(),
            Self::FloorSurface(v) => v.appearance(),
            Self::GroundSurface(v) => v.appearance(),
            Self::InteriorWallSurface(v) => v.appearance(),
            Self::OtherConstruction(v) => v.appearance(),
            Self::OuterCeilingSurface(v) => v.appearance(),
            Self::OuterFloorSurface(v) => v.appearance(),
            Self::RoofSurface(v) => v.appearance(),
            Self::WallSurface(v) => v.appearance(),
            Self::Window(v) => v.appearance(),
            Self::WindowSurface(v) => v.appearance(),
            Self::Bridge(v) => v.appearance(),
            Self::BridgeConstructiveElement(v) => v.appearance(),
            Self::BridgeFurniture(v) => v.appearance(),
            Self::BridgeInstallation(v) => v.appearance(),
            Self::BridgePart(v) => v.appearance(),
            Self::BridgeRoom(v) => v.appearance(),
            Self::Building(v) => v.appearance(),
            Self::BuildingConstructiveElement(v) => v.appearance(),
            Self::BuildingFurniture(v) => v.appearance(),
            Self::BuildingInstallation(v) => v.appearance(),
            Self::BuildingPart(v) => v.appearance(),
            Self::BuildingRoom(v) => v.appearance(),
            Self::BuildingUnit(v) => v.appearance(),
            Self::Storey(v) => v.appearance(),
            Self::CityFurniture(v) => v.appearance(),
            Self::CityObjectGroup(v) => v.appearance(),
            Self::ClosureSurface(v) => v.appearance(),
            Self::GenericLogicalSpace(v) => v.appearance(),
            Self::GenericOccupiedSpace(v) => v.appearance(),
            Self::GenericThematicSurface(v) => v.appearance(),
            Self::GenericUnoccupiedSpace(v) => v.appearance(),
            Self::LandUse(v) => v.appearance(),
            Self::BreaklineRelief(v) => v.appearance(),
            Self::MassPointRelief(v) => v.appearance(),
            Self::RasterRelief(v) => v.appearance(),
            Self::ReliefFeature(v) => v.appearance(),
            Self::TINRelief(v) => v.appearance(),
            Self::AuxiliaryTrafficArea(v) => v.appearance(),
            Self::AuxiliaryTrafficSpace(v) => v.appearance(),
            Self::ClearanceSpace(v) => v.appearance(),
            Self::Hole(v) => v.appearance(),
            Self::HoleSurface(v) => v.appearance(),
            Self::Intersection(v) => v.appearance(),
            Self::Marking(v) => v.appearance(),
            Self::Railway(v) => v.appearance(),
            Self::Road(v) => v.appearance(),
            Self::Section(v) => v.appearance(),
            Self::Square(v) => v.appearance(),
            Self::Track(v) => v.appearance(),
            Self::TrafficArea(v) => v.appearance(),
            Self::TrafficSpace(v) => v.appearance(),
            Self::Waterway(v) => v.appearance(),
            Self::HollowSpace(v) => v.appearance(),
            Self::Tunnel(v) => v.appearance(),
            Self::TunnelConstructiveElement(v) => v.appearance(),
            Self::TunnelFurniture(v) => v.appearance(),
            Self::TunnelInstallation(v) => v.appearance(),
            Self::TunnelPart(v) => v.appearance(),
            Self::PlantCover(v) => v.appearance(),
            Self::SolitaryVegetationObject(v) => v.appearance(),
            Self::WaterBody(v) => v.appearance(),
            Self::WaterGroundSurface(v) => v.appearance(),
            Self::WaterSurface(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::CeilingSurface(v) => v.generalizes_to(),
            Self::Door(v) => v.generalizes_to(),
            Self::DoorSurface(v) => v.generalizes_to(),
            Self::FloorSurface(v) => v.generalizes_to(),
            Self::GroundSurface(v) => v.generalizes_to(),
            Self::InteriorWallSurface(v) => v.generalizes_to(),
            Self::OtherConstruction(v) => v.generalizes_to(),
            Self::OuterCeilingSurface(v) => v.generalizes_to(),
            Self::OuterFloorSurface(v) => v.generalizes_to(),
            Self::RoofSurface(v) => v.generalizes_to(),
            Self::WallSurface(v) => v.generalizes_to(),
            Self::Window(v) => v.generalizes_to(),
            Self::WindowSurface(v) => v.generalizes_to(),
            Self::Bridge(v) => v.generalizes_to(),
            Self::BridgeConstructiveElement(v) => v.generalizes_to(),
            Self::BridgeFurniture(v) => v.generalizes_to(),
            Self::BridgeInstallation(v) => v.generalizes_to(),
            Self::BridgePart(v) => v.generalizes_to(),
            Self::BridgeRoom(v) => v.generalizes_to(),
            Self::Building(v) => v.generalizes_to(),
            Self::BuildingConstructiveElement(v) => v.generalizes_to(),
            Self::BuildingFurniture(v) => v.generalizes_to(),
            Self::BuildingInstallation(v) => v.generalizes_to(),
            Self::BuildingPart(v) => v.generalizes_to(),
            Self::BuildingRoom(v) => v.generalizes_to(),
            Self::BuildingUnit(v) => v.generalizes_to(),
            Self::Storey(v) => v.generalizes_to(),
            Self::CityFurniture(v) => v.generalizes_to(),
            Self::CityObjectGroup(v) => v.generalizes_to(),
            Self::ClosureSurface(v) => v.generalizes_to(),
            Self::GenericLogicalSpace(v) => v.generalizes_to(),
            Self::GenericOccupiedSpace(v) => v.generalizes_to(),
            Self::GenericThematicSurface(v) => v.generalizes_to(),
            Self::GenericUnoccupiedSpace(v) => v.generalizes_to(),
            Self::LandUse(v) => v.generalizes_to(),
            Self::BreaklineRelief(v) => v.generalizes_to(),
            Self::MassPointRelief(v) => v.generalizes_to(),
            Self::RasterRelief(v) => v.generalizes_to(),
            Self::ReliefFeature(v) => v.generalizes_to(),
            Self::TINRelief(v) => v.generalizes_to(),
            Self::AuxiliaryTrafficArea(v) => v.generalizes_to(),
            Self::AuxiliaryTrafficSpace(v) => v.generalizes_to(),
            Self::ClearanceSpace(v) => v.generalizes_to(),
            Self::Hole(v) => v.generalizes_to(),
            Self::HoleSurface(v) => v.generalizes_to(),
            Self::Intersection(v) => v.generalizes_to(),
            Self::Marking(v) => v.generalizes_to(),
            Self::Railway(v) => v.generalizes_to(),
            Self::Road(v) => v.generalizes_to(),
            Self::Section(v) => v.generalizes_to(),
            Self::Square(v) => v.generalizes_to(),
            Self::Track(v) => v.generalizes_to(),
            Self::TrafficArea(v) => v.generalizes_to(),
            Self::TrafficSpace(v) => v.generalizes_to(),
            Self::Waterway(v) => v.generalizes_to(),
            Self::HollowSpace(v) => v.generalizes_to(),
            Self::Tunnel(v) => v.generalizes_to(),
            Self::TunnelConstructiveElement(v) => v.generalizes_to(),
            Self::TunnelFurniture(v) => v.generalizes_to(),
            Self::TunnelInstallation(v) => v.generalizes_to(),
            Self::TunnelPart(v) => v.generalizes_to(),
            Self::PlantCover(v) => v.generalizes_to(),
            Self::SolitaryVegetationObject(v) => v.generalizes_to(),
            Self::WaterBody(v) => v.generalizes_to(),
            Self::WaterGroundSurface(v) => v.generalizes_to(),
            Self::WaterSurface(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::CeilingSurface(v) => v.external_reference(),
            Self::Door(v) => v.external_reference(),
            Self::DoorSurface(v) => v.external_reference(),
            Self::FloorSurface(v) => v.external_reference(),
            Self::GroundSurface(v) => v.external_reference(),
            Self::InteriorWallSurface(v) => v.external_reference(),
            Self::OtherConstruction(v) => v.external_reference(),
            Self::OuterCeilingSurface(v) => v.external_reference(),
            Self::OuterFloorSurface(v) => v.external_reference(),
            Self::RoofSurface(v) => v.external_reference(),
            Self::WallSurface(v) => v.external_reference(),
            Self::Window(v) => v.external_reference(),
            Self::WindowSurface(v) => v.external_reference(),
            Self::Bridge(v) => v.external_reference(),
            Self::BridgeConstructiveElement(v) => v.external_reference(),
            Self::BridgeFurniture(v) => v.external_reference(),
            Self::BridgeInstallation(v) => v.external_reference(),
            Self::BridgePart(v) => v.external_reference(),
            Self::BridgeRoom(v) => v.external_reference(),
            Self::Building(v) => v.external_reference(),
            Self::BuildingConstructiveElement(v) => v.external_reference(),
            Self::BuildingFurniture(v) => v.external_reference(),
            Self::BuildingInstallation(v) => v.external_reference(),
            Self::BuildingPart(v) => v.external_reference(),
            Self::BuildingRoom(v) => v.external_reference(),
            Self::BuildingUnit(v) => v.external_reference(),
            Self::Storey(v) => v.external_reference(),
            Self::CityFurniture(v) => v.external_reference(),
            Self::CityObjectGroup(v) => v.external_reference(),
            Self::ClosureSurface(v) => v.external_reference(),
            Self::GenericLogicalSpace(v) => v.external_reference(),
            Self::GenericOccupiedSpace(v) => v.external_reference(),
            Self::GenericThematicSurface(v) => v.external_reference(),
            Self::GenericUnoccupiedSpace(v) => v.external_reference(),
            Self::LandUse(v) => v.external_reference(),
            Self::BreaklineRelief(v) => v.external_reference(),
            Self::MassPointRelief(v) => v.external_reference(),
            Self::RasterRelief(v) => v.external_reference(),
            Self::ReliefFeature(v) => v.external_reference(),
            Self::TINRelief(v) => v.external_reference(),
            Self::AuxiliaryTrafficArea(v) => v.external_reference(),
            Self::AuxiliaryTrafficSpace(v) => v.external_reference(),
            Self::ClearanceSpace(v) => v.external_reference(),
            Self::Hole(v) => v.external_reference(),
            Self::HoleSurface(v) => v.external_reference(),
            Self::Intersection(v) => v.external_reference(),
            Self::Marking(v) => v.external_reference(),
            Self::Railway(v) => v.external_reference(),
            Self::Road(v) => v.external_reference(),
            Self::Section(v) => v.external_reference(),
            Self::Square(v) => v.external_reference(),
            Self::Track(v) => v.external_reference(),
            Self::TrafficArea(v) => v.external_reference(),
            Self::TrafficSpace(v) => v.external_reference(),
            Self::Waterway(v) => v.external_reference(),
            Self::HollowSpace(v) => v.external_reference(),
            Self::Tunnel(v) => v.external_reference(),
            Self::TunnelConstructiveElement(v) => v.external_reference(),
            Self::TunnelFurniture(v) => v.external_reference(),
            Self::TunnelInstallation(v) => v.external_reference(),
            Self::TunnelPart(v) => v.external_reference(),
            Self::PlantCover(v) => v.external_reference(),
            Self::SolitaryVegetationObject(v) => v.external_reference(),
            Self::WaterBody(v) => v.external_reference(),
            Self::WaterGroundSurface(v) => v.external_reference(),
            Self::WaterSurface(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::CeilingSurface(v) => v.related_to(),
            Self::Door(v) => v.related_to(),
            Self::DoorSurface(v) => v.related_to(),
            Self::FloorSurface(v) => v.related_to(),
            Self::GroundSurface(v) => v.related_to(),
            Self::InteriorWallSurface(v) => v.related_to(),
            Self::OtherConstruction(v) => v.related_to(),
            Self::OuterCeilingSurface(v) => v.related_to(),
            Self::OuterFloorSurface(v) => v.related_to(),
            Self::RoofSurface(v) => v.related_to(),
            Self::WallSurface(v) => v.related_to(),
            Self::Window(v) => v.related_to(),
            Self::WindowSurface(v) => v.related_to(),
            Self::Bridge(v) => v.related_to(),
            Self::BridgeConstructiveElement(v) => v.related_to(),
            Self::BridgeFurniture(v) => v.related_to(),
            Self::BridgeInstallation(v) => v.related_to(),
            Self::BridgePart(v) => v.related_to(),
            Self::BridgeRoom(v) => v.related_to(),
            Self::Building(v) => v.related_to(),
            Self::BuildingConstructiveElement(v) => v.related_to(),
            Self::BuildingFurniture(v) => v.related_to(),
            Self::BuildingInstallation(v) => v.related_to(),
            Self::BuildingPart(v) => v.related_to(),
            Self::BuildingRoom(v) => v.related_to(),
            Self::BuildingUnit(v) => v.related_to(),
            Self::Storey(v) => v.related_to(),
            Self::CityFurniture(v) => v.related_to(),
            Self::CityObjectGroup(v) => v.related_to(),
            Self::ClosureSurface(v) => v.related_to(),
            Self::GenericLogicalSpace(v) => v.related_to(),
            Self::GenericOccupiedSpace(v) => v.related_to(),
            Self::GenericThematicSurface(v) => v.related_to(),
            Self::GenericUnoccupiedSpace(v) => v.related_to(),
            Self::LandUse(v) => v.related_to(),
            Self::BreaklineRelief(v) => v.related_to(),
            Self::MassPointRelief(v) => v.related_to(),
            Self::RasterRelief(v) => v.related_to(),
            Self::ReliefFeature(v) => v.related_to(),
            Self::TINRelief(v) => v.related_to(),
            Self::AuxiliaryTrafficArea(v) => v.related_to(),
            Self::AuxiliaryTrafficSpace(v) => v.related_to(),
            Self::ClearanceSpace(v) => v.related_to(),
            Self::Hole(v) => v.related_to(),
            Self::HoleSurface(v) => v.related_to(),
            Self::Intersection(v) => v.related_to(),
            Self::Marking(v) => v.related_to(),
            Self::Railway(v) => v.related_to(),
            Self::Road(v) => v.related_to(),
            Self::Section(v) => v.related_to(),
            Self::Square(v) => v.related_to(),
            Self::Track(v) => v.related_to(),
            Self::TrafficArea(v) => v.related_to(),
            Self::TrafficSpace(v) => v.related_to(),
            Self::Waterway(v) => v.related_to(),
            Self::HollowSpace(v) => v.related_to(),
            Self::Tunnel(v) => v.related_to(),
            Self::TunnelConstructiveElement(v) => v.related_to(),
            Self::TunnelFurniture(v) => v.related_to(),
            Self::TunnelInstallation(v) => v.related_to(),
            Self::TunnelPart(v) => v.related_to(),
            Self::PlantCover(v) => v.related_to(),
            Self::SolitaryVegetationObject(v) => v.related_to(),
            Self::WaterBody(v) => v.related_to(),
            Self::WaterGroundSurface(v) => v.related_to(),
            Self::WaterSurface(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::CeilingSurface(v) => v.dynamizer(),
            Self::Door(v) => v.dynamizer(),
            Self::DoorSurface(v) => v.dynamizer(),
            Self::FloorSurface(v) => v.dynamizer(),
            Self::GroundSurface(v) => v.dynamizer(),
            Self::InteriorWallSurface(v) => v.dynamizer(),
            Self::OtherConstruction(v) => v.dynamizer(),
            Self::OuterCeilingSurface(v) => v.dynamizer(),
            Self::OuterFloorSurface(v) => v.dynamizer(),
            Self::RoofSurface(v) => v.dynamizer(),
            Self::WallSurface(v) => v.dynamizer(),
            Self::Window(v) => v.dynamizer(),
            Self::WindowSurface(v) => v.dynamizer(),
            Self::Bridge(v) => v.dynamizer(),
            Self::BridgeConstructiveElement(v) => v.dynamizer(),
            Self::BridgeFurniture(v) => v.dynamizer(),
            Self::BridgeInstallation(v) => v.dynamizer(),
            Self::BridgePart(v) => v.dynamizer(),
            Self::BridgeRoom(v) => v.dynamizer(),
            Self::Building(v) => v.dynamizer(),
            Self::BuildingConstructiveElement(v) => v.dynamizer(),
            Self::BuildingFurniture(v) => v.dynamizer(),
            Self::BuildingInstallation(v) => v.dynamizer(),
            Self::BuildingPart(v) => v.dynamizer(),
            Self::BuildingRoom(v) => v.dynamizer(),
            Self::BuildingUnit(v) => v.dynamizer(),
            Self::Storey(v) => v.dynamizer(),
            Self::CityFurniture(v) => v.dynamizer(),
            Self::CityObjectGroup(v) => v.dynamizer(),
            Self::ClosureSurface(v) => v.dynamizer(),
            Self::GenericLogicalSpace(v) => v.dynamizer(),
            Self::GenericOccupiedSpace(v) => v.dynamizer(),
            Self::GenericThematicSurface(v) => v.dynamizer(),
            Self::GenericUnoccupiedSpace(v) => v.dynamizer(),
            Self::LandUse(v) => v.dynamizer(),
            Self::BreaklineRelief(v) => v.dynamizer(),
            Self::MassPointRelief(v) => v.dynamizer(),
            Self::RasterRelief(v) => v.dynamizer(),
            Self::ReliefFeature(v) => v.dynamizer(),
            Self::TINRelief(v) => v.dynamizer(),
            Self::AuxiliaryTrafficArea(v) => v.dynamizer(),
            Self::AuxiliaryTrafficSpace(v) => v.dynamizer(),
            Self::ClearanceSpace(v) => v.dynamizer(),
            Self::Hole(v) => v.dynamizer(),
            Self::HoleSurface(v) => v.dynamizer(),
            Self::Intersection(v) => v.dynamizer(),
            Self::Marking(v) => v.dynamizer(),
            Self::Railway(v) => v.dynamizer(),
            Self::Road(v) => v.dynamizer(),
            Self::Section(v) => v.dynamizer(),
            Self::Square(v) => v.dynamizer(),
            Self::Track(v) => v.dynamizer(),
            Self::TrafficArea(v) => v.dynamizer(),
            Self::TrafficSpace(v) => v.dynamizer(),
            Self::Waterway(v) => v.dynamizer(),
            Self::HollowSpace(v) => v.dynamizer(),
            Self::Tunnel(v) => v.dynamizer(),
            Self::TunnelConstructiveElement(v) => v.dynamizer(),
            Self::TunnelFurniture(v) => v.dynamizer(),
            Self::TunnelInstallation(v) => v.dynamizer(),
            Self::TunnelPart(v) => v.dynamizer(),
            Self::PlantCover(v) => v.dynamizer(),
            Self::SolitaryVegetationObject(v) => v.dynamizer(),
            Self::WaterBody(v) => v.dynamizer(),
            Self::WaterGroundSurface(v) => v.dynamizer(),
            Self::WaterSurface(v) => v.dynamizer(),
        }
    }
}
impl From<CeilingSurface> for AbstractCityObject {
    fn from(v: CeilingSurface) -> Self {
        Self::CeilingSurface(Box::new(v))
    }
}
impl From<Door> for AbstractCityObject {
    fn from(v: Door) -> Self {
        Self::Door(Box::new(v))
    }
}
impl From<DoorSurface> for AbstractCityObject {
    fn from(v: DoorSurface) -> Self {
        Self::DoorSurface(Box::new(v))
    }
}
impl From<FloorSurface> for AbstractCityObject {
    fn from(v: FloorSurface) -> Self {
        Self::FloorSurface(Box::new(v))
    }
}
impl From<GroundSurface> for AbstractCityObject {
    fn from(v: GroundSurface) -> Self {
        Self::GroundSurface(Box::new(v))
    }
}
impl From<InteriorWallSurface> for AbstractCityObject {
    fn from(v: InteriorWallSurface) -> Self {
        Self::InteriorWallSurface(Box::new(v))
    }
}
impl From<OtherConstruction> for AbstractCityObject {
    fn from(v: OtherConstruction) -> Self {
        Self::OtherConstruction(Box::new(v))
    }
}
impl From<OuterCeilingSurface> for AbstractCityObject {
    fn from(v: OuterCeilingSurface) -> Self {
        Self::OuterCeilingSurface(Box::new(v))
    }
}
impl From<OuterFloorSurface> for AbstractCityObject {
    fn from(v: OuterFloorSurface) -> Self {
        Self::OuterFloorSurface(Box::new(v))
    }
}
impl From<RoofSurface> for AbstractCityObject {
    fn from(v: RoofSurface) -> Self {
        Self::RoofSurface(Box::new(v))
    }
}
impl From<WallSurface> for AbstractCityObject {
    fn from(v: WallSurface) -> Self {
        Self::WallSurface(Box::new(v))
    }
}
impl From<Window> for AbstractCityObject {
    fn from(v: Window) -> Self {
        Self::Window(Box::new(v))
    }
}
impl From<WindowSurface> for AbstractCityObject {
    fn from(v: WindowSurface) -> Self {
        Self::WindowSurface(Box::new(v))
    }
}
impl From<Bridge> for AbstractCityObject {
    fn from(v: Bridge) -> Self {
        Self::Bridge(Box::new(v))
    }
}
impl From<BridgeConstructiveElement> for AbstractCityObject {
    fn from(v: BridgeConstructiveElement) -> Self {
        Self::BridgeConstructiveElement(Box::new(v))
    }
}
impl From<BridgeFurniture> for AbstractCityObject {
    fn from(v: BridgeFurniture) -> Self {
        Self::BridgeFurniture(Box::new(v))
    }
}
impl From<BridgeInstallation> for AbstractCityObject {
    fn from(v: BridgeInstallation) -> Self {
        Self::BridgeInstallation(Box::new(v))
    }
}
impl From<BridgePart> for AbstractCityObject {
    fn from(v: BridgePart) -> Self {
        Self::BridgePart(Box::new(v))
    }
}
impl From<BridgeRoom> for AbstractCityObject {
    fn from(v: BridgeRoom) -> Self {
        Self::BridgeRoom(Box::new(v))
    }
}
impl From<Building> for AbstractCityObject {
    fn from(v: Building) -> Self {
        Self::Building(Box::new(v))
    }
}
impl From<BuildingConstructiveElement> for AbstractCityObject {
    fn from(v: BuildingConstructiveElement) -> Self {
        Self::BuildingConstructiveElement(Box::new(v))
    }
}
impl From<BuildingFurniture> for AbstractCityObject {
    fn from(v: BuildingFurniture) -> Self {
        Self::BuildingFurniture(Box::new(v))
    }
}
impl From<BuildingInstallation> for AbstractCityObject {
    fn from(v: BuildingInstallation) -> Self {
        Self::BuildingInstallation(Box::new(v))
    }
}
impl From<BuildingPart> for AbstractCityObject {
    fn from(v: BuildingPart) -> Self {
        Self::BuildingPart(Box::new(v))
    }
}
impl From<BuildingRoom> for AbstractCityObject {
    fn from(v: BuildingRoom) -> Self {
        Self::BuildingRoom(Box::new(v))
    }
}
impl From<BuildingUnit> for AbstractCityObject {
    fn from(v: BuildingUnit) -> Self {
        Self::BuildingUnit(Box::new(v))
    }
}
impl From<Storey> for AbstractCityObject {
    fn from(v: Storey) -> Self {
        Self::Storey(Box::new(v))
    }
}
impl From<CityFurniture> for AbstractCityObject {
    fn from(v: CityFurniture) -> Self {
        Self::CityFurniture(Box::new(v))
    }
}
impl From<CityObjectGroup> for AbstractCityObject {
    fn from(v: CityObjectGroup) -> Self {
        Self::CityObjectGroup(Box::new(v))
    }
}
impl From<ClosureSurface> for AbstractCityObject {
    fn from(v: ClosureSurface) -> Self {
        Self::ClosureSurface(Box::new(v))
    }
}
impl From<GenericLogicalSpace> for AbstractCityObject {
    fn from(v: GenericLogicalSpace) -> Self {
        Self::GenericLogicalSpace(Box::new(v))
    }
}
impl From<GenericOccupiedSpace> for AbstractCityObject {
    fn from(v: GenericOccupiedSpace) -> Self {
        Self::GenericOccupiedSpace(Box::new(v))
    }
}
impl From<GenericThematicSurface> for AbstractCityObject {
    fn from(v: GenericThematicSurface) -> Self {
        Self::GenericThematicSurface(Box::new(v))
    }
}
impl From<GenericUnoccupiedSpace> for AbstractCityObject {
    fn from(v: GenericUnoccupiedSpace) -> Self {
        Self::GenericUnoccupiedSpace(Box::new(v))
    }
}
impl From<LandUse> for AbstractCityObject {
    fn from(v: LandUse) -> Self {
        Self::LandUse(Box::new(v))
    }
}
impl From<BreaklineRelief> for AbstractCityObject {
    fn from(v: BreaklineRelief) -> Self {
        Self::BreaklineRelief(Box::new(v))
    }
}
impl From<MassPointRelief> for AbstractCityObject {
    fn from(v: MassPointRelief) -> Self {
        Self::MassPointRelief(Box::new(v))
    }
}
impl From<RasterRelief> for AbstractCityObject {
    fn from(v: RasterRelief) -> Self {
        Self::RasterRelief(Box::new(v))
    }
}
impl From<ReliefFeature> for AbstractCityObject {
    fn from(v: ReliefFeature) -> Self {
        Self::ReliefFeature(Box::new(v))
    }
}
impl From<TINRelief> for AbstractCityObject {
    fn from(v: TINRelief) -> Self {
        Self::TINRelief(Box::new(v))
    }
}
impl From<AuxiliaryTrafficArea> for AbstractCityObject {
    fn from(v: AuxiliaryTrafficArea) -> Self {
        Self::AuxiliaryTrafficArea(Box::new(v))
    }
}
impl From<AuxiliaryTrafficSpace> for AbstractCityObject {
    fn from(v: AuxiliaryTrafficSpace) -> Self {
        Self::AuxiliaryTrafficSpace(Box::new(v))
    }
}
impl From<ClearanceSpace> for AbstractCityObject {
    fn from(v: ClearanceSpace) -> Self {
        Self::ClearanceSpace(Box::new(v))
    }
}
impl From<Hole> for AbstractCityObject {
    fn from(v: Hole) -> Self {
        Self::Hole(Box::new(v))
    }
}
impl From<HoleSurface> for AbstractCityObject {
    fn from(v: HoleSurface) -> Self {
        Self::HoleSurface(Box::new(v))
    }
}
impl From<Intersection> for AbstractCityObject {
    fn from(v: Intersection) -> Self {
        Self::Intersection(Box::new(v))
    }
}
impl From<Marking> for AbstractCityObject {
    fn from(v: Marking) -> Self {
        Self::Marking(Box::new(v))
    }
}
impl From<Railway> for AbstractCityObject {
    fn from(v: Railway) -> Self {
        Self::Railway(Box::new(v))
    }
}
impl From<Road> for AbstractCityObject {
    fn from(v: Road) -> Self {
        Self::Road(Box::new(v))
    }
}
impl From<Section> for AbstractCityObject {
    fn from(v: Section) -> Self {
        Self::Section(Box::new(v))
    }
}
impl From<Square> for AbstractCityObject {
    fn from(v: Square) -> Self {
        Self::Square(Box::new(v))
    }
}
impl From<Track> for AbstractCityObject {
    fn from(v: Track) -> Self {
        Self::Track(Box::new(v))
    }
}
impl From<TrafficArea> for AbstractCityObject {
    fn from(v: TrafficArea) -> Self {
        Self::TrafficArea(Box::new(v))
    }
}
impl From<TrafficSpace> for AbstractCityObject {
    fn from(v: TrafficSpace) -> Self {
        Self::TrafficSpace(Box::new(v))
    }
}
impl From<Waterway> for AbstractCityObject {
    fn from(v: Waterway) -> Self {
        Self::Waterway(Box::new(v))
    }
}
impl From<HollowSpace> for AbstractCityObject {
    fn from(v: HollowSpace) -> Self {
        Self::HollowSpace(Box::new(v))
    }
}
impl From<Tunnel> for AbstractCityObject {
    fn from(v: Tunnel) -> Self {
        Self::Tunnel(Box::new(v))
    }
}
impl From<TunnelConstructiveElement> for AbstractCityObject {
    fn from(v: TunnelConstructiveElement) -> Self {
        Self::TunnelConstructiveElement(Box::new(v))
    }
}
impl From<TunnelFurniture> for AbstractCityObject {
    fn from(v: TunnelFurniture) -> Self {
        Self::TunnelFurniture(Box::new(v))
    }
}
impl From<TunnelInstallation> for AbstractCityObject {
    fn from(v: TunnelInstallation) -> Self {
        Self::TunnelInstallation(Box::new(v))
    }
}
impl From<TunnelPart> for AbstractCityObject {
    fn from(v: TunnelPart) -> Self {
        Self::TunnelPart(Box::new(v))
    }
}
impl From<PlantCover> for AbstractCityObject {
    fn from(v: PlantCover) -> Self {
        Self::PlantCover(Box::new(v))
    }
}
impl From<SolitaryVegetationObject> for AbstractCityObject {
    fn from(v: SolitaryVegetationObject) -> Self {
        Self::SolitaryVegetationObject(Box::new(v))
    }
}
impl From<WaterBody> for AbstractCityObject {
    fn from(v: WaterBody) -> Self {
        Self::WaterBody(Box::new(v))
    }
}
impl From<WaterGroundSurface> for AbstractCityObject {
    fn from(v: WaterGroundSurface) -> Self {
        Self::WaterGroundSurface(Box::new(v))
    }
}
impl From<WaterSurface> for AbstractCityObject {
    fn from(v: WaterSurface) -> Self {
        Self::WaterSurface(Box::new(v))
    }
}
pub trait AbstractCityObjectAccessors {
    fn ceiling_surfaces(&self) -> impl Iterator<Item = &CeilingSurface>;
    fn doors(&self) -> impl Iterator<Item = &Door>;
    fn door_surfaces(&self) -> impl Iterator<Item = &DoorSurface>;
    fn floor_surfaces(&self) -> impl Iterator<Item = &FloorSurface>;
    fn ground_surfaces(&self) -> impl Iterator<Item = &GroundSurface>;
    fn interior_wall_surfaces(&self) -> impl Iterator<Item = &InteriorWallSurface>;
    fn other_constructions(&self) -> impl Iterator<Item = &OtherConstruction>;
    fn outer_ceiling_surfaces(&self) -> impl Iterator<Item = &OuterCeilingSurface>;
    fn outer_floor_surfaces(&self) -> impl Iterator<Item = &OuterFloorSurface>;
    fn roof_surfaces(&self) -> impl Iterator<Item = &RoofSurface>;
    fn wall_surfaces(&self) -> impl Iterator<Item = &WallSurface>;
    fn windows(&self) -> impl Iterator<Item = &Window>;
    fn window_surfaces(&self) -> impl Iterator<Item = &WindowSurface>;
    fn bridges(&self) -> impl Iterator<Item = &Bridge>;
    fn bridge_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BridgeConstructiveElement>;
    fn bridge_furnitures(&self) -> impl Iterator<Item = &BridgeFurniture>;
    fn bridge_installations(&self) -> impl Iterator<Item = &BridgeInstallation>;
    fn bridge_parts(&self) -> impl Iterator<Item = &BridgePart>;
    fn bridge_rooms(&self) -> impl Iterator<Item = &BridgeRoom>;
    fn buildings(&self) -> impl Iterator<Item = &Building>;
    fn building_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BuildingConstructiveElement>;
    fn building_furnitures(&self) -> impl Iterator<Item = &BuildingFurniture>;
    fn building_installations(&self) -> impl Iterator<Item = &BuildingInstallation>;
    fn building_parts(&self) -> impl Iterator<Item = &BuildingPart>;
    fn building_rooms(&self) -> impl Iterator<Item = &BuildingRoom>;
    fn building_units(&self) -> impl Iterator<Item = &BuildingUnit>;
    fn storeys(&self) -> impl Iterator<Item = &Storey>;
    fn city_furnitures(&self) -> impl Iterator<Item = &CityFurniture>;
    fn city_object_groups(&self) -> impl Iterator<Item = &CityObjectGroup>;
    fn closure_surfaces(&self) -> impl Iterator<Item = &ClosureSurface>;
    fn generic_logical_spaces(&self) -> impl Iterator<Item = &GenericLogicalSpace>;
    fn generic_occupied_spaces(&self) -> impl Iterator<Item = &GenericOccupiedSpace>;
    fn generic_thematic_surfaces(&self) -> impl Iterator<Item = &GenericThematicSurface>;
    fn generic_unoccupied_spaces(&self) -> impl Iterator<Item = &GenericUnoccupiedSpace>;
    fn land_uses(&self) -> impl Iterator<Item = &LandUse>;
    fn breakline_reliefs(&self) -> impl Iterator<Item = &BreaklineRelief>;
    fn mass_point_reliefs(&self) -> impl Iterator<Item = &MassPointRelief>;
    fn raster_reliefs(&self) -> impl Iterator<Item = &RasterRelief>;
    fn relief_features(&self) -> impl Iterator<Item = &ReliefFeature>;
    fn tin_reliefs(&self) -> impl Iterator<Item = &TINRelief>;
    fn auxiliary_traffic_areas(&self) -> impl Iterator<Item = &AuxiliaryTrafficArea>;
    fn auxiliary_traffic_spaces(&self) -> impl Iterator<Item = &AuxiliaryTrafficSpace>;
    fn clearance_spaces(&self) -> impl Iterator<Item = &ClearanceSpace>;
    fn holes(&self) -> impl Iterator<Item = &Hole>;
    fn hole_surfaces(&self) -> impl Iterator<Item = &HoleSurface>;
    fn intersections(&self) -> impl Iterator<Item = &Intersection>;
    fn markings(&self) -> impl Iterator<Item = &Marking>;
    fn railways(&self) -> impl Iterator<Item = &Railway>;
    fn roads(&self) -> impl Iterator<Item = &Road>;
    fn sections(&self) -> impl Iterator<Item = &Section>;
    fn squares(&self) -> impl Iterator<Item = &Square>;
    fn tracks(&self) -> impl Iterator<Item = &Track>;
    fn traffic_areas(&self) -> impl Iterator<Item = &TrafficArea>;
    fn traffic_spaces(&self) -> impl Iterator<Item = &TrafficSpace>;
    fn waterways(&self) -> impl Iterator<Item = &Waterway>;
    fn hollow_spaces(&self) -> impl Iterator<Item = &HollowSpace>;
    fn tunnels(&self) -> impl Iterator<Item = &Tunnel>;
    fn tunnel_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &TunnelConstructiveElement>;
    fn tunnel_furnitures(&self) -> impl Iterator<Item = &TunnelFurniture>;
    fn tunnel_installations(&self) -> impl Iterator<Item = &TunnelInstallation>;
    fn tunnel_parts(&self) -> impl Iterator<Item = &TunnelPart>;
    fn plant_covers(&self) -> impl Iterator<Item = &PlantCover>;
    fn solitary_vegetation_objects(
        &self,
    ) -> impl Iterator<Item = &SolitaryVegetationObject>;
    fn water_bodys(&self) -> impl Iterator<Item = &WaterBody>;
    fn water_ground_surfaces(&self) -> impl Iterator<Item = &WaterGroundSurface>;
    fn water_surfaces(&self) -> impl Iterator<Item = &WaterSurface>;
}
impl AbstractCityObjectAccessors for [AbstractCityObject] {
    fn ceiling_surfaces(&self) -> impl Iterator<Item = &CeilingSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::CeilingSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn doors(&self) -> impl Iterator<Item = &Door> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::Door(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn door_surfaces(&self) -> impl Iterator<Item = &DoorSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::DoorSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn floor_surfaces(&self) -> impl Iterator<Item = &FloorSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::FloorSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn ground_surfaces(&self) -> impl Iterator<Item = &GroundSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::GroundSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn interior_wall_surfaces(&self) -> impl Iterator<Item = &InteriorWallSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::InteriorWallSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn other_constructions(&self) -> impl Iterator<Item = &OtherConstruction> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::OtherConstruction(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn outer_ceiling_surfaces(&self) -> impl Iterator<Item = &OuterCeilingSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::OuterCeilingSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn outer_floor_surfaces(&self) -> impl Iterator<Item = &OuterFloorSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::OuterFloorSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn roof_surfaces(&self) -> impl Iterator<Item = &RoofSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::RoofSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn wall_surfaces(&self) -> impl Iterator<Item = &WallSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::WallSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn windows(&self) -> impl Iterator<Item = &Window> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::Window(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn window_surfaces(&self) -> impl Iterator<Item = &WindowSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::WindowSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridges(&self) -> impl Iterator<Item = &Bridge> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::Bridge(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BridgeConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::BridgeConstructiveElement(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_furnitures(&self) -> impl Iterator<Item = &BridgeFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::BridgeFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_installations(&self) -> impl Iterator<Item = &BridgeInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::BridgeInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_parts(&self) -> impl Iterator<Item = &BridgePart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::BridgePart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_rooms(&self) -> impl Iterator<Item = &BridgeRoom> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::BridgeRoom(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn buildings(&self) -> impl Iterator<Item = &Building> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::Building(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BuildingConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::BuildingConstructiveElement(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_furnitures(&self) -> impl Iterator<Item = &BuildingFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::BuildingFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_installations(&self) -> impl Iterator<Item = &BuildingInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::BuildingInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_parts(&self) -> impl Iterator<Item = &BuildingPart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::BuildingPart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_rooms(&self) -> impl Iterator<Item = &BuildingRoom> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::BuildingRoom(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_units(&self) -> impl Iterator<Item = &BuildingUnit> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::BuildingUnit(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn storeys(&self) -> impl Iterator<Item = &Storey> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::Storey(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn city_furnitures(&self) -> impl Iterator<Item = &CityFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::CityFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn city_object_groups(&self) -> impl Iterator<Item = &CityObjectGroup> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::CityObjectGroup(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn closure_surfaces(&self) -> impl Iterator<Item = &ClosureSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::ClosureSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_logical_spaces(&self) -> impl Iterator<Item = &GenericLogicalSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::GenericLogicalSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_occupied_spaces(&self) -> impl Iterator<Item = &GenericOccupiedSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::GenericOccupiedSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_thematic_surfaces(
        &self,
    ) -> impl Iterator<Item = &GenericThematicSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::GenericThematicSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_unoccupied_spaces(
        &self,
    ) -> impl Iterator<Item = &GenericUnoccupiedSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::GenericUnoccupiedSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn land_uses(&self) -> impl Iterator<Item = &LandUse> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::LandUse(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn breakline_reliefs(&self) -> impl Iterator<Item = &BreaklineRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::BreaklineRelief(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn mass_point_reliefs(&self) -> impl Iterator<Item = &MassPointRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::MassPointRelief(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn raster_reliefs(&self) -> impl Iterator<Item = &RasterRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::RasterRelief(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn relief_features(&self) -> impl Iterator<Item = &ReliefFeature> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::ReliefFeature(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tin_reliefs(&self) -> impl Iterator<Item = &TINRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::TINRelief(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn auxiliary_traffic_areas(&self) -> impl Iterator<Item = &AuxiliaryTrafficArea> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::AuxiliaryTrafficArea(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn auxiliary_traffic_spaces(&self) -> impl Iterator<Item = &AuxiliaryTrafficSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::AuxiliaryTrafficSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn clearance_spaces(&self) -> impl Iterator<Item = &ClearanceSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::ClearanceSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn holes(&self) -> impl Iterator<Item = &Hole> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::Hole(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn hole_surfaces(&self) -> impl Iterator<Item = &HoleSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::HoleSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn intersections(&self) -> impl Iterator<Item = &Intersection> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::Intersection(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn markings(&self) -> impl Iterator<Item = &Marking> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::Marking(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn railways(&self) -> impl Iterator<Item = &Railway> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::Railway(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn roads(&self) -> impl Iterator<Item = &Road> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::Road(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn sections(&self) -> impl Iterator<Item = &Section> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::Section(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn squares(&self) -> impl Iterator<Item = &Square> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::Square(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tracks(&self) -> impl Iterator<Item = &Track> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::Track(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn traffic_areas(&self) -> impl Iterator<Item = &TrafficArea> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::TrafficArea(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn traffic_spaces(&self) -> impl Iterator<Item = &TrafficSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::TrafficSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn waterways(&self) -> impl Iterator<Item = &Waterway> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::Waterway(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn hollow_spaces(&self) -> impl Iterator<Item = &HollowSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::HollowSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnels(&self) -> impl Iterator<Item = &Tunnel> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::Tunnel(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &TunnelConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::TunnelConstructiveElement(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_furnitures(&self) -> impl Iterator<Item = &TunnelFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::TunnelFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_installations(&self) -> impl Iterator<Item = &TunnelInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::TunnelInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_parts(&self) -> impl Iterator<Item = &TunnelPart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::TunnelPart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn plant_covers(&self) -> impl Iterator<Item = &PlantCover> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::PlantCover(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn solitary_vegetation_objects(
        &self,
    ) -> impl Iterator<Item = &SolitaryVegetationObject> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::SolitaryVegetationObject(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn water_bodys(&self) -> impl Iterator<Item = &WaterBody> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::WaterBody(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn water_ground_surfaces(&self) -> impl Iterator<Item = &WaterGroundSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::WaterGroundSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn water_surfaces(&self) -> impl Iterator<Item = &WaterSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractCityObject::WaterSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
}
pub trait AbstractDynamizerTrait: AbstractFeatureWithLifespanTrait {}
#[derive(Debug, Clone)]
pub enum AbstractDynamizer {
    Dynamizer(Dynamizer),
}
impl Default for AbstractDynamizer {
    fn default() -> Self {
        Self::Dynamizer(Default::default())
    }
}
impl AbstractFeatureTrait for AbstractDynamizer {
    fn feature_id(&self) -> &ID {
        match self {
            Self::Dynamizer(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::Dynamizer(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::Dynamizer(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::Dynamizer(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractDynamizer {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::Dynamizer(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::Dynamizer(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::Dynamizer(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::Dynamizer(v) => v.valid_to(),
        }
    }
}
impl AbstractDynamizerTrait for AbstractDynamizer {}
impl From<Dynamizer> for AbstractDynamizer {
    fn from(v: Dynamizer) -> Self {
        Self::Dynamizer(v)
    }
}
pub trait AbstractDynamizerAccessors {
    fn dynamizers(&self) -> impl Iterator<Item = &Dynamizer>;
}
impl AbstractDynamizerAccessors for [AbstractDynamizer] {
    fn dynamizers(&self) -> impl Iterator<Item = &Dynamizer> {
        self.iter()
            .map(|item| match item {
                AbstractDynamizer::Dynamizer(v) => v,
            })
    }
}
pub trait AbstractVersionTrait: AbstractFeatureWithLifespanTrait {}
#[derive(Debug, Clone)]
pub enum AbstractVersion {
    Version(Version),
}
impl Default for AbstractVersion {
    fn default() -> Self {
        Self::Version(Default::default())
    }
}
impl AbstractFeatureTrait for AbstractVersion {
    fn feature_id(&self) -> &ID {
        match self {
            Self::Version(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::Version(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::Version(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::Version(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractVersion {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::Version(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::Version(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::Version(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::Version(v) => v.valid_to(),
        }
    }
}
impl AbstractVersionTrait for AbstractVersion {}
impl From<Version> for AbstractVersion {
    fn from(v: Version) -> Self {
        Self::Version(v)
    }
}
pub trait AbstractVersionAccessors {
    fn versions(&self) -> impl Iterator<Item = &Version>;
}
impl AbstractVersionAccessors for [AbstractVersion] {
    fn versions(&self) -> impl Iterator<Item = &Version> {
        self.iter()
            .map(|item| match item {
                AbstractVersion::Version(v) => v,
            })
    }
}
pub trait AbstractVersionTransitionTrait: AbstractFeatureWithLifespanTrait {}
#[derive(Debug, Clone)]
pub enum AbstractVersionTransition {
    VersionTransition(VersionTransition),
}
impl Default for AbstractVersionTransition {
    fn default() -> Self {
        Self::VersionTransition(Default::default())
    }
}
impl AbstractFeatureTrait for AbstractVersionTransition {
    fn feature_id(&self) -> &ID {
        match self {
            Self::VersionTransition(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::VersionTransition(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::VersionTransition(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::VersionTransition(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractVersionTransition {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::VersionTransition(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::VersionTransition(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::VersionTransition(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::VersionTransition(v) => v.valid_to(),
        }
    }
}
impl AbstractVersionTransitionTrait for AbstractVersionTransition {}
impl From<VersionTransition> for AbstractVersionTransition {
    fn from(v: VersionTransition) -> Self {
        Self::VersionTransition(v)
    }
}
pub trait AbstractVersionTransitionAccessors {
    fn version_transitions(&self) -> impl Iterator<Item = &VersionTransition>;
}
impl AbstractVersionTransitionAccessors for [AbstractVersionTransition] {
    fn version_transitions(&self) -> impl Iterator<Item = &VersionTransition> {
        self.iter()
            .map(|item| match item {
                AbstractVersionTransition::VersionTransition(v) => v,
            })
    }
}
#[derive(Debug, Clone, Default)]
pub struct CityModel {
    pub feature_id: ID,
    pub identifier: Option<String>,
    pub name: Vec<String>,
    pub description: Option<String>,
    pub creation_date: Option<String>,
    pub termination_date: Option<String>,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    pub engineering_crs: Option<String>,
    pub city_model_member: Vec<CityModelMember>,
}
impl AbstractFeatureTrait for CityModel {
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
}
impl AbstractFeatureWithLifespanTrait for CityModel {
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
        let mut creation_date = None;
        let mut termination_date = None;
        let mut valid_from = None;
        let mut valid_to = None;
        let mut engineering_crs = None;
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
                (crate::namespace::NS_CORE, "engineeringCRS") => {
                    engineering_crs = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
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
            creation_date,
            termination_date,
            valid_from,
            valid_to,
            engineering_crs,
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
pub trait AbstractSpaceTrait: AbstractCityObjectTrait {
    fn space_type(&self) -> Option<SpaceType>;
    fn volume(&self) -> &[QualifiedVolume];
    fn area(&self) -> &[QualifiedArea];
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve>;
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface>;
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface>;
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid>;
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid>;
    fn boundary(&self) -> &[AbstractSpaceBoundary];
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve>;
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid>;
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition>;
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve>;
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface>;
}
#[derive(Debug, Clone)]
pub enum AbstractSpace {
    Door(Box<Door>),
    OtherConstruction(Box<OtherConstruction>),
    Window(Box<Window>),
    Bridge(Box<Bridge>),
    BridgeConstructiveElement(Box<BridgeConstructiveElement>),
    BridgeFurniture(Box<BridgeFurniture>),
    BridgeInstallation(Box<BridgeInstallation>),
    BridgePart(Box<BridgePart>),
    BridgeRoom(Box<BridgeRoom>),
    Building(Box<Building>),
    BuildingConstructiveElement(Box<BuildingConstructiveElement>),
    BuildingFurniture(Box<BuildingFurniture>),
    BuildingInstallation(Box<BuildingInstallation>),
    BuildingPart(Box<BuildingPart>),
    BuildingRoom(Box<BuildingRoom>),
    BuildingUnit(Box<BuildingUnit>),
    Storey(Box<Storey>),
    CityFurniture(Box<CityFurniture>),
    CityObjectGroup(Box<CityObjectGroup>),
    GenericLogicalSpace(Box<GenericLogicalSpace>),
    GenericOccupiedSpace(Box<GenericOccupiedSpace>),
    GenericUnoccupiedSpace(Box<GenericUnoccupiedSpace>),
    AuxiliaryTrafficSpace(Box<AuxiliaryTrafficSpace>),
    ClearanceSpace(Box<ClearanceSpace>),
    Hole(Box<Hole>),
    Intersection(Box<Intersection>),
    Railway(Box<Railway>),
    Road(Box<Road>),
    Section(Box<Section>),
    Square(Box<Square>),
    Track(Box<Track>),
    TrafficSpace(Box<TrafficSpace>),
    Waterway(Box<Waterway>),
    HollowSpace(Box<HollowSpace>),
    Tunnel(Box<Tunnel>),
    TunnelConstructiveElement(Box<TunnelConstructiveElement>),
    TunnelFurniture(Box<TunnelFurniture>),
    TunnelInstallation(Box<TunnelInstallation>),
    TunnelPart(Box<TunnelPart>),
    PlantCover(Box<PlantCover>),
    SolitaryVegetationObject(Box<SolitaryVegetationObject>),
    WaterBody(Box<WaterBody>),
}
impl Default for AbstractSpace {
    fn default() -> Self {
        Self::Door(Box::default())
    }
}
impl AbstractFeatureTrait for AbstractSpace {
    fn feature_id(&self) -> &ID {
        match self {
            Self::Door(v) => v.feature_id(),
            Self::OtherConstruction(v) => v.feature_id(),
            Self::Window(v) => v.feature_id(),
            Self::Bridge(v) => v.feature_id(),
            Self::BridgeConstructiveElement(v) => v.feature_id(),
            Self::BridgeFurniture(v) => v.feature_id(),
            Self::BridgeInstallation(v) => v.feature_id(),
            Self::BridgePart(v) => v.feature_id(),
            Self::BridgeRoom(v) => v.feature_id(),
            Self::Building(v) => v.feature_id(),
            Self::BuildingConstructiveElement(v) => v.feature_id(),
            Self::BuildingFurniture(v) => v.feature_id(),
            Self::BuildingInstallation(v) => v.feature_id(),
            Self::BuildingPart(v) => v.feature_id(),
            Self::BuildingRoom(v) => v.feature_id(),
            Self::BuildingUnit(v) => v.feature_id(),
            Self::Storey(v) => v.feature_id(),
            Self::CityFurniture(v) => v.feature_id(),
            Self::CityObjectGroup(v) => v.feature_id(),
            Self::GenericLogicalSpace(v) => v.feature_id(),
            Self::GenericOccupiedSpace(v) => v.feature_id(),
            Self::GenericUnoccupiedSpace(v) => v.feature_id(),
            Self::AuxiliaryTrafficSpace(v) => v.feature_id(),
            Self::ClearanceSpace(v) => v.feature_id(),
            Self::Hole(v) => v.feature_id(),
            Self::Intersection(v) => v.feature_id(),
            Self::Railway(v) => v.feature_id(),
            Self::Road(v) => v.feature_id(),
            Self::Section(v) => v.feature_id(),
            Self::Square(v) => v.feature_id(),
            Self::Track(v) => v.feature_id(),
            Self::TrafficSpace(v) => v.feature_id(),
            Self::Waterway(v) => v.feature_id(),
            Self::HollowSpace(v) => v.feature_id(),
            Self::Tunnel(v) => v.feature_id(),
            Self::TunnelConstructiveElement(v) => v.feature_id(),
            Self::TunnelFurniture(v) => v.feature_id(),
            Self::TunnelInstallation(v) => v.feature_id(),
            Self::TunnelPart(v) => v.feature_id(),
            Self::PlantCover(v) => v.feature_id(),
            Self::SolitaryVegetationObject(v) => v.feature_id(),
            Self::WaterBody(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.identifier(),
            Self::OtherConstruction(v) => v.identifier(),
            Self::Window(v) => v.identifier(),
            Self::Bridge(v) => v.identifier(),
            Self::BridgeConstructiveElement(v) => v.identifier(),
            Self::BridgeFurniture(v) => v.identifier(),
            Self::BridgeInstallation(v) => v.identifier(),
            Self::BridgePart(v) => v.identifier(),
            Self::BridgeRoom(v) => v.identifier(),
            Self::Building(v) => v.identifier(),
            Self::BuildingConstructiveElement(v) => v.identifier(),
            Self::BuildingFurniture(v) => v.identifier(),
            Self::BuildingInstallation(v) => v.identifier(),
            Self::BuildingPart(v) => v.identifier(),
            Self::BuildingRoom(v) => v.identifier(),
            Self::BuildingUnit(v) => v.identifier(),
            Self::Storey(v) => v.identifier(),
            Self::CityFurniture(v) => v.identifier(),
            Self::CityObjectGroup(v) => v.identifier(),
            Self::GenericLogicalSpace(v) => v.identifier(),
            Self::GenericOccupiedSpace(v) => v.identifier(),
            Self::GenericUnoccupiedSpace(v) => v.identifier(),
            Self::AuxiliaryTrafficSpace(v) => v.identifier(),
            Self::ClearanceSpace(v) => v.identifier(),
            Self::Hole(v) => v.identifier(),
            Self::Intersection(v) => v.identifier(),
            Self::Railway(v) => v.identifier(),
            Self::Road(v) => v.identifier(),
            Self::Section(v) => v.identifier(),
            Self::Square(v) => v.identifier(),
            Self::Track(v) => v.identifier(),
            Self::TrafficSpace(v) => v.identifier(),
            Self::Waterway(v) => v.identifier(),
            Self::HollowSpace(v) => v.identifier(),
            Self::Tunnel(v) => v.identifier(),
            Self::TunnelConstructiveElement(v) => v.identifier(),
            Self::TunnelFurniture(v) => v.identifier(),
            Self::TunnelInstallation(v) => v.identifier(),
            Self::TunnelPart(v) => v.identifier(),
            Self::PlantCover(v) => v.identifier(),
            Self::SolitaryVegetationObject(v) => v.identifier(),
            Self::WaterBody(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::Door(v) => v.name(),
            Self::OtherConstruction(v) => v.name(),
            Self::Window(v) => v.name(),
            Self::Bridge(v) => v.name(),
            Self::BridgeConstructiveElement(v) => v.name(),
            Self::BridgeFurniture(v) => v.name(),
            Self::BridgeInstallation(v) => v.name(),
            Self::BridgePart(v) => v.name(),
            Self::BridgeRoom(v) => v.name(),
            Self::Building(v) => v.name(),
            Self::BuildingConstructiveElement(v) => v.name(),
            Self::BuildingFurniture(v) => v.name(),
            Self::BuildingInstallation(v) => v.name(),
            Self::BuildingPart(v) => v.name(),
            Self::BuildingRoom(v) => v.name(),
            Self::BuildingUnit(v) => v.name(),
            Self::Storey(v) => v.name(),
            Self::CityFurniture(v) => v.name(),
            Self::CityObjectGroup(v) => v.name(),
            Self::GenericLogicalSpace(v) => v.name(),
            Self::GenericOccupiedSpace(v) => v.name(),
            Self::GenericUnoccupiedSpace(v) => v.name(),
            Self::AuxiliaryTrafficSpace(v) => v.name(),
            Self::ClearanceSpace(v) => v.name(),
            Self::Hole(v) => v.name(),
            Self::Intersection(v) => v.name(),
            Self::Railway(v) => v.name(),
            Self::Road(v) => v.name(),
            Self::Section(v) => v.name(),
            Self::Square(v) => v.name(),
            Self::Track(v) => v.name(),
            Self::TrafficSpace(v) => v.name(),
            Self::Waterway(v) => v.name(),
            Self::HollowSpace(v) => v.name(),
            Self::Tunnel(v) => v.name(),
            Self::TunnelConstructiveElement(v) => v.name(),
            Self::TunnelFurniture(v) => v.name(),
            Self::TunnelInstallation(v) => v.name(),
            Self::TunnelPart(v) => v.name(),
            Self::PlantCover(v) => v.name(),
            Self::SolitaryVegetationObject(v) => v.name(),
            Self::WaterBody(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.description(),
            Self::OtherConstruction(v) => v.description(),
            Self::Window(v) => v.description(),
            Self::Bridge(v) => v.description(),
            Self::BridgeConstructiveElement(v) => v.description(),
            Self::BridgeFurniture(v) => v.description(),
            Self::BridgeInstallation(v) => v.description(),
            Self::BridgePart(v) => v.description(),
            Self::BridgeRoom(v) => v.description(),
            Self::Building(v) => v.description(),
            Self::BuildingConstructiveElement(v) => v.description(),
            Self::BuildingFurniture(v) => v.description(),
            Self::BuildingInstallation(v) => v.description(),
            Self::BuildingPart(v) => v.description(),
            Self::BuildingRoom(v) => v.description(),
            Self::BuildingUnit(v) => v.description(),
            Self::Storey(v) => v.description(),
            Self::CityFurniture(v) => v.description(),
            Self::CityObjectGroup(v) => v.description(),
            Self::GenericLogicalSpace(v) => v.description(),
            Self::GenericOccupiedSpace(v) => v.description(),
            Self::GenericUnoccupiedSpace(v) => v.description(),
            Self::AuxiliaryTrafficSpace(v) => v.description(),
            Self::ClearanceSpace(v) => v.description(),
            Self::Hole(v) => v.description(),
            Self::Intersection(v) => v.description(),
            Self::Railway(v) => v.description(),
            Self::Road(v) => v.description(),
            Self::Section(v) => v.description(),
            Self::Square(v) => v.description(),
            Self::Track(v) => v.description(),
            Self::TrafficSpace(v) => v.description(),
            Self::Waterway(v) => v.description(),
            Self::HollowSpace(v) => v.description(),
            Self::Tunnel(v) => v.description(),
            Self::TunnelConstructiveElement(v) => v.description(),
            Self::TunnelFurniture(v) => v.description(),
            Self::TunnelInstallation(v) => v.description(),
            Self::TunnelPart(v) => v.description(),
            Self::PlantCover(v) => v.description(),
            Self::SolitaryVegetationObject(v) => v.description(),
            Self::WaterBody(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractSpace {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.creation_date(),
            Self::OtherConstruction(v) => v.creation_date(),
            Self::Window(v) => v.creation_date(),
            Self::Bridge(v) => v.creation_date(),
            Self::BridgeConstructiveElement(v) => v.creation_date(),
            Self::BridgeFurniture(v) => v.creation_date(),
            Self::BridgeInstallation(v) => v.creation_date(),
            Self::BridgePart(v) => v.creation_date(),
            Self::BridgeRoom(v) => v.creation_date(),
            Self::Building(v) => v.creation_date(),
            Self::BuildingConstructiveElement(v) => v.creation_date(),
            Self::BuildingFurniture(v) => v.creation_date(),
            Self::BuildingInstallation(v) => v.creation_date(),
            Self::BuildingPart(v) => v.creation_date(),
            Self::BuildingRoom(v) => v.creation_date(),
            Self::BuildingUnit(v) => v.creation_date(),
            Self::Storey(v) => v.creation_date(),
            Self::CityFurniture(v) => v.creation_date(),
            Self::CityObjectGroup(v) => v.creation_date(),
            Self::GenericLogicalSpace(v) => v.creation_date(),
            Self::GenericOccupiedSpace(v) => v.creation_date(),
            Self::GenericUnoccupiedSpace(v) => v.creation_date(),
            Self::AuxiliaryTrafficSpace(v) => v.creation_date(),
            Self::ClearanceSpace(v) => v.creation_date(),
            Self::Hole(v) => v.creation_date(),
            Self::Intersection(v) => v.creation_date(),
            Self::Railway(v) => v.creation_date(),
            Self::Road(v) => v.creation_date(),
            Self::Section(v) => v.creation_date(),
            Self::Square(v) => v.creation_date(),
            Self::Track(v) => v.creation_date(),
            Self::TrafficSpace(v) => v.creation_date(),
            Self::Waterway(v) => v.creation_date(),
            Self::HollowSpace(v) => v.creation_date(),
            Self::Tunnel(v) => v.creation_date(),
            Self::TunnelConstructiveElement(v) => v.creation_date(),
            Self::TunnelFurniture(v) => v.creation_date(),
            Self::TunnelInstallation(v) => v.creation_date(),
            Self::TunnelPart(v) => v.creation_date(),
            Self::PlantCover(v) => v.creation_date(),
            Self::SolitaryVegetationObject(v) => v.creation_date(),
            Self::WaterBody(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.termination_date(),
            Self::OtherConstruction(v) => v.termination_date(),
            Self::Window(v) => v.termination_date(),
            Self::Bridge(v) => v.termination_date(),
            Self::BridgeConstructiveElement(v) => v.termination_date(),
            Self::BridgeFurniture(v) => v.termination_date(),
            Self::BridgeInstallation(v) => v.termination_date(),
            Self::BridgePart(v) => v.termination_date(),
            Self::BridgeRoom(v) => v.termination_date(),
            Self::Building(v) => v.termination_date(),
            Self::BuildingConstructiveElement(v) => v.termination_date(),
            Self::BuildingFurniture(v) => v.termination_date(),
            Self::BuildingInstallation(v) => v.termination_date(),
            Self::BuildingPart(v) => v.termination_date(),
            Self::BuildingRoom(v) => v.termination_date(),
            Self::BuildingUnit(v) => v.termination_date(),
            Self::Storey(v) => v.termination_date(),
            Self::CityFurniture(v) => v.termination_date(),
            Self::CityObjectGroup(v) => v.termination_date(),
            Self::GenericLogicalSpace(v) => v.termination_date(),
            Self::GenericOccupiedSpace(v) => v.termination_date(),
            Self::GenericUnoccupiedSpace(v) => v.termination_date(),
            Self::AuxiliaryTrafficSpace(v) => v.termination_date(),
            Self::ClearanceSpace(v) => v.termination_date(),
            Self::Hole(v) => v.termination_date(),
            Self::Intersection(v) => v.termination_date(),
            Self::Railway(v) => v.termination_date(),
            Self::Road(v) => v.termination_date(),
            Self::Section(v) => v.termination_date(),
            Self::Square(v) => v.termination_date(),
            Self::Track(v) => v.termination_date(),
            Self::TrafficSpace(v) => v.termination_date(),
            Self::Waterway(v) => v.termination_date(),
            Self::HollowSpace(v) => v.termination_date(),
            Self::Tunnel(v) => v.termination_date(),
            Self::TunnelConstructiveElement(v) => v.termination_date(),
            Self::TunnelFurniture(v) => v.termination_date(),
            Self::TunnelInstallation(v) => v.termination_date(),
            Self::TunnelPart(v) => v.termination_date(),
            Self::PlantCover(v) => v.termination_date(),
            Self::SolitaryVegetationObject(v) => v.termination_date(),
            Self::WaterBody(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.valid_from(),
            Self::OtherConstruction(v) => v.valid_from(),
            Self::Window(v) => v.valid_from(),
            Self::Bridge(v) => v.valid_from(),
            Self::BridgeConstructiveElement(v) => v.valid_from(),
            Self::BridgeFurniture(v) => v.valid_from(),
            Self::BridgeInstallation(v) => v.valid_from(),
            Self::BridgePart(v) => v.valid_from(),
            Self::BridgeRoom(v) => v.valid_from(),
            Self::Building(v) => v.valid_from(),
            Self::BuildingConstructiveElement(v) => v.valid_from(),
            Self::BuildingFurniture(v) => v.valid_from(),
            Self::BuildingInstallation(v) => v.valid_from(),
            Self::BuildingPart(v) => v.valid_from(),
            Self::BuildingRoom(v) => v.valid_from(),
            Self::BuildingUnit(v) => v.valid_from(),
            Self::Storey(v) => v.valid_from(),
            Self::CityFurniture(v) => v.valid_from(),
            Self::CityObjectGroup(v) => v.valid_from(),
            Self::GenericLogicalSpace(v) => v.valid_from(),
            Self::GenericOccupiedSpace(v) => v.valid_from(),
            Self::GenericUnoccupiedSpace(v) => v.valid_from(),
            Self::AuxiliaryTrafficSpace(v) => v.valid_from(),
            Self::ClearanceSpace(v) => v.valid_from(),
            Self::Hole(v) => v.valid_from(),
            Self::Intersection(v) => v.valid_from(),
            Self::Railway(v) => v.valid_from(),
            Self::Road(v) => v.valid_from(),
            Self::Section(v) => v.valid_from(),
            Self::Square(v) => v.valid_from(),
            Self::Track(v) => v.valid_from(),
            Self::TrafficSpace(v) => v.valid_from(),
            Self::Waterway(v) => v.valid_from(),
            Self::HollowSpace(v) => v.valid_from(),
            Self::Tunnel(v) => v.valid_from(),
            Self::TunnelConstructiveElement(v) => v.valid_from(),
            Self::TunnelFurniture(v) => v.valid_from(),
            Self::TunnelInstallation(v) => v.valid_from(),
            Self::TunnelPart(v) => v.valid_from(),
            Self::PlantCover(v) => v.valid_from(),
            Self::SolitaryVegetationObject(v) => v.valid_from(),
            Self::WaterBody(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.valid_to(),
            Self::OtherConstruction(v) => v.valid_to(),
            Self::Window(v) => v.valid_to(),
            Self::Bridge(v) => v.valid_to(),
            Self::BridgeConstructiveElement(v) => v.valid_to(),
            Self::BridgeFurniture(v) => v.valid_to(),
            Self::BridgeInstallation(v) => v.valid_to(),
            Self::BridgePart(v) => v.valid_to(),
            Self::BridgeRoom(v) => v.valid_to(),
            Self::Building(v) => v.valid_to(),
            Self::BuildingConstructiveElement(v) => v.valid_to(),
            Self::BuildingFurniture(v) => v.valid_to(),
            Self::BuildingInstallation(v) => v.valid_to(),
            Self::BuildingPart(v) => v.valid_to(),
            Self::BuildingRoom(v) => v.valid_to(),
            Self::BuildingUnit(v) => v.valid_to(),
            Self::Storey(v) => v.valid_to(),
            Self::CityFurniture(v) => v.valid_to(),
            Self::CityObjectGroup(v) => v.valid_to(),
            Self::GenericLogicalSpace(v) => v.valid_to(),
            Self::GenericOccupiedSpace(v) => v.valid_to(),
            Self::GenericUnoccupiedSpace(v) => v.valid_to(),
            Self::AuxiliaryTrafficSpace(v) => v.valid_to(),
            Self::ClearanceSpace(v) => v.valid_to(),
            Self::Hole(v) => v.valid_to(),
            Self::Intersection(v) => v.valid_to(),
            Self::Railway(v) => v.valid_to(),
            Self::Road(v) => v.valid_to(),
            Self::Section(v) => v.valid_to(),
            Self::Square(v) => v.valid_to(),
            Self::Track(v) => v.valid_to(),
            Self::TrafficSpace(v) => v.valid_to(),
            Self::Waterway(v) => v.valid_to(),
            Self::HollowSpace(v) => v.valid_to(),
            Self::Tunnel(v) => v.valid_to(),
            Self::TunnelConstructiveElement(v) => v.valid_to(),
            Self::TunnelFurniture(v) => v.valid_to(),
            Self::TunnelInstallation(v) => v.valid_to(),
            Self::TunnelPart(v) => v.valid_to(),
            Self::PlantCover(v) => v.valid_to(),
            Self::SolitaryVegetationObject(v) => v.valid_to(),
            Self::WaterBody(v) => v.valid_to(),
        }
    }
}
impl AbstractCityObjectTrait for AbstractSpace {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::Door(v) => v.relative_to_terrain(),
            Self::OtherConstruction(v) => v.relative_to_terrain(),
            Self::Window(v) => v.relative_to_terrain(),
            Self::Bridge(v) => v.relative_to_terrain(),
            Self::BridgeConstructiveElement(v) => v.relative_to_terrain(),
            Self::BridgeFurniture(v) => v.relative_to_terrain(),
            Self::BridgeInstallation(v) => v.relative_to_terrain(),
            Self::BridgePart(v) => v.relative_to_terrain(),
            Self::BridgeRoom(v) => v.relative_to_terrain(),
            Self::Building(v) => v.relative_to_terrain(),
            Self::BuildingConstructiveElement(v) => v.relative_to_terrain(),
            Self::BuildingFurniture(v) => v.relative_to_terrain(),
            Self::BuildingInstallation(v) => v.relative_to_terrain(),
            Self::BuildingPart(v) => v.relative_to_terrain(),
            Self::BuildingRoom(v) => v.relative_to_terrain(),
            Self::BuildingUnit(v) => v.relative_to_terrain(),
            Self::Storey(v) => v.relative_to_terrain(),
            Self::CityFurniture(v) => v.relative_to_terrain(),
            Self::CityObjectGroup(v) => v.relative_to_terrain(),
            Self::GenericLogicalSpace(v) => v.relative_to_terrain(),
            Self::GenericOccupiedSpace(v) => v.relative_to_terrain(),
            Self::GenericUnoccupiedSpace(v) => v.relative_to_terrain(),
            Self::AuxiliaryTrafficSpace(v) => v.relative_to_terrain(),
            Self::ClearanceSpace(v) => v.relative_to_terrain(),
            Self::Hole(v) => v.relative_to_terrain(),
            Self::Intersection(v) => v.relative_to_terrain(),
            Self::Railway(v) => v.relative_to_terrain(),
            Self::Road(v) => v.relative_to_terrain(),
            Self::Section(v) => v.relative_to_terrain(),
            Self::Square(v) => v.relative_to_terrain(),
            Self::Track(v) => v.relative_to_terrain(),
            Self::TrafficSpace(v) => v.relative_to_terrain(),
            Self::Waterway(v) => v.relative_to_terrain(),
            Self::HollowSpace(v) => v.relative_to_terrain(),
            Self::Tunnel(v) => v.relative_to_terrain(),
            Self::TunnelConstructiveElement(v) => v.relative_to_terrain(),
            Self::TunnelFurniture(v) => v.relative_to_terrain(),
            Self::TunnelInstallation(v) => v.relative_to_terrain(),
            Self::TunnelPart(v) => v.relative_to_terrain(),
            Self::PlantCover(v) => v.relative_to_terrain(),
            Self::SolitaryVegetationObject(v) => v.relative_to_terrain(),
            Self::WaterBody(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::Door(v) => v.relative_to_water(),
            Self::OtherConstruction(v) => v.relative_to_water(),
            Self::Window(v) => v.relative_to_water(),
            Self::Bridge(v) => v.relative_to_water(),
            Self::BridgeConstructiveElement(v) => v.relative_to_water(),
            Self::BridgeFurniture(v) => v.relative_to_water(),
            Self::BridgeInstallation(v) => v.relative_to_water(),
            Self::BridgePart(v) => v.relative_to_water(),
            Self::BridgeRoom(v) => v.relative_to_water(),
            Self::Building(v) => v.relative_to_water(),
            Self::BuildingConstructiveElement(v) => v.relative_to_water(),
            Self::BuildingFurniture(v) => v.relative_to_water(),
            Self::BuildingInstallation(v) => v.relative_to_water(),
            Self::BuildingPart(v) => v.relative_to_water(),
            Self::BuildingRoom(v) => v.relative_to_water(),
            Self::BuildingUnit(v) => v.relative_to_water(),
            Self::Storey(v) => v.relative_to_water(),
            Self::CityFurniture(v) => v.relative_to_water(),
            Self::CityObjectGroup(v) => v.relative_to_water(),
            Self::GenericLogicalSpace(v) => v.relative_to_water(),
            Self::GenericOccupiedSpace(v) => v.relative_to_water(),
            Self::GenericUnoccupiedSpace(v) => v.relative_to_water(),
            Self::AuxiliaryTrafficSpace(v) => v.relative_to_water(),
            Self::ClearanceSpace(v) => v.relative_to_water(),
            Self::Hole(v) => v.relative_to_water(),
            Self::Intersection(v) => v.relative_to_water(),
            Self::Railway(v) => v.relative_to_water(),
            Self::Road(v) => v.relative_to_water(),
            Self::Section(v) => v.relative_to_water(),
            Self::Square(v) => v.relative_to_water(),
            Self::Track(v) => v.relative_to_water(),
            Self::TrafficSpace(v) => v.relative_to_water(),
            Self::Waterway(v) => v.relative_to_water(),
            Self::HollowSpace(v) => v.relative_to_water(),
            Self::Tunnel(v) => v.relative_to_water(),
            Self::TunnelConstructiveElement(v) => v.relative_to_water(),
            Self::TunnelFurniture(v) => v.relative_to_water(),
            Self::TunnelInstallation(v) => v.relative_to_water(),
            Self::TunnelPart(v) => v.relative_to_water(),
            Self::PlantCover(v) => v.relative_to_water(),
            Self::SolitaryVegetationObject(v) => v.relative_to_water(),
            Self::WaterBody(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::Door(v) => v.appearance(),
            Self::OtherConstruction(v) => v.appearance(),
            Self::Window(v) => v.appearance(),
            Self::Bridge(v) => v.appearance(),
            Self::BridgeConstructiveElement(v) => v.appearance(),
            Self::BridgeFurniture(v) => v.appearance(),
            Self::BridgeInstallation(v) => v.appearance(),
            Self::BridgePart(v) => v.appearance(),
            Self::BridgeRoom(v) => v.appearance(),
            Self::Building(v) => v.appearance(),
            Self::BuildingConstructiveElement(v) => v.appearance(),
            Self::BuildingFurniture(v) => v.appearance(),
            Self::BuildingInstallation(v) => v.appearance(),
            Self::BuildingPart(v) => v.appearance(),
            Self::BuildingRoom(v) => v.appearance(),
            Self::BuildingUnit(v) => v.appearance(),
            Self::Storey(v) => v.appearance(),
            Self::CityFurniture(v) => v.appearance(),
            Self::CityObjectGroup(v) => v.appearance(),
            Self::GenericLogicalSpace(v) => v.appearance(),
            Self::GenericOccupiedSpace(v) => v.appearance(),
            Self::GenericUnoccupiedSpace(v) => v.appearance(),
            Self::AuxiliaryTrafficSpace(v) => v.appearance(),
            Self::ClearanceSpace(v) => v.appearance(),
            Self::Hole(v) => v.appearance(),
            Self::Intersection(v) => v.appearance(),
            Self::Railway(v) => v.appearance(),
            Self::Road(v) => v.appearance(),
            Self::Section(v) => v.appearance(),
            Self::Square(v) => v.appearance(),
            Self::Track(v) => v.appearance(),
            Self::TrafficSpace(v) => v.appearance(),
            Self::Waterway(v) => v.appearance(),
            Self::HollowSpace(v) => v.appearance(),
            Self::Tunnel(v) => v.appearance(),
            Self::TunnelConstructiveElement(v) => v.appearance(),
            Self::TunnelFurniture(v) => v.appearance(),
            Self::TunnelInstallation(v) => v.appearance(),
            Self::TunnelPart(v) => v.appearance(),
            Self::PlantCover(v) => v.appearance(),
            Self::SolitaryVegetationObject(v) => v.appearance(),
            Self::WaterBody(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::Door(v) => v.generalizes_to(),
            Self::OtherConstruction(v) => v.generalizes_to(),
            Self::Window(v) => v.generalizes_to(),
            Self::Bridge(v) => v.generalizes_to(),
            Self::BridgeConstructiveElement(v) => v.generalizes_to(),
            Self::BridgeFurniture(v) => v.generalizes_to(),
            Self::BridgeInstallation(v) => v.generalizes_to(),
            Self::BridgePart(v) => v.generalizes_to(),
            Self::BridgeRoom(v) => v.generalizes_to(),
            Self::Building(v) => v.generalizes_to(),
            Self::BuildingConstructiveElement(v) => v.generalizes_to(),
            Self::BuildingFurniture(v) => v.generalizes_to(),
            Self::BuildingInstallation(v) => v.generalizes_to(),
            Self::BuildingPart(v) => v.generalizes_to(),
            Self::BuildingRoom(v) => v.generalizes_to(),
            Self::BuildingUnit(v) => v.generalizes_to(),
            Self::Storey(v) => v.generalizes_to(),
            Self::CityFurniture(v) => v.generalizes_to(),
            Self::CityObjectGroup(v) => v.generalizes_to(),
            Self::GenericLogicalSpace(v) => v.generalizes_to(),
            Self::GenericOccupiedSpace(v) => v.generalizes_to(),
            Self::GenericUnoccupiedSpace(v) => v.generalizes_to(),
            Self::AuxiliaryTrafficSpace(v) => v.generalizes_to(),
            Self::ClearanceSpace(v) => v.generalizes_to(),
            Self::Hole(v) => v.generalizes_to(),
            Self::Intersection(v) => v.generalizes_to(),
            Self::Railway(v) => v.generalizes_to(),
            Self::Road(v) => v.generalizes_to(),
            Self::Section(v) => v.generalizes_to(),
            Self::Square(v) => v.generalizes_to(),
            Self::Track(v) => v.generalizes_to(),
            Self::TrafficSpace(v) => v.generalizes_to(),
            Self::Waterway(v) => v.generalizes_to(),
            Self::HollowSpace(v) => v.generalizes_to(),
            Self::Tunnel(v) => v.generalizes_to(),
            Self::TunnelConstructiveElement(v) => v.generalizes_to(),
            Self::TunnelFurniture(v) => v.generalizes_to(),
            Self::TunnelInstallation(v) => v.generalizes_to(),
            Self::TunnelPart(v) => v.generalizes_to(),
            Self::PlantCover(v) => v.generalizes_to(),
            Self::SolitaryVegetationObject(v) => v.generalizes_to(),
            Self::WaterBody(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::Door(v) => v.external_reference(),
            Self::OtherConstruction(v) => v.external_reference(),
            Self::Window(v) => v.external_reference(),
            Self::Bridge(v) => v.external_reference(),
            Self::BridgeConstructiveElement(v) => v.external_reference(),
            Self::BridgeFurniture(v) => v.external_reference(),
            Self::BridgeInstallation(v) => v.external_reference(),
            Self::BridgePart(v) => v.external_reference(),
            Self::BridgeRoom(v) => v.external_reference(),
            Self::Building(v) => v.external_reference(),
            Self::BuildingConstructiveElement(v) => v.external_reference(),
            Self::BuildingFurniture(v) => v.external_reference(),
            Self::BuildingInstallation(v) => v.external_reference(),
            Self::BuildingPart(v) => v.external_reference(),
            Self::BuildingRoom(v) => v.external_reference(),
            Self::BuildingUnit(v) => v.external_reference(),
            Self::Storey(v) => v.external_reference(),
            Self::CityFurniture(v) => v.external_reference(),
            Self::CityObjectGroup(v) => v.external_reference(),
            Self::GenericLogicalSpace(v) => v.external_reference(),
            Self::GenericOccupiedSpace(v) => v.external_reference(),
            Self::GenericUnoccupiedSpace(v) => v.external_reference(),
            Self::AuxiliaryTrafficSpace(v) => v.external_reference(),
            Self::ClearanceSpace(v) => v.external_reference(),
            Self::Hole(v) => v.external_reference(),
            Self::Intersection(v) => v.external_reference(),
            Self::Railway(v) => v.external_reference(),
            Self::Road(v) => v.external_reference(),
            Self::Section(v) => v.external_reference(),
            Self::Square(v) => v.external_reference(),
            Self::Track(v) => v.external_reference(),
            Self::TrafficSpace(v) => v.external_reference(),
            Self::Waterway(v) => v.external_reference(),
            Self::HollowSpace(v) => v.external_reference(),
            Self::Tunnel(v) => v.external_reference(),
            Self::TunnelConstructiveElement(v) => v.external_reference(),
            Self::TunnelFurniture(v) => v.external_reference(),
            Self::TunnelInstallation(v) => v.external_reference(),
            Self::TunnelPart(v) => v.external_reference(),
            Self::PlantCover(v) => v.external_reference(),
            Self::SolitaryVegetationObject(v) => v.external_reference(),
            Self::WaterBody(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::Door(v) => v.related_to(),
            Self::OtherConstruction(v) => v.related_to(),
            Self::Window(v) => v.related_to(),
            Self::Bridge(v) => v.related_to(),
            Self::BridgeConstructiveElement(v) => v.related_to(),
            Self::BridgeFurniture(v) => v.related_to(),
            Self::BridgeInstallation(v) => v.related_to(),
            Self::BridgePart(v) => v.related_to(),
            Self::BridgeRoom(v) => v.related_to(),
            Self::Building(v) => v.related_to(),
            Self::BuildingConstructiveElement(v) => v.related_to(),
            Self::BuildingFurniture(v) => v.related_to(),
            Self::BuildingInstallation(v) => v.related_to(),
            Self::BuildingPart(v) => v.related_to(),
            Self::BuildingRoom(v) => v.related_to(),
            Self::BuildingUnit(v) => v.related_to(),
            Self::Storey(v) => v.related_to(),
            Self::CityFurniture(v) => v.related_to(),
            Self::CityObjectGroup(v) => v.related_to(),
            Self::GenericLogicalSpace(v) => v.related_to(),
            Self::GenericOccupiedSpace(v) => v.related_to(),
            Self::GenericUnoccupiedSpace(v) => v.related_to(),
            Self::AuxiliaryTrafficSpace(v) => v.related_to(),
            Self::ClearanceSpace(v) => v.related_to(),
            Self::Hole(v) => v.related_to(),
            Self::Intersection(v) => v.related_to(),
            Self::Railway(v) => v.related_to(),
            Self::Road(v) => v.related_to(),
            Self::Section(v) => v.related_to(),
            Self::Square(v) => v.related_to(),
            Self::Track(v) => v.related_to(),
            Self::TrafficSpace(v) => v.related_to(),
            Self::Waterway(v) => v.related_to(),
            Self::HollowSpace(v) => v.related_to(),
            Self::Tunnel(v) => v.related_to(),
            Self::TunnelConstructiveElement(v) => v.related_to(),
            Self::TunnelFurniture(v) => v.related_to(),
            Self::TunnelInstallation(v) => v.related_to(),
            Self::TunnelPart(v) => v.related_to(),
            Self::PlantCover(v) => v.related_to(),
            Self::SolitaryVegetationObject(v) => v.related_to(),
            Self::WaterBody(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::Door(v) => v.dynamizer(),
            Self::OtherConstruction(v) => v.dynamizer(),
            Self::Window(v) => v.dynamizer(),
            Self::Bridge(v) => v.dynamizer(),
            Self::BridgeConstructiveElement(v) => v.dynamizer(),
            Self::BridgeFurniture(v) => v.dynamizer(),
            Self::BridgeInstallation(v) => v.dynamizer(),
            Self::BridgePart(v) => v.dynamizer(),
            Self::BridgeRoom(v) => v.dynamizer(),
            Self::Building(v) => v.dynamizer(),
            Self::BuildingConstructiveElement(v) => v.dynamizer(),
            Self::BuildingFurniture(v) => v.dynamizer(),
            Self::BuildingInstallation(v) => v.dynamizer(),
            Self::BuildingPart(v) => v.dynamizer(),
            Self::BuildingRoom(v) => v.dynamizer(),
            Self::BuildingUnit(v) => v.dynamizer(),
            Self::Storey(v) => v.dynamizer(),
            Self::CityFurniture(v) => v.dynamizer(),
            Self::CityObjectGroup(v) => v.dynamizer(),
            Self::GenericLogicalSpace(v) => v.dynamizer(),
            Self::GenericOccupiedSpace(v) => v.dynamizer(),
            Self::GenericUnoccupiedSpace(v) => v.dynamizer(),
            Self::AuxiliaryTrafficSpace(v) => v.dynamizer(),
            Self::ClearanceSpace(v) => v.dynamizer(),
            Self::Hole(v) => v.dynamizer(),
            Self::Intersection(v) => v.dynamizer(),
            Self::Railway(v) => v.dynamizer(),
            Self::Road(v) => v.dynamizer(),
            Self::Section(v) => v.dynamizer(),
            Self::Square(v) => v.dynamizer(),
            Self::Track(v) => v.dynamizer(),
            Self::TrafficSpace(v) => v.dynamizer(),
            Self::Waterway(v) => v.dynamizer(),
            Self::HollowSpace(v) => v.dynamizer(),
            Self::Tunnel(v) => v.dynamizer(),
            Self::TunnelConstructiveElement(v) => v.dynamizer(),
            Self::TunnelFurniture(v) => v.dynamizer(),
            Self::TunnelInstallation(v) => v.dynamizer(),
            Self::TunnelPart(v) => v.dynamizer(),
            Self::PlantCover(v) => v.dynamizer(),
            Self::SolitaryVegetationObject(v) => v.dynamizer(),
            Self::WaterBody(v) => v.dynamizer(),
        }
    }
}
impl AbstractSpaceTrait for AbstractSpace {
    fn space_type(&self) -> Option<SpaceType> {
        match self {
            Self::Door(v) => v.space_type(),
            Self::OtherConstruction(v) => v.space_type(),
            Self::Window(v) => v.space_type(),
            Self::Bridge(v) => v.space_type(),
            Self::BridgeConstructiveElement(v) => v.space_type(),
            Self::BridgeFurniture(v) => v.space_type(),
            Self::BridgeInstallation(v) => v.space_type(),
            Self::BridgePart(v) => v.space_type(),
            Self::BridgeRoom(v) => v.space_type(),
            Self::Building(v) => v.space_type(),
            Self::BuildingConstructiveElement(v) => v.space_type(),
            Self::BuildingFurniture(v) => v.space_type(),
            Self::BuildingInstallation(v) => v.space_type(),
            Self::BuildingPart(v) => v.space_type(),
            Self::BuildingRoom(v) => v.space_type(),
            Self::BuildingUnit(v) => v.space_type(),
            Self::Storey(v) => v.space_type(),
            Self::CityFurniture(v) => v.space_type(),
            Self::CityObjectGroup(v) => v.space_type(),
            Self::GenericLogicalSpace(v) => v.space_type(),
            Self::GenericOccupiedSpace(v) => v.space_type(),
            Self::GenericUnoccupiedSpace(v) => v.space_type(),
            Self::AuxiliaryTrafficSpace(v) => v.space_type(),
            Self::ClearanceSpace(v) => v.space_type(),
            Self::Hole(v) => v.space_type(),
            Self::Intersection(v) => v.space_type(),
            Self::Railway(v) => v.space_type(),
            Self::Road(v) => v.space_type(),
            Self::Section(v) => v.space_type(),
            Self::Square(v) => v.space_type(),
            Self::Track(v) => v.space_type(),
            Self::TrafficSpace(v) => v.space_type(),
            Self::Waterway(v) => v.space_type(),
            Self::HollowSpace(v) => v.space_type(),
            Self::Tunnel(v) => v.space_type(),
            Self::TunnelConstructiveElement(v) => v.space_type(),
            Self::TunnelFurniture(v) => v.space_type(),
            Self::TunnelInstallation(v) => v.space_type(),
            Self::TunnelPart(v) => v.space_type(),
            Self::PlantCover(v) => v.space_type(),
            Self::SolitaryVegetationObject(v) => v.space_type(),
            Self::WaterBody(v) => v.space_type(),
        }
    }
    fn volume(&self) -> &[QualifiedVolume] {
        match self {
            Self::Door(v) => v.volume(),
            Self::OtherConstruction(v) => v.volume(),
            Self::Window(v) => v.volume(),
            Self::Bridge(v) => v.volume(),
            Self::BridgeConstructiveElement(v) => v.volume(),
            Self::BridgeFurniture(v) => v.volume(),
            Self::BridgeInstallation(v) => v.volume(),
            Self::BridgePart(v) => v.volume(),
            Self::BridgeRoom(v) => v.volume(),
            Self::Building(v) => v.volume(),
            Self::BuildingConstructiveElement(v) => v.volume(),
            Self::BuildingFurniture(v) => v.volume(),
            Self::BuildingInstallation(v) => v.volume(),
            Self::BuildingPart(v) => v.volume(),
            Self::BuildingRoom(v) => v.volume(),
            Self::BuildingUnit(v) => v.volume(),
            Self::Storey(v) => v.volume(),
            Self::CityFurniture(v) => v.volume(),
            Self::CityObjectGroup(v) => v.volume(),
            Self::GenericLogicalSpace(v) => v.volume(),
            Self::GenericOccupiedSpace(v) => v.volume(),
            Self::GenericUnoccupiedSpace(v) => v.volume(),
            Self::AuxiliaryTrafficSpace(v) => v.volume(),
            Self::ClearanceSpace(v) => v.volume(),
            Self::Hole(v) => v.volume(),
            Self::Intersection(v) => v.volume(),
            Self::Railway(v) => v.volume(),
            Self::Road(v) => v.volume(),
            Self::Section(v) => v.volume(),
            Self::Square(v) => v.volume(),
            Self::Track(v) => v.volume(),
            Self::TrafficSpace(v) => v.volume(),
            Self::Waterway(v) => v.volume(),
            Self::HollowSpace(v) => v.volume(),
            Self::Tunnel(v) => v.volume(),
            Self::TunnelConstructiveElement(v) => v.volume(),
            Self::TunnelFurniture(v) => v.volume(),
            Self::TunnelInstallation(v) => v.volume(),
            Self::TunnelPart(v) => v.volume(),
            Self::PlantCover(v) => v.volume(),
            Self::SolitaryVegetationObject(v) => v.volume(),
            Self::WaterBody(v) => v.volume(),
        }
    }
    fn area(&self) -> &[QualifiedArea] {
        match self {
            Self::Door(v) => v.area(),
            Self::OtherConstruction(v) => v.area(),
            Self::Window(v) => v.area(),
            Self::Bridge(v) => v.area(),
            Self::BridgeConstructiveElement(v) => v.area(),
            Self::BridgeFurniture(v) => v.area(),
            Self::BridgeInstallation(v) => v.area(),
            Self::BridgePart(v) => v.area(),
            Self::BridgeRoom(v) => v.area(),
            Self::Building(v) => v.area(),
            Self::BuildingConstructiveElement(v) => v.area(),
            Self::BuildingFurniture(v) => v.area(),
            Self::BuildingInstallation(v) => v.area(),
            Self::BuildingPart(v) => v.area(),
            Self::BuildingRoom(v) => v.area(),
            Self::BuildingUnit(v) => v.area(),
            Self::Storey(v) => v.area(),
            Self::CityFurniture(v) => v.area(),
            Self::CityObjectGroup(v) => v.area(),
            Self::GenericLogicalSpace(v) => v.area(),
            Self::GenericOccupiedSpace(v) => v.area(),
            Self::GenericUnoccupiedSpace(v) => v.area(),
            Self::AuxiliaryTrafficSpace(v) => v.area(),
            Self::ClearanceSpace(v) => v.area(),
            Self::Hole(v) => v.area(),
            Self::Intersection(v) => v.area(),
            Self::Railway(v) => v.area(),
            Self::Road(v) => v.area(),
            Self::Section(v) => v.area(),
            Self::Square(v) => v.area(),
            Self::Track(v) => v.area(),
            Self::TrafficSpace(v) => v.area(),
            Self::Waterway(v) => v.area(),
            Self::HollowSpace(v) => v.area(),
            Self::Tunnel(v) => v.area(),
            Self::TunnelConstructiveElement(v) => v.area(),
            Self::TunnelFurniture(v) => v.area(),
            Self::TunnelInstallation(v) => v.area(),
            Self::TunnelPart(v) => v.area(),
            Self::PlantCover(v) => v.area(),
            Self::SolitaryVegetationObject(v) => v.area(),
            Self::WaterBody(v) => v.area(),
        }
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod2_multi_curve(),
            Self::OtherConstruction(v) => v.lod2_multi_curve(),
            Self::Window(v) => v.lod2_multi_curve(),
            Self::Bridge(v) => v.lod2_multi_curve(),
            Self::BridgeConstructiveElement(v) => v.lod2_multi_curve(),
            Self::BridgeFurniture(v) => v.lod2_multi_curve(),
            Self::BridgeInstallation(v) => v.lod2_multi_curve(),
            Self::BridgePart(v) => v.lod2_multi_curve(),
            Self::BridgeRoom(v) => v.lod2_multi_curve(),
            Self::Building(v) => v.lod2_multi_curve(),
            Self::BuildingConstructiveElement(v) => v.lod2_multi_curve(),
            Self::BuildingFurniture(v) => v.lod2_multi_curve(),
            Self::BuildingInstallation(v) => v.lod2_multi_curve(),
            Self::BuildingPart(v) => v.lod2_multi_curve(),
            Self::BuildingRoom(v) => v.lod2_multi_curve(),
            Self::BuildingUnit(v) => v.lod2_multi_curve(),
            Self::Storey(v) => v.lod2_multi_curve(),
            Self::CityFurniture(v) => v.lod2_multi_curve(),
            Self::CityObjectGroup(v) => v.lod2_multi_curve(),
            Self::GenericLogicalSpace(v) => v.lod2_multi_curve(),
            Self::GenericOccupiedSpace(v) => v.lod2_multi_curve(),
            Self::GenericUnoccupiedSpace(v) => v.lod2_multi_curve(),
            Self::AuxiliaryTrafficSpace(v) => v.lod2_multi_curve(),
            Self::ClearanceSpace(v) => v.lod2_multi_curve(),
            Self::Hole(v) => v.lod2_multi_curve(),
            Self::Intersection(v) => v.lod2_multi_curve(),
            Self::Railway(v) => v.lod2_multi_curve(),
            Self::Road(v) => v.lod2_multi_curve(),
            Self::Section(v) => v.lod2_multi_curve(),
            Self::Square(v) => v.lod2_multi_curve(),
            Self::Track(v) => v.lod2_multi_curve(),
            Self::TrafficSpace(v) => v.lod2_multi_curve(),
            Self::Waterway(v) => v.lod2_multi_curve(),
            Self::HollowSpace(v) => v.lod2_multi_curve(),
            Self::Tunnel(v) => v.lod2_multi_curve(),
            Self::TunnelConstructiveElement(v) => v.lod2_multi_curve(),
            Self::TunnelFurniture(v) => v.lod2_multi_curve(),
            Self::TunnelInstallation(v) => v.lod2_multi_curve(),
            Self::TunnelPart(v) => v.lod2_multi_curve(),
            Self::PlantCover(v) => v.lod2_multi_curve(),
            Self::SolitaryVegetationObject(v) => v.lod2_multi_curve(),
            Self::WaterBody(v) => v.lod2_multi_curve(),
        }
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::Door(v) => v.lod3_multi_surface(),
            Self::OtherConstruction(v) => v.lod3_multi_surface(),
            Self::Window(v) => v.lod3_multi_surface(),
            Self::Bridge(v) => v.lod3_multi_surface(),
            Self::BridgeConstructiveElement(v) => v.lod3_multi_surface(),
            Self::BridgeFurniture(v) => v.lod3_multi_surface(),
            Self::BridgeInstallation(v) => v.lod3_multi_surface(),
            Self::BridgePart(v) => v.lod3_multi_surface(),
            Self::BridgeRoom(v) => v.lod3_multi_surface(),
            Self::Building(v) => v.lod3_multi_surface(),
            Self::BuildingConstructiveElement(v) => v.lod3_multi_surface(),
            Self::BuildingFurniture(v) => v.lod3_multi_surface(),
            Self::BuildingInstallation(v) => v.lod3_multi_surface(),
            Self::BuildingPart(v) => v.lod3_multi_surface(),
            Self::BuildingRoom(v) => v.lod3_multi_surface(),
            Self::BuildingUnit(v) => v.lod3_multi_surface(),
            Self::Storey(v) => v.lod3_multi_surface(),
            Self::CityFurniture(v) => v.lod3_multi_surface(),
            Self::CityObjectGroup(v) => v.lod3_multi_surface(),
            Self::GenericLogicalSpace(v) => v.lod3_multi_surface(),
            Self::GenericOccupiedSpace(v) => v.lod3_multi_surface(),
            Self::GenericUnoccupiedSpace(v) => v.lod3_multi_surface(),
            Self::AuxiliaryTrafficSpace(v) => v.lod3_multi_surface(),
            Self::ClearanceSpace(v) => v.lod3_multi_surface(),
            Self::Hole(v) => v.lod3_multi_surface(),
            Self::Intersection(v) => v.lod3_multi_surface(),
            Self::Railway(v) => v.lod3_multi_surface(),
            Self::Road(v) => v.lod3_multi_surface(),
            Self::Section(v) => v.lod3_multi_surface(),
            Self::Square(v) => v.lod3_multi_surface(),
            Self::Track(v) => v.lod3_multi_surface(),
            Self::TrafficSpace(v) => v.lod3_multi_surface(),
            Self::Waterway(v) => v.lod3_multi_surface(),
            Self::HollowSpace(v) => v.lod3_multi_surface(),
            Self::Tunnel(v) => v.lod3_multi_surface(),
            Self::TunnelConstructiveElement(v) => v.lod3_multi_surface(),
            Self::TunnelFurniture(v) => v.lod3_multi_surface(),
            Self::TunnelInstallation(v) => v.lod3_multi_surface(),
            Self::TunnelPart(v) => v.lod3_multi_surface(),
            Self::PlantCover(v) => v.lod3_multi_surface(),
            Self::SolitaryVegetationObject(v) => v.lod3_multi_surface(),
            Self::WaterBody(v) => v.lod3_multi_surface(),
        }
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::Door(v) => v.lod0_multi_surface(),
            Self::OtherConstruction(v) => v.lod0_multi_surface(),
            Self::Window(v) => v.lod0_multi_surface(),
            Self::Bridge(v) => v.lod0_multi_surface(),
            Self::BridgeConstructiveElement(v) => v.lod0_multi_surface(),
            Self::BridgeFurniture(v) => v.lod0_multi_surface(),
            Self::BridgeInstallation(v) => v.lod0_multi_surface(),
            Self::BridgePart(v) => v.lod0_multi_surface(),
            Self::BridgeRoom(v) => v.lod0_multi_surface(),
            Self::Building(v) => v.lod0_multi_surface(),
            Self::BuildingConstructiveElement(v) => v.lod0_multi_surface(),
            Self::BuildingFurniture(v) => v.lod0_multi_surface(),
            Self::BuildingInstallation(v) => v.lod0_multi_surface(),
            Self::BuildingPart(v) => v.lod0_multi_surface(),
            Self::BuildingRoom(v) => v.lod0_multi_surface(),
            Self::BuildingUnit(v) => v.lod0_multi_surface(),
            Self::Storey(v) => v.lod0_multi_surface(),
            Self::CityFurniture(v) => v.lod0_multi_surface(),
            Self::CityObjectGroup(v) => v.lod0_multi_surface(),
            Self::GenericLogicalSpace(v) => v.lod0_multi_surface(),
            Self::GenericOccupiedSpace(v) => v.lod0_multi_surface(),
            Self::GenericUnoccupiedSpace(v) => v.lod0_multi_surface(),
            Self::AuxiliaryTrafficSpace(v) => v.lod0_multi_surface(),
            Self::ClearanceSpace(v) => v.lod0_multi_surface(),
            Self::Hole(v) => v.lod0_multi_surface(),
            Self::Intersection(v) => v.lod0_multi_surface(),
            Self::Railway(v) => v.lod0_multi_surface(),
            Self::Road(v) => v.lod0_multi_surface(),
            Self::Section(v) => v.lod0_multi_surface(),
            Self::Square(v) => v.lod0_multi_surface(),
            Self::Track(v) => v.lod0_multi_surface(),
            Self::TrafficSpace(v) => v.lod0_multi_surface(),
            Self::Waterway(v) => v.lod0_multi_surface(),
            Self::HollowSpace(v) => v.lod0_multi_surface(),
            Self::Tunnel(v) => v.lod0_multi_surface(),
            Self::TunnelConstructiveElement(v) => v.lod0_multi_surface(),
            Self::TunnelFurniture(v) => v.lod0_multi_surface(),
            Self::TunnelInstallation(v) => v.lod0_multi_surface(),
            Self::TunnelPart(v) => v.lod0_multi_surface(),
            Self::PlantCover(v) => v.lod0_multi_surface(),
            Self::SolitaryVegetationObject(v) => v.lod0_multi_surface(),
            Self::WaterBody(v) => v.lod0_multi_surface(),
        }
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::Door(v) => v.lod1_solid(),
            Self::OtherConstruction(v) => v.lod1_solid(),
            Self::Window(v) => v.lod1_solid(),
            Self::Bridge(v) => v.lod1_solid(),
            Self::BridgeConstructiveElement(v) => v.lod1_solid(),
            Self::BridgeFurniture(v) => v.lod1_solid(),
            Self::BridgeInstallation(v) => v.lod1_solid(),
            Self::BridgePart(v) => v.lod1_solid(),
            Self::BridgeRoom(v) => v.lod1_solid(),
            Self::Building(v) => v.lod1_solid(),
            Self::BuildingConstructiveElement(v) => v.lod1_solid(),
            Self::BuildingFurniture(v) => v.lod1_solid(),
            Self::BuildingInstallation(v) => v.lod1_solid(),
            Self::BuildingPart(v) => v.lod1_solid(),
            Self::BuildingRoom(v) => v.lod1_solid(),
            Self::BuildingUnit(v) => v.lod1_solid(),
            Self::Storey(v) => v.lod1_solid(),
            Self::CityFurniture(v) => v.lod1_solid(),
            Self::CityObjectGroup(v) => v.lod1_solid(),
            Self::GenericLogicalSpace(v) => v.lod1_solid(),
            Self::GenericOccupiedSpace(v) => v.lod1_solid(),
            Self::GenericUnoccupiedSpace(v) => v.lod1_solid(),
            Self::AuxiliaryTrafficSpace(v) => v.lod1_solid(),
            Self::ClearanceSpace(v) => v.lod1_solid(),
            Self::Hole(v) => v.lod1_solid(),
            Self::Intersection(v) => v.lod1_solid(),
            Self::Railway(v) => v.lod1_solid(),
            Self::Road(v) => v.lod1_solid(),
            Self::Section(v) => v.lod1_solid(),
            Self::Square(v) => v.lod1_solid(),
            Self::Track(v) => v.lod1_solid(),
            Self::TrafficSpace(v) => v.lod1_solid(),
            Self::Waterway(v) => v.lod1_solid(),
            Self::HollowSpace(v) => v.lod1_solid(),
            Self::Tunnel(v) => v.lod1_solid(),
            Self::TunnelConstructiveElement(v) => v.lod1_solid(),
            Self::TunnelFurniture(v) => v.lod1_solid(),
            Self::TunnelInstallation(v) => v.lod1_solid(),
            Self::TunnelPart(v) => v.lod1_solid(),
            Self::PlantCover(v) => v.lod1_solid(),
            Self::SolitaryVegetationObject(v) => v.lod1_solid(),
            Self::WaterBody(v) => v.lod1_solid(),
        }
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::Door(v) => v.lod3_solid(),
            Self::OtherConstruction(v) => v.lod3_solid(),
            Self::Window(v) => v.lod3_solid(),
            Self::Bridge(v) => v.lod3_solid(),
            Self::BridgeConstructiveElement(v) => v.lod3_solid(),
            Self::BridgeFurniture(v) => v.lod3_solid(),
            Self::BridgeInstallation(v) => v.lod3_solid(),
            Self::BridgePart(v) => v.lod3_solid(),
            Self::BridgeRoom(v) => v.lod3_solid(),
            Self::Building(v) => v.lod3_solid(),
            Self::BuildingConstructiveElement(v) => v.lod3_solid(),
            Self::BuildingFurniture(v) => v.lod3_solid(),
            Self::BuildingInstallation(v) => v.lod3_solid(),
            Self::BuildingPart(v) => v.lod3_solid(),
            Self::BuildingRoom(v) => v.lod3_solid(),
            Self::BuildingUnit(v) => v.lod3_solid(),
            Self::Storey(v) => v.lod3_solid(),
            Self::CityFurniture(v) => v.lod3_solid(),
            Self::CityObjectGroup(v) => v.lod3_solid(),
            Self::GenericLogicalSpace(v) => v.lod3_solid(),
            Self::GenericOccupiedSpace(v) => v.lod3_solid(),
            Self::GenericUnoccupiedSpace(v) => v.lod3_solid(),
            Self::AuxiliaryTrafficSpace(v) => v.lod3_solid(),
            Self::ClearanceSpace(v) => v.lod3_solid(),
            Self::Hole(v) => v.lod3_solid(),
            Self::Intersection(v) => v.lod3_solid(),
            Self::Railway(v) => v.lod3_solid(),
            Self::Road(v) => v.lod3_solid(),
            Self::Section(v) => v.lod3_solid(),
            Self::Square(v) => v.lod3_solid(),
            Self::Track(v) => v.lod3_solid(),
            Self::TrafficSpace(v) => v.lod3_solid(),
            Self::Waterway(v) => v.lod3_solid(),
            Self::HollowSpace(v) => v.lod3_solid(),
            Self::Tunnel(v) => v.lod3_solid(),
            Self::TunnelConstructiveElement(v) => v.lod3_solid(),
            Self::TunnelFurniture(v) => v.lod3_solid(),
            Self::TunnelInstallation(v) => v.lod3_solid(),
            Self::TunnelPart(v) => v.lod3_solid(),
            Self::PlantCover(v) => v.lod3_solid(),
            Self::SolitaryVegetationObject(v) => v.lod3_solid(),
            Self::WaterBody(v) => v.lod3_solid(),
        }
    }
    fn boundary(&self) -> &[AbstractSpaceBoundary] {
        match self {
            Self::Door(v) => v.boundary(),
            Self::OtherConstruction(v) => v.boundary(),
            Self::Window(v) => v.boundary(),
            Self::Bridge(v) => v.boundary(),
            Self::BridgeConstructiveElement(v) => v.boundary(),
            Self::BridgeFurniture(v) => v.boundary(),
            Self::BridgeInstallation(v) => v.boundary(),
            Self::BridgePart(v) => v.boundary(),
            Self::BridgeRoom(v) => v.boundary(),
            Self::Building(v) => v.boundary(),
            Self::BuildingConstructiveElement(v) => v.boundary(),
            Self::BuildingFurniture(v) => v.boundary(),
            Self::BuildingInstallation(v) => v.boundary(),
            Self::BuildingPart(v) => v.boundary(),
            Self::BuildingRoom(v) => v.boundary(),
            Self::BuildingUnit(v) => v.boundary(),
            Self::Storey(v) => v.boundary(),
            Self::CityFurniture(v) => v.boundary(),
            Self::CityObjectGroup(v) => v.boundary(),
            Self::GenericLogicalSpace(v) => v.boundary(),
            Self::GenericOccupiedSpace(v) => v.boundary(),
            Self::GenericUnoccupiedSpace(v) => v.boundary(),
            Self::AuxiliaryTrafficSpace(v) => v.boundary(),
            Self::ClearanceSpace(v) => v.boundary(),
            Self::Hole(v) => v.boundary(),
            Self::Intersection(v) => v.boundary(),
            Self::Railway(v) => v.boundary(),
            Self::Road(v) => v.boundary(),
            Self::Section(v) => v.boundary(),
            Self::Square(v) => v.boundary(),
            Self::Track(v) => v.boundary(),
            Self::TrafficSpace(v) => v.boundary(),
            Self::Waterway(v) => v.boundary(),
            Self::HollowSpace(v) => v.boundary(),
            Self::Tunnel(v) => v.boundary(),
            Self::TunnelConstructiveElement(v) => v.boundary(),
            Self::TunnelFurniture(v) => v.boundary(),
            Self::TunnelInstallation(v) => v.boundary(),
            Self::TunnelPart(v) => v.boundary(),
            Self::PlantCover(v) => v.boundary(),
            Self::SolitaryVegetationObject(v) => v.boundary(),
            Self::WaterBody(v) => v.boundary(),
        }
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod0_multi_curve(),
            Self::OtherConstruction(v) => v.lod0_multi_curve(),
            Self::Window(v) => v.lod0_multi_curve(),
            Self::Bridge(v) => v.lod0_multi_curve(),
            Self::BridgeConstructiveElement(v) => v.lod0_multi_curve(),
            Self::BridgeFurniture(v) => v.lod0_multi_curve(),
            Self::BridgeInstallation(v) => v.lod0_multi_curve(),
            Self::BridgePart(v) => v.lod0_multi_curve(),
            Self::BridgeRoom(v) => v.lod0_multi_curve(),
            Self::Building(v) => v.lod0_multi_curve(),
            Self::BuildingConstructiveElement(v) => v.lod0_multi_curve(),
            Self::BuildingFurniture(v) => v.lod0_multi_curve(),
            Self::BuildingInstallation(v) => v.lod0_multi_curve(),
            Self::BuildingPart(v) => v.lod0_multi_curve(),
            Self::BuildingRoom(v) => v.lod0_multi_curve(),
            Self::BuildingUnit(v) => v.lod0_multi_curve(),
            Self::Storey(v) => v.lod0_multi_curve(),
            Self::CityFurniture(v) => v.lod0_multi_curve(),
            Self::CityObjectGroup(v) => v.lod0_multi_curve(),
            Self::GenericLogicalSpace(v) => v.lod0_multi_curve(),
            Self::GenericOccupiedSpace(v) => v.lod0_multi_curve(),
            Self::GenericUnoccupiedSpace(v) => v.lod0_multi_curve(),
            Self::AuxiliaryTrafficSpace(v) => v.lod0_multi_curve(),
            Self::ClearanceSpace(v) => v.lod0_multi_curve(),
            Self::Hole(v) => v.lod0_multi_curve(),
            Self::Intersection(v) => v.lod0_multi_curve(),
            Self::Railway(v) => v.lod0_multi_curve(),
            Self::Road(v) => v.lod0_multi_curve(),
            Self::Section(v) => v.lod0_multi_curve(),
            Self::Square(v) => v.lod0_multi_curve(),
            Self::Track(v) => v.lod0_multi_curve(),
            Self::TrafficSpace(v) => v.lod0_multi_curve(),
            Self::Waterway(v) => v.lod0_multi_curve(),
            Self::HollowSpace(v) => v.lod0_multi_curve(),
            Self::Tunnel(v) => v.lod0_multi_curve(),
            Self::TunnelConstructiveElement(v) => v.lod0_multi_curve(),
            Self::TunnelFurniture(v) => v.lod0_multi_curve(),
            Self::TunnelInstallation(v) => v.lod0_multi_curve(),
            Self::TunnelPart(v) => v.lod0_multi_curve(),
            Self::PlantCover(v) => v.lod0_multi_curve(),
            Self::SolitaryVegetationObject(v) => v.lod0_multi_curve(),
            Self::WaterBody(v) => v.lod0_multi_curve(),
        }
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::Door(v) => v.lod2_solid(),
            Self::OtherConstruction(v) => v.lod2_solid(),
            Self::Window(v) => v.lod2_solid(),
            Self::Bridge(v) => v.lod2_solid(),
            Self::BridgeConstructiveElement(v) => v.lod2_solid(),
            Self::BridgeFurniture(v) => v.lod2_solid(),
            Self::BridgeInstallation(v) => v.lod2_solid(),
            Self::BridgePart(v) => v.lod2_solid(),
            Self::BridgeRoom(v) => v.lod2_solid(),
            Self::Building(v) => v.lod2_solid(),
            Self::BuildingConstructiveElement(v) => v.lod2_solid(),
            Self::BuildingFurniture(v) => v.lod2_solid(),
            Self::BuildingInstallation(v) => v.lod2_solid(),
            Self::BuildingPart(v) => v.lod2_solid(),
            Self::BuildingRoom(v) => v.lod2_solid(),
            Self::BuildingUnit(v) => v.lod2_solid(),
            Self::Storey(v) => v.lod2_solid(),
            Self::CityFurniture(v) => v.lod2_solid(),
            Self::CityObjectGroup(v) => v.lod2_solid(),
            Self::GenericLogicalSpace(v) => v.lod2_solid(),
            Self::GenericOccupiedSpace(v) => v.lod2_solid(),
            Self::GenericUnoccupiedSpace(v) => v.lod2_solid(),
            Self::AuxiliaryTrafficSpace(v) => v.lod2_solid(),
            Self::ClearanceSpace(v) => v.lod2_solid(),
            Self::Hole(v) => v.lod2_solid(),
            Self::Intersection(v) => v.lod2_solid(),
            Self::Railway(v) => v.lod2_solid(),
            Self::Road(v) => v.lod2_solid(),
            Self::Section(v) => v.lod2_solid(),
            Self::Square(v) => v.lod2_solid(),
            Self::Track(v) => v.lod2_solid(),
            Self::TrafficSpace(v) => v.lod2_solid(),
            Self::Waterway(v) => v.lod2_solid(),
            Self::HollowSpace(v) => v.lod2_solid(),
            Self::Tunnel(v) => v.lod2_solid(),
            Self::TunnelConstructiveElement(v) => v.lod2_solid(),
            Self::TunnelFurniture(v) => v.lod2_solid(),
            Self::TunnelInstallation(v) => v.lod2_solid(),
            Self::TunnelPart(v) => v.lod2_solid(),
            Self::PlantCover(v) => v.lod2_solid(),
            Self::SolitaryVegetationObject(v) => v.lod2_solid(),
            Self::WaterBody(v) => v.lod2_solid(),
        }
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        match self {
            Self::Door(v) => v.lod0_point(),
            Self::OtherConstruction(v) => v.lod0_point(),
            Self::Window(v) => v.lod0_point(),
            Self::Bridge(v) => v.lod0_point(),
            Self::BridgeConstructiveElement(v) => v.lod0_point(),
            Self::BridgeFurniture(v) => v.lod0_point(),
            Self::BridgeInstallation(v) => v.lod0_point(),
            Self::BridgePart(v) => v.lod0_point(),
            Self::BridgeRoom(v) => v.lod0_point(),
            Self::Building(v) => v.lod0_point(),
            Self::BuildingConstructiveElement(v) => v.lod0_point(),
            Self::BuildingFurniture(v) => v.lod0_point(),
            Self::BuildingInstallation(v) => v.lod0_point(),
            Self::BuildingPart(v) => v.lod0_point(),
            Self::BuildingRoom(v) => v.lod0_point(),
            Self::BuildingUnit(v) => v.lod0_point(),
            Self::Storey(v) => v.lod0_point(),
            Self::CityFurniture(v) => v.lod0_point(),
            Self::CityObjectGroup(v) => v.lod0_point(),
            Self::GenericLogicalSpace(v) => v.lod0_point(),
            Self::GenericOccupiedSpace(v) => v.lod0_point(),
            Self::GenericUnoccupiedSpace(v) => v.lod0_point(),
            Self::AuxiliaryTrafficSpace(v) => v.lod0_point(),
            Self::ClearanceSpace(v) => v.lod0_point(),
            Self::Hole(v) => v.lod0_point(),
            Self::Intersection(v) => v.lod0_point(),
            Self::Railway(v) => v.lod0_point(),
            Self::Road(v) => v.lod0_point(),
            Self::Section(v) => v.lod0_point(),
            Self::Square(v) => v.lod0_point(),
            Self::Track(v) => v.lod0_point(),
            Self::TrafficSpace(v) => v.lod0_point(),
            Self::Waterway(v) => v.lod0_point(),
            Self::HollowSpace(v) => v.lod0_point(),
            Self::Tunnel(v) => v.lod0_point(),
            Self::TunnelConstructiveElement(v) => v.lod0_point(),
            Self::TunnelFurniture(v) => v.lod0_point(),
            Self::TunnelInstallation(v) => v.lod0_point(),
            Self::TunnelPart(v) => v.lod0_point(),
            Self::PlantCover(v) => v.lod0_point(),
            Self::SolitaryVegetationObject(v) => v.lod0_point(),
            Self::WaterBody(v) => v.lod0_point(),
        }
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod3_multi_curve(),
            Self::OtherConstruction(v) => v.lod3_multi_curve(),
            Self::Window(v) => v.lod3_multi_curve(),
            Self::Bridge(v) => v.lod3_multi_curve(),
            Self::BridgeConstructiveElement(v) => v.lod3_multi_curve(),
            Self::BridgeFurniture(v) => v.lod3_multi_curve(),
            Self::BridgeInstallation(v) => v.lod3_multi_curve(),
            Self::BridgePart(v) => v.lod3_multi_curve(),
            Self::BridgeRoom(v) => v.lod3_multi_curve(),
            Self::Building(v) => v.lod3_multi_curve(),
            Self::BuildingConstructiveElement(v) => v.lod3_multi_curve(),
            Self::BuildingFurniture(v) => v.lod3_multi_curve(),
            Self::BuildingInstallation(v) => v.lod3_multi_curve(),
            Self::BuildingPart(v) => v.lod3_multi_curve(),
            Self::BuildingRoom(v) => v.lod3_multi_curve(),
            Self::BuildingUnit(v) => v.lod3_multi_curve(),
            Self::Storey(v) => v.lod3_multi_curve(),
            Self::CityFurniture(v) => v.lod3_multi_curve(),
            Self::CityObjectGroup(v) => v.lod3_multi_curve(),
            Self::GenericLogicalSpace(v) => v.lod3_multi_curve(),
            Self::GenericOccupiedSpace(v) => v.lod3_multi_curve(),
            Self::GenericUnoccupiedSpace(v) => v.lod3_multi_curve(),
            Self::AuxiliaryTrafficSpace(v) => v.lod3_multi_curve(),
            Self::ClearanceSpace(v) => v.lod3_multi_curve(),
            Self::Hole(v) => v.lod3_multi_curve(),
            Self::Intersection(v) => v.lod3_multi_curve(),
            Self::Railway(v) => v.lod3_multi_curve(),
            Self::Road(v) => v.lod3_multi_curve(),
            Self::Section(v) => v.lod3_multi_curve(),
            Self::Square(v) => v.lod3_multi_curve(),
            Self::Track(v) => v.lod3_multi_curve(),
            Self::TrafficSpace(v) => v.lod3_multi_curve(),
            Self::Waterway(v) => v.lod3_multi_curve(),
            Self::HollowSpace(v) => v.lod3_multi_curve(),
            Self::Tunnel(v) => v.lod3_multi_curve(),
            Self::TunnelConstructiveElement(v) => v.lod3_multi_curve(),
            Self::TunnelFurniture(v) => v.lod3_multi_curve(),
            Self::TunnelInstallation(v) => v.lod3_multi_curve(),
            Self::TunnelPart(v) => v.lod3_multi_curve(),
            Self::PlantCover(v) => v.lod3_multi_curve(),
            Self::SolitaryVegetationObject(v) => v.lod3_multi_curve(),
            Self::WaterBody(v) => v.lod3_multi_curve(),
        }
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::Door(v) => v.lod2_multi_surface(),
            Self::OtherConstruction(v) => v.lod2_multi_surface(),
            Self::Window(v) => v.lod2_multi_surface(),
            Self::Bridge(v) => v.lod2_multi_surface(),
            Self::BridgeConstructiveElement(v) => v.lod2_multi_surface(),
            Self::BridgeFurniture(v) => v.lod2_multi_surface(),
            Self::BridgeInstallation(v) => v.lod2_multi_surface(),
            Self::BridgePart(v) => v.lod2_multi_surface(),
            Self::BridgeRoom(v) => v.lod2_multi_surface(),
            Self::Building(v) => v.lod2_multi_surface(),
            Self::BuildingConstructiveElement(v) => v.lod2_multi_surface(),
            Self::BuildingFurniture(v) => v.lod2_multi_surface(),
            Self::BuildingInstallation(v) => v.lod2_multi_surface(),
            Self::BuildingPart(v) => v.lod2_multi_surface(),
            Self::BuildingRoom(v) => v.lod2_multi_surface(),
            Self::BuildingUnit(v) => v.lod2_multi_surface(),
            Self::Storey(v) => v.lod2_multi_surface(),
            Self::CityFurniture(v) => v.lod2_multi_surface(),
            Self::CityObjectGroup(v) => v.lod2_multi_surface(),
            Self::GenericLogicalSpace(v) => v.lod2_multi_surface(),
            Self::GenericOccupiedSpace(v) => v.lod2_multi_surface(),
            Self::GenericUnoccupiedSpace(v) => v.lod2_multi_surface(),
            Self::AuxiliaryTrafficSpace(v) => v.lod2_multi_surface(),
            Self::ClearanceSpace(v) => v.lod2_multi_surface(),
            Self::Hole(v) => v.lod2_multi_surface(),
            Self::Intersection(v) => v.lod2_multi_surface(),
            Self::Railway(v) => v.lod2_multi_surface(),
            Self::Road(v) => v.lod2_multi_surface(),
            Self::Section(v) => v.lod2_multi_surface(),
            Self::Square(v) => v.lod2_multi_surface(),
            Self::Track(v) => v.lod2_multi_surface(),
            Self::TrafficSpace(v) => v.lod2_multi_surface(),
            Self::Waterway(v) => v.lod2_multi_surface(),
            Self::HollowSpace(v) => v.lod2_multi_surface(),
            Self::Tunnel(v) => v.lod2_multi_surface(),
            Self::TunnelConstructiveElement(v) => v.lod2_multi_surface(),
            Self::TunnelFurniture(v) => v.lod2_multi_surface(),
            Self::TunnelInstallation(v) => v.lod2_multi_surface(),
            Self::TunnelPart(v) => v.lod2_multi_surface(),
            Self::PlantCover(v) => v.lod2_multi_surface(),
            Self::SolitaryVegetationObject(v) => v.lod2_multi_surface(),
            Self::WaterBody(v) => v.lod2_multi_surface(),
        }
    }
}
impl From<Door> for AbstractSpace {
    fn from(v: Door) -> Self {
        Self::Door(Box::new(v))
    }
}
impl From<OtherConstruction> for AbstractSpace {
    fn from(v: OtherConstruction) -> Self {
        Self::OtherConstruction(Box::new(v))
    }
}
impl From<Window> for AbstractSpace {
    fn from(v: Window) -> Self {
        Self::Window(Box::new(v))
    }
}
impl From<Bridge> for AbstractSpace {
    fn from(v: Bridge) -> Self {
        Self::Bridge(Box::new(v))
    }
}
impl From<BridgeConstructiveElement> for AbstractSpace {
    fn from(v: BridgeConstructiveElement) -> Self {
        Self::BridgeConstructiveElement(Box::new(v))
    }
}
impl From<BridgeFurniture> for AbstractSpace {
    fn from(v: BridgeFurniture) -> Self {
        Self::BridgeFurniture(Box::new(v))
    }
}
impl From<BridgeInstallation> for AbstractSpace {
    fn from(v: BridgeInstallation) -> Self {
        Self::BridgeInstallation(Box::new(v))
    }
}
impl From<BridgePart> for AbstractSpace {
    fn from(v: BridgePart) -> Self {
        Self::BridgePart(Box::new(v))
    }
}
impl From<BridgeRoom> for AbstractSpace {
    fn from(v: BridgeRoom) -> Self {
        Self::BridgeRoom(Box::new(v))
    }
}
impl From<Building> for AbstractSpace {
    fn from(v: Building) -> Self {
        Self::Building(Box::new(v))
    }
}
impl From<BuildingConstructiveElement> for AbstractSpace {
    fn from(v: BuildingConstructiveElement) -> Self {
        Self::BuildingConstructiveElement(Box::new(v))
    }
}
impl From<BuildingFurniture> for AbstractSpace {
    fn from(v: BuildingFurniture) -> Self {
        Self::BuildingFurniture(Box::new(v))
    }
}
impl From<BuildingInstallation> for AbstractSpace {
    fn from(v: BuildingInstallation) -> Self {
        Self::BuildingInstallation(Box::new(v))
    }
}
impl From<BuildingPart> for AbstractSpace {
    fn from(v: BuildingPart) -> Self {
        Self::BuildingPart(Box::new(v))
    }
}
impl From<BuildingRoom> for AbstractSpace {
    fn from(v: BuildingRoom) -> Self {
        Self::BuildingRoom(Box::new(v))
    }
}
impl From<BuildingUnit> for AbstractSpace {
    fn from(v: BuildingUnit) -> Self {
        Self::BuildingUnit(Box::new(v))
    }
}
impl From<Storey> for AbstractSpace {
    fn from(v: Storey) -> Self {
        Self::Storey(Box::new(v))
    }
}
impl From<CityFurniture> for AbstractSpace {
    fn from(v: CityFurniture) -> Self {
        Self::CityFurniture(Box::new(v))
    }
}
impl From<CityObjectGroup> for AbstractSpace {
    fn from(v: CityObjectGroup) -> Self {
        Self::CityObjectGroup(Box::new(v))
    }
}
impl From<GenericLogicalSpace> for AbstractSpace {
    fn from(v: GenericLogicalSpace) -> Self {
        Self::GenericLogicalSpace(Box::new(v))
    }
}
impl From<GenericOccupiedSpace> for AbstractSpace {
    fn from(v: GenericOccupiedSpace) -> Self {
        Self::GenericOccupiedSpace(Box::new(v))
    }
}
impl From<GenericUnoccupiedSpace> for AbstractSpace {
    fn from(v: GenericUnoccupiedSpace) -> Self {
        Self::GenericUnoccupiedSpace(Box::new(v))
    }
}
impl From<AuxiliaryTrafficSpace> for AbstractSpace {
    fn from(v: AuxiliaryTrafficSpace) -> Self {
        Self::AuxiliaryTrafficSpace(Box::new(v))
    }
}
impl From<ClearanceSpace> for AbstractSpace {
    fn from(v: ClearanceSpace) -> Self {
        Self::ClearanceSpace(Box::new(v))
    }
}
impl From<Hole> for AbstractSpace {
    fn from(v: Hole) -> Self {
        Self::Hole(Box::new(v))
    }
}
impl From<Intersection> for AbstractSpace {
    fn from(v: Intersection) -> Self {
        Self::Intersection(Box::new(v))
    }
}
impl From<Railway> for AbstractSpace {
    fn from(v: Railway) -> Self {
        Self::Railway(Box::new(v))
    }
}
impl From<Road> for AbstractSpace {
    fn from(v: Road) -> Self {
        Self::Road(Box::new(v))
    }
}
impl From<Section> for AbstractSpace {
    fn from(v: Section) -> Self {
        Self::Section(Box::new(v))
    }
}
impl From<Square> for AbstractSpace {
    fn from(v: Square) -> Self {
        Self::Square(Box::new(v))
    }
}
impl From<Track> for AbstractSpace {
    fn from(v: Track) -> Self {
        Self::Track(Box::new(v))
    }
}
impl From<TrafficSpace> for AbstractSpace {
    fn from(v: TrafficSpace) -> Self {
        Self::TrafficSpace(Box::new(v))
    }
}
impl From<Waterway> for AbstractSpace {
    fn from(v: Waterway) -> Self {
        Self::Waterway(Box::new(v))
    }
}
impl From<HollowSpace> for AbstractSpace {
    fn from(v: HollowSpace) -> Self {
        Self::HollowSpace(Box::new(v))
    }
}
impl From<Tunnel> for AbstractSpace {
    fn from(v: Tunnel) -> Self {
        Self::Tunnel(Box::new(v))
    }
}
impl From<TunnelConstructiveElement> for AbstractSpace {
    fn from(v: TunnelConstructiveElement) -> Self {
        Self::TunnelConstructiveElement(Box::new(v))
    }
}
impl From<TunnelFurniture> for AbstractSpace {
    fn from(v: TunnelFurniture) -> Self {
        Self::TunnelFurniture(Box::new(v))
    }
}
impl From<TunnelInstallation> for AbstractSpace {
    fn from(v: TunnelInstallation) -> Self {
        Self::TunnelInstallation(Box::new(v))
    }
}
impl From<TunnelPart> for AbstractSpace {
    fn from(v: TunnelPart) -> Self {
        Self::TunnelPart(Box::new(v))
    }
}
impl From<PlantCover> for AbstractSpace {
    fn from(v: PlantCover) -> Self {
        Self::PlantCover(Box::new(v))
    }
}
impl From<SolitaryVegetationObject> for AbstractSpace {
    fn from(v: SolitaryVegetationObject) -> Self {
        Self::SolitaryVegetationObject(Box::new(v))
    }
}
impl From<WaterBody> for AbstractSpace {
    fn from(v: WaterBody) -> Self {
        Self::WaterBody(Box::new(v))
    }
}
pub trait AbstractSpaceAccessors {
    fn doors(&self) -> impl Iterator<Item = &Door>;
    fn other_constructions(&self) -> impl Iterator<Item = &OtherConstruction>;
    fn windows(&self) -> impl Iterator<Item = &Window>;
    fn bridges(&self) -> impl Iterator<Item = &Bridge>;
    fn bridge_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BridgeConstructiveElement>;
    fn bridge_furnitures(&self) -> impl Iterator<Item = &BridgeFurniture>;
    fn bridge_installations(&self) -> impl Iterator<Item = &BridgeInstallation>;
    fn bridge_parts(&self) -> impl Iterator<Item = &BridgePart>;
    fn bridge_rooms(&self) -> impl Iterator<Item = &BridgeRoom>;
    fn buildings(&self) -> impl Iterator<Item = &Building>;
    fn building_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BuildingConstructiveElement>;
    fn building_furnitures(&self) -> impl Iterator<Item = &BuildingFurniture>;
    fn building_installations(&self) -> impl Iterator<Item = &BuildingInstallation>;
    fn building_parts(&self) -> impl Iterator<Item = &BuildingPart>;
    fn building_rooms(&self) -> impl Iterator<Item = &BuildingRoom>;
    fn building_units(&self) -> impl Iterator<Item = &BuildingUnit>;
    fn storeys(&self) -> impl Iterator<Item = &Storey>;
    fn city_furnitures(&self) -> impl Iterator<Item = &CityFurniture>;
    fn city_object_groups(&self) -> impl Iterator<Item = &CityObjectGroup>;
    fn generic_logical_spaces(&self) -> impl Iterator<Item = &GenericLogicalSpace>;
    fn generic_occupied_spaces(&self) -> impl Iterator<Item = &GenericOccupiedSpace>;
    fn generic_unoccupied_spaces(&self) -> impl Iterator<Item = &GenericUnoccupiedSpace>;
    fn auxiliary_traffic_spaces(&self) -> impl Iterator<Item = &AuxiliaryTrafficSpace>;
    fn clearance_spaces(&self) -> impl Iterator<Item = &ClearanceSpace>;
    fn holes(&self) -> impl Iterator<Item = &Hole>;
    fn intersections(&self) -> impl Iterator<Item = &Intersection>;
    fn railways(&self) -> impl Iterator<Item = &Railway>;
    fn roads(&self) -> impl Iterator<Item = &Road>;
    fn sections(&self) -> impl Iterator<Item = &Section>;
    fn squares(&self) -> impl Iterator<Item = &Square>;
    fn tracks(&self) -> impl Iterator<Item = &Track>;
    fn traffic_spaces(&self) -> impl Iterator<Item = &TrafficSpace>;
    fn waterways(&self) -> impl Iterator<Item = &Waterway>;
    fn hollow_spaces(&self) -> impl Iterator<Item = &HollowSpace>;
    fn tunnels(&self) -> impl Iterator<Item = &Tunnel>;
    fn tunnel_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &TunnelConstructiveElement>;
    fn tunnel_furnitures(&self) -> impl Iterator<Item = &TunnelFurniture>;
    fn tunnel_installations(&self) -> impl Iterator<Item = &TunnelInstallation>;
    fn tunnel_parts(&self) -> impl Iterator<Item = &TunnelPart>;
    fn plant_covers(&self) -> impl Iterator<Item = &PlantCover>;
    fn solitary_vegetation_objects(
        &self,
    ) -> impl Iterator<Item = &SolitaryVegetationObject>;
    fn water_bodys(&self) -> impl Iterator<Item = &WaterBody>;
}
impl AbstractSpaceAccessors for [AbstractSpace] {
    fn doors(&self) -> impl Iterator<Item = &Door> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::Door(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn other_constructions(&self) -> impl Iterator<Item = &OtherConstruction> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::OtherConstruction(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn windows(&self) -> impl Iterator<Item = &Window> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::Window(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridges(&self) -> impl Iterator<Item = &Bridge> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::Bridge(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BridgeConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::BridgeConstructiveElement(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_furnitures(&self) -> impl Iterator<Item = &BridgeFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::BridgeFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_installations(&self) -> impl Iterator<Item = &BridgeInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::BridgeInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_parts(&self) -> impl Iterator<Item = &BridgePart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::BridgePart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_rooms(&self) -> impl Iterator<Item = &BridgeRoom> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::BridgeRoom(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn buildings(&self) -> impl Iterator<Item = &Building> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::Building(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BuildingConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::BuildingConstructiveElement(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_furnitures(&self) -> impl Iterator<Item = &BuildingFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::BuildingFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_installations(&self) -> impl Iterator<Item = &BuildingInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::BuildingInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_parts(&self) -> impl Iterator<Item = &BuildingPart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::BuildingPart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_rooms(&self) -> impl Iterator<Item = &BuildingRoom> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::BuildingRoom(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_units(&self) -> impl Iterator<Item = &BuildingUnit> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::BuildingUnit(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn storeys(&self) -> impl Iterator<Item = &Storey> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::Storey(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn city_furnitures(&self) -> impl Iterator<Item = &CityFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::CityFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn city_object_groups(&self) -> impl Iterator<Item = &CityObjectGroup> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::CityObjectGroup(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_logical_spaces(&self) -> impl Iterator<Item = &GenericLogicalSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::GenericLogicalSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_occupied_spaces(&self) -> impl Iterator<Item = &GenericOccupiedSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::GenericOccupiedSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_unoccupied_spaces(
        &self,
    ) -> impl Iterator<Item = &GenericUnoccupiedSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::GenericUnoccupiedSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn auxiliary_traffic_spaces(&self) -> impl Iterator<Item = &AuxiliaryTrafficSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::AuxiliaryTrafficSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn clearance_spaces(&self) -> impl Iterator<Item = &ClearanceSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::ClearanceSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn holes(&self) -> impl Iterator<Item = &Hole> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::Hole(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn intersections(&self) -> impl Iterator<Item = &Intersection> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::Intersection(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn railways(&self) -> impl Iterator<Item = &Railway> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::Railway(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn roads(&self) -> impl Iterator<Item = &Road> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::Road(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn sections(&self) -> impl Iterator<Item = &Section> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::Section(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn squares(&self) -> impl Iterator<Item = &Square> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::Square(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tracks(&self) -> impl Iterator<Item = &Track> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::Track(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn traffic_spaces(&self) -> impl Iterator<Item = &TrafficSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::TrafficSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn waterways(&self) -> impl Iterator<Item = &Waterway> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::Waterway(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn hollow_spaces(&self) -> impl Iterator<Item = &HollowSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::HollowSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnels(&self) -> impl Iterator<Item = &Tunnel> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::Tunnel(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &TunnelConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::TunnelConstructiveElement(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_furnitures(&self) -> impl Iterator<Item = &TunnelFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::TunnelFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_installations(&self) -> impl Iterator<Item = &TunnelInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::TunnelInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_parts(&self) -> impl Iterator<Item = &TunnelPart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::TunnelPart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn plant_covers(&self) -> impl Iterator<Item = &PlantCover> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::PlantCover(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn solitary_vegetation_objects(
        &self,
    ) -> impl Iterator<Item = &SolitaryVegetationObject> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::SolitaryVegetationObject(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn water_bodys(&self) -> impl Iterator<Item = &WaterBody> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpace::WaterBody(v) => Some(v.as_ref()),
                _ => None,
            })
    }
}
pub trait AbstractSpaceBoundaryTrait: AbstractCityObjectTrait {}
#[derive(Debug, Clone)]
pub enum AbstractSpaceBoundary {
    CeilingSurface(Box<CeilingSurface>),
    DoorSurface(Box<DoorSurface>),
    FloorSurface(Box<FloorSurface>),
    GroundSurface(Box<GroundSurface>),
    InteriorWallSurface(Box<InteriorWallSurface>),
    OuterCeilingSurface(Box<OuterCeilingSurface>),
    OuterFloorSurface(Box<OuterFloorSurface>),
    RoofSurface(Box<RoofSurface>),
    WallSurface(Box<WallSurface>),
    WindowSurface(Box<WindowSurface>),
    ClosureSurface(Box<ClosureSurface>),
    GenericThematicSurface(Box<GenericThematicSurface>),
    LandUse(Box<LandUse>),
    BreaklineRelief(Box<BreaklineRelief>),
    MassPointRelief(Box<MassPointRelief>),
    RasterRelief(Box<RasterRelief>),
    ReliefFeature(Box<ReliefFeature>),
    TINRelief(Box<TINRelief>),
    AuxiliaryTrafficArea(Box<AuxiliaryTrafficArea>),
    HoleSurface(Box<HoleSurface>),
    Marking(Box<Marking>),
    TrafficArea(Box<TrafficArea>),
    WaterGroundSurface(Box<WaterGroundSurface>),
    WaterSurface(Box<WaterSurface>),
}
impl Default for AbstractSpaceBoundary {
    fn default() -> Self {
        Self::CeilingSurface(Box::default())
    }
}
impl AbstractFeatureTrait for AbstractSpaceBoundary {
    fn feature_id(&self) -> &ID {
        match self {
            Self::CeilingSurface(v) => v.feature_id(),
            Self::DoorSurface(v) => v.feature_id(),
            Self::FloorSurface(v) => v.feature_id(),
            Self::GroundSurface(v) => v.feature_id(),
            Self::InteriorWallSurface(v) => v.feature_id(),
            Self::OuterCeilingSurface(v) => v.feature_id(),
            Self::OuterFloorSurface(v) => v.feature_id(),
            Self::RoofSurface(v) => v.feature_id(),
            Self::WallSurface(v) => v.feature_id(),
            Self::WindowSurface(v) => v.feature_id(),
            Self::ClosureSurface(v) => v.feature_id(),
            Self::GenericThematicSurface(v) => v.feature_id(),
            Self::LandUse(v) => v.feature_id(),
            Self::BreaklineRelief(v) => v.feature_id(),
            Self::MassPointRelief(v) => v.feature_id(),
            Self::RasterRelief(v) => v.feature_id(),
            Self::ReliefFeature(v) => v.feature_id(),
            Self::TINRelief(v) => v.feature_id(),
            Self::AuxiliaryTrafficArea(v) => v.feature_id(),
            Self::HoleSurface(v) => v.feature_id(),
            Self::Marking(v) => v.feature_id(),
            Self::TrafficArea(v) => v.feature_id(),
            Self::WaterGroundSurface(v) => v.feature_id(),
            Self::WaterSurface(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.identifier(),
            Self::DoorSurface(v) => v.identifier(),
            Self::FloorSurface(v) => v.identifier(),
            Self::GroundSurface(v) => v.identifier(),
            Self::InteriorWallSurface(v) => v.identifier(),
            Self::OuterCeilingSurface(v) => v.identifier(),
            Self::OuterFloorSurface(v) => v.identifier(),
            Self::RoofSurface(v) => v.identifier(),
            Self::WallSurface(v) => v.identifier(),
            Self::WindowSurface(v) => v.identifier(),
            Self::ClosureSurface(v) => v.identifier(),
            Self::GenericThematicSurface(v) => v.identifier(),
            Self::LandUse(v) => v.identifier(),
            Self::BreaklineRelief(v) => v.identifier(),
            Self::MassPointRelief(v) => v.identifier(),
            Self::RasterRelief(v) => v.identifier(),
            Self::ReliefFeature(v) => v.identifier(),
            Self::TINRelief(v) => v.identifier(),
            Self::AuxiliaryTrafficArea(v) => v.identifier(),
            Self::HoleSurface(v) => v.identifier(),
            Self::Marking(v) => v.identifier(),
            Self::TrafficArea(v) => v.identifier(),
            Self::WaterGroundSurface(v) => v.identifier(),
            Self::WaterSurface(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::CeilingSurface(v) => v.name(),
            Self::DoorSurface(v) => v.name(),
            Self::FloorSurface(v) => v.name(),
            Self::GroundSurface(v) => v.name(),
            Self::InteriorWallSurface(v) => v.name(),
            Self::OuterCeilingSurface(v) => v.name(),
            Self::OuterFloorSurface(v) => v.name(),
            Self::RoofSurface(v) => v.name(),
            Self::WallSurface(v) => v.name(),
            Self::WindowSurface(v) => v.name(),
            Self::ClosureSurface(v) => v.name(),
            Self::GenericThematicSurface(v) => v.name(),
            Self::LandUse(v) => v.name(),
            Self::BreaklineRelief(v) => v.name(),
            Self::MassPointRelief(v) => v.name(),
            Self::RasterRelief(v) => v.name(),
            Self::ReliefFeature(v) => v.name(),
            Self::TINRelief(v) => v.name(),
            Self::AuxiliaryTrafficArea(v) => v.name(),
            Self::HoleSurface(v) => v.name(),
            Self::Marking(v) => v.name(),
            Self::TrafficArea(v) => v.name(),
            Self::WaterGroundSurface(v) => v.name(),
            Self::WaterSurface(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.description(),
            Self::DoorSurface(v) => v.description(),
            Self::FloorSurface(v) => v.description(),
            Self::GroundSurface(v) => v.description(),
            Self::InteriorWallSurface(v) => v.description(),
            Self::OuterCeilingSurface(v) => v.description(),
            Self::OuterFloorSurface(v) => v.description(),
            Self::RoofSurface(v) => v.description(),
            Self::WallSurface(v) => v.description(),
            Self::WindowSurface(v) => v.description(),
            Self::ClosureSurface(v) => v.description(),
            Self::GenericThematicSurface(v) => v.description(),
            Self::LandUse(v) => v.description(),
            Self::BreaklineRelief(v) => v.description(),
            Self::MassPointRelief(v) => v.description(),
            Self::RasterRelief(v) => v.description(),
            Self::ReliefFeature(v) => v.description(),
            Self::TINRelief(v) => v.description(),
            Self::AuxiliaryTrafficArea(v) => v.description(),
            Self::HoleSurface(v) => v.description(),
            Self::Marking(v) => v.description(),
            Self::TrafficArea(v) => v.description(),
            Self::WaterGroundSurface(v) => v.description(),
            Self::WaterSurface(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractSpaceBoundary {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.creation_date(),
            Self::DoorSurface(v) => v.creation_date(),
            Self::FloorSurface(v) => v.creation_date(),
            Self::GroundSurface(v) => v.creation_date(),
            Self::InteriorWallSurface(v) => v.creation_date(),
            Self::OuterCeilingSurface(v) => v.creation_date(),
            Self::OuterFloorSurface(v) => v.creation_date(),
            Self::RoofSurface(v) => v.creation_date(),
            Self::WallSurface(v) => v.creation_date(),
            Self::WindowSurface(v) => v.creation_date(),
            Self::ClosureSurface(v) => v.creation_date(),
            Self::GenericThematicSurface(v) => v.creation_date(),
            Self::LandUse(v) => v.creation_date(),
            Self::BreaklineRelief(v) => v.creation_date(),
            Self::MassPointRelief(v) => v.creation_date(),
            Self::RasterRelief(v) => v.creation_date(),
            Self::ReliefFeature(v) => v.creation_date(),
            Self::TINRelief(v) => v.creation_date(),
            Self::AuxiliaryTrafficArea(v) => v.creation_date(),
            Self::HoleSurface(v) => v.creation_date(),
            Self::Marking(v) => v.creation_date(),
            Self::TrafficArea(v) => v.creation_date(),
            Self::WaterGroundSurface(v) => v.creation_date(),
            Self::WaterSurface(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.termination_date(),
            Self::DoorSurface(v) => v.termination_date(),
            Self::FloorSurface(v) => v.termination_date(),
            Self::GroundSurface(v) => v.termination_date(),
            Self::InteriorWallSurface(v) => v.termination_date(),
            Self::OuterCeilingSurface(v) => v.termination_date(),
            Self::OuterFloorSurface(v) => v.termination_date(),
            Self::RoofSurface(v) => v.termination_date(),
            Self::WallSurface(v) => v.termination_date(),
            Self::WindowSurface(v) => v.termination_date(),
            Self::ClosureSurface(v) => v.termination_date(),
            Self::GenericThematicSurface(v) => v.termination_date(),
            Self::LandUse(v) => v.termination_date(),
            Self::BreaklineRelief(v) => v.termination_date(),
            Self::MassPointRelief(v) => v.termination_date(),
            Self::RasterRelief(v) => v.termination_date(),
            Self::ReliefFeature(v) => v.termination_date(),
            Self::TINRelief(v) => v.termination_date(),
            Self::AuxiliaryTrafficArea(v) => v.termination_date(),
            Self::HoleSurface(v) => v.termination_date(),
            Self::Marking(v) => v.termination_date(),
            Self::TrafficArea(v) => v.termination_date(),
            Self::WaterGroundSurface(v) => v.termination_date(),
            Self::WaterSurface(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.valid_from(),
            Self::DoorSurface(v) => v.valid_from(),
            Self::FloorSurface(v) => v.valid_from(),
            Self::GroundSurface(v) => v.valid_from(),
            Self::InteriorWallSurface(v) => v.valid_from(),
            Self::OuterCeilingSurface(v) => v.valid_from(),
            Self::OuterFloorSurface(v) => v.valid_from(),
            Self::RoofSurface(v) => v.valid_from(),
            Self::WallSurface(v) => v.valid_from(),
            Self::WindowSurface(v) => v.valid_from(),
            Self::ClosureSurface(v) => v.valid_from(),
            Self::GenericThematicSurface(v) => v.valid_from(),
            Self::LandUse(v) => v.valid_from(),
            Self::BreaklineRelief(v) => v.valid_from(),
            Self::MassPointRelief(v) => v.valid_from(),
            Self::RasterRelief(v) => v.valid_from(),
            Self::ReliefFeature(v) => v.valid_from(),
            Self::TINRelief(v) => v.valid_from(),
            Self::AuxiliaryTrafficArea(v) => v.valid_from(),
            Self::HoleSurface(v) => v.valid_from(),
            Self::Marking(v) => v.valid_from(),
            Self::TrafficArea(v) => v.valid_from(),
            Self::WaterGroundSurface(v) => v.valid_from(),
            Self::WaterSurface(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.valid_to(),
            Self::DoorSurface(v) => v.valid_to(),
            Self::FloorSurface(v) => v.valid_to(),
            Self::GroundSurface(v) => v.valid_to(),
            Self::InteriorWallSurface(v) => v.valid_to(),
            Self::OuterCeilingSurface(v) => v.valid_to(),
            Self::OuterFloorSurface(v) => v.valid_to(),
            Self::RoofSurface(v) => v.valid_to(),
            Self::WallSurface(v) => v.valid_to(),
            Self::WindowSurface(v) => v.valid_to(),
            Self::ClosureSurface(v) => v.valid_to(),
            Self::GenericThematicSurface(v) => v.valid_to(),
            Self::LandUse(v) => v.valid_to(),
            Self::BreaklineRelief(v) => v.valid_to(),
            Self::MassPointRelief(v) => v.valid_to(),
            Self::RasterRelief(v) => v.valid_to(),
            Self::ReliefFeature(v) => v.valid_to(),
            Self::TINRelief(v) => v.valid_to(),
            Self::AuxiliaryTrafficArea(v) => v.valid_to(),
            Self::HoleSurface(v) => v.valid_to(),
            Self::Marking(v) => v.valid_to(),
            Self::TrafficArea(v) => v.valid_to(),
            Self::WaterGroundSurface(v) => v.valid_to(),
            Self::WaterSurface(v) => v.valid_to(),
        }
    }
}
impl AbstractCityObjectTrait for AbstractSpaceBoundary {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::CeilingSurface(v) => v.relative_to_terrain(),
            Self::DoorSurface(v) => v.relative_to_terrain(),
            Self::FloorSurface(v) => v.relative_to_terrain(),
            Self::GroundSurface(v) => v.relative_to_terrain(),
            Self::InteriorWallSurface(v) => v.relative_to_terrain(),
            Self::OuterCeilingSurface(v) => v.relative_to_terrain(),
            Self::OuterFloorSurface(v) => v.relative_to_terrain(),
            Self::RoofSurface(v) => v.relative_to_terrain(),
            Self::WallSurface(v) => v.relative_to_terrain(),
            Self::WindowSurface(v) => v.relative_to_terrain(),
            Self::ClosureSurface(v) => v.relative_to_terrain(),
            Self::GenericThematicSurface(v) => v.relative_to_terrain(),
            Self::LandUse(v) => v.relative_to_terrain(),
            Self::BreaklineRelief(v) => v.relative_to_terrain(),
            Self::MassPointRelief(v) => v.relative_to_terrain(),
            Self::RasterRelief(v) => v.relative_to_terrain(),
            Self::ReliefFeature(v) => v.relative_to_terrain(),
            Self::TINRelief(v) => v.relative_to_terrain(),
            Self::AuxiliaryTrafficArea(v) => v.relative_to_terrain(),
            Self::HoleSurface(v) => v.relative_to_terrain(),
            Self::Marking(v) => v.relative_to_terrain(),
            Self::TrafficArea(v) => v.relative_to_terrain(),
            Self::WaterGroundSurface(v) => v.relative_to_terrain(),
            Self::WaterSurface(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::CeilingSurface(v) => v.relative_to_water(),
            Self::DoorSurface(v) => v.relative_to_water(),
            Self::FloorSurface(v) => v.relative_to_water(),
            Self::GroundSurface(v) => v.relative_to_water(),
            Self::InteriorWallSurface(v) => v.relative_to_water(),
            Self::OuterCeilingSurface(v) => v.relative_to_water(),
            Self::OuterFloorSurface(v) => v.relative_to_water(),
            Self::RoofSurface(v) => v.relative_to_water(),
            Self::WallSurface(v) => v.relative_to_water(),
            Self::WindowSurface(v) => v.relative_to_water(),
            Self::ClosureSurface(v) => v.relative_to_water(),
            Self::GenericThematicSurface(v) => v.relative_to_water(),
            Self::LandUse(v) => v.relative_to_water(),
            Self::BreaklineRelief(v) => v.relative_to_water(),
            Self::MassPointRelief(v) => v.relative_to_water(),
            Self::RasterRelief(v) => v.relative_to_water(),
            Self::ReliefFeature(v) => v.relative_to_water(),
            Self::TINRelief(v) => v.relative_to_water(),
            Self::AuxiliaryTrafficArea(v) => v.relative_to_water(),
            Self::HoleSurface(v) => v.relative_to_water(),
            Self::Marking(v) => v.relative_to_water(),
            Self::TrafficArea(v) => v.relative_to_water(),
            Self::WaterGroundSurface(v) => v.relative_to_water(),
            Self::WaterSurface(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::CeilingSurface(v) => v.appearance(),
            Self::DoorSurface(v) => v.appearance(),
            Self::FloorSurface(v) => v.appearance(),
            Self::GroundSurface(v) => v.appearance(),
            Self::InteriorWallSurface(v) => v.appearance(),
            Self::OuterCeilingSurface(v) => v.appearance(),
            Self::OuterFloorSurface(v) => v.appearance(),
            Self::RoofSurface(v) => v.appearance(),
            Self::WallSurface(v) => v.appearance(),
            Self::WindowSurface(v) => v.appearance(),
            Self::ClosureSurface(v) => v.appearance(),
            Self::GenericThematicSurface(v) => v.appearance(),
            Self::LandUse(v) => v.appearance(),
            Self::BreaklineRelief(v) => v.appearance(),
            Self::MassPointRelief(v) => v.appearance(),
            Self::RasterRelief(v) => v.appearance(),
            Self::ReliefFeature(v) => v.appearance(),
            Self::TINRelief(v) => v.appearance(),
            Self::AuxiliaryTrafficArea(v) => v.appearance(),
            Self::HoleSurface(v) => v.appearance(),
            Self::Marking(v) => v.appearance(),
            Self::TrafficArea(v) => v.appearance(),
            Self::WaterGroundSurface(v) => v.appearance(),
            Self::WaterSurface(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::CeilingSurface(v) => v.generalizes_to(),
            Self::DoorSurface(v) => v.generalizes_to(),
            Self::FloorSurface(v) => v.generalizes_to(),
            Self::GroundSurface(v) => v.generalizes_to(),
            Self::InteriorWallSurface(v) => v.generalizes_to(),
            Self::OuterCeilingSurface(v) => v.generalizes_to(),
            Self::OuterFloorSurface(v) => v.generalizes_to(),
            Self::RoofSurface(v) => v.generalizes_to(),
            Self::WallSurface(v) => v.generalizes_to(),
            Self::WindowSurface(v) => v.generalizes_to(),
            Self::ClosureSurface(v) => v.generalizes_to(),
            Self::GenericThematicSurface(v) => v.generalizes_to(),
            Self::LandUse(v) => v.generalizes_to(),
            Self::BreaklineRelief(v) => v.generalizes_to(),
            Self::MassPointRelief(v) => v.generalizes_to(),
            Self::RasterRelief(v) => v.generalizes_to(),
            Self::ReliefFeature(v) => v.generalizes_to(),
            Self::TINRelief(v) => v.generalizes_to(),
            Self::AuxiliaryTrafficArea(v) => v.generalizes_to(),
            Self::HoleSurface(v) => v.generalizes_to(),
            Self::Marking(v) => v.generalizes_to(),
            Self::TrafficArea(v) => v.generalizes_to(),
            Self::WaterGroundSurface(v) => v.generalizes_to(),
            Self::WaterSurface(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::CeilingSurface(v) => v.external_reference(),
            Self::DoorSurface(v) => v.external_reference(),
            Self::FloorSurface(v) => v.external_reference(),
            Self::GroundSurface(v) => v.external_reference(),
            Self::InteriorWallSurface(v) => v.external_reference(),
            Self::OuterCeilingSurface(v) => v.external_reference(),
            Self::OuterFloorSurface(v) => v.external_reference(),
            Self::RoofSurface(v) => v.external_reference(),
            Self::WallSurface(v) => v.external_reference(),
            Self::WindowSurface(v) => v.external_reference(),
            Self::ClosureSurface(v) => v.external_reference(),
            Self::GenericThematicSurface(v) => v.external_reference(),
            Self::LandUse(v) => v.external_reference(),
            Self::BreaklineRelief(v) => v.external_reference(),
            Self::MassPointRelief(v) => v.external_reference(),
            Self::RasterRelief(v) => v.external_reference(),
            Self::ReliefFeature(v) => v.external_reference(),
            Self::TINRelief(v) => v.external_reference(),
            Self::AuxiliaryTrafficArea(v) => v.external_reference(),
            Self::HoleSurface(v) => v.external_reference(),
            Self::Marking(v) => v.external_reference(),
            Self::TrafficArea(v) => v.external_reference(),
            Self::WaterGroundSurface(v) => v.external_reference(),
            Self::WaterSurface(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::CeilingSurface(v) => v.related_to(),
            Self::DoorSurface(v) => v.related_to(),
            Self::FloorSurface(v) => v.related_to(),
            Self::GroundSurface(v) => v.related_to(),
            Self::InteriorWallSurface(v) => v.related_to(),
            Self::OuterCeilingSurface(v) => v.related_to(),
            Self::OuterFloorSurface(v) => v.related_to(),
            Self::RoofSurface(v) => v.related_to(),
            Self::WallSurface(v) => v.related_to(),
            Self::WindowSurface(v) => v.related_to(),
            Self::ClosureSurface(v) => v.related_to(),
            Self::GenericThematicSurface(v) => v.related_to(),
            Self::LandUse(v) => v.related_to(),
            Self::BreaklineRelief(v) => v.related_to(),
            Self::MassPointRelief(v) => v.related_to(),
            Self::RasterRelief(v) => v.related_to(),
            Self::ReliefFeature(v) => v.related_to(),
            Self::TINRelief(v) => v.related_to(),
            Self::AuxiliaryTrafficArea(v) => v.related_to(),
            Self::HoleSurface(v) => v.related_to(),
            Self::Marking(v) => v.related_to(),
            Self::TrafficArea(v) => v.related_to(),
            Self::WaterGroundSurface(v) => v.related_to(),
            Self::WaterSurface(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::CeilingSurface(v) => v.dynamizer(),
            Self::DoorSurface(v) => v.dynamizer(),
            Self::FloorSurface(v) => v.dynamizer(),
            Self::GroundSurface(v) => v.dynamizer(),
            Self::InteriorWallSurface(v) => v.dynamizer(),
            Self::OuterCeilingSurface(v) => v.dynamizer(),
            Self::OuterFloorSurface(v) => v.dynamizer(),
            Self::RoofSurface(v) => v.dynamizer(),
            Self::WallSurface(v) => v.dynamizer(),
            Self::WindowSurface(v) => v.dynamizer(),
            Self::ClosureSurface(v) => v.dynamizer(),
            Self::GenericThematicSurface(v) => v.dynamizer(),
            Self::LandUse(v) => v.dynamizer(),
            Self::BreaklineRelief(v) => v.dynamizer(),
            Self::MassPointRelief(v) => v.dynamizer(),
            Self::RasterRelief(v) => v.dynamizer(),
            Self::ReliefFeature(v) => v.dynamizer(),
            Self::TINRelief(v) => v.dynamizer(),
            Self::AuxiliaryTrafficArea(v) => v.dynamizer(),
            Self::HoleSurface(v) => v.dynamizer(),
            Self::Marking(v) => v.dynamizer(),
            Self::TrafficArea(v) => v.dynamizer(),
            Self::WaterGroundSurface(v) => v.dynamizer(),
            Self::WaterSurface(v) => v.dynamizer(),
        }
    }
}
impl AbstractSpaceBoundaryTrait for AbstractSpaceBoundary {}
impl From<CeilingSurface> for AbstractSpaceBoundary {
    fn from(v: CeilingSurface) -> Self {
        Self::CeilingSurface(Box::new(v))
    }
}
impl From<DoorSurface> for AbstractSpaceBoundary {
    fn from(v: DoorSurface) -> Self {
        Self::DoorSurface(Box::new(v))
    }
}
impl From<FloorSurface> for AbstractSpaceBoundary {
    fn from(v: FloorSurface) -> Self {
        Self::FloorSurface(Box::new(v))
    }
}
impl From<GroundSurface> for AbstractSpaceBoundary {
    fn from(v: GroundSurface) -> Self {
        Self::GroundSurface(Box::new(v))
    }
}
impl From<InteriorWallSurface> for AbstractSpaceBoundary {
    fn from(v: InteriorWallSurface) -> Self {
        Self::InteriorWallSurface(Box::new(v))
    }
}
impl From<OuterCeilingSurface> for AbstractSpaceBoundary {
    fn from(v: OuterCeilingSurface) -> Self {
        Self::OuterCeilingSurface(Box::new(v))
    }
}
impl From<OuterFloorSurface> for AbstractSpaceBoundary {
    fn from(v: OuterFloorSurface) -> Self {
        Self::OuterFloorSurface(Box::new(v))
    }
}
impl From<RoofSurface> for AbstractSpaceBoundary {
    fn from(v: RoofSurface) -> Self {
        Self::RoofSurface(Box::new(v))
    }
}
impl From<WallSurface> for AbstractSpaceBoundary {
    fn from(v: WallSurface) -> Self {
        Self::WallSurface(Box::new(v))
    }
}
impl From<WindowSurface> for AbstractSpaceBoundary {
    fn from(v: WindowSurface) -> Self {
        Self::WindowSurface(Box::new(v))
    }
}
impl From<ClosureSurface> for AbstractSpaceBoundary {
    fn from(v: ClosureSurface) -> Self {
        Self::ClosureSurface(Box::new(v))
    }
}
impl From<GenericThematicSurface> for AbstractSpaceBoundary {
    fn from(v: GenericThematicSurface) -> Self {
        Self::GenericThematicSurface(Box::new(v))
    }
}
impl From<LandUse> for AbstractSpaceBoundary {
    fn from(v: LandUse) -> Self {
        Self::LandUse(Box::new(v))
    }
}
impl From<BreaklineRelief> for AbstractSpaceBoundary {
    fn from(v: BreaklineRelief) -> Self {
        Self::BreaklineRelief(Box::new(v))
    }
}
impl From<MassPointRelief> for AbstractSpaceBoundary {
    fn from(v: MassPointRelief) -> Self {
        Self::MassPointRelief(Box::new(v))
    }
}
impl From<RasterRelief> for AbstractSpaceBoundary {
    fn from(v: RasterRelief) -> Self {
        Self::RasterRelief(Box::new(v))
    }
}
impl From<ReliefFeature> for AbstractSpaceBoundary {
    fn from(v: ReliefFeature) -> Self {
        Self::ReliefFeature(Box::new(v))
    }
}
impl From<TINRelief> for AbstractSpaceBoundary {
    fn from(v: TINRelief) -> Self {
        Self::TINRelief(Box::new(v))
    }
}
impl From<AuxiliaryTrafficArea> for AbstractSpaceBoundary {
    fn from(v: AuxiliaryTrafficArea) -> Self {
        Self::AuxiliaryTrafficArea(Box::new(v))
    }
}
impl From<HoleSurface> for AbstractSpaceBoundary {
    fn from(v: HoleSurface) -> Self {
        Self::HoleSurface(Box::new(v))
    }
}
impl From<Marking> for AbstractSpaceBoundary {
    fn from(v: Marking) -> Self {
        Self::Marking(Box::new(v))
    }
}
impl From<TrafficArea> for AbstractSpaceBoundary {
    fn from(v: TrafficArea) -> Self {
        Self::TrafficArea(Box::new(v))
    }
}
impl From<WaterGroundSurface> for AbstractSpaceBoundary {
    fn from(v: WaterGroundSurface) -> Self {
        Self::WaterGroundSurface(Box::new(v))
    }
}
impl From<WaterSurface> for AbstractSpaceBoundary {
    fn from(v: WaterSurface) -> Self {
        Self::WaterSurface(Box::new(v))
    }
}
pub trait AbstractSpaceBoundaryAccessors {
    fn ceiling_surfaces(&self) -> impl Iterator<Item = &CeilingSurface>;
    fn door_surfaces(&self) -> impl Iterator<Item = &DoorSurface>;
    fn floor_surfaces(&self) -> impl Iterator<Item = &FloorSurface>;
    fn ground_surfaces(&self) -> impl Iterator<Item = &GroundSurface>;
    fn interior_wall_surfaces(&self) -> impl Iterator<Item = &InteriorWallSurface>;
    fn outer_ceiling_surfaces(&self) -> impl Iterator<Item = &OuterCeilingSurface>;
    fn outer_floor_surfaces(&self) -> impl Iterator<Item = &OuterFloorSurface>;
    fn roof_surfaces(&self) -> impl Iterator<Item = &RoofSurface>;
    fn wall_surfaces(&self) -> impl Iterator<Item = &WallSurface>;
    fn window_surfaces(&self) -> impl Iterator<Item = &WindowSurface>;
    fn closure_surfaces(&self) -> impl Iterator<Item = &ClosureSurface>;
    fn generic_thematic_surfaces(&self) -> impl Iterator<Item = &GenericThematicSurface>;
    fn land_uses(&self) -> impl Iterator<Item = &LandUse>;
    fn breakline_reliefs(&self) -> impl Iterator<Item = &BreaklineRelief>;
    fn mass_point_reliefs(&self) -> impl Iterator<Item = &MassPointRelief>;
    fn raster_reliefs(&self) -> impl Iterator<Item = &RasterRelief>;
    fn relief_features(&self) -> impl Iterator<Item = &ReliefFeature>;
    fn tin_reliefs(&self) -> impl Iterator<Item = &TINRelief>;
    fn auxiliary_traffic_areas(&self) -> impl Iterator<Item = &AuxiliaryTrafficArea>;
    fn hole_surfaces(&self) -> impl Iterator<Item = &HoleSurface>;
    fn markings(&self) -> impl Iterator<Item = &Marking>;
    fn traffic_areas(&self) -> impl Iterator<Item = &TrafficArea>;
    fn water_ground_surfaces(&self) -> impl Iterator<Item = &WaterGroundSurface>;
    fn water_surfaces(&self) -> impl Iterator<Item = &WaterSurface>;
}
impl AbstractSpaceBoundaryAccessors for [AbstractSpaceBoundary] {
    fn ceiling_surfaces(&self) -> impl Iterator<Item = &CeilingSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::CeilingSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn door_surfaces(&self) -> impl Iterator<Item = &DoorSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::DoorSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn floor_surfaces(&self) -> impl Iterator<Item = &FloorSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::FloorSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn ground_surfaces(&self) -> impl Iterator<Item = &GroundSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::GroundSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn interior_wall_surfaces(&self) -> impl Iterator<Item = &InteriorWallSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::InteriorWallSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn outer_ceiling_surfaces(&self) -> impl Iterator<Item = &OuterCeilingSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::OuterCeilingSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn outer_floor_surfaces(&self) -> impl Iterator<Item = &OuterFloorSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::OuterFloorSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn roof_surfaces(&self) -> impl Iterator<Item = &RoofSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::RoofSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn wall_surfaces(&self) -> impl Iterator<Item = &WallSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::WallSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn window_surfaces(&self) -> impl Iterator<Item = &WindowSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::WindowSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn closure_surfaces(&self) -> impl Iterator<Item = &ClosureSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::ClosureSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_thematic_surfaces(
        &self,
    ) -> impl Iterator<Item = &GenericThematicSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::GenericThematicSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn land_uses(&self) -> impl Iterator<Item = &LandUse> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::LandUse(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn breakline_reliefs(&self) -> impl Iterator<Item = &BreaklineRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::BreaklineRelief(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn mass_point_reliefs(&self) -> impl Iterator<Item = &MassPointRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::MassPointRelief(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn raster_reliefs(&self) -> impl Iterator<Item = &RasterRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::RasterRelief(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn relief_features(&self) -> impl Iterator<Item = &ReliefFeature> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::ReliefFeature(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tin_reliefs(&self) -> impl Iterator<Item = &TINRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::TINRelief(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn auxiliary_traffic_areas(&self) -> impl Iterator<Item = &AuxiliaryTrafficArea> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::AuxiliaryTrafficArea(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn hole_surfaces(&self) -> impl Iterator<Item = &HoleSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::HoleSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn markings(&self) -> impl Iterator<Item = &Marking> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::Marking(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn traffic_areas(&self) -> impl Iterator<Item = &TrafficArea> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::TrafficArea(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn water_ground_surfaces(&self) -> impl Iterator<Item = &WaterGroundSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::WaterGroundSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn water_surfaces(&self) -> impl Iterator<Item = &WaterSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractSpaceBoundary::WaterSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
}
pub trait AbstractLogicalSpaceTrait: AbstractSpaceTrait {}
#[derive(Debug, Clone)]
pub enum AbstractLogicalSpace {
    BuildingUnit(BuildingUnit),
    Storey(Storey),
    CityObjectGroup(CityObjectGroup),
    GenericLogicalSpace(GenericLogicalSpace),
}
impl Default for AbstractLogicalSpace {
    fn default() -> Self {
        Self::BuildingUnit(Default::default())
    }
}
impl AbstractFeatureTrait for AbstractLogicalSpace {
    fn feature_id(&self) -> &ID {
        match self {
            Self::BuildingUnit(v) => v.feature_id(),
            Self::Storey(v) => v.feature_id(),
            Self::CityObjectGroup(v) => v.feature_id(),
            Self::GenericLogicalSpace(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::BuildingUnit(v) => v.identifier(),
            Self::Storey(v) => v.identifier(),
            Self::CityObjectGroup(v) => v.identifier(),
            Self::GenericLogicalSpace(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::BuildingUnit(v) => v.name(),
            Self::Storey(v) => v.name(),
            Self::CityObjectGroup(v) => v.name(),
            Self::GenericLogicalSpace(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::BuildingUnit(v) => v.description(),
            Self::Storey(v) => v.description(),
            Self::CityObjectGroup(v) => v.description(),
            Self::GenericLogicalSpace(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractLogicalSpace {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::BuildingUnit(v) => v.creation_date(),
            Self::Storey(v) => v.creation_date(),
            Self::CityObjectGroup(v) => v.creation_date(),
            Self::GenericLogicalSpace(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::BuildingUnit(v) => v.termination_date(),
            Self::Storey(v) => v.termination_date(),
            Self::CityObjectGroup(v) => v.termination_date(),
            Self::GenericLogicalSpace(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::BuildingUnit(v) => v.valid_from(),
            Self::Storey(v) => v.valid_from(),
            Self::CityObjectGroup(v) => v.valid_from(),
            Self::GenericLogicalSpace(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::BuildingUnit(v) => v.valid_to(),
            Self::Storey(v) => v.valid_to(),
            Self::CityObjectGroup(v) => v.valid_to(),
            Self::GenericLogicalSpace(v) => v.valid_to(),
        }
    }
}
impl AbstractCityObjectTrait for AbstractLogicalSpace {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::BuildingUnit(v) => v.relative_to_terrain(),
            Self::Storey(v) => v.relative_to_terrain(),
            Self::CityObjectGroup(v) => v.relative_to_terrain(),
            Self::GenericLogicalSpace(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::BuildingUnit(v) => v.relative_to_water(),
            Self::Storey(v) => v.relative_to_water(),
            Self::CityObjectGroup(v) => v.relative_to_water(),
            Self::GenericLogicalSpace(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::BuildingUnit(v) => v.appearance(),
            Self::Storey(v) => v.appearance(),
            Self::CityObjectGroup(v) => v.appearance(),
            Self::GenericLogicalSpace(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::BuildingUnit(v) => v.generalizes_to(),
            Self::Storey(v) => v.generalizes_to(),
            Self::CityObjectGroup(v) => v.generalizes_to(),
            Self::GenericLogicalSpace(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::BuildingUnit(v) => v.external_reference(),
            Self::Storey(v) => v.external_reference(),
            Self::CityObjectGroup(v) => v.external_reference(),
            Self::GenericLogicalSpace(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::BuildingUnit(v) => v.related_to(),
            Self::Storey(v) => v.related_to(),
            Self::CityObjectGroup(v) => v.related_to(),
            Self::GenericLogicalSpace(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::BuildingUnit(v) => v.dynamizer(),
            Self::Storey(v) => v.dynamizer(),
            Self::CityObjectGroup(v) => v.dynamizer(),
            Self::GenericLogicalSpace(v) => v.dynamizer(),
        }
    }
}
impl AbstractSpaceTrait for AbstractLogicalSpace {
    fn space_type(&self) -> Option<SpaceType> {
        match self {
            Self::BuildingUnit(v) => v.space_type(),
            Self::Storey(v) => v.space_type(),
            Self::CityObjectGroup(v) => v.space_type(),
            Self::GenericLogicalSpace(v) => v.space_type(),
        }
    }
    fn volume(&self) -> &[QualifiedVolume] {
        match self {
            Self::BuildingUnit(v) => v.volume(),
            Self::Storey(v) => v.volume(),
            Self::CityObjectGroup(v) => v.volume(),
            Self::GenericLogicalSpace(v) => v.volume(),
        }
    }
    fn area(&self) -> &[QualifiedArea] {
        match self {
            Self::BuildingUnit(v) => v.area(),
            Self::Storey(v) => v.area(),
            Self::CityObjectGroup(v) => v.area(),
            Self::GenericLogicalSpace(v) => v.area(),
        }
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BuildingUnit(v) => v.lod2_multi_curve(),
            Self::Storey(v) => v.lod2_multi_curve(),
            Self::CityObjectGroup(v) => v.lod2_multi_curve(),
            Self::GenericLogicalSpace(v) => v.lod2_multi_curve(),
        }
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::BuildingUnit(v) => v.lod3_multi_surface(),
            Self::Storey(v) => v.lod3_multi_surface(),
            Self::CityObjectGroup(v) => v.lod3_multi_surface(),
            Self::GenericLogicalSpace(v) => v.lod3_multi_surface(),
        }
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::BuildingUnit(v) => v.lod0_multi_surface(),
            Self::Storey(v) => v.lod0_multi_surface(),
            Self::CityObjectGroup(v) => v.lod0_multi_surface(),
            Self::GenericLogicalSpace(v) => v.lod0_multi_surface(),
        }
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::BuildingUnit(v) => v.lod1_solid(),
            Self::Storey(v) => v.lod1_solid(),
            Self::CityObjectGroup(v) => v.lod1_solid(),
            Self::GenericLogicalSpace(v) => v.lod1_solid(),
        }
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::BuildingUnit(v) => v.lod3_solid(),
            Self::Storey(v) => v.lod3_solid(),
            Self::CityObjectGroup(v) => v.lod3_solid(),
            Self::GenericLogicalSpace(v) => v.lod3_solid(),
        }
    }
    fn boundary(&self) -> &[AbstractSpaceBoundary] {
        match self {
            Self::BuildingUnit(v) => v.boundary(),
            Self::Storey(v) => v.boundary(),
            Self::CityObjectGroup(v) => v.boundary(),
            Self::GenericLogicalSpace(v) => v.boundary(),
        }
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BuildingUnit(v) => v.lod0_multi_curve(),
            Self::Storey(v) => v.lod0_multi_curve(),
            Self::CityObjectGroup(v) => v.lod0_multi_curve(),
            Self::GenericLogicalSpace(v) => v.lod0_multi_curve(),
        }
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::BuildingUnit(v) => v.lod2_solid(),
            Self::Storey(v) => v.lod2_solid(),
            Self::CityObjectGroup(v) => v.lod2_solid(),
            Self::GenericLogicalSpace(v) => v.lod2_solid(),
        }
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        match self {
            Self::BuildingUnit(v) => v.lod0_point(),
            Self::Storey(v) => v.lod0_point(),
            Self::CityObjectGroup(v) => v.lod0_point(),
            Self::GenericLogicalSpace(v) => v.lod0_point(),
        }
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BuildingUnit(v) => v.lod3_multi_curve(),
            Self::Storey(v) => v.lod3_multi_curve(),
            Self::CityObjectGroup(v) => v.lod3_multi_curve(),
            Self::GenericLogicalSpace(v) => v.lod3_multi_curve(),
        }
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::BuildingUnit(v) => v.lod2_multi_surface(),
            Self::Storey(v) => v.lod2_multi_surface(),
            Self::CityObjectGroup(v) => v.lod2_multi_surface(),
            Self::GenericLogicalSpace(v) => v.lod2_multi_surface(),
        }
    }
}
impl AbstractLogicalSpaceTrait for AbstractLogicalSpace {}
impl From<BuildingUnit> for AbstractLogicalSpace {
    fn from(v: BuildingUnit) -> Self {
        Self::BuildingUnit(v)
    }
}
impl From<Storey> for AbstractLogicalSpace {
    fn from(v: Storey) -> Self {
        Self::Storey(v)
    }
}
impl From<CityObjectGroup> for AbstractLogicalSpace {
    fn from(v: CityObjectGroup) -> Self {
        Self::CityObjectGroup(v)
    }
}
impl From<GenericLogicalSpace> for AbstractLogicalSpace {
    fn from(v: GenericLogicalSpace) -> Self {
        Self::GenericLogicalSpace(v)
    }
}
pub trait AbstractLogicalSpaceAccessors {
    fn building_units(&self) -> impl Iterator<Item = &BuildingUnit>;
    fn storeys(&self) -> impl Iterator<Item = &Storey>;
    fn city_object_groups(&self) -> impl Iterator<Item = &CityObjectGroup>;
    fn generic_logical_spaces(&self) -> impl Iterator<Item = &GenericLogicalSpace>;
}
impl AbstractLogicalSpaceAccessors for [AbstractLogicalSpace] {
    fn building_units(&self) -> impl Iterator<Item = &BuildingUnit> {
        self.iter()
            .filter_map(|item| match item {
                AbstractLogicalSpace::BuildingUnit(v) => Some(v),
                _ => None,
            })
    }
    fn storeys(&self) -> impl Iterator<Item = &Storey> {
        self.iter()
            .filter_map(|item| match item {
                AbstractLogicalSpace::Storey(v) => Some(v),
                _ => None,
            })
    }
    fn city_object_groups(&self) -> impl Iterator<Item = &CityObjectGroup> {
        self.iter()
            .filter_map(|item| match item {
                AbstractLogicalSpace::CityObjectGroup(v) => Some(v),
                _ => None,
            })
    }
    fn generic_logical_spaces(&self) -> impl Iterator<Item = &GenericLogicalSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractLogicalSpace::GenericLogicalSpace(v) => Some(v),
                _ => None,
            })
    }
}
pub trait AbstractPhysicalSpaceTrait: AbstractSpaceTrait {
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve>;
    fn point_cloud(&self) -> Option<&AbstractPointCloud>;
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve>;
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve>;
}
#[derive(Debug, Clone)]
pub enum AbstractPhysicalSpace {
    Door(Box<Door>),
    OtherConstruction(Box<OtherConstruction>),
    Window(Box<Window>),
    Bridge(Box<Bridge>),
    BridgeConstructiveElement(Box<BridgeConstructiveElement>),
    BridgeFurniture(Box<BridgeFurniture>),
    BridgeInstallation(Box<BridgeInstallation>),
    BridgePart(Box<BridgePart>),
    BridgeRoom(Box<BridgeRoom>),
    Building(Box<Building>),
    BuildingConstructiveElement(Box<BuildingConstructiveElement>),
    BuildingFurniture(Box<BuildingFurniture>),
    BuildingInstallation(Box<BuildingInstallation>),
    BuildingPart(Box<BuildingPart>),
    BuildingRoom(Box<BuildingRoom>),
    CityFurniture(Box<CityFurniture>),
    GenericOccupiedSpace(Box<GenericOccupiedSpace>),
    GenericUnoccupiedSpace(Box<GenericUnoccupiedSpace>),
    AuxiliaryTrafficSpace(Box<AuxiliaryTrafficSpace>),
    ClearanceSpace(Box<ClearanceSpace>),
    Hole(Box<Hole>),
    Intersection(Box<Intersection>),
    Railway(Box<Railway>),
    Road(Box<Road>),
    Section(Box<Section>),
    Square(Box<Square>),
    Track(Box<Track>),
    TrafficSpace(Box<TrafficSpace>),
    Waterway(Box<Waterway>),
    HollowSpace(Box<HollowSpace>),
    Tunnel(Box<Tunnel>),
    TunnelConstructiveElement(Box<TunnelConstructiveElement>),
    TunnelFurniture(Box<TunnelFurniture>),
    TunnelInstallation(Box<TunnelInstallation>),
    TunnelPart(Box<TunnelPart>),
    PlantCover(Box<PlantCover>),
    SolitaryVegetationObject(Box<SolitaryVegetationObject>),
    WaterBody(Box<WaterBody>),
}
impl Default for AbstractPhysicalSpace {
    fn default() -> Self {
        Self::Door(Box::default())
    }
}
impl AbstractFeatureTrait for AbstractPhysicalSpace {
    fn feature_id(&self) -> &ID {
        match self {
            Self::Door(v) => v.feature_id(),
            Self::OtherConstruction(v) => v.feature_id(),
            Self::Window(v) => v.feature_id(),
            Self::Bridge(v) => v.feature_id(),
            Self::BridgeConstructiveElement(v) => v.feature_id(),
            Self::BridgeFurniture(v) => v.feature_id(),
            Self::BridgeInstallation(v) => v.feature_id(),
            Self::BridgePart(v) => v.feature_id(),
            Self::BridgeRoom(v) => v.feature_id(),
            Self::Building(v) => v.feature_id(),
            Self::BuildingConstructiveElement(v) => v.feature_id(),
            Self::BuildingFurniture(v) => v.feature_id(),
            Self::BuildingInstallation(v) => v.feature_id(),
            Self::BuildingPart(v) => v.feature_id(),
            Self::BuildingRoom(v) => v.feature_id(),
            Self::CityFurniture(v) => v.feature_id(),
            Self::GenericOccupiedSpace(v) => v.feature_id(),
            Self::GenericUnoccupiedSpace(v) => v.feature_id(),
            Self::AuxiliaryTrafficSpace(v) => v.feature_id(),
            Self::ClearanceSpace(v) => v.feature_id(),
            Self::Hole(v) => v.feature_id(),
            Self::Intersection(v) => v.feature_id(),
            Self::Railway(v) => v.feature_id(),
            Self::Road(v) => v.feature_id(),
            Self::Section(v) => v.feature_id(),
            Self::Square(v) => v.feature_id(),
            Self::Track(v) => v.feature_id(),
            Self::TrafficSpace(v) => v.feature_id(),
            Self::Waterway(v) => v.feature_id(),
            Self::HollowSpace(v) => v.feature_id(),
            Self::Tunnel(v) => v.feature_id(),
            Self::TunnelConstructiveElement(v) => v.feature_id(),
            Self::TunnelFurniture(v) => v.feature_id(),
            Self::TunnelInstallation(v) => v.feature_id(),
            Self::TunnelPart(v) => v.feature_id(),
            Self::PlantCover(v) => v.feature_id(),
            Self::SolitaryVegetationObject(v) => v.feature_id(),
            Self::WaterBody(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.identifier(),
            Self::OtherConstruction(v) => v.identifier(),
            Self::Window(v) => v.identifier(),
            Self::Bridge(v) => v.identifier(),
            Self::BridgeConstructiveElement(v) => v.identifier(),
            Self::BridgeFurniture(v) => v.identifier(),
            Self::BridgeInstallation(v) => v.identifier(),
            Self::BridgePart(v) => v.identifier(),
            Self::BridgeRoom(v) => v.identifier(),
            Self::Building(v) => v.identifier(),
            Self::BuildingConstructiveElement(v) => v.identifier(),
            Self::BuildingFurniture(v) => v.identifier(),
            Self::BuildingInstallation(v) => v.identifier(),
            Self::BuildingPart(v) => v.identifier(),
            Self::BuildingRoom(v) => v.identifier(),
            Self::CityFurniture(v) => v.identifier(),
            Self::GenericOccupiedSpace(v) => v.identifier(),
            Self::GenericUnoccupiedSpace(v) => v.identifier(),
            Self::AuxiliaryTrafficSpace(v) => v.identifier(),
            Self::ClearanceSpace(v) => v.identifier(),
            Self::Hole(v) => v.identifier(),
            Self::Intersection(v) => v.identifier(),
            Self::Railway(v) => v.identifier(),
            Self::Road(v) => v.identifier(),
            Self::Section(v) => v.identifier(),
            Self::Square(v) => v.identifier(),
            Self::Track(v) => v.identifier(),
            Self::TrafficSpace(v) => v.identifier(),
            Self::Waterway(v) => v.identifier(),
            Self::HollowSpace(v) => v.identifier(),
            Self::Tunnel(v) => v.identifier(),
            Self::TunnelConstructiveElement(v) => v.identifier(),
            Self::TunnelFurniture(v) => v.identifier(),
            Self::TunnelInstallation(v) => v.identifier(),
            Self::TunnelPart(v) => v.identifier(),
            Self::PlantCover(v) => v.identifier(),
            Self::SolitaryVegetationObject(v) => v.identifier(),
            Self::WaterBody(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::Door(v) => v.name(),
            Self::OtherConstruction(v) => v.name(),
            Self::Window(v) => v.name(),
            Self::Bridge(v) => v.name(),
            Self::BridgeConstructiveElement(v) => v.name(),
            Self::BridgeFurniture(v) => v.name(),
            Self::BridgeInstallation(v) => v.name(),
            Self::BridgePart(v) => v.name(),
            Self::BridgeRoom(v) => v.name(),
            Self::Building(v) => v.name(),
            Self::BuildingConstructiveElement(v) => v.name(),
            Self::BuildingFurniture(v) => v.name(),
            Self::BuildingInstallation(v) => v.name(),
            Self::BuildingPart(v) => v.name(),
            Self::BuildingRoom(v) => v.name(),
            Self::CityFurniture(v) => v.name(),
            Self::GenericOccupiedSpace(v) => v.name(),
            Self::GenericUnoccupiedSpace(v) => v.name(),
            Self::AuxiliaryTrafficSpace(v) => v.name(),
            Self::ClearanceSpace(v) => v.name(),
            Self::Hole(v) => v.name(),
            Self::Intersection(v) => v.name(),
            Self::Railway(v) => v.name(),
            Self::Road(v) => v.name(),
            Self::Section(v) => v.name(),
            Self::Square(v) => v.name(),
            Self::Track(v) => v.name(),
            Self::TrafficSpace(v) => v.name(),
            Self::Waterway(v) => v.name(),
            Self::HollowSpace(v) => v.name(),
            Self::Tunnel(v) => v.name(),
            Self::TunnelConstructiveElement(v) => v.name(),
            Self::TunnelFurniture(v) => v.name(),
            Self::TunnelInstallation(v) => v.name(),
            Self::TunnelPart(v) => v.name(),
            Self::PlantCover(v) => v.name(),
            Self::SolitaryVegetationObject(v) => v.name(),
            Self::WaterBody(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.description(),
            Self::OtherConstruction(v) => v.description(),
            Self::Window(v) => v.description(),
            Self::Bridge(v) => v.description(),
            Self::BridgeConstructiveElement(v) => v.description(),
            Self::BridgeFurniture(v) => v.description(),
            Self::BridgeInstallation(v) => v.description(),
            Self::BridgePart(v) => v.description(),
            Self::BridgeRoom(v) => v.description(),
            Self::Building(v) => v.description(),
            Self::BuildingConstructiveElement(v) => v.description(),
            Self::BuildingFurniture(v) => v.description(),
            Self::BuildingInstallation(v) => v.description(),
            Self::BuildingPart(v) => v.description(),
            Self::BuildingRoom(v) => v.description(),
            Self::CityFurniture(v) => v.description(),
            Self::GenericOccupiedSpace(v) => v.description(),
            Self::GenericUnoccupiedSpace(v) => v.description(),
            Self::AuxiliaryTrafficSpace(v) => v.description(),
            Self::ClearanceSpace(v) => v.description(),
            Self::Hole(v) => v.description(),
            Self::Intersection(v) => v.description(),
            Self::Railway(v) => v.description(),
            Self::Road(v) => v.description(),
            Self::Section(v) => v.description(),
            Self::Square(v) => v.description(),
            Self::Track(v) => v.description(),
            Self::TrafficSpace(v) => v.description(),
            Self::Waterway(v) => v.description(),
            Self::HollowSpace(v) => v.description(),
            Self::Tunnel(v) => v.description(),
            Self::TunnelConstructiveElement(v) => v.description(),
            Self::TunnelFurniture(v) => v.description(),
            Self::TunnelInstallation(v) => v.description(),
            Self::TunnelPart(v) => v.description(),
            Self::PlantCover(v) => v.description(),
            Self::SolitaryVegetationObject(v) => v.description(),
            Self::WaterBody(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractPhysicalSpace {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.creation_date(),
            Self::OtherConstruction(v) => v.creation_date(),
            Self::Window(v) => v.creation_date(),
            Self::Bridge(v) => v.creation_date(),
            Self::BridgeConstructiveElement(v) => v.creation_date(),
            Self::BridgeFurniture(v) => v.creation_date(),
            Self::BridgeInstallation(v) => v.creation_date(),
            Self::BridgePart(v) => v.creation_date(),
            Self::BridgeRoom(v) => v.creation_date(),
            Self::Building(v) => v.creation_date(),
            Self::BuildingConstructiveElement(v) => v.creation_date(),
            Self::BuildingFurniture(v) => v.creation_date(),
            Self::BuildingInstallation(v) => v.creation_date(),
            Self::BuildingPart(v) => v.creation_date(),
            Self::BuildingRoom(v) => v.creation_date(),
            Self::CityFurniture(v) => v.creation_date(),
            Self::GenericOccupiedSpace(v) => v.creation_date(),
            Self::GenericUnoccupiedSpace(v) => v.creation_date(),
            Self::AuxiliaryTrafficSpace(v) => v.creation_date(),
            Self::ClearanceSpace(v) => v.creation_date(),
            Self::Hole(v) => v.creation_date(),
            Self::Intersection(v) => v.creation_date(),
            Self::Railway(v) => v.creation_date(),
            Self::Road(v) => v.creation_date(),
            Self::Section(v) => v.creation_date(),
            Self::Square(v) => v.creation_date(),
            Self::Track(v) => v.creation_date(),
            Self::TrafficSpace(v) => v.creation_date(),
            Self::Waterway(v) => v.creation_date(),
            Self::HollowSpace(v) => v.creation_date(),
            Self::Tunnel(v) => v.creation_date(),
            Self::TunnelConstructiveElement(v) => v.creation_date(),
            Self::TunnelFurniture(v) => v.creation_date(),
            Self::TunnelInstallation(v) => v.creation_date(),
            Self::TunnelPart(v) => v.creation_date(),
            Self::PlantCover(v) => v.creation_date(),
            Self::SolitaryVegetationObject(v) => v.creation_date(),
            Self::WaterBody(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.termination_date(),
            Self::OtherConstruction(v) => v.termination_date(),
            Self::Window(v) => v.termination_date(),
            Self::Bridge(v) => v.termination_date(),
            Self::BridgeConstructiveElement(v) => v.termination_date(),
            Self::BridgeFurniture(v) => v.termination_date(),
            Self::BridgeInstallation(v) => v.termination_date(),
            Self::BridgePart(v) => v.termination_date(),
            Self::BridgeRoom(v) => v.termination_date(),
            Self::Building(v) => v.termination_date(),
            Self::BuildingConstructiveElement(v) => v.termination_date(),
            Self::BuildingFurniture(v) => v.termination_date(),
            Self::BuildingInstallation(v) => v.termination_date(),
            Self::BuildingPart(v) => v.termination_date(),
            Self::BuildingRoom(v) => v.termination_date(),
            Self::CityFurniture(v) => v.termination_date(),
            Self::GenericOccupiedSpace(v) => v.termination_date(),
            Self::GenericUnoccupiedSpace(v) => v.termination_date(),
            Self::AuxiliaryTrafficSpace(v) => v.termination_date(),
            Self::ClearanceSpace(v) => v.termination_date(),
            Self::Hole(v) => v.termination_date(),
            Self::Intersection(v) => v.termination_date(),
            Self::Railway(v) => v.termination_date(),
            Self::Road(v) => v.termination_date(),
            Self::Section(v) => v.termination_date(),
            Self::Square(v) => v.termination_date(),
            Self::Track(v) => v.termination_date(),
            Self::TrafficSpace(v) => v.termination_date(),
            Self::Waterway(v) => v.termination_date(),
            Self::HollowSpace(v) => v.termination_date(),
            Self::Tunnel(v) => v.termination_date(),
            Self::TunnelConstructiveElement(v) => v.termination_date(),
            Self::TunnelFurniture(v) => v.termination_date(),
            Self::TunnelInstallation(v) => v.termination_date(),
            Self::TunnelPart(v) => v.termination_date(),
            Self::PlantCover(v) => v.termination_date(),
            Self::SolitaryVegetationObject(v) => v.termination_date(),
            Self::WaterBody(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.valid_from(),
            Self::OtherConstruction(v) => v.valid_from(),
            Self::Window(v) => v.valid_from(),
            Self::Bridge(v) => v.valid_from(),
            Self::BridgeConstructiveElement(v) => v.valid_from(),
            Self::BridgeFurniture(v) => v.valid_from(),
            Self::BridgeInstallation(v) => v.valid_from(),
            Self::BridgePart(v) => v.valid_from(),
            Self::BridgeRoom(v) => v.valid_from(),
            Self::Building(v) => v.valid_from(),
            Self::BuildingConstructiveElement(v) => v.valid_from(),
            Self::BuildingFurniture(v) => v.valid_from(),
            Self::BuildingInstallation(v) => v.valid_from(),
            Self::BuildingPart(v) => v.valid_from(),
            Self::BuildingRoom(v) => v.valid_from(),
            Self::CityFurniture(v) => v.valid_from(),
            Self::GenericOccupiedSpace(v) => v.valid_from(),
            Self::GenericUnoccupiedSpace(v) => v.valid_from(),
            Self::AuxiliaryTrafficSpace(v) => v.valid_from(),
            Self::ClearanceSpace(v) => v.valid_from(),
            Self::Hole(v) => v.valid_from(),
            Self::Intersection(v) => v.valid_from(),
            Self::Railway(v) => v.valid_from(),
            Self::Road(v) => v.valid_from(),
            Self::Section(v) => v.valid_from(),
            Self::Square(v) => v.valid_from(),
            Self::Track(v) => v.valid_from(),
            Self::TrafficSpace(v) => v.valid_from(),
            Self::Waterway(v) => v.valid_from(),
            Self::HollowSpace(v) => v.valid_from(),
            Self::Tunnel(v) => v.valid_from(),
            Self::TunnelConstructiveElement(v) => v.valid_from(),
            Self::TunnelFurniture(v) => v.valid_from(),
            Self::TunnelInstallation(v) => v.valid_from(),
            Self::TunnelPart(v) => v.valid_from(),
            Self::PlantCover(v) => v.valid_from(),
            Self::SolitaryVegetationObject(v) => v.valid_from(),
            Self::WaterBody(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.valid_to(),
            Self::OtherConstruction(v) => v.valid_to(),
            Self::Window(v) => v.valid_to(),
            Self::Bridge(v) => v.valid_to(),
            Self::BridgeConstructiveElement(v) => v.valid_to(),
            Self::BridgeFurniture(v) => v.valid_to(),
            Self::BridgeInstallation(v) => v.valid_to(),
            Self::BridgePart(v) => v.valid_to(),
            Self::BridgeRoom(v) => v.valid_to(),
            Self::Building(v) => v.valid_to(),
            Self::BuildingConstructiveElement(v) => v.valid_to(),
            Self::BuildingFurniture(v) => v.valid_to(),
            Self::BuildingInstallation(v) => v.valid_to(),
            Self::BuildingPart(v) => v.valid_to(),
            Self::BuildingRoom(v) => v.valid_to(),
            Self::CityFurniture(v) => v.valid_to(),
            Self::GenericOccupiedSpace(v) => v.valid_to(),
            Self::GenericUnoccupiedSpace(v) => v.valid_to(),
            Self::AuxiliaryTrafficSpace(v) => v.valid_to(),
            Self::ClearanceSpace(v) => v.valid_to(),
            Self::Hole(v) => v.valid_to(),
            Self::Intersection(v) => v.valid_to(),
            Self::Railway(v) => v.valid_to(),
            Self::Road(v) => v.valid_to(),
            Self::Section(v) => v.valid_to(),
            Self::Square(v) => v.valid_to(),
            Self::Track(v) => v.valid_to(),
            Self::TrafficSpace(v) => v.valid_to(),
            Self::Waterway(v) => v.valid_to(),
            Self::HollowSpace(v) => v.valid_to(),
            Self::Tunnel(v) => v.valid_to(),
            Self::TunnelConstructiveElement(v) => v.valid_to(),
            Self::TunnelFurniture(v) => v.valid_to(),
            Self::TunnelInstallation(v) => v.valid_to(),
            Self::TunnelPart(v) => v.valid_to(),
            Self::PlantCover(v) => v.valid_to(),
            Self::SolitaryVegetationObject(v) => v.valid_to(),
            Self::WaterBody(v) => v.valid_to(),
        }
    }
}
impl AbstractCityObjectTrait for AbstractPhysicalSpace {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::Door(v) => v.relative_to_terrain(),
            Self::OtherConstruction(v) => v.relative_to_terrain(),
            Self::Window(v) => v.relative_to_terrain(),
            Self::Bridge(v) => v.relative_to_terrain(),
            Self::BridgeConstructiveElement(v) => v.relative_to_terrain(),
            Self::BridgeFurniture(v) => v.relative_to_terrain(),
            Self::BridgeInstallation(v) => v.relative_to_terrain(),
            Self::BridgePart(v) => v.relative_to_terrain(),
            Self::BridgeRoom(v) => v.relative_to_terrain(),
            Self::Building(v) => v.relative_to_terrain(),
            Self::BuildingConstructiveElement(v) => v.relative_to_terrain(),
            Self::BuildingFurniture(v) => v.relative_to_terrain(),
            Self::BuildingInstallation(v) => v.relative_to_terrain(),
            Self::BuildingPart(v) => v.relative_to_terrain(),
            Self::BuildingRoom(v) => v.relative_to_terrain(),
            Self::CityFurniture(v) => v.relative_to_terrain(),
            Self::GenericOccupiedSpace(v) => v.relative_to_terrain(),
            Self::GenericUnoccupiedSpace(v) => v.relative_to_terrain(),
            Self::AuxiliaryTrafficSpace(v) => v.relative_to_terrain(),
            Self::ClearanceSpace(v) => v.relative_to_terrain(),
            Self::Hole(v) => v.relative_to_terrain(),
            Self::Intersection(v) => v.relative_to_terrain(),
            Self::Railway(v) => v.relative_to_terrain(),
            Self::Road(v) => v.relative_to_terrain(),
            Self::Section(v) => v.relative_to_terrain(),
            Self::Square(v) => v.relative_to_terrain(),
            Self::Track(v) => v.relative_to_terrain(),
            Self::TrafficSpace(v) => v.relative_to_terrain(),
            Self::Waterway(v) => v.relative_to_terrain(),
            Self::HollowSpace(v) => v.relative_to_terrain(),
            Self::Tunnel(v) => v.relative_to_terrain(),
            Self::TunnelConstructiveElement(v) => v.relative_to_terrain(),
            Self::TunnelFurniture(v) => v.relative_to_terrain(),
            Self::TunnelInstallation(v) => v.relative_to_terrain(),
            Self::TunnelPart(v) => v.relative_to_terrain(),
            Self::PlantCover(v) => v.relative_to_terrain(),
            Self::SolitaryVegetationObject(v) => v.relative_to_terrain(),
            Self::WaterBody(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::Door(v) => v.relative_to_water(),
            Self::OtherConstruction(v) => v.relative_to_water(),
            Self::Window(v) => v.relative_to_water(),
            Self::Bridge(v) => v.relative_to_water(),
            Self::BridgeConstructiveElement(v) => v.relative_to_water(),
            Self::BridgeFurniture(v) => v.relative_to_water(),
            Self::BridgeInstallation(v) => v.relative_to_water(),
            Self::BridgePart(v) => v.relative_to_water(),
            Self::BridgeRoom(v) => v.relative_to_water(),
            Self::Building(v) => v.relative_to_water(),
            Self::BuildingConstructiveElement(v) => v.relative_to_water(),
            Self::BuildingFurniture(v) => v.relative_to_water(),
            Self::BuildingInstallation(v) => v.relative_to_water(),
            Self::BuildingPart(v) => v.relative_to_water(),
            Self::BuildingRoom(v) => v.relative_to_water(),
            Self::CityFurniture(v) => v.relative_to_water(),
            Self::GenericOccupiedSpace(v) => v.relative_to_water(),
            Self::GenericUnoccupiedSpace(v) => v.relative_to_water(),
            Self::AuxiliaryTrafficSpace(v) => v.relative_to_water(),
            Self::ClearanceSpace(v) => v.relative_to_water(),
            Self::Hole(v) => v.relative_to_water(),
            Self::Intersection(v) => v.relative_to_water(),
            Self::Railway(v) => v.relative_to_water(),
            Self::Road(v) => v.relative_to_water(),
            Self::Section(v) => v.relative_to_water(),
            Self::Square(v) => v.relative_to_water(),
            Self::Track(v) => v.relative_to_water(),
            Self::TrafficSpace(v) => v.relative_to_water(),
            Self::Waterway(v) => v.relative_to_water(),
            Self::HollowSpace(v) => v.relative_to_water(),
            Self::Tunnel(v) => v.relative_to_water(),
            Self::TunnelConstructiveElement(v) => v.relative_to_water(),
            Self::TunnelFurniture(v) => v.relative_to_water(),
            Self::TunnelInstallation(v) => v.relative_to_water(),
            Self::TunnelPart(v) => v.relative_to_water(),
            Self::PlantCover(v) => v.relative_to_water(),
            Self::SolitaryVegetationObject(v) => v.relative_to_water(),
            Self::WaterBody(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::Door(v) => v.appearance(),
            Self::OtherConstruction(v) => v.appearance(),
            Self::Window(v) => v.appearance(),
            Self::Bridge(v) => v.appearance(),
            Self::BridgeConstructiveElement(v) => v.appearance(),
            Self::BridgeFurniture(v) => v.appearance(),
            Self::BridgeInstallation(v) => v.appearance(),
            Self::BridgePart(v) => v.appearance(),
            Self::BridgeRoom(v) => v.appearance(),
            Self::Building(v) => v.appearance(),
            Self::BuildingConstructiveElement(v) => v.appearance(),
            Self::BuildingFurniture(v) => v.appearance(),
            Self::BuildingInstallation(v) => v.appearance(),
            Self::BuildingPart(v) => v.appearance(),
            Self::BuildingRoom(v) => v.appearance(),
            Self::CityFurniture(v) => v.appearance(),
            Self::GenericOccupiedSpace(v) => v.appearance(),
            Self::GenericUnoccupiedSpace(v) => v.appearance(),
            Self::AuxiliaryTrafficSpace(v) => v.appearance(),
            Self::ClearanceSpace(v) => v.appearance(),
            Self::Hole(v) => v.appearance(),
            Self::Intersection(v) => v.appearance(),
            Self::Railway(v) => v.appearance(),
            Self::Road(v) => v.appearance(),
            Self::Section(v) => v.appearance(),
            Self::Square(v) => v.appearance(),
            Self::Track(v) => v.appearance(),
            Self::TrafficSpace(v) => v.appearance(),
            Self::Waterway(v) => v.appearance(),
            Self::HollowSpace(v) => v.appearance(),
            Self::Tunnel(v) => v.appearance(),
            Self::TunnelConstructiveElement(v) => v.appearance(),
            Self::TunnelFurniture(v) => v.appearance(),
            Self::TunnelInstallation(v) => v.appearance(),
            Self::TunnelPart(v) => v.appearance(),
            Self::PlantCover(v) => v.appearance(),
            Self::SolitaryVegetationObject(v) => v.appearance(),
            Self::WaterBody(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::Door(v) => v.generalizes_to(),
            Self::OtherConstruction(v) => v.generalizes_to(),
            Self::Window(v) => v.generalizes_to(),
            Self::Bridge(v) => v.generalizes_to(),
            Self::BridgeConstructiveElement(v) => v.generalizes_to(),
            Self::BridgeFurniture(v) => v.generalizes_to(),
            Self::BridgeInstallation(v) => v.generalizes_to(),
            Self::BridgePart(v) => v.generalizes_to(),
            Self::BridgeRoom(v) => v.generalizes_to(),
            Self::Building(v) => v.generalizes_to(),
            Self::BuildingConstructiveElement(v) => v.generalizes_to(),
            Self::BuildingFurniture(v) => v.generalizes_to(),
            Self::BuildingInstallation(v) => v.generalizes_to(),
            Self::BuildingPart(v) => v.generalizes_to(),
            Self::BuildingRoom(v) => v.generalizes_to(),
            Self::CityFurniture(v) => v.generalizes_to(),
            Self::GenericOccupiedSpace(v) => v.generalizes_to(),
            Self::GenericUnoccupiedSpace(v) => v.generalizes_to(),
            Self::AuxiliaryTrafficSpace(v) => v.generalizes_to(),
            Self::ClearanceSpace(v) => v.generalizes_to(),
            Self::Hole(v) => v.generalizes_to(),
            Self::Intersection(v) => v.generalizes_to(),
            Self::Railway(v) => v.generalizes_to(),
            Self::Road(v) => v.generalizes_to(),
            Self::Section(v) => v.generalizes_to(),
            Self::Square(v) => v.generalizes_to(),
            Self::Track(v) => v.generalizes_to(),
            Self::TrafficSpace(v) => v.generalizes_to(),
            Self::Waterway(v) => v.generalizes_to(),
            Self::HollowSpace(v) => v.generalizes_to(),
            Self::Tunnel(v) => v.generalizes_to(),
            Self::TunnelConstructiveElement(v) => v.generalizes_to(),
            Self::TunnelFurniture(v) => v.generalizes_to(),
            Self::TunnelInstallation(v) => v.generalizes_to(),
            Self::TunnelPart(v) => v.generalizes_to(),
            Self::PlantCover(v) => v.generalizes_to(),
            Self::SolitaryVegetationObject(v) => v.generalizes_to(),
            Self::WaterBody(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::Door(v) => v.external_reference(),
            Self::OtherConstruction(v) => v.external_reference(),
            Self::Window(v) => v.external_reference(),
            Self::Bridge(v) => v.external_reference(),
            Self::BridgeConstructiveElement(v) => v.external_reference(),
            Self::BridgeFurniture(v) => v.external_reference(),
            Self::BridgeInstallation(v) => v.external_reference(),
            Self::BridgePart(v) => v.external_reference(),
            Self::BridgeRoom(v) => v.external_reference(),
            Self::Building(v) => v.external_reference(),
            Self::BuildingConstructiveElement(v) => v.external_reference(),
            Self::BuildingFurniture(v) => v.external_reference(),
            Self::BuildingInstallation(v) => v.external_reference(),
            Self::BuildingPart(v) => v.external_reference(),
            Self::BuildingRoom(v) => v.external_reference(),
            Self::CityFurniture(v) => v.external_reference(),
            Self::GenericOccupiedSpace(v) => v.external_reference(),
            Self::GenericUnoccupiedSpace(v) => v.external_reference(),
            Self::AuxiliaryTrafficSpace(v) => v.external_reference(),
            Self::ClearanceSpace(v) => v.external_reference(),
            Self::Hole(v) => v.external_reference(),
            Self::Intersection(v) => v.external_reference(),
            Self::Railway(v) => v.external_reference(),
            Self::Road(v) => v.external_reference(),
            Self::Section(v) => v.external_reference(),
            Self::Square(v) => v.external_reference(),
            Self::Track(v) => v.external_reference(),
            Self::TrafficSpace(v) => v.external_reference(),
            Self::Waterway(v) => v.external_reference(),
            Self::HollowSpace(v) => v.external_reference(),
            Self::Tunnel(v) => v.external_reference(),
            Self::TunnelConstructiveElement(v) => v.external_reference(),
            Self::TunnelFurniture(v) => v.external_reference(),
            Self::TunnelInstallation(v) => v.external_reference(),
            Self::TunnelPart(v) => v.external_reference(),
            Self::PlantCover(v) => v.external_reference(),
            Self::SolitaryVegetationObject(v) => v.external_reference(),
            Self::WaterBody(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::Door(v) => v.related_to(),
            Self::OtherConstruction(v) => v.related_to(),
            Self::Window(v) => v.related_to(),
            Self::Bridge(v) => v.related_to(),
            Self::BridgeConstructiveElement(v) => v.related_to(),
            Self::BridgeFurniture(v) => v.related_to(),
            Self::BridgeInstallation(v) => v.related_to(),
            Self::BridgePart(v) => v.related_to(),
            Self::BridgeRoom(v) => v.related_to(),
            Self::Building(v) => v.related_to(),
            Self::BuildingConstructiveElement(v) => v.related_to(),
            Self::BuildingFurniture(v) => v.related_to(),
            Self::BuildingInstallation(v) => v.related_to(),
            Self::BuildingPart(v) => v.related_to(),
            Self::BuildingRoom(v) => v.related_to(),
            Self::CityFurniture(v) => v.related_to(),
            Self::GenericOccupiedSpace(v) => v.related_to(),
            Self::GenericUnoccupiedSpace(v) => v.related_to(),
            Self::AuxiliaryTrafficSpace(v) => v.related_to(),
            Self::ClearanceSpace(v) => v.related_to(),
            Self::Hole(v) => v.related_to(),
            Self::Intersection(v) => v.related_to(),
            Self::Railway(v) => v.related_to(),
            Self::Road(v) => v.related_to(),
            Self::Section(v) => v.related_to(),
            Self::Square(v) => v.related_to(),
            Self::Track(v) => v.related_to(),
            Self::TrafficSpace(v) => v.related_to(),
            Self::Waterway(v) => v.related_to(),
            Self::HollowSpace(v) => v.related_to(),
            Self::Tunnel(v) => v.related_to(),
            Self::TunnelConstructiveElement(v) => v.related_to(),
            Self::TunnelFurniture(v) => v.related_to(),
            Self::TunnelInstallation(v) => v.related_to(),
            Self::TunnelPart(v) => v.related_to(),
            Self::PlantCover(v) => v.related_to(),
            Self::SolitaryVegetationObject(v) => v.related_to(),
            Self::WaterBody(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::Door(v) => v.dynamizer(),
            Self::OtherConstruction(v) => v.dynamizer(),
            Self::Window(v) => v.dynamizer(),
            Self::Bridge(v) => v.dynamizer(),
            Self::BridgeConstructiveElement(v) => v.dynamizer(),
            Self::BridgeFurniture(v) => v.dynamizer(),
            Self::BridgeInstallation(v) => v.dynamizer(),
            Self::BridgePart(v) => v.dynamizer(),
            Self::BridgeRoom(v) => v.dynamizer(),
            Self::Building(v) => v.dynamizer(),
            Self::BuildingConstructiveElement(v) => v.dynamizer(),
            Self::BuildingFurniture(v) => v.dynamizer(),
            Self::BuildingInstallation(v) => v.dynamizer(),
            Self::BuildingPart(v) => v.dynamizer(),
            Self::BuildingRoom(v) => v.dynamizer(),
            Self::CityFurniture(v) => v.dynamizer(),
            Self::GenericOccupiedSpace(v) => v.dynamizer(),
            Self::GenericUnoccupiedSpace(v) => v.dynamizer(),
            Self::AuxiliaryTrafficSpace(v) => v.dynamizer(),
            Self::ClearanceSpace(v) => v.dynamizer(),
            Self::Hole(v) => v.dynamizer(),
            Self::Intersection(v) => v.dynamizer(),
            Self::Railway(v) => v.dynamizer(),
            Self::Road(v) => v.dynamizer(),
            Self::Section(v) => v.dynamizer(),
            Self::Square(v) => v.dynamizer(),
            Self::Track(v) => v.dynamizer(),
            Self::TrafficSpace(v) => v.dynamizer(),
            Self::Waterway(v) => v.dynamizer(),
            Self::HollowSpace(v) => v.dynamizer(),
            Self::Tunnel(v) => v.dynamizer(),
            Self::TunnelConstructiveElement(v) => v.dynamizer(),
            Self::TunnelFurniture(v) => v.dynamizer(),
            Self::TunnelInstallation(v) => v.dynamizer(),
            Self::TunnelPart(v) => v.dynamizer(),
            Self::PlantCover(v) => v.dynamizer(),
            Self::SolitaryVegetationObject(v) => v.dynamizer(),
            Self::WaterBody(v) => v.dynamizer(),
        }
    }
}
impl AbstractSpaceTrait for AbstractPhysicalSpace {
    fn space_type(&self) -> Option<SpaceType> {
        match self {
            Self::Door(v) => v.space_type(),
            Self::OtherConstruction(v) => v.space_type(),
            Self::Window(v) => v.space_type(),
            Self::Bridge(v) => v.space_type(),
            Self::BridgeConstructiveElement(v) => v.space_type(),
            Self::BridgeFurniture(v) => v.space_type(),
            Self::BridgeInstallation(v) => v.space_type(),
            Self::BridgePart(v) => v.space_type(),
            Self::BridgeRoom(v) => v.space_type(),
            Self::Building(v) => v.space_type(),
            Self::BuildingConstructiveElement(v) => v.space_type(),
            Self::BuildingFurniture(v) => v.space_type(),
            Self::BuildingInstallation(v) => v.space_type(),
            Self::BuildingPart(v) => v.space_type(),
            Self::BuildingRoom(v) => v.space_type(),
            Self::CityFurniture(v) => v.space_type(),
            Self::GenericOccupiedSpace(v) => v.space_type(),
            Self::GenericUnoccupiedSpace(v) => v.space_type(),
            Self::AuxiliaryTrafficSpace(v) => v.space_type(),
            Self::ClearanceSpace(v) => v.space_type(),
            Self::Hole(v) => v.space_type(),
            Self::Intersection(v) => v.space_type(),
            Self::Railway(v) => v.space_type(),
            Self::Road(v) => v.space_type(),
            Self::Section(v) => v.space_type(),
            Self::Square(v) => v.space_type(),
            Self::Track(v) => v.space_type(),
            Self::TrafficSpace(v) => v.space_type(),
            Self::Waterway(v) => v.space_type(),
            Self::HollowSpace(v) => v.space_type(),
            Self::Tunnel(v) => v.space_type(),
            Self::TunnelConstructiveElement(v) => v.space_type(),
            Self::TunnelFurniture(v) => v.space_type(),
            Self::TunnelInstallation(v) => v.space_type(),
            Self::TunnelPart(v) => v.space_type(),
            Self::PlantCover(v) => v.space_type(),
            Self::SolitaryVegetationObject(v) => v.space_type(),
            Self::WaterBody(v) => v.space_type(),
        }
    }
    fn volume(&self) -> &[QualifiedVolume] {
        match self {
            Self::Door(v) => v.volume(),
            Self::OtherConstruction(v) => v.volume(),
            Self::Window(v) => v.volume(),
            Self::Bridge(v) => v.volume(),
            Self::BridgeConstructiveElement(v) => v.volume(),
            Self::BridgeFurniture(v) => v.volume(),
            Self::BridgeInstallation(v) => v.volume(),
            Self::BridgePart(v) => v.volume(),
            Self::BridgeRoom(v) => v.volume(),
            Self::Building(v) => v.volume(),
            Self::BuildingConstructiveElement(v) => v.volume(),
            Self::BuildingFurniture(v) => v.volume(),
            Self::BuildingInstallation(v) => v.volume(),
            Self::BuildingPart(v) => v.volume(),
            Self::BuildingRoom(v) => v.volume(),
            Self::CityFurniture(v) => v.volume(),
            Self::GenericOccupiedSpace(v) => v.volume(),
            Self::GenericUnoccupiedSpace(v) => v.volume(),
            Self::AuxiliaryTrafficSpace(v) => v.volume(),
            Self::ClearanceSpace(v) => v.volume(),
            Self::Hole(v) => v.volume(),
            Self::Intersection(v) => v.volume(),
            Self::Railway(v) => v.volume(),
            Self::Road(v) => v.volume(),
            Self::Section(v) => v.volume(),
            Self::Square(v) => v.volume(),
            Self::Track(v) => v.volume(),
            Self::TrafficSpace(v) => v.volume(),
            Self::Waterway(v) => v.volume(),
            Self::HollowSpace(v) => v.volume(),
            Self::Tunnel(v) => v.volume(),
            Self::TunnelConstructiveElement(v) => v.volume(),
            Self::TunnelFurniture(v) => v.volume(),
            Self::TunnelInstallation(v) => v.volume(),
            Self::TunnelPart(v) => v.volume(),
            Self::PlantCover(v) => v.volume(),
            Self::SolitaryVegetationObject(v) => v.volume(),
            Self::WaterBody(v) => v.volume(),
        }
    }
    fn area(&self) -> &[QualifiedArea] {
        match self {
            Self::Door(v) => v.area(),
            Self::OtherConstruction(v) => v.area(),
            Self::Window(v) => v.area(),
            Self::Bridge(v) => v.area(),
            Self::BridgeConstructiveElement(v) => v.area(),
            Self::BridgeFurniture(v) => v.area(),
            Self::BridgeInstallation(v) => v.area(),
            Self::BridgePart(v) => v.area(),
            Self::BridgeRoom(v) => v.area(),
            Self::Building(v) => v.area(),
            Self::BuildingConstructiveElement(v) => v.area(),
            Self::BuildingFurniture(v) => v.area(),
            Self::BuildingInstallation(v) => v.area(),
            Self::BuildingPart(v) => v.area(),
            Self::BuildingRoom(v) => v.area(),
            Self::CityFurniture(v) => v.area(),
            Self::GenericOccupiedSpace(v) => v.area(),
            Self::GenericUnoccupiedSpace(v) => v.area(),
            Self::AuxiliaryTrafficSpace(v) => v.area(),
            Self::ClearanceSpace(v) => v.area(),
            Self::Hole(v) => v.area(),
            Self::Intersection(v) => v.area(),
            Self::Railway(v) => v.area(),
            Self::Road(v) => v.area(),
            Self::Section(v) => v.area(),
            Self::Square(v) => v.area(),
            Self::Track(v) => v.area(),
            Self::TrafficSpace(v) => v.area(),
            Self::Waterway(v) => v.area(),
            Self::HollowSpace(v) => v.area(),
            Self::Tunnel(v) => v.area(),
            Self::TunnelConstructiveElement(v) => v.area(),
            Self::TunnelFurniture(v) => v.area(),
            Self::TunnelInstallation(v) => v.area(),
            Self::TunnelPart(v) => v.area(),
            Self::PlantCover(v) => v.area(),
            Self::SolitaryVegetationObject(v) => v.area(),
            Self::WaterBody(v) => v.area(),
        }
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod2_multi_curve(),
            Self::OtherConstruction(v) => v.lod2_multi_curve(),
            Self::Window(v) => v.lod2_multi_curve(),
            Self::Bridge(v) => v.lod2_multi_curve(),
            Self::BridgeConstructiveElement(v) => v.lod2_multi_curve(),
            Self::BridgeFurniture(v) => v.lod2_multi_curve(),
            Self::BridgeInstallation(v) => v.lod2_multi_curve(),
            Self::BridgePart(v) => v.lod2_multi_curve(),
            Self::BridgeRoom(v) => v.lod2_multi_curve(),
            Self::Building(v) => v.lod2_multi_curve(),
            Self::BuildingConstructiveElement(v) => v.lod2_multi_curve(),
            Self::BuildingFurniture(v) => v.lod2_multi_curve(),
            Self::BuildingInstallation(v) => v.lod2_multi_curve(),
            Self::BuildingPart(v) => v.lod2_multi_curve(),
            Self::BuildingRoom(v) => v.lod2_multi_curve(),
            Self::CityFurniture(v) => v.lod2_multi_curve(),
            Self::GenericOccupiedSpace(v) => v.lod2_multi_curve(),
            Self::GenericUnoccupiedSpace(v) => v.lod2_multi_curve(),
            Self::AuxiliaryTrafficSpace(v) => v.lod2_multi_curve(),
            Self::ClearanceSpace(v) => v.lod2_multi_curve(),
            Self::Hole(v) => v.lod2_multi_curve(),
            Self::Intersection(v) => v.lod2_multi_curve(),
            Self::Railway(v) => v.lod2_multi_curve(),
            Self::Road(v) => v.lod2_multi_curve(),
            Self::Section(v) => v.lod2_multi_curve(),
            Self::Square(v) => v.lod2_multi_curve(),
            Self::Track(v) => v.lod2_multi_curve(),
            Self::TrafficSpace(v) => v.lod2_multi_curve(),
            Self::Waterway(v) => v.lod2_multi_curve(),
            Self::HollowSpace(v) => v.lod2_multi_curve(),
            Self::Tunnel(v) => v.lod2_multi_curve(),
            Self::TunnelConstructiveElement(v) => v.lod2_multi_curve(),
            Self::TunnelFurniture(v) => v.lod2_multi_curve(),
            Self::TunnelInstallation(v) => v.lod2_multi_curve(),
            Self::TunnelPart(v) => v.lod2_multi_curve(),
            Self::PlantCover(v) => v.lod2_multi_curve(),
            Self::SolitaryVegetationObject(v) => v.lod2_multi_curve(),
            Self::WaterBody(v) => v.lod2_multi_curve(),
        }
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::Door(v) => v.lod3_multi_surface(),
            Self::OtherConstruction(v) => v.lod3_multi_surface(),
            Self::Window(v) => v.lod3_multi_surface(),
            Self::Bridge(v) => v.lod3_multi_surface(),
            Self::BridgeConstructiveElement(v) => v.lod3_multi_surface(),
            Self::BridgeFurniture(v) => v.lod3_multi_surface(),
            Self::BridgeInstallation(v) => v.lod3_multi_surface(),
            Self::BridgePart(v) => v.lod3_multi_surface(),
            Self::BridgeRoom(v) => v.lod3_multi_surface(),
            Self::Building(v) => v.lod3_multi_surface(),
            Self::BuildingConstructiveElement(v) => v.lod3_multi_surface(),
            Self::BuildingFurniture(v) => v.lod3_multi_surface(),
            Self::BuildingInstallation(v) => v.lod3_multi_surface(),
            Self::BuildingPart(v) => v.lod3_multi_surface(),
            Self::BuildingRoom(v) => v.lod3_multi_surface(),
            Self::CityFurniture(v) => v.lod3_multi_surface(),
            Self::GenericOccupiedSpace(v) => v.lod3_multi_surface(),
            Self::GenericUnoccupiedSpace(v) => v.lod3_multi_surface(),
            Self::AuxiliaryTrafficSpace(v) => v.lod3_multi_surface(),
            Self::ClearanceSpace(v) => v.lod3_multi_surface(),
            Self::Hole(v) => v.lod3_multi_surface(),
            Self::Intersection(v) => v.lod3_multi_surface(),
            Self::Railway(v) => v.lod3_multi_surface(),
            Self::Road(v) => v.lod3_multi_surface(),
            Self::Section(v) => v.lod3_multi_surface(),
            Self::Square(v) => v.lod3_multi_surface(),
            Self::Track(v) => v.lod3_multi_surface(),
            Self::TrafficSpace(v) => v.lod3_multi_surface(),
            Self::Waterway(v) => v.lod3_multi_surface(),
            Self::HollowSpace(v) => v.lod3_multi_surface(),
            Self::Tunnel(v) => v.lod3_multi_surface(),
            Self::TunnelConstructiveElement(v) => v.lod3_multi_surface(),
            Self::TunnelFurniture(v) => v.lod3_multi_surface(),
            Self::TunnelInstallation(v) => v.lod3_multi_surface(),
            Self::TunnelPart(v) => v.lod3_multi_surface(),
            Self::PlantCover(v) => v.lod3_multi_surface(),
            Self::SolitaryVegetationObject(v) => v.lod3_multi_surface(),
            Self::WaterBody(v) => v.lod3_multi_surface(),
        }
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::Door(v) => v.lod0_multi_surface(),
            Self::OtherConstruction(v) => v.lod0_multi_surface(),
            Self::Window(v) => v.lod0_multi_surface(),
            Self::Bridge(v) => v.lod0_multi_surface(),
            Self::BridgeConstructiveElement(v) => v.lod0_multi_surface(),
            Self::BridgeFurniture(v) => v.lod0_multi_surface(),
            Self::BridgeInstallation(v) => v.lod0_multi_surface(),
            Self::BridgePart(v) => v.lod0_multi_surface(),
            Self::BridgeRoom(v) => v.lod0_multi_surface(),
            Self::Building(v) => v.lod0_multi_surface(),
            Self::BuildingConstructiveElement(v) => v.lod0_multi_surface(),
            Self::BuildingFurniture(v) => v.lod0_multi_surface(),
            Self::BuildingInstallation(v) => v.lod0_multi_surface(),
            Self::BuildingPart(v) => v.lod0_multi_surface(),
            Self::BuildingRoom(v) => v.lod0_multi_surface(),
            Self::CityFurniture(v) => v.lod0_multi_surface(),
            Self::GenericOccupiedSpace(v) => v.lod0_multi_surface(),
            Self::GenericUnoccupiedSpace(v) => v.lod0_multi_surface(),
            Self::AuxiliaryTrafficSpace(v) => v.lod0_multi_surface(),
            Self::ClearanceSpace(v) => v.lod0_multi_surface(),
            Self::Hole(v) => v.lod0_multi_surface(),
            Self::Intersection(v) => v.lod0_multi_surface(),
            Self::Railway(v) => v.lod0_multi_surface(),
            Self::Road(v) => v.lod0_multi_surface(),
            Self::Section(v) => v.lod0_multi_surface(),
            Self::Square(v) => v.lod0_multi_surface(),
            Self::Track(v) => v.lod0_multi_surface(),
            Self::TrafficSpace(v) => v.lod0_multi_surface(),
            Self::Waterway(v) => v.lod0_multi_surface(),
            Self::HollowSpace(v) => v.lod0_multi_surface(),
            Self::Tunnel(v) => v.lod0_multi_surface(),
            Self::TunnelConstructiveElement(v) => v.lod0_multi_surface(),
            Self::TunnelFurniture(v) => v.lod0_multi_surface(),
            Self::TunnelInstallation(v) => v.lod0_multi_surface(),
            Self::TunnelPart(v) => v.lod0_multi_surface(),
            Self::PlantCover(v) => v.lod0_multi_surface(),
            Self::SolitaryVegetationObject(v) => v.lod0_multi_surface(),
            Self::WaterBody(v) => v.lod0_multi_surface(),
        }
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::Door(v) => v.lod1_solid(),
            Self::OtherConstruction(v) => v.lod1_solid(),
            Self::Window(v) => v.lod1_solid(),
            Self::Bridge(v) => v.lod1_solid(),
            Self::BridgeConstructiveElement(v) => v.lod1_solid(),
            Self::BridgeFurniture(v) => v.lod1_solid(),
            Self::BridgeInstallation(v) => v.lod1_solid(),
            Self::BridgePart(v) => v.lod1_solid(),
            Self::BridgeRoom(v) => v.lod1_solid(),
            Self::Building(v) => v.lod1_solid(),
            Self::BuildingConstructiveElement(v) => v.lod1_solid(),
            Self::BuildingFurniture(v) => v.lod1_solid(),
            Self::BuildingInstallation(v) => v.lod1_solid(),
            Self::BuildingPart(v) => v.lod1_solid(),
            Self::BuildingRoom(v) => v.lod1_solid(),
            Self::CityFurniture(v) => v.lod1_solid(),
            Self::GenericOccupiedSpace(v) => v.lod1_solid(),
            Self::GenericUnoccupiedSpace(v) => v.lod1_solid(),
            Self::AuxiliaryTrafficSpace(v) => v.lod1_solid(),
            Self::ClearanceSpace(v) => v.lod1_solid(),
            Self::Hole(v) => v.lod1_solid(),
            Self::Intersection(v) => v.lod1_solid(),
            Self::Railway(v) => v.lod1_solid(),
            Self::Road(v) => v.lod1_solid(),
            Self::Section(v) => v.lod1_solid(),
            Self::Square(v) => v.lod1_solid(),
            Self::Track(v) => v.lod1_solid(),
            Self::TrafficSpace(v) => v.lod1_solid(),
            Self::Waterway(v) => v.lod1_solid(),
            Self::HollowSpace(v) => v.lod1_solid(),
            Self::Tunnel(v) => v.lod1_solid(),
            Self::TunnelConstructiveElement(v) => v.lod1_solid(),
            Self::TunnelFurniture(v) => v.lod1_solid(),
            Self::TunnelInstallation(v) => v.lod1_solid(),
            Self::TunnelPart(v) => v.lod1_solid(),
            Self::PlantCover(v) => v.lod1_solid(),
            Self::SolitaryVegetationObject(v) => v.lod1_solid(),
            Self::WaterBody(v) => v.lod1_solid(),
        }
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::Door(v) => v.lod3_solid(),
            Self::OtherConstruction(v) => v.lod3_solid(),
            Self::Window(v) => v.lod3_solid(),
            Self::Bridge(v) => v.lod3_solid(),
            Self::BridgeConstructiveElement(v) => v.lod3_solid(),
            Self::BridgeFurniture(v) => v.lod3_solid(),
            Self::BridgeInstallation(v) => v.lod3_solid(),
            Self::BridgePart(v) => v.lod3_solid(),
            Self::BridgeRoom(v) => v.lod3_solid(),
            Self::Building(v) => v.lod3_solid(),
            Self::BuildingConstructiveElement(v) => v.lod3_solid(),
            Self::BuildingFurniture(v) => v.lod3_solid(),
            Self::BuildingInstallation(v) => v.lod3_solid(),
            Self::BuildingPart(v) => v.lod3_solid(),
            Self::BuildingRoom(v) => v.lod3_solid(),
            Self::CityFurniture(v) => v.lod3_solid(),
            Self::GenericOccupiedSpace(v) => v.lod3_solid(),
            Self::GenericUnoccupiedSpace(v) => v.lod3_solid(),
            Self::AuxiliaryTrafficSpace(v) => v.lod3_solid(),
            Self::ClearanceSpace(v) => v.lod3_solid(),
            Self::Hole(v) => v.lod3_solid(),
            Self::Intersection(v) => v.lod3_solid(),
            Self::Railway(v) => v.lod3_solid(),
            Self::Road(v) => v.lod3_solid(),
            Self::Section(v) => v.lod3_solid(),
            Self::Square(v) => v.lod3_solid(),
            Self::Track(v) => v.lod3_solid(),
            Self::TrafficSpace(v) => v.lod3_solid(),
            Self::Waterway(v) => v.lod3_solid(),
            Self::HollowSpace(v) => v.lod3_solid(),
            Self::Tunnel(v) => v.lod3_solid(),
            Self::TunnelConstructiveElement(v) => v.lod3_solid(),
            Self::TunnelFurniture(v) => v.lod3_solid(),
            Self::TunnelInstallation(v) => v.lod3_solid(),
            Self::TunnelPart(v) => v.lod3_solid(),
            Self::PlantCover(v) => v.lod3_solid(),
            Self::SolitaryVegetationObject(v) => v.lod3_solid(),
            Self::WaterBody(v) => v.lod3_solid(),
        }
    }
    fn boundary(&self) -> &[AbstractSpaceBoundary] {
        match self {
            Self::Door(v) => v.boundary(),
            Self::OtherConstruction(v) => v.boundary(),
            Self::Window(v) => v.boundary(),
            Self::Bridge(v) => v.boundary(),
            Self::BridgeConstructiveElement(v) => v.boundary(),
            Self::BridgeFurniture(v) => v.boundary(),
            Self::BridgeInstallation(v) => v.boundary(),
            Self::BridgePart(v) => v.boundary(),
            Self::BridgeRoom(v) => v.boundary(),
            Self::Building(v) => v.boundary(),
            Self::BuildingConstructiveElement(v) => v.boundary(),
            Self::BuildingFurniture(v) => v.boundary(),
            Self::BuildingInstallation(v) => v.boundary(),
            Self::BuildingPart(v) => v.boundary(),
            Self::BuildingRoom(v) => v.boundary(),
            Self::CityFurniture(v) => v.boundary(),
            Self::GenericOccupiedSpace(v) => v.boundary(),
            Self::GenericUnoccupiedSpace(v) => v.boundary(),
            Self::AuxiliaryTrafficSpace(v) => v.boundary(),
            Self::ClearanceSpace(v) => v.boundary(),
            Self::Hole(v) => v.boundary(),
            Self::Intersection(v) => v.boundary(),
            Self::Railway(v) => v.boundary(),
            Self::Road(v) => v.boundary(),
            Self::Section(v) => v.boundary(),
            Self::Square(v) => v.boundary(),
            Self::Track(v) => v.boundary(),
            Self::TrafficSpace(v) => v.boundary(),
            Self::Waterway(v) => v.boundary(),
            Self::HollowSpace(v) => v.boundary(),
            Self::Tunnel(v) => v.boundary(),
            Self::TunnelConstructiveElement(v) => v.boundary(),
            Self::TunnelFurniture(v) => v.boundary(),
            Self::TunnelInstallation(v) => v.boundary(),
            Self::TunnelPart(v) => v.boundary(),
            Self::PlantCover(v) => v.boundary(),
            Self::SolitaryVegetationObject(v) => v.boundary(),
            Self::WaterBody(v) => v.boundary(),
        }
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod0_multi_curve(),
            Self::OtherConstruction(v) => v.lod0_multi_curve(),
            Self::Window(v) => v.lod0_multi_curve(),
            Self::Bridge(v) => v.lod0_multi_curve(),
            Self::BridgeConstructiveElement(v) => v.lod0_multi_curve(),
            Self::BridgeFurniture(v) => v.lod0_multi_curve(),
            Self::BridgeInstallation(v) => v.lod0_multi_curve(),
            Self::BridgePart(v) => v.lod0_multi_curve(),
            Self::BridgeRoom(v) => v.lod0_multi_curve(),
            Self::Building(v) => v.lod0_multi_curve(),
            Self::BuildingConstructiveElement(v) => v.lod0_multi_curve(),
            Self::BuildingFurniture(v) => v.lod0_multi_curve(),
            Self::BuildingInstallation(v) => v.lod0_multi_curve(),
            Self::BuildingPart(v) => v.lod0_multi_curve(),
            Self::BuildingRoom(v) => v.lod0_multi_curve(),
            Self::CityFurniture(v) => v.lod0_multi_curve(),
            Self::GenericOccupiedSpace(v) => v.lod0_multi_curve(),
            Self::GenericUnoccupiedSpace(v) => v.lod0_multi_curve(),
            Self::AuxiliaryTrafficSpace(v) => v.lod0_multi_curve(),
            Self::ClearanceSpace(v) => v.lod0_multi_curve(),
            Self::Hole(v) => v.lod0_multi_curve(),
            Self::Intersection(v) => v.lod0_multi_curve(),
            Self::Railway(v) => v.lod0_multi_curve(),
            Self::Road(v) => v.lod0_multi_curve(),
            Self::Section(v) => v.lod0_multi_curve(),
            Self::Square(v) => v.lod0_multi_curve(),
            Self::Track(v) => v.lod0_multi_curve(),
            Self::TrafficSpace(v) => v.lod0_multi_curve(),
            Self::Waterway(v) => v.lod0_multi_curve(),
            Self::HollowSpace(v) => v.lod0_multi_curve(),
            Self::Tunnel(v) => v.lod0_multi_curve(),
            Self::TunnelConstructiveElement(v) => v.lod0_multi_curve(),
            Self::TunnelFurniture(v) => v.lod0_multi_curve(),
            Self::TunnelInstallation(v) => v.lod0_multi_curve(),
            Self::TunnelPart(v) => v.lod0_multi_curve(),
            Self::PlantCover(v) => v.lod0_multi_curve(),
            Self::SolitaryVegetationObject(v) => v.lod0_multi_curve(),
            Self::WaterBody(v) => v.lod0_multi_curve(),
        }
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::Door(v) => v.lod2_solid(),
            Self::OtherConstruction(v) => v.lod2_solid(),
            Self::Window(v) => v.lod2_solid(),
            Self::Bridge(v) => v.lod2_solid(),
            Self::BridgeConstructiveElement(v) => v.lod2_solid(),
            Self::BridgeFurniture(v) => v.lod2_solid(),
            Self::BridgeInstallation(v) => v.lod2_solid(),
            Self::BridgePart(v) => v.lod2_solid(),
            Self::BridgeRoom(v) => v.lod2_solid(),
            Self::Building(v) => v.lod2_solid(),
            Self::BuildingConstructiveElement(v) => v.lod2_solid(),
            Self::BuildingFurniture(v) => v.lod2_solid(),
            Self::BuildingInstallation(v) => v.lod2_solid(),
            Self::BuildingPart(v) => v.lod2_solid(),
            Self::BuildingRoom(v) => v.lod2_solid(),
            Self::CityFurniture(v) => v.lod2_solid(),
            Self::GenericOccupiedSpace(v) => v.lod2_solid(),
            Self::GenericUnoccupiedSpace(v) => v.lod2_solid(),
            Self::AuxiliaryTrafficSpace(v) => v.lod2_solid(),
            Self::ClearanceSpace(v) => v.lod2_solid(),
            Self::Hole(v) => v.lod2_solid(),
            Self::Intersection(v) => v.lod2_solid(),
            Self::Railway(v) => v.lod2_solid(),
            Self::Road(v) => v.lod2_solid(),
            Self::Section(v) => v.lod2_solid(),
            Self::Square(v) => v.lod2_solid(),
            Self::Track(v) => v.lod2_solid(),
            Self::TrafficSpace(v) => v.lod2_solid(),
            Self::Waterway(v) => v.lod2_solid(),
            Self::HollowSpace(v) => v.lod2_solid(),
            Self::Tunnel(v) => v.lod2_solid(),
            Self::TunnelConstructiveElement(v) => v.lod2_solid(),
            Self::TunnelFurniture(v) => v.lod2_solid(),
            Self::TunnelInstallation(v) => v.lod2_solid(),
            Self::TunnelPart(v) => v.lod2_solid(),
            Self::PlantCover(v) => v.lod2_solid(),
            Self::SolitaryVegetationObject(v) => v.lod2_solid(),
            Self::WaterBody(v) => v.lod2_solid(),
        }
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        match self {
            Self::Door(v) => v.lod0_point(),
            Self::OtherConstruction(v) => v.lod0_point(),
            Self::Window(v) => v.lod0_point(),
            Self::Bridge(v) => v.lod0_point(),
            Self::BridgeConstructiveElement(v) => v.lod0_point(),
            Self::BridgeFurniture(v) => v.lod0_point(),
            Self::BridgeInstallation(v) => v.lod0_point(),
            Self::BridgePart(v) => v.lod0_point(),
            Self::BridgeRoom(v) => v.lod0_point(),
            Self::Building(v) => v.lod0_point(),
            Self::BuildingConstructiveElement(v) => v.lod0_point(),
            Self::BuildingFurniture(v) => v.lod0_point(),
            Self::BuildingInstallation(v) => v.lod0_point(),
            Self::BuildingPart(v) => v.lod0_point(),
            Self::BuildingRoom(v) => v.lod0_point(),
            Self::CityFurniture(v) => v.lod0_point(),
            Self::GenericOccupiedSpace(v) => v.lod0_point(),
            Self::GenericUnoccupiedSpace(v) => v.lod0_point(),
            Self::AuxiliaryTrafficSpace(v) => v.lod0_point(),
            Self::ClearanceSpace(v) => v.lod0_point(),
            Self::Hole(v) => v.lod0_point(),
            Self::Intersection(v) => v.lod0_point(),
            Self::Railway(v) => v.lod0_point(),
            Self::Road(v) => v.lod0_point(),
            Self::Section(v) => v.lod0_point(),
            Self::Square(v) => v.lod0_point(),
            Self::Track(v) => v.lod0_point(),
            Self::TrafficSpace(v) => v.lod0_point(),
            Self::Waterway(v) => v.lod0_point(),
            Self::HollowSpace(v) => v.lod0_point(),
            Self::Tunnel(v) => v.lod0_point(),
            Self::TunnelConstructiveElement(v) => v.lod0_point(),
            Self::TunnelFurniture(v) => v.lod0_point(),
            Self::TunnelInstallation(v) => v.lod0_point(),
            Self::TunnelPart(v) => v.lod0_point(),
            Self::PlantCover(v) => v.lod0_point(),
            Self::SolitaryVegetationObject(v) => v.lod0_point(),
            Self::WaterBody(v) => v.lod0_point(),
        }
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod3_multi_curve(),
            Self::OtherConstruction(v) => v.lod3_multi_curve(),
            Self::Window(v) => v.lod3_multi_curve(),
            Self::Bridge(v) => v.lod3_multi_curve(),
            Self::BridgeConstructiveElement(v) => v.lod3_multi_curve(),
            Self::BridgeFurniture(v) => v.lod3_multi_curve(),
            Self::BridgeInstallation(v) => v.lod3_multi_curve(),
            Self::BridgePart(v) => v.lod3_multi_curve(),
            Self::BridgeRoom(v) => v.lod3_multi_curve(),
            Self::Building(v) => v.lod3_multi_curve(),
            Self::BuildingConstructiveElement(v) => v.lod3_multi_curve(),
            Self::BuildingFurniture(v) => v.lod3_multi_curve(),
            Self::BuildingInstallation(v) => v.lod3_multi_curve(),
            Self::BuildingPart(v) => v.lod3_multi_curve(),
            Self::BuildingRoom(v) => v.lod3_multi_curve(),
            Self::CityFurniture(v) => v.lod3_multi_curve(),
            Self::GenericOccupiedSpace(v) => v.lod3_multi_curve(),
            Self::GenericUnoccupiedSpace(v) => v.lod3_multi_curve(),
            Self::AuxiliaryTrafficSpace(v) => v.lod3_multi_curve(),
            Self::ClearanceSpace(v) => v.lod3_multi_curve(),
            Self::Hole(v) => v.lod3_multi_curve(),
            Self::Intersection(v) => v.lod3_multi_curve(),
            Self::Railway(v) => v.lod3_multi_curve(),
            Self::Road(v) => v.lod3_multi_curve(),
            Self::Section(v) => v.lod3_multi_curve(),
            Self::Square(v) => v.lod3_multi_curve(),
            Self::Track(v) => v.lod3_multi_curve(),
            Self::TrafficSpace(v) => v.lod3_multi_curve(),
            Self::Waterway(v) => v.lod3_multi_curve(),
            Self::HollowSpace(v) => v.lod3_multi_curve(),
            Self::Tunnel(v) => v.lod3_multi_curve(),
            Self::TunnelConstructiveElement(v) => v.lod3_multi_curve(),
            Self::TunnelFurniture(v) => v.lod3_multi_curve(),
            Self::TunnelInstallation(v) => v.lod3_multi_curve(),
            Self::TunnelPart(v) => v.lod3_multi_curve(),
            Self::PlantCover(v) => v.lod3_multi_curve(),
            Self::SolitaryVegetationObject(v) => v.lod3_multi_curve(),
            Self::WaterBody(v) => v.lod3_multi_curve(),
        }
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::Door(v) => v.lod2_multi_surface(),
            Self::OtherConstruction(v) => v.lod2_multi_surface(),
            Self::Window(v) => v.lod2_multi_surface(),
            Self::Bridge(v) => v.lod2_multi_surface(),
            Self::BridgeConstructiveElement(v) => v.lod2_multi_surface(),
            Self::BridgeFurniture(v) => v.lod2_multi_surface(),
            Self::BridgeInstallation(v) => v.lod2_multi_surface(),
            Self::BridgePart(v) => v.lod2_multi_surface(),
            Self::BridgeRoom(v) => v.lod2_multi_surface(),
            Self::Building(v) => v.lod2_multi_surface(),
            Self::BuildingConstructiveElement(v) => v.lod2_multi_surface(),
            Self::BuildingFurniture(v) => v.lod2_multi_surface(),
            Self::BuildingInstallation(v) => v.lod2_multi_surface(),
            Self::BuildingPart(v) => v.lod2_multi_surface(),
            Self::BuildingRoom(v) => v.lod2_multi_surface(),
            Self::CityFurniture(v) => v.lod2_multi_surface(),
            Self::GenericOccupiedSpace(v) => v.lod2_multi_surface(),
            Self::GenericUnoccupiedSpace(v) => v.lod2_multi_surface(),
            Self::AuxiliaryTrafficSpace(v) => v.lod2_multi_surface(),
            Self::ClearanceSpace(v) => v.lod2_multi_surface(),
            Self::Hole(v) => v.lod2_multi_surface(),
            Self::Intersection(v) => v.lod2_multi_surface(),
            Self::Railway(v) => v.lod2_multi_surface(),
            Self::Road(v) => v.lod2_multi_surface(),
            Self::Section(v) => v.lod2_multi_surface(),
            Self::Square(v) => v.lod2_multi_surface(),
            Self::Track(v) => v.lod2_multi_surface(),
            Self::TrafficSpace(v) => v.lod2_multi_surface(),
            Self::Waterway(v) => v.lod2_multi_surface(),
            Self::HollowSpace(v) => v.lod2_multi_surface(),
            Self::Tunnel(v) => v.lod2_multi_surface(),
            Self::TunnelConstructiveElement(v) => v.lod2_multi_surface(),
            Self::TunnelFurniture(v) => v.lod2_multi_surface(),
            Self::TunnelInstallation(v) => v.lod2_multi_surface(),
            Self::TunnelPart(v) => v.lod2_multi_surface(),
            Self::PlantCover(v) => v.lod2_multi_surface(),
            Self::SolitaryVegetationObject(v) => v.lod2_multi_surface(),
            Self::WaterBody(v) => v.lod2_multi_surface(),
        }
    }
}
impl AbstractPhysicalSpaceTrait for AbstractPhysicalSpace {
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod3_terrain_intersection_curve(),
            Self::OtherConstruction(v) => v.lod3_terrain_intersection_curve(),
            Self::Window(v) => v.lod3_terrain_intersection_curve(),
            Self::Bridge(v) => v.lod3_terrain_intersection_curve(),
            Self::BridgeConstructiveElement(v) => v.lod3_terrain_intersection_curve(),
            Self::BridgeFurniture(v) => v.lod3_terrain_intersection_curve(),
            Self::BridgeInstallation(v) => v.lod3_terrain_intersection_curve(),
            Self::BridgePart(v) => v.lod3_terrain_intersection_curve(),
            Self::BridgeRoom(v) => v.lod3_terrain_intersection_curve(),
            Self::Building(v) => v.lod3_terrain_intersection_curve(),
            Self::BuildingConstructiveElement(v) => v.lod3_terrain_intersection_curve(),
            Self::BuildingFurniture(v) => v.lod3_terrain_intersection_curve(),
            Self::BuildingInstallation(v) => v.lod3_terrain_intersection_curve(),
            Self::BuildingPart(v) => v.lod3_terrain_intersection_curve(),
            Self::BuildingRoom(v) => v.lod3_terrain_intersection_curve(),
            Self::CityFurniture(v) => v.lod3_terrain_intersection_curve(),
            Self::GenericOccupiedSpace(v) => v.lod3_terrain_intersection_curve(),
            Self::GenericUnoccupiedSpace(v) => v.lod3_terrain_intersection_curve(),
            Self::AuxiliaryTrafficSpace(v) => v.lod3_terrain_intersection_curve(),
            Self::ClearanceSpace(v) => v.lod3_terrain_intersection_curve(),
            Self::Hole(v) => v.lod3_terrain_intersection_curve(),
            Self::Intersection(v) => v.lod3_terrain_intersection_curve(),
            Self::Railway(v) => v.lod3_terrain_intersection_curve(),
            Self::Road(v) => v.lod3_terrain_intersection_curve(),
            Self::Section(v) => v.lod3_terrain_intersection_curve(),
            Self::Square(v) => v.lod3_terrain_intersection_curve(),
            Self::Track(v) => v.lod3_terrain_intersection_curve(),
            Self::TrafficSpace(v) => v.lod3_terrain_intersection_curve(),
            Self::Waterway(v) => v.lod3_terrain_intersection_curve(),
            Self::HollowSpace(v) => v.lod3_terrain_intersection_curve(),
            Self::Tunnel(v) => v.lod3_terrain_intersection_curve(),
            Self::TunnelConstructiveElement(v) => v.lod3_terrain_intersection_curve(),
            Self::TunnelFurniture(v) => v.lod3_terrain_intersection_curve(),
            Self::TunnelInstallation(v) => v.lod3_terrain_intersection_curve(),
            Self::TunnelPart(v) => v.lod3_terrain_intersection_curve(),
            Self::PlantCover(v) => v.lod3_terrain_intersection_curve(),
            Self::SolitaryVegetationObject(v) => v.lod3_terrain_intersection_curve(),
            Self::WaterBody(v) => v.lod3_terrain_intersection_curve(),
        }
    }
    fn point_cloud(&self) -> Option<&AbstractPointCloud> {
        match self {
            Self::Door(v) => v.point_cloud(),
            Self::OtherConstruction(v) => v.point_cloud(),
            Self::Window(v) => v.point_cloud(),
            Self::Bridge(v) => v.point_cloud(),
            Self::BridgeConstructiveElement(v) => v.point_cloud(),
            Self::BridgeFurniture(v) => v.point_cloud(),
            Self::BridgeInstallation(v) => v.point_cloud(),
            Self::BridgePart(v) => v.point_cloud(),
            Self::BridgeRoom(v) => v.point_cloud(),
            Self::Building(v) => v.point_cloud(),
            Self::BuildingConstructiveElement(v) => v.point_cloud(),
            Self::BuildingFurniture(v) => v.point_cloud(),
            Self::BuildingInstallation(v) => v.point_cloud(),
            Self::BuildingPart(v) => v.point_cloud(),
            Self::BuildingRoom(v) => v.point_cloud(),
            Self::CityFurniture(v) => v.point_cloud(),
            Self::GenericOccupiedSpace(v) => v.point_cloud(),
            Self::GenericUnoccupiedSpace(v) => v.point_cloud(),
            Self::AuxiliaryTrafficSpace(v) => v.point_cloud(),
            Self::ClearanceSpace(v) => v.point_cloud(),
            Self::Hole(v) => v.point_cloud(),
            Self::Intersection(v) => v.point_cloud(),
            Self::Railway(v) => v.point_cloud(),
            Self::Road(v) => v.point_cloud(),
            Self::Section(v) => v.point_cloud(),
            Self::Square(v) => v.point_cloud(),
            Self::Track(v) => v.point_cloud(),
            Self::TrafficSpace(v) => v.point_cloud(),
            Self::Waterway(v) => v.point_cloud(),
            Self::HollowSpace(v) => v.point_cloud(),
            Self::Tunnel(v) => v.point_cloud(),
            Self::TunnelConstructiveElement(v) => v.point_cloud(),
            Self::TunnelFurniture(v) => v.point_cloud(),
            Self::TunnelInstallation(v) => v.point_cloud(),
            Self::TunnelPart(v) => v.point_cloud(),
            Self::PlantCover(v) => v.point_cloud(),
            Self::SolitaryVegetationObject(v) => v.point_cloud(),
            Self::WaterBody(v) => v.point_cloud(),
        }
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod1_terrain_intersection_curve(),
            Self::OtherConstruction(v) => v.lod1_terrain_intersection_curve(),
            Self::Window(v) => v.lod1_terrain_intersection_curve(),
            Self::Bridge(v) => v.lod1_terrain_intersection_curve(),
            Self::BridgeConstructiveElement(v) => v.lod1_terrain_intersection_curve(),
            Self::BridgeFurniture(v) => v.lod1_terrain_intersection_curve(),
            Self::BridgeInstallation(v) => v.lod1_terrain_intersection_curve(),
            Self::BridgePart(v) => v.lod1_terrain_intersection_curve(),
            Self::BridgeRoom(v) => v.lod1_terrain_intersection_curve(),
            Self::Building(v) => v.lod1_terrain_intersection_curve(),
            Self::BuildingConstructiveElement(v) => v.lod1_terrain_intersection_curve(),
            Self::BuildingFurniture(v) => v.lod1_terrain_intersection_curve(),
            Self::BuildingInstallation(v) => v.lod1_terrain_intersection_curve(),
            Self::BuildingPart(v) => v.lod1_terrain_intersection_curve(),
            Self::BuildingRoom(v) => v.lod1_terrain_intersection_curve(),
            Self::CityFurniture(v) => v.lod1_terrain_intersection_curve(),
            Self::GenericOccupiedSpace(v) => v.lod1_terrain_intersection_curve(),
            Self::GenericUnoccupiedSpace(v) => v.lod1_terrain_intersection_curve(),
            Self::AuxiliaryTrafficSpace(v) => v.lod1_terrain_intersection_curve(),
            Self::ClearanceSpace(v) => v.lod1_terrain_intersection_curve(),
            Self::Hole(v) => v.lod1_terrain_intersection_curve(),
            Self::Intersection(v) => v.lod1_terrain_intersection_curve(),
            Self::Railway(v) => v.lod1_terrain_intersection_curve(),
            Self::Road(v) => v.lod1_terrain_intersection_curve(),
            Self::Section(v) => v.lod1_terrain_intersection_curve(),
            Self::Square(v) => v.lod1_terrain_intersection_curve(),
            Self::Track(v) => v.lod1_terrain_intersection_curve(),
            Self::TrafficSpace(v) => v.lod1_terrain_intersection_curve(),
            Self::Waterway(v) => v.lod1_terrain_intersection_curve(),
            Self::HollowSpace(v) => v.lod1_terrain_intersection_curve(),
            Self::Tunnel(v) => v.lod1_terrain_intersection_curve(),
            Self::TunnelConstructiveElement(v) => v.lod1_terrain_intersection_curve(),
            Self::TunnelFurniture(v) => v.lod1_terrain_intersection_curve(),
            Self::TunnelInstallation(v) => v.lod1_terrain_intersection_curve(),
            Self::TunnelPart(v) => v.lod1_terrain_intersection_curve(),
            Self::PlantCover(v) => v.lod1_terrain_intersection_curve(),
            Self::SolitaryVegetationObject(v) => v.lod1_terrain_intersection_curve(),
            Self::WaterBody(v) => v.lod1_terrain_intersection_curve(),
        }
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod2_terrain_intersection_curve(),
            Self::OtherConstruction(v) => v.lod2_terrain_intersection_curve(),
            Self::Window(v) => v.lod2_terrain_intersection_curve(),
            Self::Bridge(v) => v.lod2_terrain_intersection_curve(),
            Self::BridgeConstructiveElement(v) => v.lod2_terrain_intersection_curve(),
            Self::BridgeFurniture(v) => v.lod2_terrain_intersection_curve(),
            Self::BridgeInstallation(v) => v.lod2_terrain_intersection_curve(),
            Self::BridgePart(v) => v.lod2_terrain_intersection_curve(),
            Self::BridgeRoom(v) => v.lod2_terrain_intersection_curve(),
            Self::Building(v) => v.lod2_terrain_intersection_curve(),
            Self::BuildingConstructiveElement(v) => v.lod2_terrain_intersection_curve(),
            Self::BuildingFurniture(v) => v.lod2_terrain_intersection_curve(),
            Self::BuildingInstallation(v) => v.lod2_terrain_intersection_curve(),
            Self::BuildingPart(v) => v.lod2_terrain_intersection_curve(),
            Self::BuildingRoom(v) => v.lod2_terrain_intersection_curve(),
            Self::CityFurniture(v) => v.lod2_terrain_intersection_curve(),
            Self::GenericOccupiedSpace(v) => v.lod2_terrain_intersection_curve(),
            Self::GenericUnoccupiedSpace(v) => v.lod2_terrain_intersection_curve(),
            Self::AuxiliaryTrafficSpace(v) => v.lod2_terrain_intersection_curve(),
            Self::ClearanceSpace(v) => v.lod2_terrain_intersection_curve(),
            Self::Hole(v) => v.lod2_terrain_intersection_curve(),
            Self::Intersection(v) => v.lod2_terrain_intersection_curve(),
            Self::Railway(v) => v.lod2_terrain_intersection_curve(),
            Self::Road(v) => v.lod2_terrain_intersection_curve(),
            Self::Section(v) => v.lod2_terrain_intersection_curve(),
            Self::Square(v) => v.lod2_terrain_intersection_curve(),
            Self::Track(v) => v.lod2_terrain_intersection_curve(),
            Self::TrafficSpace(v) => v.lod2_terrain_intersection_curve(),
            Self::Waterway(v) => v.lod2_terrain_intersection_curve(),
            Self::HollowSpace(v) => v.lod2_terrain_intersection_curve(),
            Self::Tunnel(v) => v.lod2_terrain_intersection_curve(),
            Self::TunnelConstructiveElement(v) => v.lod2_terrain_intersection_curve(),
            Self::TunnelFurniture(v) => v.lod2_terrain_intersection_curve(),
            Self::TunnelInstallation(v) => v.lod2_terrain_intersection_curve(),
            Self::TunnelPart(v) => v.lod2_terrain_intersection_curve(),
            Self::PlantCover(v) => v.lod2_terrain_intersection_curve(),
            Self::SolitaryVegetationObject(v) => v.lod2_terrain_intersection_curve(),
            Self::WaterBody(v) => v.lod2_terrain_intersection_curve(),
        }
    }
}
impl From<Door> for AbstractPhysicalSpace {
    fn from(v: Door) -> Self {
        Self::Door(Box::new(v))
    }
}
impl From<OtherConstruction> for AbstractPhysicalSpace {
    fn from(v: OtherConstruction) -> Self {
        Self::OtherConstruction(Box::new(v))
    }
}
impl From<Window> for AbstractPhysicalSpace {
    fn from(v: Window) -> Self {
        Self::Window(Box::new(v))
    }
}
impl From<Bridge> for AbstractPhysicalSpace {
    fn from(v: Bridge) -> Self {
        Self::Bridge(Box::new(v))
    }
}
impl From<BridgeConstructiveElement> for AbstractPhysicalSpace {
    fn from(v: BridgeConstructiveElement) -> Self {
        Self::BridgeConstructiveElement(Box::new(v))
    }
}
impl From<BridgeFurniture> for AbstractPhysicalSpace {
    fn from(v: BridgeFurniture) -> Self {
        Self::BridgeFurniture(Box::new(v))
    }
}
impl From<BridgeInstallation> for AbstractPhysicalSpace {
    fn from(v: BridgeInstallation) -> Self {
        Self::BridgeInstallation(Box::new(v))
    }
}
impl From<BridgePart> for AbstractPhysicalSpace {
    fn from(v: BridgePart) -> Self {
        Self::BridgePart(Box::new(v))
    }
}
impl From<BridgeRoom> for AbstractPhysicalSpace {
    fn from(v: BridgeRoom) -> Self {
        Self::BridgeRoom(Box::new(v))
    }
}
impl From<Building> for AbstractPhysicalSpace {
    fn from(v: Building) -> Self {
        Self::Building(Box::new(v))
    }
}
impl From<BuildingConstructiveElement> for AbstractPhysicalSpace {
    fn from(v: BuildingConstructiveElement) -> Self {
        Self::BuildingConstructiveElement(Box::new(v))
    }
}
impl From<BuildingFurniture> for AbstractPhysicalSpace {
    fn from(v: BuildingFurniture) -> Self {
        Self::BuildingFurniture(Box::new(v))
    }
}
impl From<BuildingInstallation> for AbstractPhysicalSpace {
    fn from(v: BuildingInstallation) -> Self {
        Self::BuildingInstallation(Box::new(v))
    }
}
impl From<BuildingPart> for AbstractPhysicalSpace {
    fn from(v: BuildingPart) -> Self {
        Self::BuildingPart(Box::new(v))
    }
}
impl From<BuildingRoom> for AbstractPhysicalSpace {
    fn from(v: BuildingRoom) -> Self {
        Self::BuildingRoom(Box::new(v))
    }
}
impl From<CityFurniture> for AbstractPhysicalSpace {
    fn from(v: CityFurniture) -> Self {
        Self::CityFurniture(Box::new(v))
    }
}
impl From<GenericOccupiedSpace> for AbstractPhysicalSpace {
    fn from(v: GenericOccupiedSpace) -> Self {
        Self::GenericOccupiedSpace(Box::new(v))
    }
}
impl From<GenericUnoccupiedSpace> for AbstractPhysicalSpace {
    fn from(v: GenericUnoccupiedSpace) -> Self {
        Self::GenericUnoccupiedSpace(Box::new(v))
    }
}
impl From<AuxiliaryTrafficSpace> for AbstractPhysicalSpace {
    fn from(v: AuxiliaryTrafficSpace) -> Self {
        Self::AuxiliaryTrafficSpace(Box::new(v))
    }
}
impl From<ClearanceSpace> for AbstractPhysicalSpace {
    fn from(v: ClearanceSpace) -> Self {
        Self::ClearanceSpace(Box::new(v))
    }
}
impl From<Hole> for AbstractPhysicalSpace {
    fn from(v: Hole) -> Self {
        Self::Hole(Box::new(v))
    }
}
impl From<Intersection> for AbstractPhysicalSpace {
    fn from(v: Intersection) -> Self {
        Self::Intersection(Box::new(v))
    }
}
impl From<Railway> for AbstractPhysicalSpace {
    fn from(v: Railway) -> Self {
        Self::Railway(Box::new(v))
    }
}
impl From<Road> for AbstractPhysicalSpace {
    fn from(v: Road) -> Self {
        Self::Road(Box::new(v))
    }
}
impl From<Section> for AbstractPhysicalSpace {
    fn from(v: Section) -> Self {
        Self::Section(Box::new(v))
    }
}
impl From<Square> for AbstractPhysicalSpace {
    fn from(v: Square) -> Self {
        Self::Square(Box::new(v))
    }
}
impl From<Track> for AbstractPhysicalSpace {
    fn from(v: Track) -> Self {
        Self::Track(Box::new(v))
    }
}
impl From<TrafficSpace> for AbstractPhysicalSpace {
    fn from(v: TrafficSpace) -> Self {
        Self::TrafficSpace(Box::new(v))
    }
}
impl From<Waterway> for AbstractPhysicalSpace {
    fn from(v: Waterway) -> Self {
        Self::Waterway(Box::new(v))
    }
}
impl From<HollowSpace> for AbstractPhysicalSpace {
    fn from(v: HollowSpace) -> Self {
        Self::HollowSpace(Box::new(v))
    }
}
impl From<Tunnel> for AbstractPhysicalSpace {
    fn from(v: Tunnel) -> Self {
        Self::Tunnel(Box::new(v))
    }
}
impl From<TunnelConstructiveElement> for AbstractPhysicalSpace {
    fn from(v: TunnelConstructiveElement) -> Self {
        Self::TunnelConstructiveElement(Box::new(v))
    }
}
impl From<TunnelFurniture> for AbstractPhysicalSpace {
    fn from(v: TunnelFurniture) -> Self {
        Self::TunnelFurniture(Box::new(v))
    }
}
impl From<TunnelInstallation> for AbstractPhysicalSpace {
    fn from(v: TunnelInstallation) -> Self {
        Self::TunnelInstallation(Box::new(v))
    }
}
impl From<TunnelPart> for AbstractPhysicalSpace {
    fn from(v: TunnelPart) -> Self {
        Self::TunnelPart(Box::new(v))
    }
}
impl From<PlantCover> for AbstractPhysicalSpace {
    fn from(v: PlantCover) -> Self {
        Self::PlantCover(Box::new(v))
    }
}
impl From<SolitaryVegetationObject> for AbstractPhysicalSpace {
    fn from(v: SolitaryVegetationObject) -> Self {
        Self::SolitaryVegetationObject(Box::new(v))
    }
}
impl From<WaterBody> for AbstractPhysicalSpace {
    fn from(v: WaterBody) -> Self {
        Self::WaterBody(Box::new(v))
    }
}
pub trait AbstractPhysicalSpaceAccessors {
    fn doors(&self) -> impl Iterator<Item = &Door>;
    fn other_constructions(&self) -> impl Iterator<Item = &OtherConstruction>;
    fn windows(&self) -> impl Iterator<Item = &Window>;
    fn bridges(&self) -> impl Iterator<Item = &Bridge>;
    fn bridge_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BridgeConstructiveElement>;
    fn bridge_furnitures(&self) -> impl Iterator<Item = &BridgeFurniture>;
    fn bridge_installations(&self) -> impl Iterator<Item = &BridgeInstallation>;
    fn bridge_parts(&self) -> impl Iterator<Item = &BridgePart>;
    fn bridge_rooms(&self) -> impl Iterator<Item = &BridgeRoom>;
    fn buildings(&self) -> impl Iterator<Item = &Building>;
    fn building_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BuildingConstructiveElement>;
    fn building_furnitures(&self) -> impl Iterator<Item = &BuildingFurniture>;
    fn building_installations(&self) -> impl Iterator<Item = &BuildingInstallation>;
    fn building_parts(&self) -> impl Iterator<Item = &BuildingPart>;
    fn building_rooms(&self) -> impl Iterator<Item = &BuildingRoom>;
    fn city_furnitures(&self) -> impl Iterator<Item = &CityFurniture>;
    fn generic_occupied_spaces(&self) -> impl Iterator<Item = &GenericOccupiedSpace>;
    fn generic_unoccupied_spaces(&self) -> impl Iterator<Item = &GenericUnoccupiedSpace>;
    fn auxiliary_traffic_spaces(&self) -> impl Iterator<Item = &AuxiliaryTrafficSpace>;
    fn clearance_spaces(&self) -> impl Iterator<Item = &ClearanceSpace>;
    fn holes(&self) -> impl Iterator<Item = &Hole>;
    fn intersections(&self) -> impl Iterator<Item = &Intersection>;
    fn railways(&self) -> impl Iterator<Item = &Railway>;
    fn roads(&self) -> impl Iterator<Item = &Road>;
    fn sections(&self) -> impl Iterator<Item = &Section>;
    fn squares(&self) -> impl Iterator<Item = &Square>;
    fn tracks(&self) -> impl Iterator<Item = &Track>;
    fn traffic_spaces(&self) -> impl Iterator<Item = &TrafficSpace>;
    fn waterways(&self) -> impl Iterator<Item = &Waterway>;
    fn hollow_spaces(&self) -> impl Iterator<Item = &HollowSpace>;
    fn tunnels(&self) -> impl Iterator<Item = &Tunnel>;
    fn tunnel_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &TunnelConstructiveElement>;
    fn tunnel_furnitures(&self) -> impl Iterator<Item = &TunnelFurniture>;
    fn tunnel_installations(&self) -> impl Iterator<Item = &TunnelInstallation>;
    fn tunnel_parts(&self) -> impl Iterator<Item = &TunnelPart>;
    fn plant_covers(&self) -> impl Iterator<Item = &PlantCover>;
    fn solitary_vegetation_objects(
        &self,
    ) -> impl Iterator<Item = &SolitaryVegetationObject>;
    fn water_bodys(&self) -> impl Iterator<Item = &WaterBody>;
}
impl AbstractPhysicalSpaceAccessors for [AbstractPhysicalSpace] {
    fn doors(&self) -> impl Iterator<Item = &Door> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::Door(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn other_constructions(&self) -> impl Iterator<Item = &OtherConstruction> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::OtherConstruction(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn windows(&self) -> impl Iterator<Item = &Window> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::Window(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridges(&self) -> impl Iterator<Item = &Bridge> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::Bridge(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BridgeConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::BridgeConstructiveElement(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_furnitures(&self) -> impl Iterator<Item = &BridgeFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::BridgeFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_installations(&self) -> impl Iterator<Item = &BridgeInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::BridgeInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_parts(&self) -> impl Iterator<Item = &BridgePart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::BridgePart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_rooms(&self) -> impl Iterator<Item = &BridgeRoom> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::BridgeRoom(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn buildings(&self) -> impl Iterator<Item = &Building> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::Building(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BuildingConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::BuildingConstructiveElement(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_furnitures(&self) -> impl Iterator<Item = &BuildingFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::BuildingFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_installations(&self) -> impl Iterator<Item = &BuildingInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::BuildingInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_parts(&self) -> impl Iterator<Item = &BuildingPart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::BuildingPart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_rooms(&self) -> impl Iterator<Item = &BuildingRoom> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::BuildingRoom(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn city_furnitures(&self) -> impl Iterator<Item = &CityFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::CityFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_occupied_spaces(&self) -> impl Iterator<Item = &GenericOccupiedSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::GenericOccupiedSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_unoccupied_spaces(
        &self,
    ) -> impl Iterator<Item = &GenericUnoccupiedSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::GenericUnoccupiedSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn auxiliary_traffic_spaces(&self) -> impl Iterator<Item = &AuxiliaryTrafficSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::AuxiliaryTrafficSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn clearance_spaces(&self) -> impl Iterator<Item = &ClearanceSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::ClearanceSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn holes(&self) -> impl Iterator<Item = &Hole> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::Hole(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn intersections(&self) -> impl Iterator<Item = &Intersection> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::Intersection(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn railways(&self) -> impl Iterator<Item = &Railway> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::Railway(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn roads(&self) -> impl Iterator<Item = &Road> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::Road(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn sections(&self) -> impl Iterator<Item = &Section> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::Section(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn squares(&self) -> impl Iterator<Item = &Square> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::Square(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tracks(&self) -> impl Iterator<Item = &Track> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::Track(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn traffic_spaces(&self) -> impl Iterator<Item = &TrafficSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::TrafficSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn waterways(&self) -> impl Iterator<Item = &Waterway> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::Waterway(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn hollow_spaces(&self) -> impl Iterator<Item = &HollowSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::HollowSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnels(&self) -> impl Iterator<Item = &Tunnel> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::Tunnel(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &TunnelConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::TunnelConstructiveElement(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_furnitures(&self) -> impl Iterator<Item = &TunnelFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::TunnelFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_installations(&self) -> impl Iterator<Item = &TunnelInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::TunnelInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_parts(&self) -> impl Iterator<Item = &TunnelPart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::TunnelPart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn plant_covers(&self) -> impl Iterator<Item = &PlantCover> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::PlantCover(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn solitary_vegetation_objects(
        &self,
    ) -> impl Iterator<Item = &SolitaryVegetationObject> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::SolitaryVegetationObject(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn water_bodys(&self) -> impl Iterator<Item = &WaterBody> {
        self.iter()
            .filter_map(|item| match item {
                AbstractPhysicalSpace::WaterBody(v) => Some(v.as_ref()),
                _ => None,
            })
    }
}
pub trait AbstractThematicSurfaceTrait: AbstractSpaceBoundaryTrait {
    fn area(&self) -> &[QualifiedArea];
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface>;
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface>;
    fn point_cloud(&self) -> Option<&AbstractPointCloud>;
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve>;
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface>;
    fn lod1_multi_surface(&self) -> Option<&crate::geometry::MultiSurface>;
}
#[derive(Debug, Clone)]
pub enum AbstractThematicSurface {
    CeilingSurface(Box<CeilingSurface>),
    DoorSurface(Box<DoorSurface>),
    FloorSurface(Box<FloorSurface>),
    GroundSurface(Box<GroundSurface>),
    InteriorWallSurface(Box<InteriorWallSurface>),
    OuterCeilingSurface(Box<OuterCeilingSurface>),
    OuterFloorSurface(Box<OuterFloorSurface>),
    RoofSurface(Box<RoofSurface>),
    WallSurface(Box<WallSurface>),
    WindowSurface(Box<WindowSurface>),
    ClosureSurface(Box<ClosureSurface>),
    GenericThematicSurface(Box<GenericThematicSurface>),
    LandUse(Box<LandUse>),
    AuxiliaryTrafficArea(Box<AuxiliaryTrafficArea>),
    HoleSurface(Box<HoleSurface>),
    Marking(Box<Marking>),
    TrafficArea(Box<TrafficArea>),
    WaterGroundSurface(Box<WaterGroundSurface>),
    WaterSurface(Box<WaterSurface>),
}
impl Default for AbstractThematicSurface {
    fn default() -> Self {
        Self::CeilingSurface(Box::default())
    }
}
impl AbstractFeatureTrait for AbstractThematicSurface {
    fn feature_id(&self) -> &ID {
        match self {
            Self::CeilingSurface(v) => v.feature_id(),
            Self::DoorSurface(v) => v.feature_id(),
            Self::FloorSurface(v) => v.feature_id(),
            Self::GroundSurface(v) => v.feature_id(),
            Self::InteriorWallSurface(v) => v.feature_id(),
            Self::OuterCeilingSurface(v) => v.feature_id(),
            Self::OuterFloorSurface(v) => v.feature_id(),
            Self::RoofSurface(v) => v.feature_id(),
            Self::WallSurface(v) => v.feature_id(),
            Self::WindowSurface(v) => v.feature_id(),
            Self::ClosureSurface(v) => v.feature_id(),
            Self::GenericThematicSurface(v) => v.feature_id(),
            Self::LandUse(v) => v.feature_id(),
            Self::AuxiliaryTrafficArea(v) => v.feature_id(),
            Self::HoleSurface(v) => v.feature_id(),
            Self::Marking(v) => v.feature_id(),
            Self::TrafficArea(v) => v.feature_id(),
            Self::WaterGroundSurface(v) => v.feature_id(),
            Self::WaterSurface(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.identifier(),
            Self::DoorSurface(v) => v.identifier(),
            Self::FloorSurface(v) => v.identifier(),
            Self::GroundSurface(v) => v.identifier(),
            Self::InteriorWallSurface(v) => v.identifier(),
            Self::OuterCeilingSurface(v) => v.identifier(),
            Self::OuterFloorSurface(v) => v.identifier(),
            Self::RoofSurface(v) => v.identifier(),
            Self::WallSurface(v) => v.identifier(),
            Self::WindowSurface(v) => v.identifier(),
            Self::ClosureSurface(v) => v.identifier(),
            Self::GenericThematicSurface(v) => v.identifier(),
            Self::LandUse(v) => v.identifier(),
            Self::AuxiliaryTrafficArea(v) => v.identifier(),
            Self::HoleSurface(v) => v.identifier(),
            Self::Marking(v) => v.identifier(),
            Self::TrafficArea(v) => v.identifier(),
            Self::WaterGroundSurface(v) => v.identifier(),
            Self::WaterSurface(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::CeilingSurface(v) => v.name(),
            Self::DoorSurface(v) => v.name(),
            Self::FloorSurface(v) => v.name(),
            Self::GroundSurface(v) => v.name(),
            Self::InteriorWallSurface(v) => v.name(),
            Self::OuterCeilingSurface(v) => v.name(),
            Self::OuterFloorSurface(v) => v.name(),
            Self::RoofSurface(v) => v.name(),
            Self::WallSurface(v) => v.name(),
            Self::WindowSurface(v) => v.name(),
            Self::ClosureSurface(v) => v.name(),
            Self::GenericThematicSurface(v) => v.name(),
            Self::LandUse(v) => v.name(),
            Self::AuxiliaryTrafficArea(v) => v.name(),
            Self::HoleSurface(v) => v.name(),
            Self::Marking(v) => v.name(),
            Self::TrafficArea(v) => v.name(),
            Self::WaterGroundSurface(v) => v.name(),
            Self::WaterSurface(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.description(),
            Self::DoorSurface(v) => v.description(),
            Self::FloorSurface(v) => v.description(),
            Self::GroundSurface(v) => v.description(),
            Self::InteriorWallSurface(v) => v.description(),
            Self::OuterCeilingSurface(v) => v.description(),
            Self::OuterFloorSurface(v) => v.description(),
            Self::RoofSurface(v) => v.description(),
            Self::WallSurface(v) => v.description(),
            Self::WindowSurface(v) => v.description(),
            Self::ClosureSurface(v) => v.description(),
            Self::GenericThematicSurface(v) => v.description(),
            Self::LandUse(v) => v.description(),
            Self::AuxiliaryTrafficArea(v) => v.description(),
            Self::HoleSurface(v) => v.description(),
            Self::Marking(v) => v.description(),
            Self::TrafficArea(v) => v.description(),
            Self::WaterGroundSurface(v) => v.description(),
            Self::WaterSurface(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractThematicSurface {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.creation_date(),
            Self::DoorSurface(v) => v.creation_date(),
            Self::FloorSurface(v) => v.creation_date(),
            Self::GroundSurface(v) => v.creation_date(),
            Self::InteriorWallSurface(v) => v.creation_date(),
            Self::OuterCeilingSurface(v) => v.creation_date(),
            Self::OuterFloorSurface(v) => v.creation_date(),
            Self::RoofSurface(v) => v.creation_date(),
            Self::WallSurface(v) => v.creation_date(),
            Self::WindowSurface(v) => v.creation_date(),
            Self::ClosureSurface(v) => v.creation_date(),
            Self::GenericThematicSurface(v) => v.creation_date(),
            Self::LandUse(v) => v.creation_date(),
            Self::AuxiliaryTrafficArea(v) => v.creation_date(),
            Self::HoleSurface(v) => v.creation_date(),
            Self::Marking(v) => v.creation_date(),
            Self::TrafficArea(v) => v.creation_date(),
            Self::WaterGroundSurface(v) => v.creation_date(),
            Self::WaterSurface(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.termination_date(),
            Self::DoorSurface(v) => v.termination_date(),
            Self::FloorSurface(v) => v.termination_date(),
            Self::GroundSurface(v) => v.termination_date(),
            Self::InteriorWallSurface(v) => v.termination_date(),
            Self::OuterCeilingSurface(v) => v.termination_date(),
            Self::OuterFloorSurface(v) => v.termination_date(),
            Self::RoofSurface(v) => v.termination_date(),
            Self::WallSurface(v) => v.termination_date(),
            Self::WindowSurface(v) => v.termination_date(),
            Self::ClosureSurface(v) => v.termination_date(),
            Self::GenericThematicSurface(v) => v.termination_date(),
            Self::LandUse(v) => v.termination_date(),
            Self::AuxiliaryTrafficArea(v) => v.termination_date(),
            Self::HoleSurface(v) => v.termination_date(),
            Self::Marking(v) => v.termination_date(),
            Self::TrafficArea(v) => v.termination_date(),
            Self::WaterGroundSurface(v) => v.termination_date(),
            Self::WaterSurface(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.valid_from(),
            Self::DoorSurface(v) => v.valid_from(),
            Self::FloorSurface(v) => v.valid_from(),
            Self::GroundSurface(v) => v.valid_from(),
            Self::InteriorWallSurface(v) => v.valid_from(),
            Self::OuterCeilingSurface(v) => v.valid_from(),
            Self::OuterFloorSurface(v) => v.valid_from(),
            Self::RoofSurface(v) => v.valid_from(),
            Self::WallSurface(v) => v.valid_from(),
            Self::WindowSurface(v) => v.valid_from(),
            Self::ClosureSurface(v) => v.valid_from(),
            Self::GenericThematicSurface(v) => v.valid_from(),
            Self::LandUse(v) => v.valid_from(),
            Self::AuxiliaryTrafficArea(v) => v.valid_from(),
            Self::HoleSurface(v) => v.valid_from(),
            Self::Marking(v) => v.valid_from(),
            Self::TrafficArea(v) => v.valid_from(),
            Self::WaterGroundSurface(v) => v.valid_from(),
            Self::WaterSurface(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.valid_to(),
            Self::DoorSurface(v) => v.valid_to(),
            Self::FloorSurface(v) => v.valid_to(),
            Self::GroundSurface(v) => v.valid_to(),
            Self::InteriorWallSurface(v) => v.valid_to(),
            Self::OuterCeilingSurface(v) => v.valid_to(),
            Self::OuterFloorSurface(v) => v.valid_to(),
            Self::RoofSurface(v) => v.valid_to(),
            Self::WallSurface(v) => v.valid_to(),
            Self::WindowSurface(v) => v.valid_to(),
            Self::ClosureSurface(v) => v.valid_to(),
            Self::GenericThematicSurface(v) => v.valid_to(),
            Self::LandUse(v) => v.valid_to(),
            Self::AuxiliaryTrafficArea(v) => v.valid_to(),
            Self::HoleSurface(v) => v.valid_to(),
            Self::Marking(v) => v.valid_to(),
            Self::TrafficArea(v) => v.valid_to(),
            Self::WaterGroundSurface(v) => v.valid_to(),
            Self::WaterSurface(v) => v.valid_to(),
        }
    }
}
impl AbstractCityObjectTrait for AbstractThematicSurface {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::CeilingSurface(v) => v.relative_to_terrain(),
            Self::DoorSurface(v) => v.relative_to_terrain(),
            Self::FloorSurface(v) => v.relative_to_terrain(),
            Self::GroundSurface(v) => v.relative_to_terrain(),
            Self::InteriorWallSurface(v) => v.relative_to_terrain(),
            Self::OuterCeilingSurface(v) => v.relative_to_terrain(),
            Self::OuterFloorSurface(v) => v.relative_to_terrain(),
            Self::RoofSurface(v) => v.relative_to_terrain(),
            Self::WallSurface(v) => v.relative_to_terrain(),
            Self::WindowSurface(v) => v.relative_to_terrain(),
            Self::ClosureSurface(v) => v.relative_to_terrain(),
            Self::GenericThematicSurface(v) => v.relative_to_terrain(),
            Self::LandUse(v) => v.relative_to_terrain(),
            Self::AuxiliaryTrafficArea(v) => v.relative_to_terrain(),
            Self::HoleSurface(v) => v.relative_to_terrain(),
            Self::Marking(v) => v.relative_to_terrain(),
            Self::TrafficArea(v) => v.relative_to_terrain(),
            Self::WaterGroundSurface(v) => v.relative_to_terrain(),
            Self::WaterSurface(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::CeilingSurface(v) => v.relative_to_water(),
            Self::DoorSurface(v) => v.relative_to_water(),
            Self::FloorSurface(v) => v.relative_to_water(),
            Self::GroundSurface(v) => v.relative_to_water(),
            Self::InteriorWallSurface(v) => v.relative_to_water(),
            Self::OuterCeilingSurface(v) => v.relative_to_water(),
            Self::OuterFloorSurface(v) => v.relative_to_water(),
            Self::RoofSurface(v) => v.relative_to_water(),
            Self::WallSurface(v) => v.relative_to_water(),
            Self::WindowSurface(v) => v.relative_to_water(),
            Self::ClosureSurface(v) => v.relative_to_water(),
            Self::GenericThematicSurface(v) => v.relative_to_water(),
            Self::LandUse(v) => v.relative_to_water(),
            Self::AuxiliaryTrafficArea(v) => v.relative_to_water(),
            Self::HoleSurface(v) => v.relative_to_water(),
            Self::Marking(v) => v.relative_to_water(),
            Self::TrafficArea(v) => v.relative_to_water(),
            Self::WaterGroundSurface(v) => v.relative_to_water(),
            Self::WaterSurface(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::CeilingSurface(v) => v.appearance(),
            Self::DoorSurface(v) => v.appearance(),
            Self::FloorSurface(v) => v.appearance(),
            Self::GroundSurface(v) => v.appearance(),
            Self::InteriorWallSurface(v) => v.appearance(),
            Self::OuterCeilingSurface(v) => v.appearance(),
            Self::OuterFloorSurface(v) => v.appearance(),
            Self::RoofSurface(v) => v.appearance(),
            Self::WallSurface(v) => v.appearance(),
            Self::WindowSurface(v) => v.appearance(),
            Self::ClosureSurface(v) => v.appearance(),
            Self::GenericThematicSurface(v) => v.appearance(),
            Self::LandUse(v) => v.appearance(),
            Self::AuxiliaryTrafficArea(v) => v.appearance(),
            Self::HoleSurface(v) => v.appearance(),
            Self::Marking(v) => v.appearance(),
            Self::TrafficArea(v) => v.appearance(),
            Self::WaterGroundSurface(v) => v.appearance(),
            Self::WaterSurface(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::CeilingSurface(v) => v.generalizes_to(),
            Self::DoorSurface(v) => v.generalizes_to(),
            Self::FloorSurface(v) => v.generalizes_to(),
            Self::GroundSurface(v) => v.generalizes_to(),
            Self::InteriorWallSurface(v) => v.generalizes_to(),
            Self::OuterCeilingSurface(v) => v.generalizes_to(),
            Self::OuterFloorSurface(v) => v.generalizes_to(),
            Self::RoofSurface(v) => v.generalizes_to(),
            Self::WallSurface(v) => v.generalizes_to(),
            Self::WindowSurface(v) => v.generalizes_to(),
            Self::ClosureSurface(v) => v.generalizes_to(),
            Self::GenericThematicSurface(v) => v.generalizes_to(),
            Self::LandUse(v) => v.generalizes_to(),
            Self::AuxiliaryTrafficArea(v) => v.generalizes_to(),
            Self::HoleSurface(v) => v.generalizes_to(),
            Self::Marking(v) => v.generalizes_to(),
            Self::TrafficArea(v) => v.generalizes_to(),
            Self::WaterGroundSurface(v) => v.generalizes_to(),
            Self::WaterSurface(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::CeilingSurface(v) => v.external_reference(),
            Self::DoorSurface(v) => v.external_reference(),
            Self::FloorSurface(v) => v.external_reference(),
            Self::GroundSurface(v) => v.external_reference(),
            Self::InteriorWallSurface(v) => v.external_reference(),
            Self::OuterCeilingSurface(v) => v.external_reference(),
            Self::OuterFloorSurface(v) => v.external_reference(),
            Self::RoofSurface(v) => v.external_reference(),
            Self::WallSurface(v) => v.external_reference(),
            Self::WindowSurface(v) => v.external_reference(),
            Self::ClosureSurface(v) => v.external_reference(),
            Self::GenericThematicSurface(v) => v.external_reference(),
            Self::LandUse(v) => v.external_reference(),
            Self::AuxiliaryTrafficArea(v) => v.external_reference(),
            Self::HoleSurface(v) => v.external_reference(),
            Self::Marking(v) => v.external_reference(),
            Self::TrafficArea(v) => v.external_reference(),
            Self::WaterGroundSurface(v) => v.external_reference(),
            Self::WaterSurface(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::CeilingSurface(v) => v.related_to(),
            Self::DoorSurface(v) => v.related_to(),
            Self::FloorSurface(v) => v.related_to(),
            Self::GroundSurface(v) => v.related_to(),
            Self::InteriorWallSurface(v) => v.related_to(),
            Self::OuterCeilingSurface(v) => v.related_to(),
            Self::OuterFloorSurface(v) => v.related_to(),
            Self::RoofSurface(v) => v.related_to(),
            Self::WallSurface(v) => v.related_to(),
            Self::WindowSurface(v) => v.related_to(),
            Self::ClosureSurface(v) => v.related_to(),
            Self::GenericThematicSurface(v) => v.related_to(),
            Self::LandUse(v) => v.related_to(),
            Self::AuxiliaryTrafficArea(v) => v.related_to(),
            Self::HoleSurface(v) => v.related_to(),
            Self::Marking(v) => v.related_to(),
            Self::TrafficArea(v) => v.related_to(),
            Self::WaterGroundSurface(v) => v.related_to(),
            Self::WaterSurface(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::CeilingSurface(v) => v.dynamizer(),
            Self::DoorSurface(v) => v.dynamizer(),
            Self::FloorSurface(v) => v.dynamizer(),
            Self::GroundSurface(v) => v.dynamizer(),
            Self::InteriorWallSurface(v) => v.dynamizer(),
            Self::OuterCeilingSurface(v) => v.dynamizer(),
            Self::OuterFloorSurface(v) => v.dynamizer(),
            Self::RoofSurface(v) => v.dynamizer(),
            Self::WallSurface(v) => v.dynamizer(),
            Self::WindowSurface(v) => v.dynamizer(),
            Self::ClosureSurface(v) => v.dynamizer(),
            Self::GenericThematicSurface(v) => v.dynamizer(),
            Self::LandUse(v) => v.dynamizer(),
            Self::AuxiliaryTrafficArea(v) => v.dynamizer(),
            Self::HoleSurface(v) => v.dynamizer(),
            Self::Marking(v) => v.dynamizer(),
            Self::TrafficArea(v) => v.dynamizer(),
            Self::WaterGroundSurface(v) => v.dynamizer(),
            Self::WaterSurface(v) => v.dynamizer(),
        }
    }
}
impl AbstractSpaceBoundaryTrait for AbstractThematicSurface {}
impl AbstractThematicSurfaceTrait for AbstractThematicSurface {
    fn area(&self) -> &[QualifiedArea] {
        match self {
            Self::CeilingSurface(v) => v.area(),
            Self::DoorSurface(v) => v.area(),
            Self::FloorSurface(v) => v.area(),
            Self::GroundSurface(v) => v.area(),
            Self::InteriorWallSurface(v) => v.area(),
            Self::OuterCeilingSurface(v) => v.area(),
            Self::OuterFloorSurface(v) => v.area(),
            Self::RoofSurface(v) => v.area(),
            Self::WallSurface(v) => v.area(),
            Self::WindowSurface(v) => v.area(),
            Self::ClosureSurface(v) => v.area(),
            Self::GenericThematicSurface(v) => v.area(),
            Self::LandUse(v) => v.area(),
            Self::AuxiliaryTrafficArea(v) => v.area(),
            Self::HoleSurface(v) => v.area(),
            Self::Marking(v) => v.area(),
            Self::TrafficArea(v) => v.area(),
            Self::WaterGroundSurface(v) => v.area(),
            Self::WaterSurface(v) => v.area(),
        }
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::CeilingSurface(v) => v.lod3_multi_surface(),
            Self::DoorSurface(v) => v.lod3_multi_surface(),
            Self::FloorSurface(v) => v.lod3_multi_surface(),
            Self::GroundSurface(v) => v.lod3_multi_surface(),
            Self::InteriorWallSurface(v) => v.lod3_multi_surface(),
            Self::OuterCeilingSurface(v) => v.lod3_multi_surface(),
            Self::OuterFloorSurface(v) => v.lod3_multi_surface(),
            Self::RoofSurface(v) => v.lod3_multi_surface(),
            Self::WallSurface(v) => v.lod3_multi_surface(),
            Self::WindowSurface(v) => v.lod3_multi_surface(),
            Self::ClosureSurface(v) => v.lod3_multi_surface(),
            Self::GenericThematicSurface(v) => v.lod3_multi_surface(),
            Self::LandUse(v) => v.lod3_multi_surface(),
            Self::AuxiliaryTrafficArea(v) => v.lod3_multi_surface(),
            Self::HoleSurface(v) => v.lod3_multi_surface(),
            Self::Marking(v) => v.lod3_multi_surface(),
            Self::TrafficArea(v) => v.lod3_multi_surface(),
            Self::WaterGroundSurface(v) => v.lod3_multi_surface(),
            Self::WaterSurface(v) => v.lod3_multi_surface(),
        }
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::CeilingSurface(v) => v.lod2_multi_surface(),
            Self::DoorSurface(v) => v.lod2_multi_surface(),
            Self::FloorSurface(v) => v.lod2_multi_surface(),
            Self::GroundSurface(v) => v.lod2_multi_surface(),
            Self::InteriorWallSurface(v) => v.lod2_multi_surface(),
            Self::OuterCeilingSurface(v) => v.lod2_multi_surface(),
            Self::OuterFloorSurface(v) => v.lod2_multi_surface(),
            Self::RoofSurface(v) => v.lod2_multi_surface(),
            Self::WallSurface(v) => v.lod2_multi_surface(),
            Self::WindowSurface(v) => v.lod2_multi_surface(),
            Self::ClosureSurface(v) => v.lod2_multi_surface(),
            Self::GenericThematicSurface(v) => v.lod2_multi_surface(),
            Self::LandUse(v) => v.lod2_multi_surface(),
            Self::AuxiliaryTrafficArea(v) => v.lod2_multi_surface(),
            Self::HoleSurface(v) => v.lod2_multi_surface(),
            Self::Marking(v) => v.lod2_multi_surface(),
            Self::TrafficArea(v) => v.lod2_multi_surface(),
            Self::WaterGroundSurface(v) => v.lod2_multi_surface(),
            Self::WaterSurface(v) => v.lod2_multi_surface(),
        }
    }
    fn point_cloud(&self) -> Option<&AbstractPointCloud> {
        match self {
            Self::CeilingSurface(v) => v.point_cloud(),
            Self::DoorSurface(v) => v.point_cloud(),
            Self::FloorSurface(v) => v.point_cloud(),
            Self::GroundSurface(v) => v.point_cloud(),
            Self::InteriorWallSurface(v) => v.point_cloud(),
            Self::OuterCeilingSurface(v) => v.point_cloud(),
            Self::OuterFloorSurface(v) => v.point_cloud(),
            Self::RoofSurface(v) => v.point_cloud(),
            Self::WallSurface(v) => v.point_cloud(),
            Self::WindowSurface(v) => v.point_cloud(),
            Self::ClosureSurface(v) => v.point_cloud(),
            Self::GenericThematicSurface(v) => v.point_cloud(),
            Self::LandUse(v) => v.point_cloud(),
            Self::AuxiliaryTrafficArea(v) => v.point_cloud(),
            Self::HoleSurface(v) => v.point_cloud(),
            Self::Marking(v) => v.point_cloud(),
            Self::TrafficArea(v) => v.point_cloud(),
            Self::WaterGroundSurface(v) => v.point_cloud(),
            Self::WaterSurface(v) => v.point_cloud(),
        }
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::CeilingSurface(v) => v.lod0_multi_curve(),
            Self::DoorSurface(v) => v.lod0_multi_curve(),
            Self::FloorSurface(v) => v.lod0_multi_curve(),
            Self::GroundSurface(v) => v.lod0_multi_curve(),
            Self::InteriorWallSurface(v) => v.lod0_multi_curve(),
            Self::OuterCeilingSurface(v) => v.lod0_multi_curve(),
            Self::OuterFloorSurface(v) => v.lod0_multi_curve(),
            Self::RoofSurface(v) => v.lod0_multi_curve(),
            Self::WallSurface(v) => v.lod0_multi_curve(),
            Self::WindowSurface(v) => v.lod0_multi_curve(),
            Self::ClosureSurface(v) => v.lod0_multi_curve(),
            Self::GenericThematicSurface(v) => v.lod0_multi_curve(),
            Self::LandUse(v) => v.lod0_multi_curve(),
            Self::AuxiliaryTrafficArea(v) => v.lod0_multi_curve(),
            Self::HoleSurface(v) => v.lod0_multi_curve(),
            Self::Marking(v) => v.lod0_multi_curve(),
            Self::TrafficArea(v) => v.lod0_multi_curve(),
            Self::WaterGroundSurface(v) => v.lod0_multi_curve(),
            Self::WaterSurface(v) => v.lod0_multi_curve(),
        }
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::CeilingSurface(v) => v.lod0_multi_surface(),
            Self::DoorSurface(v) => v.lod0_multi_surface(),
            Self::FloorSurface(v) => v.lod0_multi_surface(),
            Self::GroundSurface(v) => v.lod0_multi_surface(),
            Self::InteriorWallSurface(v) => v.lod0_multi_surface(),
            Self::OuterCeilingSurface(v) => v.lod0_multi_surface(),
            Self::OuterFloorSurface(v) => v.lod0_multi_surface(),
            Self::RoofSurface(v) => v.lod0_multi_surface(),
            Self::WallSurface(v) => v.lod0_multi_surface(),
            Self::WindowSurface(v) => v.lod0_multi_surface(),
            Self::ClosureSurface(v) => v.lod0_multi_surface(),
            Self::GenericThematicSurface(v) => v.lod0_multi_surface(),
            Self::LandUse(v) => v.lod0_multi_surface(),
            Self::AuxiliaryTrafficArea(v) => v.lod0_multi_surface(),
            Self::HoleSurface(v) => v.lod0_multi_surface(),
            Self::Marking(v) => v.lod0_multi_surface(),
            Self::TrafficArea(v) => v.lod0_multi_surface(),
            Self::WaterGroundSurface(v) => v.lod0_multi_surface(),
            Self::WaterSurface(v) => v.lod0_multi_surface(),
        }
    }
    fn lod1_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::CeilingSurface(v) => v.lod1_multi_surface(),
            Self::DoorSurface(v) => v.lod1_multi_surface(),
            Self::FloorSurface(v) => v.lod1_multi_surface(),
            Self::GroundSurface(v) => v.lod1_multi_surface(),
            Self::InteriorWallSurface(v) => v.lod1_multi_surface(),
            Self::OuterCeilingSurface(v) => v.lod1_multi_surface(),
            Self::OuterFloorSurface(v) => v.lod1_multi_surface(),
            Self::RoofSurface(v) => v.lod1_multi_surface(),
            Self::WallSurface(v) => v.lod1_multi_surface(),
            Self::WindowSurface(v) => v.lod1_multi_surface(),
            Self::ClosureSurface(v) => v.lod1_multi_surface(),
            Self::GenericThematicSurface(v) => v.lod1_multi_surface(),
            Self::LandUse(v) => v.lod1_multi_surface(),
            Self::AuxiliaryTrafficArea(v) => v.lod1_multi_surface(),
            Self::HoleSurface(v) => v.lod1_multi_surface(),
            Self::Marking(v) => v.lod1_multi_surface(),
            Self::TrafficArea(v) => v.lod1_multi_surface(),
            Self::WaterGroundSurface(v) => v.lod1_multi_surface(),
            Self::WaterSurface(v) => v.lod1_multi_surface(),
        }
    }
}
impl From<CeilingSurface> for AbstractThematicSurface {
    fn from(v: CeilingSurface) -> Self {
        Self::CeilingSurface(Box::new(v))
    }
}
impl From<DoorSurface> for AbstractThematicSurface {
    fn from(v: DoorSurface) -> Self {
        Self::DoorSurface(Box::new(v))
    }
}
impl From<FloorSurface> for AbstractThematicSurface {
    fn from(v: FloorSurface) -> Self {
        Self::FloorSurface(Box::new(v))
    }
}
impl From<GroundSurface> for AbstractThematicSurface {
    fn from(v: GroundSurface) -> Self {
        Self::GroundSurface(Box::new(v))
    }
}
impl From<InteriorWallSurface> for AbstractThematicSurface {
    fn from(v: InteriorWallSurface) -> Self {
        Self::InteriorWallSurface(Box::new(v))
    }
}
impl From<OuterCeilingSurface> for AbstractThematicSurface {
    fn from(v: OuterCeilingSurface) -> Self {
        Self::OuterCeilingSurface(Box::new(v))
    }
}
impl From<OuterFloorSurface> for AbstractThematicSurface {
    fn from(v: OuterFloorSurface) -> Self {
        Self::OuterFloorSurface(Box::new(v))
    }
}
impl From<RoofSurface> for AbstractThematicSurface {
    fn from(v: RoofSurface) -> Self {
        Self::RoofSurface(Box::new(v))
    }
}
impl From<WallSurface> for AbstractThematicSurface {
    fn from(v: WallSurface) -> Self {
        Self::WallSurface(Box::new(v))
    }
}
impl From<WindowSurface> for AbstractThematicSurface {
    fn from(v: WindowSurface) -> Self {
        Self::WindowSurface(Box::new(v))
    }
}
impl From<ClosureSurface> for AbstractThematicSurface {
    fn from(v: ClosureSurface) -> Self {
        Self::ClosureSurface(Box::new(v))
    }
}
impl From<GenericThematicSurface> for AbstractThematicSurface {
    fn from(v: GenericThematicSurface) -> Self {
        Self::GenericThematicSurface(Box::new(v))
    }
}
impl From<LandUse> for AbstractThematicSurface {
    fn from(v: LandUse) -> Self {
        Self::LandUse(Box::new(v))
    }
}
impl From<AuxiliaryTrafficArea> for AbstractThematicSurface {
    fn from(v: AuxiliaryTrafficArea) -> Self {
        Self::AuxiliaryTrafficArea(Box::new(v))
    }
}
impl From<HoleSurface> for AbstractThematicSurface {
    fn from(v: HoleSurface) -> Self {
        Self::HoleSurface(Box::new(v))
    }
}
impl From<Marking> for AbstractThematicSurface {
    fn from(v: Marking) -> Self {
        Self::Marking(Box::new(v))
    }
}
impl From<TrafficArea> for AbstractThematicSurface {
    fn from(v: TrafficArea) -> Self {
        Self::TrafficArea(Box::new(v))
    }
}
impl From<WaterGroundSurface> for AbstractThematicSurface {
    fn from(v: WaterGroundSurface) -> Self {
        Self::WaterGroundSurface(Box::new(v))
    }
}
impl From<WaterSurface> for AbstractThematicSurface {
    fn from(v: WaterSurface) -> Self {
        Self::WaterSurface(Box::new(v))
    }
}
pub trait AbstractThematicSurfaceAccessors {
    fn ceiling_surfaces(&self) -> impl Iterator<Item = &CeilingSurface>;
    fn door_surfaces(&self) -> impl Iterator<Item = &DoorSurface>;
    fn floor_surfaces(&self) -> impl Iterator<Item = &FloorSurface>;
    fn ground_surfaces(&self) -> impl Iterator<Item = &GroundSurface>;
    fn interior_wall_surfaces(&self) -> impl Iterator<Item = &InteriorWallSurface>;
    fn outer_ceiling_surfaces(&self) -> impl Iterator<Item = &OuterCeilingSurface>;
    fn outer_floor_surfaces(&self) -> impl Iterator<Item = &OuterFloorSurface>;
    fn roof_surfaces(&self) -> impl Iterator<Item = &RoofSurface>;
    fn wall_surfaces(&self) -> impl Iterator<Item = &WallSurface>;
    fn window_surfaces(&self) -> impl Iterator<Item = &WindowSurface>;
    fn closure_surfaces(&self) -> impl Iterator<Item = &ClosureSurface>;
    fn generic_thematic_surfaces(&self) -> impl Iterator<Item = &GenericThematicSurface>;
    fn land_uses(&self) -> impl Iterator<Item = &LandUse>;
    fn auxiliary_traffic_areas(&self) -> impl Iterator<Item = &AuxiliaryTrafficArea>;
    fn hole_surfaces(&self) -> impl Iterator<Item = &HoleSurface>;
    fn markings(&self) -> impl Iterator<Item = &Marking>;
    fn traffic_areas(&self) -> impl Iterator<Item = &TrafficArea>;
    fn water_ground_surfaces(&self) -> impl Iterator<Item = &WaterGroundSurface>;
    fn water_surfaces(&self) -> impl Iterator<Item = &WaterSurface>;
}
impl AbstractThematicSurfaceAccessors for [AbstractThematicSurface] {
    fn ceiling_surfaces(&self) -> impl Iterator<Item = &CeilingSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::CeilingSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn door_surfaces(&self) -> impl Iterator<Item = &DoorSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::DoorSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn floor_surfaces(&self) -> impl Iterator<Item = &FloorSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::FloorSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn ground_surfaces(&self) -> impl Iterator<Item = &GroundSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::GroundSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn interior_wall_surfaces(&self) -> impl Iterator<Item = &InteriorWallSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::InteriorWallSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn outer_ceiling_surfaces(&self) -> impl Iterator<Item = &OuterCeilingSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::OuterCeilingSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn outer_floor_surfaces(&self) -> impl Iterator<Item = &OuterFloorSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::OuterFloorSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn roof_surfaces(&self) -> impl Iterator<Item = &RoofSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::RoofSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn wall_surfaces(&self) -> impl Iterator<Item = &WallSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::WallSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn window_surfaces(&self) -> impl Iterator<Item = &WindowSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::WindowSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn closure_surfaces(&self) -> impl Iterator<Item = &ClosureSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::ClosureSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_thematic_surfaces(
        &self,
    ) -> impl Iterator<Item = &GenericThematicSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::GenericThematicSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn land_uses(&self) -> impl Iterator<Item = &LandUse> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::LandUse(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn auxiliary_traffic_areas(&self) -> impl Iterator<Item = &AuxiliaryTrafficArea> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::AuxiliaryTrafficArea(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn hole_surfaces(&self) -> impl Iterator<Item = &HoleSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::HoleSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn markings(&self) -> impl Iterator<Item = &Marking> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::Marking(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn traffic_areas(&self) -> impl Iterator<Item = &TrafficArea> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::TrafficArea(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn water_ground_surfaces(&self) -> impl Iterator<Item = &WaterGroundSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::WaterGroundSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn water_surfaces(&self) -> impl Iterator<Item = &WaterSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractThematicSurface::WaterSurface(v) => Some(v.as_ref()),
                _ => None,
            })
    }
}
pub trait AbstractOccupiedSpaceTrait: AbstractPhysicalSpaceTrait {
    fn lod3_implicit_representation(&self) -> Option<&ImplicitGeometry>;
    fn lod2_implicit_representation(&self) -> Option<&ImplicitGeometry>;
    fn lod1_implicit_representation(&self) -> Option<&ImplicitGeometry>;
}
#[derive(Debug, Clone)]
pub enum AbstractOccupiedSpace {
    Door(Box<Door>),
    OtherConstruction(Box<OtherConstruction>),
    Window(Box<Window>),
    Bridge(Box<Bridge>),
    BridgeConstructiveElement(Box<BridgeConstructiveElement>),
    BridgeFurniture(Box<BridgeFurniture>),
    BridgeInstallation(Box<BridgeInstallation>),
    BridgePart(Box<BridgePart>),
    Building(Box<Building>),
    BuildingConstructiveElement(Box<BuildingConstructiveElement>),
    BuildingFurniture(Box<BuildingFurniture>),
    BuildingInstallation(Box<BuildingInstallation>),
    BuildingPart(Box<BuildingPart>),
    CityFurniture(Box<CityFurniture>),
    GenericOccupiedSpace(Box<GenericOccupiedSpace>),
    Tunnel(Box<Tunnel>),
    TunnelConstructiveElement(Box<TunnelConstructiveElement>),
    TunnelFurniture(Box<TunnelFurniture>),
    TunnelInstallation(Box<TunnelInstallation>),
    TunnelPart(Box<TunnelPart>),
    PlantCover(Box<PlantCover>),
    SolitaryVegetationObject(Box<SolitaryVegetationObject>),
    WaterBody(Box<WaterBody>),
}
impl Default for AbstractOccupiedSpace {
    fn default() -> Self {
        Self::Door(Box::default())
    }
}
impl AbstractFeatureTrait for AbstractOccupiedSpace {
    fn feature_id(&self) -> &ID {
        match self {
            Self::Door(v) => v.feature_id(),
            Self::OtherConstruction(v) => v.feature_id(),
            Self::Window(v) => v.feature_id(),
            Self::Bridge(v) => v.feature_id(),
            Self::BridgeConstructiveElement(v) => v.feature_id(),
            Self::BridgeFurniture(v) => v.feature_id(),
            Self::BridgeInstallation(v) => v.feature_id(),
            Self::BridgePart(v) => v.feature_id(),
            Self::Building(v) => v.feature_id(),
            Self::BuildingConstructiveElement(v) => v.feature_id(),
            Self::BuildingFurniture(v) => v.feature_id(),
            Self::BuildingInstallation(v) => v.feature_id(),
            Self::BuildingPart(v) => v.feature_id(),
            Self::CityFurniture(v) => v.feature_id(),
            Self::GenericOccupiedSpace(v) => v.feature_id(),
            Self::Tunnel(v) => v.feature_id(),
            Self::TunnelConstructiveElement(v) => v.feature_id(),
            Self::TunnelFurniture(v) => v.feature_id(),
            Self::TunnelInstallation(v) => v.feature_id(),
            Self::TunnelPart(v) => v.feature_id(),
            Self::PlantCover(v) => v.feature_id(),
            Self::SolitaryVegetationObject(v) => v.feature_id(),
            Self::WaterBody(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.identifier(),
            Self::OtherConstruction(v) => v.identifier(),
            Self::Window(v) => v.identifier(),
            Self::Bridge(v) => v.identifier(),
            Self::BridgeConstructiveElement(v) => v.identifier(),
            Self::BridgeFurniture(v) => v.identifier(),
            Self::BridgeInstallation(v) => v.identifier(),
            Self::BridgePart(v) => v.identifier(),
            Self::Building(v) => v.identifier(),
            Self::BuildingConstructiveElement(v) => v.identifier(),
            Self::BuildingFurniture(v) => v.identifier(),
            Self::BuildingInstallation(v) => v.identifier(),
            Self::BuildingPart(v) => v.identifier(),
            Self::CityFurniture(v) => v.identifier(),
            Self::GenericOccupiedSpace(v) => v.identifier(),
            Self::Tunnel(v) => v.identifier(),
            Self::TunnelConstructiveElement(v) => v.identifier(),
            Self::TunnelFurniture(v) => v.identifier(),
            Self::TunnelInstallation(v) => v.identifier(),
            Self::TunnelPart(v) => v.identifier(),
            Self::PlantCover(v) => v.identifier(),
            Self::SolitaryVegetationObject(v) => v.identifier(),
            Self::WaterBody(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::Door(v) => v.name(),
            Self::OtherConstruction(v) => v.name(),
            Self::Window(v) => v.name(),
            Self::Bridge(v) => v.name(),
            Self::BridgeConstructiveElement(v) => v.name(),
            Self::BridgeFurniture(v) => v.name(),
            Self::BridgeInstallation(v) => v.name(),
            Self::BridgePart(v) => v.name(),
            Self::Building(v) => v.name(),
            Self::BuildingConstructiveElement(v) => v.name(),
            Self::BuildingFurniture(v) => v.name(),
            Self::BuildingInstallation(v) => v.name(),
            Self::BuildingPart(v) => v.name(),
            Self::CityFurniture(v) => v.name(),
            Self::GenericOccupiedSpace(v) => v.name(),
            Self::Tunnel(v) => v.name(),
            Self::TunnelConstructiveElement(v) => v.name(),
            Self::TunnelFurniture(v) => v.name(),
            Self::TunnelInstallation(v) => v.name(),
            Self::TunnelPart(v) => v.name(),
            Self::PlantCover(v) => v.name(),
            Self::SolitaryVegetationObject(v) => v.name(),
            Self::WaterBody(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.description(),
            Self::OtherConstruction(v) => v.description(),
            Self::Window(v) => v.description(),
            Self::Bridge(v) => v.description(),
            Self::BridgeConstructiveElement(v) => v.description(),
            Self::BridgeFurniture(v) => v.description(),
            Self::BridgeInstallation(v) => v.description(),
            Self::BridgePart(v) => v.description(),
            Self::Building(v) => v.description(),
            Self::BuildingConstructiveElement(v) => v.description(),
            Self::BuildingFurniture(v) => v.description(),
            Self::BuildingInstallation(v) => v.description(),
            Self::BuildingPart(v) => v.description(),
            Self::CityFurniture(v) => v.description(),
            Self::GenericOccupiedSpace(v) => v.description(),
            Self::Tunnel(v) => v.description(),
            Self::TunnelConstructiveElement(v) => v.description(),
            Self::TunnelFurniture(v) => v.description(),
            Self::TunnelInstallation(v) => v.description(),
            Self::TunnelPart(v) => v.description(),
            Self::PlantCover(v) => v.description(),
            Self::SolitaryVegetationObject(v) => v.description(),
            Self::WaterBody(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractOccupiedSpace {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.creation_date(),
            Self::OtherConstruction(v) => v.creation_date(),
            Self::Window(v) => v.creation_date(),
            Self::Bridge(v) => v.creation_date(),
            Self::BridgeConstructiveElement(v) => v.creation_date(),
            Self::BridgeFurniture(v) => v.creation_date(),
            Self::BridgeInstallation(v) => v.creation_date(),
            Self::BridgePart(v) => v.creation_date(),
            Self::Building(v) => v.creation_date(),
            Self::BuildingConstructiveElement(v) => v.creation_date(),
            Self::BuildingFurniture(v) => v.creation_date(),
            Self::BuildingInstallation(v) => v.creation_date(),
            Self::BuildingPart(v) => v.creation_date(),
            Self::CityFurniture(v) => v.creation_date(),
            Self::GenericOccupiedSpace(v) => v.creation_date(),
            Self::Tunnel(v) => v.creation_date(),
            Self::TunnelConstructiveElement(v) => v.creation_date(),
            Self::TunnelFurniture(v) => v.creation_date(),
            Self::TunnelInstallation(v) => v.creation_date(),
            Self::TunnelPart(v) => v.creation_date(),
            Self::PlantCover(v) => v.creation_date(),
            Self::SolitaryVegetationObject(v) => v.creation_date(),
            Self::WaterBody(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.termination_date(),
            Self::OtherConstruction(v) => v.termination_date(),
            Self::Window(v) => v.termination_date(),
            Self::Bridge(v) => v.termination_date(),
            Self::BridgeConstructiveElement(v) => v.termination_date(),
            Self::BridgeFurniture(v) => v.termination_date(),
            Self::BridgeInstallation(v) => v.termination_date(),
            Self::BridgePart(v) => v.termination_date(),
            Self::Building(v) => v.termination_date(),
            Self::BuildingConstructiveElement(v) => v.termination_date(),
            Self::BuildingFurniture(v) => v.termination_date(),
            Self::BuildingInstallation(v) => v.termination_date(),
            Self::BuildingPart(v) => v.termination_date(),
            Self::CityFurniture(v) => v.termination_date(),
            Self::GenericOccupiedSpace(v) => v.termination_date(),
            Self::Tunnel(v) => v.termination_date(),
            Self::TunnelConstructiveElement(v) => v.termination_date(),
            Self::TunnelFurniture(v) => v.termination_date(),
            Self::TunnelInstallation(v) => v.termination_date(),
            Self::TunnelPart(v) => v.termination_date(),
            Self::PlantCover(v) => v.termination_date(),
            Self::SolitaryVegetationObject(v) => v.termination_date(),
            Self::WaterBody(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.valid_from(),
            Self::OtherConstruction(v) => v.valid_from(),
            Self::Window(v) => v.valid_from(),
            Self::Bridge(v) => v.valid_from(),
            Self::BridgeConstructiveElement(v) => v.valid_from(),
            Self::BridgeFurniture(v) => v.valid_from(),
            Self::BridgeInstallation(v) => v.valid_from(),
            Self::BridgePart(v) => v.valid_from(),
            Self::Building(v) => v.valid_from(),
            Self::BuildingConstructiveElement(v) => v.valid_from(),
            Self::BuildingFurniture(v) => v.valid_from(),
            Self::BuildingInstallation(v) => v.valid_from(),
            Self::BuildingPart(v) => v.valid_from(),
            Self::CityFurniture(v) => v.valid_from(),
            Self::GenericOccupiedSpace(v) => v.valid_from(),
            Self::Tunnel(v) => v.valid_from(),
            Self::TunnelConstructiveElement(v) => v.valid_from(),
            Self::TunnelFurniture(v) => v.valid_from(),
            Self::TunnelInstallation(v) => v.valid_from(),
            Self::TunnelPart(v) => v.valid_from(),
            Self::PlantCover(v) => v.valid_from(),
            Self::SolitaryVegetationObject(v) => v.valid_from(),
            Self::WaterBody(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.valid_to(),
            Self::OtherConstruction(v) => v.valid_to(),
            Self::Window(v) => v.valid_to(),
            Self::Bridge(v) => v.valid_to(),
            Self::BridgeConstructiveElement(v) => v.valid_to(),
            Self::BridgeFurniture(v) => v.valid_to(),
            Self::BridgeInstallation(v) => v.valid_to(),
            Self::BridgePart(v) => v.valid_to(),
            Self::Building(v) => v.valid_to(),
            Self::BuildingConstructiveElement(v) => v.valid_to(),
            Self::BuildingFurniture(v) => v.valid_to(),
            Self::BuildingInstallation(v) => v.valid_to(),
            Self::BuildingPart(v) => v.valid_to(),
            Self::CityFurniture(v) => v.valid_to(),
            Self::GenericOccupiedSpace(v) => v.valid_to(),
            Self::Tunnel(v) => v.valid_to(),
            Self::TunnelConstructiveElement(v) => v.valid_to(),
            Self::TunnelFurniture(v) => v.valid_to(),
            Self::TunnelInstallation(v) => v.valid_to(),
            Self::TunnelPart(v) => v.valid_to(),
            Self::PlantCover(v) => v.valid_to(),
            Self::SolitaryVegetationObject(v) => v.valid_to(),
            Self::WaterBody(v) => v.valid_to(),
        }
    }
}
impl AbstractCityObjectTrait for AbstractOccupiedSpace {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::Door(v) => v.relative_to_terrain(),
            Self::OtherConstruction(v) => v.relative_to_terrain(),
            Self::Window(v) => v.relative_to_terrain(),
            Self::Bridge(v) => v.relative_to_terrain(),
            Self::BridgeConstructiveElement(v) => v.relative_to_terrain(),
            Self::BridgeFurniture(v) => v.relative_to_terrain(),
            Self::BridgeInstallation(v) => v.relative_to_terrain(),
            Self::BridgePart(v) => v.relative_to_terrain(),
            Self::Building(v) => v.relative_to_terrain(),
            Self::BuildingConstructiveElement(v) => v.relative_to_terrain(),
            Self::BuildingFurniture(v) => v.relative_to_terrain(),
            Self::BuildingInstallation(v) => v.relative_to_terrain(),
            Self::BuildingPart(v) => v.relative_to_terrain(),
            Self::CityFurniture(v) => v.relative_to_terrain(),
            Self::GenericOccupiedSpace(v) => v.relative_to_terrain(),
            Self::Tunnel(v) => v.relative_to_terrain(),
            Self::TunnelConstructiveElement(v) => v.relative_to_terrain(),
            Self::TunnelFurniture(v) => v.relative_to_terrain(),
            Self::TunnelInstallation(v) => v.relative_to_terrain(),
            Self::TunnelPart(v) => v.relative_to_terrain(),
            Self::PlantCover(v) => v.relative_to_terrain(),
            Self::SolitaryVegetationObject(v) => v.relative_to_terrain(),
            Self::WaterBody(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::Door(v) => v.relative_to_water(),
            Self::OtherConstruction(v) => v.relative_to_water(),
            Self::Window(v) => v.relative_to_water(),
            Self::Bridge(v) => v.relative_to_water(),
            Self::BridgeConstructiveElement(v) => v.relative_to_water(),
            Self::BridgeFurniture(v) => v.relative_to_water(),
            Self::BridgeInstallation(v) => v.relative_to_water(),
            Self::BridgePart(v) => v.relative_to_water(),
            Self::Building(v) => v.relative_to_water(),
            Self::BuildingConstructiveElement(v) => v.relative_to_water(),
            Self::BuildingFurniture(v) => v.relative_to_water(),
            Self::BuildingInstallation(v) => v.relative_to_water(),
            Self::BuildingPart(v) => v.relative_to_water(),
            Self::CityFurniture(v) => v.relative_to_water(),
            Self::GenericOccupiedSpace(v) => v.relative_to_water(),
            Self::Tunnel(v) => v.relative_to_water(),
            Self::TunnelConstructiveElement(v) => v.relative_to_water(),
            Self::TunnelFurniture(v) => v.relative_to_water(),
            Self::TunnelInstallation(v) => v.relative_to_water(),
            Self::TunnelPart(v) => v.relative_to_water(),
            Self::PlantCover(v) => v.relative_to_water(),
            Self::SolitaryVegetationObject(v) => v.relative_to_water(),
            Self::WaterBody(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::Door(v) => v.appearance(),
            Self::OtherConstruction(v) => v.appearance(),
            Self::Window(v) => v.appearance(),
            Self::Bridge(v) => v.appearance(),
            Self::BridgeConstructiveElement(v) => v.appearance(),
            Self::BridgeFurniture(v) => v.appearance(),
            Self::BridgeInstallation(v) => v.appearance(),
            Self::BridgePart(v) => v.appearance(),
            Self::Building(v) => v.appearance(),
            Self::BuildingConstructiveElement(v) => v.appearance(),
            Self::BuildingFurniture(v) => v.appearance(),
            Self::BuildingInstallation(v) => v.appearance(),
            Self::BuildingPart(v) => v.appearance(),
            Self::CityFurniture(v) => v.appearance(),
            Self::GenericOccupiedSpace(v) => v.appearance(),
            Self::Tunnel(v) => v.appearance(),
            Self::TunnelConstructiveElement(v) => v.appearance(),
            Self::TunnelFurniture(v) => v.appearance(),
            Self::TunnelInstallation(v) => v.appearance(),
            Self::TunnelPart(v) => v.appearance(),
            Self::PlantCover(v) => v.appearance(),
            Self::SolitaryVegetationObject(v) => v.appearance(),
            Self::WaterBody(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::Door(v) => v.generalizes_to(),
            Self::OtherConstruction(v) => v.generalizes_to(),
            Self::Window(v) => v.generalizes_to(),
            Self::Bridge(v) => v.generalizes_to(),
            Self::BridgeConstructiveElement(v) => v.generalizes_to(),
            Self::BridgeFurniture(v) => v.generalizes_to(),
            Self::BridgeInstallation(v) => v.generalizes_to(),
            Self::BridgePart(v) => v.generalizes_to(),
            Self::Building(v) => v.generalizes_to(),
            Self::BuildingConstructiveElement(v) => v.generalizes_to(),
            Self::BuildingFurniture(v) => v.generalizes_to(),
            Self::BuildingInstallation(v) => v.generalizes_to(),
            Self::BuildingPart(v) => v.generalizes_to(),
            Self::CityFurniture(v) => v.generalizes_to(),
            Self::GenericOccupiedSpace(v) => v.generalizes_to(),
            Self::Tunnel(v) => v.generalizes_to(),
            Self::TunnelConstructiveElement(v) => v.generalizes_to(),
            Self::TunnelFurniture(v) => v.generalizes_to(),
            Self::TunnelInstallation(v) => v.generalizes_to(),
            Self::TunnelPart(v) => v.generalizes_to(),
            Self::PlantCover(v) => v.generalizes_to(),
            Self::SolitaryVegetationObject(v) => v.generalizes_to(),
            Self::WaterBody(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::Door(v) => v.external_reference(),
            Self::OtherConstruction(v) => v.external_reference(),
            Self::Window(v) => v.external_reference(),
            Self::Bridge(v) => v.external_reference(),
            Self::BridgeConstructiveElement(v) => v.external_reference(),
            Self::BridgeFurniture(v) => v.external_reference(),
            Self::BridgeInstallation(v) => v.external_reference(),
            Self::BridgePart(v) => v.external_reference(),
            Self::Building(v) => v.external_reference(),
            Self::BuildingConstructiveElement(v) => v.external_reference(),
            Self::BuildingFurniture(v) => v.external_reference(),
            Self::BuildingInstallation(v) => v.external_reference(),
            Self::BuildingPart(v) => v.external_reference(),
            Self::CityFurniture(v) => v.external_reference(),
            Self::GenericOccupiedSpace(v) => v.external_reference(),
            Self::Tunnel(v) => v.external_reference(),
            Self::TunnelConstructiveElement(v) => v.external_reference(),
            Self::TunnelFurniture(v) => v.external_reference(),
            Self::TunnelInstallation(v) => v.external_reference(),
            Self::TunnelPart(v) => v.external_reference(),
            Self::PlantCover(v) => v.external_reference(),
            Self::SolitaryVegetationObject(v) => v.external_reference(),
            Self::WaterBody(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::Door(v) => v.related_to(),
            Self::OtherConstruction(v) => v.related_to(),
            Self::Window(v) => v.related_to(),
            Self::Bridge(v) => v.related_to(),
            Self::BridgeConstructiveElement(v) => v.related_to(),
            Self::BridgeFurniture(v) => v.related_to(),
            Self::BridgeInstallation(v) => v.related_to(),
            Self::BridgePart(v) => v.related_to(),
            Self::Building(v) => v.related_to(),
            Self::BuildingConstructiveElement(v) => v.related_to(),
            Self::BuildingFurniture(v) => v.related_to(),
            Self::BuildingInstallation(v) => v.related_to(),
            Self::BuildingPart(v) => v.related_to(),
            Self::CityFurniture(v) => v.related_to(),
            Self::GenericOccupiedSpace(v) => v.related_to(),
            Self::Tunnel(v) => v.related_to(),
            Self::TunnelConstructiveElement(v) => v.related_to(),
            Self::TunnelFurniture(v) => v.related_to(),
            Self::TunnelInstallation(v) => v.related_to(),
            Self::TunnelPart(v) => v.related_to(),
            Self::PlantCover(v) => v.related_to(),
            Self::SolitaryVegetationObject(v) => v.related_to(),
            Self::WaterBody(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::Door(v) => v.dynamizer(),
            Self::OtherConstruction(v) => v.dynamizer(),
            Self::Window(v) => v.dynamizer(),
            Self::Bridge(v) => v.dynamizer(),
            Self::BridgeConstructiveElement(v) => v.dynamizer(),
            Self::BridgeFurniture(v) => v.dynamizer(),
            Self::BridgeInstallation(v) => v.dynamizer(),
            Self::BridgePart(v) => v.dynamizer(),
            Self::Building(v) => v.dynamizer(),
            Self::BuildingConstructiveElement(v) => v.dynamizer(),
            Self::BuildingFurniture(v) => v.dynamizer(),
            Self::BuildingInstallation(v) => v.dynamizer(),
            Self::BuildingPart(v) => v.dynamizer(),
            Self::CityFurniture(v) => v.dynamizer(),
            Self::GenericOccupiedSpace(v) => v.dynamizer(),
            Self::Tunnel(v) => v.dynamizer(),
            Self::TunnelConstructiveElement(v) => v.dynamizer(),
            Self::TunnelFurniture(v) => v.dynamizer(),
            Self::TunnelInstallation(v) => v.dynamizer(),
            Self::TunnelPart(v) => v.dynamizer(),
            Self::PlantCover(v) => v.dynamizer(),
            Self::SolitaryVegetationObject(v) => v.dynamizer(),
            Self::WaterBody(v) => v.dynamizer(),
        }
    }
}
impl AbstractSpaceTrait for AbstractOccupiedSpace {
    fn space_type(&self) -> Option<SpaceType> {
        match self {
            Self::Door(v) => v.space_type(),
            Self::OtherConstruction(v) => v.space_type(),
            Self::Window(v) => v.space_type(),
            Self::Bridge(v) => v.space_type(),
            Self::BridgeConstructiveElement(v) => v.space_type(),
            Self::BridgeFurniture(v) => v.space_type(),
            Self::BridgeInstallation(v) => v.space_type(),
            Self::BridgePart(v) => v.space_type(),
            Self::Building(v) => v.space_type(),
            Self::BuildingConstructiveElement(v) => v.space_type(),
            Self::BuildingFurniture(v) => v.space_type(),
            Self::BuildingInstallation(v) => v.space_type(),
            Self::BuildingPart(v) => v.space_type(),
            Self::CityFurniture(v) => v.space_type(),
            Self::GenericOccupiedSpace(v) => v.space_type(),
            Self::Tunnel(v) => v.space_type(),
            Self::TunnelConstructiveElement(v) => v.space_type(),
            Self::TunnelFurniture(v) => v.space_type(),
            Self::TunnelInstallation(v) => v.space_type(),
            Self::TunnelPart(v) => v.space_type(),
            Self::PlantCover(v) => v.space_type(),
            Self::SolitaryVegetationObject(v) => v.space_type(),
            Self::WaterBody(v) => v.space_type(),
        }
    }
    fn volume(&self) -> &[QualifiedVolume] {
        match self {
            Self::Door(v) => v.volume(),
            Self::OtherConstruction(v) => v.volume(),
            Self::Window(v) => v.volume(),
            Self::Bridge(v) => v.volume(),
            Self::BridgeConstructiveElement(v) => v.volume(),
            Self::BridgeFurniture(v) => v.volume(),
            Self::BridgeInstallation(v) => v.volume(),
            Self::BridgePart(v) => v.volume(),
            Self::Building(v) => v.volume(),
            Self::BuildingConstructiveElement(v) => v.volume(),
            Self::BuildingFurniture(v) => v.volume(),
            Self::BuildingInstallation(v) => v.volume(),
            Self::BuildingPart(v) => v.volume(),
            Self::CityFurniture(v) => v.volume(),
            Self::GenericOccupiedSpace(v) => v.volume(),
            Self::Tunnel(v) => v.volume(),
            Self::TunnelConstructiveElement(v) => v.volume(),
            Self::TunnelFurniture(v) => v.volume(),
            Self::TunnelInstallation(v) => v.volume(),
            Self::TunnelPart(v) => v.volume(),
            Self::PlantCover(v) => v.volume(),
            Self::SolitaryVegetationObject(v) => v.volume(),
            Self::WaterBody(v) => v.volume(),
        }
    }
    fn area(&self) -> &[QualifiedArea] {
        match self {
            Self::Door(v) => v.area(),
            Self::OtherConstruction(v) => v.area(),
            Self::Window(v) => v.area(),
            Self::Bridge(v) => v.area(),
            Self::BridgeConstructiveElement(v) => v.area(),
            Self::BridgeFurniture(v) => v.area(),
            Self::BridgeInstallation(v) => v.area(),
            Self::BridgePart(v) => v.area(),
            Self::Building(v) => v.area(),
            Self::BuildingConstructiveElement(v) => v.area(),
            Self::BuildingFurniture(v) => v.area(),
            Self::BuildingInstallation(v) => v.area(),
            Self::BuildingPart(v) => v.area(),
            Self::CityFurniture(v) => v.area(),
            Self::GenericOccupiedSpace(v) => v.area(),
            Self::Tunnel(v) => v.area(),
            Self::TunnelConstructiveElement(v) => v.area(),
            Self::TunnelFurniture(v) => v.area(),
            Self::TunnelInstallation(v) => v.area(),
            Self::TunnelPart(v) => v.area(),
            Self::PlantCover(v) => v.area(),
            Self::SolitaryVegetationObject(v) => v.area(),
            Self::WaterBody(v) => v.area(),
        }
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod2_multi_curve(),
            Self::OtherConstruction(v) => v.lod2_multi_curve(),
            Self::Window(v) => v.lod2_multi_curve(),
            Self::Bridge(v) => v.lod2_multi_curve(),
            Self::BridgeConstructiveElement(v) => v.lod2_multi_curve(),
            Self::BridgeFurniture(v) => v.lod2_multi_curve(),
            Self::BridgeInstallation(v) => v.lod2_multi_curve(),
            Self::BridgePart(v) => v.lod2_multi_curve(),
            Self::Building(v) => v.lod2_multi_curve(),
            Self::BuildingConstructiveElement(v) => v.lod2_multi_curve(),
            Self::BuildingFurniture(v) => v.lod2_multi_curve(),
            Self::BuildingInstallation(v) => v.lod2_multi_curve(),
            Self::BuildingPart(v) => v.lod2_multi_curve(),
            Self::CityFurniture(v) => v.lod2_multi_curve(),
            Self::GenericOccupiedSpace(v) => v.lod2_multi_curve(),
            Self::Tunnel(v) => v.lod2_multi_curve(),
            Self::TunnelConstructiveElement(v) => v.lod2_multi_curve(),
            Self::TunnelFurniture(v) => v.lod2_multi_curve(),
            Self::TunnelInstallation(v) => v.lod2_multi_curve(),
            Self::TunnelPart(v) => v.lod2_multi_curve(),
            Self::PlantCover(v) => v.lod2_multi_curve(),
            Self::SolitaryVegetationObject(v) => v.lod2_multi_curve(),
            Self::WaterBody(v) => v.lod2_multi_curve(),
        }
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::Door(v) => v.lod3_multi_surface(),
            Self::OtherConstruction(v) => v.lod3_multi_surface(),
            Self::Window(v) => v.lod3_multi_surface(),
            Self::Bridge(v) => v.lod3_multi_surface(),
            Self::BridgeConstructiveElement(v) => v.lod3_multi_surface(),
            Self::BridgeFurniture(v) => v.lod3_multi_surface(),
            Self::BridgeInstallation(v) => v.lod3_multi_surface(),
            Self::BridgePart(v) => v.lod3_multi_surface(),
            Self::Building(v) => v.lod3_multi_surface(),
            Self::BuildingConstructiveElement(v) => v.lod3_multi_surface(),
            Self::BuildingFurniture(v) => v.lod3_multi_surface(),
            Self::BuildingInstallation(v) => v.lod3_multi_surface(),
            Self::BuildingPart(v) => v.lod3_multi_surface(),
            Self::CityFurniture(v) => v.lod3_multi_surface(),
            Self::GenericOccupiedSpace(v) => v.lod3_multi_surface(),
            Self::Tunnel(v) => v.lod3_multi_surface(),
            Self::TunnelConstructiveElement(v) => v.lod3_multi_surface(),
            Self::TunnelFurniture(v) => v.lod3_multi_surface(),
            Self::TunnelInstallation(v) => v.lod3_multi_surface(),
            Self::TunnelPart(v) => v.lod3_multi_surface(),
            Self::PlantCover(v) => v.lod3_multi_surface(),
            Self::SolitaryVegetationObject(v) => v.lod3_multi_surface(),
            Self::WaterBody(v) => v.lod3_multi_surface(),
        }
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::Door(v) => v.lod0_multi_surface(),
            Self::OtherConstruction(v) => v.lod0_multi_surface(),
            Self::Window(v) => v.lod0_multi_surface(),
            Self::Bridge(v) => v.lod0_multi_surface(),
            Self::BridgeConstructiveElement(v) => v.lod0_multi_surface(),
            Self::BridgeFurniture(v) => v.lod0_multi_surface(),
            Self::BridgeInstallation(v) => v.lod0_multi_surface(),
            Self::BridgePart(v) => v.lod0_multi_surface(),
            Self::Building(v) => v.lod0_multi_surface(),
            Self::BuildingConstructiveElement(v) => v.lod0_multi_surface(),
            Self::BuildingFurniture(v) => v.lod0_multi_surface(),
            Self::BuildingInstallation(v) => v.lod0_multi_surface(),
            Self::BuildingPart(v) => v.lod0_multi_surface(),
            Self::CityFurniture(v) => v.lod0_multi_surface(),
            Self::GenericOccupiedSpace(v) => v.lod0_multi_surface(),
            Self::Tunnel(v) => v.lod0_multi_surface(),
            Self::TunnelConstructiveElement(v) => v.lod0_multi_surface(),
            Self::TunnelFurniture(v) => v.lod0_multi_surface(),
            Self::TunnelInstallation(v) => v.lod0_multi_surface(),
            Self::TunnelPart(v) => v.lod0_multi_surface(),
            Self::PlantCover(v) => v.lod0_multi_surface(),
            Self::SolitaryVegetationObject(v) => v.lod0_multi_surface(),
            Self::WaterBody(v) => v.lod0_multi_surface(),
        }
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::Door(v) => v.lod1_solid(),
            Self::OtherConstruction(v) => v.lod1_solid(),
            Self::Window(v) => v.lod1_solid(),
            Self::Bridge(v) => v.lod1_solid(),
            Self::BridgeConstructiveElement(v) => v.lod1_solid(),
            Self::BridgeFurniture(v) => v.lod1_solid(),
            Self::BridgeInstallation(v) => v.lod1_solid(),
            Self::BridgePart(v) => v.lod1_solid(),
            Self::Building(v) => v.lod1_solid(),
            Self::BuildingConstructiveElement(v) => v.lod1_solid(),
            Self::BuildingFurniture(v) => v.lod1_solid(),
            Self::BuildingInstallation(v) => v.lod1_solid(),
            Self::BuildingPart(v) => v.lod1_solid(),
            Self::CityFurniture(v) => v.lod1_solid(),
            Self::GenericOccupiedSpace(v) => v.lod1_solid(),
            Self::Tunnel(v) => v.lod1_solid(),
            Self::TunnelConstructiveElement(v) => v.lod1_solid(),
            Self::TunnelFurniture(v) => v.lod1_solid(),
            Self::TunnelInstallation(v) => v.lod1_solid(),
            Self::TunnelPart(v) => v.lod1_solid(),
            Self::PlantCover(v) => v.lod1_solid(),
            Self::SolitaryVegetationObject(v) => v.lod1_solid(),
            Self::WaterBody(v) => v.lod1_solid(),
        }
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::Door(v) => v.lod3_solid(),
            Self::OtherConstruction(v) => v.lod3_solid(),
            Self::Window(v) => v.lod3_solid(),
            Self::Bridge(v) => v.lod3_solid(),
            Self::BridgeConstructiveElement(v) => v.lod3_solid(),
            Self::BridgeFurniture(v) => v.lod3_solid(),
            Self::BridgeInstallation(v) => v.lod3_solid(),
            Self::BridgePart(v) => v.lod3_solid(),
            Self::Building(v) => v.lod3_solid(),
            Self::BuildingConstructiveElement(v) => v.lod3_solid(),
            Self::BuildingFurniture(v) => v.lod3_solid(),
            Self::BuildingInstallation(v) => v.lod3_solid(),
            Self::BuildingPart(v) => v.lod3_solid(),
            Self::CityFurniture(v) => v.lod3_solid(),
            Self::GenericOccupiedSpace(v) => v.lod3_solid(),
            Self::Tunnel(v) => v.lod3_solid(),
            Self::TunnelConstructiveElement(v) => v.lod3_solid(),
            Self::TunnelFurniture(v) => v.lod3_solid(),
            Self::TunnelInstallation(v) => v.lod3_solid(),
            Self::TunnelPart(v) => v.lod3_solid(),
            Self::PlantCover(v) => v.lod3_solid(),
            Self::SolitaryVegetationObject(v) => v.lod3_solid(),
            Self::WaterBody(v) => v.lod3_solid(),
        }
    }
    fn boundary(&self) -> &[AbstractSpaceBoundary] {
        match self {
            Self::Door(v) => v.boundary(),
            Self::OtherConstruction(v) => v.boundary(),
            Self::Window(v) => v.boundary(),
            Self::Bridge(v) => v.boundary(),
            Self::BridgeConstructiveElement(v) => v.boundary(),
            Self::BridgeFurniture(v) => v.boundary(),
            Self::BridgeInstallation(v) => v.boundary(),
            Self::BridgePart(v) => v.boundary(),
            Self::Building(v) => v.boundary(),
            Self::BuildingConstructiveElement(v) => v.boundary(),
            Self::BuildingFurniture(v) => v.boundary(),
            Self::BuildingInstallation(v) => v.boundary(),
            Self::BuildingPart(v) => v.boundary(),
            Self::CityFurniture(v) => v.boundary(),
            Self::GenericOccupiedSpace(v) => v.boundary(),
            Self::Tunnel(v) => v.boundary(),
            Self::TunnelConstructiveElement(v) => v.boundary(),
            Self::TunnelFurniture(v) => v.boundary(),
            Self::TunnelInstallation(v) => v.boundary(),
            Self::TunnelPart(v) => v.boundary(),
            Self::PlantCover(v) => v.boundary(),
            Self::SolitaryVegetationObject(v) => v.boundary(),
            Self::WaterBody(v) => v.boundary(),
        }
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod0_multi_curve(),
            Self::OtherConstruction(v) => v.lod0_multi_curve(),
            Self::Window(v) => v.lod0_multi_curve(),
            Self::Bridge(v) => v.lod0_multi_curve(),
            Self::BridgeConstructiveElement(v) => v.lod0_multi_curve(),
            Self::BridgeFurniture(v) => v.lod0_multi_curve(),
            Self::BridgeInstallation(v) => v.lod0_multi_curve(),
            Self::BridgePart(v) => v.lod0_multi_curve(),
            Self::Building(v) => v.lod0_multi_curve(),
            Self::BuildingConstructiveElement(v) => v.lod0_multi_curve(),
            Self::BuildingFurniture(v) => v.lod0_multi_curve(),
            Self::BuildingInstallation(v) => v.lod0_multi_curve(),
            Self::BuildingPart(v) => v.lod0_multi_curve(),
            Self::CityFurniture(v) => v.lod0_multi_curve(),
            Self::GenericOccupiedSpace(v) => v.lod0_multi_curve(),
            Self::Tunnel(v) => v.lod0_multi_curve(),
            Self::TunnelConstructiveElement(v) => v.lod0_multi_curve(),
            Self::TunnelFurniture(v) => v.lod0_multi_curve(),
            Self::TunnelInstallation(v) => v.lod0_multi_curve(),
            Self::TunnelPart(v) => v.lod0_multi_curve(),
            Self::PlantCover(v) => v.lod0_multi_curve(),
            Self::SolitaryVegetationObject(v) => v.lod0_multi_curve(),
            Self::WaterBody(v) => v.lod0_multi_curve(),
        }
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::Door(v) => v.lod2_solid(),
            Self::OtherConstruction(v) => v.lod2_solid(),
            Self::Window(v) => v.lod2_solid(),
            Self::Bridge(v) => v.lod2_solid(),
            Self::BridgeConstructiveElement(v) => v.lod2_solid(),
            Self::BridgeFurniture(v) => v.lod2_solid(),
            Self::BridgeInstallation(v) => v.lod2_solid(),
            Self::BridgePart(v) => v.lod2_solid(),
            Self::Building(v) => v.lod2_solid(),
            Self::BuildingConstructiveElement(v) => v.lod2_solid(),
            Self::BuildingFurniture(v) => v.lod2_solid(),
            Self::BuildingInstallation(v) => v.lod2_solid(),
            Self::BuildingPart(v) => v.lod2_solid(),
            Self::CityFurniture(v) => v.lod2_solid(),
            Self::GenericOccupiedSpace(v) => v.lod2_solid(),
            Self::Tunnel(v) => v.lod2_solid(),
            Self::TunnelConstructiveElement(v) => v.lod2_solid(),
            Self::TunnelFurniture(v) => v.lod2_solid(),
            Self::TunnelInstallation(v) => v.lod2_solid(),
            Self::TunnelPart(v) => v.lod2_solid(),
            Self::PlantCover(v) => v.lod2_solid(),
            Self::SolitaryVegetationObject(v) => v.lod2_solid(),
            Self::WaterBody(v) => v.lod2_solid(),
        }
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        match self {
            Self::Door(v) => v.lod0_point(),
            Self::OtherConstruction(v) => v.lod0_point(),
            Self::Window(v) => v.lod0_point(),
            Self::Bridge(v) => v.lod0_point(),
            Self::BridgeConstructiveElement(v) => v.lod0_point(),
            Self::BridgeFurniture(v) => v.lod0_point(),
            Self::BridgeInstallation(v) => v.lod0_point(),
            Self::BridgePart(v) => v.lod0_point(),
            Self::Building(v) => v.lod0_point(),
            Self::BuildingConstructiveElement(v) => v.lod0_point(),
            Self::BuildingFurniture(v) => v.lod0_point(),
            Self::BuildingInstallation(v) => v.lod0_point(),
            Self::BuildingPart(v) => v.lod0_point(),
            Self::CityFurniture(v) => v.lod0_point(),
            Self::GenericOccupiedSpace(v) => v.lod0_point(),
            Self::Tunnel(v) => v.lod0_point(),
            Self::TunnelConstructiveElement(v) => v.lod0_point(),
            Self::TunnelFurniture(v) => v.lod0_point(),
            Self::TunnelInstallation(v) => v.lod0_point(),
            Self::TunnelPart(v) => v.lod0_point(),
            Self::PlantCover(v) => v.lod0_point(),
            Self::SolitaryVegetationObject(v) => v.lod0_point(),
            Self::WaterBody(v) => v.lod0_point(),
        }
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod3_multi_curve(),
            Self::OtherConstruction(v) => v.lod3_multi_curve(),
            Self::Window(v) => v.lod3_multi_curve(),
            Self::Bridge(v) => v.lod3_multi_curve(),
            Self::BridgeConstructiveElement(v) => v.lod3_multi_curve(),
            Self::BridgeFurniture(v) => v.lod3_multi_curve(),
            Self::BridgeInstallation(v) => v.lod3_multi_curve(),
            Self::BridgePart(v) => v.lod3_multi_curve(),
            Self::Building(v) => v.lod3_multi_curve(),
            Self::BuildingConstructiveElement(v) => v.lod3_multi_curve(),
            Self::BuildingFurniture(v) => v.lod3_multi_curve(),
            Self::BuildingInstallation(v) => v.lod3_multi_curve(),
            Self::BuildingPart(v) => v.lod3_multi_curve(),
            Self::CityFurniture(v) => v.lod3_multi_curve(),
            Self::GenericOccupiedSpace(v) => v.lod3_multi_curve(),
            Self::Tunnel(v) => v.lod3_multi_curve(),
            Self::TunnelConstructiveElement(v) => v.lod3_multi_curve(),
            Self::TunnelFurniture(v) => v.lod3_multi_curve(),
            Self::TunnelInstallation(v) => v.lod3_multi_curve(),
            Self::TunnelPart(v) => v.lod3_multi_curve(),
            Self::PlantCover(v) => v.lod3_multi_curve(),
            Self::SolitaryVegetationObject(v) => v.lod3_multi_curve(),
            Self::WaterBody(v) => v.lod3_multi_curve(),
        }
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::Door(v) => v.lod2_multi_surface(),
            Self::OtherConstruction(v) => v.lod2_multi_surface(),
            Self::Window(v) => v.lod2_multi_surface(),
            Self::Bridge(v) => v.lod2_multi_surface(),
            Self::BridgeConstructiveElement(v) => v.lod2_multi_surface(),
            Self::BridgeFurniture(v) => v.lod2_multi_surface(),
            Self::BridgeInstallation(v) => v.lod2_multi_surface(),
            Self::BridgePart(v) => v.lod2_multi_surface(),
            Self::Building(v) => v.lod2_multi_surface(),
            Self::BuildingConstructiveElement(v) => v.lod2_multi_surface(),
            Self::BuildingFurniture(v) => v.lod2_multi_surface(),
            Self::BuildingInstallation(v) => v.lod2_multi_surface(),
            Self::BuildingPart(v) => v.lod2_multi_surface(),
            Self::CityFurniture(v) => v.lod2_multi_surface(),
            Self::GenericOccupiedSpace(v) => v.lod2_multi_surface(),
            Self::Tunnel(v) => v.lod2_multi_surface(),
            Self::TunnelConstructiveElement(v) => v.lod2_multi_surface(),
            Self::TunnelFurniture(v) => v.lod2_multi_surface(),
            Self::TunnelInstallation(v) => v.lod2_multi_surface(),
            Self::TunnelPart(v) => v.lod2_multi_surface(),
            Self::PlantCover(v) => v.lod2_multi_surface(),
            Self::SolitaryVegetationObject(v) => v.lod2_multi_surface(),
            Self::WaterBody(v) => v.lod2_multi_surface(),
        }
    }
}
impl AbstractPhysicalSpaceTrait for AbstractOccupiedSpace {
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod3_terrain_intersection_curve(),
            Self::OtherConstruction(v) => v.lod3_terrain_intersection_curve(),
            Self::Window(v) => v.lod3_terrain_intersection_curve(),
            Self::Bridge(v) => v.lod3_terrain_intersection_curve(),
            Self::BridgeConstructiveElement(v) => v.lod3_terrain_intersection_curve(),
            Self::BridgeFurniture(v) => v.lod3_terrain_intersection_curve(),
            Self::BridgeInstallation(v) => v.lod3_terrain_intersection_curve(),
            Self::BridgePart(v) => v.lod3_terrain_intersection_curve(),
            Self::Building(v) => v.lod3_terrain_intersection_curve(),
            Self::BuildingConstructiveElement(v) => v.lod3_terrain_intersection_curve(),
            Self::BuildingFurniture(v) => v.lod3_terrain_intersection_curve(),
            Self::BuildingInstallation(v) => v.lod3_terrain_intersection_curve(),
            Self::BuildingPart(v) => v.lod3_terrain_intersection_curve(),
            Self::CityFurniture(v) => v.lod3_terrain_intersection_curve(),
            Self::GenericOccupiedSpace(v) => v.lod3_terrain_intersection_curve(),
            Self::Tunnel(v) => v.lod3_terrain_intersection_curve(),
            Self::TunnelConstructiveElement(v) => v.lod3_terrain_intersection_curve(),
            Self::TunnelFurniture(v) => v.lod3_terrain_intersection_curve(),
            Self::TunnelInstallation(v) => v.lod3_terrain_intersection_curve(),
            Self::TunnelPart(v) => v.lod3_terrain_intersection_curve(),
            Self::PlantCover(v) => v.lod3_terrain_intersection_curve(),
            Self::SolitaryVegetationObject(v) => v.lod3_terrain_intersection_curve(),
            Self::WaterBody(v) => v.lod3_terrain_intersection_curve(),
        }
    }
    fn point_cloud(&self) -> Option<&AbstractPointCloud> {
        match self {
            Self::Door(v) => v.point_cloud(),
            Self::OtherConstruction(v) => v.point_cloud(),
            Self::Window(v) => v.point_cloud(),
            Self::Bridge(v) => v.point_cloud(),
            Self::BridgeConstructiveElement(v) => v.point_cloud(),
            Self::BridgeFurniture(v) => v.point_cloud(),
            Self::BridgeInstallation(v) => v.point_cloud(),
            Self::BridgePart(v) => v.point_cloud(),
            Self::Building(v) => v.point_cloud(),
            Self::BuildingConstructiveElement(v) => v.point_cloud(),
            Self::BuildingFurniture(v) => v.point_cloud(),
            Self::BuildingInstallation(v) => v.point_cloud(),
            Self::BuildingPart(v) => v.point_cloud(),
            Self::CityFurniture(v) => v.point_cloud(),
            Self::GenericOccupiedSpace(v) => v.point_cloud(),
            Self::Tunnel(v) => v.point_cloud(),
            Self::TunnelConstructiveElement(v) => v.point_cloud(),
            Self::TunnelFurniture(v) => v.point_cloud(),
            Self::TunnelInstallation(v) => v.point_cloud(),
            Self::TunnelPart(v) => v.point_cloud(),
            Self::PlantCover(v) => v.point_cloud(),
            Self::SolitaryVegetationObject(v) => v.point_cloud(),
            Self::WaterBody(v) => v.point_cloud(),
        }
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod1_terrain_intersection_curve(),
            Self::OtherConstruction(v) => v.lod1_terrain_intersection_curve(),
            Self::Window(v) => v.lod1_terrain_intersection_curve(),
            Self::Bridge(v) => v.lod1_terrain_intersection_curve(),
            Self::BridgeConstructiveElement(v) => v.lod1_terrain_intersection_curve(),
            Self::BridgeFurniture(v) => v.lod1_terrain_intersection_curve(),
            Self::BridgeInstallation(v) => v.lod1_terrain_intersection_curve(),
            Self::BridgePart(v) => v.lod1_terrain_intersection_curve(),
            Self::Building(v) => v.lod1_terrain_intersection_curve(),
            Self::BuildingConstructiveElement(v) => v.lod1_terrain_intersection_curve(),
            Self::BuildingFurniture(v) => v.lod1_terrain_intersection_curve(),
            Self::BuildingInstallation(v) => v.lod1_terrain_intersection_curve(),
            Self::BuildingPart(v) => v.lod1_terrain_intersection_curve(),
            Self::CityFurniture(v) => v.lod1_terrain_intersection_curve(),
            Self::GenericOccupiedSpace(v) => v.lod1_terrain_intersection_curve(),
            Self::Tunnel(v) => v.lod1_terrain_intersection_curve(),
            Self::TunnelConstructiveElement(v) => v.lod1_terrain_intersection_curve(),
            Self::TunnelFurniture(v) => v.lod1_terrain_intersection_curve(),
            Self::TunnelInstallation(v) => v.lod1_terrain_intersection_curve(),
            Self::TunnelPart(v) => v.lod1_terrain_intersection_curve(),
            Self::PlantCover(v) => v.lod1_terrain_intersection_curve(),
            Self::SolitaryVegetationObject(v) => v.lod1_terrain_intersection_curve(),
            Self::WaterBody(v) => v.lod1_terrain_intersection_curve(),
        }
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod2_terrain_intersection_curve(),
            Self::OtherConstruction(v) => v.lod2_terrain_intersection_curve(),
            Self::Window(v) => v.lod2_terrain_intersection_curve(),
            Self::Bridge(v) => v.lod2_terrain_intersection_curve(),
            Self::BridgeConstructiveElement(v) => v.lod2_terrain_intersection_curve(),
            Self::BridgeFurniture(v) => v.lod2_terrain_intersection_curve(),
            Self::BridgeInstallation(v) => v.lod2_terrain_intersection_curve(),
            Self::BridgePart(v) => v.lod2_terrain_intersection_curve(),
            Self::Building(v) => v.lod2_terrain_intersection_curve(),
            Self::BuildingConstructiveElement(v) => v.lod2_terrain_intersection_curve(),
            Self::BuildingFurniture(v) => v.lod2_terrain_intersection_curve(),
            Self::BuildingInstallation(v) => v.lod2_terrain_intersection_curve(),
            Self::BuildingPart(v) => v.lod2_terrain_intersection_curve(),
            Self::CityFurniture(v) => v.lod2_terrain_intersection_curve(),
            Self::GenericOccupiedSpace(v) => v.lod2_terrain_intersection_curve(),
            Self::Tunnel(v) => v.lod2_terrain_intersection_curve(),
            Self::TunnelConstructiveElement(v) => v.lod2_terrain_intersection_curve(),
            Self::TunnelFurniture(v) => v.lod2_terrain_intersection_curve(),
            Self::TunnelInstallation(v) => v.lod2_terrain_intersection_curve(),
            Self::TunnelPart(v) => v.lod2_terrain_intersection_curve(),
            Self::PlantCover(v) => v.lod2_terrain_intersection_curve(),
            Self::SolitaryVegetationObject(v) => v.lod2_terrain_intersection_curve(),
            Self::WaterBody(v) => v.lod2_terrain_intersection_curve(),
        }
    }
}
impl AbstractOccupiedSpaceTrait for AbstractOccupiedSpace {
    fn lod3_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::Door(v) => v.lod3_implicit_representation(),
            Self::OtherConstruction(v) => v.lod3_implicit_representation(),
            Self::Window(v) => v.lod3_implicit_representation(),
            Self::Bridge(v) => v.lod3_implicit_representation(),
            Self::BridgeConstructiveElement(v) => v.lod3_implicit_representation(),
            Self::BridgeFurniture(v) => v.lod3_implicit_representation(),
            Self::BridgeInstallation(v) => v.lod3_implicit_representation(),
            Self::BridgePart(v) => v.lod3_implicit_representation(),
            Self::Building(v) => v.lod3_implicit_representation(),
            Self::BuildingConstructiveElement(v) => v.lod3_implicit_representation(),
            Self::BuildingFurniture(v) => v.lod3_implicit_representation(),
            Self::BuildingInstallation(v) => v.lod3_implicit_representation(),
            Self::BuildingPart(v) => v.lod3_implicit_representation(),
            Self::CityFurniture(v) => v.lod3_implicit_representation(),
            Self::GenericOccupiedSpace(v) => v.lod3_implicit_representation(),
            Self::Tunnel(v) => v.lod3_implicit_representation(),
            Self::TunnelConstructiveElement(v) => v.lod3_implicit_representation(),
            Self::TunnelFurniture(v) => v.lod3_implicit_representation(),
            Self::TunnelInstallation(v) => v.lod3_implicit_representation(),
            Self::TunnelPart(v) => v.lod3_implicit_representation(),
            Self::PlantCover(v) => v.lod3_implicit_representation(),
            Self::SolitaryVegetationObject(v) => v.lod3_implicit_representation(),
            Self::WaterBody(v) => v.lod3_implicit_representation(),
        }
    }
    fn lod2_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::Door(v) => v.lod2_implicit_representation(),
            Self::OtherConstruction(v) => v.lod2_implicit_representation(),
            Self::Window(v) => v.lod2_implicit_representation(),
            Self::Bridge(v) => v.lod2_implicit_representation(),
            Self::BridgeConstructiveElement(v) => v.lod2_implicit_representation(),
            Self::BridgeFurniture(v) => v.lod2_implicit_representation(),
            Self::BridgeInstallation(v) => v.lod2_implicit_representation(),
            Self::BridgePart(v) => v.lod2_implicit_representation(),
            Self::Building(v) => v.lod2_implicit_representation(),
            Self::BuildingConstructiveElement(v) => v.lod2_implicit_representation(),
            Self::BuildingFurniture(v) => v.lod2_implicit_representation(),
            Self::BuildingInstallation(v) => v.lod2_implicit_representation(),
            Self::BuildingPart(v) => v.lod2_implicit_representation(),
            Self::CityFurniture(v) => v.lod2_implicit_representation(),
            Self::GenericOccupiedSpace(v) => v.lod2_implicit_representation(),
            Self::Tunnel(v) => v.lod2_implicit_representation(),
            Self::TunnelConstructiveElement(v) => v.lod2_implicit_representation(),
            Self::TunnelFurniture(v) => v.lod2_implicit_representation(),
            Self::TunnelInstallation(v) => v.lod2_implicit_representation(),
            Self::TunnelPart(v) => v.lod2_implicit_representation(),
            Self::PlantCover(v) => v.lod2_implicit_representation(),
            Self::SolitaryVegetationObject(v) => v.lod2_implicit_representation(),
            Self::WaterBody(v) => v.lod2_implicit_representation(),
        }
    }
    fn lod1_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::Door(v) => v.lod1_implicit_representation(),
            Self::OtherConstruction(v) => v.lod1_implicit_representation(),
            Self::Window(v) => v.lod1_implicit_representation(),
            Self::Bridge(v) => v.lod1_implicit_representation(),
            Self::BridgeConstructiveElement(v) => v.lod1_implicit_representation(),
            Self::BridgeFurniture(v) => v.lod1_implicit_representation(),
            Self::BridgeInstallation(v) => v.lod1_implicit_representation(),
            Self::BridgePart(v) => v.lod1_implicit_representation(),
            Self::Building(v) => v.lod1_implicit_representation(),
            Self::BuildingConstructiveElement(v) => v.lod1_implicit_representation(),
            Self::BuildingFurniture(v) => v.lod1_implicit_representation(),
            Self::BuildingInstallation(v) => v.lod1_implicit_representation(),
            Self::BuildingPart(v) => v.lod1_implicit_representation(),
            Self::CityFurniture(v) => v.lod1_implicit_representation(),
            Self::GenericOccupiedSpace(v) => v.lod1_implicit_representation(),
            Self::Tunnel(v) => v.lod1_implicit_representation(),
            Self::TunnelConstructiveElement(v) => v.lod1_implicit_representation(),
            Self::TunnelFurniture(v) => v.lod1_implicit_representation(),
            Self::TunnelInstallation(v) => v.lod1_implicit_representation(),
            Self::TunnelPart(v) => v.lod1_implicit_representation(),
            Self::PlantCover(v) => v.lod1_implicit_representation(),
            Self::SolitaryVegetationObject(v) => v.lod1_implicit_representation(),
            Self::WaterBody(v) => v.lod1_implicit_representation(),
        }
    }
}
impl From<Door> for AbstractOccupiedSpace {
    fn from(v: Door) -> Self {
        Self::Door(Box::new(v))
    }
}
impl From<OtherConstruction> for AbstractOccupiedSpace {
    fn from(v: OtherConstruction) -> Self {
        Self::OtherConstruction(Box::new(v))
    }
}
impl From<Window> for AbstractOccupiedSpace {
    fn from(v: Window) -> Self {
        Self::Window(Box::new(v))
    }
}
impl From<Bridge> for AbstractOccupiedSpace {
    fn from(v: Bridge) -> Self {
        Self::Bridge(Box::new(v))
    }
}
impl From<BridgeConstructiveElement> for AbstractOccupiedSpace {
    fn from(v: BridgeConstructiveElement) -> Self {
        Self::BridgeConstructiveElement(Box::new(v))
    }
}
impl From<BridgeFurniture> for AbstractOccupiedSpace {
    fn from(v: BridgeFurniture) -> Self {
        Self::BridgeFurniture(Box::new(v))
    }
}
impl From<BridgeInstallation> for AbstractOccupiedSpace {
    fn from(v: BridgeInstallation) -> Self {
        Self::BridgeInstallation(Box::new(v))
    }
}
impl From<BridgePart> for AbstractOccupiedSpace {
    fn from(v: BridgePart) -> Self {
        Self::BridgePart(Box::new(v))
    }
}
impl From<Building> for AbstractOccupiedSpace {
    fn from(v: Building) -> Self {
        Self::Building(Box::new(v))
    }
}
impl From<BuildingConstructiveElement> for AbstractOccupiedSpace {
    fn from(v: BuildingConstructiveElement) -> Self {
        Self::BuildingConstructiveElement(Box::new(v))
    }
}
impl From<BuildingFurniture> for AbstractOccupiedSpace {
    fn from(v: BuildingFurniture) -> Self {
        Self::BuildingFurniture(Box::new(v))
    }
}
impl From<BuildingInstallation> for AbstractOccupiedSpace {
    fn from(v: BuildingInstallation) -> Self {
        Self::BuildingInstallation(Box::new(v))
    }
}
impl From<BuildingPart> for AbstractOccupiedSpace {
    fn from(v: BuildingPart) -> Self {
        Self::BuildingPart(Box::new(v))
    }
}
impl From<CityFurniture> for AbstractOccupiedSpace {
    fn from(v: CityFurniture) -> Self {
        Self::CityFurniture(Box::new(v))
    }
}
impl From<GenericOccupiedSpace> for AbstractOccupiedSpace {
    fn from(v: GenericOccupiedSpace) -> Self {
        Self::GenericOccupiedSpace(Box::new(v))
    }
}
impl From<Tunnel> for AbstractOccupiedSpace {
    fn from(v: Tunnel) -> Self {
        Self::Tunnel(Box::new(v))
    }
}
impl From<TunnelConstructiveElement> for AbstractOccupiedSpace {
    fn from(v: TunnelConstructiveElement) -> Self {
        Self::TunnelConstructiveElement(Box::new(v))
    }
}
impl From<TunnelFurniture> for AbstractOccupiedSpace {
    fn from(v: TunnelFurniture) -> Self {
        Self::TunnelFurniture(Box::new(v))
    }
}
impl From<TunnelInstallation> for AbstractOccupiedSpace {
    fn from(v: TunnelInstallation) -> Self {
        Self::TunnelInstallation(Box::new(v))
    }
}
impl From<TunnelPart> for AbstractOccupiedSpace {
    fn from(v: TunnelPart) -> Self {
        Self::TunnelPart(Box::new(v))
    }
}
impl From<PlantCover> for AbstractOccupiedSpace {
    fn from(v: PlantCover) -> Self {
        Self::PlantCover(Box::new(v))
    }
}
impl From<SolitaryVegetationObject> for AbstractOccupiedSpace {
    fn from(v: SolitaryVegetationObject) -> Self {
        Self::SolitaryVegetationObject(Box::new(v))
    }
}
impl From<WaterBody> for AbstractOccupiedSpace {
    fn from(v: WaterBody) -> Self {
        Self::WaterBody(Box::new(v))
    }
}
pub trait AbstractOccupiedSpaceAccessors {
    fn doors(&self) -> impl Iterator<Item = &Door>;
    fn other_constructions(&self) -> impl Iterator<Item = &OtherConstruction>;
    fn windows(&self) -> impl Iterator<Item = &Window>;
    fn bridges(&self) -> impl Iterator<Item = &Bridge>;
    fn bridge_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BridgeConstructiveElement>;
    fn bridge_furnitures(&self) -> impl Iterator<Item = &BridgeFurniture>;
    fn bridge_installations(&self) -> impl Iterator<Item = &BridgeInstallation>;
    fn bridge_parts(&self) -> impl Iterator<Item = &BridgePart>;
    fn buildings(&self) -> impl Iterator<Item = &Building>;
    fn building_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BuildingConstructiveElement>;
    fn building_furnitures(&self) -> impl Iterator<Item = &BuildingFurniture>;
    fn building_installations(&self) -> impl Iterator<Item = &BuildingInstallation>;
    fn building_parts(&self) -> impl Iterator<Item = &BuildingPart>;
    fn city_furnitures(&self) -> impl Iterator<Item = &CityFurniture>;
    fn generic_occupied_spaces(&self) -> impl Iterator<Item = &GenericOccupiedSpace>;
    fn tunnels(&self) -> impl Iterator<Item = &Tunnel>;
    fn tunnel_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &TunnelConstructiveElement>;
    fn tunnel_furnitures(&self) -> impl Iterator<Item = &TunnelFurniture>;
    fn tunnel_installations(&self) -> impl Iterator<Item = &TunnelInstallation>;
    fn tunnel_parts(&self) -> impl Iterator<Item = &TunnelPart>;
    fn plant_covers(&self) -> impl Iterator<Item = &PlantCover>;
    fn solitary_vegetation_objects(
        &self,
    ) -> impl Iterator<Item = &SolitaryVegetationObject>;
    fn water_bodys(&self) -> impl Iterator<Item = &WaterBody>;
}
impl AbstractOccupiedSpaceAccessors for [AbstractOccupiedSpace] {
    fn doors(&self) -> impl Iterator<Item = &Door> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::Door(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn other_constructions(&self) -> impl Iterator<Item = &OtherConstruction> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::OtherConstruction(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn windows(&self) -> impl Iterator<Item = &Window> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::Window(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridges(&self) -> impl Iterator<Item = &Bridge> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::Bridge(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BridgeConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::BridgeConstructiveElement(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_furnitures(&self) -> impl Iterator<Item = &BridgeFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::BridgeFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_installations(&self) -> impl Iterator<Item = &BridgeInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::BridgeInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn bridge_parts(&self) -> impl Iterator<Item = &BridgePart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::BridgePart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn buildings(&self) -> impl Iterator<Item = &Building> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::Building(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &BuildingConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::BuildingConstructiveElement(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_furnitures(&self) -> impl Iterator<Item = &BuildingFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::BuildingFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_installations(&self) -> impl Iterator<Item = &BuildingInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::BuildingInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_parts(&self) -> impl Iterator<Item = &BuildingPart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::BuildingPart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn city_furnitures(&self) -> impl Iterator<Item = &CityFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::CityFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_occupied_spaces(&self) -> impl Iterator<Item = &GenericOccupiedSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::GenericOccupiedSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnels(&self) -> impl Iterator<Item = &Tunnel> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::Tunnel(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_constructive_elements(
        &self,
    ) -> impl Iterator<Item = &TunnelConstructiveElement> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::TunnelConstructiveElement(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_furnitures(&self) -> impl Iterator<Item = &TunnelFurniture> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::TunnelFurniture(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_installations(&self) -> impl Iterator<Item = &TunnelInstallation> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::TunnelInstallation(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tunnel_parts(&self) -> impl Iterator<Item = &TunnelPart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::TunnelPart(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn plant_covers(&self) -> impl Iterator<Item = &PlantCover> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::PlantCover(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn solitary_vegetation_objects(
        &self,
    ) -> impl Iterator<Item = &SolitaryVegetationObject> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::SolitaryVegetationObject(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn water_bodys(&self) -> impl Iterator<Item = &WaterBody> {
        self.iter()
            .filter_map(|item| match item {
                AbstractOccupiedSpace::WaterBody(v) => Some(v.as_ref()),
                _ => None,
            })
    }
}
pub trait AbstractUnoccupiedSpaceTrait: AbstractPhysicalSpaceTrait {}
#[derive(Debug, Clone)]
pub enum AbstractUnoccupiedSpace {
    BridgeRoom(Box<BridgeRoom>),
    BuildingRoom(Box<BuildingRoom>),
    GenericUnoccupiedSpace(Box<GenericUnoccupiedSpace>),
    AuxiliaryTrafficSpace(Box<AuxiliaryTrafficSpace>),
    ClearanceSpace(Box<ClearanceSpace>),
    Hole(Box<Hole>),
    Intersection(Box<Intersection>),
    Railway(Box<Railway>),
    Road(Box<Road>),
    Section(Box<Section>),
    Square(Box<Square>),
    Track(Box<Track>),
    TrafficSpace(Box<TrafficSpace>),
    Waterway(Box<Waterway>),
    HollowSpace(Box<HollowSpace>),
}
impl Default for AbstractUnoccupiedSpace {
    fn default() -> Self {
        Self::BridgeRoom(Box::default())
    }
}
impl AbstractFeatureTrait for AbstractUnoccupiedSpace {
    fn feature_id(&self) -> &ID {
        match self {
            Self::BridgeRoom(v) => v.feature_id(),
            Self::BuildingRoom(v) => v.feature_id(),
            Self::GenericUnoccupiedSpace(v) => v.feature_id(),
            Self::AuxiliaryTrafficSpace(v) => v.feature_id(),
            Self::ClearanceSpace(v) => v.feature_id(),
            Self::Hole(v) => v.feature_id(),
            Self::Intersection(v) => v.feature_id(),
            Self::Railway(v) => v.feature_id(),
            Self::Road(v) => v.feature_id(),
            Self::Section(v) => v.feature_id(),
            Self::Square(v) => v.feature_id(),
            Self::Track(v) => v.feature_id(),
            Self::TrafficSpace(v) => v.feature_id(),
            Self::Waterway(v) => v.feature_id(),
            Self::HollowSpace(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::BridgeRoom(v) => v.identifier(),
            Self::BuildingRoom(v) => v.identifier(),
            Self::GenericUnoccupiedSpace(v) => v.identifier(),
            Self::AuxiliaryTrafficSpace(v) => v.identifier(),
            Self::ClearanceSpace(v) => v.identifier(),
            Self::Hole(v) => v.identifier(),
            Self::Intersection(v) => v.identifier(),
            Self::Railway(v) => v.identifier(),
            Self::Road(v) => v.identifier(),
            Self::Section(v) => v.identifier(),
            Self::Square(v) => v.identifier(),
            Self::Track(v) => v.identifier(),
            Self::TrafficSpace(v) => v.identifier(),
            Self::Waterway(v) => v.identifier(),
            Self::HollowSpace(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::BridgeRoom(v) => v.name(),
            Self::BuildingRoom(v) => v.name(),
            Self::GenericUnoccupiedSpace(v) => v.name(),
            Self::AuxiliaryTrafficSpace(v) => v.name(),
            Self::ClearanceSpace(v) => v.name(),
            Self::Hole(v) => v.name(),
            Self::Intersection(v) => v.name(),
            Self::Railway(v) => v.name(),
            Self::Road(v) => v.name(),
            Self::Section(v) => v.name(),
            Self::Square(v) => v.name(),
            Self::Track(v) => v.name(),
            Self::TrafficSpace(v) => v.name(),
            Self::Waterway(v) => v.name(),
            Self::HollowSpace(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::BridgeRoom(v) => v.description(),
            Self::BuildingRoom(v) => v.description(),
            Self::GenericUnoccupiedSpace(v) => v.description(),
            Self::AuxiliaryTrafficSpace(v) => v.description(),
            Self::ClearanceSpace(v) => v.description(),
            Self::Hole(v) => v.description(),
            Self::Intersection(v) => v.description(),
            Self::Railway(v) => v.description(),
            Self::Road(v) => v.description(),
            Self::Section(v) => v.description(),
            Self::Square(v) => v.description(),
            Self::Track(v) => v.description(),
            Self::TrafficSpace(v) => v.description(),
            Self::Waterway(v) => v.description(),
            Self::HollowSpace(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractUnoccupiedSpace {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::BridgeRoom(v) => v.creation_date(),
            Self::BuildingRoom(v) => v.creation_date(),
            Self::GenericUnoccupiedSpace(v) => v.creation_date(),
            Self::AuxiliaryTrafficSpace(v) => v.creation_date(),
            Self::ClearanceSpace(v) => v.creation_date(),
            Self::Hole(v) => v.creation_date(),
            Self::Intersection(v) => v.creation_date(),
            Self::Railway(v) => v.creation_date(),
            Self::Road(v) => v.creation_date(),
            Self::Section(v) => v.creation_date(),
            Self::Square(v) => v.creation_date(),
            Self::Track(v) => v.creation_date(),
            Self::TrafficSpace(v) => v.creation_date(),
            Self::Waterway(v) => v.creation_date(),
            Self::HollowSpace(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::BridgeRoom(v) => v.termination_date(),
            Self::BuildingRoom(v) => v.termination_date(),
            Self::GenericUnoccupiedSpace(v) => v.termination_date(),
            Self::AuxiliaryTrafficSpace(v) => v.termination_date(),
            Self::ClearanceSpace(v) => v.termination_date(),
            Self::Hole(v) => v.termination_date(),
            Self::Intersection(v) => v.termination_date(),
            Self::Railway(v) => v.termination_date(),
            Self::Road(v) => v.termination_date(),
            Self::Section(v) => v.termination_date(),
            Self::Square(v) => v.termination_date(),
            Self::Track(v) => v.termination_date(),
            Self::TrafficSpace(v) => v.termination_date(),
            Self::Waterway(v) => v.termination_date(),
            Self::HollowSpace(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::BridgeRoom(v) => v.valid_from(),
            Self::BuildingRoom(v) => v.valid_from(),
            Self::GenericUnoccupiedSpace(v) => v.valid_from(),
            Self::AuxiliaryTrafficSpace(v) => v.valid_from(),
            Self::ClearanceSpace(v) => v.valid_from(),
            Self::Hole(v) => v.valid_from(),
            Self::Intersection(v) => v.valid_from(),
            Self::Railway(v) => v.valid_from(),
            Self::Road(v) => v.valid_from(),
            Self::Section(v) => v.valid_from(),
            Self::Square(v) => v.valid_from(),
            Self::Track(v) => v.valid_from(),
            Self::TrafficSpace(v) => v.valid_from(),
            Self::Waterway(v) => v.valid_from(),
            Self::HollowSpace(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::BridgeRoom(v) => v.valid_to(),
            Self::BuildingRoom(v) => v.valid_to(),
            Self::GenericUnoccupiedSpace(v) => v.valid_to(),
            Self::AuxiliaryTrafficSpace(v) => v.valid_to(),
            Self::ClearanceSpace(v) => v.valid_to(),
            Self::Hole(v) => v.valid_to(),
            Self::Intersection(v) => v.valid_to(),
            Self::Railway(v) => v.valid_to(),
            Self::Road(v) => v.valid_to(),
            Self::Section(v) => v.valid_to(),
            Self::Square(v) => v.valid_to(),
            Self::Track(v) => v.valid_to(),
            Self::TrafficSpace(v) => v.valid_to(),
            Self::Waterway(v) => v.valid_to(),
            Self::HollowSpace(v) => v.valid_to(),
        }
    }
}
impl AbstractCityObjectTrait for AbstractUnoccupiedSpace {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::BridgeRoom(v) => v.relative_to_terrain(),
            Self::BuildingRoom(v) => v.relative_to_terrain(),
            Self::GenericUnoccupiedSpace(v) => v.relative_to_terrain(),
            Self::AuxiliaryTrafficSpace(v) => v.relative_to_terrain(),
            Self::ClearanceSpace(v) => v.relative_to_terrain(),
            Self::Hole(v) => v.relative_to_terrain(),
            Self::Intersection(v) => v.relative_to_terrain(),
            Self::Railway(v) => v.relative_to_terrain(),
            Self::Road(v) => v.relative_to_terrain(),
            Self::Section(v) => v.relative_to_terrain(),
            Self::Square(v) => v.relative_to_terrain(),
            Self::Track(v) => v.relative_to_terrain(),
            Self::TrafficSpace(v) => v.relative_to_terrain(),
            Self::Waterway(v) => v.relative_to_terrain(),
            Self::HollowSpace(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::BridgeRoom(v) => v.relative_to_water(),
            Self::BuildingRoom(v) => v.relative_to_water(),
            Self::GenericUnoccupiedSpace(v) => v.relative_to_water(),
            Self::AuxiliaryTrafficSpace(v) => v.relative_to_water(),
            Self::ClearanceSpace(v) => v.relative_to_water(),
            Self::Hole(v) => v.relative_to_water(),
            Self::Intersection(v) => v.relative_to_water(),
            Self::Railway(v) => v.relative_to_water(),
            Self::Road(v) => v.relative_to_water(),
            Self::Section(v) => v.relative_to_water(),
            Self::Square(v) => v.relative_to_water(),
            Self::Track(v) => v.relative_to_water(),
            Self::TrafficSpace(v) => v.relative_to_water(),
            Self::Waterway(v) => v.relative_to_water(),
            Self::HollowSpace(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::BridgeRoom(v) => v.appearance(),
            Self::BuildingRoom(v) => v.appearance(),
            Self::GenericUnoccupiedSpace(v) => v.appearance(),
            Self::AuxiliaryTrafficSpace(v) => v.appearance(),
            Self::ClearanceSpace(v) => v.appearance(),
            Self::Hole(v) => v.appearance(),
            Self::Intersection(v) => v.appearance(),
            Self::Railway(v) => v.appearance(),
            Self::Road(v) => v.appearance(),
            Self::Section(v) => v.appearance(),
            Self::Square(v) => v.appearance(),
            Self::Track(v) => v.appearance(),
            Self::TrafficSpace(v) => v.appearance(),
            Self::Waterway(v) => v.appearance(),
            Self::HollowSpace(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::BridgeRoom(v) => v.generalizes_to(),
            Self::BuildingRoom(v) => v.generalizes_to(),
            Self::GenericUnoccupiedSpace(v) => v.generalizes_to(),
            Self::AuxiliaryTrafficSpace(v) => v.generalizes_to(),
            Self::ClearanceSpace(v) => v.generalizes_to(),
            Self::Hole(v) => v.generalizes_to(),
            Self::Intersection(v) => v.generalizes_to(),
            Self::Railway(v) => v.generalizes_to(),
            Self::Road(v) => v.generalizes_to(),
            Self::Section(v) => v.generalizes_to(),
            Self::Square(v) => v.generalizes_to(),
            Self::Track(v) => v.generalizes_to(),
            Self::TrafficSpace(v) => v.generalizes_to(),
            Self::Waterway(v) => v.generalizes_to(),
            Self::HollowSpace(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::BridgeRoom(v) => v.external_reference(),
            Self::BuildingRoom(v) => v.external_reference(),
            Self::GenericUnoccupiedSpace(v) => v.external_reference(),
            Self::AuxiliaryTrafficSpace(v) => v.external_reference(),
            Self::ClearanceSpace(v) => v.external_reference(),
            Self::Hole(v) => v.external_reference(),
            Self::Intersection(v) => v.external_reference(),
            Self::Railway(v) => v.external_reference(),
            Self::Road(v) => v.external_reference(),
            Self::Section(v) => v.external_reference(),
            Self::Square(v) => v.external_reference(),
            Self::Track(v) => v.external_reference(),
            Self::TrafficSpace(v) => v.external_reference(),
            Self::Waterway(v) => v.external_reference(),
            Self::HollowSpace(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::BridgeRoom(v) => v.related_to(),
            Self::BuildingRoom(v) => v.related_to(),
            Self::GenericUnoccupiedSpace(v) => v.related_to(),
            Self::AuxiliaryTrafficSpace(v) => v.related_to(),
            Self::ClearanceSpace(v) => v.related_to(),
            Self::Hole(v) => v.related_to(),
            Self::Intersection(v) => v.related_to(),
            Self::Railway(v) => v.related_to(),
            Self::Road(v) => v.related_to(),
            Self::Section(v) => v.related_to(),
            Self::Square(v) => v.related_to(),
            Self::Track(v) => v.related_to(),
            Self::TrafficSpace(v) => v.related_to(),
            Self::Waterway(v) => v.related_to(),
            Self::HollowSpace(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::BridgeRoom(v) => v.dynamizer(),
            Self::BuildingRoom(v) => v.dynamizer(),
            Self::GenericUnoccupiedSpace(v) => v.dynamizer(),
            Self::AuxiliaryTrafficSpace(v) => v.dynamizer(),
            Self::ClearanceSpace(v) => v.dynamizer(),
            Self::Hole(v) => v.dynamizer(),
            Self::Intersection(v) => v.dynamizer(),
            Self::Railway(v) => v.dynamizer(),
            Self::Road(v) => v.dynamizer(),
            Self::Section(v) => v.dynamizer(),
            Self::Square(v) => v.dynamizer(),
            Self::Track(v) => v.dynamizer(),
            Self::TrafficSpace(v) => v.dynamizer(),
            Self::Waterway(v) => v.dynamizer(),
            Self::HollowSpace(v) => v.dynamizer(),
        }
    }
}
impl AbstractSpaceTrait for AbstractUnoccupiedSpace {
    fn space_type(&self) -> Option<SpaceType> {
        match self {
            Self::BridgeRoom(v) => v.space_type(),
            Self::BuildingRoom(v) => v.space_type(),
            Self::GenericUnoccupiedSpace(v) => v.space_type(),
            Self::AuxiliaryTrafficSpace(v) => v.space_type(),
            Self::ClearanceSpace(v) => v.space_type(),
            Self::Hole(v) => v.space_type(),
            Self::Intersection(v) => v.space_type(),
            Self::Railway(v) => v.space_type(),
            Self::Road(v) => v.space_type(),
            Self::Section(v) => v.space_type(),
            Self::Square(v) => v.space_type(),
            Self::Track(v) => v.space_type(),
            Self::TrafficSpace(v) => v.space_type(),
            Self::Waterway(v) => v.space_type(),
            Self::HollowSpace(v) => v.space_type(),
        }
    }
    fn volume(&self) -> &[QualifiedVolume] {
        match self {
            Self::BridgeRoom(v) => v.volume(),
            Self::BuildingRoom(v) => v.volume(),
            Self::GenericUnoccupiedSpace(v) => v.volume(),
            Self::AuxiliaryTrafficSpace(v) => v.volume(),
            Self::ClearanceSpace(v) => v.volume(),
            Self::Hole(v) => v.volume(),
            Self::Intersection(v) => v.volume(),
            Self::Railway(v) => v.volume(),
            Self::Road(v) => v.volume(),
            Self::Section(v) => v.volume(),
            Self::Square(v) => v.volume(),
            Self::Track(v) => v.volume(),
            Self::TrafficSpace(v) => v.volume(),
            Self::Waterway(v) => v.volume(),
            Self::HollowSpace(v) => v.volume(),
        }
    }
    fn area(&self) -> &[QualifiedArea] {
        match self {
            Self::BridgeRoom(v) => v.area(),
            Self::BuildingRoom(v) => v.area(),
            Self::GenericUnoccupiedSpace(v) => v.area(),
            Self::AuxiliaryTrafficSpace(v) => v.area(),
            Self::ClearanceSpace(v) => v.area(),
            Self::Hole(v) => v.area(),
            Self::Intersection(v) => v.area(),
            Self::Railway(v) => v.area(),
            Self::Road(v) => v.area(),
            Self::Section(v) => v.area(),
            Self::Square(v) => v.area(),
            Self::Track(v) => v.area(),
            Self::TrafficSpace(v) => v.area(),
            Self::Waterway(v) => v.area(),
            Self::HollowSpace(v) => v.area(),
        }
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeRoom(v) => v.lod2_multi_curve(),
            Self::BuildingRoom(v) => v.lod2_multi_curve(),
            Self::GenericUnoccupiedSpace(v) => v.lod2_multi_curve(),
            Self::AuxiliaryTrafficSpace(v) => v.lod2_multi_curve(),
            Self::ClearanceSpace(v) => v.lod2_multi_curve(),
            Self::Hole(v) => v.lod2_multi_curve(),
            Self::Intersection(v) => v.lod2_multi_curve(),
            Self::Railway(v) => v.lod2_multi_curve(),
            Self::Road(v) => v.lod2_multi_curve(),
            Self::Section(v) => v.lod2_multi_curve(),
            Self::Square(v) => v.lod2_multi_curve(),
            Self::Track(v) => v.lod2_multi_curve(),
            Self::TrafficSpace(v) => v.lod2_multi_curve(),
            Self::Waterway(v) => v.lod2_multi_curve(),
            Self::HollowSpace(v) => v.lod2_multi_curve(),
        }
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::BridgeRoom(v) => v.lod3_multi_surface(),
            Self::BuildingRoom(v) => v.lod3_multi_surface(),
            Self::GenericUnoccupiedSpace(v) => v.lod3_multi_surface(),
            Self::AuxiliaryTrafficSpace(v) => v.lod3_multi_surface(),
            Self::ClearanceSpace(v) => v.lod3_multi_surface(),
            Self::Hole(v) => v.lod3_multi_surface(),
            Self::Intersection(v) => v.lod3_multi_surface(),
            Self::Railway(v) => v.lod3_multi_surface(),
            Self::Road(v) => v.lod3_multi_surface(),
            Self::Section(v) => v.lod3_multi_surface(),
            Self::Square(v) => v.lod3_multi_surface(),
            Self::Track(v) => v.lod3_multi_surface(),
            Self::TrafficSpace(v) => v.lod3_multi_surface(),
            Self::Waterway(v) => v.lod3_multi_surface(),
            Self::HollowSpace(v) => v.lod3_multi_surface(),
        }
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::BridgeRoom(v) => v.lod0_multi_surface(),
            Self::BuildingRoom(v) => v.lod0_multi_surface(),
            Self::GenericUnoccupiedSpace(v) => v.lod0_multi_surface(),
            Self::AuxiliaryTrafficSpace(v) => v.lod0_multi_surface(),
            Self::ClearanceSpace(v) => v.lod0_multi_surface(),
            Self::Hole(v) => v.lod0_multi_surface(),
            Self::Intersection(v) => v.lod0_multi_surface(),
            Self::Railway(v) => v.lod0_multi_surface(),
            Self::Road(v) => v.lod0_multi_surface(),
            Self::Section(v) => v.lod0_multi_surface(),
            Self::Square(v) => v.lod0_multi_surface(),
            Self::Track(v) => v.lod0_multi_surface(),
            Self::TrafficSpace(v) => v.lod0_multi_surface(),
            Self::Waterway(v) => v.lod0_multi_surface(),
            Self::HollowSpace(v) => v.lod0_multi_surface(),
        }
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::BridgeRoom(v) => v.lod1_solid(),
            Self::BuildingRoom(v) => v.lod1_solid(),
            Self::GenericUnoccupiedSpace(v) => v.lod1_solid(),
            Self::AuxiliaryTrafficSpace(v) => v.lod1_solid(),
            Self::ClearanceSpace(v) => v.lod1_solid(),
            Self::Hole(v) => v.lod1_solid(),
            Self::Intersection(v) => v.lod1_solid(),
            Self::Railway(v) => v.lod1_solid(),
            Self::Road(v) => v.lod1_solid(),
            Self::Section(v) => v.lod1_solid(),
            Self::Square(v) => v.lod1_solid(),
            Self::Track(v) => v.lod1_solid(),
            Self::TrafficSpace(v) => v.lod1_solid(),
            Self::Waterway(v) => v.lod1_solid(),
            Self::HollowSpace(v) => v.lod1_solid(),
        }
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::BridgeRoom(v) => v.lod3_solid(),
            Self::BuildingRoom(v) => v.lod3_solid(),
            Self::GenericUnoccupiedSpace(v) => v.lod3_solid(),
            Self::AuxiliaryTrafficSpace(v) => v.lod3_solid(),
            Self::ClearanceSpace(v) => v.lod3_solid(),
            Self::Hole(v) => v.lod3_solid(),
            Self::Intersection(v) => v.lod3_solid(),
            Self::Railway(v) => v.lod3_solid(),
            Self::Road(v) => v.lod3_solid(),
            Self::Section(v) => v.lod3_solid(),
            Self::Square(v) => v.lod3_solid(),
            Self::Track(v) => v.lod3_solid(),
            Self::TrafficSpace(v) => v.lod3_solid(),
            Self::Waterway(v) => v.lod3_solid(),
            Self::HollowSpace(v) => v.lod3_solid(),
        }
    }
    fn boundary(&self) -> &[AbstractSpaceBoundary] {
        match self {
            Self::BridgeRoom(v) => v.boundary(),
            Self::BuildingRoom(v) => v.boundary(),
            Self::GenericUnoccupiedSpace(v) => v.boundary(),
            Self::AuxiliaryTrafficSpace(v) => v.boundary(),
            Self::ClearanceSpace(v) => v.boundary(),
            Self::Hole(v) => v.boundary(),
            Self::Intersection(v) => v.boundary(),
            Self::Railway(v) => v.boundary(),
            Self::Road(v) => v.boundary(),
            Self::Section(v) => v.boundary(),
            Self::Square(v) => v.boundary(),
            Self::Track(v) => v.boundary(),
            Self::TrafficSpace(v) => v.boundary(),
            Self::Waterway(v) => v.boundary(),
            Self::HollowSpace(v) => v.boundary(),
        }
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeRoom(v) => v.lod0_multi_curve(),
            Self::BuildingRoom(v) => v.lod0_multi_curve(),
            Self::GenericUnoccupiedSpace(v) => v.lod0_multi_curve(),
            Self::AuxiliaryTrafficSpace(v) => v.lod0_multi_curve(),
            Self::ClearanceSpace(v) => v.lod0_multi_curve(),
            Self::Hole(v) => v.lod0_multi_curve(),
            Self::Intersection(v) => v.lod0_multi_curve(),
            Self::Railway(v) => v.lod0_multi_curve(),
            Self::Road(v) => v.lod0_multi_curve(),
            Self::Section(v) => v.lod0_multi_curve(),
            Self::Square(v) => v.lod0_multi_curve(),
            Self::Track(v) => v.lod0_multi_curve(),
            Self::TrafficSpace(v) => v.lod0_multi_curve(),
            Self::Waterway(v) => v.lod0_multi_curve(),
            Self::HollowSpace(v) => v.lod0_multi_curve(),
        }
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::BridgeRoom(v) => v.lod2_solid(),
            Self::BuildingRoom(v) => v.lod2_solid(),
            Self::GenericUnoccupiedSpace(v) => v.lod2_solid(),
            Self::AuxiliaryTrafficSpace(v) => v.lod2_solid(),
            Self::ClearanceSpace(v) => v.lod2_solid(),
            Self::Hole(v) => v.lod2_solid(),
            Self::Intersection(v) => v.lod2_solid(),
            Self::Railway(v) => v.lod2_solid(),
            Self::Road(v) => v.lod2_solid(),
            Self::Section(v) => v.lod2_solid(),
            Self::Square(v) => v.lod2_solid(),
            Self::Track(v) => v.lod2_solid(),
            Self::TrafficSpace(v) => v.lod2_solid(),
            Self::Waterway(v) => v.lod2_solid(),
            Self::HollowSpace(v) => v.lod2_solid(),
        }
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        match self {
            Self::BridgeRoom(v) => v.lod0_point(),
            Self::BuildingRoom(v) => v.lod0_point(),
            Self::GenericUnoccupiedSpace(v) => v.lod0_point(),
            Self::AuxiliaryTrafficSpace(v) => v.lod0_point(),
            Self::ClearanceSpace(v) => v.lod0_point(),
            Self::Hole(v) => v.lod0_point(),
            Self::Intersection(v) => v.lod0_point(),
            Self::Railway(v) => v.lod0_point(),
            Self::Road(v) => v.lod0_point(),
            Self::Section(v) => v.lod0_point(),
            Self::Square(v) => v.lod0_point(),
            Self::Track(v) => v.lod0_point(),
            Self::TrafficSpace(v) => v.lod0_point(),
            Self::Waterway(v) => v.lod0_point(),
            Self::HollowSpace(v) => v.lod0_point(),
        }
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeRoom(v) => v.lod3_multi_curve(),
            Self::BuildingRoom(v) => v.lod3_multi_curve(),
            Self::GenericUnoccupiedSpace(v) => v.lod3_multi_curve(),
            Self::AuxiliaryTrafficSpace(v) => v.lod3_multi_curve(),
            Self::ClearanceSpace(v) => v.lod3_multi_curve(),
            Self::Hole(v) => v.lod3_multi_curve(),
            Self::Intersection(v) => v.lod3_multi_curve(),
            Self::Railway(v) => v.lod3_multi_curve(),
            Self::Road(v) => v.lod3_multi_curve(),
            Self::Section(v) => v.lod3_multi_curve(),
            Self::Square(v) => v.lod3_multi_curve(),
            Self::Track(v) => v.lod3_multi_curve(),
            Self::TrafficSpace(v) => v.lod3_multi_curve(),
            Self::Waterway(v) => v.lod3_multi_curve(),
            Self::HollowSpace(v) => v.lod3_multi_curve(),
        }
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::BridgeRoom(v) => v.lod2_multi_surface(),
            Self::BuildingRoom(v) => v.lod2_multi_surface(),
            Self::GenericUnoccupiedSpace(v) => v.lod2_multi_surface(),
            Self::AuxiliaryTrafficSpace(v) => v.lod2_multi_surface(),
            Self::ClearanceSpace(v) => v.lod2_multi_surface(),
            Self::Hole(v) => v.lod2_multi_surface(),
            Self::Intersection(v) => v.lod2_multi_surface(),
            Self::Railway(v) => v.lod2_multi_surface(),
            Self::Road(v) => v.lod2_multi_surface(),
            Self::Section(v) => v.lod2_multi_surface(),
            Self::Square(v) => v.lod2_multi_surface(),
            Self::Track(v) => v.lod2_multi_surface(),
            Self::TrafficSpace(v) => v.lod2_multi_surface(),
            Self::Waterway(v) => v.lod2_multi_surface(),
            Self::HollowSpace(v) => v.lod2_multi_surface(),
        }
    }
}
impl AbstractPhysicalSpaceTrait for AbstractUnoccupiedSpace {
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeRoom(v) => v.lod3_terrain_intersection_curve(),
            Self::BuildingRoom(v) => v.lod3_terrain_intersection_curve(),
            Self::GenericUnoccupiedSpace(v) => v.lod3_terrain_intersection_curve(),
            Self::AuxiliaryTrafficSpace(v) => v.lod3_terrain_intersection_curve(),
            Self::ClearanceSpace(v) => v.lod3_terrain_intersection_curve(),
            Self::Hole(v) => v.lod3_terrain_intersection_curve(),
            Self::Intersection(v) => v.lod3_terrain_intersection_curve(),
            Self::Railway(v) => v.lod3_terrain_intersection_curve(),
            Self::Road(v) => v.lod3_terrain_intersection_curve(),
            Self::Section(v) => v.lod3_terrain_intersection_curve(),
            Self::Square(v) => v.lod3_terrain_intersection_curve(),
            Self::Track(v) => v.lod3_terrain_intersection_curve(),
            Self::TrafficSpace(v) => v.lod3_terrain_intersection_curve(),
            Self::Waterway(v) => v.lod3_terrain_intersection_curve(),
            Self::HollowSpace(v) => v.lod3_terrain_intersection_curve(),
        }
    }
    fn point_cloud(&self) -> Option<&AbstractPointCloud> {
        match self {
            Self::BridgeRoom(v) => v.point_cloud(),
            Self::BuildingRoom(v) => v.point_cloud(),
            Self::GenericUnoccupiedSpace(v) => v.point_cloud(),
            Self::AuxiliaryTrafficSpace(v) => v.point_cloud(),
            Self::ClearanceSpace(v) => v.point_cloud(),
            Self::Hole(v) => v.point_cloud(),
            Self::Intersection(v) => v.point_cloud(),
            Self::Railway(v) => v.point_cloud(),
            Self::Road(v) => v.point_cloud(),
            Self::Section(v) => v.point_cloud(),
            Self::Square(v) => v.point_cloud(),
            Self::Track(v) => v.point_cloud(),
            Self::TrafficSpace(v) => v.point_cloud(),
            Self::Waterway(v) => v.point_cloud(),
            Self::HollowSpace(v) => v.point_cloud(),
        }
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeRoom(v) => v.lod1_terrain_intersection_curve(),
            Self::BuildingRoom(v) => v.lod1_terrain_intersection_curve(),
            Self::GenericUnoccupiedSpace(v) => v.lod1_terrain_intersection_curve(),
            Self::AuxiliaryTrafficSpace(v) => v.lod1_terrain_intersection_curve(),
            Self::ClearanceSpace(v) => v.lod1_terrain_intersection_curve(),
            Self::Hole(v) => v.lod1_terrain_intersection_curve(),
            Self::Intersection(v) => v.lod1_terrain_intersection_curve(),
            Self::Railway(v) => v.lod1_terrain_intersection_curve(),
            Self::Road(v) => v.lod1_terrain_intersection_curve(),
            Self::Section(v) => v.lod1_terrain_intersection_curve(),
            Self::Square(v) => v.lod1_terrain_intersection_curve(),
            Self::Track(v) => v.lod1_terrain_intersection_curve(),
            Self::TrafficSpace(v) => v.lod1_terrain_intersection_curve(),
            Self::Waterway(v) => v.lod1_terrain_intersection_curve(),
            Self::HollowSpace(v) => v.lod1_terrain_intersection_curve(),
        }
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeRoom(v) => v.lod2_terrain_intersection_curve(),
            Self::BuildingRoom(v) => v.lod2_terrain_intersection_curve(),
            Self::GenericUnoccupiedSpace(v) => v.lod2_terrain_intersection_curve(),
            Self::AuxiliaryTrafficSpace(v) => v.lod2_terrain_intersection_curve(),
            Self::ClearanceSpace(v) => v.lod2_terrain_intersection_curve(),
            Self::Hole(v) => v.lod2_terrain_intersection_curve(),
            Self::Intersection(v) => v.lod2_terrain_intersection_curve(),
            Self::Railway(v) => v.lod2_terrain_intersection_curve(),
            Self::Road(v) => v.lod2_terrain_intersection_curve(),
            Self::Section(v) => v.lod2_terrain_intersection_curve(),
            Self::Square(v) => v.lod2_terrain_intersection_curve(),
            Self::Track(v) => v.lod2_terrain_intersection_curve(),
            Self::TrafficSpace(v) => v.lod2_terrain_intersection_curve(),
            Self::Waterway(v) => v.lod2_terrain_intersection_curve(),
            Self::HollowSpace(v) => v.lod2_terrain_intersection_curve(),
        }
    }
}
impl AbstractUnoccupiedSpaceTrait for AbstractUnoccupiedSpace {}
impl From<BridgeRoom> for AbstractUnoccupiedSpace {
    fn from(v: BridgeRoom) -> Self {
        Self::BridgeRoom(Box::new(v))
    }
}
impl From<BuildingRoom> for AbstractUnoccupiedSpace {
    fn from(v: BuildingRoom) -> Self {
        Self::BuildingRoom(Box::new(v))
    }
}
impl From<GenericUnoccupiedSpace> for AbstractUnoccupiedSpace {
    fn from(v: GenericUnoccupiedSpace) -> Self {
        Self::GenericUnoccupiedSpace(Box::new(v))
    }
}
impl From<AuxiliaryTrafficSpace> for AbstractUnoccupiedSpace {
    fn from(v: AuxiliaryTrafficSpace) -> Self {
        Self::AuxiliaryTrafficSpace(Box::new(v))
    }
}
impl From<ClearanceSpace> for AbstractUnoccupiedSpace {
    fn from(v: ClearanceSpace) -> Self {
        Self::ClearanceSpace(Box::new(v))
    }
}
impl From<Hole> for AbstractUnoccupiedSpace {
    fn from(v: Hole) -> Self {
        Self::Hole(Box::new(v))
    }
}
impl From<Intersection> for AbstractUnoccupiedSpace {
    fn from(v: Intersection) -> Self {
        Self::Intersection(Box::new(v))
    }
}
impl From<Railway> for AbstractUnoccupiedSpace {
    fn from(v: Railway) -> Self {
        Self::Railway(Box::new(v))
    }
}
impl From<Road> for AbstractUnoccupiedSpace {
    fn from(v: Road) -> Self {
        Self::Road(Box::new(v))
    }
}
impl From<Section> for AbstractUnoccupiedSpace {
    fn from(v: Section) -> Self {
        Self::Section(Box::new(v))
    }
}
impl From<Square> for AbstractUnoccupiedSpace {
    fn from(v: Square) -> Self {
        Self::Square(Box::new(v))
    }
}
impl From<Track> for AbstractUnoccupiedSpace {
    fn from(v: Track) -> Self {
        Self::Track(Box::new(v))
    }
}
impl From<TrafficSpace> for AbstractUnoccupiedSpace {
    fn from(v: TrafficSpace) -> Self {
        Self::TrafficSpace(Box::new(v))
    }
}
impl From<Waterway> for AbstractUnoccupiedSpace {
    fn from(v: Waterway) -> Self {
        Self::Waterway(Box::new(v))
    }
}
impl From<HollowSpace> for AbstractUnoccupiedSpace {
    fn from(v: HollowSpace) -> Self {
        Self::HollowSpace(Box::new(v))
    }
}
pub trait AbstractUnoccupiedSpaceAccessors {
    fn bridge_rooms(&self) -> impl Iterator<Item = &BridgeRoom>;
    fn building_rooms(&self) -> impl Iterator<Item = &BuildingRoom>;
    fn generic_unoccupied_spaces(&self) -> impl Iterator<Item = &GenericUnoccupiedSpace>;
    fn auxiliary_traffic_spaces(&self) -> impl Iterator<Item = &AuxiliaryTrafficSpace>;
    fn clearance_spaces(&self) -> impl Iterator<Item = &ClearanceSpace>;
    fn holes(&self) -> impl Iterator<Item = &Hole>;
    fn intersections(&self) -> impl Iterator<Item = &Intersection>;
    fn railways(&self) -> impl Iterator<Item = &Railway>;
    fn roads(&self) -> impl Iterator<Item = &Road>;
    fn sections(&self) -> impl Iterator<Item = &Section>;
    fn squares(&self) -> impl Iterator<Item = &Square>;
    fn tracks(&self) -> impl Iterator<Item = &Track>;
    fn traffic_spaces(&self) -> impl Iterator<Item = &TrafficSpace>;
    fn waterways(&self) -> impl Iterator<Item = &Waterway>;
    fn hollow_spaces(&self) -> impl Iterator<Item = &HollowSpace>;
}
impl AbstractUnoccupiedSpaceAccessors for [AbstractUnoccupiedSpace] {
    fn bridge_rooms(&self) -> impl Iterator<Item = &BridgeRoom> {
        self.iter()
            .filter_map(|item| match item {
                AbstractUnoccupiedSpace::BridgeRoom(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn building_rooms(&self) -> impl Iterator<Item = &BuildingRoom> {
        self.iter()
            .filter_map(|item| match item {
                AbstractUnoccupiedSpace::BuildingRoom(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn generic_unoccupied_spaces(
        &self,
    ) -> impl Iterator<Item = &GenericUnoccupiedSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractUnoccupiedSpace::GenericUnoccupiedSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn auxiliary_traffic_spaces(&self) -> impl Iterator<Item = &AuxiliaryTrafficSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractUnoccupiedSpace::AuxiliaryTrafficSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn clearance_spaces(&self) -> impl Iterator<Item = &ClearanceSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractUnoccupiedSpace::ClearanceSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn holes(&self) -> impl Iterator<Item = &Hole> {
        self.iter()
            .filter_map(|item| match item {
                AbstractUnoccupiedSpace::Hole(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn intersections(&self) -> impl Iterator<Item = &Intersection> {
        self.iter()
            .filter_map(|item| match item {
                AbstractUnoccupiedSpace::Intersection(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn railways(&self) -> impl Iterator<Item = &Railway> {
        self.iter()
            .filter_map(|item| match item {
                AbstractUnoccupiedSpace::Railway(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn roads(&self) -> impl Iterator<Item = &Road> {
        self.iter()
            .filter_map(|item| match item {
                AbstractUnoccupiedSpace::Road(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn sections(&self) -> impl Iterator<Item = &Section> {
        self.iter()
            .filter_map(|item| match item {
                AbstractUnoccupiedSpace::Section(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn squares(&self) -> impl Iterator<Item = &Square> {
        self.iter()
            .filter_map(|item| match item {
                AbstractUnoccupiedSpace::Square(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn tracks(&self) -> impl Iterator<Item = &Track> {
        self.iter()
            .filter_map(|item| match item {
                AbstractUnoccupiedSpace::Track(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn traffic_spaces(&self) -> impl Iterator<Item = &TrafficSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractUnoccupiedSpace::TrafficSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn waterways(&self) -> impl Iterator<Item = &Waterway> {
        self.iter()
            .filter_map(|item| match item {
                AbstractUnoccupiedSpace::Waterway(v) => Some(v.as_ref()),
                _ => None,
            })
    }
    fn hollow_spaces(&self) -> impl Iterator<Item = &HollowSpace> {
        self.iter()
            .filter_map(|item| match item {
                AbstractUnoccupiedSpace::HollowSpace(v) => Some(v.as_ref()),
                _ => None,
            })
    }
}
#[derive(Debug, Clone, Default)]
pub struct ClosureSurface {
    pub feature_id: ID,
    pub identifier: Option<String>,
    pub name: Vec<String>,
    pub description: Option<String>,
    pub creation_date: Option<String>,
    pub termination_date: Option<String>,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    pub relative_to_terrain: Option<RelativeToTerrain>,
    pub relative_to_water: Option<RelativeToWater>,
    pub appearance: Vec<AbstractAppearance>,
    pub generalizes_to: Vec<AbstractCityObject>,
    pub external_reference: Vec<ExternalReference>,
    pub related_to: Vec<AbstractCityObject>,
    pub dynamizer: Vec<AbstractDynamizer>,
    pub area: Vec<QualifiedArea>,
    pub lod3_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod2_multi_surface: Option<crate::geometry::MultiSurface>,
    pub point_cloud: Option<AbstractPointCloud>,
    pub lod0_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod0_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod1_multi_surface: Option<crate::geometry::MultiSurface>,
}
impl AbstractFeatureTrait for ClosureSurface {
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
}
impl AbstractFeatureWithLifespanTrait for ClosureSurface {
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
}
impl AbstractCityObjectTrait for ClosureSurface {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        self.relative_to_terrain
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        self.relative_to_water
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        &self.appearance
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        &self.generalizes_to
    }
    fn external_reference(&self) -> &[ExternalReference] {
        &self.external_reference
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        &self.related_to
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        &self.dynamizer
    }
}
impl AbstractSpaceBoundaryTrait for ClosureSurface {}
impl AbstractThematicSurfaceTrait for ClosureSurface {
    fn area(&self) -> &[QualifiedArea] {
        &self.area
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod3_multi_surface.as_ref()
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod2_multi_surface.as_ref()
    }
    fn point_cloud(&self) -> Option<&AbstractPointCloud> {
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
        let mut creation_date = None;
        let mut termination_date = None;
        let mut valid_from = None;
        let mut valid_to = None;
        let mut relative_to_terrain = None;
        let mut relative_to_water = None;
        let mut appearance = Vec::new();
        let mut generalizes_to = Vec::new();
        let mut external_reference = Vec::new();
        let mut related_to = Vec::new();
        let mut dynamizer = Vec::new();
        let mut area = Vec::new();
        let mut lod3_multi_surface = None;
        let mut lod2_multi_surface = None;
        let mut point_cloud = None;
        let mut lod0_multi_curve = None;
        let mut lod0_multi_surface = None;
        let mut lod1_multi_surface = None;
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
                (crate::namespace::NS_CORE, "appearance") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        appearance
                            .push(
                                super::dispatchers::parse_abstract_appearance(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_CORE, "generalizesTo") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        generalizes_to
                            .push(
                                super::dispatchers::parse_abstract_city_object(
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
                                super::dispatchers::parse_abstract_city_object(
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
                                super::dispatchers::parse_abstract_dynamizer(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_CORE, "area") => {
                    area.push(QualifiedArea::from_gml(&mut sub)?);
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
                            super::dispatchers::parse_abstract_point_cloud(
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
            creation_date,
            termination_date,
            valid_from,
            valid_to,
            relative_to_terrain,
            relative_to_water,
            appearance,
            generalizes_to,
            external_reference,
            related_to,
            dynamizer,
            area,
            lod3_multi_surface,
            lod2_multi_surface,
            point_cloud,
            lod0_multi_curve,
            lod0_multi_surface,
            lod1_multi_surface,
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
