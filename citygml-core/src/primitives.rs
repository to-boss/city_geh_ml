use crate::error::ReaderError;
use crate::from_gml::FromGml;
use crate::gml_reader::SubtreeReader;

impl FromGml for String {
    fn from_gml(reader: &mut SubtreeReader<'_>) -> Result<Self, ReaderError> {
        reader.read_text()
    }
}

impl FromGml for i64 {
    fn from_gml(reader: &mut SubtreeReader<'_>) -> Result<Self, ReaderError> {
        let text = reader.read_text()?;
        text.trim().parse::<i64>().map_err(|e| ReaderError::Parse {
            message: format!("Expected integer, got '{text}': {e}"),
        })
    }
}

impl FromGml for f64 {
    fn from_gml(reader: &mut SubtreeReader<'_>) -> Result<Self, ReaderError> {
        let text = reader.read_text()?;
        text.trim().parse::<f64>().map_err(|e| ReaderError::Parse {
            message: format!("Expected float, got '{text}': {e}"),
        })
    }
}

impl FromGml for bool {
    fn from_gml(reader: &mut SubtreeReader<'_>) -> Result<Self, ReaderError> {
        let text = reader.read_text()?;
        match text.trim() {
            "true" | "1" => Ok(true),
            "false" | "0" => Ok(false),
            other => Err(ReaderError::Parse {
                message: format!("Expected boolean, got '{other}'"),
            }),
        }
    }
}

impl FromGml for char {
    fn from_gml(reader: &mut SubtreeReader<'_>) -> Result<Self, ReaderError> {
        let text = reader.read_text()?;
        text.chars().next().ok_or_else(|| ReaderError::Parse {
            message: "Expected a character, got empty string".into(),
        })
    }
}

// Geometry types get FromGml via the geometry parser

use crate::geometry::*;
use crate::gml_geometry::*;

impl FromGml for MultiSurface {
    fn from_gml(reader: &mut SubtreeReader<'_>) -> Result<Self, ReaderError> {
        parse_multi_surface(reader)
    }
}

impl FromGml for Solid {
    fn from_gml(reader: &mut SubtreeReader<'_>) -> Result<Self, ReaderError> {
        parse_solid(reader)
    }
}

impl FromGml for TriangulatedSurface {
    fn from_gml(reader: &mut SubtreeReader<'_>) -> Result<Self, ReaderError> {
        parse_triangulated_surface(reader)
    }
}

impl FromGml for DirectPosition {
    fn from_gml(reader: &mut SubtreeReader<'_>) -> Result<Self, ReaderError> {
        parse_point(reader)
    }
}

impl FromGml for Polygon {
    fn from_gml(reader: &mut SubtreeReader<'_>) -> Result<Self, ReaderError> {
        parse_polygon(reader)
    }
}

impl FromGml for MultiCurve {
    fn from_gml(reader: &mut SubtreeReader<'_>) -> Result<Self, ReaderError> {
        parse_multi_curve(reader)
    }
}
