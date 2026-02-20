#![allow(unused_imports, unused_mut, unused_variables)]
use super::*;

pub trait ADEOfAbstractReliefComponent: std::fmt::Debug {}
pub trait ADEOfBreaklineRelief: std::fmt::Debug {}
pub trait ADEOfMassPointRelief: std::fmt::Debug {}
pub trait ADEOfRasterRelief: std::fmt::Debug {}
pub trait ADEOfReliefFeature: std::fmt::Debug {}
pub trait ADEOfTINRelief: std::fmt::Debug {}
pub trait AbstractReliefComponent: AbstractSpaceBoundary {
    fn lod(&self) -> &IntegerBetween0and3;
    fn ade_of_abstract_relief_component(
        &self,
    ) -> &[Box<dyn ADEOfAbstractReliefComponent>];
    fn extent(&self) -> Option<&crate::geometry::Polygon>;
}
#[derive(Debug, Default)]
pub struct ReliefFeature {
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
    pub lod: IntegerBetween0and3,
    pub ade_of_relief_feature: Vec<Box<dyn ADEOfReliefFeature>>,
    pub relief_component: Vec<Box<dyn AbstractReliefComponent>>,
}
impl AbstractFeature for ReliefFeature {
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
impl AbstractFeatureWithLifespan for ReliefFeature {
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
impl AbstractCityObject for ReliefFeature {
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
impl AbstractSpaceBoundary for ReliefFeature {
    fn ade_of_abstract_space_boundary(&self) -> &[Box<dyn ADEOfAbstractSpaceBoundary>] {
        &self.ade_of_abstract_space_boundary
    }
}
impl ReliefFeature {
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
        let mut lod = Default::default();
        let mut ade_of_relief_feature = Vec::new();
        let mut relief_component = Vec::new();
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
                (crate::namespace::NS_RELIEF, "lod") => {
                    lod = IntegerBetween0and3(sub.read_text()?);
                }
                (crate::namespace::NS_RELIEF, "adeOfReliefFeature") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_RELIEF, "reliefComponent") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        relief_component
                            .push(
                                super::dispatchers::parse_dyn_abstract_relief_component(
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
        Ok(ReliefFeature {
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
            lod,
            ade_of_relief_feature,
            relief_component,
        })
    }
}
impl crate::from_gml::FromGml for ReliefFeature {
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
pub struct BreaklineRelief {
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
    pub lod: IntegerBetween0and3,
    pub ade_of_abstract_relief_component: Vec<Box<dyn ADEOfAbstractReliefComponent>>,
    pub extent: Option<crate::geometry::Polygon>,
    pub ade_of_breakline_relief: Vec<Box<dyn ADEOfBreaklineRelief>>,
    pub ridge_or_valley_lines: Option<crate::geometry::MultiCurve>,
    pub breaklines: Option<crate::geometry::MultiCurve>,
}
impl AbstractFeature for BreaklineRelief {
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
impl AbstractFeatureWithLifespan for BreaklineRelief {
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
impl AbstractCityObject for BreaklineRelief {
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
impl AbstractSpaceBoundary for BreaklineRelief {
    fn ade_of_abstract_space_boundary(&self) -> &[Box<dyn ADEOfAbstractSpaceBoundary>] {
        &self.ade_of_abstract_space_boundary
    }
}
impl AbstractReliefComponent for BreaklineRelief {
    fn lod(&self) -> &IntegerBetween0and3 {
        &self.lod
    }
    fn ade_of_abstract_relief_component(
        &self,
    ) -> &[Box<dyn ADEOfAbstractReliefComponent>] {
        &self.ade_of_abstract_relief_component
    }
    fn extent(&self) -> Option<&crate::geometry::Polygon> {
        self.extent.as_ref()
    }
}
impl BreaklineRelief {
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
        let mut lod = Default::default();
        let mut ade_of_abstract_relief_component = Vec::new();
        let mut extent = None;
        let mut ade_of_breakline_relief = Vec::new();
        let mut ridge_or_valley_lines = None;
        let mut breaklines = None;
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
                (crate::namespace::NS_RELIEF, "lod") => {
                    lod = IntegerBetween0and3(sub.read_text()?);
                }
                (crate::namespace::NS_RELIEF, "adeOfAbstractReliefComponent") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_RELIEF, "extent") => {
                    extent = Some({
                        let mut geom_sub = sub.subtree();
                        if let Some(geom_info) = geom_sub.next_element()? {
                            if geom_info.local_name == "Polygon" {
                                crate::gml_geometry::parse_polygon(&mut geom_sub)?
                            } else {
                                geom_sub.skip_element()?;
                                crate::geometry::Polygon::default()
                            }
                        } else {
                            crate::geometry::Polygon::default()
                        }
                    });
                }
                (crate::namespace::NS_RELIEF, "adeOfBreaklineRelief") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_RELIEF, "ridgeOrValleyLines") => {
                    ridge_or_valley_lines = Some({
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
                (crate::namespace::NS_RELIEF, "breaklines") => {
                    breaklines = Some({
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
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(BreaklineRelief {
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
            lod,
            ade_of_abstract_relief_component,
            extent,
            ade_of_breakline_relief,
            ridge_or_valley_lines,
            breaklines,
        })
    }
}
impl crate::from_gml::FromGml for BreaklineRelief {
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
pub struct MassPointRelief {
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
    pub lod: IntegerBetween0and3,
    pub ade_of_abstract_relief_component: Vec<Box<dyn ADEOfAbstractReliefComponent>>,
    pub extent: Option<crate::geometry::Polygon>,
    pub ade_of_mass_point_relief: Vec<Box<dyn ADEOfMassPointRelief>>,
    pub point_cloud: Option<Box<dyn AbstractPointCloud>>,
    pub relief_points: Option<Vec<crate::geometry::DirectPosition>>,
}
impl AbstractFeature for MassPointRelief {
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
impl AbstractFeatureWithLifespan for MassPointRelief {
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
impl AbstractCityObject for MassPointRelief {
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
impl AbstractSpaceBoundary for MassPointRelief {
    fn ade_of_abstract_space_boundary(&self) -> &[Box<dyn ADEOfAbstractSpaceBoundary>] {
        &self.ade_of_abstract_space_boundary
    }
}
impl AbstractReliefComponent for MassPointRelief {
    fn lod(&self) -> &IntegerBetween0and3 {
        &self.lod
    }
    fn ade_of_abstract_relief_component(
        &self,
    ) -> &[Box<dyn ADEOfAbstractReliefComponent>] {
        &self.ade_of_abstract_relief_component
    }
    fn extent(&self) -> Option<&crate::geometry::Polygon> {
        self.extent.as_ref()
    }
}
impl MassPointRelief {
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
        let mut lod = Default::default();
        let mut ade_of_abstract_relief_component = Vec::new();
        let mut extent = None;
        let mut ade_of_mass_point_relief = Vec::new();
        let mut point_cloud = None;
        let mut relief_points = None;
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
                (crate::namespace::NS_RELIEF, "lod") => {
                    lod = IntegerBetween0and3(sub.read_text()?);
                }
                (crate::namespace::NS_RELIEF, "adeOfAbstractReliefComponent") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_RELIEF, "extent") => {
                    extent = Some({
                        let mut geom_sub = sub.subtree();
                        if let Some(geom_info) = geom_sub.next_element()? {
                            if geom_info.local_name == "Polygon" {
                                crate::gml_geometry::parse_polygon(&mut geom_sub)?
                            } else {
                                geom_sub.skip_element()?;
                                crate::geometry::Polygon::default()
                            }
                        } else {
                            crate::geometry::Polygon::default()
                        }
                    });
                }
                (crate::namespace::NS_RELIEF, "adeOfMassPointRelief") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_RELIEF, "pointCloud") => {
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
                (crate::namespace::NS_RELIEF, "reliefPoints") => {
                    relief_points = Some({
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
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(MassPointRelief {
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
            lod,
            ade_of_abstract_relief_component,
            extent,
            ade_of_mass_point_relief,
            point_cloud,
            relief_points,
        })
    }
}
impl crate::from_gml::FromGml for MassPointRelief {
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
pub struct RasterRelief {
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
    pub lod: IntegerBetween0and3,
    pub ade_of_abstract_relief_component: Vec<Box<dyn ADEOfAbstractReliefComponent>>,
    pub extent: Option<crate::geometry::Polygon>,
    pub ade_of_raster_relief: Vec<Box<dyn ADEOfRasterRelief>>,
    pub grid: (),
}
impl AbstractFeature for RasterRelief {
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
impl AbstractFeatureWithLifespan for RasterRelief {
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
impl AbstractCityObject for RasterRelief {
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
impl AbstractSpaceBoundary for RasterRelief {
    fn ade_of_abstract_space_boundary(&self) -> &[Box<dyn ADEOfAbstractSpaceBoundary>] {
        &self.ade_of_abstract_space_boundary
    }
}
impl AbstractReliefComponent for RasterRelief {
    fn lod(&self) -> &IntegerBetween0and3 {
        &self.lod
    }
    fn ade_of_abstract_relief_component(
        &self,
    ) -> &[Box<dyn ADEOfAbstractReliefComponent>] {
        &self.ade_of_abstract_relief_component
    }
    fn extent(&self) -> Option<&crate::geometry::Polygon> {
        self.extent.as_ref()
    }
}
impl RasterRelief {
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
        let mut lod = Default::default();
        let mut ade_of_abstract_relief_component = Vec::new();
        let mut extent = None;
        let mut ade_of_raster_relief = Vec::new();
        let mut grid = Default::default();
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
                (crate::namespace::NS_RELIEF, "lod") => {
                    lod = IntegerBetween0and3(sub.read_text()?);
                }
                (crate::namespace::NS_RELIEF, "adeOfAbstractReliefComponent") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_RELIEF, "extent") => {
                    extent = Some({
                        let mut geom_sub = sub.subtree();
                        if let Some(geom_info) = geom_sub.next_element()? {
                            if geom_info.local_name == "Polygon" {
                                crate::gml_geometry::parse_polygon(&mut geom_sub)?
                            } else {
                                geom_sub.skip_element()?;
                                crate::geometry::Polygon::default()
                            }
                        } else {
                            crate::geometry::Polygon::default()
                        }
                    });
                }
                (crate::namespace::NS_RELIEF, "adeOfRasterRelief") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_RELIEF, "grid") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(RasterRelief {
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
            lod,
            ade_of_abstract_relief_component,
            extent,
            ade_of_raster_relief,
            grid,
        })
    }
}
impl crate::from_gml::FromGml for RasterRelief {
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
pub struct TINRelief {
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
    pub lod: IntegerBetween0and3,
    pub ade_of_abstract_relief_component: Vec<Box<dyn ADEOfAbstractReliefComponent>>,
    pub extent: Option<crate::geometry::Polygon>,
    pub ade_of_tin_relief: Vec<Box<dyn ADEOfTINRelief>>,
    pub tin: crate::geometry::TriangulatedSurface,
}
impl AbstractFeature for TINRelief {
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
impl AbstractFeatureWithLifespan for TINRelief {
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
impl AbstractCityObject for TINRelief {
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
impl AbstractSpaceBoundary for TINRelief {
    fn ade_of_abstract_space_boundary(&self) -> &[Box<dyn ADEOfAbstractSpaceBoundary>] {
        &self.ade_of_abstract_space_boundary
    }
}
impl AbstractReliefComponent for TINRelief {
    fn lod(&self) -> &IntegerBetween0and3 {
        &self.lod
    }
    fn ade_of_abstract_relief_component(
        &self,
    ) -> &[Box<dyn ADEOfAbstractReliefComponent>] {
        &self.ade_of_abstract_relief_component
    }
    fn extent(&self) -> Option<&crate::geometry::Polygon> {
        self.extent.as_ref()
    }
}
impl TINRelief {
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
        let mut lod = Default::default();
        let mut ade_of_abstract_relief_component = Vec::new();
        let mut extent = None;
        let mut ade_of_tin_relief = Vec::new();
        let mut tin = Default::default();
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
                (crate::namespace::NS_RELIEF, "lod") => {
                    lod = IntegerBetween0and3(sub.read_text()?);
                }
                (crate::namespace::NS_RELIEF, "adeOfAbstractReliefComponent") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_RELIEF, "extent") => {
                    extent = Some({
                        let mut geom_sub = sub.subtree();
                        if let Some(geom_info) = geom_sub.next_element()? {
                            if geom_info.local_name == "Polygon" {
                                crate::gml_geometry::parse_polygon(&mut geom_sub)?
                            } else {
                                geom_sub.skip_element()?;
                                crate::geometry::Polygon::default()
                            }
                        } else {
                            crate::geometry::Polygon::default()
                        }
                    });
                }
                (crate::namespace::NS_RELIEF, "adeOfTINRelief") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_RELIEF, "tin") => {
                    tin = {
                        let mut geom_sub = sub.subtree();
                        if let Some(geom_info) = geom_sub.next_element()? {
                            if geom_info.local_name == "TriangulatedSurface"
                                || geom_info.local_name == "Tin"
                            {
                                crate::gml_geometry::parse_triangulated_surface(
                                    &mut geom_sub,
                                )?
                            } else {
                                geom_sub.skip_element()?;
                                crate::geometry::TriangulatedSurface::default()
                            }
                        } else {
                            crate::geometry::TriangulatedSurface::default()
                        }
                    };
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(TINRelief {
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
            lod,
            ade_of_abstract_relief_component,
            extent,
            ade_of_tin_relief,
            tin,
        })
    }
}
impl crate::from_gml::FromGml for TINRelief {
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
