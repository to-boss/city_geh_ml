#![allow(unused_imports, unused_mut, unused_variables)]
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TextureType {
    #[default]
    Specific,
    Typical,
    Unknown,
}
impl TextureType {
    pub fn from_gml_text(text: &str) -> Result<Self, crate::error::ReaderError> {
        match text.trim() {
            "specific" => Ok(TextureType::Specific),
            "typical" => Ok(TextureType::Typical),
            "unknown" => Ok(TextureType::Unknown),
            other => {
                Err(crate::error::ReaderError::Parse {
                    message: format!(
                        "Unknown {} value: '{}'", stringify!(TextureType), other,
                    ),
                })
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WrapMode {
    #[default]
    None,
    Wrap,
    Mirror,
    Clamp,
    Border,
}
impl WrapMode {
    pub fn from_gml_text(text: &str) -> Result<Self, crate::error::ReaderError> {
        match text.trim() {
            "none" => Ok(WrapMode::None),
            "wrap" => Ok(WrapMode::Wrap),
            "mirror" => Ok(WrapMode::Mirror),
            "clamp" => Ok(WrapMode::Clamp),
            "border" => Ok(WrapMode::Border),
            other => {
                Err(crate::error::ReaderError::Parse {
                    message: format!(
                        "Unknown {} value: '{}'", stringify!(WrapMode), other,
                    ),
                })
            }
        }
    }
}
pub trait AbstractTextureParameterization: std::fmt::Debug {}
pub trait ADEOfAbstractSurfaceData: std::fmt::Debug {}
pub trait ADEOfAbstractTexture: std::fmt::Debug {}
pub trait ADEOfAppearance: std::fmt::Debug {}
pub trait ADEOfGeoreferencedTexture: std::fmt::Debug {}
pub trait ADEOfParameterizedTexture: std::fmt::Debug {}
pub trait ADEOfX3DMaterial: std::fmt::Debug {}
#[derive(Debug, Clone, Default)]
pub struct TexCoordGen {
    pub world_to_texture: TransformationMatrix3x4,
    pub crs: Option<String>,
}
impl crate::from_gml::FromGml for TexCoordGen {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut world_to_texture = Default::default();
        let mut crs = None;
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_APPEARANCE, "worldToTexture") => {
                    let child_info = crate::gml_reader::ElementInfo {
                        namespace: info.namespace.clone(),
                        local_name: info.local_name.clone(),
                        attributes: info.attributes.clone(),
                    };
                    world_to_texture = TransformationMatrix3x4::from_gml_with_info(
                        &mut sub,
                        &child_info,
                    )?;
                }
                (crate::namespace::NS_APPEARANCE, "crs") => {
                    crs = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(TexCoordGen {
            world_to_texture,
            crs,
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct TexCoordList {
    pub texture_coordinates: Vec<DoubleList>,
    pub ring: Vec<String>,
}
impl crate::from_gml::FromGml for TexCoordList {
    fn from_gml(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let mut texture_coordinates = Vec::new();
        let mut ring = Vec::new();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_APPEARANCE, "textureCoordinates") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        texture_coordinates
                            .push(
                                DoubleList::from_gml_with_info(&mut wrapper, &child_info)?,
                            );
                    }
                }
                (crate::namespace::NS_APPEARANCE, "ring") => {
                    ring.push(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(TexCoordList {
            texture_coordinates,
            ring,
        })
    }
}
pub trait AbstractSurfaceData: AbstractFeature {
    fn is_front(&self) -> Option<bool>;
    fn ade_of_abstract_surface_data(&self) -> &[Box<dyn ADEOfAbstractSurfaceData>];
}
#[derive(Debug, Clone, Default)]
pub struct Color {
    pub list: DoubleBetween0and1,
}
impl Color {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut list = Default::default();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "list") => {
                    list = DoubleBetween0and1(sub.read_text()?);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(Color { list })
    }
}
impl crate::from_gml::FromGml for Color {
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
pub struct ColorPlusOpacity {
    pub list: DoubleBetween0and1,
}
impl ColorPlusOpacity {
    pub fn from_gml_with_info(
        reader: &mut crate::gml_reader::SubtreeReader<'_>,
        info: &crate::gml_reader::ElementInfo,
    ) -> Result<Self, crate::error::ReaderError> {
        use crate::from_gml::FromGml;
        let _gml_id = crate::gml_reader::SubtreeReader::gml_id(info).unwrap_or_default();
        let mut list = Default::default();
        let mut sub = reader.subtree();
        while let Some(info) = sub.next_element()? {
            match (info.namespace.as_str(), info.local_name.as_str()) {
                (crate::namespace::NS_CORE, "list") => {
                    list = DoubleBetween0and1(sub.read_text()?);
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(ColorPlusOpacity { list })
    }
}
impl crate::from_gml::FromGml for ColorPlusOpacity {
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
pub trait AbstractTexture: AbstractSurfaceData {
    fn image_uri(&self) -> &String;
    fn mime_type(&self) -> Option<&MimeTypeValue>;
    fn texture_type(&self) -> Option<TextureType>;
    fn wrap_mode(&self) -> Option<WrapMode>;
    fn border_color(&self) -> Option<&ColorPlusOpacity>;
    fn ade_of_abstract_texture(&self) -> &[Box<dyn ADEOfAbstractTexture>];
}
#[derive(Debug, Default)]
pub struct X3DMaterial {
    pub feature_id: ID,
    pub identifier: Option<String>,
    pub name: Vec<String>,
    pub description: Option<String>,
    pub ade_of_abstract_feature: Vec<Box<dyn ADEOfAbstractFeature>>,
    pub is_front: Option<bool>,
    pub ade_of_abstract_surface_data: Vec<Box<dyn ADEOfAbstractSurfaceData>>,
    pub ambient_intensity: Option<DoubleBetween0and1>,
    pub diffuse_color: Option<Color>,
    pub emissive_color: Option<Color>,
    pub specular_color: Option<Color>,
    pub shininess: Option<DoubleBetween0and1>,
    pub transparency: Option<DoubleBetween0and1>,
    pub is_smooth: Option<bool>,
    pub target: Vec<String>,
    pub ade_of_x3_d_material: Vec<Box<dyn ADEOfX3DMaterial>>,
}
impl AbstractFeature for X3DMaterial {
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
impl AbstractSurfaceData for X3DMaterial {
    fn is_front(&self) -> Option<bool> {
        self.is_front
    }
    fn ade_of_abstract_surface_data(&self) -> &[Box<dyn ADEOfAbstractSurfaceData>] {
        &self.ade_of_abstract_surface_data
    }
}
impl X3DMaterial {
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
        let mut is_front = None;
        let mut ade_of_abstract_surface_data = Vec::new();
        let mut ambient_intensity = None;
        let mut diffuse_color = None;
        let mut emissive_color = None;
        let mut specular_color = None;
        let mut shininess = None;
        let mut transparency = None;
        let mut is_smooth = None;
        let mut target = Vec::new();
        let mut ade_of_x3_d_material = Vec::new();
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
                (crate::namespace::NS_APPEARANCE, "isFront") => {
                    is_front = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_APPEARANCE, "adeOfAbstractSurfaceData") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_APPEARANCE, "ambientIntensity") => {
                    ambient_intensity = Some(DoubleBetween0and1(sub.read_text()?));
                }
                (crate::namespace::NS_APPEARANCE, "diffuseColor") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        diffuse_color = Some(
                            Color::from_gml_with_info(&mut wrapper, &child_info)?,
                        );
                    } else {
                        diffuse_color = Some(Color::from_gml(&mut sub)?);
                    }
                }
                (crate::namespace::NS_APPEARANCE, "emissiveColor") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        emissive_color = Some(
                            Color::from_gml_with_info(&mut wrapper, &child_info)?,
                        );
                    } else {
                        emissive_color = Some(Color::from_gml(&mut sub)?);
                    }
                }
                (crate::namespace::NS_APPEARANCE, "specularColor") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        specular_color = Some(
                            Color::from_gml_with_info(&mut wrapper, &child_info)?,
                        );
                    } else {
                        specular_color = Some(Color::from_gml(&mut sub)?);
                    }
                }
                (crate::namespace::NS_APPEARANCE, "shininess") => {
                    shininess = Some(DoubleBetween0and1(sub.read_text()?));
                }
                (crate::namespace::NS_APPEARANCE, "transparency") => {
                    transparency = Some(DoubleBetween0and1(sub.read_text()?));
                }
                (crate::namespace::NS_APPEARANCE, "isSmooth") => {
                    is_smooth = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_APPEARANCE, "target") => {
                    target.push(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_APPEARANCE, "adeOfX3DMaterial") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(X3DMaterial {
            feature_id,
            identifier,
            name,
            description,
            ade_of_abstract_feature,
            is_front,
            ade_of_abstract_surface_data,
            ambient_intensity,
            diffuse_color,
            emissive_color,
            specular_color,
            shininess,
            transparency,
            is_smooth,
            target,
            ade_of_x3_d_material,
        })
    }
}
impl crate::from_gml::FromGml for X3DMaterial {
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
pub struct GeoreferencedTexture {
    pub feature_id: ID,
    pub identifier: Option<String>,
    pub name: Vec<String>,
    pub description: Option<String>,
    pub ade_of_abstract_feature: Vec<Box<dyn ADEOfAbstractFeature>>,
    pub is_front: Option<bool>,
    pub ade_of_abstract_surface_data: Vec<Box<dyn ADEOfAbstractSurfaceData>>,
    pub image_uri: String,
    pub mime_type: Option<MimeTypeValue>,
    pub texture_type: Option<TextureType>,
    pub wrap_mode: Option<WrapMode>,
    pub border_color: Option<ColorPlusOpacity>,
    pub ade_of_abstract_texture: Vec<Box<dyn ADEOfAbstractTexture>>,
    pub prefer_world_file: Option<bool>,
    pub orientation: Option<TransformationMatrix2x2>,
    pub target: Vec<String>,
    pub ade_of_georeferenced_texture: Vec<Box<dyn ADEOfGeoreferencedTexture>>,
    pub reference_point: Option<crate::geometry::DirectPosition>,
}
impl AbstractFeature for GeoreferencedTexture {
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
impl AbstractSurfaceData for GeoreferencedTexture {
    fn is_front(&self) -> Option<bool> {
        self.is_front
    }
    fn ade_of_abstract_surface_data(&self) -> &[Box<dyn ADEOfAbstractSurfaceData>] {
        &self.ade_of_abstract_surface_data
    }
}
impl AbstractTexture for GeoreferencedTexture {
    fn image_uri(&self) -> &String {
        &self.image_uri
    }
    fn mime_type(&self) -> Option<&MimeTypeValue> {
        self.mime_type.as_ref()
    }
    fn texture_type(&self) -> Option<TextureType> {
        self.texture_type
    }
    fn wrap_mode(&self) -> Option<WrapMode> {
        self.wrap_mode
    }
    fn border_color(&self) -> Option<&ColorPlusOpacity> {
        self.border_color.as_ref()
    }
    fn ade_of_abstract_texture(&self) -> &[Box<dyn ADEOfAbstractTexture>] {
        &self.ade_of_abstract_texture
    }
}
impl GeoreferencedTexture {
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
        let mut is_front = None;
        let mut ade_of_abstract_surface_data = Vec::new();
        let mut image_uri = Default::default();
        let mut mime_type = None;
        let mut texture_type = None;
        let mut wrap_mode = None;
        let mut border_color = None;
        let mut ade_of_abstract_texture = Vec::new();
        let mut prefer_world_file = None;
        let mut orientation = None;
        let mut target = Vec::new();
        let mut ade_of_georeferenced_texture = Vec::new();
        let mut reference_point = None;
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
                (crate::namespace::NS_APPEARANCE, "isFront") => {
                    is_front = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_APPEARANCE, "adeOfAbstractSurfaceData") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_APPEARANCE, "imageURI") => {
                    image_uri = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_APPEARANCE, "mimeType") => {
                    mime_type = Some(MimeTypeValue(sub.read_text()?));
                }
                (crate::namespace::NS_APPEARANCE, "textureType") => {
                    texture_type = Some(TextureType::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_APPEARANCE, "wrapMode") => {
                    wrap_mode = Some(WrapMode::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_APPEARANCE, "borderColor") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        border_color = Some(
                            ColorPlusOpacity::from_gml_with_info(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    } else {
                        border_color = Some(ColorPlusOpacity::from_gml(&mut sub)?);
                    }
                }
                (crate::namespace::NS_APPEARANCE, "adeOfAbstractTexture") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_APPEARANCE, "preferWorldFile") => {
                    prefer_world_file = Some(
                        crate::from_gml::FromGml::from_gml(&mut sub)?,
                    );
                }
                (crate::namespace::NS_APPEARANCE, "orientation") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        orientation = Some(
                            TransformationMatrix2x2::from_gml_with_info(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    } else {
                        orientation = Some(TransformationMatrix2x2::from_gml(&mut sub)?);
                    }
                }
                (crate::namespace::NS_APPEARANCE, "target") => {
                    target.push(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_APPEARANCE, "adeOfGeoreferencedTexture") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_APPEARANCE, "referencePoint") => {
                    reference_point = Some({
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
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(GeoreferencedTexture {
            feature_id,
            identifier,
            name,
            description,
            ade_of_abstract_feature,
            is_front,
            ade_of_abstract_surface_data,
            image_uri,
            mime_type,
            texture_type,
            wrap_mode,
            border_color,
            ade_of_abstract_texture,
            prefer_world_file,
            orientation,
            target,
            ade_of_georeferenced_texture,
            reference_point,
        })
    }
}
impl crate::from_gml::FromGml for GeoreferencedTexture {
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
pub struct ParameterizedTexture {
    pub feature_id: ID,
    pub identifier: Option<String>,
    pub name: Vec<String>,
    pub description: Option<String>,
    pub ade_of_abstract_feature: Vec<Box<dyn ADEOfAbstractFeature>>,
    pub is_front: Option<bool>,
    pub ade_of_abstract_surface_data: Vec<Box<dyn ADEOfAbstractSurfaceData>>,
    pub image_uri: String,
    pub mime_type: Option<MimeTypeValue>,
    pub texture_type: Option<TextureType>,
    pub wrap_mode: Option<WrapMode>,
    pub border_color: Option<ColorPlusOpacity>,
    pub ade_of_abstract_texture: Vec<Box<dyn ADEOfAbstractTexture>>,
    pub ade_of_parameterized_texture: Vec<Box<dyn ADEOfParameterizedTexture>>,
    pub texture_parameterization: Vec<Box<dyn AbstractTextureParameterization>>,
}
impl AbstractFeature for ParameterizedTexture {
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
impl AbstractSurfaceData for ParameterizedTexture {
    fn is_front(&self) -> Option<bool> {
        self.is_front
    }
    fn ade_of_abstract_surface_data(&self) -> &[Box<dyn ADEOfAbstractSurfaceData>] {
        &self.ade_of_abstract_surface_data
    }
}
impl AbstractTexture for ParameterizedTexture {
    fn image_uri(&self) -> &String {
        &self.image_uri
    }
    fn mime_type(&self) -> Option<&MimeTypeValue> {
        self.mime_type.as_ref()
    }
    fn texture_type(&self) -> Option<TextureType> {
        self.texture_type
    }
    fn wrap_mode(&self) -> Option<WrapMode> {
        self.wrap_mode
    }
    fn border_color(&self) -> Option<&ColorPlusOpacity> {
        self.border_color.as_ref()
    }
    fn ade_of_abstract_texture(&self) -> &[Box<dyn ADEOfAbstractTexture>] {
        &self.ade_of_abstract_texture
    }
}
impl ParameterizedTexture {
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
        let mut is_front = None;
        let mut ade_of_abstract_surface_data = Vec::new();
        let mut image_uri = Default::default();
        let mut mime_type = None;
        let mut texture_type = None;
        let mut wrap_mode = None;
        let mut border_color = None;
        let mut ade_of_abstract_texture = Vec::new();
        let mut ade_of_parameterized_texture = Vec::new();
        let mut texture_parameterization = Vec::new();
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
                (crate::namespace::NS_APPEARANCE, "isFront") => {
                    is_front = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_APPEARANCE, "adeOfAbstractSurfaceData") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_APPEARANCE, "imageURI") => {
                    image_uri = crate::from_gml::FromGml::from_gml(&mut sub)?;
                }
                (crate::namespace::NS_APPEARANCE, "mimeType") => {
                    mime_type = Some(MimeTypeValue(sub.read_text()?));
                }
                (crate::namespace::NS_APPEARANCE, "textureType") => {
                    texture_type = Some(TextureType::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_APPEARANCE, "wrapMode") => {
                    wrap_mode = Some(WrapMode::from_gml_text(&sub.read_text()?)?);
                }
                (crate::namespace::NS_APPEARANCE, "borderColor") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        border_color = Some(
                            ColorPlusOpacity::from_gml_with_info(
                                &mut wrapper,
                                &child_info,
                            )?,
                        );
                    } else {
                        border_color = Some(ColorPlusOpacity::from_gml(&mut sub)?);
                    }
                }
                (crate::namespace::NS_APPEARANCE, "adeOfAbstractTexture") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_APPEARANCE, "adeOfParameterizedTexture") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_APPEARANCE, "textureParameterization") => {
                    sub.skip_element()?;
                }
                _ => {
                    sub.skip_element()?;
                }
            }
        }
        Ok(ParameterizedTexture {
            feature_id,
            identifier,
            name,
            description,
            ade_of_abstract_feature,
            is_front,
            ade_of_abstract_surface_data,
            image_uri,
            mime_type,
            texture_type,
            wrap_mode,
            border_color,
            ade_of_abstract_texture,
            ade_of_parameterized_texture,
            texture_parameterization,
        })
    }
}
impl crate::from_gml::FromGml for ParameterizedTexture {
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
pub struct Appearance {
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
    pub ade_of_abstract_appearance: Vec<Box<dyn ADEOfAbstractAppearance>>,
    pub theme: Option<String>,
    pub ade_of_appearance: Vec<Box<dyn ADEOfAppearance>>,
    pub surface_data: Vec<Box<dyn AbstractSurfaceData>>,
}
impl AbstractFeature for Appearance {
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
impl AbstractFeatureWithLifespan for Appearance {
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
impl AbstractAppearance for Appearance {
    fn ade_of_abstract_appearance(&self) -> &[Box<dyn ADEOfAbstractAppearance>] {
        &self.ade_of_abstract_appearance
    }
}
impl Appearance {
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
        let mut ade_of_abstract_appearance = Vec::new();
        let mut theme = None;
        let mut ade_of_appearance = Vec::new();
        let mut surface_data = Vec::new();
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
                (crate::namespace::NS_CORE, "adeOfAbstractAppearance") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_APPEARANCE, "theme") => {
                    theme = Some(crate::from_gml::FromGml::from_gml(&mut sub)?);
                }
                (crate::namespace::NS_APPEARANCE, "adeOfAppearance") => {
                    sub.skip_element()?;
                }
                (crate::namespace::NS_APPEARANCE, "surfaceData") => {
                    let mut wrapper = sub.subtree();
                    if let Some(child_info) = wrapper.next_element()? {
                        surface_data
                            .push(
                                super::dispatchers::parse_dyn_abstract_surface_data(
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
        Ok(Appearance {
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
            ade_of_abstract_appearance,
            theme,
            ade_of_appearance,
            surface_data,
        })
    }
}
impl crate::from_gml::FromGml for Appearance {
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
