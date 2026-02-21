#![allow(unused_imports, unused_mut, unused_variables)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct HollowSpaceClassValue(pub String);
impl crate::from_gml::FromGml for HollowSpaceClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(HollowSpaceClassValue(reader.read_text()?))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct HollowSpaceFunctionValue(pub String);
impl crate::from_gml::FromGml for HollowSpaceFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(HollowSpaceFunctionValue(reader.read_text()?))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct HollowSpaceUsageValue(pub String);
impl crate::from_gml::FromGml for HollowSpaceUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(HollowSpaceUsageValue(reader.read_text()?))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TunnelClassValue(pub String);
impl crate::from_gml::FromGml for TunnelClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TunnelClassValue(reader.read_text()?))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TunnelConstructiveElementClassValue(pub String);
impl crate::from_gml::FromGml for TunnelConstructiveElementClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TunnelConstructiveElementClassValue(reader.read_text()?))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TunnelConstructiveElementFunctionValue(pub String);
impl crate::from_gml::FromGml for TunnelConstructiveElementFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TunnelConstructiveElementFunctionValue(reader.read_text()?))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TunnelConstructiveElementUsageValue(pub String);
impl crate::from_gml::FromGml for TunnelConstructiveElementUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TunnelConstructiveElementUsageValue(reader.read_text()?))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TunnelFunctionValue(pub String);
impl crate::from_gml::FromGml for TunnelFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TunnelFunctionValue(reader.read_text()?))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TunnelFurnitureClassValue(pub String);
impl crate::from_gml::FromGml for TunnelFurnitureClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TunnelFurnitureClassValue(reader.read_text()?))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TunnelFurnitureFunctionValue(pub String);
impl crate::from_gml::FromGml for TunnelFurnitureFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TunnelFurnitureFunctionValue(reader.read_text()?))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TunnelFurnitureUsageValue(pub String);
impl crate::from_gml::FromGml for TunnelFurnitureUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TunnelFurnitureUsageValue(reader.read_text()?))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TunnelInstallationClassValue(pub String);
impl crate::from_gml::FromGml for TunnelInstallationClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TunnelInstallationClassValue(reader.read_text()?))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TunnelInstallationFunctionValue(pub String);
impl crate::from_gml::FromGml for TunnelInstallationFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TunnelInstallationFunctionValue(reader.read_text()?))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TunnelInstallationUsageValue(pub String);
impl crate::from_gml::FromGml for TunnelInstallationUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TunnelInstallationUsageValue(reader.read_text()?))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TunnelUsageValue(pub String);
impl crate::from_gml::FromGml for TunnelUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TunnelUsageValue(reader.read_text()?))
    }
}

#[derive(Debug, Clone, Default)]
pub struct HollowSpace {
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
    pub class_: Option<HollowSpaceClassValue>,
    pub function: Vec<HollowSpaceFunctionValue>,
    pub usage: Vec<HollowSpaceUsageValue>,
    pub tunnel_installation: Vec<TunnelInstallation>,
    pub tunnel_furniture: Vec<TunnelFurniture>,
}

impl AbstractFeatureTrait for HollowSpace {
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

impl AbstractFeatureWithLifespanTrait for HollowSpace {
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

impl AbstractCityObjectTrait for HollowSpace {
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

impl AbstractSpaceTrait for HollowSpace {
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

impl AbstractPhysicalSpaceTrait for HollowSpace {
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

impl AbstractUnoccupiedSpaceTrait for HollowSpace {}
impl HollowSpace {
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
        let mut class_ = None;
        let mut function = Vec::new();
        let mut usage = Vec::new();
        let mut tunnel_installation = Vec::new();
        let mut tunnel_furniture = Vec::new();
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
                (crate::namespace::NS_TUNNEL, "class") => {
                    class_ = Some(HollowSpaceClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TUNNEL, "function") => {
                    function.push(HollowSpaceFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_TUNNEL, "usage") => {
                    usage.push(HollowSpaceUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_TUNNEL, "tunnelInstallation") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        tunnel_installation
                            .push(
                                TunnelInstallation::from_gml_with_info(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_TUNNEL, "tunnelFurniture") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        tunnel_furniture
                            .push(
                                TunnelFurniture::from_gml_with_info(
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
        Ok(HollowSpace {
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
            class_,
            function,
            usage,
            tunnel_installation,
            tunnel_furniture,
        })
    }
}

impl crate::from_gml::FromGml for HollowSpace {
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

pub trait AbstractTunnelTrait: AbstractConstructionTrait {
    fn class_(&self) -> Option<&TunnelClassValue>;
    fn function(&self) -> &[TunnelFunctionValue];
    fn usage(&self) -> &[TunnelUsageValue];
    fn tunnel_constructive_element(&self) -> &[TunnelConstructiveElement];
    fn tunnel_furniture(&self) -> &[TunnelFurniture];
    fn tunnel_installation(&self) -> &[TunnelInstallation];
    fn hollow_space(&self) -> &[HollowSpace];
}

#[derive(Debug, Clone)]
pub enum AbstractTunnel {
    Tunnel(Tunnel),
    TunnelPart(TunnelPart),
}

impl Default for AbstractTunnel {
    fn default() -> Self {
        Self::Tunnel(Default::default())
    }
}

impl AbstractFeatureTrait for AbstractTunnel {
    fn feature_id(&self) -> &ID {
        match self {
            Self::Tunnel(v) => v.feature_id(),
            Self::TunnelPart(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::Tunnel(v) => v.identifier(),
            Self::TunnelPart(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::Tunnel(v) => v.name(),
            Self::TunnelPart(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::Tunnel(v) => v.description(),
            Self::TunnelPart(v) => v.description(),
        }
    }
}

impl AbstractFeatureWithLifespanTrait for AbstractTunnel {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::Tunnel(v) => v.creation_date(),
            Self::TunnelPart(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::Tunnel(v) => v.termination_date(),
            Self::TunnelPart(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::Tunnel(v) => v.valid_from(),
            Self::TunnelPart(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::Tunnel(v) => v.valid_to(),
            Self::TunnelPart(v) => v.valid_to(),
        }
    }
}

impl AbstractCityObjectTrait for AbstractTunnel {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::Tunnel(v) => v.relative_to_terrain(),
            Self::TunnelPart(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::Tunnel(v) => v.relative_to_water(),
            Self::TunnelPart(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::Tunnel(v) => v.appearance(),
            Self::TunnelPart(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::Tunnel(v) => v.generalizes_to(),
            Self::TunnelPart(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::Tunnel(v) => v.external_reference(),
            Self::TunnelPart(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::Tunnel(v) => v.related_to(),
            Self::TunnelPart(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::Tunnel(v) => v.dynamizer(),
            Self::TunnelPart(v) => v.dynamizer(),
        }
    }
}

impl AbstractSpaceTrait for AbstractTunnel {
    fn space_type(&self) -> Option<SpaceType> {
        match self {
            Self::Tunnel(v) => v.space_type(),
            Self::TunnelPart(v) => v.space_type(),
        }
    }
    fn volume(&self) -> &[QualifiedVolume] {
        match self {
            Self::Tunnel(v) => v.volume(),
            Self::TunnelPart(v) => v.volume(),
        }
    }
    fn area(&self) -> &[QualifiedArea] {
        match self {
            Self::Tunnel(v) => v.area(),
            Self::TunnelPart(v) => v.area(),
        }
    }
    fn lod2_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Tunnel(v) => v.lod2_multi_curve(),
            Self::TunnelPart(v) => v.lod2_multi_curve(),
        }
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::Tunnel(v) => v.lod3_multi_surface(),
            Self::TunnelPart(v) => v.lod3_multi_surface(),
        }
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::Tunnel(v) => v.lod0_multi_surface(),
            Self::TunnelPart(v) => v.lod0_multi_surface(),
        }
    }
    fn lod1_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::Tunnel(v) => v.lod1_solid(),
            Self::TunnelPart(v) => v.lod1_solid(),
        }
    }
    fn lod3_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::Tunnel(v) => v.lod3_solid(),
            Self::TunnelPart(v) => v.lod3_solid(),
        }
    }
    fn boundary(&self) -> &[AbstractSpaceBoundary] {
        match self {
            Self::Tunnel(v) => v.boundary(),
            Self::TunnelPart(v) => v.boundary(),
        }
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Tunnel(v) => v.lod0_multi_curve(),
            Self::TunnelPart(v) => v.lod0_multi_curve(),
        }
    }
    fn lod2_solid(&self) -> Option<&crate::geometry::Solid> {
        match self {
            Self::Tunnel(v) => v.lod2_solid(),
            Self::TunnelPart(v) => v.lod2_solid(),
        }
    }
    fn lod0_point(&self) -> Option<&crate::geometry::DirectPosition> {
        match self {
            Self::Tunnel(v) => v.lod0_point(),
            Self::TunnelPart(v) => v.lod0_point(),
        }
    }
    fn lod3_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Tunnel(v) => v.lod3_multi_curve(),
            Self::TunnelPart(v) => v.lod3_multi_curve(),
        }
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::Tunnel(v) => v.lod2_multi_surface(),
            Self::TunnelPart(v) => v.lod2_multi_surface(),
        }
    }
}

impl AbstractPhysicalSpaceTrait for AbstractTunnel {
    fn lod3_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Tunnel(v) => v.lod3_terrain_intersection_curve(),
            Self::TunnelPart(v) => v.lod3_terrain_intersection_curve(),
        }
    }
    fn point_cloud(&self) -> Option<&AbstractPointCloud> {
        match self {
            Self::Tunnel(v) => v.point_cloud(),
            Self::TunnelPart(v) => v.point_cloud(),
        }
    }
    fn lod1_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Tunnel(v) => v.lod1_terrain_intersection_curve(),
            Self::TunnelPart(v) => v.lod1_terrain_intersection_curve(),
        }
    }
    fn lod2_terrain_intersection_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::Tunnel(v) => v.lod2_terrain_intersection_curve(),
            Self::TunnelPart(v) => v.lod2_terrain_intersection_curve(),
        }
    }
}

impl AbstractOccupiedSpaceTrait for AbstractTunnel {
    fn lod3_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::Tunnel(v) => v.lod3_implicit_representation(),
            Self::TunnelPart(v) => v.lod3_implicit_representation(),
        }
    }
    fn lod2_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::Tunnel(v) => v.lod2_implicit_representation(),
            Self::TunnelPart(v) => v.lod2_implicit_representation(),
        }
    }
    fn lod1_implicit_representation(&self) -> Option<&ImplicitGeometry> {
        match self {
            Self::Tunnel(v) => v.lod1_implicit_representation(),
            Self::TunnelPart(v) => v.lod1_implicit_representation(),
        }
    }
}

impl AbstractConstructionTrait for AbstractTunnel {
    fn condition_of_construction(&self) -> Option<ConditionOfConstructionValue> {
        match self {
            Self::Tunnel(v) => v.condition_of_construction(),
            Self::TunnelPart(v) => v.condition_of_construction(),
        }
    }
    fn date_of_construction(&self) -> Option<&String> {
        match self {
            Self::Tunnel(v) => v.date_of_construction(),
            Self::TunnelPart(v) => v.date_of_construction(),
        }
    }
    fn date_of_demolition(&self) -> Option<&String> {
        match self {
            Self::Tunnel(v) => v.date_of_demolition(),
            Self::TunnelPart(v) => v.date_of_demolition(),
        }
    }
    fn construction_event(&self) -> &[ConstructionEvent] {
        match self {
            Self::Tunnel(v) => v.construction_event(),
            Self::TunnelPart(v) => v.construction_event(),
        }
    }
    fn elevation(&self) -> &[Elevation] {
        match self {
            Self::Tunnel(v) => v.elevation(),
            Self::TunnelPart(v) => v.elevation(),
        }
    }
    fn height(&self) -> &[Height] {
        match self {
            Self::Tunnel(v) => v.height(),
            Self::TunnelPart(v) => v.height(),
        }
    }
    fn occupancy(&self) -> &[Occupancy] {
        match self {
            Self::Tunnel(v) => v.occupancy(),
            Self::TunnelPart(v) => v.occupancy(),
        }
    }
}

impl AbstractTunnelTrait for AbstractTunnel {
    fn class_(&self) -> Option<&TunnelClassValue> {
        match self {
            Self::Tunnel(v) => v.class_(),
            Self::TunnelPart(v) => v.class_(),
        }
    }
    fn function(&self) -> &[TunnelFunctionValue] {
        match self {
            Self::Tunnel(v) => v.function(),
            Self::TunnelPart(v) => v.function(),
        }
    }
    fn usage(&self) -> &[TunnelUsageValue] {
        match self {
            Self::Tunnel(v) => v.usage(),
            Self::TunnelPart(v) => v.usage(),
        }
    }
    fn tunnel_constructive_element(&self) -> &[TunnelConstructiveElement] {
        match self {
            Self::Tunnel(v) => v.tunnel_constructive_element(),
            Self::TunnelPart(v) => v.tunnel_constructive_element(),
        }
    }
    fn tunnel_furniture(&self) -> &[TunnelFurniture] {
        match self {
            Self::Tunnel(v) => v.tunnel_furniture(),
            Self::TunnelPart(v) => v.tunnel_furniture(),
        }
    }
    fn tunnel_installation(&self) -> &[TunnelInstallation] {
        match self {
            Self::Tunnel(v) => v.tunnel_installation(),
            Self::TunnelPart(v) => v.tunnel_installation(),
        }
    }
    fn hollow_space(&self) -> &[HollowSpace] {
        match self {
            Self::Tunnel(v) => v.hollow_space(),
            Self::TunnelPart(v) => v.hollow_space(),
        }
    }
}

impl From<Tunnel> for AbstractTunnel {
    fn from(v: Tunnel) -> Self {
        Self::Tunnel(v)
    }
}

impl From<TunnelPart> for AbstractTunnel {
    fn from(v: TunnelPart) -> Self {
        Self::TunnelPart(v)
    }
}

pub trait AbstractTunnelAccessors {
    fn tunnels(&self) -> impl Iterator<Item = &Tunnel>;
    fn tunnel_parts(&self) -> impl Iterator<Item = &TunnelPart>;
}

impl AbstractTunnelAccessors for [AbstractTunnel] {
    fn tunnels(&self) -> impl Iterator<Item = &Tunnel> {
        self.iter()
            .filter_map(|item| match item {
                AbstractTunnel::Tunnel(v) => Some(v),
                _ => None,
            })
    }
    fn tunnel_parts(&self) -> impl Iterator<Item = &TunnelPart> {
        self.iter()
            .filter_map(|item| match item {
                AbstractTunnel::TunnelPart(v) => Some(v),
                _ => None,
            })
    }
}

#[derive(Debug, Clone, Default)]
pub struct TunnelConstructiveElement {
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
    pub is_structural_element: Option<bool>,
    pub filling: Vec<AbstractFillingElement>,
    pub class_: Option<TunnelConstructiveElementClassValue>,
    pub function: Vec<TunnelConstructiveElementFunctionValue>,
    pub usage: Vec<TunnelConstructiveElementUsageValue>,
}

impl AbstractFeatureTrait for TunnelConstructiveElement {
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

impl AbstractFeatureWithLifespanTrait for TunnelConstructiveElement {
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

impl AbstractCityObjectTrait for TunnelConstructiveElement {
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

impl AbstractSpaceTrait for TunnelConstructiveElement {
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

impl AbstractPhysicalSpaceTrait for TunnelConstructiveElement {
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

impl AbstractOccupiedSpaceTrait for TunnelConstructiveElement {
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

impl AbstractConstructiveElementTrait for TunnelConstructiveElement {
    fn is_structural_element(&self) -> Option<bool> {
        self.is_structural_element
    }
    fn filling(&self) -> &[AbstractFillingElement] {
        &self.filling
    }
}

impl TunnelConstructiveElement {
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
        let mut is_structural_element = None;
        let mut filling = Vec::new();
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
                (crate::namespace::NS_CONSTRUCTION, "isStructuralElement") => {
                    is_structural_element = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_CONSTRUCTION, "filling") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        filling
                            .push(
                                super::dispatchers::parse_abstract_filling_element(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_TUNNEL, "class") => {
                    class_ = Some(TunnelConstructiveElementClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TUNNEL, "function") => {
                    function
                        .push(TunnelConstructiveElementFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_TUNNEL, "usage") => {
                    usage.push(TunnelConstructiveElementUsageValue(sub.read_text()?));
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(TunnelConstructiveElement {
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
            is_structural_element,
            filling,
            class_,
            function,
            usage,
        })
    }
}

impl crate::from_gml::FromGml for TunnelConstructiveElement {
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
pub struct TunnelFurniture {
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
    pub class_: Option<TunnelFurnitureClassValue>,
    pub function: Vec<TunnelFurnitureFunctionValue>,
    pub usage: Vec<TunnelFurnitureUsageValue>,
}

impl AbstractFeatureTrait for TunnelFurniture {
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

impl AbstractFeatureWithLifespanTrait for TunnelFurniture {
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

impl AbstractCityObjectTrait for TunnelFurniture {
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

impl AbstractSpaceTrait for TunnelFurniture {
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

impl AbstractPhysicalSpaceTrait for TunnelFurniture {
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

impl AbstractOccupiedSpaceTrait for TunnelFurniture {
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

impl AbstractFurnitureTrait for TunnelFurniture {}
impl TunnelFurniture {
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
                (crate::namespace::NS_TUNNEL, "class") => {
                    class_ = Some(TunnelFurnitureClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TUNNEL, "function") => {
                    function.push(TunnelFurnitureFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_TUNNEL, "usage") => {
                    usage.push(TunnelFurnitureUsageValue(sub.read_text()?));
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(TunnelFurniture {
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

impl crate::from_gml::FromGml for TunnelFurniture {
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
pub struct TunnelInstallation {
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
    pub relation_to_construction: Option<RelationToConstruction>,
    pub class_: Option<TunnelInstallationClassValue>,
    pub function: Vec<TunnelInstallationFunctionValue>,
    pub usage: Vec<TunnelInstallationUsageValue>,
}

impl AbstractFeatureTrait for TunnelInstallation {
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

impl AbstractFeatureWithLifespanTrait for TunnelInstallation {
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

impl AbstractCityObjectTrait for TunnelInstallation {
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

impl AbstractSpaceTrait for TunnelInstallation {
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

impl AbstractPhysicalSpaceTrait for TunnelInstallation {
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

impl AbstractOccupiedSpaceTrait for TunnelInstallation {
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

impl AbstractInstallationTrait for TunnelInstallation {
    fn relation_to_construction(&self) -> Option<RelationToConstruction> {
        self.relation_to_construction
    }
}

impl TunnelInstallation {
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
        let mut relation_to_construction = None;
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
                (crate::namespace::NS_CONSTRUCTION, "relationToConstruction") => {
                    relation_to_construction = Some(
                        RelationToConstruction::from_gml_text(&sub.read_text()?)?,
                    );
                }
                (crate::namespace::NS_TUNNEL, "class") => {
                    class_ = Some(TunnelInstallationClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TUNNEL, "function") => {
                    function.push(TunnelInstallationFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_TUNNEL, "usage") => {
                    usage.push(TunnelInstallationUsageValue(sub.read_text()?));
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(TunnelInstallation {
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
            relation_to_construction,
            class_,
            function,
            usage,
        })
    }
}

impl crate::from_gml::FromGml for TunnelInstallation {
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
pub struct Tunnel {
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
    pub class_: Option<TunnelClassValue>,
    pub function: Vec<TunnelFunctionValue>,
    pub usage: Vec<TunnelUsageValue>,
    pub tunnel_constructive_element: Vec<TunnelConstructiveElement>,
    pub tunnel_furniture: Vec<TunnelFurniture>,
    pub tunnel_installation: Vec<TunnelInstallation>,
    pub hollow_space: Vec<HollowSpace>,
    pub tunnel_part: Vec<TunnelPart>,
}

impl AbstractFeatureTrait for Tunnel {
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

impl AbstractFeatureWithLifespanTrait for Tunnel {
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

impl AbstractCityObjectTrait for Tunnel {
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

impl AbstractSpaceTrait for Tunnel {
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

impl AbstractPhysicalSpaceTrait for Tunnel {
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

impl AbstractOccupiedSpaceTrait for Tunnel {
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

impl AbstractConstructionTrait for Tunnel {
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

impl AbstractTunnelTrait for Tunnel {
    fn class_(&self) -> Option<&TunnelClassValue> {
        self.class_.as_ref()
    }
    fn function(&self) -> &[TunnelFunctionValue] {
        &self.function
    }
    fn usage(&self) -> &[TunnelUsageValue] {
        &self.usage
    }
    fn tunnel_constructive_element(&self) -> &[TunnelConstructiveElement] {
        &self.tunnel_constructive_element
    }
    fn tunnel_furniture(&self) -> &[TunnelFurniture] {
        &self.tunnel_furniture
    }
    fn tunnel_installation(&self) -> &[TunnelInstallation] {
        &self.tunnel_installation
    }
    fn hollow_space(&self) -> &[HollowSpace] {
        &self.hollow_space
    }
}

impl Tunnel {
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
        let mut tunnel_constructive_element = Vec::new();
        let mut tunnel_furniture = Vec::new();
        let mut tunnel_installation = Vec::new();
        let mut hollow_space = Vec::new();
        let mut tunnel_part = Vec::new();
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
                (crate::namespace::NS_TUNNEL, "class") => {
                    class_ = Some(TunnelClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TUNNEL, "function") => {
                    function.push(TunnelFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_TUNNEL, "usage") => {
                    usage.push(TunnelUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_TUNNEL, "tunnelConstructiveElement") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        tunnel_constructive_element
                            .push(
                                TunnelConstructiveElement::from_gml_with_info(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_TUNNEL, "tunnelFurniture") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        tunnel_furniture
                            .push(
                                TunnelFurniture::from_gml_with_info(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_TUNNEL, "tunnelInstallation") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        tunnel_installation
                            .push(
                                TunnelInstallation::from_gml_with_info(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_TUNNEL, "hollowSpace") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        hollow_space
                            .push(
                                HollowSpace::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_TUNNEL, "tunnelPart") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        tunnel_part
                            .push(
                                TunnelPart::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Tunnel {
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
            tunnel_constructive_element,
            tunnel_furniture,
            tunnel_installation,
            hollow_space,
            tunnel_part,
        })
    }
}

impl crate::from_gml::FromGml for Tunnel {
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
pub struct TunnelPart {
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
    pub class_: Option<TunnelClassValue>,
    pub function: Vec<TunnelFunctionValue>,
    pub usage: Vec<TunnelUsageValue>,
    pub tunnel_constructive_element: Vec<TunnelConstructiveElement>,
    pub tunnel_furniture: Vec<TunnelFurniture>,
    pub tunnel_installation: Vec<TunnelInstallation>,
    pub hollow_space: Vec<HollowSpace>,
}

impl AbstractFeatureTrait for TunnelPart {
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

impl AbstractFeatureWithLifespanTrait for TunnelPart {
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

impl AbstractCityObjectTrait for TunnelPart {
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

impl AbstractSpaceTrait for TunnelPart {
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

impl AbstractPhysicalSpaceTrait for TunnelPart {
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

impl AbstractOccupiedSpaceTrait for TunnelPart {
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

impl AbstractConstructionTrait for TunnelPart {
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

impl AbstractTunnelTrait for TunnelPart {
    fn class_(&self) -> Option<&TunnelClassValue> {
        self.class_.as_ref()
    }
    fn function(&self) -> &[TunnelFunctionValue] {
        &self.function
    }
    fn usage(&self) -> &[TunnelUsageValue] {
        &self.usage
    }
    fn tunnel_constructive_element(&self) -> &[TunnelConstructiveElement] {
        &self.tunnel_constructive_element
    }
    fn tunnel_furniture(&self) -> &[TunnelFurniture] {
        &self.tunnel_furniture
    }
    fn tunnel_installation(&self) -> &[TunnelInstallation] {
        &self.tunnel_installation
    }
    fn hollow_space(&self) -> &[HollowSpace] {
        &self.hollow_space
    }
}

impl TunnelPart {
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
        let mut tunnel_constructive_element = Vec::new();
        let mut tunnel_furniture = Vec::new();
        let mut tunnel_installation = Vec::new();
        let mut hollow_space = Vec::new();
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
                (crate::namespace::NS_TUNNEL, "class") => {
                    class_ = Some(TunnelClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_TUNNEL, "function") => {
                    function.push(TunnelFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_TUNNEL, "usage") => {
                    usage.push(TunnelUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_TUNNEL, "tunnelConstructiveElement") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        tunnel_constructive_element
                            .push(
                                TunnelConstructiveElement::from_gml_with_info(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_TUNNEL, "tunnelFurniture") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        tunnel_furniture
                            .push(
                                TunnelFurniture::from_gml_with_info(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_TUNNEL, "tunnelInstallation") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        tunnel_installation
                            .push(
                                TunnelInstallation::from_gml_with_info(
                                    &mut wrapper,
                                    &child_info,
                                )?,
                            );
                    }
                }
                (crate::namespace::NS_TUNNEL, "hollowSpace") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        hollow_space
                            .push(
                                HollowSpace::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(TunnelPart {
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
            tunnel_constructive_element,
            tunnel_furniture,
            tunnel_installation,
            hollow_space,
        })
    }
}

impl crate::from_gml::FromGml for TunnelPart {
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
