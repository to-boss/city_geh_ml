#![allow(unused_imports, unused_mut, unused_variables)]
use super::*;

pub fn parse_dyn_abstract_construction(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractConstruction>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "OtherConstruction") => {
            Ok(Box::new(super::OtherConstruction::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "Bridge") => {
            Ok(Box::new(super::Bridge::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgePart") => {
            Ok(Box::new(super::BridgePart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "Building") => {
            Ok(Box::new(super::Building::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingPart") => {
            Ok(Box::new(super::BuildingPart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "Tunnel") => {
            Ok(Box::new(super::Tunnel::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelPart") => {
            Ok(Box::new(super::TunnelPart::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_construction_surface(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractConstructionSurface>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "CeilingSurface") => {
            Ok(Box::new(super::CeilingSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "FloorSurface") => {
            Ok(Box::new(super::FloorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "GroundSurface") => {
            Ok(Box::new(super::GroundSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "InteriorWallSurface") => {
            Ok(Box::new(super::InteriorWallSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterCeilingSurface") => {
            Ok(Box::new(super::OuterCeilingSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterFloorSurface") => {
            Ok(Box::new(super::OuterFloorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "RoofSurface") => {
            Ok(Box::new(super::RoofSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "WallSurface") => {
            Ok(Box::new(super::WallSurface::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_constructive_element(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractConstructiveElement>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_BRIDGE, "BridgeConstructiveElement") => {
            Ok(
                Box::new(
                    super::BridgeConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingConstructiveElement") => {
            Ok(
                Box::new(
                    super::BuildingConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelConstructiveElement") => {
            Ok(
                Box::new(
                    super::TunnelConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_filling_element(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractFillingElement>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "Door") => {
            Ok(Box::new(super::Door::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "Window") => {
            Ok(Box::new(super::Window::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_filling_surface(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractFillingSurface>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "DoorSurface") => {
            Ok(Box::new(super::DoorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "WindowSurface") => {
            Ok(Box::new(super::WindowSurface::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_furniture(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractFurniture>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_BRIDGE, "BridgeFurniture") => {
            Ok(Box::new(super::BridgeFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingFurniture") => {
            Ok(Box::new(super::BuildingFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelFurniture") => {
            Ok(Box::new(super::TunnelFurniture::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_installation(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractInstallation>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_BRIDGE, "BridgeInstallation") => {
            Ok(Box::new(super::BridgeInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingInstallation") => {
            Ok(Box::new(super::BuildingInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelInstallation") => {
            Ok(Box::new(super::TunnelInstallation::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_atomic_timeseries(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractAtomicTimeseries>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_DYNAMIZER, "GenericTimeseries") => {
            Ok(Box::new(super::GenericTimeseries::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_DYNAMIZER, "StandardFileTimeseries") => {
            Ok(
                Box::new(
                    super::StandardFileTimeseries::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_DYNAMIZER, "TabulatedFileTimeseries") => {
            Ok(
                Box::new(
                    super::TabulatedFileTimeseries::from_gml_with_info(reader, info)?,
                ),
            )
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_timeseries(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractTimeseries>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_DYNAMIZER, "CompositeTimeseries") => {
            Ok(Box::new(super::CompositeTimeseries::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_DYNAMIZER, "GenericTimeseries") => {
            Ok(Box::new(super::GenericTimeseries::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_DYNAMIZER, "StandardFileTimeseries") => {
            Ok(
                Box::new(
                    super::StandardFileTimeseries::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_DYNAMIZER, "TabulatedFileTimeseries") => {
            Ok(
                Box::new(
                    super::TabulatedFileTimeseries::from_gml_with_info(reader, info)?,
                ),
            )
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_surface_data(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractSurfaceData>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_APPEARANCE, "GeoreferencedTexture") => {
            Ok(Box::new(super::GeoreferencedTexture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_APPEARANCE, "ParameterizedTexture") => {
            Ok(Box::new(super::ParameterizedTexture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_APPEARANCE, "X3DMaterial") => {
            Ok(Box::new(super::X3DMaterial::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_texture(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractTexture>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_APPEARANCE, "GeoreferencedTexture") => {
            Ok(Box::new(super::GeoreferencedTexture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_APPEARANCE, "ParameterizedTexture") => {
            Ok(Box::new(super::ParameterizedTexture::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_bridge(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractBridge>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_BRIDGE, "Bridge") => {
            Ok(Box::new(super::Bridge::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgePart") => {
            Ok(Box::new(super::BridgePart::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_building(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractBuilding>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_BUILDING, "Building") => {
            Ok(Box::new(super::Building::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingPart") => {
            Ok(Box::new(super::BuildingPart::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_building_subdivision(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractBuildingSubdivision>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_BUILDING, "BuildingUnit") => {
            Ok(Box::new(super::BuildingUnit::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "Storey") => {
            Ok(Box::new(super::Storey::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_appearance(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractAppearance>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_APPEARANCE, "Appearance") => {
            Ok(Box::new(super::Appearance::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_city_object(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractCityObject>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "CeilingSurface") => {
            Ok(Box::new(super::CeilingSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "Door") => {
            Ok(Box::new(super::Door::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "DoorSurface") => {
            Ok(Box::new(super::DoorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "FloorSurface") => {
            Ok(Box::new(super::FloorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "GroundSurface") => {
            Ok(Box::new(super::GroundSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "InteriorWallSurface") => {
            Ok(Box::new(super::InteriorWallSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OtherConstruction") => {
            Ok(Box::new(super::OtherConstruction::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterCeilingSurface") => {
            Ok(Box::new(super::OuterCeilingSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterFloorSurface") => {
            Ok(Box::new(super::OuterFloorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "RoofSurface") => {
            Ok(Box::new(super::RoofSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "WallSurface") => {
            Ok(Box::new(super::WallSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "Window") => {
            Ok(Box::new(super::Window::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "WindowSurface") => {
            Ok(Box::new(super::WindowSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "Bridge") => {
            Ok(Box::new(super::Bridge::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgeConstructiveElement") => {
            Ok(
                Box::new(
                    super::BridgeConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeFurniture") => {
            Ok(Box::new(super::BridgeFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgeInstallation") => {
            Ok(Box::new(super::BridgeInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgePart") => {
            Ok(Box::new(super::BridgePart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgeRoom") => {
            Ok(Box::new(super::BridgeRoom::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "Building") => {
            Ok(Box::new(super::Building::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingConstructiveElement") => {
            Ok(
                Box::new(
                    super::BuildingConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingFurniture") => {
            Ok(Box::new(super::BuildingFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingInstallation") => {
            Ok(Box::new(super::BuildingInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingPart") => {
            Ok(Box::new(super::BuildingPart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingRoom") => {
            Ok(Box::new(super::BuildingRoom::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingUnit") => {
            Ok(Box::new(super::BuildingUnit::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "Storey") => {
            Ok(Box::new(super::Storey::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CITY_FURNITURE, "CityFurniture") => {
            Ok(Box::new(super::CityFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CITY_OBJECT_GROUP, "CityObjectGroup") => {
            Ok(Box::new(super::CityObjectGroup::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CORE, "ClosureSurface") => {
            Ok(Box::new(super::ClosureSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericLogicalSpace") => {
            Ok(Box::new(super::GenericLogicalSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericOccupiedSpace") => {
            Ok(Box::new(super::GenericOccupiedSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericThematicSurface") => {
            Ok(
                Box::new(
                    super::GenericThematicSurface::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericUnoccupiedSpace") => {
            Ok(
                Box::new(
                    super::GenericUnoccupiedSpace::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_LAND_USE, "LandUse") => {
            Ok(Box::new(super::LandUse::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "BreaklineRelief") => {
            Ok(Box::new(super::BreaklineRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "MassPointRelief") => {
            Ok(Box::new(super::MassPointRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "RasterRelief") => {
            Ok(Box::new(super::RasterRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "ReliefFeature") => {
            Ok(Box::new(super::ReliefFeature::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "TINRelief") => {
            Ok(Box::new(super::TINRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficArea") => {
            Ok(Box::new(super::AuxiliaryTrafficArea::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficSpace") => {
            Ok(Box::new(super::AuxiliaryTrafficSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "ClearanceSpace") => {
            Ok(Box::new(super::ClearanceSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Hole") => {
            Ok(Box::new(super::Hole::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "HoleSurface") => {
            Ok(Box::new(super::HoleSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Intersection") => {
            Ok(Box::new(super::Intersection::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Marking") => {
            Ok(Box::new(super::Marking::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Railway") => {
            Ok(Box::new(super::Railway::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Road") => {
            Ok(Box::new(super::Road::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Section") => {
            Ok(Box::new(super::Section::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Square") => {
            Ok(Box::new(super::Square::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Track") => {
            Ok(Box::new(super::Track::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficArea") => {
            Ok(Box::new(super::TrafficArea::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficSpace") => {
            Ok(Box::new(super::TrafficSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Waterway") => {
            Ok(Box::new(super::Waterway::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "HollowSpace") => {
            Ok(Box::new(super::HollowSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "Tunnel") => {
            Ok(Box::new(super::Tunnel::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelConstructiveElement") => {
            Ok(
                Box::new(
                    super::TunnelConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelFurniture") => {
            Ok(Box::new(super::TunnelFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelInstallation") => {
            Ok(Box::new(super::TunnelInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelPart") => {
            Ok(Box::new(super::TunnelPart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_VEGETATION, "PlantCover") => {
            Ok(Box::new(super::PlantCover::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_VEGETATION, "SolitaryVegetationObject") => {
            Ok(
                Box::new(
                    super::SolitaryVegetationObject::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterBody") => {
            Ok(Box::new(super::WaterBody::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_WATER_BODY, "WaterGroundSurface") => {
            Ok(Box::new(super::WaterGroundSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_WATER_BODY, "WaterSurface") => {
            Ok(Box::new(super::WaterSurface::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_dynamizer(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractDynamizer>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_DYNAMIZER, "Dynamizer") => {
            Ok(Box::new(super::Dynamizer::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_feature(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractFeature>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "CeilingSurface") => {
            Ok(Box::new(super::CeilingSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "Door") => {
            Ok(Box::new(super::Door::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "DoorSurface") => {
            Ok(Box::new(super::DoorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "FloorSurface") => {
            Ok(Box::new(super::FloorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "GroundSurface") => {
            Ok(Box::new(super::GroundSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "InteriorWallSurface") => {
            Ok(Box::new(super::InteriorWallSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OtherConstruction") => {
            Ok(Box::new(super::OtherConstruction::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterCeilingSurface") => {
            Ok(Box::new(super::OuterCeilingSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterFloorSurface") => {
            Ok(Box::new(super::OuterFloorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "RoofSurface") => {
            Ok(Box::new(super::RoofSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "WallSurface") => {
            Ok(Box::new(super::WallSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "Window") => {
            Ok(Box::new(super::Window::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "WindowSurface") => {
            Ok(Box::new(super::WindowSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_DYNAMIZER, "CompositeTimeseries") => {
            Ok(Box::new(super::CompositeTimeseries::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_DYNAMIZER, "Dynamizer") => {
            Ok(Box::new(super::Dynamizer::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_DYNAMIZER, "GenericTimeseries") => {
            Ok(Box::new(super::GenericTimeseries::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_DYNAMIZER, "StandardFileTimeseries") => {
            Ok(
                Box::new(
                    super::StandardFileTimeseries::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_DYNAMIZER, "TabulatedFileTimeseries") => {
            Ok(
                Box::new(
                    super::TabulatedFileTimeseries::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_POINT_CLOUD, "PointCloud") => {
            Ok(Box::new(super::PointCloud::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_VERSIONING, "Version") => {
            Ok(Box::new(super::Version::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_VERSIONING, "VersionTransition") => {
            Ok(Box::new(super::VersionTransition::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_APPEARANCE, "Appearance") => {
            Ok(Box::new(super::Appearance::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_APPEARANCE, "GeoreferencedTexture") => {
            Ok(Box::new(super::GeoreferencedTexture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_APPEARANCE, "ParameterizedTexture") => {
            Ok(Box::new(super::ParameterizedTexture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_APPEARANCE, "X3DMaterial") => {
            Ok(Box::new(super::X3DMaterial::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "Bridge") => {
            Ok(Box::new(super::Bridge::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgeConstructiveElement") => {
            Ok(
                Box::new(
                    super::BridgeConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeFurniture") => {
            Ok(Box::new(super::BridgeFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgeInstallation") => {
            Ok(Box::new(super::BridgeInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgePart") => {
            Ok(Box::new(super::BridgePart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgeRoom") => {
            Ok(Box::new(super::BridgeRoom::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "Building") => {
            Ok(Box::new(super::Building::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingConstructiveElement") => {
            Ok(
                Box::new(
                    super::BuildingConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingFurniture") => {
            Ok(Box::new(super::BuildingFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingInstallation") => {
            Ok(Box::new(super::BuildingInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingPart") => {
            Ok(Box::new(super::BuildingPart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingRoom") => {
            Ok(Box::new(super::BuildingRoom::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingUnit") => {
            Ok(Box::new(super::BuildingUnit::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "Storey") => {
            Ok(Box::new(super::Storey::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CITY_FURNITURE, "CityFurniture") => {
            Ok(Box::new(super::CityFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CITY_OBJECT_GROUP, "CityObjectGroup") => {
            Ok(Box::new(super::CityObjectGroup::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CORE, "Address") => {
            Ok(Box::new(super::Address::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CORE, "CityModel") => {
            Ok(Box::new(super::CityModel::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CORE, "ClosureSurface") => {
            Ok(Box::new(super::ClosureSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericLogicalSpace") => {
            Ok(Box::new(super::GenericLogicalSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericOccupiedSpace") => {
            Ok(Box::new(super::GenericOccupiedSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericThematicSurface") => {
            Ok(
                Box::new(
                    super::GenericThematicSurface::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericUnoccupiedSpace") => {
            Ok(
                Box::new(
                    super::GenericUnoccupiedSpace::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_LAND_USE, "LandUse") => {
            Ok(Box::new(super::LandUse::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "BreaklineRelief") => {
            Ok(Box::new(super::BreaklineRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "MassPointRelief") => {
            Ok(Box::new(super::MassPointRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "RasterRelief") => {
            Ok(Box::new(super::RasterRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "ReliefFeature") => {
            Ok(Box::new(super::ReliefFeature::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "TINRelief") => {
            Ok(Box::new(super::TINRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficArea") => {
            Ok(Box::new(super::AuxiliaryTrafficArea::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficSpace") => {
            Ok(Box::new(super::AuxiliaryTrafficSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "ClearanceSpace") => {
            Ok(Box::new(super::ClearanceSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Hole") => {
            Ok(Box::new(super::Hole::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "HoleSurface") => {
            Ok(Box::new(super::HoleSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Intersection") => {
            Ok(Box::new(super::Intersection::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Marking") => {
            Ok(Box::new(super::Marking::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Railway") => {
            Ok(Box::new(super::Railway::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Road") => {
            Ok(Box::new(super::Road::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Section") => {
            Ok(Box::new(super::Section::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Square") => {
            Ok(Box::new(super::Square::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Track") => {
            Ok(Box::new(super::Track::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficArea") => {
            Ok(Box::new(super::TrafficArea::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficSpace") => {
            Ok(Box::new(super::TrafficSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Waterway") => {
            Ok(Box::new(super::Waterway::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "HollowSpace") => {
            Ok(Box::new(super::HollowSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "Tunnel") => {
            Ok(Box::new(super::Tunnel::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelConstructiveElement") => {
            Ok(
                Box::new(
                    super::TunnelConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelFurniture") => {
            Ok(Box::new(super::TunnelFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelInstallation") => {
            Ok(Box::new(super::TunnelInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelPart") => {
            Ok(Box::new(super::TunnelPart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_VEGETATION, "PlantCover") => {
            Ok(Box::new(super::PlantCover::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_VEGETATION, "SolitaryVegetationObject") => {
            Ok(
                Box::new(
                    super::SolitaryVegetationObject::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterBody") => {
            Ok(Box::new(super::WaterBody::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_WATER_BODY, "WaterGroundSurface") => {
            Ok(Box::new(super::WaterGroundSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_WATER_BODY, "WaterSurface") => {
            Ok(Box::new(super::WaterSurface::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_feature_with_lifespan(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractFeatureWithLifespan>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "CeilingSurface") => {
            Ok(Box::new(super::CeilingSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "Door") => {
            Ok(Box::new(super::Door::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "DoorSurface") => {
            Ok(Box::new(super::DoorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "FloorSurface") => {
            Ok(Box::new(super::FloorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "GroundSurface") => {
            Ok(Box::new(super::GroundSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "InteriorWallSurface") => {
            Ok(Box::new(super::InteriorWallSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OtherConstruction") => {
            Ok(Box::new(super::OtherConstruction::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterCeilingSurface") => {
            Ok(Box::new(super::OuterCeilingSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterFloorSurface") => {
            Ok(Box::new(super::OuterFloorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "RoofSurface") => {
            Ok(Box::new(super::RoofSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "WallSurface") => {
            Ok(Box::new(super::WallSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "Window") => {
            Ok(Box::new(super::Window::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "WindowSurface") => {
            Ok(Box::new(super::WindowSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_DYNAMIZER, "Dynamizer") => {
            Ok(Box::new(super::Dynamizer::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_VERSIONING, "Version") => {
            Ok(Box::new(super::Version::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_VERSIONING, "VersionTransition") => {
            Ok(Box::new(super::VersionTransition::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_APPEARANCE, "Appearance") => {
            Ok(Box::new(super::Appearance::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "Bridge") => {
            Ok(Box::new(super::Bridge::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgeConstructiveElement") => {
            Ok(
                Box::new(
                    super::BridgeConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeFurniture") => {
            Ok(Box::new(super::BridgeFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgeInstallation") => {
            Ok(Box::new(super::BridgeInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgePart") => {
            Ok(Box::new(super::BridgePart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgeRoom") => {
            Ok(Box::new(super::BridgeRoom::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "Building") => {
            Ok(Box::new(super::Building::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingConstructiveElement") => {
            Ok(
                Box::new(
                    super::BuildingConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingFurniture") => {
            Ok(Box::new(super::BuildingFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingInstallation") => {
            Ok(Box::new(super::BuildingInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingPart") => {
            Ok(Box::new(super::BuildingPart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingRoom") => {
            Ok(Box::new(super::BuildingRoom::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingUnit") => {
            Ok(Box::new(super::BuildingUnit::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "Storey") => {
            Ok(Box::new(super::Storey::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CITY_FURNITURE, "CityFurniture") => {
            Ok(Box::new(super::CityFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CITY_OBJECT_GROUP, "CityObjectGroup") => {
            Ok(Box::new(super::CityObjectGroup::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CORE, "CityModel") => {
            Ok(Box::new(super::CityModel::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CORE, "ClosureSurface") => {
            Ok(Box::new(super::ClosureSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericLogicalSpace") => {
            Ok(Box::new(super::GenericLogicalSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericOccupiedSpace") => {
            Ok(Box::new(super::GenericOccupiedSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericThematicSurface") => {
            Ok(
                Box::new(
                    super::GenericThematicSurface::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericUnoccupiedSpace") => {
            Ok(
                Box::new(
                    super::GenericUnoccupiedSpace::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_LAND_USE, "LandUse") => {
            Ok(Box::new(super::LandUse::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "BreaklineRelief") => {
            Ok(Box::new(super::BreaklineRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "MassPointRelief") => {
            Ok(Box::new(super::MassPointRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "RasterRelief") => {
            Ok(Box::new(super::RasterRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "ReliefFeature") => {
            Ok(Box::new(super::ReliefFeature::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "TINRelief") => {
            Ok(Box::new(super::TINRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficArea") => {
            Ok(Box::new(super::AuxiliaryTrafficArea::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficSpace") => {
            Ok(Box::new(super::AuxiliaryTrafficSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "ClearanceSpace") => {
            Ok(Box::new(super::ClearanceSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Hole") => {
            Ok(Box::new(super::Hole::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "HoleSurface") => {
            Ok(Box::new(super::HoleSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Intersection") => {
            Ok(Box::new(super::Intersection::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Marking") => {
            Ok(Box::new(super::Marking::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Railway") => {
            Ok(Box::new(super::Railway::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Road") => {
            Ok(Box::new(super::Road::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Section") => {
            Ok(Box::new(super::Section::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Square") => {
            Ok(Box::new(super::Square::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Track") => {
            Ok(Box::new(super::Track::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficArea") => {
            Ok(Box::new(super::TrafficArea::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficSpace") => {
            Ok(Box::new(super::TrafficSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Waterway") => {
            Ok(Box::new(super::Waterway::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "HollowSpace") => {
            Ok(Box::new(super::HollowSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "Tunnel") => {
            Ok(Box::new(super::Tunnel::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelConstructiveElement") => {
            Ok(
                Box::new(
                    super::TunnelConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelFurniture") => {
            Ok(Box::new(super::TunnelFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelInstallation") => {
            Ok(Box::new(super::TunnelInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelPart") => {
            Ok(Box::new(super::TunnelPart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_VEGETATION, "PlantCover") => {
            Ok(Box::new(super::PlantCover::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_VEGETATION, "SolitaryVegetationObject") => {
            Ok(
                Box::new(
                    super::SolitaryVegetationObject::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterBody") => {
            Ok(Box::new(super::WaterBody::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_WATER_BODY, "WaterGroundSurface") => {
            Ok(Box::new(super::WaterGroundSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_WATER_BODY, "WaterSurface") => {
            Ok(Box::new(super::WaterSurface::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_logical_space(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractLogicalSpace>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_BUILDING, "BuildingUnit") => {
            Ok(Box::new(super::BuildingUnit::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "Storey") => {
            Ok(Box::new(super::Storey::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CITY_OBJECT_GROUP, "CityObjectGroup") => {
            Ok(Box::new(super::CityObjectGroup::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericLogicalSpace") => {
            Ok(Box::new(super::GenericLogicalSpace::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_occupied_space(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractOccupiedSpace>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "Door") => {
            Ok(Box::new(super::Door::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OtherConstruction") => {
            Ok(Box::new(super::OtherConstruction::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "Window") => {
            Ok(Box::new(super::Window::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "Bridge") => {
            Ok(Box::new(super::Bridge::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgeConstructiveElement") => {
            Ok(
                Box::new(
                    super::BridgeConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeFurniture") => {
            Ok(Box::new(super::BridgeFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgeInstallation") => {
            Ok(Box::new(super::BridgeInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgePart") => {
            Ok(Box::new(super::BridgePart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "Building") => {
            Ok(Box::new(super::Building::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingConstructiveElement") => {
            Ok(
                Box::new(
                    super::BuildingConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingFurniture") => {
            Ok(Box::new(super::BuildingFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingInstallation") => {
            Ok(Box::new(super::BuildingInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingPart") => {
            Ok(Box::new(super::BuildingPart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CITY_FURNITURE, "CityFurniture") => {
            Ok(Box::new(super::CityFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericOccupiedSpace") => {
            Ok(Box::new(super::GenericOccupiedSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "Tunnel") => {
            Ok(Box::new(super::Tunnel::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelConstructiveElement") => {
            Ok(
                Box::new(
                    super::TunnelConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelFurniture") => {
            Ok(Box::new(super::TunnelFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelInstallation") => {
            Ok(Box::new(super::TunnelInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelPart") => {
            Ok(Box::new(super::TunnelPart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_VEGETATION, "PlantCover") => {
            Ok(Box::new(super::PlantCover::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_VEGETATION, "SolitaryVegetationObject") => {
            Ok(
                Box::new(
                    super::SolitaryVegetationObject::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterBody") => {
            Ok(Box::new(super::WaterBody::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_physical_space(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractPhysicalSpace>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "Door") => {
            Ok(Box::new(super::Door::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OtherConstruction") => {
            Ok(Box::new(super::OtherConstruction::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "Window") => {
            Ok(Box::new(super::Window::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "Bridge") => {
            Ok(Box::new(super::Bridge::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgeConstructiveElement") => {
            Ok(
                Box::new(
                    super::BridgeConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeFurniture") => {
            Ok(Box::new(super::BridgeFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgeInstallation") => {
            Ok(Box::new(super::BridgeInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgePart") => {
            Ok(Box::new(super::BridgePart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgeRoom") => {
            Ok(Box::new(super::BridgeRoom::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "Building") => {
            Ok(Box::new(super::Building::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingConstructiveElement") => {
            Ok(
                Box::new(
                    super::BuildingConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingFurniture") => {
            Ok(Box::new(super::BuildingFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingInstallation") => {
            Ok(Box::new(super::BuildingInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingPart") => {
            Ok(Box::new(super::BuildingPart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingRoom") => {
            Ok(Box::new(super::BuildingRoom::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CITY_FURNITURE, "CityFurniture") => {
            Ok(Box::new(super::CityFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericOccupiedSpace") => {
            Ok(Box::new(super::GenericOccupiedSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericUnoccupiedSpace") => {
            Ok(
                Box::new(
                    super::GenericUnoccupiedSpace::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficSpace") => {
            Ok(Box::new(super::AuxiliaryTrafficSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "ClearanceSpace") => {
            Ok(Box::new(super::ClearanceSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Hole") => {
            Ok(Box::new(super::Hole::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Intersection") => {
            Ok(Box::new(super::Intersection::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Railway") => {
            Ok(Box::new(super::Railway::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Road") => {
            Ok(Box::new(super::Road::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Section") => {
            Ok(Box::new(super::Section::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Square") => {
            Ok(Box::new(super::Square::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Track") => {
            Ok(Box::new(super::Track::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficSpace") => {
            Ok(Box::new(super::TrafficSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Waterway") => {
            Ok(Box::new(super::Waterway::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "HollowSpace") => {
            Ok(Box::new(super::HollowSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "Tunnel") => {
            Ok(Box::new(super::Tunnel::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelConstructiveElement") => {
            Ok(
                Box::new(
                    super::TunnelConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelFurniture") => {
            Ok(Box::new(super::TunnelFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelInstallation") => {
            Ok(Box::new(super::TunnelInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelPart") => {
            Ok(Box::new(super::TunnelPart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_VEGETATION, "PlantCover") => {
            Ok(Box::new(super::PlantCover::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_VEGETATION, "SolitaryVegetationObject") => {
            Ok(
                Box::new(
                    super::SolitaryVegetationObject::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterBody") => {
            Ok(Box::new(super::WaterBody::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_point_cloud(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractPointCloud>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_POINT_CLOUD, "PointCloud") => {
            Ok(Box::new(super::PointCloud::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_space(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractSpace>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "Door") => {
            Ok(Box::new(super::Door::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OtherConstruction") => {
            Ok(Box::new(super::OtherConstruction::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "Window") => {
            Ok(Box::new(super::Window::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "Bridge") => {
            Ok(Box::new(super::Bridge::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgeConstructiveElement") => {
            Ok(
                Box::new(
                    super::BridgeConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeFurniture") => {
            Ok(Box::new(super::BridgeFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgeInstallation") => {
            Ok(Box::new(super::BridgeInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgePart") => {
            Ok(Box::new(super::BridgePart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BRIDGE, "BridgeRoom") => {
            Ok(Box::new(super::BridgeRoom::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "Building") => {
            Ok(Box::new(super::Building::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingConstructiveElement") => {
            Ok(
                Box::new(
                    super::BuildingConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingFurniture") => {
            Ok(Box::new(super::BuildingFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingInstallation") => {
            Ok(Box::new(super::BuildingInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingPart") => {
            Ok(Box::new(super::BuildingPart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingRoom") => {
            Ok(Box::new(super::BuildingRoom::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingUnit") => {
            Ok(Box::new(super::BuildingUnit::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "Storey") => {
            Ok(Box::new(super::Storey::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CITY_FURNITURE, "CityFurniture") => {
            Ok(Box::new(super::CityFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CITY_OBJECT_GROUP, "CityObjectGroup") => {
            Ok(Box::new(super::CityObjectGroup::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericLogicalSpace") => {
            Ok(Box::new(super::GenericLogicalSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericOccupiedSpace") => {
            Ok(Box::new(super::GenericOccupiedSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericUnoccupiedSpace") => {
            Ok(
                Box::new(
                    super::GenericUnoccupiedSpace::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficSpace") => {
            Ok(Box::new(super::AuxiliaryTrafficSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "ClearanceSpace") => {
            Ok(Box::new(super::ClearanceSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Hole") => {
            Ok(Box::new(super::Hole::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Intersection") => {
            Ok(Box::new(super::Intersection::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Railway") => {
            Ok(Box::new(super::Railway::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Road") => {
            Ok(Box::new(super::Road::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Section") => {
            Ok(Box::new(super::Section::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Square") => {
            Ok(Box::new(super::Square::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Track") => {
            Ok(Box::new(super::Track::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficSpace") => {
            Ok(Box::new(super::TrafficSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Waterway") => {
            Ok(Box::new(super::Waterway::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "HollowSpace") => {
            Ok(Box::new(super::HollowSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "Tunnel") => {
            Ok(Box::new(super::Tunnel::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelConstructiveElement") => {
            Ok(
                Box::new(
                    super::TunnelConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelFurniture") => {
            Ok(Box::new(super::TunnelFurniture::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelInstallation") => {
            Ok(Box::new(super::TunnelInstallation::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelPart") => {
            Ok(Box::new(super::TunnelPart::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_VEGETATION, "PlantCover") => {
            Ok(Box::new(super::PlantCover::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_VEGETATION, "SolitaryVegetationObject") => {
            Ok(
                Box::new(
                    super::SolitaryVegetationObject::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterBody") => {
            Ok(Box::new(super::WaterBody::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_space_boundary(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractSpaceBoundary>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "CeilingSurface") => {
            Ok(Box::new(super::CeilingSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "DoorSurface") => {
            Ok(Box::new(super::DoorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "FloorSurface") => {
            Ok(Box::new(super::FloorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "GroundSurface") => {
            Ok(Box::new(super::GroundSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "InteriorWallSurface") => {
            Ok(Box::new(super::InteriorWallSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterCeilingSurface") => {
            Ok(Box::new(super::OuterCeilingSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterFloorSurface") => {
            Ok(Box::new(super::OuterFloorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "RoofSurface") => {
            Ok(Box::new(super::RoofSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "WallSurface") => {
            Ok(Box::new(super::WallSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "WindowSurface") => {
            Ok(Box::new(super::WindowSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CORE, "ClosureSurface") => {
            Ok(Box::new(super::ClosureSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericThematicSurface") => {
            Ok(
                Box::new(
                    super::GenericThematicSurface::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_LAND_USE, "LandUse") => {
            Ok(Box::new(super::LandUse::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "BreaklineRelief") => {
            Ok(Box::new(super::BreaklineRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "MassPointRelief") => {
            Ok(Box::new(super::MassPointRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "RasterRelief") => {
            Ok(Box::new(super::RasterRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "ReliefFeature") => {
            Ok(Box::new(super::ReliefFeature::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "TINRelief") => {
            Ok(Box::new(super::TINRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficArea") => {
            Ok(Box::new(super::AuxiliaryTrafficArea::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "HoleSurface") => {
            Ok(Box::new(super::HoleSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Marking") => {
            Ok(Box::new(super::Marking::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficArea") => {
            Ok(Box::new(super::TrafficArea::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_WATER_BODY, "WaterGroundSurface") => {
            Ok(Box::new(super::WaterGroundSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_WATER_BODY, "WaterSurface") => {
            Ok(Box::new(super::WaterSurface::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_thematic_surface(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractThematicSurface>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "CeilingSurface") => {
            Ok(Box::new(super::CeilingSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "DoorSurface") => {
            Ok(Box::new(super::DoorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "FloorSurface") => {
            Ok(Box::new(super::FloorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "GroundSurface") => {
            Ok(Box::new(super::GroundSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "InteriorWallSurface") => {
            Ok(Box::new(super::InteriorWallSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterCeilingSurface") => {
            Ok(Box::new(super::OuterCeilingSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterFloorSurface") => {
            Ok(Box::new(super::OuterFloorSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "RoofSurface") => {
            Ok(Box::new(super::RoofSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "WallSurface") => {
            Ok(Box::new(super::WallSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CONSTRUCTION, "WindowSurface") => {
            Ok(Box::new(super::WindowSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_CORE, "ClosureSurface") => {
            Ok(Box::new(super::ClosureSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericThematicSurface") => {
            Ok(
                Box::new(
                    super::GenericThematicSurface::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_LAND_USE, "LandUse") => {
            Ok(Box::new(super::LandUse::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficArea") => {
            Ok(Box::new(super::AuxiliaryTrafficArea::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "HoleSurface") => {
            Ok(Box::new(super::HoleSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Marking") => {
            Ok(Box::new(super::Marking::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficArea") => {
            Ok(Box::new(super::TrafficArea::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_WATER_BODY, "WaterGroundSurface") => {
            Ok(Box::new(super::WaterGroundSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_WATER_BODY, "WaterSurface") => {
            Ok(Box::new(super::WaterSurface::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_unoccupied_space(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractUnoccupiedSpace>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_BRIDGE, "BridgeRoom") => {
            Ok(Box::new(super::BridgeRoom::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_BUILDING, "BuildingRoom") => {
            Ok(Box::new(super::BuildingRoom::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_GENERICS, "GenericUnoccupiedSpace") => {
            Ok(
                Box::new(
                    super::GenericUnoccupiedSpace::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficSpace") => {
            Ok(Box::new(super::AuxiliaryTrafficSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "ClearanceSpace") => {
            Ok(Box::new(super::ClearanceSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Hole") => {
            Ok(Box::new(super::Hole::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Intersection") => {
            Ok(Box::new(super::Intersection::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Railway") => {
            Ok(Box::new(super::Railway::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Road") => {
            Ok(Box::new(super::Road::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Section") => {
            Ok(Box::new(super::Section::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Square") => {
            Ok(Box::new(super::Square::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Track") => {
            Ok(Box::new(super::Track::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficSpace") => {
            Ok(Box::new(super::TrafficSpace::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Waterway") => {
            Ok(Box::new(super::Waterway::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "HollowSpace") => {
            Ok(Box::new(super::HollowSpace::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_version(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractVersion>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_VERSIONING, "Version") => {
            Ok(Box::new(super::Version::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_version_transition(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractVersionTransition>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_VERSIONING, "VersionTransition") => {
            Ok(Box::new(super::VersionTransition::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_relief_component(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractReliefComponent>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_RELIEF, "BreaklineRelief") => {
            Ok(Box::new(super::BreaklineRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "MassPointRelief") => {
            Ok(Box::new(super::MassPointRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "RasterRelief") => {
            Ok(Box::new(super::RasterRelief::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_RELIEF, "TINRelief") => {
            Ok(Box::new(super::TINRelief::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_transportation_space(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractTransportationSpace>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_TRANSPORTATION, "Intersection") => {
            Ok(Box::new(super::Intersection::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Railway") => {
            Ok(Box::new(super::Railway::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Road") => {
            Ok(Box::new(super::Road::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Section") => {
            Ok(Box::new(super::Section::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Square") => {
            Ok(Box::new(super::Square::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Track") => {
            Ok(Box::new(super::Track::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TRANSPORTATION, "Waterway") => {
            Ok(Box::new(super::Waterway::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_tunnel(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractTunnel>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_TUNNEL, "Tunnel") => {
            Ok(Box::new(super::Tunnel::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_TUNNEL, "TunnelPart") => {
            Ok(Box::new(super::TunnelPart::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_vegetation_object(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractVegetationObject>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_VEGETATION, "PlantCover") => {
            Ok(Box::new(super::PlantCover::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_VEGETATION, "SolitaryVegetationObject") => {
            Ok(
                Box::new(
                    super::SolitaryVegetationObject::from_gml_with_info(reader, info)?,
                ),
            )
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
pub fn parse_dyn_abstract_water_boundary_surface(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<Box<dyn super::AbstractWaterBoundarySurface>, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_WATER_BODY, "WaterGroundSurface") => {
            Ok(Box::new(super::WaterGroundSurface::from_gml_with_info(reader, info)?))
        }
        (crate::namespace::NS_WATER_BODY, "WaterSurface") => {
            Ok(Box::new(super::WaterSurface::from_gml_with_info(reader, info)?))
        }
        _ => {
            Err(crate::error::ReaderError::UnsupportedFeature {
                namespace: info.namespace.clone(),
                local_name: info.local_name.clone(),
            })
        }
    }
}
