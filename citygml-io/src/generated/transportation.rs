#![allow(unused_imports, unused_mut, unused_variables)]
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GranularityValue {
    #[default]
    Lane,
    Way,
}
impl GranularityValue {
    pub fn from_gml_text(text: &str) -> Result<Self, crate::error::ReaderError> {
        match text.trim() {
            "lane" => Ok(GranularityValue::Lane),
            "way" => Ok(GranularityValue::Way),
            other => {
                Err(crate::error::ReaderError::Parse {
                    message: format!(
                        "Unknown {} value: '{}'", stringify!(GranularityValue), other,
                    ),
                })
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TrafficDirectionValue {
    #[default]
    Forwards,
    Backwards,
    Both,
}
impl TrafficDirectionValue {
    pub fn from_gml_text(text: &str) -> Result<Self, crate::error::ReaderError> {
        match text.trim() {
            "forwards" => Ok(TrafficDirectionValue::Forwards),
            "backwards" => Ok(TrafficDirectionValue::Backwards),
            "both" => Ok(TrafficDirectionValue::Both),
            other => {
                Err(crate::error::ReaderError::Parse {
                    message: format!(
                        "Unknown {} value: '{}'", stringify!(TrafficDirectionValue),
                        other,
                    ),
                })
            }
        }
    }
}
pub trait ADEOfAbstractTransportationSpace: std::fmt::Debug {}
pub trait ADEOfAuxiliaryTrafficArea: std::fmt::Debug {}
pub trait ADEOfAuxiliaryTrafficSpace: std::fmt::Debug {}
pub trait ADEOfClearanceSpace: std::fmt::Debug {}
pub trait ADEOfHole: std::fmt::Debug {}
pub trait ADEOfHoleSurface: std::fmt::Debug {}
pub trait ADEOfIntersection: std::fmt::Debug {}
pub trait ADEOfMarking: std::fmt::Debug {}
pub trait ADEOfRailway: std::fmt::Debug {}
pub trait ADEOfRoad: std::fmt::Debug {}
pub trait ADEOfSection: std::fmt::Debug {}
pub trait ADEOfSquare: std::fmt::Debug {}
pub trait ADEOfTrack: std::fmt::Debug {}
pub trait ADEOfTrafficArea: std::fmt::Debug {}
pub trait ADEOfTrafficSpace: std::fmt::Debug {}
pub trait ADEOfWaterway: std::fmt::Debug {}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AuxiliaryTrafficAreaClassValue(pub String);
impl crate::from_gml::FromGml for AuxiliaryTrafficAreaClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(AuxiliaryTrafficAreaClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AuxiliaryTrafficAreaFunctionValue(pub String);
impl crate::from_gml::FromGml for AuxiliaryTrafficAreaFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(AuxiliaryTrafficAreaFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AuxiliaryTrafficAreaUsageValue(pub String);
impl crate::from_gml::FromGml for AuxiliaryTrafficAreaUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(AuxiliaryTrafficAreaUsageValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AuxiliaryTrafficSpaceClassValue(pub String);
impl crate::from_gml::FromGml for AuxiliaryTrafficSpaceClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(AuxiliaryTrafficSpaceClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AuxiliaryTrafficSpaceFunctionValue(pub String);
impl crate::from_gml::FromGml for AuxiliaryTrafficSpaceFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(AuxiliaryTrafficSpaceFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AuxiliaryTrafficSpaceUsageValue(pub String);
impl crate::from_gml::FromGml for AuxiliaryTrafficSpaceUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(AuxiliaryTrafficSpaceUsageValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ClearanceSpaceClassValue(pub String);
impl crate::from_gml::FromGml for ClearanceSpaceClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(ClearanceSpaceClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct HoleClassValue(pub String);
impl crate::from_gml::FromGml for HoleClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(HoleClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct IntersectionClassValue(pub String);
impl crate::from_gml::FromGml for IntersectionClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(IntersectionClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct MarkingClassValue(pub String);
impl crate::from_gml::FromGml for MarkingClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(MarkingClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct RailwayClassValue(pub String);
impl crate::from_gml::FromGml for RailwayClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(RailwayClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct RailwayFunctionValue(pub String);
impl crate::from_gml::FromGml for RailwayFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(RailwayFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct RailwayUsageValue(pub String);
impl crate::from_gml::FromGml for RailwayUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(RailwayUsageValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct RoadClassValue(pub String);
impl crate::from_gml::FromGml for RoadClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(RoadClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct RoadFunctionValue(pub String);
impl crate::from_gml::FromGml for RoadFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(RoadFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct RoadUsageValue(pub String);
impl crate::from_gml::FromGml for RoadUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(RoadUsageValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SectionClassValue(pub String);
impl crate::from_gml::FromGml for SectionClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(SectionClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SquareClassValue(pub String);
impl crate::from_gml::FromGml for SquareClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(SquareClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SquareFunctionValue(pub String);
impl crate::from_gml::FromGml for SquareFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(SquareFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SquareUsageValue(pub String);
impl crate::from_gml::FromGml for SquareUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(SquareUsageValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SurfaceMaterialValue(pub String);
impl crate::from_gml::FromGml for SurfaceMaterialValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(SurfaceMaterialValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TrackClassValue(pub String);
impl crate::from_gml::FromGml for TrackClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TrackClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TrackFunctionValue(pub String);
impl crate::from_gml::FromGml for TrackFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TrackFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TrackUsageValue(pub String);
impl crate::from_gml::FromGml for TrackUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TrackUsageValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TrafficAreaClassValue(pub String);
impl crate::from_gml::FromGml for TrafficAreaClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TrafficAreaClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TrafficAreaFunctionValue(pub String);
impl crate::from_gml::FromGml for TrafficAreaFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TrafficAreaFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TrafficAreaUsageValue(pub String);
impl crate::from_gml::FromGml for TrafficAreaUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TrafficAreaUsageValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TrafficSpaceClassValue(pub String);
impl crate::from_gml::FromGml for TrafficSpaceClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TrafficSpaceClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TrafficSpaceFunctionValue(pub String);
impl crate::from_gml::FromGml for TrafficSpaceFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TrafficSpaceFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TrafficSpaceUsageValue(pub String);
impl crate::from_gml::FromGml for TrafficSpaceUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TrafficSpaceUsageValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct WaterwayClassValue(pub String);
impl crate::from_gml::FromGml for WaterwayClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(WaterwayClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct WaterwayFunctionValue(pub String);
impl crate::from_gml::FromGml for WaterwayFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(WaterwayFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct WaterwayUsageValue(pub String);
impl crate::from_gml::FromGml for WaterwayUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(WaterwayUsageValue(reader.read_text()?))
    }
}
#[derive(Debug, Default)]
pub struct AuxiliaryTrafficArea {
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
    pub class_: Option<AuxiliaryTrafficAreaClassValue>,
    pub function: Vec<AuxiliaryTrafficAreaFunctionValue>,
    pub usage: Vec<AuxiliaryTrafficAreaUsageValue>,
    pub surface_material: Option<SurfaceMaterialValue>,
    pub ade_of_auxiliary_traffic_area: Vec<Box<dyn ADEOfAuxiliaryTrafficArea>>,
}
impl AbstractFeature for AuxiliaryTrafficArea {
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
impl AbstractFeatureWithLifespan for AuxiliaryTrafficArea {
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
impl AbstractCityObject for AuxiliaryTrafficArea {
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
impl AbstractSpaceBoundary for AuxiliaryTrafficArea {
    fn ade_of_abstract_space_boundary(&self) -> &[Box<dyn ADEOfAbstractSpaceBoundary>] {
        &self.ade_of_abstract_space_boundary
    }
}
impl AbstractThematicSurface for AuxiliaryTrafficArea {
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
impl AuxiliaryTrafficArea {
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
        let mut class_ = None;
        let mut function = Vec::new();
        let mut usage = Vec::new();
        let mut surface_material = None;
        let mut ade_of_auxiliary_traffic_area = Vec::new();
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
                (crate::namespace::NS_TRANSPORTATION, "class") => {
                    class_ = Some(AuxiliaryTrafficAreaClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "function") => {
                    function.push(AuxiliaryTrafficAreaFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "usage") => {
                    usage.push(AuxiliaryTrafficAreaUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "surfaceMaterial") => {
                    surface_material = Some(SurfaceMaterialValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "adeOfAuxiliaryTrafficArea") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(AuxiliaryTrafficArea {
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
            class_,
            function,
            usage,
            surface_material,
            ade_of_auxiliary_traffic_area,
        })
    }
}
impl crate::from_gml::FromGml for AuxiliaryTrafficArea {
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
#[derive(Debug, Default)]
pub struct HoleSurface {
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
    pub ade_of_hole_surface: Vec<Box<dyn ADEOfHoleSurface>>,
}
impl AbstractFeature for HoleSurface {
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
impl AbstractFeatureWithLifespan for HoleSurface {
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
impl AbstractCityObject for HoleSurface {
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
impl AbstractSpaceBoundary for HoleSurface {
    fn ade_of_abstract_space_boundary(&self) -> &[Box<dyn ADEOfAbstractSpaceBoundary>] {
        &self.ade_of_abstract_space_boundary
    }
}
impl AbstractThematicSurface for HoleSurface {
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
impl HoleSurface {
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
        let mut ade_of_hole_surface = Vec::new();
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
                (crate::namespace::NS_TRANSPORTATION, "adeOfHoleSurface") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(HoleSurface {
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
            ade_of_hole_surface,
        })
    }
}
impl crate::from_gml::FromGml for HoleSurface {
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
#[derive(Debug, Default)]
pub struct Marking {
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
    pub class_: Option<MarkingClassValue>,
    pub ade_of_marking: Vec<Box<dyn ADEOfMarking>>,
}
impl AbstractFeature for Marking {
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
impl AbstractFeatureWithLifespan for Marking {
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
impl AbstractCityObject for Marking {
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
impl AbstractSpaceBoundary for Marking {
    fn ade_of_abstract_space_boundary(&self) -> &[Box<dyn ADEOfAbstractSpaceBoundary>] {
        &self.ade_of_abstract_space_boundary
    }
}
impl AbstractThematicSurface for Marking {
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
impl Marking {
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
        let mut class_ = None;
        let mut ade_of_marking = Vec::new();
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
                (crate::namespace::NS_TRANSPORTATION, "class") => {
                    class_ = Some(MarkingClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "adeOfMarking") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Marking {
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
            class_,
            ade_of_marking,
        })
    }
}
impl crate::from_gml::FromGml for Marking {
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
#[derive(Debug, Default)]
pub struct TrafficArea {
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
    pub class_: Option<TrafficAreaClassValue>,
    pub function: Vec<TrafficAreaFunctionValue>,
    pub usage: Vec<TrafficAreaUsageValue>,
    pub surface_material: Option<SurfaceMaterialValue>,
    pub ade_of_traffic_area: Vec<Box<dyn ADEOfTrafficArea>>,
}
impl AbstractFeature for TrafficArea {
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
impl AbstractFeatureWithLifespan for TrafficArea {
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
impl AbstractCityObject for TrafficArea {
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
impl AbstractSpaceBoundary for TrafficArea {
    fn ade_of_abstract_space_boundary(&self) -> &[Box<dyn ADEOfAbstractSpaceBoundary>] {
        &self.ade_of_abstract_space_boundary
    }
}
impl AbstractThematicSurface for TrafficArea {
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
impl TrafficArea {
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
        let mut class_ = None;
        let mut function = Vec::new();
        let mut usage = Vec::new();
        let mut surface_material = None;
        let mut ade_of_traffic_area = Vec::new();
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
                (crate::namespace::NS_TRANSPORTATION, "class") => {
                    class_ = Some(TrafficAreaClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "function") => {
                    function.push(TrafficAreaFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "usage") => {
                    usage.push(TrafficAreaUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "surfaceMaterial") => {
                    surface_material = Some(SurfaceMaterialValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "adeOfTrafficArea") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(TrafficArea {
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
            class_,
            function,
            usage,
            surface_material,
            ade_of_traffic_area,
        })
    }
}
impl crate::from_gml::FromGml for TrafficArea {
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
pub trait AbstractTransportationSpace: AbstractUnoccupiedSpace {
    fn traffic_direction(&self) -> Option<TrafficDirectionValue>;
    fn occupancy(&self) -> &[Occupancy];
    fn ade_of_abstract_transportation_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractTransportationSpace>];
    fn auxiliary_traffic_space(&self) -> &[AuxiliaryTrafficSpace];
    fn hole(&self) -> &[Hole];
    fn traffic_space(&self) -> &[TrafficSpace];
    fn marking(&self) -> &[Marking];
}
#[derive(Debug, Default)]
pub struct AuxiliaryTrafficSpace {
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
    pub space_type: Option<SpaceType>,
    pub volume: Vec<QualifiedVolume>,
    pub area: Vec<QualifiedArea>,
    pub ade_of_abstract_space: Vec<Box<dyn ADEOfAbstractSpace>>,
    pub lod2_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod3_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod0_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod1_solid: Option<crate::geometry::Solid>,
    pub lod3_solid: Option<crate::geometry::Solid>,
    pub boundary: Vec<Box<dyn AbstractSpaceBoundary>>,
    pub lod0_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_solid: Option<crate::geometry::Solid>,
    pub lod0_point: Option<crate::geometry::DirectPosition>,
    pub lod3_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_multi_surface: Option<crate::geometry::MultiSurface>,
    pub ade_of_abstract_physical_space: Vec<Box<dyn ADEOfAbstractPhysicalSpace>>,
    pub lod3_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub point_cloud: Option<Box<dyn AbstractPointCloud>>,
    pub lod1_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub ade_of_abstract_unoccupied_space: Vec<Box<dyn ADEOfAbstractUnoccupiedSpace>>,
    pub class_: Option<AuxiliaryTrafficSpaceClassValue>,
    pub function: Vec<AuxiliaryTrafficSpaceFunctionValue>,
    pub usage: Vec<AuxiliaryTrafficSpaceUsageValue>,
    pub granularity: GranularityValue,
    pub ade_of_auxiliary_traffic_space: Vec<Box<dyn ADEOfAuxiliaryTrafficSpace>>,
}
impl AbstractFeature for AuxiliaryTrafficSpace {
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
impl AbstractFeatureWithLifespan for AuxiliaryTrafficSpace {
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
impl AbstractCityObject for AuxiliaryTrafficSpace {
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
impl AbstractSpace for AuxiliaryTrafficSpace {
    fn space_type(&self) -> Option<SpaceType> {
        self.space_type
    }
    fn volume(&self) -> &[QualifiedVolume] {
        &self.volume
    }
    fn area(&self) -> &[QualifiedArea] {
        &self.area
    }
    fn ade_of_abstract_space(&self) -> &[Box<dyn ADEOfAbstractSpace>] {
        &self.ade_of_abstract_space
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_multi_curve.as_ref()
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod3_multi_surface.as_ref()
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod0_multi_surface.as_ref()
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod1_solid.as_ref()
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod3_solid.as_ref()
    }
    fn boundary(&self) -> &[Box<dyn AbstractSpaceBoundary>] {
        &self.boundary
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod0_multi_curve.as_ref()
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod2_solid.as_ref()
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        self.lod0_point.as_ref()
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_multi_curve.as_ref()
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod2_multi_surface.as_ref()
    }
}
impl AbstractPhysicalSpace for AuxiliaryTrafficSpace {
    fn ade_of_abstract_physical_space(&self) -> &[Box<dyn ADEOfAbstractPhysicalSpace>] {
        &self.ade_of_abstract_physical_space
    }
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_terrain_intersection_curve.as_ref()
    }
    fn point_cloud(&self) -> Option<&Box<dyn AbstractPointCloud>> {
        self.point_cloud.as_ref()
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod1_terrain_intersection_curve.as_ref()
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_terrain_intersection_curve.as_ref()
    }
}
impl AbstractUnoccupiedSpace for AuxiliaryTrafficSpace {
    fn ade_of_abstract_unoccupied_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractUnoccupiedSpace>] {
        &self.ade_of_abstract_unoccupied_space
    }
}
impl AuxiliaryTrafficSpace {
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
        let mut space_type = None;
        let mut volume = Vec::new();
        let mut area = Vec::new();
        let mut ade_of_abstract_space = Vec::new();
        let mut lod2_multi_curve = None;
        let mut lod3_multi_surface = None;
        let mut lod0_multi_surface = None;
        let mut lod1_solid = None;
        let mut lod3_solid = None;
        let mut boundary = Vec::new();
        let mut lod0_multi_curve = None;
        let mut lod2_solid = None;
        let mut lod0_point = None;
        let mut lod3_multi_curve = None;
        let mut lod2_multi_surface = None;
        let mut ade_of_abstract_physical_space = Vec::new();
        let mut lod3_terrain_intersection_curve = None;
        let mut point_cloud = None;
        let mut lod1_terrain_intersection_curve = None;
        let mut lod2_terrain_intersection_curve = None;
        let mut ade_of_abstract_unoccupied_space = Vec::new();
        let mut class_ = None;
        let mut function = Vec::new();
        let mut usage = Vec::new();
        let mut granularity = Default::default();
        let mut ade_of_auxiliary_traffic_space = Vec::new();
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
                (crate::namespace::NS_CORE, "spaceType") => {
                    space_type = Some(SpaceType::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_CORE, "volume") => {
                    volume.push(QualifiedVolume::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "area") => {
                    area.push(QualifiedArea::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "adeOfAbstractSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod2MultiCurve") => {
                    lod2_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1Solid") => {
                    lod1_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3Solid") => {
                    lod3_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "boundary") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        boundary
                            .push(
                                super::dispatchers::parse_dyn_abstract_space_boundary(
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
                (crate::namespace::NS_CORE, "lod2Solid") => {
                    lod2_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod0Point") => {
                    lod0_point = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3MultiCurve") => {
                    lod3_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractPhysicalSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod3TerrainIntersectionCurve") => {
                    lod3_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1TerrainIntersectionCurve") => {
                    lod1_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod2TerrainIntersectionCurve") => {
                    lod2_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractUnoccupiedSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "class") => {
                    class_ = Some(AuxiliaryTrafficSpaceClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "function") => {
                    function.push(AuxiliaryTrafficSpaceFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "usage") => {
                    usage.push(AuxiliaryTrafficSpaceUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "granularity") => {
                    granularity = GranularityValue::from_gml_text(&sub.read_text()?)?;
                }
                (crate::namespace::NS_TRANSPORTATION, "adeOfAuxiliaryTrafficSpace") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(AuxiliaryTrafficSpace {
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
            space_type,
            volume,
            area,
            ade_of_abstract_space,
            lod2_multi_curve,
            lod3_multi_surface,
            lod0_multi_surface,
            lod1_solid,
            lod3_solid,
            boundary,
            lod0_multi_curve,
            lod2_solid,
            lod0_point,
            lod3_multi_curve,
            lod2_multi_surface,
            ade_of_abstract_physical_space,
            lod3_terrain_intersection_curve,
            point_cloud,
            lod1_terrain_intersection_curve,
            lod2_terrain_intersection_curve,
            ade_of_abstract_unoccupied_space,
            class_,
            function,
            usage,
            granularity,
            ade_of_auxiliary_traffic_space,
        })
    }
}
impl crate::from_gml::FromGml for AuxiliaryTrafficSpace {
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
#[derive(Debug, Default)]
pub struct ClearanceSpace {
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
    pub space_type: Option<SpaceType>,
    pub volume: Vec<QualifiedVolume>,
    pub area: Vec<QualifiedArea>,
    pub ade_of_abstract_space: Vec<Box<dyn ADEOfAbstractSpace>>,
    pub lod2_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod3_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod0_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod1_solid: Option<crate::geometry::Solid>,
    pub lod3_solid: Option<crate::geometry::Solid>,
    pub boundary: Vec<Box<dyn AbstractSpaceBoundary>>,
    pub lod0_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_solid: Option<crate::geometry::Solid>,
    pub lod0_point: Option<crate::geometry::DirectPosition>,
    pub lod3_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_multi_surface: Option<crate::geometry::MultiSurface>,
    pub ade_of_abstract_physical_space: Vec<Box<dyn ADEOfAbstractPhysicalSpace>>,
    pub lod3_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub point_cloud: Option<Box<dyn AbstractPointCloud>>,
    pub lod1_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub ade_of_abstract_unoccupied_space: Vec<Box<dyn ADEOfAbstractUnoccupiedSpace>>,
    pub class_: Vec<ClearanceSpaceClassValue>,
    pub ade_of_clearance_space: Vec<Box<dyn ADEOfClearanceSpace>>,
}
impl AbstractFeature for ClearanceSpace {
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
impl AbstractFeatureWithLifespan for ClearanceSpace {
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
impl AbstractCityObject for ClearanceSpace {
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
impl AbstractSpace for ClearanceSpace {
    fn space_type(&self) -> Option<SpaceType> {
        self.space_type
    }
    fn volume(&self) -> &[QualifiedVolume] {
        &self.volume
    }
    fn area(&self) -> &[QualifiedArea] {
        &self.area
    }
    fn ade_of_abstract_space(&self) -> &[Box<dyn ADEOfAbstractSpace>] {
        &self.ade_of_abstract_space
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_multi_curve.as_ref()
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod3_multi_surface.as_ref()
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod0_multi_surface.as_ref()
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod1_solid.as_ref()
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod3_solid.as_ref()
    }
    fn boundary(&self) -> &[Box<dyn AbstractSpaceBoundary>] {
        &self.boundary
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod0_multi_curve.as_ref()
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod2_solid.as_ref()
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        self.lod0_point.as_ref()
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_multi_curve.as_ref()
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod2_multi_surface.as_ref()
    }
}
impl AbstractPhysicalSpace for ClearanceSpace {
    fn ade_of_abstract_physical_space(&self) -> &[Box<dyn ADEOfAbstractPhysicalSpace>] {
        &self.ade_of_abstract_physical_space
    }
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_terrain_intersection_curve.as_ref()
    }
    fn point_cloud(&self) -> Option<&Box<dyn AbstractPointCloud>> {
        self.point_cloud.as_ref()
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod1_terrain_intersection_curve.as_ref()
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_terrain_intersection_curve.as_ref()
    }
}
impl AbstractUnoccupiedSpace for ClearanceSpace {
    fn ade_of_abstract_unoccupied_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractUnoccupiedSpace>] {
        &self.ade_of_abstract_unoccupied_space
    }
}
impl ClearanceSpace {
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
        let mut space_type = None;
        let mut volume = Vec::new();
        let mut area = Vec::new();
        let mut ade_of_abstract_space = Vec::new();
        let mut lod2_multi_curve = None;
        let mut lod3_multi_surface = None;
        let mut lod0_multi_surface = None;
        let mut lod1_solid = None;
        let mut lod3_solid = None;
        let mut boundary = Vec::new();
        let mut lod0_multi_curve = None;
        let mut lod2_solid = None;
        let mut lod0_point = None;
        let mut lod3_multi_curve = None;
        let mut lod2_multi_surface = None;
        let mut ade_of_abstract_physical_space = Vec::new();
        let mut lod3_terrain_intersection_curve = None;
        let mut point_cloud = None;
        let mut lod1_terrain_intersection_curve = None;
        let mut lod2_terrain_intersection_curve = None;
        let mut ade_of_abstract_unoccupied_space = Vec::new();
        let mut class_ = Vec::new();
        let mut ade_of_clearance_space = Vec::new();
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
                (crate::namespace::NS_CORE, "spaceType") => {
                    space_type = Some(SpaceType::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_CORE, "volume") => {
                    volume.push(QualifiedVolume::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "area") => {
                    area.push(QualifiedArea::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "adeOfAbstractSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod2MultiCurve") => {
                    lod2_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1Solid") => {
                    lod1_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3Solid") => {
                    lod3_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "boundary") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        boundary
                            .push(
                                super::dispatchers::parse_dyn_abstract_space_boundary(
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
                (crate::namespace::NS_CORE, "lod2Solid") => {
                    lod2_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod0Point") => {
                    lod0_point = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3MultiCurve") => {
                    lod3_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractPhysicalSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod3TerrainIntersectionCurve") => {
                    lod3_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1TerrainIntersectionCurve") => {
                    lod1_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod2TerrainIntersectionCurve") => {
                    lod2_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractUnoccupiedSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "class") => {
                    class_.push(ClearanceSpaceClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "adeOfClearanceSpace") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(ClearanceSpace {
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
            space_type,
            volume,
            area,
            ade_of_abstract_space,
            lod2_multi_curve,
            lod3_multi_surface,
            lod0_multi_surface,
            lod1_solid,
            lod3_solid,
            boundary,
            lod0_multi_curve,
            lod2_solid,
            lod0_point,
            lod3_multi_curve,
            lod2_multi_surface,
            ade_of_abstract_physical_space,
            lod3_terrain_intersection_curve,
            point_cloud,
            lod1_terrain_intersection_curve,
            lod2_terrain_intersection_curve,
            ade_of_abstract_unoccupied_space,
            class_,
            ade_of_clearance_space,
        })
    }
}
impl crate::from_gml::FromGml for ClearanceSpace {
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
#[derive(Debug, Default)]
pub struct Hole {
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
    pub space_type: Option<SpaceType>,
    pub volume: Vec<QualifiedVolume>,
    pub area: Vec<QualifiedArea>,
    pub ade_of_abstract_space: Vec<Box<dyn ADEOfAbstractSpace>>,
    pub lod2_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod3_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod0_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod1_solid: Option<crate::geometry::Solid>,
    pub lod3_solid: Option<crate::geometry::Solid>,
    pub boundary: Vec<Box<dyn AbstractSpaceBoundary>>,
    pub lod0_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_solid: Option<crate::geometry::Solid>,
    pub lod0_point: Option<crate::geometry::DirectPosition>,
    pub lod3_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_multi_surface: Option<crate::geometry::MultiSurface>,
    pub ade_of_abstract_physical_space: Vec<Box<dyn ADEOfAbstractPhysicalSpace>>,
    pub lod3_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub point_cloud: Option<Box<dyn AbstractPointCloud>>,
    pub lod1_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub ade_of_abstract_unoccupied_space: Vec<Box<dyn ADEOfAbstractUnoccupiedSpace>>,
    pub class_: Option<HoleClassValue>,
    pub ade_of_hole: Vec<Box<dyn ADEOfHole>>,
}
impl AbstractFeature for Hole {
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
impl AbstractFeatureWithLifespan for Hole {
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
impl AbstractCityObject for Hole {
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
impl AbstractSpace for Hole {
    fn space_type(&self) -> Option<SpaceType> {
        self.space_type
    }
    fn volume(&self) -> &[QualifiedVolume] {
        &self.volume
    }
    fn area(&self) -> &[QualifiedArea] {
        &self.area
    }
    fn ade_of_abstract_space(&self) -> &[Box<dyn ADEOfAbstractSpace>] {
        &self.ade_of_abstract_space
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_multi_curve.as_ref()
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod3_multi_surface.as_ref()
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod0_multi_surface.as_ref()
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod1_solid.as_ref()
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod3_solid.as_ref()
    }
    fn boundary(&self) -> &[Box<dyn AbstractSpaceBoundary>] {
        &self.boundary
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod0_multi_curve.as_ref()
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod2_solid.as_ref()
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        self.lod0_point.as_ref()
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_multi_curve.as_ref()
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod2_multi_surface.as_ref()
    }
}
impl AbstractPhysicalSpace for Hole {
    fn ade_of_abstract_physical_space(&self) -> &[Box<dyn ADEOfAbstractPhysicalSpace>] {
        &self.ade_of_abstract_physical_space
    }
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_terrain_intersection_curve.as_ref()
    }
    fn point_cloud(&self) -> Option<&Box<dyn AbstractPointCloud>> {
        self.point_cloud.as_ref()
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod1_terrain_intersection_curve.as_ref()
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_terrain_intersection_curve.as_ref()
    }
}
impl AbstractUnoccupiedSpace for Hole {
    fn ade_of_abstract_unoccupied_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractUnoccupiedSpace>] {
        &self.ade_of_abstract_unoccupied_space
    }
}
impl Hole {
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
        let mut space_type = None;
        let mut volume = Vec::new();
        let mut area = Vec::new();
        let mut ade_of_abstract_space = Vec::new();
        let mut lod2_multi_curve = None;
        let mut lod3_multi_surface = None;
        let mut lod0_multi_surface = None;
        let mut lod1_solid = None;
        let mut lod3_solid = None;
        let mut boundary = Vec::new();
        let mut lod0_multi_curve = None;
        let mut lod2_solid = None;
        let mut lod0_point = None;
        let mut lod3_multi_curve = None;
        let mut lod2_multi_surface = None;
        let mut ade_of_abstract_physical_space = Vec::new();
        let mut lod3_terrain_intersection_curve = None;
        let mut point_cloud = None;
        let mut lod1_terrain_intersection_curve = None;
        let mut lod2_terrain_intersection_curve = None;
        let mut ade_of_abstract_unoccupied_space = Vec::new();
        let mut class_ = None;
        let mut ade_of_hole = Vec::new();
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
                (crate::namespace::NS_CORE, "spaceType") => {
                    space_type = Some(SpaceType::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_CORE, "volume") => {
                    volume.push(QualifiedVolume::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "area") => {
                    area.push(QualifiedArea::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "adeOfAbstractSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod2MultiCurve") => {
                    lod2_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1Solid") => {
                    lod1_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3Solid") => {
                    lod3_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "boundary") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        boundary
                            .push(
                                super::dispatchers::parse_dyn_abstract_space_boundary(
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
                (crate::namespace::NS_CORE, "lod2Solid") => {
                    lod2_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod0Point") => {
                    lod0_point = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3MultiCurve") => {
                    lod3_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractPhysicalSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod3TerrainIntersectionCurve") => {
                    lod3_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1TerrainIntersectionCurve") => {
                    lod1_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod2TerrainIntersectionCurve") => {
                    lod2_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractUnoccupiedSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "class") => {
                    class_ = Some(HoleClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "adeOfHole") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Hole {
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
            space_type,
            volume,
            area,
            ade_of_abstract_space,
            lod2_multi_curve,
            lod3_multi_surface,
            lod0_multi_surface,
            lod1_solid,
            lod3_solid,
            boundary,
            lod0_multi_curve,
            lod2_solid,
            lod0_point,
            lod3_multi_curve,
            lod2_multi_surface,
            ade_of_abstract_physical_space,
            lod3_terrain_intersection_curve,
            point_cloud,
            lod1_terrain_intersection_curve,
            lod2_terrain_intersection_curve,
            ade_of_abstract_unoccupied_space,
            class_,
            ade_of_hole,
        })
    }
}
impl crate::from_gml::FromGml for Hole {
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
#[derive(Debug, Default)]
pub struct TrafficSpace {
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
    pub space_type: Option<SpaceType>,
    pub volume: Vec<QualifiedVolume>,
    pub area: Vec<QualifiedArea>,
    pub ade_of_abstract_space: Vec<Box<dyn ADEOfAbstractSpace>>,
    pub lod2_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod3_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod0_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod1_solid: Option<crate::geometry::Solid>,
    pub lod3_solid: Option<crate::geometry::Solid>,
    pub boundary: Vec<Box<dyn AbstractSpaceBoundary>>,
    pub lod0_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_solid: Option<crate::geometry::Solid>,
    pub lod0_point: Option<crate::geometry::DirectPosition>,
    pub lod3_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_multi_surface: Option<crate::geometry::MultiSurface>,
    pub ade_of_abstract_physical_space: Vec<Box<dyn ADEOfAbstractPhysicalSpace>>,
    pub lod3_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub point_cloud: Option<Box<dyn AbstractPointCloud>>,
    pub lod1_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub ade_of_abstract_unoccupied_space: Vec<Box<dyn ADEOfAbstractUnoccupiedSpace>>,
    pub class_: Option<TrafficSpaceClassValue>,
    pub function: Vec<TrafficSpaceFunctionValue>,
    pub usage: Vec<TrafficSpaceUsageValue>,
    pub granularity: GranularityValue,
    pub traffic_direction: Option<TrafficDirectionValue>,
    pub occupancy: Vec<Occupancy>,
    pub ade_of_traffic_space: Vec<Box<dyn ADEOfTrafficSpace>>,
    pub predecessor: Vec<TrafficSpace>,
    pub clearance_space: Vec<ClearanceSpace>,
    pub successor: Vec<TrafficSpace>,
}
impl AbstractFeature for TrafficSpace {
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
impl AbstractFeatureWithLifespan for TrafficSpace {
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
impl AbstractCityObject for TrafficSpace {
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
impl AbstractSpace for TrafficSpace {
    fn space_type(&self) -> Option<SpaceType> {
        self.space_type
    }
    fn volume(&self) -> &[QualifiedVolume] {
        &self.volume
    }
    fn area(&self) -> &[QualifiedArea] {
        &self.area
    }
    fn ade_of_abstract_space(&self) -> &[Box<dyn ADEOfAbstractSpace>] {
        &self.ade_of_abstract_space
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_multi_curve.as_ref()
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod3_multi_surface.as_ref()
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod0_multi_surface.as_ref()
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod1_solid.as_ref()
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod3_solid.as_ref()
    }
    fn boundary(&self) -> &[Box<dyn AbstractSpaceBoundary>] {
        &self.boundary
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod0_multi_curve.as_ref()
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod2_solid.as_ref()
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        self.lod0_point.as_ref()
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_multi_curve.as_ref()
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod2_multi_surface.as_ref()
    }
}
impl AbstractPhysicalSpace for TrafficSpace {
    fn ade_of_abstract_physical_space(&self) -> &[Box<dyn ADEOfAbstractPhysicalSpace>] {
        &self.ade_of_abstract_physical_space
    }
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_terrain_intersection_curve.as_ref()
    }
    fn point_cloud(&self) -> Option<&Box<dyn AbstractPointCloud>> {
        self.point_cloud.as_ref()
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod1_terrain_intersection_curve.as_ref()
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_terrain_intersection_curve.as_ref()
    }
}
impl AbstractUnoccupiedSpace for TrafficSpace {
    fn ade_of_abstract_unoccupied_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractUnoccupiedSpace>] {
        &self.ade_of_abstract_unoccupied_space
    }
}
impl TrafficSpace {
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
        let mut space_type = None;
        let mut volume = Vec::new();
        let mut area = Vec::new();
        let mut ade_of_abstract_space = Vec::new();
        let mut lod2_multi_curve = None;
        let mut lod3_multi_surface = None;
        let mut lod0_multi_surface = None;
        let mut lod1_solid = None;
        let mut lod3_solid = None;
        let mut boundary = Vec::new();
        let mut lod0_multi_curve = None;
        let mut lod2_solid = None;
        let mut lod0_point = None;
        let mut lod3_multi_curve = None;
        let mut lod2_multi_surface = None;
        let mut ade_of_abstract_physical_space = Vec::new();
        let mut lod3_terrain_intersection_curve = None;
        let mut point_cloud = None;
        let mut lod1_terrain_intersection_curve = None;
        let mut lod2_terrain_intersection_curve = None;
        let mut ade_of_abstract_unoccupied_space = Vec::new();
        let mut class_ = None;
        let mut function = Vec::new();
        let mut usage = Vec::new();
        let mut granularity = Default::default();
        let mut traffic_direction = None;
        let mut occupancy = Vec::new();
        let mut ade_of_traffic_space = Vec::new();
        let mut predecessor = Vec::new();
        let mut clearance_space = Vec::new();
        let mut successor = Vec::new();
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
                (crate::namespace::NS_CORE, "spaceType") => {
                    space_type = Some(SpaceType::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_CORE, "volume") => {
                    volume.push(QualifiedVolume::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "area") => {
                    area.push(QualifiedArea::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "adeOfAbstractSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod2MultiCurve") => {
                    lod2_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1Solid") => {
                    lod1_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3Solid") => {
                    lod3_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "boundary") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        boundary
                            .push(
                                super::dispatchers::parse_dyn_abstract_space_boundary(
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
                (crate::namespace::NS_CORE, "lod2Solid") => {
                    lod2_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod0Point") => {
                    lod0_point = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3MultiCurve") => {
                    lod3_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractPhysicalSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod3TerrainIntersectionCurve") => {
                    lod3_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1TerrainIntersectionCurve") => {
                    lod1_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod2TerrainIntersectionCurve") => {
                    lod2_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractUnoccupiedSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "class") => {
                    class_ = Some(TrafficSpaceClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "function") => {
                    function.push(TrafficSpaceFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "usage") => {
                    usage.push(TrafficSpaceUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "granularity") => {
                    granularity = GranularityValue::from_gml_text(&sub.read_text()?)?;
                }
                (crate::namespace::NS_TRANSPORTATION, "trafficDirection") => {
                    traffic_direction = Some(
                        TrafficDirectionValue::from_gml_text(&sub.read_text()?)?,
                    );
                }
                (crate::namespace::NS_TRANSPORTATION, "occupancy") => {
                    occupancy.push(Occupancy::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_TRANSPORTATION, "adeOfTrafficSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "predecessor") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        predecessor
                            .push(
                                TrafficSpace::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "clearanceSpace") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        clearance_space
                            .push(
                                ClearanceSpace::from_gml_with_info(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "successor") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        successor
                            .push(
                                TrafficSpace::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(TrafficSpace {
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
            space_type,
            volume,
            area,
            ade_of_abstract_space,
            lod2_multi_curve,
            lod3_multi_surface,
            lod0_multi_surface,
            lod1_solid,
            lod3_solid,
            boundary,
            lod0_multi_curve,
            lod2_solid,
            lod0_point,
            lod3_multi_curve,
            lod2_multi_surface,
            ade_of_abstract_physical_space,
            lod3_terrain_intersection_curve,
            point_cloud,
            lod1_terrain_intersection_curve,
            lod2_terrain_intersection_curve,
            ade_of_abstract_unoccupied_space,
            class_,
            function,
            usage,
            granularity,
            traffic_direction,
            occupancy,
            ade_of_traffic_space,
            predecessor,
            clearance_space,
            successor,
        })
    }
}
impl crate::from_gml::FromGml for TrafficSpace {
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
#[derive(Debug, Default)]
pub struct Intersection {
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
    pub space_type: Option<SpaceType>,
    pub volume: Vec<QualifiedVolume>,
    pub area: Vec<QualifiedArea>,
    pub ade_of_abstract_space: Vec<Box<dyn ADEOfAbstractSpace>>,
    pub lod2_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod3_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod0_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod1_solid: Option<crate::geometry::Solid>,
    pub lod3_solid: Option<crate::geometry::Solid>,
    pub boundary: Vec<Box<dyn AbstractSpaceBoundary>>,
    pub lod0_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_solid: Option<crate::geometry::Solid>,
    pub lod0_point: Option<crate::geometry::DirectPosition>,
    pub lod3_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_multi_surface: Option<crate::geometry::MultiSurface>,
    pub ade_of_abstract_physical_space: Vec<Box<dyn ADEOfAbstractPhysicalSpace>>,
    pub lod3_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub point_cloud: Option<Box<dyn AbstractPointCloud>>,
    pub lod1_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub ade_of_abstract_unoccupied_space: Vec<Box<dyn ADEOfAbstractUnoccupiedSpace>>,
    pub traffic_direction: Option<TrafficDirectionValue>,
    pub occupancy: Vec<Occupancy>,
    pub ade_of_abstract_transportation_space: Vec<
        Box<dyn ADEOfAbstractTransportationSpace>,
    >,
    pub auxiliary_traffic_space: Vec<AuxiliaryTrafficSpace>,
    pub hole: Vec<Hole>,
    pub traffic_space: Vec<TrafficSpace>,
    pub marking: Vec<Marking>,
    pub class_: Option<IntersectionClassValue>,
    pub ade_of_intersection: Vec<Box<dyn ADEOfIntersection>>,
}
impl AbstractFeature for Intersection {
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
impl AbstractFeatureWithLifespan for Intersection {
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
impl AbstractCityObject for Intersection {
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
impl AbstractSpace for Intersection {
    fn space_type(&self) -> Option<SpaceType> {
        self.space_type
    }
    fn volume(&self) -> &[QualifiedVolume] {
        &self.volume
    }
    fn area(&self) -> &[QualifiedArea] {
        &self.area
    }
    fn ade_of_abstract_space(&self) -> &[Box<dyn ADEOfAbstractSpace>] {
        &self.ade_of_abstract_space
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_multi_curve.as_ref()
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod3_multi_surface.as_ref()
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod0_multi_surface.as_ref()
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod1_solid.as_ref()
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod3_solid.as_ref()
    }
    fn boundary(&self) -> &[Box<dyn AbstractSpaceBoundary>] {
        &self.boundary
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod0_multi_curve.as_ref()
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod2_solid.as_ref()
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        self.lod0_point.as_ref()
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_multi_curve.as_ref()
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod2_multi_surface.as_ref()
    }
}
impl AbstractPhysicalSpace for Intersection {
    fn ade_of_abstract_physical_space(&self) -> &[Box<dyn ADEOfAbstractPhysicalSpace>] {
        &self.ade_of_abstract_physical_space
    }
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_terrain_intersection_curve.as_ref()
    }
    fn point_cloud(&self) -> Option<&Box<dyn AbstractPointCloud>> {
        self.point_cloud.as_ref()
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod1_terrain_intersection_curve.as_ref()
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_terrain_intersection_curve.as_ref()
    }
}
impl AbstractUnoccupiedSpace for Intersection {
    fn ade_of_abstract_unoccupied_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractUnoccupiedSpace>] {
        &self.ade_of_abstract_unoccupied_space
    }
}
impl AbstractTransportationSpace for Intersection {
    fn traffic_direction(&self) -> Option<TrafficDirectionValue> {
        self.traffic_direction
    }
    fn occupancy(&self) -> &[Occupancy] {
        &self.occupancy
    }
    fn ade_of_abstract_transportation_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractTransportationSpace>] {
        &self.ade_of_abstract_transportation_space
    }
    fn auxiliary_traffic_space(&self) -> &[AuxiliaryTrafficSpace] {
        &self.auxiliary_traffic_space
    }
    fn hole(&self) -> &[Hole] {
        &self.hole
    }
    fn traffic_space(&self) -> &[TrafficSpace] {
        &self.traffic_space
    }
    fn marking(&self) -> &[Marking] {
        &self.marking
    }
}
impl Intersection {
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
        let mut space_type = None;
        let mut volume = Vec::new();
        let mut area = Vec::new();
        let mut ade_of_abstract_space = Vec::new();
        let mut lod2_multi_curve = None;
        let mut lod3_multi_surface = None;
        let mut lod0_multi_surface = None;
        let mut lod1_solid = None;
        let mut lod3_solid = None;
        let mut boundary = Vec::new();
        let mut lod0_multi_curve = None;
        let mut lod2_solid = None;
        let mut lod0_point = None;
        let mut lod3_multi_curve = None;
        let mut lod2_multi_surface = None;
        let mut ade_of_abstract_physical_space = Vec::new();
        let mut lod3_terrain_intersection_curve = None;
        let mut point_cloud = None;
        let mut lod1_terrain_intersection_curve = None;
        let mut lod2_terrain_intersection_curve = None;
        let mut ade_of_abstract_unoccupied_space = Vec::new();
        let mut traffic_direction = None;
        let mut occupancy = Vec::new();
        let mut ade_of_abstract_transportation_space = Vec::new();
        let mut auxiliary_traffic_space = Vec::new();
        let mut hole = Vec::new();
        let mut traffic_space = Vec::new();
        let mut marking = Vec::new();
        let mut class_ = None;
        let mut ade_of_intersection = Vec::new();
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
                (crate::namespace::NS_CORE, "spaceType") => {
                    space_type = Some(SpaceType::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_CORE, "volume") => {
                    volume.push(QualifiedVolume::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "area") => {
                    area.push(QualifiedArea::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "adeOfAbstractSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod2MultiCurve") => {
                    lod2_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1Solid") => {
                    lod1_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3Solid") => {
                    lod3_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "boundary") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        boundary
                            .push(
                                super::dispatchers::parse_dyn_abstract_space_boundary(
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
                (crate::namespace::NS_CORE, "lod2Solid") => {
                    lod2_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod0Point") => {
                    lod0_point = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3MultiCurve") => {
                    lod3_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractPhysicalSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod3TerrainIntersectionCurve") => {
                    lod3_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1TerrainIntersectionCurve") => {
                    lod1_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod2TerrainIntersectionCurve") => {
                    lod2_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractUnoccupiedSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "trafficDirection") => {
                    traffic_direction = Some(
                        TrafficDirectionValue::from_gml_text(&sub.read_text()?)?,
                    );
                }
                (crate::namespace::NS_TRANSPORTATION, "occupancy") => {
                    occupancy.push(Occupancy::from_gml(&mut sub)?);
                }
                (
                    crate::namespace::NS_TRANSPORTATION,
                    "adeOfAbstractTransportationSpace",
                ) => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "auxiliaryTrafficSpace") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        auxiliary_traffic_space
                            .push(
                                AuxiliaryTrafficSpace::from_gml_with_info(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "hole") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        hole.push(Hole::from_gml_with_info(&mut wrapper, &child_info)?);
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "trafficSpace") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        traffic_space
                            .push(
                                TrafficSpace::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "marking") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        marking
                            .push(
                                Marking::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "class") => {
                    class_ = Some(IntersectionClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "adeOfIntersection") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Intersection {
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
            space_type,
            volume,
            area,
            ade_of_abstract_space,
            lod2_multi_curve,
            lod3_multi_surface,
            lod0_multi_surface,
            lod1_solid,
            lod3_solid,
            boundary,
            lod0_multi_curve,
            lod2_solid,
            lod0_point,
            lod3_multi_curve,
            lod2_multi_surface,
            ade_of_abstract_physical_space,
            lod3_terrain_intersection_curve,
            point_cloud,
            lod1_terrain_intersection_curve,
            lod2_terrain_intersection_curve,
            ade_of_abstract_unoccupied_space,
            traffic_direction,
            occupancy,
            ade_of_abstract_transportation_space,
            auxiliary_traffic_space,
            hole,
            traffic_space,
            marking,
            class_,
            ade_of_intersection,
        })
    }
}
impl crate::from_gml::FromGml for Intersection {
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
#[derive(Debug, Default)]
pub struct Railway {
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
    pub space_type: Option<SpaceType>,
    pub volume: Vec<QualifiedVolume>,
    pub area: Vec<QualifiedArea>,
    pub ade_of_abstract_space: Vec<Box<dyn ADEOfAbstractSpace>>,
    pub lod2_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod3_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod0_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod1_solid: Option<crate::geometry::Solid>,
    pub lod3_solid: Option<crate::geometry::Solid>,
    pub boundary: Vec<Box<dyn AbstractSpaceBoundary>>,
    pub lod0_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_solid: Option<crate::geometry::Solid>,
    pub lod0_point: Option<crate::geometry::DirectPosition>,
    pub lod3_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_multi_surface: Option<crate::geometry::MultiSurface>,
    pub ade_of_abstract_physical_space: Vec<Box<dyn ADEOfAbstractPhysicalSpace>>,
    pub lod3_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub point_cloud: Option<Box<dyn AbstractPointCloud>>,
    pub lod1_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub ade_of_abstract_unoccupied_space: Vec<Box<dyn ADEOfAbstractUnoccupiedSpace>>,
    pub traffic_direction: Option<TrafficDirectionValue>,
    pub occupancy: Vec<Occupancy>,
    pub ade_of_abstract_transportation_space: Vec<
        Box<dyn ADEOfAbstractTransportationSpace>,
    >,
    pub auxiliary_traffic_space: Vec<AuxiliaryTrafficSpace>,
    pub hole: Vec<Hole>,
    pub traffic_space: Vec<TrafficSpace>,
    pub marking: Vec<Marking>,
    pub class_: Option<RailwayClassValue>,
    pub function: Vec<RailwayFunctionValue>,
    pub usage: Vec<RailwayUsageValue>,
    pub ade_of_railway: Vec<Box<dyn ADEOfRailway>>,
    pub intersection: Vec<Intersection>,
    pub section: Vec<Section>,
}
impl AbstractFeature for Railway {
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
impl AbstractFeatureWithLifespan for Railway {
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
impl AbstractCityObject for Railway {
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
impl AbstractSpace for Railway {
    fn space_type(&self) -> Option<SpaceType> {
        self.space_type
    }
    fn volume(&self) -> &[QualifiedVolume] {
        &self.volume
    }
    fn area(&self) -> &[QualifiedArea] {
        &self.area
    }
    fn ade_of_abstract_space(&self) -> &[Box<dyn ADEOfAbstractSpace>] {
        &self.ade_of_abstract_space
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_multi_curve.as_ref()
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod3_multi_surface.as_ref()
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod0_multi_surface.as_ref()
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod1_solid.as_ref()
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod3_solid.as_ref()
    }
    fn boundary(&self) -> &[Box<dyn AbstractSpaceBoundary>] {
        &self.boundary
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod0_multi_curve.as_ref()
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod2_solid.as_ref()
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        self.lod0_point.as_ref()
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_multi_curve.as_ref()
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod2_multi_surface.as_ref()
    }
}
impl AbstractPhysicalSpace for Railway {
    fn ade_of_abstract_physical_space(&self) -> &[Box<dyn ADEOfAbstractPhysicalSpace>] {
        &self.ade_of_abstract_physical_space
    }
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_terrain_intersection_curve.as_ref()
    }
    fn point_cloud(&self) -> Option<&Box<dyn AbstractPointCloud>> {
        self.point_cloud.as_ref()
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod1_terrain_intersection_curve.as_ref()
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_terrain_intersection_curve.as_ref()
    }
}
impl AbstractUnoccupiedSpace for Railway {
    fn ade_of_abstract_unoccupied_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractUnoccupiedSpace>] {
        &self.ade_of_abstract_unoccupied_space
    }
}
impl AbstractTransportationSpace for Railway {
    fn traffic_direction(&self) -> Option<TrafficDirectionValue> {
        self.traffic_direction
    }
    fn occupancy(&self) -> &[Occupancy] {
        &self.occupancy
    }
    fn ade_of_abstract_transportation_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractTransportationSpace>] {
        &self.ade_of_abstract_transportation_space
    }
    fn auxiliary_traffic_space(&self) -> &[AuxiliaryTrafficSpace] {
        &self.auxiliary_traffic_space
    }
    fn hole(&self) -> &[Hole] {
        &self.hole
    }
    fn traffic_space(&self) -> &[TrafficSpace] {
        &self.traffic_space
    }
    fn marking(&self) -> &[Marking] {
        &self.marking
    }
}
impl Railway {
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
        let mut space_type = None;
        let mut volume = Vec::new();
        let mut area = Vec::new();
        let mut ade_of_abstract_space = Vec::new();
        let mut lod2_multi_curve = None;
        let mut lod3_multi_surface = None;
        let mut lod0_multi_surface = None;
        let mut lod1_solid = None;
        let mut lod3_solid = None;
        let mut boundary = Vec::new();
        let mut lod0_multi_curve = None;
        let mut lod2_solid = None;
        let mut lod0_point = None;
        let mut lod3_multi_curve = None;
        let mut lod2_multi_surface = None;
        let mut ade_of_abstract_physical_space = Vec::new();
        let mut lod3_terrain_intersection_curve = None;
        let mut point_cloud = None;
        let mut lod1_terrain_intersection_curve = None;
        let mut lod2_terrain_intersection_curve = None;
        let mut ade_of_abstract_unoccupied_space = Vec::new();
        let mut traffic_direction = None;
        let mut occupancy = Vec::new();
        let mut ade_of_abstract_transportation_space = Vec::new();
        let mut auxiliary_traffic_space = Vec::new();
        let mut hole = Vec::new();
        let mut traffic_space = Vec::new();
        let mut marking = Vec::new();
        let mut class_ = None;
        let mut function = Vec::new();
        let mut usage = Vec::new();
        let mut ade_of_railway = Vec::new();
        let mut intersection = Vec::new();
        let mut section = Vec::new();
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
                (crate::namespace::NS_CORE, "spaceType") => {
                    space_type = Some(SpaceType::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_CORE, "volume") => {
                    volume.push(QualifiedVolume::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "area") => {
                    area.push(QualifiedArea::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "adeOfAbstractSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod2MultiCurve") => {
                    lod2_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1Solid") => {
                    lod1_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3Solid") => {
                    lod3_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "boundary") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        boundary
                            .push(
                                super::dispatchers::parse_dyn_abstract_space_boundary(
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
                (crate::namespace::NS_CORE, "lod2Solid") => {
                    lod2_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod0Point") => {
                    lod0_point = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3MultiCurve") => {
                    lod3_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractPhysicalSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod3TerrainIntersectionCurve") => {
                    lod3_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1TerrainIntersectionCurve") => {
                    lod1_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod2TerrainIntersectionCurve") => {
                    lod2_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractUnoccupiedSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "trafficDirection") => {
                    traffic_direction = Some(
                        TrafficDirectionValue::from_gml_text(&sub.read_text()?)?,
                    );
                }
                (crate::namespace::NS_TRANSPORTATION, "occupancy") => {
                    occupancy.push(Occupancy::from_gml(&mut sub)?);
                }
                (
                    crate::namespace::NS_TRANSPORTATION,
                    "adeOfAbstractTransportationSpace",
                ) => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "auxiliaryTrafficSpace") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        auxiliary_traffic_space
                            .push(
                                AuxiliaryTrafficSpace::from_gml_with_info(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "hole") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        hole.push(Hole::from_gml_with_info(&mut wrapper, &child_info)?);
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "trafficSpace") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        traffic_space
                            .push(
                                TrafficSpace::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "marking") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        marking
                            .push(
                                Marking::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "class") => {
                    class_ = Some(RailwayClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "function") => {
                    function.push(RailwayFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "usage") => {
                    usage.push(RailwayUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "adeOfRailway") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "intersection") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        intersection
                            .push(
                                Intersection::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "section") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        section
                            .push(
                                Section::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Railway {
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
            space_type,
            volume,
            area,
            ade_of_abstract_space,
            lod2_multi_curve,
            lod3_multi_surface,
            lod0_multi_surface,
            lod1_solid,
            lod3_solid,
            boundary,
            lod0_multi_curve,
            lod2_solid,
            lod0_point,
            lod3_multi_curve,
            lod2_multi_surface,
            ade_of_abstract_physical_space,
            lod3_terrain_intersection_curve,
            point_cloud,
            lod1_terrain_intersection_curve,
            lod2_terrain_intersection_curve,
            ade_of_abstract_unoccupied_space,
            traffic_direction,
            occupancy,
            ade_of_abstract_transportation_space,
            auxiliary_traffic_space,
            hole,
            traffic_space,
            marking,
            class_,
            function,
            usage,
            ade_of_railway,
            intersection,
            section,
        })
    }
}
impl crate::from_gml::FromGml for Railway {
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
#[derive(Debug, Default)]
pub struct Road {
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
    pub space_type: Option<SpaceType>,
    pub volume: Vec<QualifiedVolume>,
    pub area: Vec<QualifiedArea>,
    pub ade_of_abstract_space: Vec<Box<dyn ADEOfAbstractSpace>>,
    pub lod2_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod3_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod0_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod1_solid: Option<crate::geometry::Solid>,
    pub lod3_solid: Option<crate::geometry::Solid>,
    pub boundary: Vec<Box<dyn AbstractSpaceBoundary>>,
    pub lod0_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_solid: Option<crate::geometry::Solid>,
    pub lod0_point: Option<crate::geometry::DirectPosition>,
    pub lod3_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_multi_surface: Option<crate::geometry::MultiSurface>,
    pub ade_of_abstract_physical_space: Vec<Box<dyn ADEOfAbstractPhysicalSpace>>,
    pub lod3_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub point_cloud: Option<Box<dyn AbstractPointCloud>>,
    pub lod1_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub ade_of_abstract_unoccupied_space: Vec<Box<dyn ADEOfAbstractUnoccupiedSpace>>,
    pub traffic_direction: Option<TrafficDirectionValue>,
    pub occupancy: Vec<Occupancy>,
    pub ade_of_abstract_transportation_space: Vec<
        Box<dyn ADEOfAbstractTransportationSpace>,
    >,
    pub auxiliary_traffic_space: Vec<AuxiliaryTrafficSpace>,
    pub hole: Vec<Hole>,
    pub traffic_space: Vec<TrafficSpace>,
    pub marking: Vec<Marking>,
    pub class_: Option<RoadClassValue>,
    pub function: Vec<RoadFunctionValue>,
    pub usage: Vec<RoadUsageValue>,
    pub ade_of_road: Vec<Box<dyn ADEOfRoad>>,
    pub intersection: Vec<Intersection>,
    pub section: Vec<Section>,
}
impl AbstractFeature for Road {
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
impl AbstractFeatureWithLifespan for Road {
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
impl AbstractCityObject for Road {
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
impl AbstractSpace for Road {
    fn space_type(&self) -> Option<SpaceType> {
        self.space_type
    }
    fn volume(&self) -> &[QualifiedVolume] {
        &self.volume
    }
    fn area(&self) -> &[QualifiedArea] {
        &self.area
    }
    fn ade_of_abstract_space(&self) -> &[Box<dyn ADEOfAbstractSpace>] {
        &self.ade_of_abstract_space
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_multi_curve.as_ref()
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod3_multi_surface.as_ref()
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod0_multi_surface.as_ref()
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod1_solid.as_ref()
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod3_solid.as_ref()
    }
    fn boundary(&self) -> &[Box<dyn AbstractSpaceBoundary>] {
        &self.boundary
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod0_multi_curve.as_ref()
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod2_solid.as_ref()
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        self.lod0_point.as_ref()
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_multi_curve.as_ref()
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod2_multi_surface.as_ref()
    }
}
impl AbstractPhysicalSpace for Road {
    fn ade_of_abstract_physical_space(&self) -> &[Box<dyn ADEOfAbstractPhysicalSpace>] {
        &self.ade_of_abstract_physical_space
    }
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_terrain_intersection_curve.as_ref()
    }
    fn point_cloud(&self) -> Option<&Box<dyn AbstractPointCloud>> {
        self.point_cloud.as_ref()
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod1_terrain_intersection_curve.as_ref()
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_terrain_intersection_curve.as_ref()
    }
}
impl AbstractUnoccupiedSpace for Road {
    fn ade_of_abstract_unoccupied_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractUnoccupiedSpace>] {
        &self.ade_of_abstract_unoccupied_space
    }
}
impl AbstractTransportationSpace for Road {
    fn traffic_direction(&self) -> Option<TrafficDirectionValue> {
        self.traffic_direction
    }
    fn occupancy(&self) -> &[Occupancy] {
        &self.occupancy
    }
    fn ade_of_abstract_transportation_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractTransportationSpace>] {
        &self.ade_of_abstract_transportation_space
    }
    fn auxiliary_traffic_space(&self) -> &[AuxiliaryTrafficSpace] {
        &self.auxiliary_traffic_space
    }
    fn hole(&self) -> &[Hole] {
        &self.hole
    }
    fn traffic_space(&self) -> &[TrafficSpace] {
        &self.traffic_space
    }
    fn marking(&self) -> &[Marking] {
        &self.marking
    }
}
impl Road {
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
        let mut space_type = None;
        let mut volume = Vec::new();
        let mut area = Vec::new();
        let mut ade_of_abstract_space = Vec::new();
        let mut lod2_multi_curve = None;
        let mut lod3_multi_surface = None;
        let mut lod0_multi_surface = None;
        let mut lod1_solid = None;
        let mut lod3_solid = None;
        let mut boundary = Vec::new();
        let mut lod0_multi_curve = None;
        let mut lod2_solid = None;
        let mut lod0_point = None;
        let mut lod3_multi_curve = None;
        let mut lod2_multi_surface = None;
        let mut ade_of_abstract_physical_space = Vec::new();
        let mut lod3_terrain_intersection_curve = None;
        let mut point_cloud = None;
        let mut lod1_terrain_intersection_curve = None;
        let mut lod2_terrain_intersection_curve = None;
        let mut ade_of_abstract_unoccupied_space = Vec::new();
        let mut traffic_direction = None;
        let mut occupancy = Vec::new();
        let mut ade_of_abstract_transportation_space = Vec::new();
        let mut auxiliary_traffic_space = Vec::new();
        let mut hole = Vec::new();
        let mut traffic_space = Vec::new();
        let mut marking = Vec::new();
        let mut class_ = None;
        let mut function = Vec::new();
        let mut usage = Vec::new();
        let mut ade_of_road = Vec::new();
        let mut intersection = Vec::new();
        let mut section = Vec::new();
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
                (crate::namespace::NS_CORE, "spaceType") => {
                    space_type = Some(SpaceType::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_CORE, "volume") => {
                    volume.push(QualifiedVolume::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "area") => {
                    area.push(QualifiedArea::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "adeOfAbstractSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod2MultiCurve") => {
                    lod2_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1Solid") => {
                    lod1_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3Solid") => {
                    lod3_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "boundary") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        boundary
                            .push(
                                super::dispatchers::parse_dyn_abstract_space_boundary(
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
                (crate::namespace::NS_CORE, "lod2Solid") => {
                    lod2_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod0Point") => {
                    lod0_point = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3MultiCurve") => {
                    lod3_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractPhysicalSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod3TerrainIntersectionCurve") => {
                    lod3_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1TerrainIntersectionCurve") => {
                    lod1_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod2TerrainIntersectionCurve") => {
                    lod2_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractUnoccupiedSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "trafficDirection") => {
                    traffic_direction = Some(
                        TrafficDirectionValue::from_gml_text(&sub.read_text()?)?,
                    );
                }
                (crate::namespace::NS_TRANSPORTATION, "occupancy") => {
                    occupancy.push(Occupancy::from_gml(&mut sub)?);
                }
                (
                    crate::namespace::NS_TRANSPORTATION,
                    "adeOfAbstractTransportationSpace",
                ) => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "auxiliaryTrafficSpace") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        auxiliary_traffic_space
                            .push(
                                AuxiliaryTrafficSpace::from_gml_with_info(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "hole") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        hole.push(Hole::from_gml_with_info(&mut wrapper, &child_info)?);
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "trafficSpace") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        traffic_space
                            .push(
                                TrafficSpace::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "marking") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        marking
                            .push(
                                Marking::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "class") => {
                    class_ = Some(RoadClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "function") => {
                    function.push(RoadFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "usage") => {
                    usage.push(RoadUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "adeOfRoad") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "intersection") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        intersection
                            .push(
                                Intersection::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "section") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        section
                            .push(
                                Section::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Road {
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
            space_type,
            volume,
            area,
            ade_of_abstract_space,
            lod2_multi_curve,
            lod3_multi_surface,
            lod0_multi_surface,
            lod1_solid,
            lod3_solid,
            boundary,
            lod0_multi_curve,
            lod2_solid,
            lod0_point,
            lod3_multi_curve,
            lod2_multi_surface,
            ade_of_abstract_physical_space,
            lod3_terrain_intersection_curve,
            point_cloud,
            lod1_terrain_intersection_curve,
            lod2_terrain_intersection_curve,
            ade_of_abstract_unoccupied_space,
            traffic_direction,
            occupancy,
            ade_of_abstract_transportation_space,
            auxiliary_traffic_space,
            hole,
            traffic_space,
            marking,
            class_,
            function,
            usage,
            ade_of_road,
            intersection,
            section,
        })
    }
}
impl crate::from_gml::FromGml for Road {
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
#[derive(Debug, Default)]
pub struct Section {
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
    pub space_type: Option<SpaceType>,
    pub volume: Vec<QualifiedVolume>,
    pub area: Vec<QualifiedArea>,
    pub ade_of_abstract_space: Vec<Box<dyn ADEOfAbstractSpace>>,
    pub lod2_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod3_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod0_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod1_solid: Option<crate::geometry::Solid>,
    pub lod3_solid: Option<crate::geometry::Solid>,
    pub boundary: Vec<Box<dyn AbstractSpaceBoundary>>,
    pub lod0_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_solid: Option<crate::geometry::Solid>,
    pub lod0_point: Option<crate::geometry::DirectPosition>,
    pub lod3_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_multi_surface: Option<crate::geometry::MultiSurface>,
    pub ade_of_abstract_physical_space: Vec<Box<dyn ADEOfAbstractPhysicalSpace>>,
    pub lod3_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub point_cloud: Option<Box<dyn AbstractPointCloud>>,
    pub lod1_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub ade_of_abstract_unoccupied_space: Vec<Box<dyn ADEOfAbstractUnoccupiedSpace>>,
    pub traffic_direction: Option<TrafficDirectionValue>,
    pub occupancy: Vec<Occupancy>,
    pub ade_of_abstract_transportation_space: Vec<
        Box<dyn ADEOfAbstractTransportationSpace>,
    >,
    pub auxiliary_traffic_space: Vec<AuxiliaryTrafficSpace>,
    pub hole: Vec<Hole>,
    pub traffic_space: Vec<TrafficSpace>,
    pub marking: Vec<Marking>,
    pub class_: Option<SectionClassValue>,
    pub ade_of_section: Vec<Box<dyn ADEOfSection>>,
}
impl AbstractFeature for Section {
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
impl AbstractFeatureWithLifespan for Section {
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
impl AbstractCityObject for Section {
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
impl AbstractSpace for Section {
    fn space_type(&self) -> Option<SpaceType> {
        self.space_type
    }
    fn volume(&self) -> &[QualifiedVolume] {
        &self.volume
    }
    fn area(&self) -> &[QualifiedArea] {
        &self.area
    }
    fn ade_of_abstract_space(&self) -> &[Box<dyn ADEOfAbstractSpace>] {
        &self.ade_of_abstract_space
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_multi_curve.as_ref()
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod3_multi_surface.as_ref()
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod0_multi_surface.as_ref()
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod1_solid.as_ref()
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod3_solid.as_ref()
    }
    fn boundary(&self) -> &[Box<dyn AbstractSpaceBoundary>] {
        &self.boundary
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod0_multi_curve.as_ref()
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod2_solid.as_ref()
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        self.lod0_point.as_ref()
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_multi_curve.as_ref()
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod2_multi_surface.as_ref()
    }
}
impl AbstractPhysicalSpace for Section {
    fn ade_of_abstract_physical_space(&self) -> &[Box<dyn ADEOfAbstractPhysicalSpace>] {
        &self.ade_of_abstract_physical_space
    }
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_terrain_intersection_curve.as_ref()
    }
    fn point_cloud(&self) -> Option<&Box<dyn AbstractPointCloud>> {
        self.point_cloud.as_ref()
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod1_terrain_intersection_curve.as_ref()
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_terrain_intersection_curve.as_ref()
    }
}
impl AbstractUnoccupiedSpace for Section {
    fn ade_of_abstract_unoccupied_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractUnoccupiedSpace>] {
        &self.ade_of_abstract_unoccupied_space
    }
}
impl AbstractTransportationSpace for Section {
    fn traffic_direction(&self) -> Option<TrafficDirectionValue> {
        self.traffic_direction
    }
    fn occupancy(&self) -> &[Occupancy] {
        &self.occupancy
    }
    fn ade_of_abstract_transportation_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractTransportationSpace>] {
        &self.ade_of_abstract_transportation_space
    }
    fn auxiliary_traffic_space(&self) -> &[AuxiliaryTrafficSpace] {
        &self.auxiliary_traffic_space
    }
    fn hole(&self) -> &[Hole] {
        &self.hole
    }
    fn traffic_space(&self) -> &[TrafficSpace] {
        &self.traffic_space
    }
    fn marking(&self) -> &[Marking] {
        &self.marking
    }
}
impl Section {
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
        let mut space_type = None;
        let mut volume = Vec::new();
        let mut area = Vec::new();
        let mut ade_of_abstract_space = Vec::new();
        let mut lod2_multi_curve = None;
        let mut lod3_multi_surface = None;
        let mut lod0_multi_surface = None;
        let mut lod1_solid = None;
        let mut lod3_solid = None;
        let mut boundary = Vec::new();
        let mut lod0_multi_curve = None;
        let mut lod2_solid = None;
        let mut lod0_point = None;
        let mut lod3_multi_curve = None;
        let mut lod2_multi_surface = None;
        let mut ade_of_abstract_physical_space = Vec::new();
        let mut lod3_terrain_intersection_curve = None;
        let mut point_cloud = None;
        let mut lod1_terrain_intersection_curve = None;
        let mut lod2_terrain_intersection_curve = None;
        let mut ade_of_abstract_unoccupied_space = Vec::new();
        let mut traffic_direction = None;
        let mut occupancy = Vec::new();
        let mut ade_of_abstract_transportation_space = Vec::new();
        let mut auxiliary_traffic_space = Vec::new();
        let mut hole = Vec::new();
        let mut traffic_space = Vec::new();
        let mut marking = Vec::new();
        let mut class_ = None;
        let mut ade_of_section = Vec::new();
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
                (crate::namespace::NS_CORE, "spaceType") => {
                    space_type = Some(SpaceType::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_CORE, "volume") => {
                    volume.push(QualifiedVolume::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "area") => {
                    area.push(QualifiedArea::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "adeOfAbstractSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod2MultiCurve") => {
                    lod2_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1Solid") => {
                    lod1_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3Solid") => {
                    lod3_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "boundary") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        boundary
                            .push(
                                super::dispatchers::parse_dyn_abstract_space_boundary(
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
                (crate::namespace::NS_CORE, "lod2Solid") => {
                    lod2_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod0Point") => {
                    lod0_point = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3MultiCurve") => {
                    lod3_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractPhysicalSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod3TerrainIntersectionCurve") => {
                    lod3_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1TerrainIntersectionCurve") => {
                    lod1_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod2TerrainIntersectionCurve") => {
                    lod2_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractUnoccupiedSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "trafficDirection") => {
                    traffic_direction = Some(
                        TrafficDirectionValue::from_gml_text(&sub.read_text()?)?,
                    );
                }
                (crate::namespace::NS_TRANSPORTATION, "occupancy") => {
                    occupancy.push(Occupancy::from_gml(&mut sub)?);
                }
                (
                    crate::namespace::NS_TRANSPORTATION,
                    "adeOfAbstractTransportationSpace",
                ) => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "auxiliaryTrafficSpace") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        auxiliary_traffic_space
                            .push(
                                AuxiliaryTrafficSpace::from_gml_with_info(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "hole") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        hole.push(Hole::from_gml_with_info(&mut wrapper, &child_info)?);
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "trafficSpace") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        traffic_space
                            .push(
                                TrafficSpace::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "marking") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        marking
                            .push(
                                Marking::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "class") => {
                    class_ = Some(SectionClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "adeOfSection") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Section {
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
            space_type,
            volume,
            area,
            ade_of_abstract_space,
            lod2_multi_curve,
            lod3_multi_surface,
            lod0_multi_surface,
            lod1_solid,
            lod3_solid,
            boundary,
            lod0_multi_curve,
            lod2_solid,
            lod0_point,
            lod3_multi_curve,
            lod2_multi_surface,
            ade_of_abstract_physical_space,
            lod3_terrain_intersection_curve,
            point_cloud,
            lod1_terrain_intersection_curve,
            lod2_terrain_intersection_curve,
            ade_of_abstract_unoccupied_space,
            traffic_direction,
            occupancy,
            ade_of_abstract_transportation_space,
            auxiliary_traffic_space,
            hole,
            traffic_space,
            marking,
            class_,
            ade_of_section,
        })
    }
}
impl crate::from_gml::FromGml for Section {
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
#[derive(Debug, Default)]
pub struct Square {
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
    pub space_type: Option<SpaceType>,
    pub volume: Vec<QualifiedVolume>,
    pub area: Vec<QualifiedArea>,
    pub ade_of_abstract_space: Vec<Box<dyn ADEOfAbstractSpace>>,
    pub lod2_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod3_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod0_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod1_solid: Option<crate::geometry::Solid>,
    pub lod3_solid: Option<crate::geometry::Solid>,
    pub boundary: Vec<Box<dyn AbstractSpaceBoundary>>,
    pub lod0_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_solid: Option<crate::geometry::Solid>,
    pub lod0_point: Option<crate::geometry::DirectPosition>,
    pub lod3_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_multi_surface: Option<crate::geometry::MultiSurface>,
    pub ade_of_abstract_physical_space: Vec<Box<dyn ADEOfAbstractPhysicalSpace>>,
    pub lod3_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub point_cloud: Option<Box<dyn AbstractPointCloud>>,
    pub lod1_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub ade_of_abstract_unoccupied_space: Vec<Box<dyn ADEOfAbstractUnoccupiedSpace>>,
    pub traffic_direction: Option<TrafficDirectionValue>,
    pub occupancy: Vec<Occupancy>,
    pub ade_of_abstract_transportation_space: Vec<
        Box<dyn ADEOfAbstractTransportationSpace>,
    >,
    pub auxiliary_traffic_space: Vec<AuxiliaryTrafficSpace>,
    pub hole: Vec<Hole>,
    pub traffic_space: Vec<TrafficSpace>,
    pub marking: Vec<Marking>,
    pub class_: Option<SquareClassValue>,
    pub function: Vec<SquareFunctionValue>,
    pub usage: Vec<SquareUsageValue>,
    pub ade_of_square: Vec<Box<dyn ADEOfSquare>>,
}
impl AbstractFeature for Square {
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
impl AbstractFeatureWithLifespan for Square {
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
impl AbstractCityObject for Square {
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
impl AbstractSpace for Square {
    fn space_type(&self) -> Option<SpaceType> {
        self.space_type
    }
    fn volume(&self) -> &[QualifiedVolume] {
        &self.volume
    }
    fn area(&self) -> &[QualifiedArea] {
        &self.area
    }
    fn ade_of_abstract_space(&self) -> &[Box<dyn ADEOfAbstractSpace>] {
        &self.ade_of_abstract_space
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_multi_curve.as_ref()
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod3_multi_surface.as_ref()
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod0_multi_surface.as_ref()
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod1_solid.as_ref()
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod3_solid.as_ref()
    }
    fn boundary(&self) -> &[Box<dyn AbstractSpaceBoundary>] {
        &self.boundary
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod0_multi_curve.as_ref()
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod2_solid.as_ref()
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        self.lod0_point.as_ref()
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_multi_curve.as_ref()
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod2_multi_surface.as_ref()
    }
}
impl AbstractPhysicalSpace for Square {
    fn ade_of_abstract_physical_space(&self) -> &[Box<dyn ADEOfAbstractPhysicalSpace>] {
        &self.ade_of_abstract_physical_space
    }
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_terrain_intersection_curve.as_ref()
    }
    fn point_cloud(&self) -> Option<&Box<dyn AbstractPointCloud>> {
        self.point_cloud.as_ref()
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod1_terrain_intersection_curve.as_ref()
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_terrain_intersection_curve.as_ref()
    }
}
impl AbstractUnoccupiedSpace for Square {
    fn ade_of_abstract_unoccupied_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractUnoccupiedSpace>] {
        &self.ade_of_abstract_unoccupied_space
    }
}
impl AbstractTransportationSpace for Square {
    fn traffic_direction(&self) -> Option<TrafficDirectionValue> {
        self.traffic_direction
    }
    fn occupancy(&self) -> &[Occupancy] {
        &self.occupancy
    }
    fn ade_of_abstract_transportation_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractTransportationSpace>] {
        &self.ade_of_abstract_transportation_space
    }
    fn auxiliary_traffic_space(&self) -> &[AuxiliaryTrafficSpace] {
        &self.auxiliary_traffic_space
    }
    fn hole(&self) -> &[Hole] {
        &self.hole
    }
    fn traffic_space(&self) -> &[TrafficSpace] {
        &self.traffic_space
    }
    fn marking(&self) -> &[Marking] {
        &self.marking
    }
}
impl Square {
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
        let mut space_type = None;
        let mut volume = Vec::new();
        let mut area = Vec::new();
        let mut ade_of_abstract_space = Vec::new();
        let mut lod2_multi_curve = None;
        let mut lod3_multi_surface = None;
        let mut lod0_multi_surface = None;
        let mut lod1_solid = None;
        let mut lod3_solid = None;
        let mut boundary = Vec::new();
        let mut lod0_multi_curve = None;
        let mut lod2_solid = None;
        let mut lod0_point = None;
        let mut lod3_multi_curve = None;
        let mut lod2_multi_surface = None;
        let mut ade_of_abstract_physical_space = Vec::new();
        let mut lod3_terrain_intersection_curve = None;
        let mut point_cloud = None;
        let mut lod1_terrain_intersection_curve = None;
        let mut lod2_terrain_intersection_curve = None;
        let mut ade_of_abstract_unoccupied_space = Vec::new();
        let mut traffic_direction = None;
        let mut occupancy = Vec::new();
        let mut ade_of_abstract_transportation_space = Vec::new();
        let mut auxiliary_traffic_space = Vec::new();
        let mut hole = Vec::new();
        let mut traffic_space = Vec::new();
        let mut marking = Vec::new();
        let mut class_ = None;
        let mut function = Vec::new();
        let mut usage = Vec::new();
        let mut ade_of_square = Vec::new();
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
                (crate::namespace::NS_CORE, "spaceType") => {
                    space_type = Some(SpaceType::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_CORE, "volume") => {
                    volume.push(QualifiedVolume::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "area") => {
                    area.push(QualifiedArea::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "adeOfAbstractSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod2MultiCurve") => {
                    lod2_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1Solid") => {
                    lod1_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3Solid") => {
                    lod3_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "boundary") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        boundary
                            .push(
                                super::dispatchers::parse_dyn_abstract_space_boundary(
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
                (crate::namespace::NS_CORE, "lod2Solid") => {
                    lod2_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod0Point") => {
                    lod0_point = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3MultiCurve") => {
                    lod3_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractPhysicalSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod3TerrainIntersectionCurve") => {
                    lod3_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1TerrainIntersectionCurve") => {
                    lod1_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod2TerrainIntersectionCurve") => {
                    lod2_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractUnoccupiedSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "trafficDirection") => {
                    traffic_direction = Some(
                        TrafficDirectionValue::from_gml_text(&sub.read_text()?)?,
                    );
                }
                (crate::namespace::NS_TRANSPORTATION, "occupancy") => {
                    occupancy.push(Occupancy::from_gml(&mut sub)?);
                }
                (
                    crate::namespace::NS_TRANSPORTATION,
                    "adeOfAbstractTransportationSpace",
                ) => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "auxiliaryTrafficSpace") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        auxiliary_traffic_space
                            .push(
                                AuxiliaryTrafficSpace::from_gml_with_info(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "hole") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        hole.push(Hole::from_gml_with_info(&mut wrapper, &child_info)?);
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "trafficSpace") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        traffic_space
                            .push(
                                TrafficSpace::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "marking") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        marking
                            .push(
                                Marking::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "class") => {
                    class_ = Some(SquareClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "function") => {
                    function.push(SquareFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "usage") => {
                    usage.push(SquareUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "adeOfSquare") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Square {
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
            space_type,
            volume,
            area,
            ade_of_abstract_space,
            lod2_multi_curve,
            lod3_multi_surface,
            lod0_multi_surface,
            lod1_solid,
            lod3_solid,
            boundary,
            lod0_multi_curve,
            lod2_solid,
            lod0_point,
            lod3_multi_curve,
            lod2_multi_surface,
            ade_of_abstract_physical_space,
            lod3_terrain_intersection_curve,
            point_cloud,
            lod1_terrain_intersection_curve,
            lod2_terrain_intersection_curve,
            ade_of_abstract_unoccupied_space,
            traffic_direction,
            occupancy,
            ade_of_abstract_transportation_space,
            auxiliary_traffic_space,
            hole,
            traffic_space,
            marking,
            class_,
            function,
            usage,
            ade_of_square,
        })
    }
}
impl crate::from_gml::FromGml for Square {
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
#[derive(Debug, Default)]
pub struct Track {
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
    pub space_type: Option<SpaceType>,
    pub volume: Vec<QualifiedVolume>,
    pub area: Vec<QualifiedArea>,
    pub ade_of_abstract_space: Vec<Box<dyn ADEOfAbstractSpace>>,
    pub lod2_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod3_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod0_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod1_solid: Option<crate::geometry::Solid>,
    pub lod3_solid: Option<crate::geometry::Solid>,
    pub boundary: Vec<Box<dyn AbstractSpaceBoundary>>,
    pub lod0_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_solid: Option<crate::geometry::Solid>,
    pub lod0_point: Option<crate::geometry::DirectPosition>,
    pub lod3_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_multi_surface: Option<crate::geometry::MultiSurface>,
    pub ade_of_abstract_physical_space: Vec<Box<dyn ADEOfAbstractPhysicalSpace>>,
    pub lod3_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub point_cloud: Option<Box<dyn AbstractPointCloud>>,
    pub lod1_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub ade_of_abstract_unoccupied_space: Vec<Box<dyn ADEOfAbstractUnoccupiedSpace>>,
    pub traffic_direction: Option<TrafficDirectionValue>,
    pub occupancy: Vec<Occupancy>,
    pub ade_of_abstract_transportation_space: Vec<
        Box<dyn ADEOfAbstractTransportationSpace>,
    >,
    pub auxiliary_traffic_space: Vec<AuxiliaryTrafficSpace>,
    pub hole: Vec<Hole>,
    pub traffic_space: Vec<TrafficSpace>,
    pub marking: Vec<Marking>,
    pub class_: Option<TrackClassValue>,
    pub function: Vec<TrackFunctionValue>,
    pub usage: Vec<TrackUsageValue>,
    pub ade_of_track: Vec<Box<dyn ADEOfTrack>>,
    pub intersection: Vec<Intersection>,
    pub section: Vec<Section>,
}
impl AbstractFeature for Track {
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
impl AbstractFeatureWithLifespan for Track {
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
impl AbstractCityObject for Track {
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
impl AbstractSpace for Track {
    fn space_type(&self) -> Option<SpaceType> {
        self.space_type
    }
    fn volume(&self) -> &[QualifiedVolume] {
        &self.volume
    }
    fn area(&self) -> &[QualifiedArea] {
        &self.area
    }
    fn ade_of_abstract_space(&self) -> &[Box<dyn ADEOfAbstractSpace>] {
        &self.ade_of_abstract_space
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_multi_curve.as_ref()
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod3_multi_surface.as_ref()
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod0_multi_surface.as_ref()
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod1_solid.as_ref()
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod3_solid.as_ref()
    }
    fn boundary(&self) -> &[Box<dyn AbstractSpaceBoundary>] {
        &self.boundary
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod0_multi_curve.as_ref()
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod2_solid.as_ref()
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        self.lod0_point.as_ref()
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_multi_curve.as_ref()
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod2_multi_surface.as_ref()
    }
}
impl AbstractPhysicalSpace for Track {
    fn ade_of_abstract_physical_space(&self) -> &[Box<dyn ADEOfAbstractPhysicalSpace>] {
        &self.ade_of_abstract_physical_space
    }
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_terrain_intersection_curve.as_ref()
    }
    fn point_cloud(&self) -> Option<&Box<dyn AbstractPointCloud>> {
        self.point_cloud.as_ref()
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod1_terrain_intersection_curve.as_ref()
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_terrain_intersection_curve.as_ref()
    }
}
impl AbstractUnoccupiedSpace for Track {
    fn ade_of_abstract_unoccupied_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractUnoccupiedSpace>] {
        &self.ade_of_abstract_unoccupied_space
    }
}
impl AbstractTransportationSpace for Track {
    fn traffic_direction(&self) -> Option<TrafficDirectionValue> {
        self.traffic_direction
    }
    fn occupancy(&self) -> &[Occupancy] {
        &self.occupancy
    }
    fn ade_of_abstract_transportation_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractTransportationSpace>] {
        &self.ade_of_abstract_transportation_space
    }
    fn auxiliary_traffic_space(&self) -> &[AuxiliaryTrafficSpace] {
        &self.auxiliary_traffic_space
    }
    fn hole(&self) -> &[Hole] {
        &self.hole
    }
    fn traffic_space(&self) -> &[TrafficSpace] {
        &self.traffic_space
    }
    fn marking(&self) -> &[Marking] {
        &self.marking
    }
}
impl Track {
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
        let mut space_type = None;
        let mut volume = Vec::new();
        let mut area = Vec::new();
        let mut ade_of_abstract_space = Vec::new();
        let mut lod2_multi_curve = None;
        let mut lod3_multi_surface = None;
        let mut lod0_multi_surface = None;
        let mut lod1_solid = None;
        let mut lod3_solid = None;
        let mut boundary = Vec::new();
        let mut lod0_multi_curve = None;
        let mut lod2_solid = None;
        let mut lod0_point = None;
        let mut lod3_multi_curve = None;
        let mut lod2_multi_surface = None;
        let mut ade_of_abstract_physical_space = Vec::new();
        let mut lod3_terrain_intersection_curve = None;
        let mut point_cloud = None;
        let mut lod1_terrain_intersection_curve = None;
        let mut lod2_terrain_intersection_curve = None;
        let mut ade_of_abstract_unoccupied_space = Vec::new();
        let mut traffic_direction = None;
        let mut occupancy = Vec::new();
        let mut ade_of_abstract_transportation_space = Vec::new();
        let mut auxiliary_traffic_space = Vec::new();
        let mut hole = Vec::new();
        let mut traffic_space = Vec::new();
        let mut marking = Vec::new();
        let mut class_ = None;
        let mut function = Vec::new();
        let mut usage = Vec::new();
        let mut ade_of_track = Vec::new();
        let mut intersection = Vec::new();
        let mut section = Vec::new();
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
                (crate::namespace::NS_CORE, "spaceType") => {
                    space_type = Some(SpaceType::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_CORE, "volume") => {
                    volume.push(QualifiedVolume::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "area") => {
                    area.push(QualifiedArea::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "adeOfAbstractSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod2MultiCurve") => {
                    lod2_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1Solid") => {
                    lod1_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3Solid") => {
                    lod3_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "boundary") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        boundary
                            .push(
                                super::dispatchers::parse_dyn_abstract_space_boundary(
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
                (crate::namespace::NS_CORE, "lod2Solid") => {
                    lod2_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod0Point") => {
                    lod0_point = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3MultiCurve") => {
                    lod3_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractPhysicalSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod3TerrainIntersectionCurve") => {
                    lod3_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1TerrainIntersectionCurve") => {
                    lod1_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod2TerrainIntersectionCurve") => {
                    lod2_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractUnoccupiedSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "trafficDirection") => {
                    traffic_direction = Some(
                        TrafficDirectionValue::from_gml_text(&sub.read_text()?)?,
                    );
                }
                (crate::namespace::NS_TRANSPORTATION, "occupancy") => {
                    occupancy.push(Occupancy::from_gml(&mut sub)?);
                }
                (
                    crate::namespace::NS_TRANSPORTATION,
                    "adeOfAbstractTransportationSpace",
                ) => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "auxiliaryTrafficSpace") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        auxiliary_traffic_space
                            .push(
                                AuxiliaryTrafficSpace::from_gml_with_info(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "hole") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        hole.push(Hole::from_gml_with_info(&mut wrapper, &child_info)?);
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "trafficSpace") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        traffic_space
                            .push(
                                TrafficSpace::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "marking") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        marking
                            .push(
                                Marking::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "class") => {
                    class_ = Some(TrackClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "function") => {
                    function.push(TrackFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "usage") => {
                    usage.push(TrackUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "adeOfTrack") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "intersection") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        intersection
                            .push(
                                Intersection::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "section") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        section
                            .push(
                                Section::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Track {
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
            space_type,
            volume,
            area,
            ade_of_abstract_space,
            lod2_multi_curve,
            lod3_multi_surface,
            lod0_multi_surface,
            lod1_solid,
            lod3_solid,
            boundary,
            lod0_multi_curve,
            lod2_solid,
            lod0_point,
            lod3_multi_curve,
            lod2_multi_surface,
            ade_of_abstract_physical_space,
            lod3_terrain_intersection_curve,
            point_cloud,
            lod1_terrain_intersection_curve,
            lod2_terrain_intersection_curve,
            ade_of_abstract_unoccupied_space,
            traffic_direction,
            occupancy,
            ade_of_abstract_transportation_space,
            auxiliary_traffic_space,
            hole,
            traffic_space,
            marking,
            class_,
            function,
            usage,
            ade_of_track,
            intersection,
            section,
        })
    }
}
impl crate::from_gml::FromGml for Track {
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
#[derive(Debug, Default)]
pub struct Waterway {
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
    pub space_type: Option<SpaceType>,
    pub volume: Vec<QualifiedVolume>,
    pub area: Vec<QualifiedArea>,
    pub ade_of_abstract_space: Vec<Box<dyn ADEOfAbstractSpace>>,
    pub lod2_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod3_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod0_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod1_solid: Option<crate::geometry::Solid>,
    pub lod3_solid: Option<crate::geometry::Solid>,
    pub boundary: Vec<Box<dyn AbstractSpaceBoundary>>,
    pub lod0_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_solid: Option<crate::geometry::Solid>,
    pub lod0_point: Option<crate::geometry::DirectPosition>,
    pub lod3_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_multi_surface: Option<crate::geometry::MultiSurface>,
    pub ade_of_abstract_physical_space: Vec<Box<dyn ADEOfAbstractPhysicalSpace>>,
    pub lod3_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub point_cloud: Option<Box<dyn AbstractPointCloud>>,
    pub lod1_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub ade_of_abstract_unoccupied_space: Vec<Box<dyn ADEOfAbstractUnoccupiedSpace>>,
    pub traffic_direction: Option<TrafficDirectionValue>,
    pub occupancy: Vec<Occupancy>,
    pub ade_of_abstract_transportation_space: Vec<
        Box<dyn ADEOfAbstractTransportationSpace>,
    >,
    pub auxiliary_traffic_space: Vec<AuxiliaryTrafficSpace>,
    pub hole: Vec<Hole>,
    pub traffic_space: Vec<TrafficSpace>,
    pub marking: Vec<Marking>,
    pub class_: Option<WaterwayClassValue>,
    pub function: Vec<WaterwayFunctionValue>,
    pub usage: Vec<WaterwayUsageValue>,
    pub ade_of_waterway: Vec<Box<dyn ADEOfWaterway>>,
    pub intersection: Vec<Intersection>,
    pub section: Vec<Section>,
}
impl AbstractFeature for Waterway {
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
impl AbstractFeatureWithLifespan for Waterway {
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
impl AbstractCityObject for Waterway {
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
impl AbstractSpace for Waterway {
    fn space_type(&self) -> Option<SpaceType> {
        self.space_type
    }
    fn volume(&self) -> &[QualifiedVolume] {
        &self.volume
    }
    fn area(&self) -> &[QualifiedArea] {
        &self.area
    }
    fn ade_of_abstract_space(&self) -> &[Box<dyn ADEOfAbstractSpace>] {
        &self.ade_of_abstract_space
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_multi_curve.as_ref()
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod3_multi_surface.as_ref()
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod0_multi_surface.as_ref()
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod1_solid.as_ref()
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod3_solid.as_ref()
    }
    fn boundary(&self) -> &[Box<dyn AbstractSpaceBoundary>] {
        &self.boundary
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod0_multi_curve.as_ref()
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        self.lod2_solid.as_ref()
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        self.lod0_point.as_ref()
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_multi_curve.as_ref()
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        self.lod2_multi_surface.as_ref()
    }
}
impl AbstractPhysicalSpace for Waterway {
    fn ade_of_abstract_physical_space(&self) -> &[Box<dyn ADEOfAbstractPhysicalSpace>] {
        &self.ade_of_abstract_physical_space
    }
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_terrain_intersection_curve.as_ref()
    }
    fn point_cloud(&self) -> Option<&Box<dyn AbstractPointCloud>> {
        self.point_cloud.as_ref()
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod1_terrain_intersection_curve.as_ref()
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_terrain_intersection_curve.as_ref()
    }
}
impl AbstractUnoccupiedSpace for Waterway {
    fn ade_of_abstract_unoccupied_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractUnoccupiedSpace>] {
        &self.ade_of_abstract_unoccupied_space
    }
}
impl AbstractTransportationSpace for Waterway {
    fn traffic_direction(&self) -> Option<TrafficDirectionValue> {
        self.traffic_direction
    }
    fn occupancy(&self) -> &[Occupancy] {
        &self.occupancy
    }
    fn ade_of_abstract_transportation_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractTransportationSpace>] {
        &self.ade_of_abstract_transportation_space
    }
    fn auxiliary_traffic_space(&self) -> &[AuxiliaryTrafficSpace] {
        &self.auxiliary_traffic_space
    }
    fn hole(&self) -> &[Hole] {
        &self.hole
    }
    fn traffic_space(&self) -> &[TrafficSpace] {
        &self.traffic_space
    }
    fn marking(&self) -> &[Marking] {
        &self.marking
    }
}
impl Waterway {
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
        let mut space_type = None;
        let mut volume = Vec::new();
        let mut area = Vec::new();
        let mut ade_of_abstract_space = Vec::new();
        let mut lod2_multi_curve = None;
        let mut lod3_multi_surface = None;
        let mut lod0_multi_surface = None;
        let mut lod1_solid = None;
        let mut lod3_solid = None;
        let mut boundary = Vec::new();
        let mut lod0_multi_curve = None;
        let mut lod2_solid = None;
        let mut lod0_point = None;
        let mut lod3_multi_curve = None;
        let mut lod2_multi_surface = None;
        let mut ade_of_abstract_physical_space = Vec::new();
        let mut lod3_terrain_intersection_curve = None;
        let mut point_cloud = None;
        let mut lod1_terrain_intersection_curve = None;
        let mut lod2_terrain_intersection_curve = None;
        let mut ade_of_abstract_unoccupied_space = Vec::new();
        let mut traffic_direction = None;
        let mut occupancy = Vec::new();
        let mut ade_of_abstract_transportation_space = Vec::new();
        let mut auxiliary_traffic_space = Vec::new();
        let mut hole = Vec::new();
        let mut traffic_space = Vec::new();
        let mut marking = Vec::new();
        let mut class_ = None;
        let mut function = Vec::new();
        let mut usage = Vec::new();
        let mut ade_of_waterway = Vec::new();
        let mut intersection = Vec::new();
        let mut section = Vec::new();
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
                (crate::namespace::NS_CORE, "spaceType") => {
                    space_type = Some(SpaceType::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_CORE, "volume") => {
                    volume.push(QualifiedVolume::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "area") => {
                    area.push(QualifiedArea::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "adeOfAbstractSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod2MultiCurve") => {
                    lod2_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1Solid") => {
                    lod1_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3Solid") => {
                    lod3_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "boundary") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        boundary
                            .push(
                                super::dispatchers::parse_dyn_abstract_space_boundary(
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
                (crate::namespace::NS_CORE, "lod2Solid") => {
                    lod2_solid = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod0Point") => {
                    lod0_point = Some({
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
                    });
                }
                (crate::namespace::NS_CORE, "lod3MultiCurve") => {
                    lod3_multi_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractPhysicalSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_CORE, "lod3TerrainIntersectionCurve") => {
                    lod3_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod1TerrainIntersectionCurve") => {
                    lod1_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "lod2TerrainIntersectionCurve") => {
                    lod2_terrain_intersection_curve = Some({
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
                (crate::namespace::NS_CORE, "adeOfAbstractUnoccupiedSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "trafficDirection") => {
                    traffic_direction = Some(
                        TrafficDirectionValue::from_gml_text(&sub.read_text()?)?,
                    );
                }
                (crate::namespace::NS_TRANSPORTATION, "occupancy") => {
                    occupancy.push(Occupancy::from_gml(&mut sub)?);
                }
                (
                    crate::namespace::NS_TRANSPORTATION,
                    "adeOfAbstractTransportationSpace",
                ) => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "auxiliaryTrafficSpace") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        auxiliary_traffic_space
                            .push(
                                AuxiliaryTrafficSpace::from_gml_with_info(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "hole") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        hole.push(Hole::from_gml_with_info(&mut wrapper, &child_info)?);
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "trafficSpace") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        traffic_space
                            .push(
                                TrafficSpace::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "marking") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        marking
                            .push(
                                Marking::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "class") => {
                    class_ = Some(WaterwayClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "function") => {
                    function.push(WaterwayFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "usage") => {
                    usage.push(WaterwayUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_TRANSPORTATION, "adeOfWaterway") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_TRANSPORTATION, "intersection") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        intersection
                            .push(
                                Intersection::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TRANSPORTATION, "section") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        section
                            .push(
                                Section::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Waterway {
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
            space_type,
            volume,
            area,
            ade_of_abstract_space,
            lod2_multi_curve,
            lod3_multi_surface,
            lod0_multi_surface,
            lod1_solid,
            lod3_solid,
            boundary,
            lod0_multi_curve,
            lod2_solid,
            lod0_point,
            lod3_multi_curve,
            lod2_multi_surface,
            ade_of_abstract_physical_space,
            lod3_terrain_intersection_curve,
            point_cloud,
            lod1_terrain_intersection_curve,
            lod2_terrain_intersection_curve,
            ade_of_abstract_unoccupied_space,
            traffic_direction,
            occupancy,
            ade_of_abstract_transportation_space,
            auxiliary_traffic_space,
            hole,
            traffic_space,
            marking,
            class_,
            function,
            usage,
            ade_of_waterway,
            intersection,
            section,
        })
    }
}
impl crate::from_gml::FromGml for Waterway {
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
