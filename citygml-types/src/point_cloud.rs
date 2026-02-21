#![allow(unused_imports, unused_mut, unused_variables)]
use super::*;

#[derive(Debug, Clone, Default)]
pub struct PointCloud {
    pub feature_id: ID,
    pub identifier: Option<String>,
    pub name: Vec<String>,
    pub description: Option<String>,
    pub mime_type: Option<MimeTypeValue>,
    pub point_file: Option<String>,
    pub point_file_srs_name: Option<String>,
    pub points: Option<Vec<crate::geometry::DirectPosition>>,
}
impl AbstractFeatureTrait for PointCloud {
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
impl AbstractPointCloudTrait for PointCloud {}
impl PointCloud {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut identifier = None;
        let mut name = Vec::new();
        let mut description = None;
        let mut mime_type = None;
        let mut point_file = None;
        let mut point_file_srs_name = None;
        let mut points = None;
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
                (crate::namespace::NS_POINT_CLOUD, "mimeType") => {
                    mime_type = Some(MimeTypeValue(sub.read_text()?));
                }
                (crate::namespace::NS_POINT_CLOUD, "pointFile") => {
                    point_file = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_POINT_CLOUD, "pointFileSrsName") => {
                    point_file_srs_name = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_POINT_CLOUD, "points") => {
                    points = Some({
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
        Ok(PointCloud {
            feature_id,
            identifier,
            name,
            description,
            mime_type,
            point_file,
            point_file_srs_name,
            points,
        })
    }
}
impl crate::from_gml::FromGml for PointCloud {
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
