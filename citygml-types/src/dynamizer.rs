#![allow(unused_imports, unused_mut, unused_variables)]
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TimeseriesTypeValue {
    #[default]
    Int,
    Double,
    String,
    Geometry,
    Uri,
    Bool,
    ImplicitGeometry,
    Appearance,
}
impl TimeseriesTypeValue {
    pub fn from_gml_text(text: &str) -> Result<Self, crate::error::ReaderError> {
        match text.trim() {
            "int" => Ok(TimeseriesTypeValue::Int),
            "double" => Ok(TimeseriesTypeValue::Double),
            "string" => Ok(TimeseriesTypeValue::String),
            "geometry" => Ok(TimeseriesTypeValue::Geometry),
            "uri" => Ok(TimeseriesTypeValue::Uri),
            "bool" => Ok(TimeseriesTypeValue::Bool),
            "implicitGeometry" => Ok(TimeseriesTypeValue::ImplicitGeometry),
            "appearance" => Ok(TimeseriesTypeValue::Appearance),
            other => {
                Err(crate::error::ReaderError::Parse {
                    message: format!(
                        "Unknown {} value: '{}'", stringify!(TimeseriesTypeValue), other,
                    ),
                })
            }
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct SensorConnection {
    pub connection_type: SensorConnectionTypeValue,
    pub observation_property: String,
    pub uom: Option<String>,
    pub sensor_id: Option<String>,
    pub sensor_name: Option<String>,
    pub observation_id: Option<String>,
    pub datastream_id: Option<String>,
    pub base_url: Option<String>,
    pub auth_type: Option<AuthenticationTypeValue>,
    pub mqtt_server: Option<String>,
    pub mqtt_topic: Option<String>,
    pub link_to_observation: Option<String>,
    pub link_to_sensor_description: Option<String>,
    pub sensor_location: Option<AbstractCityObject>,
}
impl crate::from_gml::FromGml for SensorConnection {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut connection_type = Default::default();
        let mut observation_property = Default::default();
        let mut uom = None;
        let mut sensor_id = None;
        let mut sensor_name = None;
        let mut observation_id = None;
        let mut datastream_id = None;
        let mut base_url = None;
        let mut auth_type = None;
        let mut mqtt_server = None;
        let mut mqtt_topic = None;
        let mut link_to_observation = None;
        let mut link_to_sensor_description = None;
        let mut sensor_location = None;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_DYNAMIZER, "connectionType") => {
                    connection_type = SensorConnectionTypeValue(sub.read_text()?);
                }
                (crate::namespace::NS_DYNAMIZER, "observationProperty") => {
                    observation_property = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_DYNAMIZER, "uom") => {
                    uom = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "sensorID") => {
                    sensor_id = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "sensorName") => {
                    sensor_name = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "observationID") => {
                    observation_id = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "datastreamID") => {
                    datastream_id = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "baseURL") => {
                    base_url = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "authType") => {
                    auth_type = Some(AuthenticationTypeValue(sub.read_text()?));
                }
                (crate::namespace::NS_DYNAMIZER, "mqttServer") => {
                    mqtt_server = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "mqttTopic") => {
                    mqtt_topic = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "linkToObservation") => {
                    link_to_observation = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_DYNAMIZER, "linkToSensorDescription") => {
                    link_to_sensor_description = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_DYNAMIZER, "sensorLocation") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        sensor_location = Some(
                            super::dispatchers::parse_abstract_city_object(
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
        Ok(SensorConnection {
            connection_type,
            observation_property,
            uom,
            sensor_id,
            sensor_name,
            observation_id,
            datastream_id,
            base_url,
            auth_type,
            mqtt_server,
            mqtt_topic,
            link_to_observation,
            link_to_sensor_description,
            sensor_location,
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct TimeseriesComponent {
    pub repetitions: i64,
    pub additional_gap: Option<String>,
    pub timeseries: Option<AbstractTimeseries>,
}
impl crate::from_gml::FromGml for TimeseriesComponent {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut repetitions = 0i64;
        let mut additional_gap = None;
        let mut timeseries = None;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_DYNAMIZER, "repetitions") => {
                    repetitions = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_DYNAMIZER, "additionalGap") => {
                    additional_gap = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "timeseries") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        timeseries = Some(
                            super::dispatchers::parse_abstract_timeseries(
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
        Ok(TimeseriesComponent {
            repetitions,
            additional_gap,
            timeseries,
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct TimeValuePair {
    pub timestamp: String,
    pub int_value: Option<i64>,
    pub double_value: Option<f64>,
    pub string_value: Option<String>,
    pub geometry_value: Option<()>,
    pub uri_value: Option<String>,
    pub bool_value: Option<bool>,
    pub implicit_geometry_value: Option<ImplicitGeometry>,
    pub appearance_value: Option<AbstractAppearance>,
}
impl crate::from_gml::FromGml for TimeValuePair {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut timestamp = Default::default();
        let mut int_value = None;
        let mut double_value = None;
        let mut string_value = None;
        let mut geometry_value = None;
        let mut uri_value = None;
        let mut bool_value = None;
        let mut implicit_geometry_value = None;
        let mut appearance_value = None;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_DYNAMIZER, "timestamp") => {
                    timestamp = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_DYNAMIZER, "intValue") => {
                    int_value = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "doubleValue") => {
                    double_value = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "stringValue") => {
                    string_value = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "geometryValue") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_DYNAMIZER, "uriValue") => {
                    uri_value = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "boolValue") => {
                    bool_value = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "implicitGeometryValue") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        implicit_geometry_value = Some(
                            ImplicitGeometry::from_gml_with_info(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    } else {
                        implicit_geometry_value = Some(
                            ImplicitGeometry::from_gml(&mut sub)?,
                        );
                    }
                }
                (crate::namespace::NS_DYNAMIZER, "appearanceValue") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        appearance_value = Some(
                            super::dispatchers::parse_abstract_appearance(
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
        Ok(TimeValuePair {
            timestamp,
            int_value,
            double_value,
            string_value,
            geometry_value,
            uri_value,
            bool_value,
            implicit_geometry_value,
            appearance_value,
        })
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AuthenticationTypeValue(pub String);
impl crate::from_gml::FromGml for AuthenticationTypeValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(AuthenticationTypeValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SensorConnectionTypeValue(pub String);
impl crate::from_gml::FromGml for SensorConnectionTypeValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(SensorConnectionTypeValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct StandardFileTypeValue(pub String);
impl crate::from_gml::FromGml for StandardFileTypeValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(StandardFileTypeValue(reader.read_text()?))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TabulatedFileTypeValue(pub String);
impl crate::from_gml::FromGml for TabulatedFileTypeValue {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        Ok(TabulatedFileTypeValue(reader.read_text()?))
    }
}
pub trait AbstractTimeseriesTrait: AbstractFeatureTrait {
    fn first_timestamp(&self) -> Option<&String>;
    fn last_timestamp(&self) -> Option<&String>;
}
#[derive(Debug, Clone)]
pub enum AbstractTimeseries {
    CompositeTimeseries(CompositeTimeseries),
    GenericTimeseries(GenericTimeseries),
    StandardFileTimeseries(StandardFileTimeseries),
    TabulatedFileTimeseries(TabulatedFileTimeseries),
}
impl Default for AbstractTimeseries {
    fn default() -> Self {
        Self::CompositeTimeseries(Default::default())
    }
}
impl AbstractFeatureTrait for AbstractTimeseries {
    fn feature_id(&self) -> &ID {
        match self {
            Self::CompositeTimeseries(v) => v.feature_id(),
            Self::GenericTimeseries(v) => v.feature_id(),
            Self::StandardFileTimeseries(v) => v.feature_id(),
            Self::TabulatedFileTimeseries(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::CompositeTimeseries(v) => v.identifier(),
            Self::GenericTimeseries(v) => v.identifier(),
            Self::StandardFileTimeseries(v) => v.identifier(),
            Self::TabulatedFileTimeseries(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::CompositeTimeseries(v) => v.name(),
            Self::GenericTimeseries(v) => v.name(),
            Self::StandardFileTimeseries(v) => v.name(),
            Self::TabulatedFileTimeseries(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::CompositeTimeseries(v) => v.description(),
            Self::GenericTimeseries(v) => v.description(),
            Self::StandardFileTimeseries(v) => v.description(),
            Self::TabulatedFileTimeseries(v) => v.description(),
        }
    }
}
impl AbstractTimeseriesTrait for AbstractTimeseries {
    fn first_timestamp(&self) -> Option<&String> {
        match self {
            Self::CompositeTimeseries(v) => v.first_timestamp(),
            Self::GenericTimeseries(v) => v.first_timestamp(),
            Self::StandardFileTimeseries(v) => v.first_timestamp(),
            Self::TabulatedFileTimeseries(v) => v.first_timestamp(),
        }
    }
    fn last_timestamp(&self) -> Option<&String> {
        match self {
            Self::CompositeTimeseries(v) => v.last_timestamp(),
            Self::GenericTimeseries(v) => v.last_timestamp(),
            Self::StandardFileTimeseries(v) => v.last_timestamp(),
            Self::TabulatedFileTimeseries(v) => v.last_timestamp(),
        }
    }
}
impl From<CompositeTimeseries> for AbstractTimeseries {
    fn from(v: CompositeTimeseries) -> Self {
        Self::CompositeTimeseries(v)
    }
}
impl From<GenericTimeseries> for AbstractTimeseries {
    fn from(v: GenericTimeseries) -> Self {
        Self::GenericTimeseries(v)
    }
}
impl From<StandardFileTimeseries> for AbstractTimeseries {
    fn from(v: StandardFileTimeseries) -> Self {
        Self::StandardFileTimeseries(v)
    }
}
impl From<TabulatedFileTimeseries> for AbstractTimeseries {
    fn from(v: TabulatedFileTimeseries) -> Self {
        Self::TabulatedFileTimeseries(v)
    }
}
pub trait AbstractAtomicTimeseriesTrait: AbstractTimeseriesTrait {
    fn observation_property(&self) -> &String;
    fn uom(&self) -> Option<&String>;
}
#[derive(Debug, Clone)]
pub enum AbstractAtomicTimeseries {
    GenericTimeseries(GenericTimeseries),
    StandardFileTimeseries(StandardFileTimeseries),
    TabulatedFileTimeseries(TabulatedFileTimeseries),
}
impl Default for AbstractAtomicTimeseries {
    fn default() -> Self {
        Self::GenericTimeseries(Default::default())
    }
}
impl AbstractFeatureTrait for AbstractAtomicTimeseries {
    fn feature_id(&self) -> &ID {
        match self {
            Self::GenericTimeseries(v) => v.feature_id(),
            Self::StandardFileTimeseries(v) => v.feature_id(),
            Self::TabulatedFileTimeseries(v) => v.feature_id(),
        }
    }
    fn identifier(&self) -> Option<&String> {
        match self {
            Self::GenericTimeseries(v) => v.identifier(),
            Self::StandardFileTimeseries(v) => v.identifier(),
            Self::TabulatedFileTimeseries(v) => v.identifier(),
        }
    }
    fn name(&self) -> &[String] {
        match self {
            Self::GenericTimeseries(v) => v.name(),
            Self::StandardFileTimeseries(v) => v.name(),
            Self::TabulatedFileTimeseries(v) => v.name(),
        }
    }
    fn description(&self) -> Option<&String> {
        match self {
            Self::GenericTimeseries(v) => v.description(),
            Self::StandardFileTimeseries(v) => v.description(),
            Self::TabulatedFileTimeseries(v) => v.description(),
        }
    }
}
impl AbstractTimeseriesTrait for AbstractAtomicTimeseries {
    fn first_timestamp(&self) -> Option<&String> {
        match self {
            Self::GenericTimeseries(v) => v.first_timestamp(),
            Self::StandardFileTimeseries(v) => v.first_timestamp(),
            Self::TabulatedFileTimeseries(v) => v.first_timestamp(),
        }
    }
    fn last_timestamp(&self) -> Option<&String> {
        match self {
            Self::GenericTimeseries(v) => v.last_timestamp(),
            Self::StandardFileTimeseries(v) => v.last_timestamp(),
            Self::TabulatedFileTimeseries(v) => v.last_timestamp(),
        }
    }
}
impl AbstractAtomicTimeseriesTrait for AbstractAtomicTimeseries {
    fn observation_property(&self) -> &String {
        match self {
            Self::GenericTimeseries(v) => v.observation_property(),
            Self::StandardFileTimeseries(v) => v.observation_property(),
            Self::TabulatedFileTimeseries(v) => v.observation_property(),
        }
    }
    fn uom(&self) -> Option<&String> {
        match self {
            Self::GenericTimeseries(v) => v.uom(),
            Self::StandardFileTimeseries(v) => v.uom(),
            Self::TabulatedFileTimeseries(v) => v.uom(),
        }
    }
}
impl From<GenericTimeseries> for AbstractAtomicTimeseries {
    fn from(v: GenericTimeseries) -> Self {
        Self::GenericTimeseries(v)
    }
}
impl From<StandardFileTimeseries> for AbstractAtomicTimeseries {
    fn from(v: StandardFileTimeseries) -> Self {
        Self::StandardFileTimeseries(v)
    }
}
impl From<TabulatedFileTimeseries> for AbstractAtomicTimeseries {
    fn from(v: TabulatedFileTimeseries) -> Self {
        Self::TabulatedFileTimeseries(v)
    }
}
#[derive(Debug, Clone, Default)]
pub struct CompositeTimeseries {
    pub feature_id: ID,
    pub identifier: Option<String>,
    pub name: Vec<String>,
    pub description: Option<String>,
    pub first_timestamp: Option<String>,
    pub last_timestamp: Option<String>,
    pub component: Vec<TimeseriesComponent>,
}
impl AbstractFeatureTrait for CompositeTimeseries {
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
impl AbstractTimeseriesTrait for CompositeTimeseries {
    fn first_timestamp(&self) -> Option<&String> {
        self.first_timestamp.as_ref()
    }
    fn last_timestamp(&self) -> Option<&String> {
        self.last_timestamp.as_ref()
    }
}
impl CompositeTimeseries {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut identifier = None;
        let mut name = Vec::new();
        let mut description = None;
        let mut first_timestamp = None;
        let mut last_timestamp = None;
        let mut component = Vec::new();
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
                (crate::namespace::NS_DYNAMIZER, "firstTimestamp") => {
                    first_timestamp = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_DYNAMIZER, "lastTimestamp") => {
                    last_timestamp = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "component") => {
                    component.push(TimeseriesComponent::from_gml(&mut sub)?);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(CompositeTimeseries {
            feature_id,
            identifier,
            name,
            description,
            first_timestamp,
            last_timestamp,
            component,
        })
    }
}
impl crate::from_gml::FromGml for CompositeTimeseries {
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
pub struct GenericTimeseries {
    pub feature_id: ID,
    pub identifier: Option<String>,
    pub name: Vec<String>,
    pub description: Option<String>,
    pub first_timestamp: Option<String>,
    pub last_timestamp: Option<String>,
    pub observation_property: String,
    pub uom: Option<String>,
    pub value_type: TimeseriesTypeValue,
    pub time_value_pair: Vec<TimeValuePair>,
}
impl AbstractFeatureTrait for GenericTimeseries {
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
impl AbstractTimeseriesTrait for GenericTimeseries {
    fn first_timestamp(&self) -> Option<&String> {
        self.first_timestamp.as_ref()
    }
    fn last_timestamp(&self) -> Option<&String> {
        self.last_timestamp.as_ref()
    }
}
impl AbstractAtomicTimeseriesTrait for GenericTimeseries {
    fn observation_property(&self) -> &String {
        &self.observation_property
    }
    fn uom(&self) -> Option<&String> {
        self.uom.as_ref()
    }
}
impl GenericTimeseries {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut identifier = None;
        let mut name = Vec::new();
        let mut description = None;
        let mut first_timestamp = None;
        let mut last_timestamp = None;
        let mut observation_property = Default::default();
        let mut uom = None;
        let mut value_type = Default::default();
        let mut time_value_pair = Vec::new();
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
                (crate::namespace::NS_DYNAMIZER, "firstTimestamp") => {
                    first_timestamp = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_DYNAMIZER, "lastTimestamp") => {
                    last_timestamp = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "observationProperty") => {
                    observation_property = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_DYNAMIZER, "uom") => {
                    uom = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "valueType") => {
                    value_type = TimeseriesTypeValue::from_gml_text(&sub.read_text()?)?;
                }
                (crate::namespace::NS_DYNAMIZER, "timeValuePair") => {
                    time_value_pair.push(TimeValuePair::from_gml(&mut sub)?);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(GenericTimeseries {
            feature_id,
            identifier,
            name,
            description,
            first_timestamp,
            last_timestamp,
            observation_property,
            uom,
            value_type,
            time_value_pair,
        })
    }
}
impl crate::from_gml::FromGml for GenericTimeseries {
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
pub struct StandardFileTimeseries {
    pub feature_id: ID,
    pub identifier: Option<String>,
    pub name: Vec<String>,
    pub description: Option<String>,
    pub first_timestamp: Option<String>,
    pub last_timestamp: Option<String>,
    pub observation_property: String,
    pub uom: Option<String>,
    pub file_location: String,
    pub file_type: StandardFileTypeValue,
    pub mime_type: Option<MimeTypeValue>,
}
impl AbstractFeatureTrait for StandardFileTimeseries {
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
impl AbstractTimeseriesTrait for StandardFileTimeseries {
    fn first_timestamp(&self) -> Option<&String> {
        self.first_timestamp.as_ref()
    }
    fn last_timestamp(&self) -> Option<&String> {
        self.last_timestamp.as_ref()
    }
}
impl AbstractAtomicTimeseriesTrait for StandardFileTimeseries {
    fn observation_property(&self) -> &String {
        &self.observation_property
    }
    fn uom(&self) -> Option<&String> {
        self.uom.as_ref()
    }
}
impl StandardFileTimeseries {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut identifier = None;
        let mut name = Vec::new();
        let mut description = None;
        let mut first_timestamp = None;
        let mut last_timestamp = None;
        let mut observation_property = Default::default();
        let mut uom = None;
        let mut file_location = Default::default();
        let mut file_type = Default::default();
        let mut mime_type = None;
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
                (crate::namespace::NS_DYNAMIZER, "firstTimestamp") => {
                    first_timestamp = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_DYNAMIZER, "lastTimestamp") => {
                    last_timestamp = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "observationProperty") => {
                    observation_property = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_DYNAMIZER, "uom") => {
                    uom = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "fileLocation") => {
                    file_location = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_DYNAMIZER, "fileType") => {
                    file_type = StandardFileTypeValue(sub.read_text()?);
                }
                (crate::namespace::NS_DYNAMIZER, "mimeType") => {
                    mime_type = Some(MimeTypeValue(sub.read_text()?));
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(StandardFileTimeseries {
            feature_id,
            identifier,
            name,
            description,
            first_timestamp,
            last_timestamp,
            observation_property,
            uom,
            file_location,
            file_type,
            mime_type,
        })
    }
}
impl crate::from_gml::FromGml for StandardFileTimeseries {
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
pub struct TabulatedFileTimeseries {
    pub feature_id: ID,
    pub identifier: Option<String>,
    pub name: Vec<String>,
    pub description: Option<String>,
    pub first_timestamp: Option<String>,
    pub last_timestamp: Option<String>,
    pub observation_property: String,
    pub uom: Option<String>,
    pub file_location: String,
    pub file_type: TabulatedFileTypeValue,
    pub mime_type: Option<MimeTypeValue>,
    pub value_type: TimeseriesTypeValue,
    pub number_of_header_lines: Option<i64>,
    pub field_separator: String,
    pub decimal_symbol: Option<char>,
    pub id_column_no: Option<i64>,
    pub id_column_name: Option<String>,
    pub id_value: Option<String>,
    pub time_column_no: Option<i64>,
    pub time_column_name: Option<String>,
    pub value_column_no: Option<i64>,
    pub value_column_name: Option<String>,
}
impl AbstractFeatureTrait for TabulatedFileTimeseries {
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
impl AbstractTimeseriesTrait for TabulatedFileTimeseries {
    fn first_timestamp(&self) -> Option<&String> {
        self.first_timestamp.as_ref()
    }
    fn last_timestamp(&self) -> Option<&String> {
        self.last_timestamp.as_ref()
    }
}
impl AbstractAtomicTimeseriesTrait for TabulatedFileTimeseries {
    fn observation_property(&self) -> &String {
        &self.observation_property
    }
    fn uom(&self) -> Option<&String> {
        self.uom.as_ref()
    }
}
impl TabulatedFileTimeseries {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut identifier = None;
        let mut name = Vec::new();
        let mut description = None;
        let mut first_timestamp = None;
        let mut last_timestamp = None;
        let mut observation_property = Default::default();
        let mut uom = None;
        let mut file_location = Default::default();
        let mut file_type = Default::default();
        let mut mime_type = None;
        let mut value_type = Default::default();
        let mut number_of_header_lines = None;
        let mut field_separator = Default::default();
        let mut decimal_symbol = None;
        let mut id_column_no = None;
        let mut id_column_name = None;
        let mut id_value = None;
        let mut time_column_no = None;
        let mut time_column_name = None;
        let mut value_column_no = None;
        let mut value_column_name = None;
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
                (crate::namespace::NS_DYNAMIZER, "firstTimestamp") => {
                    first_timestamp = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_DYNAMIZER, "lastTimestamp") => {
                    last_timestamp = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "observationProperty") => {
                    observation_property = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_DYNAMIZER, "uom") => {
                    uom = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "fileLocation") => {
                    file_location = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_DYNAMIZER, "fileType") => {
                    file_type = TabulatedFileTypeValue(sub.read_text()?);
                }
                (crate::namespace::NS_DYNAMIZER, "mimeType") => {
                    mime_type = Some(MimeTypeValue(sub.read_text()?));
                }
                (crate::namespace::NS_DYNAMIZER, "valueType") => {
                    value_type = TimeseriesTypeValue::from_gml_text(&sub.read_text()?)?;
                }
                (crate::namespace::NS_DYNAMIZER, "numberOfHeaderLines") => {
                    number_of_header_lines = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_DYNAMIZER, "fieldSeparator") => {
                    field_separator = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_DYNAMIZER, "decimalSymbol") => {
                    decimal_symbol = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "idColumnNo") => {
                    id_column_no = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "idColumnName") => {
                    id_column_name = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "idValue") => {
                    id_value = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "timeColumnNo") => {
                    time_column_no = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "timeColumnName") => {
                    time_column_name = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_DYNAMIZER, "valueColumnNo") => {
                    value_column_no = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_DYNAMIZER, "valueColumnName") => {
                    value_column_name = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(TabulatedFileTimeseries {
            feature_id,
            identifier,
            name,
            description,
            first_timestamp,
            last_timestamp,
            observation_property,
            uom,
            file_location,
            file_type,
            mime_type,
            value_type,
            number_of_header_lines,
            field_separator,
            decimal_symbol,
            id_column_no,
            id_column_name,
            id_value,
            time_column_no,
            time_column_name,
            value_column_no,
            value_column_name,
        })
    }
}
impl crate::from_gml::FromGml for TabulatedFileTimeseries {
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
pub struct Dynamizer {
    pub feature_id: ID,
    pub identifier: Option<String>,
    pub name: Vec<String>,
    pub description: Option<String>,
    pub creation_date: Option<String>,
    pub termination_date: Option<String>,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    pub attribute_ref: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub dynamic_data: Option<AbstractTimeseries>,
    pub sensor_connection: Option<SensorConnection>,
}
impl AbstractFeatureTrait for Dynamizer {
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
impl AbstractFeatureWithLifespanTrait for Dynamizer {
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
impl AbstractDynamizerTrait for Dynamizer {}
impl Dynamizer {
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
        let mut attribute_ref = Default::default();
        let mut start_time = None;
        let mut end_time = None;
        let mut dynamic_data = None;
        let mut sensor_connection = None;
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
                (crate::namespace::NS_DYNAMIZER, "attributeRef") => {
                    attribute_ref = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_DYNAMIZER, "startTime") => {
                    start_time = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "endTime") => {
                    end_time = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_DYNAMIZER, "dynamicData") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        dynamic_data = Some(
                            super::dispatchers::parse_abstract_timeseries(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    }
                }
                (crate::namespace::NS_DYNAMIZER, "sensorConnection") => {
                    sensor_connection = Some(SensorConnection::from_gml(&mut sub)?);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Dynamizer {
            feature_id,
            identifier,
            name,
            description,
            creation_date,
            termination_date,
            valid_from,
            valid_to,
            attribute_ref,
            start_time,
            end_time,
            dynamic_data,
            sensor_connection,
        })
    }
}
impl crate::from_gml::FromGml for Dynamizer {
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
