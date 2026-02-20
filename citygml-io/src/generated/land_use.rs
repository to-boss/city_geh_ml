#![allow(unused_imports, unused_mut, unused_variables)]
use super::*;

pub trait ADEOfLandUse: std::fmt::Debug {}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct LandUseClassValue(pub String);
impl crate::from_gml::FromGml for LandUseClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(LandUseClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct LandUseFunctionValue(pub String);
impl crate::from_gml::FromGml for LandUseFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(LandUseFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct LandUseUsageValue(pub String);
impl crate::from_gml::FromGml for LandUseUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(LandUseUsageValue(reader.read_text()?))
    }
}
#[derive(Debug, Default)]
pub struct LandUse {
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
    pub class_: Option<LandUseClassValue>,
    pub function: Vec<LandUseFunctionValue>,
    pub usage: Vec<LandUseUsageValue>,
    pub ade_of_land_use: Vec<Box<dyn ADEOfLandUse>>,
}
impl AbstractFeature for LandUse {
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
impl AbstractFeatureWithLifespan for LandUse {
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
impl AbstractCityObject for LandUse {
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
impl AbstractSpaceBoundary for LandUse {
    fn ade_of_abstract_space_boundary(&self) -> &[Box<dyn ADEOfAbstractSpaceBoundary>] {
        &self.ade_of_abstract_space_boundary
    }
}
impl AbstractThematicSurface for LandUse {
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
impl LandUse {
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
        let mut ade_of_land_use = Vec::new();
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
                (crate::namespace::NS_LAND_USE, "class") => {
                    class_ = Some(LandUseClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_LAND_USE, "function") => {
                    function.push(LandUseFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_LAND_USE, "usage") => {
                    usage.push(LandUseUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_LAND_USE, "adeOfLandUse") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(LandUse {
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
            ade_of_land_use,
        })
    }
}
impl crate::from_gml::FromGml for LandUse {
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
