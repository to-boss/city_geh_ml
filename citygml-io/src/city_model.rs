use std::path::Path;

use citygml_core::error::ReaderError;
use citygml_core::gml_reader::{reader_from_path, SubtreeReader, GmlReader, ElementInfo};
use citygml_core::namespace::*;

/// The top-level parsed CityGML model.
///
/// Contains vectors of concrete feature types found in the document.
#[derive(Debug, Default)]
pub struct ParsedCityModel {
    pub buildings: Vec<citygml_types::building::Building>,
    pub building_parts: Vec<citygml_types::building::BuildingPart>,
    pub relief_features: Vec<citygml_types::relief::ReliefFeature>,
}

pub struct CitygmlReader;

impl CitygmlReader {
    /// Parse a CityGML 3.0 GML file and return the populated model.
    pub fn from_path(path: impl AsRef<Path>) -> Result<ParsedCityModel, ReaderError> {
        let mut gml = reader_from_path(path.as_ref())?;
        let mut model = ParsedCityModel::default();

        // Find the root CityModel element
        let root = match gml.next_start_element()? {
            Some(info) => info,
            None => return Ok(model),
        };

        if root.local_name != "CityModel" {
            return Err(ReaderError::UnexpectedStructure(
                format!("Expected CityModel root, got {}", root.local_name),
            ));
        }

        Self::parse_city_model_children(&mut gml, &mut model)?;
        Ok(model)
    }

    fn parse_city_model_children(
        gml: &mut GmlReader,
        model: &mut ParsedCityModel,
    ) -> Result<(), ReaderError> {
        let mut sub = gml.subtree();
        while let Some(info) = sub.next_element()? {
            if info.namespace == NS_CORE && info.local_name == "cityObjectMember" {
                Self::parse_city_object_member(&mut sub, model)?;
            } else {
                sub.skip_element()?;
            }
        }
        Ok(())
    }

    fn parse_city_object_member(
        parent: &mut SubtreeReader<'_>,
        model: &mut ParsedCityModel,
    ) -> Result<(), ReaderError> {
        let mut sub = parent.subtree();
        while let Some(info) = sub.next_element()? {
            Self::dispatch_feature(&mut sub, &info, model)?;
        }
        Ok(())
    }

    fn dispatch_feature(
        reader: &mut SubtreeReader<'_>,
        info: &ElementInfo,
        model: &mut ParsedCityModel,
    ) -> Result<(), ReaderError> {
        match (info.namespace.as_str(), info.local_name.as_str()) {
            (NS_BUILDING, "Building") => {
                let b = citygml_types::building::Building::from_gml_with_info(reader, info)?;
                model.buildings.push(b);
            }
            (NS_BUILDING, "BuildingPart") => {
                let bp = citygml_types::building::BuildingPart::from_gml_with_info(reader, info)?;
                model.building_parts.push(bp);
            }
            (NS_RELIEF, "ReliefFeature") => {
                let r = citygml_types::relief::ReliefFeature::from_gml_with_info(reader, info)?;
                model.relief_features.push(r);
            }
            _ => {
                reader.skip_element()?;
            }
        }
        Ok(())
    }
}
