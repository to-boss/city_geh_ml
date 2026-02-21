#![allow(unused_imports, unused_mut, unused_variables)]
use super::*;

pub trait AbstractReliefComponentTrait: AbstractSpaceBoundaryTrait {
    fn lod(&self) -> &IntegerBetween0and3;
    fn extent(&self) -> Option<&crate::geometry::Polygon>;
}

#[derive(Debug, Clone)]
pub enum AbstractReliefComponent {
    BreaklineRelief(BreaklineRelief),
    MassPointRelief(MassPointRelief),
    RasterRelief(RasterRelief),
    TINRelief(TINRelief),
}

impl Default for AbstractReliefComponent {
    fn default() -> Self {
        Self::BreaklineRelief(Default::default())
    }
}

impl AbstractFeatureTrait for AbstractReliefComponent {
    fn feature_id(&self) -> &ID {
        match self {
            Self::BreaklineRelief(v) => v.feature_id(),
            Self::MassPointRelief(v) => v.feature_id(),
            Self::RasterRelief(v) => v.feature_id(),
            Self::TINRelief(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::BreaklineRelief(v) => v.identifier(),
            Self::MassPointRelief(v) => v.identifier(),
            Self::RasterRelief(v) => v.identifier(),
            Self::TINRelief(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::BreaklineRelief(v) => v.name(),
            Self::MassPointRelief(v) => v.name(),
            Self::RasterRelief(v) => v.name(),
            Self::TINRelief(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::BreaklineRelief(v) => v.description(),
            Self::MassPointRelief(v) => v.description(),
            Self::RasterRelief(v) => v.description(),
            Self::TINRelief(v) => v.description(),
        }
    }
}

impl AbstractFeatureWithLifespanTrait for AbstractReliefComponent {
    fn creation_date(&self) -> Option<&String> {
        match self {
            Self::BreaklineRelief(v) => v.creation_date(),
            Self::MassPointRelief(v) => v.creation_date(),
            Self::RasterRelief(v) => v.creation_date(),
            Self::TINRelief(v) => v.creation_date(),
        }
    }
    fn termination_date(&self) -> Option<&String> {
        match self {
            Self::BreaklineRelief(v) => v.termination_date(),
            Self::MassPointRelief(v) => v.termination_date(),
            Self::RasterRelief(v) => v.termination_date(),
            Self::TINRelief(v) => v.termination_date(),
        }
    }
    fn valid_from(&self) -> Option<&String> {
        match self {
            Self::BreaklineRelief(v) => v.valid_from(),
            Self::MassPointRelief(v) => v.valid_from(),
            Self::RasterRelief(v) => v.valid_from(),
            Self::TINRelief(v) => v.valid_from(),
        }
    }
    fn valid_to(&self) -> Option<&String> {
        match self {
            Self::BreaklineRelief(v) => v.valid_to(),
            Self::MassPointRelief(v) => v.valid_to(),
            Self::RasterRelief(v) => v.valid_to(),
            Self::TINRelief(v) => v.valid_to(),
        }
    }
}

impl AbstractCityObjectTrait for AbstractReliefComponent {
    fn relative_to_terrain(&self) -> Option<RelativeToTerrain> {
        match self {
            Self::BreaklineRelief(v) => v.relative_to_terrain(),
            Self::MassPointRelief(v) => v.relative_to_terrain(),
            Self::RasterRelief(v) => v.relative_to_terrain(),
            Self::TINRelief(v) => v.relative_to_terrain(),
        }
    }
    fn relative_to_water(&self) -> Option<RelativeToWater> {
        match self {
            Self::BreaklineRelief(v) => v.relative_to_water(),
            Self::MassPointRelief(v) => v.relative_to_water(),
            Self::RasterRelief(v) => v.relative_to_water(),
            Self::TINRelief(v) => v.relative_to_water(),
        }
    }
    fn appearance(&self) -> &[AbstractAppearance] {
        match self {
            Self::BreaklineRelief(v) => v.appearance(),
            Self::MassPointRelief(v) => v.appearance(),
            Self::RasterRelief(v) => v.appearance(),
            Self::TINRelief(v) => v.appearance(),
        }
    }
    fn generalizes_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::BreaklineRelief(v) => v.generalizes_to(),
            Self::MassPointRelief(v) => v.generalizes_to(),
            Self::RasterRelief(v) => v.generalizes_to(),
            Self::TINRelief(v) => v.generalizes_to(),
        }
    }
    fn external_reference(&self) -> &[ExternalReference] {
        match self {
            Self::BreaklineRelief(v) => v.external_reference(),
            Self::MassPointRelief(v) => v.external_reference(),
            Self::RasterRelief(v) => v.external_reference(),
            Self::TINRelief(v) => v.external_reference(),
        }
    }
    fn related_to(&self) -> &[AbstractCityObject] {
        match self {
            Self::BreaklineRelief(v) => v.related_to(),
            Self::MassPointRelief(v) => v.related_to(),
            Self::RasterRelief(v) => v.related_to(),
            Self::TINRelief(v) => v.related_to(),
        }
    }
    fn dynamizer(&self) -> &[AbstractDynamizer] {
        match self {
            Self::BreaklineRelief(v) => v.dynamizer(),
            Self::MassPointRelief(v) => v.dynamizer(),
            Self::RasterRelief(v) => v.dynamizer(),
            Self::TINRelief(v) => v.dynamizer(),
        }
    }
}

impl AbstractSpaceBoundaryTrait for AbstractReliefComponent {}
impl AbstractReliefComponentTrait for AbstractReliefComponent {
    fn lod(&self) -> &IntegerBetween0and3 {
        match self {
            Self::BreaklineRelief(v) => v.lod(),
            Self::MassPointRelief(v) => v.lod(),
            Self::RasterRelief(v) => v.lod(),
            Self::TINRelief(v) => v.lod(),
        }
    }
    fn extent(&self) -> Option<&crate::geometry::Polygon> {
        match self {
            Self::BreaklineRelief(v) => v.extent(),
            Self::MassPointRelief(v) => v.extent(),
            Self::RasterRelief(v) => v.extent(),
            Self::TINRelief(v) => v.extent(),
        }
    }
}

impl From<BreaklineRelief> for AbstractReliefComponent {
    fn from(v: BreaklineRelief) -> Self {
        Self::BreaklineRelief(v)
    }
}

impl From<MassPointRelief> for AbstractReliefComponent {
    fn from(v: MassPointRelief) -> Self {
        Self::MassPointRelief(v)
    }
}

impl From<RasterRelief> for AbstractReliefComponent {
    fn from(v: RasterRelief) -> Self {
        Self::RasterRelief(v)
    }
}

impl From<TINRelief> for AbstractReliefComponent {
    fn from(v: TINRelief) -> Self {
        Self::TINRelief(v)
    }
}

pub trait AbstractReliefComponentAccessors {
    fn breakline_reliefs(&self) -> impl Iterator<Item = &BreaklineRelief>;
    fn mass_point_reliefs(&self) -> impl Iterator<Item = &MassPointRelief>;
    fn raster_reliefs(&self) -> impl Iterator<Item = &RasterRelief>;
    fn tin_reliefs(&self) -> impl Iterator<Item = &TINRelief>;
}

impl AbstractReliefComponentAccessors for [AbstractReliefComponent] {
    fn breakline_reliefs(&self) -> impl Iterator<Item = &BreaklineRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractReliefComponent::BreaklineRelief(v) => Some(v),
                _ => None,
            })
    }
    fn mass_point_reliefs(&self) -> impl Iterator<Item = &MassPointRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractReliefComponent::MassPointRelief(v) => Some(v),
                _ => None,
            })
    }
    fn raster_reliefs(&self) -> impl Iterator<Item = &RasterRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractReliefComponent::RasterRelief(v) => Some(v),
                _ => None,
            })
    }
    fn tin_reliefs(&self) -> impl Iterator<Item = &TINRelief> {
        self.iter()
            .filter_map(|item| match item {
                AbstractReliefComponent::TINRelief(v) => Some(v),
                _ => None,
            })
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReliefFeature {
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
    pub lod: IntegerBetween0and3,
    pub relief_component: Vec<AbstractReliefComponent>,
}

impl AbstractFeatureTrait for ReliefFeature {
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

impl AbstractFeatureWithLifespanTrait for ReliefFeature {
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

impl AbstractCityObjectTrait for ReliefFeature {
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

impl AbstractSpaceBoundaryTrait for ReliefFeature {}
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
        let mut lod = Default::default();
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
                (crate::namespace::NS_RELIEF, "lod") => {
                    lod = IntegerBetween0and3(sub.read_text()?);
                }
                (crate::namespace::NS_RELIEF, "reliefComponent") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        relief_component
                            .push(
                                super::dispatchers::parse_abstract_relief_component(
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
            lod,
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

#[derive(Debug, Clone, Default)]
pub struct BreaklineRelief {
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
    pub lod: IntegerBetween0and3,
    pub extent: Option<crate::geometry::Polygon>,
    pub ridge_or_valley_lines: Option<crate::geometry::MultiCurve>,
    pub breaklines: Option<crate::geometry::MultiCurve>,
}

impl AbstractFeatureTrait for BreaklineRelief {
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

impl AbstractFeatureWithLifespanTrait for BreaklineRelief {
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

impl AbstractCityObjectTrait for BreaklineRelief {
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

impl AbstractSpaceBoundaryTrait for BreaklineRelief {}
impl AbstractReliefComponentTrait for BreaklineRelief {
    fn lod(&self) -> &IntegerBetween0and3 {
        &self.lod
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
        let mut lod = Default::default();
        let mut extent = None;
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
                (crate::namespace::NS_RELIEF, "lod") => {
                    lod = IntegerBetween0and3(sub.read_text()?);
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
            lod,
            extent,
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

#[derive(Debug, Clone, Default)]
pub struct MassPointRelief {
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
    pub lod: IntegerBetween0and3,
    pub extent: Option<crate::geometry::Polygon>,
    pub point_cloud: Option<AbstractPointCloud>,
    pub relief_points: Option<Vec<crate::geometry::DirectPosition>>,
}

impl AbstractFeatureTrait for MassPointRelief {
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

impl AbstractFeatureWithLifespanTrait for MassPointRelief {
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

impl AbstractCityObjectTrait for MassPointRelief {
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

impl AbstractSpaceBoundaryTrait for MassPointRelief {}
impl AbstractReliefComponentTrait for MassPointRelief {
    fn lod(&self) -> &IntegerBetween0and3 {
        &self.lod
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
        let mut lod = Default::default();
        let mut extent = None;
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
                (crate::namespace::NS_RELIEF, "lod") => {
                    lod = IntegerBetween0and3(sub.read_text()?);
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
                (crate::namespace::NS_RELIEF, "pointCloud") => {
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
            lod,
            extent,
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

#[derive(Debug, Clone, Default)]
pub struct RasterRelief {
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
    pub lod: IntegerBetween0and3,
    pub extent: Option<crate::geometry::Polygon>,
    pub grid: (),
}

impl AbstractFeatureTrait for RasterRelief {
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

impl AbstractFeatureWithLifespanTrait for RasterRelief {
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

impl AbstractCityObjectTrait for RasterRelief {
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

impl AbstractSpaceBoundaryTrait for RasterRelief {}
impl AbstractReliefComponentTrait for RasterRelief {
    fn lod(&self) -> &IntegerBetween0and3 {
        &self.lod
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
        let mut lod = Default::default();
        let mut extent = None;
        let grid = ();
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
                (crate::namespace::NS_RELIEF, "lod") => {
                    lod = IntegerBetween0and3(sub.read_text()?);
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
            lod,
            extent,
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

#[derive(Debug, Clone, Default)]
pub struct TINRelief {
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
    pub lod: IntegerBetween0and3,
    pub extent: Option<crate::geometry::Polygon>,
    pub tin: crate::geometry::TriangulatedSurface,
}

impl AbstractFeatureTrait for TINRelief {
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

impl AbstractFeatureWithLifespanTrait for TINRelief {
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

impl AbstractCityObjectTrait for TINRelief {
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

impl AbstractSpaceBoundaryTrait for TINRelief {}
impl AbstractReliefComponentTrait for TINRelief {
    fn lod(&self) -> &IntegerBetween0and3 {
        &self.lod
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
        let mut lod = Default::default();
        let mut extent = None;
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
                (crate::namespace::NS_RELIEF, "lod") => {
                    lod = IntegerBetween0and3(sub.read_text()?);
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
            lod,
            extent,
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
