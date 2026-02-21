#![allow(unused_imports, unused_mut, unused_variables)]
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TransactionTypeValue {
    #[default]
    Insert,
    Delete,
    Replace,
}
impl TransactionTypeValue {
    pub fn from_gml_text(text: &str) -> Result<Self, crate::error::ReaderError> {
        match text.trim() {
            "insert" => Ok(TransactionTypeValue::Insert),
            "delete" => Ok(TransactionTypeValue::Delete),
            "replace" => Ok(TransactionTypeValue::Replace),
            other => {
                Err(crate::error::ReaderError::Parse {
                    message: format!(
                        "Unknown {} value: '{}'", stringify!(TransactionTypeValue),
                        other,
                    ),
                })
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TransitionTypeValue {
    #[default]
    Planned,
    Realized,
    HistoricalSuccession,
    Fork,
    Merge,
}
impl TransitionTypeValue {
    pub fn from_gml_text(text: &str) -> Result<Self, crate::error::ReaderError> {
        match text.trim() {
            "planned" => Ok(TransitionTypeValue::Planned),
            "realized" => Ok(TransitionTypeValue::Realized),
            "historicalSuccession" => Ok(TransitionTypeValue::HistoricalSuccession),
            "fork" => Ok(TransitionTypeValue::Fork),
            "merge" => Ok(TransitionTypeValue::Merge),
            other => {
                Err(crate::error::ReaderError::Parse {
                    message: format!(
                        "Unknown {} value: '{}'", stringify!(TransitionTypeValue), other,
                    ),
                })
            }
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct Transaction {
    pub type_: TransactionTypeValue,
    pub new_feature: Option<AbstractFeatureWithLifespan>,
    pub old_feature: Option<AbstractFeatureWithLifespan>,
}
impl crate::from_gml::FromGml for Transaction {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut type_ = Default::default();
        let mut new_feature = None;
        let mut old_feature = None;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_VERSIONING, "type") => {
                    type_ = TransactionTypeValue::from_gml_text(&sub.read_text()?)?;
                }
                (crate::namespace::NS_VERSIONING, "newFeature") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        new_feature = Some(
                            super::dispatchers::parse_abstract_feature_with_lifespan(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    }
                }
                (crate::namespace::NS_VERSIONING, "oldFeature") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        old_feature = Some(
                            super::dispatchers::parse_abstract_feature_with_lifespan(
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
        Ok(Transaction {
            type_,
            new_feature,
            old_feature,
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct Version {
    pub feature_id: ID,
    pub identifier: Option<String>,
    pub name: Vec<String>,
    pub description: Option<String>,
    pub creation_date: Option<String>,
    pub termination_date: Option<String>,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    pub tag: Vec<String>,
    pub version_member: Vec<AbstractFeatureWithLifespan>,
}
impl AbstractFeatureTrait for Version {
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
impl AbstractFeatureWithLifespanTrait for Version {
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
impl AbstractVersionTrait for Version {}
impl Version {
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
        let mut tag = Vec::new();
        let mut version_member = Vec::new();
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
                (crate::namespace::NS_VERSIONING, "tag") => {
                    tag.push(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_VERSIONING, "versionMember") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        version_member
                            .push(
                                super::dispatchers::parse_abstract_feature_with_lifespan(
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
        Ok(Version {
            feature_id,
            identifier,
            name,
            description,
            creation_date,
            termination_date,
            valid_from,
            valid_to,
            tag,
            version_member,
        })
    }
}
impl crate::from_gml::FromGml for Version {
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
pub struct VersionTransition {
    pub feature_id: ID,
    pub identifier: Option<String>,
    pub name: Vec<String>,
    pub description: Option<String>,
    pub creation_date: Option<String>,
    pub termination_date: Option<String>,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    pub reason: Option<String>,
    pub clone_predecessor: bool,
    pub type_: Option<TransitionTypeValue>,
    pub from: Option<Version>,
    pub to: Option<Version>,
    pub transaction: Vec<Transaction>,
}
impl AbstractFeatureTrait for VersionTransition {
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
impl AbstractFeatureWithLifespanTrait for VersionTransition {
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
impl AbstractVersionTransitionTrait for VersionTransition {}
impl VersionTransition {
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
        let mut reason = None;
        let mut clone_predecessor = false;
        let mut type_ = None;
        let mut from = None;
        let mut to = None;
        let mut transaction = Vec::new();
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
                (crate::namespace::NS_VERSIONING, "reason") => {
                    reason = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_VERSIONING, "clonePredecessor") => {
                    clone_predecessor = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_VERSIONING, "type") => {
                    type_ = Some(TransitionTypeValue::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_VERSIONING, "from") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        from = Some(
                            Version::from_gml_with_info(&mut wrapper, &child_info)?,
                        );
                    } else {
                        from = Some(Version::from_gml(&mut sub)?);
                    }
                }
                (crate::namespace::NS_VERSIONING, "to") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        to = Some(
                            Version::from_gml_with_info(&mut wrapper, &child_info)?,
                        );
                    } else {
                        to = Some(Version::from_gml(&mut sub)?);
                    }
                }
                (crate::namespace::NS_VERSIONING, "transaction") => {
                    transaction.push(Transaction::from_gml(&mut sub)?);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(VersionTransition {
            feature_id,
            identifier,
            name,
            description,
            creation_date,
            termination_date,
            valid_from,
            valid_to,
            reason,
            clone_predecessor,
            type_,
            from,
            to,
            transaction,
        })
    }
}
impl crate::from_gml::FromGml for VersionTransition {
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
