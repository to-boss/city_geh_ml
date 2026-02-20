#![allow(unused_imports, unused_mut, unused_variables)]
use super::*;

pub trait ADEOfGenericLogicalSpace: std::fmt::Debug {}
pub trait ADEOfGenericOccupiedSpace: std::fmt::Debug {}
pub trait ADEOfGenericThematicSurface: std::fmt::Debug {}
pub trait ADEOfGenericUnoccupiedSpace: std::fmt::Debug {}
#[derive(Debug, Clone, Default)]
pub struct CodeAttribute {
    pub name: String,
    pub value: Code,
}
impl crate::from_gml::FromGml for CodeAttribute {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut name = Default::default();
        let mut value = Default::default();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_GML, "name") => {
                    name = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_GENERICS, "value") => {
                    let child_info = crate::gml_reader::ElementInfo {
                        namespace: info.namespace.clone(),
                        local_name: info.local_name.clone(),
                        attributes: info.attributes.clone(),
                    };
                    value = Code::from_gml_with_info(&mut sub, &child_info)?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(CodeAttribute { name, value })
    }
}
#[derive(Debug, Clone, Default)]
pub struct DateAttribute {
    pub name: String,
    pub value: String,
}
impl crate::from_gml::FromGml for DateAttribute {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut name = Default::default();
        let mut value = Default::default();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_GML, "name") => {
                    name = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_GENERICS, "value") => {
                    value = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(DateAttribute { name, value })
    }
}
#[derive(Debug, Clone, Default)]
pub struct DoubleAttribute {
    pub name: String,
    pub value: f64,
}
impl crate::from_gml::FromGml for DoubleAttribute {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut name = Default::default();
        let mut value = 0.0f64;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_GML, "name") => {
                    name = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_GENERICS, "value") => {
                    value = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(DoubleAttribute { name, value })
    }
}
#[derive(Debug, Default)]
pub struct GenericAttributeSet {
    pub name: String,
    pub code_space: Option<String>,
    pub generic_attribute: Vec<Box<dyn AbstractGenericAttribute>>,
}
impl crate::from_gml::FromGml for GenericAttributeSet {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut name = Default::default();
        let mut code_space = None;
        let mut generic_attribute = Vec::new();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_GML, "name") => {
                    name = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_GENERICS, "codeSpace") => {
                    code_space = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_GENERICS, "genericAttribute") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(GenericAttributeSet {
            name,
            code_space,
            generic_attribute,
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct IntAttribute {
    pub name: String,
    pub value: i64,
}
impl crate::from_gml::FromGml for IntAttribute {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut name = Default::default();
        let mut value = 0i64;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_GML, "name") => {
                    name = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_GENERICS, "value") => {
                    value = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(IntAttribute { name, value })
    }
}
#[derive(Debug, Clone, Default)]
pub struct MeasureAttribute {
    pub name: String,
    pub value: f64,
}
impl crate::from_gml::FromGml for MeasureAttribute {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut name = Default::default();
        let mut value = 0.0f64;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_GML, "name") => {
                    name = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_GENERICS, "value") => {
                    value = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(MeasureAttribute { name, value })
    }
}
#[derive(Debug, Clone, Default)]
pub struct StringAttribute {
    pub name: String,
    pub value: String,
}
impl crate::from_gml::FromGml for StringAttribute {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut name = Default::default();
        let mut value = Default::default();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_GML, "name") => {
                    name = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_GENERICS, "value") => {
                    value = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(StringAttribute { name, value })
    }
}
#[derive(Debug, Clone, Default)]
pub struct UriAttribute {
    pub name: String,
    pub value: String,
}
impl crate::from_gml::FromGml for UriAttribute {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut name = Default::default();
        let mut value = Default::default();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_GML, "name") => {
                    name = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_GENERICS, "value") => {
                    value = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(UriAttribute { name, value })
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GenericLogicalSpaceClassValue(pub String);
impl crate::from_gml::FromGml for GenericLogicalSpaceClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(GenericLogicalSpaceClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GenericLogicalSpaceFunctionValue(pub String);
impl crate::from_gml::FromGml for GenericLogicalSpaceFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(GenericLogicalSpaceFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GenericLogicalSpaceUsageValue(pub String);
impl crate::from_gml::FromGml for GenericLogicalSpaceUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(GenericLogicalSpaceUsageValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GenericOccupiedSpaceClassValue(pub String);
impl crate::from_gml::FromGml for GenericOccupiedSpaceClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(GenericOccupiedSpaceClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GenericOccupiedSpaceFunctionValue(pub String);
impl crate::from_gml::FromGml for GenericOccupiedSpaceFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(GenericOccupiedSpaceFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GenericOccupiedSpaceUsageValue(pub String);
impl crate::from_gml::FromGml for GenericOccupiedSpaceUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(GenericOccupiedSpaceUsageValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GenericThematicSurfaceClassValue(pub String);
impl crate::from_gml::FromGml for GenericThematicSurfaceClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(GenericThematicSurfaceClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GenericThematicSurfaceFunctionValue(pub String);
impl crate::from_gml::FromGml for GenericThematicSurfaceFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(GenericThematicSurfaceFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GenericThematicSurfaceUsageValue(pub String);
impl crate::from_gml::FromGml for GenericThematicSurfaceUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(GenericThematicSurfaceUsageValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GenericUnoccupiedSpaceClassValue(pub String);
impl crate::from_gml::FromGml for GenericUnoccupiedSpaceClassValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(GenericUnoccupiedSpaceClassValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GenericUnoccupiedSpaceFunctionValue(pub String);
impl crate::from_gml::FromGml for GenericUnoccupiedSpaceFunctionValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(GenericUnoccupiedSpaceFunctionValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GenericUnoccupiedSpaceUsageValue(pub String);
impl crate::from_gml::FromGml for GenericUnoccupiedSpaceUsageValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(GenericUnoccupiedSpaceUsageValue(reader.read_text()?))
    }
}
#[derive(Debug, Default)]
pub struct GenericLogicalSpace {
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
    pub ade_of_abstract_logical_space: Vec<Box<dyn ADEOfAbstractLogicalSpace>>,
    pub class_: Option<GenericLogicalSpaceClassValue>,
    pub function: Vec<GenericLogicalSpaceFunctionValue>,
    pub usage: Vec<GenericLogicalSpaceUsageValue>,
    pub ade_of_generic_logical_space: Vec<Box<dyn ADEOfGenericLogicalSpace>>,
}
impl AbstractFeature for GenericLogicalSpace {
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
impl AbstractFeatureWithLifespan for GenericLogicalSpace {
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
impl AbstractCityObject for GenericLogicalSpace {
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
impl AbstractSpace for GenericLogicalSpace {
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
impl AbstractLogicalSpace for GenericLogicalSpace {
    fn ade_of_abstract_logical_space(&self) -> &[Box<dyn ADEOfAbstractLogicalSpace>] {
        &self.ade_of_abstract_logical_space
    }
}
impl GenericLogicalSpace {
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
        let mut ade_of_abstract_logical_space = Vec::new();
        let mut class_ = None;
        let mut function = Vec::new();
        let mut usage = Vec::new();
        let mut ade_of_generic_logical_space = Vec::new();
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
                (crate::namespace::NS_CORE, "adeOfAbstractLogicalSpace") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_GENERICS, "class") => {
                    class_ = Some(GenericLogicalSpaceClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_GENERICS, "function") => {
                    function.push(GenericLogicalSpaceFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_GENERICS, "usage") => {
                    usage.push(GenericLogicalSpaceUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_GENERICS, "adeOfGenericLogicalSpace") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(GenericLogicalSpace {
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
            ade_of_abstract_logical_space,
            class_,
            function,
            usage,
            ade_of_generic_logical_space,
        })
    }
}
impl crate::from_gml::FromGml for GenericLogicalSpace {
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
pub struct GenericThematicSurface {
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
    pub class_: Option<GenericThematicSurfaceClassValue>,
    pub function: Vec<GenericThematicSurfaceFunctionValue>,
    pub usage: Vec<GenericThematicSurfaceUsageValue>,
    pub ade_of_generic_thematic_surface: Vec<Box<dyn ADEOfGenericThematicSurface>>,
}
impl AbstractFeature for GenericThematicSurface {
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
impl AbstractFeatureWithLifespan for GenericThematicSurface {
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
impl AbstractCityObject for GenericThematicSurface {
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
impl AbstractSpaceBoundary for GenericThematicSurface {
    fn ade_of_abstract_space_boundary(&self) -> &[Box<dyn ADEOfAbstractSpaceBoundary>] {
        &self.ade_of_abstract_space_boundary
    }
}
impl AbstractThematicSurface for GenericThematicSurface {
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
impl GenericThematicSurface {
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
        let mut ade_of_generic_thematic_surface = Vec::new();
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
                (crate::namespace::NS_GENERICS, "class") => {
                    class_ = Some(GenericThematicSurfaceClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_GENERICS, "function") => {
                    function.push(GenericThematicSurfaceFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_GENERICS, "usage") => {
                    usage.push(GenericThematicSurfaceUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_GENERICS, "adeOfGenericThematicSurface") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(GenericThematicSurface {
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
            ade_of_generic_thematic_surface,
        })
    }
}
impl crate::from_gml::FromGml for GenericThematicSurface {
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
pub struct GenericOccupiedSpace {
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
    pub ade_of_abstract_occupied_space: Vec<Box<dyn ADEOfAbstractOccupiedSpace>>,
    pub lod3_implicit_representation: Option<ImplicitGeometry>,
    pub lod2_implicit_representation: Option<ImplicitGeometry>,
    pub lod1_implicit_representation: Option<ImplicitGeometry>,
    pub class_: Option<GenericOccupiedSpaceClassValue>,
    pub function: Vec<GenericOccupiedSpaceFunctionValue>,
    pub usage: Vec<GenericOccupiedSpaceUsageValue>,
    pub ade_of_generic_occupied_space: Vec<Box<dyn ADEOfGenericOccupiedSpace>>,
}
impl AbstractFeature for GenericOccupiedSpace {
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
impl AbstractFeatureWithLifespan for GenericOccupiedSpace {
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
impl AbstractCityObject for GenericOccupiedSpace {
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
impl AbstractSpace for GenericOccupiedSpace {
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
impl AbstractPhysicalSpace for GenericOccupiedSpace {
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
impl AbstractOccupiedSpace for GenericOccupiedSpace {
    fn ade_of_abstract_occupied_space(&self) -> &[Box<dyn ADEOfAbstractOccupiedSpace>] {
        &self.ade_of_abstract_occupied_space
    }
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
impl GenericOccupiedSpace {
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
        let mut ade_of_abstract_occupied_space = Vec::new();
        let mut lod3_implicit_representation = None;
        let mut lod2_implicit_representation = None;
        let mut lod1_implicit_representation = None;
        let mut class_ = None;
        let mut function = Vec::new();
        let mut usage = Vec::new();
        let mut ade_of_generic_occupied_space = Vec::new();
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
                (crate::namespace::NS_CORE, "adeOfAbstractOccupiedSpace") => {
                    sub.skip_element()?;
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
                (crate::namespace::NS_GENERICS, "class") => {
                    class_ = Some(GenericOccupiedSpaceClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_GENERICS, "function") => {
                    function.push(GenericOccupiedSpaceFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_GENERICS, "usage") => {
                    usage.push(GenericOccupiedSpaceUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_GENERICS, "adeOfGenericOccupiedSpace") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(GenericOccupiedSpace {
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
            ade_of_abstract_occupied_space,
            lod3_implicit_representation,
            lod2_implicit_representation,
            lod1_implicit_representation,
            class_,
            function,
            usage,
            ade_of_generic_occupied_space,
        })
    }
}
impl crate::from_gml::FromGml for GenericOccupiedSpace {
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
pub struct GenericUnoccupiedSpace {
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
    pub class_: Option<GenericUnoccupiedSpaceClassValue>,
    pub function: Vec<GenericUnoccupiedSpaceFunctionValue>,
    pub usage: Vec<GenericUnoccupiedSpaceUsageValue>,
    pub ade_of_generic_unoccupied_space: Vec<Box<dyn ADEOfGenericUnoccupiedSpace>>,
}
impl AbstractFeature for GenericUnoccupiedSpace {
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
impl AbstractFeatureWithLifespan for GenericUnoccupiedSpace {
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
impl AbstractCityObject for GenericUnoccupiedSpace {
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
impl AbstractSpace for GenericUnoccupiedSpace {
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
impl AbstractPhysicalSpace for GenericUnoccupiedSpace {
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
impl AbstractUnoccupiedSpace for GenericUnoccupiedSpace {
    fn ade_of_abstract_unoccupied_space(
        &self,
    ) -> &[Box<dyn ADEOfAbstractUnoccupiedSpace>] {
        &self.ade_of_abstract_unoccupied_space
    }
}
impl GenericUnoccupiedSpace {
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
        let mut ade_of_generic_unoccupied_space = Vec::new();
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
                (crate::namespace::NS_GENERICS, "class") => {
                    class_ = Some(GenericUnoccupiedSpaceClassValue(sub.read_text()?));
                }
                (crate::namespace::NS_GENERICS, "function") => {
                    function.push(GenericUnoccupiedSpaceFunctionValue(sub.read_text()?));
                }
                (crate::namespace::NS_GENERICS, "usage") => {
                    usage.push(GenericUnoccupiedSpaceUsageValue(sub.read_text()?));
                }
                (crate::namespace::NS_GENERICS, "adeOfGenericUnoccupiedSpace") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(GenericUnoccupiedSpace {
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
            ade_of_generic_unoccupied_space,
        })
    }
}
impl crate::from_gml::FromGml for GenericUnoccupiedSpace {
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
