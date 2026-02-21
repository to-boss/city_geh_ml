#![allow(unused_imports, unused_mut, unused_variables)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct WaterBodyClassValue(pub String);
impl crate::from_gml::FromGml for WaterBodyClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(WaterBodyClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct WaterBodyFunctionValue(pub String);
impl crate::from_gml::FromGml for WaterBodyFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(WaterBodyFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct WaterBodyUsageValue(pub String);
impl crate::from_gml::FromGml for WaterBodyUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(WaterBodyUsageValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct WaterLevelValue(pub String);
impl crate::from_gml::FromGml for WaterLevelValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(WaterLevelValue(reader.read_text()?))
    }
}
pub trait AbstractWaterBoundarySurfaceTrait: AbstractThematicSurfaceTrait {}
#[derive(Debug, Clone)]
pub enum AbstractWaterBoundarySurface {
    WaterGroundSurface(WaterGroundSurface),
    WaterSurface(WaterSurface),
}
impl Default for AbstractWaterBoundarySurface {
    fn default() -> Self {
        Self::WaterGroundSurface(Default::default())
    }
}
impl AbstractFeatureTrait for AbstractWaterBoundarySurface {
    fn feature_id(&self) -> &ID {
        match self {
            Self::WaterGroundSurface(v) => v.feature_id(),
            Self::WaterSurface(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::WaterGroundSurface(v) => v.identifier(),
            Self::WaterSurface(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::WaterGroundSurface(v) => v.name(),
            Self::WaterSurface(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::WaterGroundSurface(v) => v.description(),
            Self::WaterSurface(v) => v.description(),
        }
    }
}
impl AbstractFeatureWithLifespanTrait for AbstractWaterBoundarySurface {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::WaterGroundSurface(v) => v.creation_date(),
            Self::WaterSurface(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::WaterGroundSurface(v) => v.termination_date(),
            Self::WaterSurface(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::WaterGroundSurface(v) => v.valid_from(),
            Self::WaterSurface(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::WaterGroundSurface(v) => v.valid_to(),
            Self::WaterSurface(v) => v.valid_to(),
        }
    }
}
impl AbstractCityObjectTrait for AbstractWaterBoundarySurface {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::WaterGroundSurface(v) => v.relative_to_terrain(),
            Self::WaterSurface(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::WaterGroundSurface(v) => v.relative_to_water(),
            Self::WaterSurface(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::WaterGroundSurface(v) => v.appearance(),
            Self::WaterSurface(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::WaterGroundSurface(v) => v.generalizes_to(),
            Self::WaterSurface(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::WaterGroundSurface(v) => v.external_reference(),
            Self::WaterSurface(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::WaterGroundSurface(v) => v.related_to(),
            Self::WaterSurface(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::WaterGroundSurface(v) => v.dynamizer(),
            Self::WaterSurface(v) => v.dynamizer(),
        }
    }
}
impl AbstractSpaceBoundaryTrait for AbstractWaterBoundarySurface {}
impl AbstractThematicSurfaceTrait for AbstractWaterBoundarySurface {
    fn area(&self) -> &[QualifiedArea] {
        match self {
            Self::WaterGroundSurface(v) => v.area(),
            Self::WaterSurface(v) => v.area(),
        }
    }
    fn lod3_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::WaterGroundSurface(v) => v.lod3_multi_surface(),
            Self::WaterSurface(v) => v.lod3_multi_surface(),
        }
    }
    fn lod2_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::WaterGroundSurface(v) => v.lod2_multi_surface(),
            Self::WaterSurface(v) => v.lod2_multi_surface(),
        }
    }
    fn point_cloud(&self) -> Option<&AbstractPointCloud> {
        match self {
            Self::WaterGroundSurface(v) => v.point_cloud(),
            Self::WaterSurface(v) => v.point_cloud(),
        }
    }
    fn lod0_multi_curve(&self) -> Option<&crate::geometry::MultiCurve> {
        match self {
            Self::WaterGroundSurface(v) => v.lod0_multi_curve(),
            Self::WaterSurface(v) => v.lod0_multi_curve(),
        }
    }
    fn lod0_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::WaterGroundSurface(v) => v.lod0_multi_surface(),
            Self::WaterSurface(v) => v.lod0_multi_surface(),
        }
    }
    fn lod1_multi_surface(&self) -> Option<&crate::geometry::MultiSurface> {
        match self {
            Self::WaterGroundSurface(v) => v.lod1_multi_surface(),
            Self::WaterSurface(v) => v.lod1_multi_surface(),
        }
    }
}
impl AbstractWaterBoundarySurfaceTrait for AbstractWaterBoundarySurface {}
impl From<WaterGroundSurface> for AbstractWaterBoundarySurface {
    fn from(v: WaterGroundSurface) -> Self {
        Self::WaterGroundSurface(v)
    }
}
impl From<WaterSurface> for AbstractWaterBoundarySurface {
    fn from(v: WaterSurface) -> Self {
        Self::WaterSurface(v)
    }
}
pub trait AbstractWaterBoundarySurfaceAccessors {
    fn water_ground_surfaces(&self) -> impl Iterator<Item = &WaterGroundSurface>;
    fn water_surfaces(&self) -> impl Iterator<Item = &WaterSurface>;
}
impl AbstractWaterBoundarySurfaceAccessors for [AbstractWaterBoundarySurface] {
    fn water_ground_surfaces(&self) -> impl Iterator<Item = &WaterGroundSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractWaterBoundarySurface::WaterGroundSurface(v) => Some(v),
                _ => None,
            })
    }
    fn water_surfaces(&self) -> impl Iterator<Item = &WaterSurface> {
        self.iter()
            .filter_map(|item| match item {
                AbstractWaterBoundarySurface::WaterSurface(v) => Some(v),
                _ => None,
            })
    }
}
#[derive(Debug, Clone, Default)]
pub struct WaterBody {
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
    pub class_: Option<WaterBodyClassValue>,
    pub function: Vec<WaterBodyFunctionValue>,
    pub usage: Vec<WaterBodyUsageValue>,
}
impl AbstractFeatureTrait for WaterBody {
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
impl AbstractFeatureWithLifespanTrait for WaterBody {
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
impl AbstractCityObjectTrait for WaterBody {
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
impl AbstractSpaceTrait for WaterBody {
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
impl AbstractPhysicalSpaceTrait for WaterBody {
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
impl AbstractOccupiedSpaceTrait for WaterBody {
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
impl WaterBody {
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
                (crate::namespace::NS_WATER_BODY, "class") => {
                    class_ = Some(WaterBodyClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_WATER_BODY, "function") => {
                    function.push(WaterBodyFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_WATER_BODY, "usage") => {
                    usage.push(WaterBodyUsageValue(sub.read_text()?));
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(WaterBody {
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
impl crate::from_gml::FromGml for WaterBody {
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
pub struct WaterGroundSurface {
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
impl AbstractFeatureTrait for WaterGroundSurface {
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
impl AbstractFeatureWithLifespanTrait for WaterGroundSurface {
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
impl AbstractCityObjectTrait for WaterGroundSurface {
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
impl AbstractSpaceBoundaryTrait for WaterGroundSurface {}
impl AbstractThematicSurfaceTrait for WaterGroundSurface {
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
impl AbstractWaterBoundarySurfaceTrait for WaterGroundSurface {}
impl WaterGroundSurface {
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
        Ok(WaterGroundSurface {
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
impl crate::from_gml::FromGml for WaterGroundSurface {
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
pub struct WaterSurface {
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
    pub water_level: Option<WaterLevelValue>,
}
impl AbstractFeatureTrait for WaterSurface {
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
impl AbstractFeatureWithLifespanTrait for WaterSurface {
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
impl AbstractCityObjectTrait for WaterSurface {
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
impl AbstractSpaceBoundaryTrait for WaterSurface {}
impl AbstractThematicSurfaceTrait for WaterSurface {
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
impl AbstractWaterBoundarySurfaceTrait for WaterSurface {}
impl WaterSurface {
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
        let mut water_level = None;
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
                (crate::namespace::NS_WATER_BODY, "waterLevel") => {
                    water_level = Some(WaterLevelValue(sub.read_text()?));
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(WaterSurface {
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
            water_level,
        })
    }
}
impl crate::from_gml::FromGml for WaterSurface {
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
