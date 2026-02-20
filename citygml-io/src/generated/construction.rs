#![allow(unused_imports, unused_mut, unused_variables)]
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ConditionOfConstructionValue {
    #[default]
    Declined,
    Demolished,
    Functional,
    Projected,
    Ruin,
    UnderConstruction,
}
impl ConditionOfConstructionValue {
    pub fn from_gml_text(text: &str) -> Result<Self, crate::error::ReaderError> {
        match text.trim() {
            "declined" => Ok(ConditionOfConstructionValue::Declined),
            "demolished" => Ok(ConditionOfConstructionValue::Demolished),
            "functional" => Ok(ConditionOfConstructionValue::Functional),
            "projected" => Ok(ConditionOfConstructionValue::Projected),
            "ruin" => Ok(ConditionOfConstructionValue::Ruin),
            "underConstruction" => Ok(ConditionOfConstructionValue::UnderConstruction),
            other => {
                Err(crate::error::ReaderError::Parse {
                    message: format!(
                        "Unknown {} value: '{}'",
                        stringify!(ConditionOfConstructionValue), other,
                    ),
                })
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum HeightStatusValue {
    #[default]
    Estimated,
    Measured,
}
impl HeightStatusValue {
    pub fn from_gml_text(text: &str) -> Result<Self, crate::error::ReaderError> {
        match text.trim() {
            "estimated" => Ok(HeightStatusValue::Estimated),
            "measured" => Ok(HeightStatusValue::Measured),
            other => {
                Err(crate::error::ReaderError::Parse {
                    message: format!(
                        "Unknown {} value: '{}'", stringify!(HeightStatusValue), other,
                    ),
                })
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RelationToConstruction {
    #[default]
    Inside,
    Outside,
    BothInsideAndOutside,
}
impl RelationToConstruction {
    pub fn from_gml_text(text: &str) -> Result<Self, crate::error::ReaderError> {
        match text.trim() {
            "inside" => Ok(RelationToConstruction::Inside),
            "outside" => Ok(RelationToConstruction::Outside),
            "bothInsideAndOutside" => Ok(RelationToConstruction::BothInsideAndOutside),
            other => {
                Err(crate::error::ReaderError::Parse {
                    message: format!(
                        "Unknown {} value: '{}'", stringify!(RelationToConstruction),
                        other,
                    ),
                })
            }
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct ConstructionEvent {
    pub event: EventValue,
    pub date_of_event: String,
    pub description: Option<String>,
}
impl crate::from_gml::FromGml for ConstructionEvent {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut event = Default::default();
        let mut date_of_event = Default::default();
        let mut description = None;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CONSTRUCTION, "event") => {
                    event = EventValue(sub.read_text()?);
                }
                (crate::namespace::NS_CONSTRUCTION, "dateOfEvent") => {
                    date_of_event = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_GML, "description") => {
                    description = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(ConstructionEvent {
            event,
            date_of_event,
            description,
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct Elevation {
    pub elevation_reference: ElevationReferenceValue,
    pub elevation_value: crate::geometry::DirectPosition,
}
impl crate::from_gml::FromGml for Elevation {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut elevation_reference = Default::default();
        let mut elevation_value = Default::default();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CONSTRUCTION, "elevationReference") => {
                    elevation_reference = ElevationReferenceValue(sub.read_text()?);
                }
                (crate::namespace::NS_CONSTRUCTION, "elevationValue") => {
                    elevation_value = {
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
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Elevation {
            elevation_reference,
            elevation_value,
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct Height {
    pub high_reference: ElevationReferenceValue,
    pub low_reference: ElevationReferenceValue,
    pub status: HeightStatusValue,
    pub value: f64,
}
impl crate::from_gml::FromGml for Height {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut high_reference = Default::default();
        let mut low_reference = Default::default();
        let mut status = Default::default();
        let mut value = 0.0f64;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CONSTRUCTION, "highReference") => {
                    high_reference = ElevationReferenceValue(sub.read_text()?);
                }
                (crate::namespace::NS_CONSTRUCTION, "lowReference") => {
                    low_reference = ElevationReferenceValue(sub.read_text()?);
                }
                (crate::namespace::NS_CONSTRUCTION, "status") => {
                    status = HeightStatusValue::from_gml_text(&sub.read_text()?)?;
                }
                (crate::namespace::NS_CONSTRUCTION, "value") => {
                    value = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Height {
            high_reference,
            low_reference,
            status,
            value,
        })
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct DoorClassValue(pub String);
impl crate::from_gml::FromGml for DoorClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(DoorClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct DoorFunctionValue(pub String);
impl crate::from_gml::FromGml for DoorFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(DoorFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct DoorUsageValue(pub String);
impl crate::from_gml::FromGml for DoorUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(DoorUsageValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ElevationReferenceValue(pub String);
impl crate::from_gml::FromGml for ElevationReferenceValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(ElevationReferenceValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct EventValue(pub String);
impl crate::from_gml::FromGml for EventValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(EventValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct OtherConstructionClassValue(pub String);
impl crate::from_gml::FromGml for OtherConstructionClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(OtherConstructionClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct OtherConstructionFunctionValue(pub String);
impl crate::from_gml::FromGml for OtherConstructionFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(OtherConstructionFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct OtherConstructionUsageValue(pub String);
impl crate::from_gml::FromGml for OtherConstructionUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(OtherConstructionUsageValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct WindowClassValue(pub String);
impl crate::from_gml::FromGml for WindowClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(WindowClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct WindowFunctionValue(pub String);
impl crate::from_gml::FromGml for WindowFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(WindowFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct WindowUsageValue(pub String);
impl crate::from_gml::FromGml for WindowUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(WindowUsageValue(reader.read_text()?))
    }
}
pub trait AbstractConstructionSurfaceTrait: AbstractThematicSurfaceTrait {
    fn filling_surface(&self) -> &[AbstractFillingSurface];
}
#[derive(Debug, Clone)]
pub enum AbstractConstructionSurface {
    CeilingSurface(CeilingSurface),
    FloorSurface(FloorSurface),
    GroundSurface(GroundSurface),
    InteriorWallSurface(InteriorWallSurface),
    OuterCeilingSurface(OuterCeilingSurface),
    OuterFloorSurface(OuterFloorSurface),
    RoofSurface(RoofSurface),
    WallSurface(WallSurface),
}
impl Default for AbstractConstructionSurface {
    fn default() -> Self {
        Self::CeilingSurface(Default::default())
    }
}
impl AbstractFeatureTrait for AbstractConstructionSurface {
    fn feature_id(&self) -> &ID {
        match self {
            Self::CeilingSurface(v) => v.feature_id(),
            Self::FloorSurface(v) => v.feature_id(),
            Self::GroundSurface(v) => v.feature_id(),
            Self::InteriorWallSurface(v) => v.feature_id(),
            Self::OuterCeilingSurface(v) => v.feature_id(),
            Self::OuterFloorSurface(v) => v.feature_id(),
            Self::RoofSurface(v) => v.feature_id(),
            Self::WallSurface(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.identifier(),
            Self::FloorSurface(v) => v.identifier(),
            Self::GroundSurface(v) => v.identifier(),
            Self::InteriorWallSurface(v) => v.identifier(),
            Self::OuterCeilingSurface(v) => v.identifier(),
            Self::OuterFloorSurface(v) => v.identifier(),
            Self::RoofSurface(v) => v.identifier(),
            Self::WallSurface(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::CeilingSurface(v) => v.name(),
            Self::FloorSurface(v) => v.name(),
            Self::GroundSurface(v) => v.name(),
            Self::InteriorWallSurface(v) => v.name(),
            Self::OuterCeilingSurface(v) => v.name(),
            Self::OuterFloorSurface(v) => v.name(),
            Self::RoofSurface(v) => v.name(),
            Self::WallSurface(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.description(),
            Self::FloorSurface(v) => v.description(),
            Self::GroundSurface(v) => v.description(),
            Self::InteriorWallSurface(v) => v.description(),
            Self::OuterCeilingSurface(v) => v.description(),
            Self::OuterFloorSurface(v) => v.description(),
            Self::RoofSurface(v) => v.description(),
            Self::WallSurface(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractConstructionSurface {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.creation_date(),
            Self::FloorSurface(v) => v.creation_date(),
            Self::GroundSurface(v) => v.creation_date(),
            Self::InteriorWallSurface(v) => v.creation_date(),
            Self::OuterCeilingSurface(v) => v.creation_date(),
            Self::OuterFloorSurface(v) => v.creation_date(),
            Self::RoofSurface(v) => v.creation_date(),
            Self::WallSurface(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.termination_date(),
            Self::FloorSurface(v) => v.termination_date(),
            Self::GroundSurface(v) => v.termination_date(),
            Self::InteriorWallSurface(v) => v.termination_date(),
            Self::OuterCeilingSurface(v) => v.termination_date(),
            Self::OuterFloorSurface(v) => v.termination_date(),
            Self::RoofSurface(v) => v.termination_date(),
            Self::WallSurface(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.valid_from(),
            Self::FloorSurface(v) => v.valid_from(),
            Self::GroundSurface(v) => v.valid_from(),
            Self::InteriorWallSurface(v) => v.valid_from(),
            Self::OuterCeilingSurface(v) => v.valid_from(),
            Self::OuterFloorSurface(v) => v.valid_from(),
            Self::RoofSurface(v) => v.valid_from(),
            Self::WallSurface(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::CeilingSurface(v) => v.valid_to(),
            Self::FloorSurface(v) => v.valid_to(),
            Self::GroundSurface(v) => v.valid_to(),
            Self::InteriorWallSurface(v) => v.valid_to(),
            Self::OuterCeilingSurface(v) => v.valid_to(),
            Self::OuterFloorSurface(v) => v.valid_to(),
            Self::RoofSurface(v) => v.valid_to(),
            Self::WallSurface(v) => v.valid_to(),
        }
    }
}
impl AbstractCityObjectTrait for AbstractConstructionSurface {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::CeilingSurface(v) => v.relative_to_terrain(),
            Self::FloorSurface(v) => v.relative_to_terrain(),
            Self::GroundSurface(v) => v.relative_to_terrain(),
            Self::InteriorWallSurface(v) => v.relative_to_terrain(),
            Self::OuterCeilingSurface(v) => v.relative_to_terrain(),
            Self::OuterFloorSurface(v) => v.relative_to_terrain(),
            Self::RoofSurface(v) => v.relative_to_terrain(),
            Self::WallSurface(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::CeilingSurface(v) => v.relative_to_water(),
            Self::FloorSurface(v) => v.relative_to_water(),
            Self::GroundSurface(v) => v.relative_to_water(),
            Self::InteriorWallSurface(v) => v.relative_to_water(),
            Self::OuterCeilingSurface(v) => v.relative_to_water(),
            Self::OuterFloorSurface(v) => v.relative_to_water(),
            Self::RoofSurface(v) => v.relative_to_water(),
            Self::WallSurface(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::CeilingSurface(v) => v.appearance(),
            Self::FloorSurface(v) => v.appearance(),
            Self::GroundSurface(v) => v.appearance(),
            Self::InteriorWallSurface(v) => v.appearance(),
            Self::OuterCeilingSurface(v) => v.appearance(),
            Self::OuterFloorSurface(v) => v.appearance(),
            Self::RoofSurface(v) => v.appearance(),
            Self::WallSurface(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::CeilingSurface(v) => v.generalizes_to(),
            Self::FloorSurface(v) => v.generalizes_to(),
            Self::GroundSurface(v) => v.generalizes_to(),
            Self::InteriorWallSurface(v) => v.generalizes_to(),
            Self::OuterCeilingSurface(v) => v.generalizes_to(),
            Self::OuterFloorSurface(v) => v.generalizes_to(),
            Self::RoofSurface(v) => v.generalizes_to(),
            Self::WallSurface(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::CeilingSurface(v) => v.external_reference(),
            Self::FloorSurface(v) => v.external_reference(),
            Self::GroundSurface(v) => v.external_reference(),
            Self::InteriorWallSurface(v) => v.external_reference(),
            Self::OuterCeilingSurface(v) => v.external_reference(),
            Self::OuterFloorSurface(v) => v.external_reference(),
            Self::RoofSurface(v) => v.external_reference(),
            Self::WallSurface(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::CeilingSurface(v) => v.related_to(),
            Self::FloorSurface(v) => v.related_to(),
            Self::GroundSurface(v) => v.related_to(),
            Self::InteriorWallSurface(v) => v.related_to(),
            Self::OuterCeilingSurface(v) => v.related_to(),
            Self::OuterFloorSurface(v) => v.related_to(),
            Self::RoofSurface(v) => v.related_to(),
            Self::WallSurface(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::CeilingSurface(v) => v.dynamizer(),
            Self::FloorSurface(v) => v.dynamizer(),
            Self::GroundSurface(v) => v.dynamizer(),
            Self::InteriorWallSurface(v) => v.dynamizer(),
            Self::OuterCeilingSurface(v) => v.dynamizer(),
            Self::OuterFloorSurface(v) => v.dynamizer(),
            Self::RoofSurface(v) => v.dynamizer(),
            Self::WallSurface(v) => v.dynamizer(),
        }
    }
}
impl AbstractSpaceBoundaryTrait for AbstractConstructionSurface {}
impl AbstractThematicSurfaceTrait for AbstractConstructionSurface {
    fn area(&self) -> &[QualifiedArea] {
        match self {
            Self::CeilingSurface(v) => v.area(),
            Self::FloorSurface(v) => v.area(),
            Self::GroundSurface(v) => v.area(),
            Self::InteriorWallSurface(v) => v.area(),
            Self::OuterCeilingSurface(v) => v.area(),
            Self::OuterFloorSurface(v) => v.area(),
            Self::RoofSurface(v) => v.area(),
            Self::WallSurface(v) => v.area(),
        }
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::CeilingSurface(v) => v.lod3_multi_surface(),
            Self::FloorSurface(v) => v.lod3_multi_surface(),
            Self::GroundSurface(v) => v.lod3_multi_surface(),
            Self::InteriorWallSurface(v) => v.lod3_multi_surface(),
            Self::OuterCeilingSurface(v) => v.lod3_multi_surface(),
            Self::OuterFloorSurface(v) => v.lod3_multi_surface(),
            Self::RoofSurface(v) => v.lod3_multi_surface(),
            Self::WallSurface(v) => v.lod3_multi_surface(),
        }
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::CeilingSurface(v) => v.lod2_multi_surface(),
            Self::FloorSurface(v) => v.lod2_multi_surface(),
            Self::GroundSurface(v) => v.lod2_multi_surface(),
            Self::InteriorWallSurface(v) => v.lod2_multi_surface(),
            Self::OuterCeilingSurface(v) => v.lod2_multi_surface(),
            Self::OuterFloorSurface(v) => v.lod2_multi_surface(),
            Self::RoofSurface(v) => v.lod2_multi_surface(),
            Self::WallSurface(v) => v.lod2_multi_surface(),
        }
    }
    fn point_cloud(&self) -> Option<&AbstractPointCloud> {
        match self {
            Self::CeilingSurface(v) => v.point_cloud(),
            Self::FloorSurface(v) => v.point_cloud(),
            Self::GroundSurface(v) => v.point_cloud(),
            Self::InteriorWallSurface(v) => v.point_cloud(),
            Self::OuterCeilingSurface(v) => v.point_cloud(),
            Self::OuterFloorSurface(v) => v.point_cloud(),
            Self::RoofSurface(v) => v.point_cloud(),
            Self::WallSurface(v) => v.point_cloud(),
        }
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::CeilingSurface(v) => v.lod0_multi_curve(),
            Self::FloorSurface(v) => v.lod0_multi_curve(),
            Self::GroundSurface(v) => v.lod0_multi_curve(),
            Self::InteriorWallSurface(v) => v.lod0_multi_curve(),
            Self::OuterCeilingSurface(v) => v.lod0_multi_curve(),
            Self::OuterFloorSurface(v) => v.lod0_multi_curve(),
            Self::RoofSurface(v) => v.lod0_multi_curve(),
            Self::WallSurface(v) => v.lod0_multi_curve(),
        }
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::CeilingSurface(v) => v.lod0_multi_surface(),
            Self::FloorSurface(v) => v.lod0_multi_surface(),
            Self::GroundSurface(v) => v.lod0_multi_surface(),
            Self::InteriorWallSurface(v) => v.lod0_multi_surface(),
            Self::OuterCeilingSurface(v) => v.lod0_multi_surface(),
            Self::OuterFloorSurface(v) => v.lod0_multi_surface(),
            Self::RoofSurface(v) => v.lod0_multi_surface(),
            Self::WallSurface(v) => v.lod0_multi_surface(),
        }
    }
    fn lod1_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::CeilingSurface(v) => v.lod1_multi_surface(),
            Self::FloorSurface(v) => v.lod1_multi_surface(),
            Self::GroundSurface(v) => v.lod1_multi_surface(),
            Self::InteriorWallSurface(v) => v.lod1_multi_surface(),
            Self::OuterCeilingSurface(v) => v.lod1_multi_surface(),
            Self::OuterFloorSurface(v) => v.lod1_multi_surface(),
            Self::RoofSurface(v) => v.lod1_multi_surface(),
            Self::WallSurface(v) => v.lod1_multi_surface(),
        }
    }
}
impl AbstractConstructionSurfaceTrait for AbstractConstructionSurface {
    fn filling_surface(&self) -> &[AbstractFillingSurface] {
        match self {
            Self::CeilingSurface(v) => v.filling_surface(),
            Self::FloorSurface(v) => v.filling_surface(),
            Self::GroundSurface(v) => v.filling_surface(),
            Self::InteriorWallSurface(v) => v.filling_surface(),
            Self::OuterCeilingSurface(v) => v.filling_surface(),
            Self::OuterFloorSurface(v) => v.filling_surface(),
            Self::RoofSurface(v) => v.filling_surface(),
            Self::WallSurface(v) => v.filling_surface(),
        }
    }
}
impl From<CeilingSurface> for AbstractConstructionSurface {
    fn from(v: CeilingSurface) -> Self {
        Self::CeilingSurface(v)
    }
}
impl From<FloorSurface> for AbstractConstructionSurface {
    fn from(v: FloorSurface) -> Self {
        Self::FloorSurface(v)
    }
}
impl From<GroundSurface> for AbstractConstructionSurface {
    fn from(v: GroundSurface) -> Self {
        Self::GroundSurface(v)
    }
}
impl From<InteriorWallSurface> for AbstractConstructionSurface {
    fn from(v: InteriorWallSurface) -> Self {
        Self::InteriorWallSurface(v)
    }
}
impl From<OuterCeilingSurface> for AbstractConstructionSurface {
    fn from(v: OuterCeilingSurface) -> Self {
        Self::OuterCeilingSurface(v)
    }
}
impl From<OuterFloorSurface> for AbstractConstructionSurface {
    fn from(v: OuterFloorSurface) -> Self {
        Self::OuterFloorSurface(v)
    }
}
impl From<RoofSurface> for AbstractConstructionSurface {
    fn from(v: RoofSurface) -> Self {
        Self::RoofSurface(v)
    }
}
impl From<WallSurface> for AbstractConstructionSurface {
    fn from(v: WallSurface) -> Self {
        Self::WallSurface(v)
    }
}
pub trait AbstractFillingSurfaceTrait: AbstractThematicSurfaceTrait {}
#[derive(Debug, Clone)]
pub enum AbstractFillingSurface {
    DoorSurface(DoorSurface),
    WindowSurface(WindowSurface),
}
impl Default for AbstractFillingSurface {
    fn default() -> Self {
        Self::DoorSurface(Default::default())
    }
}
impl AbstractFeatureTrait for AbstractFillingSurface {
    fn feature_id(&self) -> &ID {
        match self {
            Self::DoorSurface(v) => v.feature_id(),
            Self::WindowSurface(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::DoorSurface(v) => v.identifier(),
            Self::WindowSurface(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::DoorSurface(v) => v.name(),
            Self::WindowSurface(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::DoorSurface(v) => v.description(),
            Self::WindowSurface(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractFillingSurface {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::DoorSurface(v) => v.creation_date(),
            Self::WindowSurface(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::DoorSurface(v) => v.termination_date(),
            Self::WindowSurface(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::DoorSurface(v) => v.valid_from(),
            Self::WindowSurface(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::DoorSurface(v) => v.valid_to(),
            Self::WindowSurface(v) => v.valid_to(),
        }
    }
}
impl AbstractCityObjectTrait for AbstractFillingSurface {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::DoorSurface(v) => v.relative_to_terrain(),
            Self::WindowSurface(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::DoorSurface(v) => v.relative_to_water(),
            Self::WindowSurface(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::DoorSurface(v) => v.appearance(),
            Self::WindowSurface(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::DoorSurface(v) => v.generalizes_to(),
            Self::WindowSurface(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::DoorSurface(v) => v.external_reference(),
            Self::WindowSurface(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::DoorSurface(v) => v.related_to(),
            Self::WindowSurface(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::DoorSurface(v) => v.dynamizer(),
            Self::WindowSurface(v) => v.dynamizer(),
        }
    }
}
impl AbstractSpaceBoundaryTrait for AbstractFillingSurface {}
impl AbstractThematicSurfaceTrait for AbstractFillingSurface {
    fn area(&self) -> &[QualifiedArea] {
        match self {
            Self::DoorSurface(v) => v.area(),
            Self::WindowSurface(v) => v.area(),
        }
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::DoorSurface(v) => v.lod3_multi_surface(),
            Self::WindowSurface(v) => v.lod3_multi_surface(),
        }
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::DoorSurface(v) => v.lod2_multi_surface(),
            Self::WindowSurface(v) => v.lod2_multi_surface(),
        }
    }
    fn point_cloud(&self) -> Option<&AbstractPointCloud> {
        match self {
            Self::DoorSurface(v) => v.point_cloud(),
            Self::WindowSurface(v) => v.point_cloud(),
        }
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::DoorSurface(v) => v.lod0_multi_curve(),
            Self::WindowSurface(v) => v.lod0_multi_curve(),
        }
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::DoorSurface(v) => v.lod0_multi_surface(),
            Self::WindowSurface(v) => v.lod0_multi_surface(),
        }
    }
    fn lod1_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::DoorSurface(v) => v.lod1_multi_surface(),
            Self::WindowSurface(v) => v.lod1_multi_surface(),
        }
    }
}
impl AbstractFillingSurfaceTrait for AbstractFillingSurface {}
impl From<DoorSurface> for AbstractFillingSurface {
    fn from(v: DoorSurface) -> Self {
        Self::DoorSurface(v)
    }
}
impl From<WindowSurface> for AbstractFillingSurface {
    fn from(v: WindowSurface) -> Self {
        Self::WindowSurface(v)
    }
}
pub trait AbstractConstructionTrait: AbstractOccupiedSpaceTrait {
    fn condition_of_construction(&self) -> Option<ConditionOfConstructionValue>;
    fn date_of_construction(&self) -> Option<&String>;
    fn date_of_demolition(&self) -> Option<&String>;
    fn construction_event(&self) -> &[ConstructionEvent];
    fn elevation(&self) -> &[Elevation];
    fn height(&self) -> &[Height];
    fn occupancy(&self) -> &[Occupancy];
}
#[derive(Debug, Clone)]
pub enum AbstractConstruction {
    OtherConstruction(OtherConstruction),
    Bridge(Bridge),
    BridgePart(BridgePart),
    Building(Building),
    BuildingPart(BuildingPart),
    Tunnel(Tunnel),
    TunnelPart(TunnelPart),
}
impl Default for AbstractConstruction {
    fn default() -> Self {
        Self::OtherConstruction(Default::default())
    }
}
impl AbstractFeatureTrait for AbstractConstruction {
    fn feature_id(&self) -> &ID {
        match self {
            Self::OtherConstruction(v) => v.feature_id(),
            Self::Bridge(v) => v.feature_id(),
            Self::BridgePart(v) => v.feature_id(),
            Self::Building(v) => v.feature_id(),
            Self::BuildingPart(v) => v.feature_id(),
            Self::Tunnel(v) => v.feature_id(),
            Self::TunnelPart(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::OtherConstruction(v) => v.identifier(),
            Self::Bridge(v) => v.identifier(),
            Self::BridgePart(v) => v.identifier(),
            Self::Building(v) => v.identifier(),
            Self::BuildingPart(v) => v.identifier(),
            Self::Tunnel(v) => v.identifier(),
            Self::TunnelPart(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::OtherConstruction(v) => v.name(),
            Self::Bridge(v) => v.name(),
            Self::BridgePart(v) => v.name(),
            Self::Building(v) => v.name(),
            Self::BuildingPart(v) => v.name(),
            Self::Tunnel(v) => v.name(),
            Self::TunnelPart(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::OtherConstruction(v) => v.description(),
            Self::Bridge(v) => v.description(),
            Self::BridgePart(v) => v.description(),
            Self::Building(v) => v.description(),
            Self::BuildingPart(v) => v.description(),
            Self::Tunnel(v) => v.description(),
            Self::TunnelPart(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractConstruction {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::OtherConstruction(v) => v.creation_date(),
            Self::Bridge(v) => v.creation_date(),
            Self::BridgePart(v) => v.creation_date(),
            Self::Building(v) => v.creation_date(),
            Self::BuildingPart(v) => v.creation_date(),
            Self::Tunnel(v) => v.creation_date(),
            Self::TunnelPart(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::OtherConstruction(v) => v.termination_date(),
            Self::Bridge(v) => v.termination_date(),
            Self::BridgePart(v) => v.termination_date(),
            Self::Building(v) => v.termination_date(),
            Self::BuildingPart(v) => v.termination_date(),
            Self::Tunnel(v) => v.termination_date(),
            Self::TunnelPart(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::OtherConstruction(v) => v.valid_from(),
            Self::Bridge(v) => v.valid_from(),
            Self::BridgePart(v) => v.valid_from(),
            Self::Building(v) => v.valid_from(),
            Self::BuildingPart(v) => v.valid_from(),
            Self::Tunnel(v) => v.valid_from(),
            Self::TunnelPart(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::OtherConstruction(v) => v.valid_to(),
            Self::Bridge(v) => v.valid_to(),
            Self::BridgePart(v) => v.valid_to(),
            Self::Building(v) => v.valid_to(),
            Self::BuildingPart(v) => v.valid_to(),
            Self::Tunnel(v) => v.valid_to(),
            Self::TunnelPart(v) => v.valid_to(),
        }
    }
}
impl AbstractCityObjectTrait for AbstractConstruction {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::OtherConstruction(v) => v.relative_to_terrain(),
            Self::Bridge(v) => v.relative_to_terrain(),
            Self::BridgePart(v) => v.relative_to_terrain(),
            Self::Building(v) => v.relative_to_terrain(),
            Self::BuildingPart(v) => v.relative_to_terrain(),
            Self::Tunnel(v) => v.relative_to_terrain(),
            Self::TunnelPart(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::OtherConstruction(v) => v.relative_to_water(),
            Self::Bridge(v) => v.relative_to_water(),
            Self::BridgePart(v) => v.relative_to_water(),
            Self::Building(v) => v.relative_to_water(),
            Self::BuildingPart(v) => v.relative_to_water(),
            Self::Tunnel(v) => v.relative_to_water(),
            Self::TunnelPart(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::OtherConstruction(v) => v.appearance(),
            Self::Bridge(v) => v.appearance(),
            Self::BridgePart(v) => v.appearance(),
            Self::Building(v) => v.appearance(),
            Self::BuildingPart(v) => v.appearance(),
            Self::Tunnel(v) => v.appearance(),
            Self::TunnelPart(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::OtherConstruction(v) => v.generalizes_to(),
            Self::Bridge(v) => v.generalizes_to(),
            Self::BridgePart(v) => v.generalizes_to(),
            Self::Building(v) => v.generalizes_to(),
            Self::BuildingPart(v) => v.generalizes_to(),
            Self::Tunnel(v) => v.generalizes_to(),
            Self::TunnelPart(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::OtherConstruction(v) => v.external_reference(),
            Self::Bridge(v) => v.external_reference(),
            Self::BridgePart(v) => v.external_reference(),
            Self::Building(v) => v.external_reference(),
            Self::BuildingPart(v) => v.external_reference(),
            Self::Tunnel(v) => v.external_reference(),
            Self::TunnelPart(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::OtherConstruction(v) => v.related_to(),
            Self::Bridge(v) => v.related_to(),
            Self::BridgePart(v) => v.related_to(),
            Self::Building(v) => v.related_to(),
            Self::BuildingPart(v) => v.related_to(),
            Self::Tunnel(v) => v.related_to(),
            Self::TunnelPart(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::OtherConstruction(v) => v.dynamizer(),
            Self::Bridge(v) => v.dynamizer(),
            Self::BridgePart(v) => v.dynamizer(),
            Self::Building(v) => v.dynamizer(),
            Self::BuildingPart(v) => v.dynamizer(),
            Self::Tunnel(v) => v.dynamizer(),
            Self::TunnelPart(v) => v.dynamizer(),
        }
    }
}
impl AbstractSpaceTrait for AbstractConstruction {
    fn space_type(&self) -> Option<SpaceType> {
        match self {
            Self::OtherConstruction(v) => v.space_type(),
            Self::Bridge(v) => v.space_type(),
            Self::BridgePart(v) => v.space_type(),
            Self::Building(v) => v.space_type(),
            Self::BuildingPart(v) => v.space_type(),
            Self::Tunnel(v) => v.space_type(),
            Self::TunnelPart(v) => v.space_type(),
        }
    }
    fn volume(&self) -> &[QualifiedVolume] {
        match self {
            Self::OtherConstruction(v) => v.volume(),
            Self::Bridge(v) => v.volume(),
            Self::BridgePart(v) => v.volume(),
            Self::Building(v) => v.volume(),
            Self::BuildingPart(v) => v.volume(),
            Self::Tunnel(v) => v.volume(),
            Self::TunnelPart(v) => v.volume(),
        }
    }
    fn area(&self) -> &[QualifiedArea] {
        match self {
            Self::OtherConstruction(v) => v.area(),
            Self::Bridge(v) => v.area(),
            Self::BridgePart(v) => v.area(),
            Self::Building(v) => v.area(),
            Self::BuildingPart(v) => v.area(),
            Self::Tunnel(v) => v.area(),
            Self::TunnelPart(v) => v.area(),
        }
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::OtherConstruction(v) => v.lod2_multi_curve(),
            Self::Bridge(v) => v.lod2_multi_curve(),
            Self::BridgePart(v) => v.lod2_multi_curve(),
            Self::Building(v) => v.lod2_multi_curve(),
            Self::BuildingPart(v) => v.lod2_multi_curve(),
            Self::Tunnel(v) => v.lod2_multi_curve(),
            Self::TunnelPart(v) => v.lod2_multi_curve(),
        }
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::OtherConstruction(v) => v.lod3_multi_surface(),
            Self::Bridge(v) => v.lod3_multi_surface(),
            Self::BridgePart(v) => v.lod3_multi_surface(),
            Self::Building(v) => v.lod3_multi_surface(),
            Self::BuildingPart(v) => v.lod3_multi_surface(),
            Self::Tunnel(v) => v.lod3_multi_surface(),
            Self::TunnelPart(v) => v.lod3_multi_surface(),
        }
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::OtherConstruction(v) => v.lod0_multi_surface(),
            Self::Bridge(v) => v.lod0_multi_surface(),
            Self::BridgePart(v) => v.lod0_multi_surface(),
            Self::Building(v) => v.lod0_multi_surface(),
            Self::BuildingPart(v) => v.lod0_multi_surface(),
            Self::Tunnel(v) => v.lod0_multi_surface(),
            Self::TunnelPart(v) => v.lod0_multi_surface(),
        }
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::OtherConstruction(v) => v.lod1_solid(),
            Self::Bridge(v) => v.lod1_solid(),
            Self::BridgePart(v) => v.lod1_solid(),
            Self::Building(v) => v.lod1_solid(),
            Self::BuildingPart(v) => v.lod1_solid(),
            Self::Tunnel(v) => v.lod1_solid(),
            Self::TunnelPart(v) => v.lod1_solid(),
        }
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::OtherConstruction(v) => v.lod3_solid(),
            Self::Bridge(v) => v.lod3_solid(),
            Self::BridgePart(v) => v.lod3_solid(),
            Self::Building(v) => v.lod3_solid(),
            Self::BuildingPart(v) => v.lod3_solid(),
            Self::Tunnel(v) => v.lod3_solid(),
            Self::TunnelPart(v) => v.lod3_solid(),
        }
    }
    fn boundary(&self) -> &[AbstractSpaceBoundary] {
        match self {
            Self::OtherConstruction(v) => v.boundary(),
            Self::Bridge(v) => v.boundary(),
            Self::BridgePart(v) => v.boundary(),
            Self::Building(v) => v.boundary(),
            Self::BuildingPart(v) => v.boundary(),
            Self::Tunnel(v) => v.boundary(),
            Self::TunnelPart(v) => v.boundary(),
        }
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::OtherConstruction(v) => v.lod0_multi_curve(),
            Self::Bridge(v) => v.lod0_multi_curve(),
            Self::BridgePart(v) => v.lod0_multi_curve(),
            Self::Building(v) => v.lod0_multi_curve(),
            Self::BuildingPart(v) => v.lod0_multi_curve(),
            Self::Tunnel(v) => v.lod0_multi_curve(),
            Self::TunnelPart(v) => v.lod0_multi_curve(),
        }
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::OtherConstruction(v) => v.lod2_solid(),
            Self::Bridge(v) => v.lod2_solid(),
            Self::BridgePart(v) => v.lod2_solid(),
            Self::Building(v) => v.lod2_solid(),
            Self::BuildingPart(v) => v.lod2_solid(),
            Self::Tunnel(v) => v.lod2_solid(),
            Self::TunnelPart(v) => v.lod2_solid(),
        }
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        match self {
            Self::OtherConstruction(v) => v.lod0_point(),
            Self::Bridge(v) => v.lod0_point(),
            Self::BridgePart(v) => v.lod0_point(),
            Self::Building(v) => v.lod0_point(),
            Self::BuildingPart(v) => v.lod0_point(),
            Self::Tunnel(v) => v.lod0_point(),
            Self::TunnelPart(v) => v.lod0_point(),
        }
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::OtherConstruction(v) => v.lod3_multi_curve(),
            Self::Bridge(v) => v.lod3_multi_curve(),
            Self::BridgePart(v) => v.lod3_multi_curve(),
            Self::Building(v) => v.lod3_multi_curve(),
            Self::BuildingPart(v) => v.lod3_multi_curve(),
            Self::Tunnel(v) => v.lod3_multi_curve(),
            Self::TunnelPart(v) => v.lod3_multi_curve(),
        }
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::OtherConstruction(v) => v.lod2_multi_surface(),
            Self::Bridge(v) => v.lod2_multi_surface(),
            Self::BridgePart(v) => v.lod2_multi_surface(),
            Self::Building(v) => v.lod2_multi_surface(),
            Self::BuildingPart(v) => v.lod2_multi_surface(),
            Self::Tunnel(v) => v.lod2_multi_surface(),
            Self::TunnelPart(v) => v.lod2_multi_surface(),
        }
    }
}
impl AbstractPhysicalSpaceTrait for AbstractConstruction {
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::OtherConstruction(v) => v.lod3_terrain_intersection_curve(),
            Self::Bridge(v) => v.lod3_terrain_intersection_curve(),
            Self::BridgePart(v) => v.lod3_terrain_intersection_curve(),
            Self::Building(v) => v.lod3_terrain_intersection_curve(),
            Self::BuildingPart(v) => v.lod3_terrain_intersection_curve(),
            Self::Tunnel(v) => v.lod3_terrain_intersection_curve(),
            Self::TunnelPart(v) => v.lod3_terrain_intersection_curve(),
        }
    }
    fn point_cloud(&self) -> Option<&AbstractPointCloud> {
        match self {
            Self::OtherConstruction(v) => v.point_cloud(),
            Self::Bridge(v) => v.point_cloud(),
            Self::BridgePart(v) => v.point_cloud(),
            Self::Building(v) => v.point_cloud(),
            Self::BuildingPart(v) => v.point_cloud(),
            Self::Tunnel(v) => v.point_cloud(),
            Self::TunnelPart(v) => v.point_cloud(),
        }
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::OtherConstruction(v) => v.lod1_terrain_intersection_curve(),
            Self::Bridge(v) => v.lod1_terrain_intersection_curve(),
            Self::BridgePart(v) => v.lod1_terrain_intersection_curve(),
            Self::Building(v) => v.lod1_terrain_intersection_curve(),
            Self::BuildingPart(v) => v.lod1_terrain_intersection_curve(),
            Self::Tunnel(v) => v.lod1_terrain_intersection_curve(),
            Self::TunnelPart(v) => v.lod1_terrain_intersection_curve(),
        }
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::OtherConstruction(v) => v.lod2_terrain_intersection_curve(),
            Self::Bridge(v) => v.lod2_terrain_intersection_curve(),
            Self::BridgePart(v) => v.lod2_terrain_intersection_curve(),
            Self::Building(v) => v.lod2_terrain_intersection_curve(),
            Self::BuildingPart(v) => v.lod2_terrain_intersection_curve(),
            Self::Tunnel(v) => v.lod2_terrain_intersection_curve(),
            Self::TunnelPart(v) => v.lod2_terrain_intersection_curve(),
        }
    }
}
impl AbstractOccupiedSpaceTrait for AbstractConstruction {
    fn lod3_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::OtherConstruction(v) => v.lod3_implicit_representation(),
            Self::Bridge(v) => v.lod3_implicit_representation(),
            Self::BridgePart(v) => v.lod3_implicit_representation(),
            Self::Building(v) => v.lod3_implicit_representation(),
            Self::BuildingPart(v) => v.lod3_implicit_representation(),
            Self::Tunnel(v) => v.lod3_implicit_representation(),
            Self::TunnelPart(v) => v.lod3_implicit_representation(),
        }
    }
    fn lod2_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::OtherConstruction(v) => v.lod2_implicit_representation(),
            Self::Bridge(v) => v.lod2_implicit_representation(),
            Self::BridgePart(v) => v.lod2_implicit_representation(),
            Self::Building(v) => v.lod2_implicit_representation(),
            Self::BuildingPart(v) => v.lod2_implicit_representation(),
            Self::Tunnel(v) => v.lod2_implicit_representation(),
            Self::TunnelPart(v) => v.lod2_implicit_representation(),
        }
    }
    fn lod1_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::OtherConstruction(v) => v.lod1_implicit_representation(),
            Self::Bridge(v) => v.lod1_implicit_representation(),
            Self::BridgePart(v) => v.lod1_implicit_representation(),
            Self::Building(v) => v.lod1_implicit_representation(),
            Self::BuildingPart(v) => v.lod1_implicit_representation(),
            Self::Tunnel(v) => v.lod1_implicit_representation(),
            Self::TunnelPart(v) => v.lod1_implicit_representation(),
        }
    }
}
impl AbstractConstructionTrait for AbstractConstruction {
    fn condition_of_construction(&self) -> Option<ConditionOfConstructionValue> {
        match self {
            Self::OtherConstruction(v) => v.condition_of_construction(),
            Self::Bridge(v) => v.condition_of_construction(),
            Self::BridgePart(v) => v.condition_of_construction(),
            Self::Building(v) => v.condition_of_construction(),
            Self::BuildingPart(v) => v.condition_of_construction(),
            Self::Tunnel(v) => v.condition_of_construction(),
            Self::TunnelPart(v) => v.condition_of_construction(),
        }
    }
    fn date_of_construction(&self) -> Option<&String> {
        match self {
            Self::OtherConstruction(v) => v.date_of_construction(),
            Self::Bridge(v) => v.date_of_construction(),
            Self::BridgePart(v) => v.date_of_construction(),
            Self::Building(v) => v.date_of_construction(),
            Self::BuildingPart(v) => v.date_of_construction(),
            Self::Tunnel(v) => v.date_of_construction(),
            Self::TunnelPart(v) => v.date_of_construction(),
        }
    }
    fn date_of_demolition(&self) -> Option<&String> {
        match self {
            Self::OtherConstruction(v) => v.date_of_demolition(),
            Self::Bridge(v) => v.date_of_demolition(),
            Self::BridgePart(v) => v.date_of_demolition(),
            Self::Building(v) => v.date_of_demolition(),
            Self::BuildingPart(v) => v.date_of_demolition(),
            Self::Tunnel(v) => v.date_of_demolition(),
            Self::TunnelPart(v) => v.date_of_demolition(),
        }
    }
    fn construction_event(&self) -> &[ConstructionEvent] {
        match self {
            Self::OtherConstruction(v) => v.construction_event(),
            Self::Bridge(v) => v.construction_event(),
            Self::BridgePart(v) => v.construction_event(),
            Self::Building(v) => v.construction_event(),
            Self::BuildingPart(v) => v.construction_event(),
            Self::Tunnel(v) => v.construction_event(),
            Self::TunnelPart(v) => v.construction_event(),
        }
    }
    fn elevation(&self) -> &[Elevation] {
        match self {
            Self::OtherConstruction(v) => v.elevation(),
            Self::Bridge(v) => v.elevation(),
            Self::BridgePart(v) => v.elevation(),
            Self::Building(v) => v.elevation(),
            Self::BuildingPart(v) => v.elevation(),
            Self::Tunnel(v) => v.elevation(),
            Self::TunnelPart(v) => v.elevation(),
        }
    }
    fn height(&self) -> &[Height] {
        match self {
            Self::OtherConstruction(v) => v.height(),
            Self::Bridge(v) => v.height(),
            Self::BridgePart(v) => v.height(),
            Self::Building(v) => v.height(),
            Self::BuildingPart(v) => v.height(),
            Self::Tunnel(v) => v.height(),
            Self::TunnelPart(v) => v.height(),
        }
    }
    fn occupancy(&self) -> &[Occupancy] {
        match self {
            Self::OtherConstruction(v) => v.occupancy(),
            Self::Bridge(v) => v.occupancy(),
            Self::BridgePart(v) => v.occupancy(),
            Self::Building(v) => v.occupancy(),
            Self::BuildingPart(v) => v.occupancy(),
            Self::Tunnel(v) => v.occupancy(),
            Self::TunnelPart(v) => v.occupancy(),
        }
    }
}
impl From<OtherConstruction> for AbstractConstruction {
    fn from(v: OtherConstruction) -> Self {
        Self::OtherConstruction(v)
    }
}
impl From<Bridge> for AbstractConstruction {
    fn from(v: Bridge) -> Self {
        Self::Bridge(v)
    }
}
impl From<BridgePart> for AbstractConstruction {
    fn from(v: BridgePart) -> Self {
        Self::BridgePart(v)
    }
}
impl From<Building> for AbstractConstruction {
    fn from(v: Building) -> Self {
        Self::Building(v)
    }
}
impl From<BuildingPart> for AbstractConstruction {
    fn from(v: BuildingPart) -> Self {
        Self::BuildingPart(v)
    }
}
impl From<Tunnel> for AbstractConstruction {
    fn from(v: Tunnel) -> Self {
        Self::Tunnel(v)
    }
}
impl From<TunnelPart> for AbstractConstruction {
    fn from(v: TunnelPart) -> Self {
        Self::TunnelPart(v)
    }
}
pub trait AbstractConstructiveElementTrait: AbstractOccupiedSpaceTrait {
    fn is_structural_element(&self) -> Option<bool>;
    fn filling(&self) -> &[AbstractFillingElement];
}
#[derive(Debug, Clone)]
pub enum AbstractConstructiveElement {
    BridgeConstructiveElement(BridgeConstructiveElement),
    BuildingConstructiveElement(BuildingConstructiveElement),
    TunnelConstructiveElement(TunnelConstructiveElement),
}
impl Default for AbstractConstructiveElement {
    fn default() -> Self {
        Self::BridgeConstructiveElement(Default::default())
    }
}
impl AbstractFeatureTrait for AbstractConstructiveElement {
    fn feature_id(&self) -> &ID {
        match self {
            Self::BridgeConstructiveElement(v) => v.feature_id(),
            Self::BuildingConstructiveElement(v) => v.feature_id(),
            Self::TunnelConstructiveElement(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::BridgeConstructiveElement(v) => v.identifier(),
            Self::BuildingConstructiveElement(v) => v.identifier(),
            Self::TunnelConstructiveElement(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::BridgeConstructiveElement(v) => v.name(),
            Self::BuildingConstructiveElement(v) => v.name(),
            Self::TunnelConstructiveElement(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::BridgeConstructiveElement(v) => v.description(),
            Self::BuildingConstructiveElement(v) => v.description(),
            Self::TunnelConstructiveElement(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractConstructiveElement {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::BridgeConstructiveElement(v) => v.creation_date(),
            Self::BuildingConstructiveElement(v) => v.creation_date(),
            Self::TunnelConstructiveElement(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::BridgeConstructiveElement(v) => v.termination_date(),
            Self::BuildingConstructiveElement(v) => v.termination_date(),
            Self::TunnelConstructiveElement(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::BridgeConstructiveElement(v) => v.valid_from(),
            Self::BuildingConstructiveElement(v) => v.valid_from(),
            Self::TunnelConstructiveElement(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::BridgeConstructiveElement(v) => v.valid_to(),
            Self::BuildingConstructiveElement(v) => v.valid_to(),
            Self::TunnelConstructiveElement(v) => v.valid_to(),
        }
    }
}
impl AbstractCityObjectTrait for AbstractConstructiveElement {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::BridgeConstructiveElement(v) => v.relative_to_terrain(),
            Self::BuildingConstructiveElement(v) => v.relative_to_terrain(),
            Self::TunnelConstructiveElement(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::BridgeConstructiveElement(v) => v.relative_to_water(),
            Self::BuildingConstructiveElement(v) => v.relative_to_water(),
            Self::TunnelConstructiveElement(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::BridgeConstructiveElement(v) => v.appearance(),
            Self::BuildingConstructiveElement(v) => v.appearance(),
            Self::TunnelConstructiveElement(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::BridgeConstructiveElement(v) => v.generalizes_to(),
            Self::BuildingConstructiveElement(v) => v.generalizes_to(),
            Self::TunnelConstructiveElement(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::BridgeConstructiveElement(v) => v.external_reference(),
            Self::BuildingConstructiveElement(v) => v.external_reference(),
            Self::TunnelConstructiveElement(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::BridgeConstructiveElement(v) => v.related_to(),
            Self::BuildingConstructiveElement(v) => v.related_to(),
            Self::TunnelConstructiveElement(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::BridgeConstructiveElement(v) => v.dynamizer(),
            Self::BuildingConstructiveElement(v) => v.dynamizer(),
            Self::TunnelConstructiveElement(v) => v.dynamizer(),
        }
    }
}
impl AbstractSpaceTrait for AbstractConstructiveElement {
    fn space_type(&self) -> Option<SpaceType> {
        match self {
            Self::BridgeConstructiveElement(v) => v.space_type(),
            Self::BuildingConstructiveElement(v) => v.space_type(),
            Self::TunnelConstructiveElement(v) => v.space_type(),
        }
    }
    fn volume(&self) -> &[QualifiedVolume] {
        match self {
            Self::BridgeConstructiveElement(v) => v.volume(),
            Self::BuildingConstructiveElement(v) => v.volume(),
            Self::TunnelConstructiveElement(v) => v.volume(),
        }
    }
    fn area(&self) -> &[QualifiedArea] {
        match self {
            Self::BridgeConstructiveElement(v) => v.area(),
            Self::BuildingConstructiveElement(v) => v.area(),
            Self::TunnelConstructiveElement(v) => v.area(),
        }
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeConstructiveElement(v) => v.lod2_multi_curve(),
            Self::BuildingConstructiveElement(v) => v.lod2_multi_curve(),
            Self::TunnelConstructiveElement(v) => v.lod2_multi_curve(),
        }
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::BridgeConstructiveElement(v) => v.lod3_multi_surface(),
            Self::BuildingConstructiveElement(v) => v.lod3_multi_surface(),
            Self::TunnelConstructiveElement(v) => v.lod3_multi_surface(),
        }
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::BridgeConstructiveElement(v) => v.lod0_multi_surface(),
            Self::BuildingConstructiveElement(v) => v.lod0_multi_surface(),
            Self::TunnelConstructiveElement(v) => v.lod0_multi_surface(),
        }
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::BridgeConstructiveElement(v) => v.lod1_solid(),
            Self::BuildingConstructiveElement(v) => v.lod1_solid(),
            Self::TunnelConstructiveElement(v) => v.lod1_solid(),
        }
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::BridgeConstructiveElement(v) => v.lod3_solid(),
            Self::BuildingConstructiveElement(v) => v.lod3_solid(),
            Self::TunnelConstructiveElement(v) => v.lod3_solid(),
        }
    }
    fn boundary(&self) -> &[AbstractSpaceBoundary] {
        match self {
            Self::BridgeConstructiveElement(v) => v.boundary(),
            Self::BuildingConstructiveElement(v) => v.boundary(),
            Self::TunnelConstructiveElement(v) => v.boundary(),
        }
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeConstructiveElement(v) => v.lod0_multi_curve(),
            Self::BuildingConstructiveElement(v) => v.lod0_multi_curve(),
            Self::TunnelConstructiveElement(v) => v.lod0_multi_curve(),
        }
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::BridgeConstructiveElement(v) => v.lod2_solid(),
            Self::BuildingConstructiveElement(v) => v.lod2_solid(),
            Self::TunnelConstructiveElement(v) => v.lod2_solid(),
        }
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        match self {
            Self::BridgeConstructiveElement(v) => v.lod0_point(),
            Self::BuildingConstructiveElement(v) => v.lod0_point(),
            Self::TunnelConstructiveElement(v) => v.lod0_point(),
        }
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeConstructiveElement(v) => v.lod3_multi_curve(),
            Self::BuildingConstructiveElement(v) => v.lod3_multi_curve(),
            Self::TunnelConstructiveElement(v) => v.lod3_multi_curve(),
        }
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::BridgeConstructiveElement(v) => v.lod2_multi_surface(),
            Self::BuildingConstructiveElement(v) => v.lod2_multi_surface(),
            Self::TunnelConstructiveElement(v) => v.lod2_multi_surface(),
        }
    }
}
impl AbstractPhysicalSpaceTrait for AbstractConstructiveElement {
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeConstructiveElement(v) => v.lod3_terrain_intersection_curve(),
            Self::BuildingConstructiveElement(v) => v.lod3_terrain_intersection_curve(),
            Self::TunnelConstructiveElement(v) => v.lod3_terrain_intersection_curve(),
        }
    }
    fn point_cloud(&self) -> Option<&AbstractPointCloud> {
        match self {
            Self::BridgeConstructiveElement(v) => v.point_cloud(),
            Self::BuildingConstructiveElement(v) => v.point_cloud(),
            Self::TunnelConstructiveElement(v) => v.point_cloud(),
        }
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeConstructiveElement(v) => v.lod1_terrain_intersection_curve(),
            Self::BuildingConstructiveElement(v) => v.lod1_terrain_intersection_curve(),
            Self::TunnelConstructiveElement(v) => v.lod1_terrain_intersection_curve(),
        }
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeConstructiveElement(v) => v.lod2_terrain_intersection_curve(),
            Self::BuildingConstructiveElement(v) => v.lod2_terrain_intersection_curve(),
            Self::TunnelConstructiveElement(v) => v.lod2_terrain_intersection_curve(),
        }
    }
}
impl AbstractOccupiedSpaceTrait for AbstractConstructiveElement {
    fn lod3_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::BridgeConstructiveElement(v) => v.lod3_implicit_representation(),
            Self::BuildingConstructiveElement(v) => v.lod3_implicit_representation(),
            Self::TunnelConstructiveElement(v) => v.lod3_implicit_representation(),
        }
    }
    fn lod2_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::BridgeConstructiveElement(v) => v.lod2_implicit_representation(),
            Self::BuildingConstructiveElement(v) => v.lod2_implicit_representation(),
            Self::TunnelConstructiveElement(v) => v.lod2_implicit_representation(),
        }
    }
    fn lod1_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::BridgeConstructiveElement(v) => v.lod1_implicit_representation(),
            Self::BuildingConstructiveElement(v) => v.lod1_implicit_representation(),
            Self::TunnelConstructiveElement(v) => v.lod1_implicit_representation(),
        }
    }
}
impl AbstractConstructiveElementTrait for AbstractConstructiveElement {
    fn is_structural_element(&self) -> Option<bool> {
        match self {
            Self::BridgeConstructiveElement(v) => v.is_structural_element(),
            Self::BuildingConstructiveElement(v) => v.is_structural_element(),
            Self::TunnelConstructiveElement(v) => v.is_structural_element(),
        }
    }
    fn filling(&self) -> &[AbstractFillingElement] {
        match self {
            Self::BridgeConstructiveElement(v) => v.filling(),
            Self::BuildingConstructiveElement(v) => v.filling(),
            Self::TunnelConstructiveElement(v) => v.filling(),
        }
    }
}
impl From<BridgeConstructiveElement> for AbstractConstructiveElement {
    fn from(v: BridgeConstructiveElement) -> Self {
        Self::BridgeConstructiveElement(v)
    }
}
impl From<BuildingConstructiveElement> for AbstractConstructiveElement {
    fn from(v: BuildingConstructiveElement) -> Self {
        Self::BuildingConstructiveElement(v)
    }
}
impl From<TunnelConstructiveElement> for AbstractConstructiveElement {
    fn from(v: TunnelConstructiveElement) -> Self {
        Self::TunnelConstructiveElement(v)
    }
}
pub trait AbstractFillingElementTrait: AbstractOccupiedSpaceTrait {}
#[derive(Debug, Clone)]
pub enum AbstractFillingElement {
    Door(Door),
    Window(Window),
}
impl Default for AbstractFillingElement {
    fn default() -> Self {
        Self::Door(Default::default())
    }
}
impl AbstractFeatureTrait for AbstractFillingElement {
    fn feature_id(&self) -> &ID {
        match self {
            Self::Door(v) => v.feature_id(),
            Self::Window(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.identifier(),
            Self::Window(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::Door(v) => v.name(),
            Self::Window(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.description(),
            Self::Window(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractFillingElement {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.creation_date(),
            Self::Window(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.termination_date(),
            Self::Window(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.valid_from(),
            Self::Window(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::Door(v) => v.valid_to(),
            Self::Window(v) => v.valid_to(),
        }
    }
}
impl AbstractCityObjectTrait for AbstractFillingElement {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::Door(v) => v.relative_to_terrain(),
            Self::Window(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::Door(v) => v.relative_to_water(),
            Self::Window(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::Door(v) => v.appearance(),
            Self::Window(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::Door(v) => v.generalizes_to(),
            Self::Window(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::Door(v) => v.external_reference(),
            Self::Window(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::Door(v) => v.related_to(),
            Self::Window(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::Door(v) => v.dynamizer(),
            Self::Window(v) => v.dynamizer(),
        }
    }
}
impl AbstractSpaceTrait for AbstractFillingElement {
    fn space_type(&self) -> Option<SpaceType> {
        match self {
            Self::Door(v) => v.space_type(),
            Self::Window(v) => v.space_type(),
        }
    }
    fn volume(&self) -> &[QualifiedVolume] {
        match self {
            Self::Door(v) => v.volume(),
            Self::Window(v) => v.volume(),
        }
    }
    fn area(&self) -> &[QualifiedArea] {
        match self {
            Self::Door(v) => v.area(),
            Self::Window(v) => v.area(),
        }
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod2_multi_curve(),
            Self::Window(v) => v.lod2_multi_curve(),
        }
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::Door(v) => v.lod3_multi_surface(),
            Self::Window(v) => v.lod3_multi_surface(),
        }
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::Door(v) => v.lod0_multi_surface(),
            Self::Window(v) => v.lod0_multi_surface(),
        }
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::Door(v) => v.lod1_solid(),
            Self::Window(v) => v.lod1_solid(),
        }
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::Door(v) => v.lod3_solid(),
            Self::Window(v) => v.lod3_solid(),
        }
    }
    fn boundary(&self) -> &[AbstractSpaceBoundary] {
        match self {
            Self::Door(v) => v.boundary(),
            Self::Window(v) => v.boundary(),
        }
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod0_multi_curve(),
            Self::Window(v) => v.lod0_multi_curve(),
        }
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::Door(v) => v.lod2_solid(),
            Self::Window(v) => v.lod2_solid(),
        }
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        match self {
            Self::Door(v) => v.lod0_point(),
            Self::Window(v) => v.lod0_point(),
        }
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod3_multi_curve(),
            Self::Window(v) => v.lod3_multi_curve(),
        }
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::Door(v) => v.lod2_multi_surface(),
            Self::Window(v) => v.lod2_multi_surface(),
        }
    }
}
impl AbstractPhysicalSpaceTrait for AbstractFillingElement {
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod3_terrain_intersection_curve(),
            Self::Window(v) => v.lod3_terrain_intersection_curve(),
        }
    }
    fn point_cloud(&self) -> Option<&AbstractPointCloud> {
        match self {
            Self::Door(v) => v.point_cloud(),
            Self::Window(v) => v.point_cloud(),
        }
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod1_terrain_intersection_curve(),
            Self::Window(v) => v.lod1_terrain_intersection_curve(),
        }
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Door(v) => v.lod2_terrain_intersection_curve(),
            Self::Window(v) => v.lod2_terrain_intersection_curve(),
        }
    }
}
impl AbstractOccupiedSpaceTrait for AbstractFillingElement {
    fn lod3_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::Door(v) => v.lod3_implicit_representation(),
            Self::Window(v) => v.lod3_implicit_representation(),
        }
    }
    fn lod2_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::Door(v) => v.lod2_implicit_representation(),
            Self::Window(v) => v.lod2_implicit_representation(),
        }
    }
    fn lod1_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::Door(v) => v.lod1_implicit_representation(),
            Self::Window(v) => v.lod1_implicit_representation(),
        }
    }
}
impl AbstractFillingElementTrait for AbstractFillingElement {}
impl From<Door> for AbstractFillingElement {
    fn from(v: Door) -> Self {
        Self::Door(v)
    }
}
impl From<Window> for AbstractFillingElement {
    fn from(v: Window) -> Self {
        Self::Window(v)
    }
}
pub trait AbstractFurnitureTrait: AbstractOccupiedSpaceTrait {}
#[derive(Debug, Clone)]
pub enum AbstractFurniture {
    BridgeFurniture(BridgeFurniture),
    BuildingFurniture(BuildingFurniture),
    TunnelFurniture(TunnelFurniture),
}
impl Default for AbstractFurniture {
    fn default() -> Self {
        Self::BridgeFurniture(Default::default())
    }
}
impl AbstractFeatureTrait for AbstractFurniture {
    fn feature_id(&self) -> &ID {
        match self {
            Self::BridgeFurniture(v) => v.feature_id(),
            Self::BuildingFurniture(v) => v.feature_id(),
            Self::TunnelFurniture(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::BridgeFurniture(v) => v.identifier(),
            Self::BuildingFurniture(v) => v.identifier(),
            Self::TunnelFurniture(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::BridgeFurniture(v) => v.name(),
            Self::BuildingFurniture(v) => v.name(),
            Self::TunnelFurniture(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::BridgeFurniture(v) => v.description(),
            Self::BuildingFurniture(v) => v.description(),
            Self::TunnelFurniture(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractFurniture {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::BridgeFurniture(v) => v.creation_date(),
            Self::BuildingFurniture(v) => v.creation_date(),
            Self::TunnelFurniture(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::BridgeFurniture(v) => v.termination_date(),
            Self::BuildingFurniture(v) => v.termination_date(),
            Self::TunnelFurniture(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::BridgeFurniture(v) => v.valid_from(),
            Self::BuildingFurniture(v) => v.valid_from(),
            Self::TunnelFurniture(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::BridgeFurniture(v) => v.valid_to(),
            Self::BuildingFurniture(v) => v.valid_to(),
            Self::TunnelFurniture(v) => v.valid_to(),
        }
    }
}
impl AbstractCityObjectTrait for AbstractFurniture {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::BridgeFurniture(v) => v.relative_to_terrain(),
            Self::BuildingFurniture(v) => v.relative_to_terrain(),
            Self::TunnelFurniture(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::BridgeFurniture(v) => v.relative_to_water(),
            Self::BuildingFurniture(v) => v.relative_to_water(),
            Self::TunnelFurniture(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::BridgeFurniture(v) => v.appearance(),
            Self::BuildingFurniture(v) => v.appearance(),
            Self::TunnelFurniture(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::BridgeFurniture(v) => v.generalizes_to(),
            Self::BuildingFurniture(v) => v.generalizes_to(),
            Self::TunnelFurniture(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::BridgeFurniture(v) => v.external_reference(),
            Self::BuildingFurniture(v) => v.external_reference(),
            Self::TunnelFurniture(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::BridgeFurniture(v) => v.related_to(),
            Self::BuildingFurniture(v) => v.related_to(),
            Self::TunnelFurniture(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::BridgeFurniture(v) => v.dynamizer(),
            Self::BuildingFurniture(v) => v.dynamizer(),
            Self::TunnelFurniture(v) => v.dynamizer(),
        }
    }
}
impl AbstractSpaceTrait for AbstractFurniture {
    fn space_type(&self) -> Option<SpaceType> {
        match self {
            Self::BridgeFurniture(v) => v.space_type(),
            Self::BuildingFurniture(v) => v.space_type(),
            Self::TunnelFurniture(v) => v.space_type(),
        }
    }
    fn volume(&self) -> &[QualifiedVolume] {
        match self {
            Self::BridgeFurniture(v) => v.volume(),
            Self::BuildingFurniture(v) => v.volume(),
            Self::TunnelFurniture(v) => v.volume(),
        }
    }
    fn area(&self) -> &[QualifiedArea] {
        match self {
            Self::BridgeFurniture(v) => v.area(),
            Self::BuildingFurniture(v) => v.area(),
            Self::TunnelFurniture(v) => v.area(),
        }
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeFurniture(v) => v.lod2_multi_curve(),
            Self::BuildingFurniture(v) => v.lod2_multi_curve(),
            Self::TunnelFurniture(v) => v.lod2_multi_curve(),
        }
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::BridgeFurniture(v) => v.lod3_multi_surface(),
            Self::BuildingFurniture(v) => v.lod3_multi_surface(),
            Self::TunnelFurniture(v) => v.lod3_multi_surface(),
        }
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::BridgeFurniture(v) => v.lod0_multi_surface(),
            Self::BuildingFurniture(v) => v.lod0_multi_surface(),
            Self::TunnelFurniture(v) => v.lod0_multi_surface(),
        }
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::BridgeFurniture(v) => v.lod1_solid(),
            Self::BuildingFurniture(v) => v.lod1_solid(),
            Self::TunnelFurniture(v) => v.lod1_solid(),
        }
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::BridgeFurniture(v) => v.lod3_solid(),
            Self::BuildingFurniture(v) => v.lod3_solid(),
            Self::TunnelFurniture(v) => v.lod3_solid(),
        }
    }
    fn boundary(&self) -> &[AbstractSpaceBoundary] {
        match self {
            Self::BridgeFurniture(v) => v.boundary(),
            Self::BuildingFurniture(v) => v.boundary(),
            Self::TunnelFurniture(v) => v.boundary(),
        }
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeFurniture(v) => v.lod0_multi_curve(),
            Self::BuildingFurniture(v) => v.lod0_multi_curve(),
            Self::TunnelFurniture(v) => v.lod0_multi_curve(),
        }
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::BridgeFurniture(v) => v.lod2_solid(),
            Self::BuildingFurniture(v) => v.lod2_solid(),
            Self::TunnelFurniture(v) => v.lod2_solid(),
        }
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        match self {
            Self::BridgeFurniture(v) => v.lod0_point(),
            Self::BuildingFurniture(v) => v.lod0_point(),
            Self::TunnelFurniture(v) => v.lod0_point(),
        }
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeFurniture(v) => v.lod3_multi_curve(),
            Self::BuildingFurniture(v) => v.lod3_multi_curve(),
            Self::TunnelFurniture(v) => v.lod3_multi_curve(),
        }
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::BridgeFurniture(v) => v.lod2_multi_surface(),
            Self::BuildingFurniture(v) => v.lod2_multi_surface(),
            Self::TunnelFurniture(v) => v.lod2_multi_surface(),
        }
    }
}
impl AbstractPhysicalSpaceTrait for AbstractFurniture {
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeFurniture(v) => v.lod3_terrain_intersection_curve(),
            Self::BuildingFurniture(v) => v.lod3_terrain_intersection_curve(),
            Self::TunnelFurniture(v) => v.lod3_terrain_intersection_curve(),
        }
    }
    fn point_cloud(&self) -> Option<&AbstractPointCloud> {
        match self {
            Self::BridgeFurniture(v) => v.point_cloud(),
            Self::BuildingFurniture(v) => v.point_cloud(),
            Self::TunnelFurniture(v) => v.point_cloud(),
        }
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeFurniture(v) => v.lod1_terrain_intersection_curve(),
            Self::BuildingFurniture(v) => v.lod1_terrain_intersection_curve(),
            Self::TunnelFurniture(v) => v.lod1_terrain_intersection_curve(),
        }
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeFurniture(v) => v.lod2_terrain_intersection_curve(),
            Self::BuildingFurniture(v) => v.lod2_terrain_intersection_curve(),
            Self::TunnelFurniture(v) => v.lod2_terrain_intersection_curve(),
        }
    }
}
impl AbstractOccupiedSpaceTrait for AbstractFurniture {
    fn lod3_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::BridgeFurniture(v) => v.lod3_implicit_representation(),
            Self::BuildingFurniture(v) => v.lod3_implicit_representation(),
            Self::TunnelFurniture(v) => v.lod3_implicit_representation(),
        }
    }
    fn lod2_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::BridgeFurniture(v) => v.lod2_implicit_representation(),
            Self::BuildingFurniture(v) => v.lod2_implicit_representation(),
            Self::TunnelFurniture(v) => v.lod2_implicit_representation(),
        }
    }
    fn lod1_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::BridgeFurniture(v) => v.lod1_implicit_representation(),
            Self::BuildingFurniture(v) => v.lod1_implicit_representation(),
            Self::TunnelFurniture(v) => v.lod1_implicit_representation(),
        }
    }
}
impl AbstractFurnitureTrait for AbstractFurniture {}
impl From<BridgeFurniture> for AbstractFurniture {
    fn from(v: BridgeFurniture) -> Self {
        Self::BridgeFurniture(v)
    }
}
impl From<BuildingFurniture> for AbstractFurniture {
    fn from(v: BuildingFurniture) -> Self {
        Self::BuildingFurniture(v)
    }
}
impl From<TunnelFurniture> for AbstractFurniture {
    fn from(v: TunnelFurniture) -> Self {
        Self::TunnelFurniture(v)
    }
}
pub trait AbstractInstallationTrait: AbstractOccupiedSpaceTrait {
    fn relation_to_construction(&self) -> Option<RelationToConstruction>;
}
#[derive(Debug, Clone)]
pub enum AbstractInstallation {
    BridgeInstallation(BridgeInstallation),
    BuildingInstallation(BuildingInstallation),
    TunnelInstallation(TunnelInstallation),
}
impl Default for AbstractInstallation {
    fn default() -> Self {
        Self::BridgeInstallation(Default::default())
    }
}
impl AbstractFeatureTrait for AbstractInstallation {
    fn feature_id(&self) -> &ID {
        match self {
            Self::BridgeInstallation(v) => v.feature_id(),
            Self::BuildingInstallation(v) => v.feature_id(),
            Self::TunnelInstallation(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::BridgeInstallation(v) => v.identifier(),
            Self::BuildingInstallation(v) => v.identifier(),
            Self::TunnelInstallation(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::BridgeInstallation(v) => v.name(),
            Self::BuildingInstallation(v) => v.name(),
            Self::TunnelInstallation(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::BridgeInstallation(v) => v.description(),
            Self::BuildingInstallation(v) => v.description(),
            Self::TunnelInstallation(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractInstallation {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::BridgeInstallation(v) => v.creation_date(),
            Self::BuildingInstallation(v) => v.creation_date(),
            Self::TunnelInstallation(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::BridgeInstallation(v) => v.termination_date(),
            Self::BuildingInstallation(v) => v.termination_date(),
            Self::TunnelInstallation(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::BridgeInstallation(v) => v.valid_from(),
            Self::BuildingInstallation(v) => v.valid_from(),
            Self::TunnelInstallation(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::BridgeInstallation(v) => v.valid_to(),
            Self::BuildingInstallation(v) => v.valid_to(),
            Self::TunnelInstallation(v) => v.valid_to(),
        }
    }
}
impl AbstractCityObjectTrait for AbstractInstallation {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::BridgeInstallation(v) => v.relative_to_terrain(),
            Self::BuildingInstallation(v) => v.relative_to_terrain(),
            Self::TunnelInstallation(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::BridgeInstallation(v) => v.relative_to_water(),
            Self::BuildingInstallation(v) => v.relative_to_water(),
            Self::TunnelInstallation(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::BridgeInstallation(v) => v.appearance(),
            Self::BuildingInstallation(v) => v.appearance(),
            Self::TunnelInstallation(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::BridgeInstallation(v) => v.generalizes_to(),
            Self::BuildingInstallation(v) => v.generalizes_to(),
            Self::TunnelInstallation(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::BridgeInstallation(v) => v.external_reference(),
            Self::BuildingInstallation(v) => v.external_reference(),
            Self::TunnelInstallation(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::BridgeInstallation(v) => v.related_to(),
            Self::BuildingInstallation(v) => v.related_to(),
            Self::TunnelInstallation(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::BridgeInstallation(v) => v.dynamizer(),
            Self::BuildingInstallation(v) => v.dynamizer(),
            Self::TunnelInstallation(v) => v.dynamizer(),
        }
    }
}
impl AbstractSpaceTrait for AbstractInstallation {
    fn space_type(&self) -> Option<SpaceType> {
        match self {
            Self::BridgeInstallation(v) => v.space_type(),
            Self::BuildingInstallation(v) => v.space_type(),
            Self::TunnelInstallation(v) => v.space_type(),
        }
    }
    fn volume(&self) -> &[QualifiedVolume] {
        match self {
            Self::BridgeInstallation(v) => v.volume(),
            Self::BuildingInstallation(v) => v.volume(),
            Self::TunnelInstallation(v) => v.volume(),
        }
    }
    fn area(&self) -> &[QualifiedArea] {
        match self {
            Self::BridgeInstallation(v) => v.area(),
            Self::BuildingInstallation(v) => v.area(),
            Self::TunnelInstallation(v) => v.area(),
        }
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeInstallation(v) => v.lod2_multi_curve(),
            Self::BuildingInstallation(v) => v.lod2_multi_curve(),
            Self::TunnelInstallation(v) => v.lod2_multi_curve(),
        }
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::BridgeInstallation(v) => v.lod3_multi_surface(),
            Self::BuildingInstallation(v) => v.lod3_multi_surface(),
            Self::TunnelInstallation(v) => v.lod3_multi_surface(),
        }
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::BridgeInstallation(v) => v.lod0_multi_surface(),
            Self::BuildingInstallation(v) => v.lod0_multi_surface(),
            Self::TunnelInstallation(v) => v.lod0_multi_surface(),
        }
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::BridgeInstallation(v) => v.lod1_solid(),
            Self::BuildingInstallation(v) => v.lod1_solid(),
            Self::TunnelInstallation(v) => v.lod1_solid(),
        }
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::BridgeInstallation(v) => v.lod3_solid(),
            Self::BuildingInstallation(v) => v.lod3_solid(),
            Self::TunnelInstallation(v) => v.lod3_solid(),
        }
    }
    fn boundary(&self) -> &[AbstractSpaceBoundary] {
        match self {
            Self::BridgeInstallation(v) => v.boundary(),
            Self::BuildingInstallation(v) => v.boundary(),
            Self::TunnelInstallation(v) => v.boundary(),
        }
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeInstallation(v) => v.lod0_multi_curve(),
            Self::BuildingInstallation(v) => v.lod0_multi_curve(),
            Self::TunnelInstallation(v) => v.lod0_multi_curve(),
        }
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::BridgeInstallation(v) => v.lod2_solid(),
            Self::BuildingInstallation(v) => v.lod2_solid(),
            Self::TunnelInstallation(v) => v.lod2_solid(),
        }
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        match self {
            Self::BridgeInstallation(v) => v.lod0_point(),
            Self::BuildingInstallation(v) => v.lod0_point(),
            Self::TunnelInstallation(v) => v.lod0_point(),
        }
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeInstallation(v) => v.lod3_multi_curve(),
            Self::BuildingInstallation(v) => v.lod3_multi_curve(),
            Self::TunnelInstallation(v) => v.lod3_multi_curve(),
        }
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::BridgeInstallation(v) => v.lod2_multi_surface(),
            Self::BuildingInstallation(v) => v.lod2_multi_surface(),
            Self::TunnelInstallation(v) => v.lod2_multi_surface(),
        }
    }
}
impl AbstractPhysicalSpaceTrait for AbstractInstallation {
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeInstallation(v) => v.lod3_terrain_intersection_curve(),
            Self::BuildingInstallation(v) => v.lod3_terrain_intersection_curve(),
            Self::TunnelInstallation(v) => v.lod3_terrain_intersection_curve(),
        }
    }
    fn point_cloud(&self) -> Option<&AbstractPointCloud> {
        match self {
            Self::BridgeInstallation(v) => v.point_cloud(),
            Self::BuildingInstallation(v) => v.point_cloud(),
            Self::TunnelInstallation(v) => v.point_cloud(),
        }
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeInstallation(v) => v.lod1_terrain_intersection_curve(),
            Self::BuildingInstallation(v) => v.lod1_terrain_intersection_curve(),
            Self::TunnelInstallation(v) => v.lod1_terrain_intersection_curve(),
        }
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::BridgeInstallation(v) => v.lod2_terrain_intersection_curve(),
            Self::BuildingInstallation(v) => v.lod2_terrain_intersection_curve(),
            Self::TunnelInstallation(v) => v.lod2_terrain_intersection_curve(),
        }
    }
}
impl AbstractOccupiedSpaceTrait for AbstractInstallation {
    fn lod3_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::BridgeInstallation(v) => v.lod3_implicit_representation(),
            Self::BuildingInstallation(v) => v.lod3_implicit_representation(),
            Self::TunnelInstallation(v) => v.lod3_implicit_representation(),
        }
    }
    fn lod2_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::BridgeInstallation(v) => v.lod2_implicit_representation(),
            Self::BuildingInstallation(v) => v.lod2_implicit_representation(),
            Self::TunnelInstallation(v) => v.lod2_implicit_representation(),
        }
    }
    fn lod1_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::BridgeInstallation(v) => v.lod1_implicit_representation(),
            Self::BuildingInstallation(v) => v.lod1_implicit_representation(),
            Self::TunnelInstallation(v) => v.lod1_implicit_representation(),
        }
    }
}
impl AbstractInstallationTrait for AbstractInstallation {
    fn relation_to_construction(&self) -> Option<RelationToConstruction> {
        match self {
            Self::BridgeInstallation(v) => v.relation_to_construction(),
            Self::BuildingInstallation(v) => v.relation_to_construction(),
            Self::TunnelInstallation(v) => v.relation_to_construction(),
        }
    }
}
impl From<BridgeInstallation> for AbstractInstallation {
    fn from(v: BridgeInstallation) -> Self {
        Self::BridgeInstallation(v)
    }
}
impl From<BuildingInstallation> for AbstractInstallation {
    fn from(v: BuildingInstallation) -> Self {
        Self::BuildingInstallation(v)
    }
}
impl From<TunnelInstallation> for AbstractInstallation {
    fn from(v: TunnelInstallation) -> Self {
        Self::TunnelInstallation(v)
    }
}
#[derive(Debug, Clone, Default)]
pub struct CeilingSurface {
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
    pub filling_surface: Vec<AbstractFillingSurface>,
}
impl AbstractFeatureTrait for CeilingSurface {
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
impl AbstractFeatureWithLifespanTrait for CeilingSurface {
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
impl AbstractCityObjectTrait for CeilingSurface {
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
impl AbstractSpaceBoundaryTrait for CeilingSurface {}
impl AbstractThematicSurfaceTrait for CeilingSurface {
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
impl AbstractConstructionSurfaceTrait for CeilingSurface {
    fn filling_surface(&self) -> &[AbstractFillingSurface] {
        &self.filling_surface
    }
}
impl CeilingSurface {
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
        let mut filling_surface = Vec::new();
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
                (crate::namespace::NS_CONSTRUCTION, "fillingSurface") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        filling_surface
                            .push(
                                super::dispatchers::parse_abstract_filling_surface(
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
        Ok(CeilingSurface {
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
            filling_surface,
        })
    }
}
impl crate::from_gml::FromGml for CeilingSurface {
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
pub struct FloorSurface {
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
    pub filling_surface: Vec<AbstractFillingSurface>,
}
impl AbstractFeatureTrait for FloorSurface {
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
impl AbstractFeatureWithLifespanTrait for FloorSurface {
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
impl AbstractCityObjectTrait for FloorSurface {
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
impl AbstractSpaceBoundaryTrait for FloorSurface {}
impl AbstractThematicSurfaceTrait for FloorSurface {
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
impl AbstractConstructionSurfaceTrait for FloorSurface {
    fn filling_surface(&self) -> &[AbstractFillingSurface] {
        &self.filling_surface
    }
}
impl FloorSurface {
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
        let mut filling_surface = Vec::new();
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
                (crate::namespace::NS_CONSTRUCTION, "fillingSurface") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        filling_surface
                            .push(
                                super::dispatchers::parse_abstract_filling_surface(
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
        Ok(FloorSurface {
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
            filling_surface,
        })
    }
}
impl crate::from_gml::FromGml for FloorSurface {
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
pub struct GroundSurface {
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
    pub filling_surface: Vec<AbstractFillingSurface>,
}
impl AbstractFeatureTrait for GroundSurface {
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
impl AbstractFeatureWithLifespanTrait for GroundSurface {
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
impl AbstractCityObjectTrait for GroundSurface {
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
impl AbstractSpaceBoundaryTrait for GroundSurface {}
impl AbstractThematicSurfaceTrait for GroundSurface {
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
impl AbstractConstructionSurfaceTrait for GroundSurface {
    fn filling_surface(&self) -> &[AbstractFillingSurface] {
        &self.filling_surface
    }
}
impl GroundSurface {
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
        let mut filling_surface = Vec::new();
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
                (crate::namespace::NS_CONSTRUCTION, "fillingSurface") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        filling_surface
                            .push(
                                super::dispatchers::parse_abstract_filling_surface(
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
        Ok(GroundSurface {
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
            filling_surface,
        })
    }
}
impl crate::from_gml::FromGml for GroundSurface {
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
pub struct InteriorWallSurface {
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
    pub filling_surface: Vec<AbstractFillingSurface>,
}
impl AbstractFeatureTrait for InteriorWallSurface {
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
impl AbstractFeatureWithLifespanTrait for InteriorWallSurface {
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
impl AbstractCityObjectTrait for InteriorWallSurface {
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
impl AbstractSpaceBoundaryTrait for InteriorWallSurface {}
impl AbstractThematicSurfaceTrait for InteriorWallSurface {
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
impl AbstractConstructionSurfaceTrait for InteriorWallSurface {
    fn filling_surface(&self) -> &[AbstractFillingSurface] {
        &self.filling_surface
    }
}
impl InteriorWallSurface {
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
        let mut filling_surface = Vec::new();
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
                (crate::namespace::NS_CONSTRUCTION, "fillingSurface") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        filling_surface
                            .push(
                                super::dispatchers::parse_abstract_filling_surface(
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
        Ok(InteriorWallSurface {
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
            filling_surface,
        })
    }
}
impl crate::from_gml::FromGml for InteriorWallSurface {
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
pub struct OuterCeilingSurface {
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
    pub filling_surface: Vec<AbstractFillingSurface>,
}
impl AbstractFeatureTrait for OuterCeilingSurface {
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
impl AbstractFeatureWithLifespanTrait for OuterCeilingSurface {
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
impl AbstractCityObjectTrait for OuterCeilingSurface {
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
impl AbstractSpaceBoundaryTrait for OuterCeilingSurface {}
impl AbstractThematicSurfaceTrait for OuterCeilingSurface {
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
impl AbstractConstructionSurfaceTrait for OuterCeilingSurface {
    fn filling_surface(&self) -> &[AbstractFillingSurface] {
        &self.filling_surface
    }
}
impl OuterCeilingSurface {
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
        let mut filling_surface = Vec::new();
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
                (crate::namespace::NS_CONSTRUCTION, "fillingSurface") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        filling_surface
                            .push(
                                super::dispatchers::parse_abstract_filling_surface(
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
        Ok(OuterCeilingSurface {
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
            filling_surface,
        })
    }
}
impl crate::from_gml::FromGml for OuterCeilingSurface {
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
pub struct OuterFloorSurface {
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
    pub filling_surface: Vec<AbstractFillingSurface>,
}
impl AbstractFeatureTrait for OuterFloorSurface {
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
impl AbstractFeatureWithLifespanTrait for OuterFloorSurface {
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
impl AbstractCityObjectTrait for OuterFloorSurface {
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
impl AbstractSpaceBoundaryTrait for OuterFloorSurface {}
impl AbstractThematicSurfaceTrait for OuterFloorSurface {
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
impl AbstractConstructionSurfaceTrait for OuterFloorSurface {
    fn filling_surface(&self) -> &[AbstractFillingSurface] {
        &self.filling_surface
    }
}
impl OuterFloorSurface {
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
        let mut filling_surface = Vec::new();
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
                (crate::namespace::NS_CONSTRUCTION, "fillingSurface") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        filling_surface
                            .push(
                                super::dispatchers::parse_abstract_filling_surface(
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
        Ok(OuterFloorSurface {
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
            filling_surface,
        })
    }
}
impl crate::from_gml::FromGml for OuterFloorSurface {
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
pub struct RoofSurface {
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
    pub filling_surface: Vec<AbstractFillingSurface>,
}
impl AbstractFeatureTrait for RoofSurface {
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
impl AbstractFeatureWithLifespanTrait for RoofSurface {
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
impl AbstractCityObjectTrait for RoofSurface {
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
impl AbstractSpaceBoundaryTrait for RoofSurface {}
impl AbstractThematicSurfaceTrait for RoofSurface {
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
impl AbstractConstructionSurfaceTrait for RoofSurface {
    fn filling_surface(&self) -> &[AbstractFillingSurface] {
        &self.filling_surface
    }
}
impl RoofSurface {
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
        let mut filling_surface = Vec::new();
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
                (crate::namespace::NS_CONSTRUCTION, "fillingSurface") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        filling_surface
                            .push(
                                super::dispatchers::parse_abstract_filling_surface(
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
        Ok(RoofSurface {
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
            filling_surface,
        })
    }
}
impl crate::from_gml::FromGml for RoofSurface {
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
pub struct WallSurface {
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
    pub filling_surface: Vec<AbstractFillingSurface>,
}
impl AbstractFeatureTrait for WallSurface {
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
impl AbstractFeatureWithLifespanTrait for WallSurface {
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
impl AbstractCityObjectTrait for WallSurface {
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
impl AbstractSpaceBoundaryTrait for WallSurface {}
impl AbstractThematicSurfaceTrait for WallSurface {
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
impl AbstractConstructionSurfaceTrait for WallSurface {
    fn filling_surface(&self) -> &[AbstractFillingSurface] {
        &self.filling_surface
    }
}
impl WallSurface {
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
        let mut filling_surface = Vec::new();
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
                (crate::namespace::NS_CONSTRUCTION, "fillingSurface") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        filling_surface
                            .push(
                                super::dispatchers::parse_abstract_filling_surface(
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
        Ok(WallSurface {
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
            filling_surface,
        })
    }
}
impl crate::from_gml::FromGml for WallSurface {
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
pub struct DoorSurface {
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
    pub address: Vec<Address>,
}
impl AbstractFeatureTrait for DoorSurface {
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
impl AbstractFeatureWithLifespanTrait for DoorSurface {
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
impl AbstractCityObjectTrait for DoorSurface {
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
impl AbstractSpaceBoundaryTrait for DoorSurface {}
impl AbstractThematicSurfaceTrait for DoorSurface {
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
impl AbstractFillingSurfaceTrait for DoorSurface {}
impl DoorSurface {
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
        let mut address = Vec::new();
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
                (crate::namespace::NS_CONSTRUCTION, "address") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        address
                            .push(
                                Address::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(DoorSurface {
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
            address,
        })
    }
}
impl crate::from_gml::FromGml for DoorSurface {
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
pub struct WindowSurface {
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
impl AbstractFeatureTrait for WindowSurface {
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
impl AbstractFeatureWithLifespanTrait for WindowSurface {
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
impl AbstractCityObjectTrait for WindowSurface {
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
impl AbstractSpaceBoundaryTrait for WindowSurface {}
impl AbstractThematicSurfaceTrait for WindowSurface {
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
impl AbstractFillingSurfaceTrait for WindowSurface {}
impl WindowSurface {
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
        Ok(WindowSurface {
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
impl crate::from_gml::FromGml for WindowSurface {
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
pub struct OtherConstruction {
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
    pub space_type: Option<SpaceType>,
    pub volume: Vec<QualifiedVolume>,
    pub area: Vec<QualifiedArea>,
    pub lod2_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod3_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod0_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod1_solid: Option<crate::geometry::Solid>,
    pub lod3_solid: Option<crate::geometry::Solid>,
    pub boundary: Vec<AbstractSpaceBoundary>,
    pub lod0_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_solid: Option<crate::geometry::Solid>,
    pub lod0_point: Option<crate::geometry::DirectPosition>,
    pub lod3_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod3_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub point_cloud: Option<AbstractPointCloud>,
    pub lod1_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub lod3_implicit_representation: Option<ImplicitGeometry>,
    pub lod2_implicit_representation: Option<ImplicitGeometry>,
    pub lod1_implicit_representation: Option<ImplicitGeometry>,
    pub condition_of_construction: Option<ConditionOfConstructionValue>,
    pub date_of_construction: Option<String>,
    pub date_of_demolition: Option<String>,
    pub construction_event: Vec<ConstructionEvent>,
    pub elevation: Vec<Elevation>,
    pub height: Vec<Height>,
    pub occupancy: Vec<Occupancy>,
    pub class_: Option<OtherConstructionClassValue>,
    pub function: Vec<OtherConstructionFunctionValue>,
    pub usage: Vec<OtherConstructionUsageValue>,
}
impl AbstractFeatureTrait for OtherConstruction {
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
impl AbstractFeatureWithLifespanTrait for OtherConstruction {
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
impl AbstractCityObjectTrait for OtherConstruction {
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
impl AbstractSpaceTrait for OtherConstruction {
    fn space_type(&self) -> Option<SpaceType> {
        self.space_type
    }
    fn volume(&self) -> &[QualifiedVolume] {
        &self.volume
    }
    fn area(&self) -> &[QualifiedArea] {
        &self.area
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
    fn boundary(&self) -> &[AbstractSpaceBoundary] {
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
impl AbstractPhysicalSpaceTrait for OtherConstruction {
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_terrain_intersection_curve.as_ref()
    }
    fn point_cloud(&self) -> Option<&AbstractPointCloud> {
        self.point_cloud.as_ref()
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod1_terrain_intersection_curve.as_ref()
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_terrain_intersection_curve.as_ref()
    }
}
impl AbstractOccupiedSpaceTrait for OtherConstruction {
    fn lod3_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        self.lod3_implicit_representation.as_ref()
    }
    fn lod2_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        self.lod2_implicit_representation.as_ref()
    }
    fn lod1_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        self.lod1_implicit_representation.as_ref()
    }
}
impl AbstractConstructionTrait for OtherConstruction {
    fn condition_of_construction(&self) -> Option<ConditionOfConstructionValue> {
        self.condition_of_construction
    }
    fn date_of_construction(&self) -> Option<&String> {
        self.date_of_construction.as_ref()
    }
    fn date_of_demolition(&self) -> Option<&String> {
        self.date_of_demolition.as_ref()
    }
    fn construction_event(&self) -> &[ConstructionEvent] {
        &self.construction_event
    }
    fn elevation(&self) -> &[Elevation] {
        &self.elevation
    }
    fn height(&self) -> &[Height] {
        &self.height
    }
    fn occupancy(&self) -> &[Occupancy] {
        &self.occupancy
    }
}
impl OtherConstruction {
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
        let mut space_type = None;
        let mut volume = Vec::new();
        let mut area = Vec::new();
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
        let mut lod3_terrain_intersection_curve = None;
        let mut point_cloud = None;
        let mut lod1_terrain_intersection_curve = None;
        let mut lod2_terrain_intersection_curve = None;
        let mut lod3_implicit_representation = None;
        let mut lod2_implicit_representation = None;
        let mut lod1_implicit_representation = None;
        let mut condition_of_construction = None;
        let mut date_of_construction = None;
        let mut date_of_demolition = None;
        let mut construction_event = Vec::new();
        let mut elevation = Vec::new();
        let mut height = Vec::new();
        let mut occupancy = Vec::new();
        let mut class_ = None;
        let mut function = Vec::new();
        let mut usage = Vec::new();
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
                (crate::namespace::NS_CORE, "spaceType") => {
                    space_type = Some(SpaceType::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_CORE, "volume") => {
                    volume.push(QualifiedVolume::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "area") => {
                    area.push(QualifiedArea::from_gml(&mut sub)?);
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
                                super::dispatchers::parse_abstract_space_boundary(
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
                            super::dispatchers::parse_abstract_point_cloud(
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
                (crate::namespace::NS_CORE, "lod3ImplicitRepresentation") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        lod3_implicit_representation = Some(
                            ImplicitGeometry::from_gml_with_info(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    } else {
                        lod3_implicit_representation = Some(
                            ImplicitGeometry::from_gml(&mut sub)?,
                        );
                    }
                }
                (crate::namespace::NS_CORE, "lod2ImplicitRepresentation") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        lod2_implicit_representation = Some(
                            ImplicitGeometry::from_gml_with_info(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    } else {
                        lod2_implicit_representation = Some(
                            ImplicitGeometry::from_gml(&mut sub)?,
                        );
                    }
                }
                (crate::namespace::NS_CORE, "lod1ImplicitRepresentation") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        lod1_implicit_representation = Some(
                            ImplicitGeometry::from_gml_with_info(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    } else {
                        lod1_implicit_representation = Some(
                            ImplicitGeometry::from_gml(&mut sub)?,
                        );
                    }
                }
                (crate::namespace::NS_CONSTRUCTION, "conditionOfConstruction") => {
                    condition_of_construction = Some(
                        ConditionOfConstructionValue::from_gml_text(&sub.read_text()?)?,
                    );
                }
                (crate::namespace::NS_CONSTRUCTION, "dateOfConstruction") => {
                    date_of_construction = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_CONSTRUCTION, "dateOfDemolition") => {
                    date_of_demolition = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_CONSTRUCTION, "constructionEvent") => {
                    construction_event.push(ConstructionEvent::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CONSTRUCTION, "elevation") => {
                    elevation.push(Elevation::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CONSTRUCTION, "height") => {
                    height.push(Height::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CONSTRUCTION, "occupancy") => {
                    occupancy.push(Occupancy::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CONSTRUCTION, "class") => {
                    class_ = Some(OtherConstructionClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_CONSTRUCTION, "function") => {
                    function.push(OtherConstructionFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_CONSTRUCTION, "usage") => {
                    usage.push(OtherConstructionUsageValue(sub.read_text()?));
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(OtherConstruction {
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
            space_type,
            volume,
            area,
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
            lod3_terrain_intersection_curve,
            point_cloud,
            lod1_terrain_intersection_curve,
            lod2_terrain_intersection_curve,
            lod3_implicit_representation,
            lod2_implicit_representation,
            lod1_implicit_representation,
            condition_of_construction,
            date_of_construction,
            date_of_demolition,
            construction_event,
            elevation,
            height,
            occupancy,
            class_,
            function,
            usage,
        })
    }
}
impl crate::from_gml::FromGml for OtherConstruction {
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
pub struct Door {
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
    pub space_type: Option<SpaceType>,
    pub volume: Vec<QualifiedVolume>,
    pub area: Vec<QualifiedArea>,
    pub lod2_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod3_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod0_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod1_solid: Option<crate::geometry::Solid>,
    pub lod3_solid: Option<crate::geometry::Solid>,
    pub boundary: Vec<AbstractSpaceBoundary>,
    pub lod0_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_solid: Option<crate::geometry::Solid>,
    pub lod0_point: Option<crate::geometry::DirectPosition>,
    pub lod3_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod3_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub point_cloud: Option<AbstractPointCloud>,
    pub lod1_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub lod3_implicit_representation: Option<ImplicitGeometry>,
    pub lod2_implicit_representation: Option<ImplicitGeometry>,
    pub lod1_implicit_representation: Option<ImplicitGeometry>,
    pub class_: Option<DoorClassValue>,
    pub function: Vec<DoorFunctionValue>,
    pub usage: Vec<DoorUsageValue>,
    pub address: Vec<Address>,
}
impl AbstractFeatureTrait for Door {
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
impl AbstractFeatureWithLifespanTrait for Door {
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
impl AbstractCityObjectTrait for Door {
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
impl AbstractSpaceTrait for Door {
    fn space_type(&self) -> Option<SpaceType> {
        self.space_type
    }
    fn volume(&self) -> &[QualifiedVolume] {
        &self.volume
    }
    fn area(&self) -> &[QualifiedArea] {
        &self.area
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
    fn boundary(&self) -> &[AbstractSpaceBoundary] {
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
impl AbstractPhysicalSpaceTrait for Door {
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_terrain_intersection_curve.as_ref()
    }
    fn point_cloud(&self) -> Option<&AbstractPointCloud> {
        self.point_cloud.as_ref()
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod1_terrain_intersection_curve.as_ref()
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_terrain_intersection_curve.as_ref()
    }
}
impl AbstractOccupiedSpaceTrait for Door {
    fn lod3_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        self.lod3_implicit_representation.as_ref()
    }
    fn lod2_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        self.lod2_implicit_representation.as_ref()
    }
    fn lod1_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        self.lod1_implicit_representation.as_ref()
    }
}
impl AbstractFillingElementTrait for Door {}
impl Door {
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
        let mut space_type = None;
        let mut volume = Vec::new();
        let mut area = Vec::new();
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
        let mut lod3_terrain_intersection_curve = None;
        let mut point_cloud = None;
        let mut lod1_terrain_intersection_curve = None;
        let mut lod2_terrain_intersection_curve = None;
        let mut lod3_implicit_representation = None;
        let mut lod2_implicit_representation = None;
        let mut lod1_implicit_representation = None;
        let mut class_ = None;
        let mut function = Vec::new();
        let mut usage = Vec::new();
        let mut address = Vec::new();
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
                (crate::namespace::NS_CORE, "spaceType") => {
                    space_type = Some(SpaceType::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_CORE, "volume") => {
                    volume.push(QualifiedVolume::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "area") => {
                    area.push(QualifiedArea::from_gml(&mut sub)?);
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
                                super::dispatchers::parse_abstract_space_boundary(
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
                            super::dispatchers::parse_abstract_point_cloud(
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
                (crate::namespace::NS_CORE, "lod3ImplicitRepresentation") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        lod3_implicit_representation = Some(
                            ImplicitGeometry::from_gml_with_info(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    } else {
                        lod3_implicit_representation = Some(
                            ImplicitGeometry::from_gml(&mut sub)?,
                        );
                    }
                }
                (crate::namespace::NS_CORE, "lod2ImplicitRepresentation") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        lod2_implicit_representation = Some(
                            ImplicitGeometry::from_gml_with_info(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    } else {
                        lod2_implicit_representation = Some(
                            ImplicitGeometry::from_gml(&mut sub)?,
                        );
                    }
                }
                (crate::namespace::NS_CORE, "lod1ImplicitRepresentation") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        lod1_implicit_representation = Some(
                            ImplicitGeometry::from_gml_with_info(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    } else {
                        lod1_implicit_representation = Some(
                            ImplicitGeometry::from_gml(&mut sub)?,
                        );
                    }
                }
                (crate::namespace::NS_CONSTRUCTION, "class") => {
                    class_ = Some(DoorClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_CONSTRUCTION, "function") => {
                    function.push(DoorFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_CONSTRUCTION, "usage") => {
                    usage.push(DoorUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_CONSTRUCTION, "address") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        address
                            .push(
                                Address::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Door {
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
            space_type,
            volume,
            area,
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
            lod3_terrain_intersection_curve,
            point_cloud,
            lod1_terrain_intersection_curve,
            lod2_terrain_intersection_curve,
            lod3_implicit_representation,
            lod2_implicit_representation,
            lod1_implicit_representation,
            class_,
            function,
            usage,
            address,
        })
    }
}
impl crate::from_gml::FromGml for Door {
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
pub struct Window {
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
    pub space_type: Option<SpaceType>,
    pub volume: Vec<QualifiedVolume>,
    pub area: Vec<QualifiedArea>,
    pub lod2_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod3_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod0_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod1_solid: Option<crate::geometry::Solid>,
    pub lod3_solid: Option<crate::geometry::Solid>,
    pub boundary: Vec<AbstractSpaceBoundary>,
    pub lod0_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_solid: Option<crate::geometry::Solid>,
    pub lod0_point: Option<crate::geometry::DirectPosition>,
    pub lod3_multi_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_multi_surface: Option<crate::geometry::MultiSurface>,
    pub lod3_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub point_cloud: Option<AbstractPointCloud>,
    pub lod1_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub lod2_terrain_intersection_curve: Option<crate::geometry::MultiCurve>,
    pub lod3_implicit_representation: Option<ImplicitGeometry>,
    pub lod2_implicit_representation: Option<ImplicitGeometry>,
    pub lod1_implicit_representation: Option<ImplicitGeometry>,
    pub class_: Option<WindowClassValue>,
    pub function: Vec<WindowFunctionValue>,
    pub usage: Vec<WindowUsageValue>,
}
impl AbstractFeatureTrait for Window {
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
impl AbstractFeatureWithLifespanTrait for Window {
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
impl AbstractCityObjectTrait for Window {
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
impl AbstractSpaceTrait for Window {
    fn space_type(&self) -> Option<SpaceType> {
        self.space_type
    }
    fn volume(&self) -> &[QualifiedVolume] {
        &self.volume
    }
    fn area(&self) -> &[QualifiedArea] {
        &self.area
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
    fn boundary(&self) -> &[AbstractSpaceBoundary] {
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
impl AbstractPhysicalSpaceTrait for Window {
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod3_terrain_intersection_curve.as_ref()
    }
    fn point_cloud(&self) -> Option<&AbstractPointCloud> {
        self.point_cloud.as_ref()
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod1_terrain_intersection_curve.as_ref()
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        self.lod2_terrain_intersection_curve.as_ref()
    }
}
impl AbstractOccupiedSpaceTrait for Window {
    fn lod3_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        self.lod3_implicit_representation.as_ref()
    }
    fn lod2_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        self.lod2_implicit_representation.as_ref()
    }
    fn lod1_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        self.lod1_implicit_representation.as_ref()
    }
}
impl AbstractFillingElementTrait for Window {}
impl Window {
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
        let mut space_type = None;
        let mut volume = Vec::new();
        let mut area = Vec::new();
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
        let mut lod3_terrain_intersection_curve = None;
        let mut point_cloud = None;
        let mut lod1_terrain_intersection_curve = None;
        let mut lod2_terrain_intersection_curve = None;
        let mut lod3_implicit_representation = None;
        let mut lod2_implicit_representation = None;
        let mut lod1_implicit_representation = None;
        let mut class_ = None;
        let mut function = Vec::new();
        let mut usage = Vec::new();
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
                (crate::namespace::NS_CORE, "spaceType") => {
                    space_type = Some(SpaceType::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_CORE, "volume") => {
                    volume.push(QualifiedVolume::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_CORE, "area") => {
                    area.push(QualifiedArea::from_gml(&mut sub)?);
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
                                super::dispatchers::parse_abstract_space_boundary(
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
                            super::dispatchers::parse_abstract_point_cloud(
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
                (crate::namespace::NS_CORE, "lod3ImplicitRepresentation") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        lod3_implicit_representation = Some(
                            ImplicitGeometry::from_gml_with_info(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    } else {
                        lod3_implicit_representation = Some(
                            ImplicitGeometry::from_gml(&mut sub)?,
                        );
                    }
                }
                (crate::namespace::NS_CORE, "lod2ImplicitRepresentation") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        lod2_implicit_representation = Some(
                            ImplicitGeometry::from_gml_with_info(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    } else {
                        lod2_implicit_representation = Some(
                            ImplicitGeometry::from_gml(&mut sub)?,
                        );
                    }
                }
                (crate::namespace::NS_CORE, "lod1ImplicitRepresentation") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        lod1_implicit_representation = Some(
                            ImplicitGeometry::from_gml_with_info(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    } else {
                        lod1_implicit_representation = Some(
                            ImplicitGeometry::from_gml(&mut sub)?,
                        );
                    }
                }
                (crate::namespace::NS_CONSTRUCTION, "class") => {
                    class_ = Some(WindowClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_CONSTRUCTION, "function") => {
                    function.push(WindowFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_CONSTRUCTION, "usage") => {
                    usage.push(WindowUsageValue(sub.read_text()?));
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Window {
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
            space_type,
            volume,
            area,
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
            lod3_terrain_intersection_curve,
            point_cloud,
            lod1_terrain_intersection_curve,
            lod2_terrain_intersection_curve,
            lod3_implicit_representation,
            lod2_implicit_representation,
            lod1_implicit_representation,
            class_,
            function,
            usage,
        })
    }
}
impl crate::from_gml::FromGml for Window {
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
