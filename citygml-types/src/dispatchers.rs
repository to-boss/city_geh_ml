#![allow(unused_imports, unused_mut, unused_variables)]
use super::*;

pub fn parse_abstract_construction(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractConstruction, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "OtherConstruction") => {
            Ok(
                super::AbstractConstruction::OtherConstruction(
                    super::OtherConstruction::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "Bridge") => {
            Ok(
                super::AbstractConstruction::Bridge(
                    super::Bridge::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgePart") => {
            Ok(
                super::AbstractConstruction::BridgePart(
                    super::BridgePart::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "Building") => {
            Ok(
                super::AbstractConstruction::Building(
                    super::Building::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingPart") => {
            Ok(
                super::AbstractConstruction::BuildingPart(
                    super::BuildingPart::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "Tunnel") => {
            Ok(
                super::AbstractConstruction::Tunnel(
                    super::Tunnel::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelPart") => {
            Ok(
                super::AbstractConstruction::TunnelPart(
                    super::TunnelPart::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_construction_surface(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractConstructionSurface, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "CeilingSurface") => {
            Ok(
                super::AbstractConstructionSurface::CeilingSurface(
                    super::CeilingSurface::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "FloorSurface") => {
            Ok(
                super::AbstractConstructionSurface::FloorSurface(
                    super::FloorSurface::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "GroundSurface") => {
            Ok(
                super::AbstractConstructionSurface::GroundSurface(
                    super::GroundSurface::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "InteriorWallSurface") => {
            Ok(
                super::AbstractConstructionSurface::InteriorWallSurface(
                    super::InteriorWallSurface::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterCeilingSurface") => {
            Ok(
                super::AbstractConstructionSurface::OuterCeilingSurface(
                    super::OuterCeilingSurface::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterFloorSurface") => {
            Ok(
                super::AbstractConstructionSurface::OuterFloorSurface(
                    super::OuterFloorSurface::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "RoofSurface") => {
            Ok(
                super::AbstractConstructionSurface::RoofSurface(
                    super::RoofSurface::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "WallSurface") => {
            Ok(
                super::AbstractConstructionSurface::WallSurface(
                    super::WallSurface::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_constructive_element(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractConstructiveElement, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_BRIDGE, "BridgeConstructiveElement") => {
            Ok(
                super::AbstractConstructiveElement::BridgeConstructiveElement(
                    super::BridgeConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingConstructiveElement") => {
            Ok(
                super::AbstractConstructiveElement::BuildingConstructiveElement(
                    super::BuildingConstructiveElement::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelConstructiveElement") => {
            Ok(
                super::AbstractConstructiveElement::TunnelConstructiveElement(
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

pub fn parse_abstract_filling_element(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractFillingElement, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "Door") => {
            Ok(
                super::AbstractFillingElement::Door(
                    super::Door::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "Window") => {
            Ok(
                super::AbstractFillingElement::Window(
                    super::Window::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_filling_surface(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractFillingSurface, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "DoorSurface") => {
            Ok(
                super::AbstractFillingSurface::DoorSurface(
                    super::DoorSurface::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "WindowSurface") => {
            Ok(
                super::AbstractFillingSurface::WindowSurface(
                    super::WindowSurface::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_furniture(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractFurniture, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_BRIDGE, "BridgeFurniture") => {
            Ok(
                super::AbstractFurniture::BridgeFurniture(
                    super::BridgeFurniture::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingFurniture") => {
            Ok(
                super::AbstractFurniture::BuildingFurniture(
                    super::BuildingFurniture::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelFurniture") => {
            Ok(
                super::AbstractFurniture::TunnelFurniture(
                    super::TunnelFurniture::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_installation(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractInstallation, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_BRIDGE, "BridgeInstallation") => {
            Ok(
                super::AbstractInstallation::BridgeInstallation(
                    super::BridgeInstallation::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingInstallation") => {
            Ok(
                super::AbstractInstallation::BuildingInstallation(
                    super::BuildingInstallation::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelInstallation") => {
            Ok(
                super::AbstractInstallation::TunnelInstallation(
                    super::TunnelInstallation::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_atomic_timeseries(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractAtomicTimeseries, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_DYNAMIZER, "GenericTimeseries") => {
            Ok(
                super::AbstractAtomicTimeseries::GenericTimeseries(
                    super::GenericTimeseries::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_DYNAMIZER, "StandardFileTimeseries") => {
            Ok(
                super::AbstractAtomicTimeseries::StandardFileTimeseries(
                    super::StandardFileTimeseries::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_DYNAMIZER, "TabulatedFileTimeseries") => {
            Ok(
                super::AbstractAtomicTimeseries::TabulatedFileTimeseries(
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

pub fn parse_abstract_timeseries(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractTimeseries, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_DYNAMIZER, "CompositeTimeseries") => {
            Ok(
                super::AbstractTimeseries::CompositeTimeseries(
                    super::CompositeTimeseries::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_DYNAMIZER, "GenericTimeseries") => {
            Ok(
                super::AbstractTimeseries::GenericTimeseries(
                    super::GenericTimeseries::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_DYNAMIZER, "StandardFileTimeseries") => {
            Ok(
                super::AbstractTimeseries::StandardFileTimeseries(
                    super::StandardFileTimeseries::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_DYNAMIZER, "TabulatedFileTimeseries") => {
            Ok(
                super::AbstractTimeseries::TabulatedFileTimeseries(
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

pub fn parse_abstract_surface_data(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractSurfaceData, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_APPEARANCE, "GeoreferencedTexture") => {
            Ok(
                super::AbstractSurfaceData::GeoreferencedTexture(
                    super::GeoreferencedTexture::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_APPEARANCE, "ParameterizedTexture") => {
            Ok(
                super::AbstractSurfaceData::ParameterizedTexture(
                    super::ParameterizedTexture::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_APPEARANCE, "X3DMaterial") => {
            Ok(
                super::AbstractSurfaceData::X3DMaterial(
                    super::X3DMaterial::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_texture(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractTexture, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_APPEARANCE, "GeoreferencedTexture") => {
            Ok(
                super::AbstractTexture::GeoreferencedTexture(
                    super::GeoreferencedTexture::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_APPEARANCE, "ParameterizedTexture") => {
            Ok(
                super::AbstractTexture::ParameterizedTexture(
                    super::ParameterizedTexture::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_bridge(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractBridge, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_BRIDGE, "Bridge") => {
            Ok(
                super::AbstractBridge::Bridge(
                    super::Bridge::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgePart") => {
            Ok(
                super::AbstractBridge::BridgePart(
                    super::BridgePart::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_building(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractBuilding, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_BUILDING, "Building") => {
            Ok(
                super::AbstractBuilding::Building(
                    super::Building::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingPart") => {
            Ok(
                super::AbstractBuilding::BuildingPart(
                    super::BuildingPart::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_building_subdivision(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractBuildingSubdivision, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_BUILDING, "BuildingUnit") => {
            Ok(
                super::AbstractBuildingSubdivision::BuildingUnit(
                    super::BuildingUnit::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "Storey") => {
            Ok(
                super::AbstractBuildingSubdivision::Storey(
                    super::Storey::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_appearance(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractAppearance, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_APPEARANCE, "Appearance") => {
            Ok(
                super::AbstractAppearance::Appearance(
                    super::Appearance::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_city_object(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractCityObject, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "CeilingSurface") => {
            Ok(
                super::AbstractCityObject::CeilingSurface(
                    Box::new(super::CeilingSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "Door") => {
            Ok(
                super::AbstractCityObject::Door(
                    Box::new(super::Door::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "DoorSurface") => {
            Ok(
                super::AbstractCityObject::DoorSurface(
                    Box::new(super::DoorSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "FloorSurface") => {
            Ok(
                super::AbstractCityObject::FloorSurface(
                    Box::new(super::FloorSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "GroundSurface") => {
            Ok(
                super::AbstractCityObject::GroundSurface(
                    Box::new(super::GroundSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "InteriorWallSurface") => {
            Ok(
                super::AbstractCityObject::InteriorWallSurface(
                    Box::new(
                        super::InteriorWallSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OtherConstruction") => {
            Ok(
                super::AbstractCityObject::OtherConstruction(
                    Box::new(super::OtherConstruction::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterCeilingSurface") => {
            Ok(
                super::AbstractCityObject::OuterCeilingSurface(
                    Box::new(
                        super::OuterCeilingSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterFloorSurface") => {
            Ok(
                super::AbstractCityObject::OuterFloorSurface(
                    Box::new(super::OuterFloorSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "RoofSurface") => {
            Ok(
                super::AbstractCityObject::RoofSurface(
                    Box::new(super::RoofSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "WallSurface") => {
            Ok(
                super::AbstractCityObject::WallSurface(
                    Box::new(super::WallSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "Window") => {
            Ok(
                super::AbstractCityObject::Window(
                    Box::new(super::Window::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "WindowSurface") => {
            Ok(
                super::AbstractCityObject::WindowSurface(
                    Box::new(super::WindowSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "Bridge") => {
            Ok(
                super::AbstractCityObject::Bridge(
                    Box::new(super::Bridge::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeConstructiveElement") => {
            Ok(
                super::AbstractCityObject::BridgeConstructiveElement(
                    Box::new(
                        super::BridgeConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeFurniture") => {
            Ok(
                super::AbstractCityObject::BridgeFurniture(
                    Box::new(super::BridgeFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeInstallation") => {
            Ok(
                super::AbstractCityObject::BridgeInstallation(
                    Box::new(
                        super::BridgeInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgePart") => {
            Ok(
                super::AbstractCityObject::BridgePart(
                    Box::new(super::BridgePart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeRoom") => {
            Ok(
                super::AbstractCityObject::BridgeRoom(
                    Box::new(super::BridgeRoom::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "Building") => {
            Ok(
                super::AbstractCityObject::Building(
                    Box::new(super::Building::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingConstructiveElement") => {
            Ok(
                super::AbstractCityObject::BuildingConstructiveElement(
                    Box::new(
                        super::BuildingConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingFurniture") => {
            Ok(
                super::AbstractCityObject::BuildingFurniture(
                    Box::new(super::BuildingFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingInstallation") => {
            Ok(
                super::AbstractCityObject::BuildingInstallation(
                    Box::new(
                        super::BuildingInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingPart") => {
            Ok(
                super::AbstractCityObject::BuildingPart(
                    Box::new(super::BuildingPart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingRoom") => {
            Ok(
                super::AbstractCityObject::BuildingRoom(
                    Box::new(super::BuildingRoom::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingUnit") => {
            Ok(
                super::AbstractCityObject::BuildingUnit(
                    Box::new(super::BuildingUnit::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "Storey") => {
            Ok(
                super::AbstractCityObject::Storey(
                    Box::new(super::Storey::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CITY_FURNITURE, "CityFurniture") => {
            Ok(
                super::AbstractCityObject::CityFurniture(
                    Box::new(super::CityFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CITY_OBJECT_GROUP, "CityObjectGroup") => {
            Ok(
                super::AbstractCityObject::CityObjectGroup(
                    Box::new(super::CityObjectGroup::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CORE, "ClosureSurface") => {
            Ok(
                super::AbstractCityObject::ClosureSurface(
                    Box::new(super::ClosureSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericLogicalSpace") => {
            Ok(
                super::AbstractCityObject::GenericLogicalSpace(
                    Box::new(
                        super::GenericLogicalSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericOccupiedSpace") => {
            Ok(
                super::AbstractCityObject::GenericOccupiedSpace(
                    Box::new(
                        super::GenericOccupiedSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericThematicSurface") => {
            Ok(
                super::AbstractCityObject::GenericThematicSurface(
                    Box::new(
                        super::GenericThematicSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericUnoccupiedSpace") => {
            Ok(
                super::AbstractCityObject::GenericUnoccupiedSpace(
                    Box::new(
                        super::GenericUnoccupiedSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_LAND_USE, "LandUse") => {
            Ok(
                super::AbstractCityObject::LandUse(
                    Box::new(super::LandUse::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "BreaklineRelief") => {
            Ok(
                super::AbstractCityObject::BreaklineRelief(
                    Box::new(super::BreaklineRelief::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "MassPointRelief") => {
            Ok(
                super::AbstractCityObject::MassPointRelief(
                    Box::new(super::MassPointRelief::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "RasterRelief") => {
            Ok(
                super::AbstractCityObject::RasterRelief(
                    Box::new(super::RasterRelief::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "ReliefFeature") => {
            Ok(
                super::AbstractCityObject::ReliefFeature(
                    Box::new(super::ReliefFeature::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "TINRelief") => {
            Ok(
                super::AbstractCityObject::TINRelief(
                    Box::new(super::TINRelief::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficArea") => {
            Ok(
                super::AbstractCityObject::AuxiliaryTrafficArea(
                    Box::new(
                        super::AuxiliaryTrafficArea::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficSpace") => {
            Ok(
                super::AbstractCityObject::AuxiliaryTrafficSpace(
                    Box::new(
                        super::AuxiliaryTrafficSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "ClearanceSpace") => {
            Ok(
                super::AbstractCityObject::ClearanceSpace(
                    Box::new(super::ClearanceSpace::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Hole") => {
            Ok(
                super::AbstractCityObject::Hole(
                    Box::new(super::Hole::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "HoleSurface") => {
            Ok(
                super::AbstractCityObject::HoleSurface(
                    Box::new(super::HoleSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Intersection") => {
            Ok(
                super::AbstractCityObject::Intersection(
                    Box::new(super::Intersection::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Marking") => {
            Ok(
                super::AbstractCityObject::Marking(
                    Box::new(super::Marking::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Railway") => {
            Ok(
                super::AbstractCityObject::Railway(
                    Box::new(super::Railway::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Road") => {
            Ok(
                super::AbstractCityObject::Road(
                    Box::new(super::Road::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Section") => {
            Ok(
                super::AbstractCityObject::Section(
                    Box::new(super::Section::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Square") => {
            Ok(
                super::AbstractCityObject::Square(
                    Box::new(super::Square::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Track") => {
            Ok(
                super::AbstractCityObject::Track(
                    Box::new(super::Track::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficArea") => {
            Ok(
                super::AbstractCityObject::TrafficArea(
                    Box::new(super::TrafficArea::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficSpace") => {
            Ok(
                super::AbstractCityObject::TrafficSpace(
                    Box::new(super::TrafficSpace::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Waterway") => {
            Ok(
                super::AbstractCityObject::Waterway(
                    Box::new(super::Waterway::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "HollowSpace") => {
            Ok(
                super::AbstractCityObject::HollowSpace(
                    Box::new(super::HollowSpace::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "Tunnel") => {
            Ok(
                super::AbstractCityObject::Tunnel(
                    Box::new(super::Tunnel::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelConstructiveElement") => {
            Ok(
                super::AbstractCityObject::TunnelConstructiveElement(
                    Box::new(
                        super::TunnelConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelFurniture") => {
            Ok(
                super::AbstractCityObject::TunnelFurniture(
                    Box::new(super::TunnelFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelInstallation") => {
            Ok(
                super::AbstractCityObject::TunnelInstallation(
                    Box::new(
                        super::TunnelInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelPart") => {
            Ok(
                super::AbstractCityObject::TunnelPart(
                    Box::new(super::TunnelPart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_VEGETATION, "PlantCover") => {
            Ok(
                super::AbstractCityObject::PlantCover(
                    Box::new(super::PlantCover::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_VEGETATION, "SolitaryVegetationObject") => {
            Ok(
                super::AbstractCityObject::SolitaryVegetationObject(
                    Box::new(
                        super::SolitaryVegetationObject::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterBody") => {
            Ok(
                super::AbstractCityObject::WaterBody(
                    Box::new(super::WaterBody::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterGroundSurface") => {
            Ok(
                super::AbstractCityObject::WaterGroundSurface(
                    Box::new(
                        super::WaterGroundSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterSurface") => {
            Ok(
                super::AbstractCityObject::WaterSurface(
                    Box::new(super::WaterSurface::from_gml_with_info(reader, info)?),
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

pub fn parse_abstract_dynamizer(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractDynamizer, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_DYNAMIZER, "Dynamizer") => {
            Ok(
                super::AbstractDynamizer::Dynamizer(
                    super::Dynamizer::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_feature(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractFeature, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "CeilingSurface") => {
            Ok(
                super::AbstractFeature::CeilingSurface(
                    Box::new(super::CeilingSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "Door") => {
            Ok(
                super::AbstractFeature::Door(
                    Box::new(super::Door::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "DoorSurface") => {
            Ok(
                super::AbstractFeature::DoorSurface(
                    Box::new(super::DoorSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "FloorSurface") => {
            Ok(
                super::AbstractFeature::FloorSurface(
                    Box::new(super::FloorSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "GroundSurface") => {
            Ok(
                super::AbstractFeature::GroundSurface(
                    Box::new(super::GroundSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "InteriorWallSurface") => {
            Ok(
                super::AbstractFeature::InteriorWallSurface(
                    Box::new(
                        super::InteriorWallSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OtherConstruction") => {
            Ok(
                super::AbstractFeature::OtherConstruction(
                    Box::new(super::OtherConstruction::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterCeilingSurface") => {
            Ok(
                super::AbstractFeature::OuterCeilingSurface(
                    Box::new(
                        super::OuterCeilingSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterFloorSurface") => {
            Ok(
                super::AbstractFeature::OuterFloorSurface(
                    Box::new(super::OuterFloorSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "RoofSurface") => {
            Ok(
                super::AbstractFeature::RoofSurface(
                    Box::new(super::RoofSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "WallSurface") => {
            Ok(
                super::AbstractFeature::WallSurface(
                    Box::new(super::WallSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "Window") => {
            Ok(
                super::AbstractFeature::Window(
                    Box::new(super::Window::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "WindowSurface") => {
            Ok(
                super::AbstractFeature::WindowSurface(
                    Box::new(super::WindowSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_DYNAMIZER, "CompositeTimeseries") => {
            Ok(
                super::AbstractFeature::CompositeTimeseries(
                    Box::new(
                        super::CompositeTimeseries::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_DYNAMIZER, "Dynamizer") => {
            Ok(
                super::AbstractFeature::Dynamizer(
                    Box::new(super::Dynamizer::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_DYNAMIZER, "GenericTimeseries") => {
            Ok(
                super::AbstractFeature::GenericTimeseries(
                    Box::new(super::GenericTimeseries::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_DYNAMIZER, "StandardFileTimeseries") => {
            Ok(
                super::AbstractFeature::StandardFileTimeseries(
                    Box::new(
                        super::StandardFileTimeseries::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_DYNAMIZER, "TabulatedFileTimeseries") => {
            Ok(
                super::AbstractFeature::TabulatedFileTimeseries(
                    Box::new(
                        super::TabulatedFileTimeseries::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_POINT_CLOUD, "PointCloud") => {
            Ok(
                super::AbstractFeature::PointCloud(
                    Box::new(super::PointCloud::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_VERSIONING, "Version") => {
            Ok(
                super::AbstractFeature::Version(
                    Box::new(super::Version::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_VERSIONING, "VersionTransition") => {
            Ok(
                super::AbstractFeature::VersionTransition(
                    Box::new(super::VersionTransition::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_APPEARANCE, "Appearance") => {
            Ok(
                super::AbstractFeature::Appearance(
                    Box::new(super::Appearance::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_APPEARANCE, "GeoreferencedTexture") => {
            Ok(
                super::AbstractFeature::GeoreferencedTexture(
                    Box::new(
                        super::GeoreferencedTexture::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_APPEARANCE, "ParameterizedTexture") => {
            Ok(
                super::AbstractFeature::ParameterizedTexture(
                    Box::new(
                        super::ParameterizedTexture::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_APPEARANCE, "X3DMaterial") => {
            Ok(
                super::AbstractFeature::X3DMaterial(
                    Box::new(super::X3DMaterial::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "Bridge") => {
            Ok(
                super::AbstractFeature::Bridge(
                    Box::new(super::Bridge::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeConstructiveElement") => {
            Ok(
                super::AbstractFeature::BridgeConstructiveElement(
                    Box::new(
                        super::BridgeConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeFurniture") => {
            Ok(
                super::AbstractFeature::BridgeFurniture(
                    Box::new(super::BridgeFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeInstallation") => {
            Ok(
                super::AbstractFeature::BridgeInstallation(
                    Box::new(
                        super::BridgeInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgePart") => {
            Ok(
                super::AbstractFeature::BridgePart(
                    Box::new(super::BridgePart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeRoom") => {
            Ok(
                super::AbstractFeature::BridgeRoom(
                    Box::new(super::BridgeRoom::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "Building") => {
            Ok(
                super::AbstractFeature::Building(
                    Box::new(super::Building::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingConstructiveElement") => {
            Ok(
                super::AbstractFeature::BuildingConstructiveElement(
                    Box::new(
                        super::BuildingConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingFurniture") => {
            Ok(
                super::AbstractFeature::BuildingFurniture(
                    Box::new(super::BuildingFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingInstallation") => {
            Ok(
                super::AbstractFeature::BuildingInstallation(
                    Box::new(
                        super::BuildingInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingPart") => {
            Ok(
                super::AbstractFeature::BuildingPart(
                    Box::new(super::BuildingPart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingRoom") => {
            Ok(
                super::AbstractFeature::BuildingRoom(
                    Box::new(super::BuildingRoom::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingUnit") => {
            Ok(
                super::AbstractFeature::BuildingUnit(
                    Box::new(super::BuildingUnit::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "Storey") => {
            Ok(
                super::AbstractFeature::Storey(
                    Box::new(super::Storey::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CITY_FURNITURE, "CityFurniture") => {
            Ok(
                super::AbstractFeature::CityFurniture(
                    Box::new(super::CityFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CITY_OBJECT_GROUP, "CityObjectGroup") => {
            Ok(
                super::AbstractFeature::CityObjectGroup(
                    Box::new(super::CityObjectGroup::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CORE, "Address") => {
            Ok(
                super::AbstractFeature::Address(
                    Box::new(super::Address::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CORE, "CityModel") => {
            Ok(
                super::AbstractFeature::CityModel(
                    Box::new(super::CityModel::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CORE, "ClosureSurface") => {
            Ok(
                super::AbstractFeature::ClosureSurface(
                    Box::new(super::ClosureSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericLogicalSpace") => {
            Ok(
                super::AbstractFeature::GenericLogicalSpace(
                    Box::new(
                        super::GenericLogicalSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericOccupiedSpace") => {
            Ok(
                super::AbstractFeature::GenericOccupiedSpace(
                    Box::new(
                        super::GenericOccupiedSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericThematicSurface") => {
            Ok(
                super::AbstractFeature::GenericThematicSurface(
                    Box::new(
                        super::GenericThematicSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericUnoccupiedSpace") => {
            Ok(
                super::AbstractFeature::GenericUnoccupiedSpace(
                    Box::new(
                        super::GenericUnoccupiedSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_LAND_USE, "LandUse") => {
            Ok(
                super::AbstractFeature::LandUse(
                    Box::new(super::LandUse::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "BreaklineRelief") => {
            Ok(
                super::AbstractFeature::BreaklineRelief(
                    Box::new(super::BreaklineRelief::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "MassPointRelief") => {
            Ok(
                super::AbstractFeature::MassPointRelief(
                    Box::new(super::MassPointRelief::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "RasterRelief") => {
            Ok(
                super::AbstractFeature::RasterRelief(
                    Box::new(super::RasterRelief::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "ReliefFeature") => {
            Ok(
                super::AbstractFeature::ReliefFeature(
                    Box::new(super::ReliefFeature::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "TINRelief") => {
            Ok(
                super::AbstractFeature::TINRelief(
                    Box::new(super::TINRelief::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficArea") => {
            Ok(
                super::AbstractFeature::AuxiliaryTrafficArea(
                    Box::new(
                        super::AuxiliaryTrafficArea::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficSpace") => {
            Ok(
                super::AbstractFeature::AuxiliaryTrafficSpace(
                    Box::new(
                        super::AuxiliaryTrafficSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "ClearanceSpace") => {
            Ok(
                super::AbstractFeature::ClearanceSpace(
                    Box::new(super::ClearanceSpace::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Hole") => {
            Ok(
                super::AbstractFeature::Hole(
                    Box::new(super::Hole::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "HoleSurface") => {
            Ok(
                super::AbstractFeature::HoleSurface(
                    Box::new(super::HoleSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Intersection") => {
            Ok(
                super::AbstractFeature::Intersection(
                    Box::new(super::Intersection::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Marking") => {
            Ok(
                super::AbstractFeature::Marking(
                    Box::new(super::Marking::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Railway") => {
            Ok(
                super::AbstractFeature::Railway(
                    Box::new(super::Railway::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Road") => {
            Ok(
                super::AbstractFeature::Road(
                    Box::new(super::Road::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Section") => {
            Ok(
                super::AbstractFeature::Section(
                    Box::new(super::Section::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Square") => {
            Ok(
                super::AbstractFeature::Square(
                    Box::new(super::Square::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Track") => {
            Ok(
                super::AbstractFeature::Track(
                    Box::new(super::Track::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficArea") => {
            Ok(
                super::AbstractFeature::TrafficArea(
                    Box::new(super::TrafficArea::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficSpace") => {
            Ok(
                super::AbstractFeature::TrafficSpace(
                    Box::new(super::TrafficSpace::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Waterway") => {
            Ok(
                super::AbstractFeature::Waterway(
                    Box::new(super::Waterway::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "HollowSpace") => {
            Ok(
                super::AbstractFeature::HollowSpace(
                    Box::new(super::HollowSpace::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "Tunnel") => {
            Ok(
                super::AbstractFeature::Tunnel(
                    Box::new(super::Tunnel::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelConstructiveElement") => {
            Ok(
                super::AbstractFeature::TunnelConstructiveElement(
                    Box::new(
                        super::TunnelConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelFurniture") => {
            Ok(
                super::AbstractFeature::TunnelFurniture(
                    Box::new(super::TunnelFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelInstallation") => {
            Ok(
                super::AbstractFeature::TunnelInstallation(
                    Box::new(
                        super::TunnelInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelPart") => {
            Ok(
                super::AbstractFeature::TunnelPart(
                    Box::new(super::TunnelPart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_VEGETATION, "PlantCover") => {
            Ok(
                super::AbstractFeature::PlantCover(
                    Box::new(super::PlantCover::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_VEGETATION, "SolitaryVegetationObject") => {
            Ok(
                super::AbstractFeature::SolitaryVegetationObject(
                    Box::new(
                        super::SolitaryVegetationObject::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterBody") => {
            Ok(
                super::AbstractFeature::WaterBody(
                    Box::new(super::WaterBody::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterGroundSurface") => {
            Ok(
                super::AbstractFeature::WaterGroundSurface(
                    Box::new(
                        super::WaterGroundSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterSurface") => {
            Ok(
                super::AbstractFeature::WaterSurface(
                    Box::new(super::WaterSurface::from_gml_with_info(reader, info)?),
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

pub fn parse_abstract_feature_with_lifespan(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractFeatureWithLifespan, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "CeilingSurface") => {
            Ok(
                super::AbstractFeatureWithLifespan::CeilingSurface(
                    Box::new(super::CeilingSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "Door") => {
            Ok(
                super::AbstractFeatureWithLifespan::Door(
                    Box::new(super::Door::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "DoorSurface") => {
            Ok(
                super::AbstractFeatureWithLifespan::DoorSurface(
                    Box::new(super::DoorSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "FloorSurface") => {
            Ok(
                super::AbstractFeatureWithLifespan::FloorSurface(
                    Box::new(super::FloorSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "GroundSurface") => {
            Ok(
                super::AbstractFeatureWithLifespan::GroundSurface(
                    Box::new(super::GroundSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "InteriorWallSurface") => {
            Ok(
                super::AbstractFeatureWithLifespan::InteriorWallSurface(
                    Box::new(
                        super::InteriorWallSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OtherConstruction") => {
            Ok(
                super::AbstractFeatureWithLifespan::OtherConstruction(
                    Box::new(super::OtherConstruction::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterCeilingSurface") => {
            Ok(
                super::AbstractFeatureWithLifespan::OuterCeilingSurface(
                    Box::new(
                        super::OuterCeilingSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterFloorSurface") => {
            Ok(
                super::AbstractFeatureWithLifespan::OuterFloorSurface(
                    Box::new(super::OuterFloorSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "RoofSurface") => {
            Ok(
                super::AbstractFeatureWithLifespan::RoofSurface(
                    Box::new(super::RoofSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "WallSurface") => {
            Ok(
                super::AbstractFeatureWithLifespan::WallSurface(
                    Box::new(super::WallSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "Window") => {
            Ok(
                super::AbstractFeatureWithLifespan::Window(
                    Box::new(super::Window::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "WindowSurface") => {
            Ok(
                super::AbstractFeatureWithLifespan::WindowSurface(
                    Box::new(super::WindowSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_DYNAMIZER, "Dynamizer") => {
            Ok(
                super::AbstractFeatureWithLifespan::Dynamizer(
                    Box::new(super::Dynamizer::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_VERSIONING, "Version") => {
            Ok(
                super::AbstractFeatureWithLifespan::Version(
                    Box::new(super::Version::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_VERSIONING, "VersionTransition") => {
            Ok(
                super::AbstractFeatureWithLifespan::VersionTransition(
                    Box::new(super::VersionTransition::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_APPEARANCE, "Appearance") => {
            Ok(
                super::AbstractFeatureWithLifespan::Appearance(
                    Box::new(super::Appearance::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "Bridge") => {
            Ok(
                super::AbstractFeatureWithLifespan::Bridge(
                    Box::new(super::Bridge::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeConstructiveElement") => {
            Ok(
                super::AbstractFeatureWithLifespan::BridgeConstructiveElement(
                    Box::new(
                        super::BridgeConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeFurniture") => {
            Ok(
                super::AbstractFeatureWithLifespan::BridgeFurniture(
                    Box::new(super::BridgeFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeInstallation") => {
            Ok(
                super::AbstractFeatureWithLifespan::BridgeInstallation(
                    Box::new(
                        super::BridgeInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgePart") => {
            Ok(
                super::AbstractFeatureWithLifespan::BridgePart(
                    Box::new(super::BridgePart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeRoom") => {
            Ok(
                super::AbstractFeatureWithLifespan::BridgeRoom(
                    Box::new(super::BridgeRoom::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "Building") => {
            Ok(
                super::AbstractFeatureWithLifespan::Building(
                    Box::new(super::Building::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingConstructiveElement") => {
            Ok(
                super::AbstractFeatureWithLifespan::BuildingConstructiveElement(
                    Box::new(
                        super::BuildingConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingFurniture") => {
            Ok(
                super::AbstractFeatureWithLifespan::BuildingFurniture(
                    Box::new(super::BuildingFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingInstallation") => {
            Ok(
                super::AbstractFeatureWithLifespan::BuildingInstallation(
                    Box::new(
                        super::BuildingInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingPart") => {
            Ok(
                super::AbstractFeatureWithLifespan::BuildingPart(
                    Box::new(super::BuildingPart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingRoom") => {
            Ok(
                super::AbstractFeatureWithLifespan::BuildingRoom(
                    Box::new(super::BuildingRoom::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingUnit") => {
            Ok(
                super::AbstractFeatureWithLifespan::BuildingUnit(
                    Box::new(super::BuildingUnit::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "Storey") => {
            Ok(
                super::AbstractFeatureWithLifespan::Storey(
                    Box::new(super::Storey::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CITY_FURNITURE, "CityFurniture") => {
            Ok(
                super::AbstractFeatureWithLifespan::CityFurniture(
                    Box::new(super::CityFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CITY_OBJECT_GROUP, "CityObjectGroup") => {
            Ok(
                super::AbstractFeatureWithLifespan::CityObjectGroup(
                    Box::new(super::CityObjectGroup::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CORE, "CityModel") => {
            Ok(
                super::AbstractFeatureWithLifespan::CityModel(
                    Box::new(super::CityModel::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CORE, "ClosureSurface") => {
            Ok(
                super::AbstractFeatureWithLifespan::ClosureSurface(
                    Box::new(super::ClosureSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericLogicalSpace") => {
            Ok(
                super::AbstractFeatureWithLifespan::GenericLogicalSpace(
                    Box::new(
                        super::GenericLogicalSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericOccupiedSpace") => {
            Ok(
                super::AbstractFeatureWithLifespan::GenericOccupiedSpace(
                    Box::new(
                        super::GenericOccupiedSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericThematicSurface") => {
            Ok(
                super::AbstractFeatureWithLifespan::GenericThematicSurface(
                    Box::new(
                        super::GenericThematicSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericUnoccupiedSpace") => {
            Ok(
                super::AbstractFeatureWithLifespan::GenericUnoccupiedSpace(
                    Box::new(
                        super::GenericUnoccupiedSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_LAND_USE, "LandUse") => {
            Ok(
                super::AbstractFeatureWithLifespan::LandUse(
                    Box::new(super::LandUse::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "BreaklineRelief") => {
            Ok(
                super::AbstractFeatureWithLifespan::BreaklineRelief(
                    Box::new(super::BreaklineRelief::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "MassPointRelief") => {
            Ok(
                super::AbstractFeatureWithLifespan::MassPointRelief(
                    Box::new(super::MassPointRelief::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "RasterRelief") => {
            Ok(
                super::AbstractFeatureWithLifespan::RasterRelief(
                    Box::new(super::RasterRelief::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "ReliefFeature") => {
            Ok(
                super::AbstractFeatureWithLifespan::ReliefFeature(
                    Box::new(super::ReliefFeature::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "TINRelief") => {
            Ok(
                super::AbstractFeatureWithLifespan::TINRelief(
                    Box::new(super::TINRelief::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficArea") => {
            Ok(
                super::AbstractFeatureWithLifespan::AuxiliaryTrafficArea(
                    Box::new(
                        super::AuxiliaryTrafficArea::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficSpace") => {
            Ok(
                super::AbstractFeatureWithLifespan::AuxiliaryTrafficSpace(
                    Box::new(
                        super::AuxiliaryTrafficSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "ClearanceSpace") => {
            Ok(
                super::AbstractFeatureWithLifespan::ClearanceSpace(
                    Box::new(super::ClearanceSpace::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Hole") => {
            Ok(
                super::AbstractFeatureWithLifespan::Hole(
                    Box::new(super::Hole::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "HoleSurface") => {
            Ok(
                super::AbstractFeatureWithLifespan::HoleSurface(
                    Box::new(super::HoleSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Intersection") => {
            Ok(
                super::AbstractFeatureWithLifespan::Intersection(
                    Box::new(super::Intersection::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Marking") => {
            Ok(
                super::AbstractFeatureWithLifespan::Marking(
                    Box::new(super::Marking::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Railway") => {
            Ok(
                super::AbstractFeatureWithLifespan::Railway(
                    Box::new(super::Railway::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Road") => {
            Ok(
                super::AbstractFeatureWithLifespan::Road(
                    Box::new(super::Road::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Section") => {
            Ok(
                super::AbstractFeatureWithLifespan::Section(
                    Box::new(super::Section::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Square") => {
            Ok(
                super::AbstractFeatureWithLifespan::Square(
                    Box::new(super::Square::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Track") => {
            Ok(
                super::AbstractFeatureWithLifespan::Track(
                    Box::new(super::Track::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficArea") => {
            Ok(
                super::AbstractFeatureWithLifespan::TrafficArea(
                    Box::new(super::TrafficArea::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficSpace") => {
            Ok(
                super::AbstractFeatureWithLifespan::TrafficSpace(
                    Box::new(super::TrafficSpace::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Waterway") => {
            Ok(
                super::AbstractFeatureWithLifespan::Waterway(
                    Box::new(super::Waterway::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "HollowSpace") => {
            Ok(
                super::AbstractFeatureWithLifespan::HollowSpace(
                    Box::new(super::HollowSpace::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "Tunnel") => {
            Ok(
                super::AbstractFeatureWithLifespan::Tunnel(
                    Box::new(super::Tunnel::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelConstructiveElement") => {
            Ok(
                super::AbstractFeatureWithLifespan::TunnelConstructiveElement(
                    Box::new(
                        super::TunnelConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelFurniture") => {
            Ok(
                super::AbstractFeatureWithLifespan::TunnelFurniture(
                    Box::new(super::TunnelFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelInstallation") => {
            Ok(
                super::AbstractFeatureWithLifespan::TunnelInstallation(
                    Box::new(
                        super::TunnelInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelPart") => {
            Ok(
                super::AbstractFeatureWithLifespan::TunnelPart(
                    Box::new(super::TunnelPart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_VEGETATION, "PlantCover") => {
            Ok(
                super::AbstractFeatureWithLifespan::PlantCover(
                    Box::new(super::PlantCover::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_VEGETATION, "SolitaryVegetationObject") => {
            Ok(
                super::AbstractFeatureWithLifespan::SolitaryVegetationObject(
                    Box::new(
                        super::SolitaryVegetationObject::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterBody") => {
            Ok(
                super::AbstractFeatureWithLifespan::WaterBody(
                    Box::new(super::WaterBody::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterGroundSurface") => {
            Ok(
                super::AbstractFeatureWithLifespan::WaterGroundSurface(
                    Box::new(
                        super::WaterGroundSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterSurface") => {
            Ok(
                super::AbstractFeatureWithLifespan::WaterSurface(
                    Box::new(super::WaterSurface::from_gml_with_info(reader, info)?),
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

pub fn parse_abstract_logical_space(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractLogicalSpace, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_BUILDING, "BuildingUnit") => {
            Ok(
                super::AbstractLogicalSpace::BuildingUnit(
                    super::BuildingUnit::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "Storey") => {
            Ok(
                super::AbstractLogicalSpace::Storey(
                    super::Storey::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_CITY_OBJECT_GROUP, "CityObjectGroup") => {
            Ok(
                super::AbstractLogicalSpace::CityObjectGroup(
                    super::CityObjectGroup::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericLogicalSpace") => {
            Ok(
                super::AbstractLogicalSpace::GenericLogicalSpace(
                    super::GenericLogicalSpace::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_occupied_space(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractOccupiedSpace, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "Door") => {
            Ok(
                super::AbstractOccupiedSpace::Door(
                    Box::new(super::Door::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OtherConstruction") => {
            Ok(
                super::AbstractOccupiedSpace::OtherConstruction(
                    Box::new(super::OtherConstruction::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "Window") => {
            Ok(
                super::AbstractOccupiedSpace::Window(
                    Box::new(super::Window::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "Bridge") => {
            Ok(
                super::AbstractOccupiedSpace::Bridge(
                    Box::new(super::Bridge::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeConstructiveElement") => {
            Ok(
                super::AbstractOccupiedSpace::BridgeConstructiveElement(
                    Box::new(
                        super::BridgeConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeFurniture") => {
            Ok(
                super::AbstractOccupiedSpace::BridgeFurniture(
                    Box::new(super::BridgeFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeInstallation") => {
            Ok(
                super::AbstractOccupiedSpace::BridgeInstallation(
                    Box::new(
                        super::BridgeInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgePart") => {
            Ok(
                super::AbstractOccupiedSpace::BridgePart(
                    Box::new(super::BridgePart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "Building") => {
            Ok(
                super::AbstractOccupiedSpace::Building(
                    Box::new(super::Building::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingConstructiveElement") => {
            Ok(
                super::AbstractOccupiedSpace::BuildingConstructiveElement(
                    Box::new(
                        super::BuildingConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingFurniture") => {
            Ok(
                super::AbstractOccupiedSpace::BuildingFurniture(
                    Box::new(super::BuildingFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingInstallation") => {
            Ok(
                super::AbstractOccupiedSpace::BuildingInstallation(
                    Box::new(
                        super::BuildingInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingPart") => {
            Ok(
                super::AbstractOccupiedSpace::BuildingPart(
                    Box::new(super::BuildingPart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CITY_FURNITURE, "CityFurniture") => {
            Ok(
                super::AbstractOccupiedSpace::CityFurniture(
                    Box::new(super::CityFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericOccupiedSpace") => {
            Ok(
                super::AbstractOccupiedSpace::GenericOccupiedSpace(
                    Box::new(
                        super::GenericOccupiedSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "Tunnel") => {
            Ok(
                super::AbstractOccupiedSpace::Tunnel(
                    Box::new(super::Tunnel::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelConstructiveElement") => {
            Ok(
                super::AbstractOccupiedSpace::TunnelConstructiveElement(
                    Box::new(
                        super::TunnelConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelFurniture") => {
            Ok(
                super::AbstractOccupiedSpace::TunnelFurniture(
                    Box::new(super::TunnelFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelInstallation") => {
            Ok(
                super::AbstractOccupiedSpace::TunnelInstallation(
                    Box::new(
                        super::TunnelInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelPart") => {
            Ok(
                super::AbstractOccupiedSpace::TunnelPart(
                    Box::new(super::TunnelPart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_VEGETATION, "PlantCover") => {
            Ok(
                super::AbstractOccupiedSpace::PlantCover(
                    Box::new(super::PlantCover::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_VEGETATION, "SolitaryVegetationObject") => {
            Ok(
                super::AbstractOccupiedSpace::SolitaryVegetationObject(
                    Box::new(
                        super::SolitaryVegetationObject::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterBody") => {
            Ok(
                super::AbstractOccupiedSpace::WaterBody(
                    Box::new(super::WaterBody::from_gml_with_info(reader, info)?),
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

pub fn parse_abstract_physical_space(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractPhysicalSpace, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "Door") => {
            Ok(
                super::AbstractPhysicalSpace::Door(
                    Box::new(super::Door::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OtherConstruction") => {
            Ok(
                super::AbstractPhysicalSpace::OtherConstruction(
                    Box::new(super::OtherConstruction::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "Window") => {
            Ok(
                super::AbstractPhysicalSpace::Window(
                    Box::new(super::Window::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "Bridge") => {
            Ok(
                super::AbstractPhysicalSpace::Bridge(
                    Box::new(super::Bridge::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeConstructiveElement") => {
            Ok(
                super::AbstractPhysicalSpace::BridgeConstructiveElement(
                    Box::new(
                        super::BridgeConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeFurniture") => {
            Ok(
                super::AbstractPhysicalSpace::BridgeFurniture(
                    Box::new(super::BridgeFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeInstallation") => {
            Ok(
                super::AbstractPhysicalSpace::BridgeInstallation(
                    Box::new(
                        super::BridgeInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgePart") => {
            Ok(
                super::AbstractPhysicalSpace::BridgePart(
                    Box::new(super::BridgePart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeRoom") => {
            Ok(
                super::AbstractPhysicalSpace::BridgeRoom(
                    Box::new(super::BridgeRoom::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "Building") => {
            Ok(
                super::AbstractPhysicalSpace::Building(
                    Box::new(super::Building::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingConstructiveElement") => {
            Ok(
                super::AbstractPhysicalSpace::BuildingConstructiveElement(
                    Box::new(
                        super::BuildingConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingFurniture") => {
            Ok(
                super::AbstractPhysicalSpace::BuildingFurniture(
                    Box::new(super::BuildingFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingInstallation") => {
            Ok(
                super::AbstractPhysicalSpace::BuildingInstallation(
                    Box::new(
                        super::BuildingInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingPart") => {
            Ok(
                super::AbstractPhysicalSpace::BuildingPart(
                    Box::new(super::BuildingPart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingRoom") => {
            Ok(
                super::AbstractPhysicalSpace::BuildingRoom(
                    Box::new(super::BuildingRoom::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CITY_FURNITURE, "CityFurniture") => {
            Ok(
                super::AbstractPhysicalSpace::CityFurniture(
                    Box::new(super::CityFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericOccupiedSpace") => {
            Ok(
                super::AbstractPhysicalSpace::GenericOccupiedSpace(
                    Box::new(
                        super::GenericOccupiedSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericUnoccupiedSpace") => {
            Ok(
                super::AbstractPhysicalSpace::GenericUnoccupiedSpace(
                    Box::new(
                        super::GenericUnoccupiedSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficSpace") => {
            Ok(
                super::AbstractPhysicalSpace::AuxiliaryTrafficSpace(
                    Box::new(
                        super::AuxiliaryTrafficSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "ClearanceSpace") => {
            Ok(
                super::AbstractPhysicalSpace::ClearanceSpace(
                    Box::new(super::ClearanceSpace::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Hole") => {
            Ok(
                super::AbstractPhysicalSpace::Hole(
                    Box::new(super::Hole::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Intersection") => {
            Ok(
                super::AbstractPhysicalSpace::Intersection(
                    Box::new(super::Intersection::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Railway") => {
            Ok(
                super::AbstractPhysicalSpace::Railway(
                    Box::new(super::Railway::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Road") => {
            Ok(
                super::AbstractPhysicalSpace::Road(
                    Box::new(super::Road::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Section") => {
            Ok(
                super::AbstractPhysicalSpace::Section(
                    Box::new(super::Section::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Square") => {
            Ok(
                super::AbstractPhysicalSpace::Square(
                    Box::new(super::Square::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Track") => {
            Ok(
                super::AbstractPhysicalSpace::Track(
                    Box::new(super::Track::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficSpace") => {
            Ok(
                super::AbstractPhysicalSpace::TrafficSpace(
                    Box::new(super::TrafficSpace::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Waterway") => {
            Ok(
                super::AbstractPhysicalSpace::Waterway(
                    Box::new(super::Waterway::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "HollowSpace") => {
            Ok(
                super::AbstractPhysicalSpace::HollowSpace(
                    Box::new(super::HollowSpace::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "Tunnel") => {
            Ok(
                super::AbstractPhysicalSpace::Tunnel(
                    Box::new(super::Tunnel::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelConstructiveElement") => {
            Ok(
                super::AbstractPhysicalSpace::TunnelConstructiveElement(
                    Box::new(
                        super::TunnelConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelFurniture") => {
            Ok(
                super::AbstractPhysicalSpace::TunnelFurniture(
                    Box::new(super::TunnelFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelInstallation") => {
            Ok(
                super::AbstractPhysicalSpace::TunnelInstallation(
                    Box::new(
                        super::TunnelInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelPart") => {
            Ok(
                super::AbstractPhysicalSpace::TunnelPart(
                    Box::new(super::TunnelPart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_VEGETATION, "PlantCover") => {
            Ok(
                super::AbstractPhysicalSpace::PlantCover(
                    Box::new(super::PlantCover::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_VEGETATION, "SolitaryVegetationObject") => {
            Ok(
                super::AbstractPhysicalSpace::SolitaryVegetationObject(
                    Box::new(
                        super::SolitaryVegetationObject::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterBody") => {
            Ok(
                super::AbstractPhysicalSpace::WaterBody(
                    Box::new(super::WaterBody::from_gml_with_info(reader, info)?),
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

pub fn parse_abstract_point_cloud(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractPointCloud, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_POINT_CLOUD, "PointCloud") => {
            Ok(
                super::AbstractPointCloud::PointCloud(
                    super::PointCloud::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_space(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractSpace, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "Door") => {
            Ok(
                super::AbstractSpace::Door(
                    Box::new(super::Door::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OtherConstruction") => {
            Ok(
                super::AbstractSpace::OtherConstruction(
                    Box::new(super::OtherConstruction::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "Window") => {
            Ok(
                super::AbstractSpace::Window(
                    Box::new(super::Window::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "Bridge") => {
            Ok(
                super::AbstractSpace::Bridge(
                    Box::new(super::Bridge::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeConstructiveElement") => {
            Ok(
                super::AbstractSpace::BridgeConstructiveElement(
                    Box::new(
                        super::BridgeConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeFurniture") => {
            Ok(
                super::AbstractSpace::BridgeFurniture(
                    Box::new(super::BridgeFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeInstallation") => {
            Ok(
                super::AbstractSpace::BridgeInstallation(
                    Box::new(
                        super::BridgeInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgePart") => {
            Ok(
                super::AbstractSpace::BridgePart(
                    Box::new(super::BridgePart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BRIDGE, "BridgeRoom") => {
            Ok(
                super::AbstractSpace::BridgeRoom(
                    Box::new(super::BridgeRoom::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "Building") => {
            Ok(
                super::AbstractSpace::Building(
                    Box::new(super::Building::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingConstructiveElement") => {
            Ok(
                super::AbstractSpace::BuildingConstructiveElement(
                    Box::new(
                        super::BuildingConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingFurniture") => {
            Ok(
                super::AbstractSpace::BuildingFurniture(
                    Box::new(super::BuildingFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingInstallation") => {
            Ok(
                super::AbstractSpace::BuildingInstallation(
                    Box::new(
                        super::BuildingInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingPart") => {
            Ok(
                super::AbstractSpace::BuildingPart(
                    Box::new(super::BuildingPart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingRoom") => {
            Ok(
                super::AbstractSpace::BuildingRoom(
                    Box::new(super::BuildingRoom::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingUnit") => {
            Ok(
                super::AbstractSpace::BuildingUnit(
                    Box::new(super::BuildingUnit::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "Storey") => {
            Ok(
                super::AbstractSpace::Storey(
                    Box::new(super::Storey::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CITY_FURNITURE, "CityFurniture") => {
            Ok(
                super::AbstractSpace::CityFurniture(
                    Box::new(super::CityFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CITY_OBJECT_GROUP, "CityObjectGroup") => {
            Ok(
                super::AbstractSpace::CityObjectGroup(
                    Box::new(super::CityObjectGroup::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericLogicalSpace") => {
            Ok(
                super::AbstractSpace::GenericLogicalSpace(
                    Box::new(
                        super::GenericLogicalSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericOccupiedSpace") => {
            Ok(
                super::AbstractSpace::GenericOccupiedSpace(
                    Box::new(
                        super::GenericOccupiedSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericUnoccupiedSpace") => {
            Ok(
                super::AbstractSpace::GenericUnoccupiedSpace(
                    Box::new(
                        super::GenericUnoccupiedSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficSpace") => {
            Ok(
                super::AbstractSpace::AuxiliaryTrafficSpace(
                    Box::new(
                        super::AuxiliaryTrafficSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "ClearanceSpace") => {
            Ok(
                super::AbstractSpace::ClearanceSpace(
                    Box::new(super::ClearanceSpace::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Hole") => {
            Ok(
                super::AbstractSpace::Hole(
                    Box::new(super::Hole::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Intersection") => {
            Ok(
                super::AbstractSpace::Intersection(
                    Box::new(super::Intersection::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Railway") => {
            Ok(
                super::AbstractSpace::Railway(
                    Box::new(super::Railway::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Road") => {
            Ok(
                super::AbstractSpace::Road(
                    Box::new(super::Road::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Section") => {
            Ok(
                super::AbstractSpace::Section(
                    Box::new(super::Section::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Square") => {
            Ok(
                super::AbstractSpace::Square(
                    Box::new(super::Square::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Track") => {
            Ok(
                super::AbstractSpace::Track(
                    Box::new(super::Track::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficSpace") => {
            Ok(
                super::AbstractSpace::TrafficSpace(
                    Box::new(super::TrafficSpace::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Waterway") => {
            Ok(
                super::AbstractSpace::Waterway(
                    Box::new(super::Waterway::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "HollowSpace") => {
            Ok(
                super::AbstractSpace::HollowSpace(
                    Box::new(super::HollowSpace::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "Tunnel") => {
            Ok(
                super::AbstractSpace::Tunnel(
                    Box::new(super::Tunnel::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelConstructiveElement") => {
            Ok(
                super::AbstractSpace::TunnelConstructiveElement(
                    Box::new(
                        super::TunnelConstructiveElement::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelFurniture") => {
            Ok(
                super::AbstractSpace::TunnelFurniture(
                    Box::new(super::TunnelFurniture::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelInstallation") => {
            Ok(
                super::AbstractSpace::TunnelInstallation(
                    Box::new(
                        super::TunnelInstallation::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelPart") => {
            Ok(
                super::AbstractSpace::TunnelPart(
                    Box::new(super::TunnelPart::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_VEGETATION, "PlantCover") => {
            Ok(
                super::AbstractSpace::PlantCover(
                    Box::new(super::PlantCover::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_VEGETATION, "SolitaryVegetationObject") => {
            Ok(
                super::AbstractSpace::SolitaryVegetationObject(
                    Box::new(
                        super::SolitaryVegetationObject::from_gml_with_info(
                            reader,
                            info,
                        )?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterBody") => {
            Ok(
                super::AbstractSpace::WaterBody(
                    Box::new(super::WaterBody::from_gml_with_info(reader, info)?),
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

pub fn parse_abstract_space_boundary(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractSpaceBoundary, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "CeilingSurface") => {
            Ok(
                super::AbstractSpaceBoundary::CeilingSurface(
                    Box::new(super::CeilingSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "DoorSurface") => {
            Ok(
                super::AbstractSpaceBoundary::DoorSurface(
                    Box::new(super::DoorSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "FloorSurface") => {
            Ok(
                super::AbstractSpaceBoundary::FloorSurface(
                    Box::new(super::FloorSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "GroundSurface") => {
            Ok(
                super::AbstractSpaceBoundary::GroundSurface(
                    Box::new(super::GroundSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "InteriorWallSurface") => {
            Ok(
                super::AbstractSpaceBoundary::InteriorWallSurface(
                    Box::new(
                        super::InteriorWallSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterCeilingSurface") => {
            Ok(
                super::AbstractSpaceBoundary::OuterCeilingSurface(
                    Box::new(
                        super::OuterCeilingSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterFloorSurface") => {
            Ok(
                super::AbstractSpaceBoundary::OuterFloorSurface(
                    Box::new(super::OuterFloorSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "RoofSurface") => {
            Ok(
                super::AbstractSpaceBoundary::RoofSurface(
                    Box::new(super::RoofSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "WallSurface") => {
            Ok(
                super::AbstractSpaceBoundary::WallSurface(
                    Box::new(super::WallSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "WindowSurface") => {
            Ok(
                super::AbstractSpaceBoundary::WindowSurface(
                    Box::new(super::WindowSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CORE, "ClosureSurface") => {
            Ok(
                super::AbstractSpaceBoundary::ClosureSurface(
                    Box::new(super::ClosureSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericThematicSurface") => {
            Ok(
                super::AbstractSpaceBoundary::GenericThematicSurface(
                    Box::new(
                        super::GenericThematicSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_LAND_USE, "LandUse") => {
            Ok(
                super::AbstractSpaceBoundary::LandUse(
                    Box::new(super::LandUse::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "BreaklineRelief") => {
            Ok(
                super::AbstractSpaceBoundary::BreaklineRelief(
                    Box::new(super::BreaklineRelief::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "MassPointRelief") => {
            Ok(
                super::AbstractSpaceBoundary::MassPointRelief(
                    Box::new(super::MassPointRelief::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "RasterRelief") => {
            Ok(
                super::AbstractSpaceBoundary::RasterRelief(
                    Box::new(super::RasterRelief::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "ReliefFeature") => {
            Ok(
                super::AbstractSpaceBoundary::ReliefFeature(
                    Box::new(super::ReliefFeature::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "TINRelief") => {
            Ok(
                super::AbstractSpaceBoundary::TINRelief(
                    Box::new(super::TINRelief::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficArea") => {
            Ok(
                super::AbstractSpaceBoundary::AuxiliaryTrafficArea(
                    Box::new(
                        super::AuxiliaryTrafficArea::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "HoleSurface") => {
            Ok(
                super::AbstractSpaceBoundary::HoleSurface(
                    Box::new(super::HoleSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Marking") => {
            Ok(
                super::AbstractSpaceBoundary::Marking(
                    Box::new(super::Marking::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficArea") => {
            Ok(
                super::AbstractSpaceBoundary::TrafficArea(
                    Box::new(super::TrafficArea::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterGroundSurface") => {
            Ok(
                super::AbstractSpaceBoundary::WaterGroundSurface(
                    Box::new(
                        super::WaterGroundSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterSurface") => {
            Ok(
                super::AbstractSpaceBoundary::WaterSurface(
                    Box::new(super::WaterSurface::from_gml_with_info(reader, info)?),
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

pub fn parse_abstract_thematic_surface(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractThematicSurface, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_CONSTRUCTION, "CeilingSurface") => {
            Ok(
                super::AbstractThematicSurface::CeilingSurface(
                    Box::new(super::CeilingSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "DoorSurface") => {
            Ok(
                super::AbstractThematicSurface::DoorSurface(
                    Box::new(super::DoorSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "FloorSurface") => {
            Ok(
                super::AbstractThematicSurface::FloorSurface(
                    Box::new(super::FloorSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "GroundSurface") => {
            Ok(
                super::AbstractThematicSurface::GroundSurface(
                    Box::new(super::GroundSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "InteriorWallSurface") => {
            Ok(
                super::AbstractThematicSurface::InteriorWallSurface(
                    Box::new(
                        super::InteriorWallSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterCeilingSurface") => {
            Ok(
                super::AbstractThematicSurface::OuterCeilingSurface(
                    Box::new(
                        super::OuterCeilingSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "OuterFloorSurface") => {
            Ok(
                super::AbstractThematicSurface::OuterFloorSurface(
                    Box::new(super::OuterFloorSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "RoofSurface") => {
            Ok(
                super::AbstractThematicSurface::RoofSurface(
                    Box::new(super::RoofSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "WallSurface") => {
            Ok(
                super::AbstractThematicSurface::WallSurface(
                    Box::new(super::WallSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CONSTRUCTION, "WindowSurface") => {
            Ok(
                super::AbstractThematicSurface::WindowSurface(
                    Box::new(super::WindowSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_CORE, "ClosureSurface") => {
            Ok(
                super::AbstractThematicSurface::ClosureSurface(
                    Box::new(super::ClosureSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericThematicSurface") => {
            Ok(
                super::AbstractThematicSurface::GenericThematicSurface(
                    Box::new(
                        super::GenericThematicSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_LAND_USE, "LandUse") => {
            Ok(
                super::AbstractThematicSurface::LandUse(
                    Box::new(super::LandUse::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficArea") => {
            Ok(
                super::AbstractThematicSurface::AuxiliaryTrafficArea(
                    Box::new(
                        super::AuxiliaryTrafficArea::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "HoleSurface") => {
            Ok(
                super::AbstractThematicSurface::HoleSurface(
                    Box::new(super::HoleSurface::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Marking") => {
            Ok(
                super::AbstractThematicSurface::Marking(
                    Box::new(super::Marking::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficArea") => {
            Ok(
                super::AbstractThematicSurface::TrafficArea(
                    Box::new(super::TrafficArea::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterGroundSurface") => {
            Ok(
                super::AbstractThematicSurface::WaterGroundSurface(
                    Box::new(
                        super::WaterGroundSurface::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterSurface") => {
            Ok(
                super::AbstractThematicSurface::WaterSurface(
                    Box::new(super::WaterSurface::from_gml_with_info(reader, info)?),
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

pub fn parse_abstract_unoccupied_space(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractUnoccupiedSpace, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_BRIDGE, "BridgeRoom") => {
            Ok(
                super::AbstractUnoccupiedSpace::BridgeRoom(
                    Box::new(super::BridgeRoom::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_BUILDING, "BuildingRoom") => {
            Ok(
                super::AbstractUnoccupiedSpace::BuildingRoom(
                    Box::new(super::BuildingRoom::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_GENERICS, "GenericUnoccupiedSpace") => {
            Ok(
                super::AbstractUnoccupiedSpace::GenericUnoccupiedSpace(
                    Box::new(
                        super::GenericUnoccupiedSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "AuxiliaryTrafficSpace") => {
            Ok(
                super::AbstractUnoccupiedSpace::AuxiliaryTrafficSpace(
                    Box::new(
                        super::AuxiliaryTrafficSpace::from_gml_with_info(reader, info)?,
                    ),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "ClearanceSpace") => {
            Ok(
                super::AbstractUnoccupiedSpace::ClearanceSpace(
                    Box::new(super::ClearanceSpace::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Hole") => {
            Ok(
                super::AbstractUnoccupiedSpace::Hole(
                    Box::new(super::Hole::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Intersection") => {
            Ok(
                super::AbstractUnoccupiedSpace::Intersection(
                    Box::new(super::Intersection::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Railway") => {
            Ok(
                super::AbstractUnoccupiedSpace::Railway(
                    Box::new(super::Railway::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Road") => {
            Ok(
                super::AbstractUnoccupiedSpace::Road(
                    Box::new(super::Road::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Section") => {
            Ok(
                super::AbstractUnoccupiedSpace::Section(
                    Box::new(super::Section::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Square") => {
            Ok(
                super::AbstractUnoccupiedSpace::Square(
                    Box::new(super::Square::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Track") => {
            Ok(
                super::AbstractUnoccupiedSpace::Track(
                    Box::new(super::Track::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "TrafficSpace") => {
            Ok(
                super::AbstractUnoccupiedSpace::TrafficSpace(
                    Box::new(super::TrafficSpace::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Waterway") => {
            Ok(
                super::AbstractUnoccupiedSpace::Waterway(
                    Box::new(super::Waterway::from_gml_with_info(reader, info)?),
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "HollowSpace") => {
            Ok(
                super::AbstractUnoccupiedSpace::HollowSpace(
                    Box::new(super::HollowSpace::from_gml_with_info(reader, info)?),
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

pub fn parse_abstract_version(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractVersion, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_VERSIONING, "Version") => {
            Ok(
                super::AbstractVersion::Version(
                    super::Version::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_version_transition(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractVersionTransition, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_VERSIONING, "VersionTransition") => {
            Ok(
                super::AbstractVersionTransition::VersionTransition(
                    super::VersionTransition::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_relief_component(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractReliefComponent, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_RELIEF, "BreaklineRelief") => {
            Ok(
                super::AbstractReliefComponent::BreaklineRelief(
                    super::BreaklineRelief::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "MassPointRelief") => {
            Ok(
                super::AbstractReliefComponent::MassPointRelief(
                    super::MassPointRelief::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "RasterRelief") => {
            Ok(
                super::AbstractReliefComponent::RasterRelief(
                    super::RasterRelief::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_RELIEF, "TINRelief") => {
            Ok(
                super::AbstractReliefComponent::TINRelief(
                    super::TINRelief::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_transportation_space(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractTransportationSpace, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_TRANSPORTATION, "Intersection") => {
            Ok(
                super::AbstractTransportationSpace::Intersection(
                    super::Intersection::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Railway") => {
            Ok(
                super::AbstractTransportationSpace::Railway(
                    super::Railway::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Road") => {
            Ok(
                super::AbstractTransportationSpace::Road(
                    super::Road::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Section") => {
            Ok(
                super::AbstractTransportationSpace::Section(
                    super::Section::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Square") => {
            Ok(
                super::AbstractTransportationSpace::Square(
                    super::Square::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Track") => {
            Ok(
                super::AbstractTransportationSpace::Track(
                    super::Track::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TRANSPORTATION, "Waterway") => {
            Ok(
                super::AbstractTransportationSpace::Waterway(
                    super::Waterway::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_tunnel(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractTunnel, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_TUNNEL, "Tunnel") => {
            Ok(
                super::AbstractTunnel::Tunnel(
                    super::Tunnel::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_TUNNEL, "TunnelPart") => {
            Ok(
                super::AbstractTunnel::TunnelPart(
                    super::TunnelPart::from_gml_with_info(reader, info)?,
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

pub fn parse_abstract_vegetation_object(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractVegetationObject, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_VEGETATION, "PlantCover") => {
            Ok(
                super::AbstractVegetationObject::PlantCover(
                    super::PlantCover::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_VEGETATION, "SolitaryVegetationObject") => {
            Ok(
                super::AbstractVegetationObject::SolitaryVegetationObject(
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

pub fn parse_abstract_water_boundary_surface(
    reader: &mut crate::gml_reader::SubtreeReader<'_>,
    info: &crate::gml_reader::ElementInfo,
) -> Result<super::AbstractWaterBoundarySurface, crate::error::ReaderError> {
    match (info.namespace.as_str(), info.local_name.as_str()) {
        (crate::namespace::NS_WATER_BODY, "WaterGroundSurface") => {
            Ok(
                super::AbstractWaterBoundarySurface::WaterGroundSurface(
                    super::WaterGroundSurface::from_gml_with_info(reader, info)?,
                ),
            )
        }
        (crate::namespace::NS_WATER_BODY, "WaterSurface") => {
            Ok(
                super::AbstractWaterBoundarySurface::WaterSurface(
                    super::WaterSurface::from_gml_with_info(reader, info)?,
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
